use crate::app::gui::{Action, Application, SUPPORTED_AUDIO_FORMATS};
use anyhow::{Result, anyhow};
use eframe::Frame;
use egui::Context;
use lofty::{
    file::{AudioFile, TaggedFileExt},
    probe::Probe as LoftyProbe,
};
use rfd::FileDialog;
use rodio::Source;
use rust_i18n::t;
use std::{env, fs, path::Path, process::Command};
use unicode_normalization::UnicodeNormalization;

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
                Action::ReadFileProps => {
                    match self.read_file_props() {
                        Ok(_) => (),
                        Err(e) => {
                            self.toasts
                                .error(t!("errors.read_props_failed", error = e.to_string()));
                            self.file_path = None;
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
                Action::StartNewInstance(file_path) => {
                    match self.start_new_instance(&file_path) {
                        Ok(_) => (),
                        Err(e) => {
                            self.toasts
                                .error(t!("errors.new_instance_failed", error = e.to_string()));
                            self.file_path = None;
                        }
                    };
                }
            }
        }

        ctx.input(|i| {
            for idx in 0..i.raw.dropped_files.len() {
                let dropped_file = &i.raw.dropped_files[idx];
                if let Some(path) = &dropped_file.path {
                    if idx == 0 {
                        self.file_path = Some(path.clone());
                        self.file_path_normilized =
                            Some(path.to_string_lossy().nfc().collect::<String>());
                        self.actions.push(Action::ReadFileProps);
                        self.actions.push(Action::PlayFile);
                    } else {
                        self.actions.push(Action::StartNewInstance(path.clone()));
                    }
                }
            }
        });

        if let Some(sink) = &self.audio_sink {
            if !sink.is_paused() {
                ctx.request_repaint_after_secs(1.0 / 60.0);
            }

            self.audio_props.position = match sink.empty() {
                true => 0,
                false => sink.get_pos().as_secs(),
            };

            if sink.empty() && !self.is_finished {
                match self.is_looped {
                    true => self.actions.push(Action::PlayFile),
                    false => self.is_finished = true,
                }
            }
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
        let files = FileDialog::new()
            .add_filter(t!("file_dialog.filter_name"), SUPPORTED_AUDIO_FORMATS)
            .pick_files()
            .unwrap_or_default();

        for (i, file) in files.iter().enumerate() {
            if i == 0 {
                self.file_path = Some(file.clone());
                self.file_path_normilized = Some(file.to_string_lossy().nfc().collect::<String>());
                self.actions.push(Action::ReadFileProps);
                self.actions.push(Action::PlayFile);
            } else {
                self.actions.push(Action::StartNewInstance(file.clone()));
            }
        }

        Ok(())
    }

    fn read_file_props(&mut self) -> Result<()> {
        if self.file_path.is_none() {
            return Err(anyhow!("No file selected"));
        }

        let tagged_file = LoftyProbe::open(self.file_path.as_ref().unwrap())?.read()?;

        let props = tagged_file.properties();

        self.audio_props.sample_rate = props.sample_rate();
        self.audio_props.bitrate = props.audio_bitrate();
        self.audio_props.channels = props.channels();
        self.audio_props.format_type = Some(tagged_file.file_type());

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

        self.audio_props.duration = source.total_duration().unwrap_or_default().as_secs();
        self.audio_sink.as_ref().unwrap().append(source);
        self.audio_sink.as_ref().unwrap().play();
        self.is_finished = false;

        Ok(())
    }

    fn start_new_instance(&self, file_path: &Path) -> Result<()> {
        let exe_path = match env::current_exe() {
            Ok(path) => path,
            Err(e) => return Err(anyhow!(e)),
        };

        Command::new(&exe_path).arg(file_path).spawn()?;

        Ok(())
    }
}
