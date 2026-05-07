use crate::app::gui::{Action, Application, SUPPORTED_AUDIO_FORMATS};
use anyhow::{Result, anyhow};
use eframe::Frame;
use egui::Context;
use rfd::FileDialog;
use rodio::Source;
use rust_i18n::t;
use std::fs;

impl Application {
    pub fn _logic(&mut self, ctx: &Context, _frame: &mut Frame) {
        while !self.actions.is_empty() {
            let action = self.actions.remove(0);

            if !self.audio_player_initialized && action != Action::InitAudioPlayer {
                break;
            }

            match action {
                Action::InitAudioPlayer => {
                    match self.init_audio_player() {
                        Ok(_) => {
                            self.audio_player_initialized = true;
                        }
                        Err(e) => {
                            self.toasts
                                .error(t!("errors.initialization_failed", error = e.to_string()));
                        }
                    };
                }
                Action::OpenFile => {
                    match self.open_file() {
                        Ok(_) => (),
                        Err(e) => {
                            self.toasts
                                .error(t!("errors.file_open_failed", error = e.to_string()));
                        }
                    };
                }
                Action::PlayFile => {
                    match self.play_file() {
                        Ok(_) => (),
                        Err(e) => {
                            self.toasts
                                .error(t!("errors.playback_failed", error = e.to_string()));
                            self.file_path = None;
                        }
                    };
                }
            }
        }

        if let Some(sink) = &self.audio_sink {
            if !sink.is_paused() {
                ctx.request_repaint_after_secs(1.0 / 60.0);
            }

            self.audio_position = match sink.empty() {
                true => 0,
                false => sink.get_pos().as_secs(),
            };
        }
    }

    fn init_audio_player(&mut self) -> Result<()> {
        self.audio_handle = Some(rodio::DeviceSinkBuilder::open_default_sink()?);
        self.audio_handle.as_mut().unwrap().log_on_drop(false);
        self.audio_sink = Some(rodio::Player::connect_new(
            self.audio_handle.as_ref().unwrap().mixer(),
        ));
        Ok(())
    }

    fn open_file(&mut self) -> Result<()> {
        let file = FileDialog::new()
            .add_filter(t!("file_dialog.filter_name"), SUPPORTED_AUDIO_FORMATS)
            .pick_file();

        if file.is_none() {
            return Err(anyhow!("No file selected"));
        }

        let file = file.unwrap();
        self.file_path = Some(file);

        self.actions.push(Action::PlayFile);

        Ok(())
    }

    fn play_file(&mut self) -> Result<()> {
        if self.file_path.is_none() {
            return Err(anyhow!("No file selected"));
        }

        if self.audio_handle.is_none() || self.audio_sink.is_none() {
            return Err(anyhow!("Audio player not initialized"));
        }

        let file = fs::File::open(self.file_path.as_ref().unwrap())?;
        let source = rodio::Decoder::try_from(file)?;

        self.audio_duration = source.total_duration().unwrap_or_default().as_secs();
        self.audio_sink.as_ref().unwrap().append(source);

        Ok(())
    }
}
