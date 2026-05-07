use crate::app::gui::{Action, Application};
use eframe::Frame;
use egui::{Align, Button, CentralPanel, Label, Layout, Panel, RichText, Sense, Slider, Ui};
use egui_material_icons::icons;
use rust_i18n::t;
use unicode_normalization::UnicodeNormalization;

impl Application {
    pub fn _ui(&mut self, ui: &mut Ui, _frame: &mut Frame) {
        self.toasts.show(ui.ctx());

        if self.file_path.is_none() {
            CentralPanel::default().show_inside(ui, |ui| {
                self.welcome_message(ui);
            });
            return;
        }

        Panel::top("header_panel").show_inside(ui, |ui| {
            self.header(ui);
        });

        Panel::bottom("footer_panel").show_inside(ui, |ui| {
            self.footer(ui);
        });

        CentralPanel::default().show_inside(ui, |ui| {
            self.controls(ui);
        });
    }

    fn welcome_message(&mut self, ui: &mut Ui) {
        let inner_response = ui.centered_and_justified(|ui| {
            let welcome_label =
                Label::new(RichText::new(t!("message.welcome")).size(18.0)).selectable(false);
            ui.add(welcome_label);
        });

        let response = ui.interact(
            inner_response.response.rect,
            ui.id().with("welcome_screen_click"),
            Sense::click(),
        );

        if response.clicked() {
            self.actions.push(Action::OpenFile);
        }

        ui.input(|i| {
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
    }

    fn header(&mut self, ui: &mut Ui) {
        let footer_label =
            Label::new(RichText::new(self.file_path_normilized.as_ref().unwrap()).size(16.0))
                .truncate();
        ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
            ui.add(footer_label);
        });
    }

    fn footer(&mut self, ui: &mut Ui) {
        let sink = self.audio_sink.as_ref().unwrap();

        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            fn format_time_pair(position: f32, duration: f32) -> String {
                fn format_time(seconds: f32) -> String {
                    let total_seconds = seconds.round() as u32;
                    let minutes = total_seconds / 60;
                    let secs = total_seconds % 60;
                    format!("{:02}:{:02}", minutes, secs)
                }

                format!("{} / {}", format_time(position), format_time(duration))
            }

            let progress_text = format_time_pair(
                self.audio_props.position as f32,
                self.audio_props.duration as f32,
            );

            let progress_label =
                Label::new(RichText::new(progress_text).size(16.0)).selectable(false);
            ui.add(progress_label);

            ui.spacing_mut().slider_width = 80.0;

            let mut volume = sink.volume();

            let volume_slider = Slider::new(&mut volume, 0.0..=1.0).show_value(false);
            if ui.add(volume_slider).dragged() {
                sink.set_volume(volume);
            }

            let volume_button = Button::new(
                RichText::new(if volume == 0.0 {
                    icons::ICON_VOLUME_OFF.codepoint
                } else if volume < 0.4 {
                    icons::ICON_VOLUME_DOWN.codepoint
                } else {
                    icons::ICON_VOLUME_UP.codepoint
                })
                .size(16.0),
            )
            .frame(false);

            let volume_btn_response = ui.add(volume_button);
            if volume_btn_response.clicked() {
                if volume == 0.0 {
                    sink.set_volume(self.volume_before_mute);
                } else {
                    self.volume_before_mute = volume;
                    sink.set_volume(0.0);
                }
            }

            ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                let mut format_type_text = String::new();
                if let Some(ft) = self.audio_props.format_type {
                    format_type_text = format!("{:?} | ", ft);
                }

                let mut bitrate_text = String::new();
                if let Some(bt) = self.audio_props.bitrate {
                    bitrate_text = format!("{} bit | ", bt);
                }

                let mut sample_rate_text = String::new();
                if let Some(sr) = self.audio_props.sample_rate {
                    sample_rate_text = format!("{} hz | ", sr);
                }

                let mut channels_text = String::new();
                if let Some(cn) = self.audio_props.channels {
                    channels_text = format!("{} cn", cn);
                }

                let info_text = format!(
                    "{}{}{}{}",
                    format_type_text, bitrate_text, sample_rate_text, channels_text
                );

                let info_label = Label::new(RichText::new(info_text).size(16.0)).truncate();
                ui.add(info_label);
            });
        });
    }

    fn controls(&mut self, ui: &mut Ui) {
        let sink = self.audio_sink.as_ref().unwrap();

        ui.horizontal_centered(|ui| {
            ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                let play_button = Button::new(
                    RichText::new(match sink.is_paused() {
                        true => icons::ICON_PLAY_ARROW.codepoint,
                        false => match sink.empty() {
                            true => icons::ICON_PLAY_ARROW.codepoint,
                            false => icons::ICON_PAUSE.codepoint,
                        },
                    })
                    .size(32.0),
                )
                .frame(false);
                if ui.add(play_button).clicked() {
                    if sink.empty() {
                        self.actions.push(Action::PlayFile);
                    } else if sink.is_paused() {
                        sink.play();
                    } else {
                        sink.pause();
                    }
                }

                let stop_button =
                    Button::new(RichText::new(icons::ICON_STOP.codepoint).size(32.0)).frame(false);
                if ui.add(stop_button).clicked() {
                    sink.stop();
                }
            });

            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                let loop_button = Button::new(
                    RichText::new(match self.is_looped {
                        true => icons::ICON_REPEAT_ONE.codepoint,
                        false => icons::ICON_REPEAT.codepoint,
                    })
                    .size(24.0),
                )
                .frame(false);
                if ui.add(loop_button).clicked() {
                    self.is_looped = !self.is_looped;
                }

                ui.scope(|ui| {
                    ui.spacing_mut().interact_size.y = 24.0;
                    ui.spacing_mut().slider_rail_height = 24.0;

                    ui.spacing_mut().slider_width = ui.available_width();
                    let progress_slider = Slider::new(
                        &mut self.audio_props.position,
                        0..=self.audio_props.duration,
                    )
                    .step_by(1.0)
                    .show_value(false)
                    .trailing_fill(true);
                    if ui
                        .add_enabled(!sink.empty(), progress_slider)
                        .drag_stopped()
                    {
                        let new_position =
                            std::time::Duration::from_secs(self.audio_props.position);
                        match sink.try_seek(new_position) {
                            Ok(_) => (),
                            Err(e) => {
                                self.toasts
                                    .error(t!("errors.seek_failed", error = e.to_string()));
                            }
                        }
                    }
                });
            });
        });
    }
}
