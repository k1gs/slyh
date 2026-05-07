mod logic;
mod ui;

use anyhow::{Result, anyhow};
use eframe::{Frame, HardwareAcceleration, NativeOptions};
use egui::{Context, Ui, ViewportBuilder, vec2};
use egui_notify::Toasts;
use rodio::{MixerDeviceSink, Player};
use std::path::PathBuf;

// This list does not represent all supported formats, just the ones that will be shown in the file dialog.
const SUPPORTED_AUDIO_FORMATS: &[&str] = &["mp3", "wav", "flac", "ogg", "aac", "opus"];

#[derive(Eq, PartialEq)]
enum Action {
    InitAudioPlayer,
    OpenFile,
    PlayFile,
}

struct Application {
    file_path: Option<PathBuf>,
    actions: Vec<Action>,

    audio_player_initialized: bool,
    audio_duration: u64,
    audio_handle: Option<MixerDeviceSink>,
    audio_sink: Option<Player>,

    volume_before_mute: f32,

    toasts: Toasts,
}

impl Application {
    fn new(file_path: Option<PathBuf>) -> Self {
        let mut actions = vec![Action::InitAudioPlayer];
        if file_path.is_some() {
            actions.push(Action::PlayFile);
        }

        Self {
            file_path,
            actions,
            audio_player_initialized: false,
            audio_duration: 0,
            audio_handle: None,
            audio_sink: None,
            volume_before_mute: 1.0,
            toasts: Toasts::default(),
        }
    }
}

impl eframe::App for Application {
    fn ui(&mut self, ui: &mut Ui, frame: &mut Frame) {
        self._ui(ui, frame);
    }

    fn logic(&mut self, ctx: &Context, frame: &mut Frame) {
        self._logic(ctx, frame);
    }
}

pub fn run_gui(file_path: Option<PathBuf>) -> Result<()> {
    let options = NativeOptions {
        vsync: true,
        centered: true,
        hardware_acceleration: HardwareAcceleration::Preferred,
        viewport: ViewportBuilder::default()
            .with_app_id("ru.arabianq.slyh")
            .with_title("Slyh - Audio Player")
            .with_inner_size(vec2(600.0, 100.0))
            .with_min_inner_size(vec2(300.0, 100.0)),
        ..Default::default()
    };

    match eframe::run_native(
        "Slyh",
        options,
        Box::new(|cc| {
            egui_material_icons::initialize(&cc.egui_ctx);
            Ok(Box::new(Application::new(file_path)))
        }),
    ) {
        Ok(_) => Ok(()),
        Err(e) => Err(anyhow!("Failed to run GUI: {}", e)),
    }
}
