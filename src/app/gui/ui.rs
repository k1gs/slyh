use crate::app::gui::{Action, Application};
use eframe::Frame;
use egui::{Align, Button, CentralPanel, Label, Layout, Panel, RichText, Sense, Slider, Ui};
use egui_material_icons::icons;
use rust_i18n::t;

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

        let is_hovering_file = ui.input(|i| !i.raw.hovered_files.is_empty());
        if is_hovering_file {
            todo!("DRAG AND DROP");
        }
    }

    fn header(&mut self, ui: &mut Ui) {
        let footer_label =
            Label::new(RichText::new(self.file_path_normilized.as_ref().unwrap()).size(10.0))
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

                format!("{}/{}", format_time(position), format_time(duration))
            }

            let progress_text =
                format_time_pair(self.audio_position as f32, self.audio_duration as f32);

            let progress_label =
                Label::new(RichText::new(progress_text).size(8.0)).selectable(false);
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
                    .size(24.0),
                );
                if ui.add(play_button).clicked() {
                    if sink.empty() {
                        self.actions.push(Action::PlayFile);
                    } else if sink.is_paused() {
                        sink.play();
                    } else {
                        sink.pause();
                    }
                }

                let stop_button = Button::new(RichText::new(icons::ICON_STOP.codepoint).size(24.0));
                if ui.add(stop_button).clicked() {
                    sink.stop();
                }
            });

            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.scope(|ui| {
                    ui.spacing_mut().interact_size.y = 24.0;
                    ui.spacing_mut().slider_rail_height = 24.0;

                    ui.spacing_mut().slider_width = ui.available_width();
                    let progress_slider =
                        Slider::new(&mut self.audio_position, 0..=self.audio_duration)
                            .step_by(1.0)
                            .show_value(false)
                            .trailing_fill(true);
                    if ui
                        .add_enabled(!sink.empty(), progress_slider)
                        .drag_stopped()
                    {
                        let new_position = std::time::Duration::from_secs(self.audio_position);
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
