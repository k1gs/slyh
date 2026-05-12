mod logic;
mod ui;

use crate::app::config::{Config, load_config};
use anyhow::{Result, anyhow};
use eframe::{Frame, HardwareAcceleration, NativeOptions};
use egui::{Context, FontData, FontDefinitions, FontFamily, FontTweak, Ui, ViewportBuilder, vec2};
use egui_notify::Toasts;
use lofty::file::FileType as LoftyFileType;
use rodio::{MixerDeviceSink, Player};
use std::{fs, path::PathBuf};
use system_fonts::{FontStyle, FoundFontSource, find_for_locale, find_for_system_locale};
use unicode_normalization::UnicodeNormalization;

// This list does not represent all supported formats, just the ones that will be shown in the file dialog.
const SUPPORTED_AUDIO_FORMATS: &[&str] = &["mp3", "wav", "flac", "ogg", "aac", "opus"];

#[derive(Eq, PartialEq)]
enum Action {
    InitAudioPlayer,
    OpenFile,
    ReadFileProps,
    PlayFile,
    StartNewInstance(PathBuf),
}

struct AudioProperties {
    duration: u64,
    position: u64,

    sample_rate: Option<u32>,
    bitrate: Option<u32>,
    channels: Option<u8>,
    format_type: Option<LoftyFileType>,
}

struct Application {
    file_path: Option<PathBuf>,
    file_path_normilized: Option<String>,

    actions: Vec<Action>,

    audio_props: AudioProperties,

    is_looped: bool,
    is_finished: bool,

    audio_player_initialized: bool,
    audio_handle: Option<MixerDeviceSink>,
    audio_sink: Option<Player>,

    volume_before_mute: f32,

    toasts: Toasts,
}

impl Application {
    fn new(file_path: Option<PathBuf>, config: Config) -> Self {
        let mut actions = vec![Action::InitAudioPlayer];
        let mut file_path_normilized = None;
        if let Some(fp) = &file_path {
            actions.push(Action::ReadFileProps);
            actions.push(Action::PlayFile);
            file_path_normilized = Some(fp.to_string_lossy().nfc().collect::<String>());
        }

        Self {
            file_path,
            file_path_normilized,
            actions,
            audio_props: AudioProperties {
                duration: 0,
                position: 0,
                sample_rate: None,
                bitrate: None,
                channels: None,
                format_type: None,
            },
            is_looped: config.audio.default_loop,
            is_finished: false,
            audio_player_initialized: false,
            audio_handle: None,
            audio_sink: None,
            volume_before_mute: config.audio.default_volume,
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

fn add_font(font_name: &str, font_bytes: &[u8], fonts: &mut FontDefinitions) -> Result<()> {
    let config = load_config()?;

    let font_data = FontData::from_owned(font_bytes.to_vec()).tweak(FontTweak {
        scale: config.fonts.scale,
        hinting_override: Some(config.fonts.enable_hinting),
        ..Default::default()
    });

    fonts
        .font_data
        .insert(font_name.to_owned(), font_data.into());

    fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .insert(0, font_name.to_owned());
    fonts
        .families
        .entry(FontFamily::Monospace)
        .or_default()
        .insert(0, font_name.to_owned());

    Ok(())
}

fn load_system_fonts(fonts: &mut FontDefinitions) -> Result<()> {
    let config = load_config()?;

    let sans_fonts = if let Some(locale) = &config.locales.force_locale {
        let (_, fonts) = find_for_locale(locale, FontStyle::Sans);
        fonts
    } else {
        let (_, _, fonts) = find_for_system_locale(FontStyle::Sans);
        fonts
    };
    let serif_fonts = if let Some(locale) = &config.locales.force_locale {
        let (_, fonts) = find_for_locale(locale, FontStyle::Serif);
        fonts
    } else {
        let (_, _, fonts) = find_for_system_locale(FontStyle::Serif);
        fonts
    };
    let system_fonts = [sans_fonts, serif_fonts].concat();

    for font in system_fonts.iter() {
        let font_bytes = match &font.source {
            FoundFontSource::Path(path) => fs::read(path)?,
            FoundFontSource::Bytes(bytes) => bytes.to_vec(),
        };

        add_font(&font.key, &font_bytes, fonts)?;
    }

    Ok(())
}

pub fn run_gui(file_path: Option<PathBuf>) -> Result<()> {
    let config = load_config()?;

    let options = NativeOptions {
        vsync: config.egui.vsync,
        centered: config.egui.centered,
        hardware_acceleration: if config.egui.hardware_acceleration {
            HardwareAcceleration::Preferred
        } else {
            HardwareAcceleration::Off
        },
        viewport: ViewportBuilder::default()
            .with_app_id("ru.arabianq.slyh")
            .with_title("Slyh - Audio Player")
            .with_inner_size(vec2(
                config.egui.initial_window_size.0,
                config.egui.initial_window_size.1,
            ))
            .with_min_inner_size(vec2(300.0, 100.0))
            .with_drag_and_drop(true),
        ..Default::default()
    };

    match eframe::run_native(
        "Slyh",
        options,
        Box::new(|cc| {
            let mut fonts = FontDefinitions::default();

            egui_material_icons::initialize(&cc.egui_ctx);

            add_font(
                "slyh_custom_font",
                include_bytes!("../../../assets/Curtsweeper-Regular.otf"),
                &mut fonts,
            )?;
            if config.fonts.use_system_fonts {
                load_system_fonts(&mut fonts)?;
            }

            cc.egui_ctx.set_fonts(fonts);

            Ok(Box::new(Application::new(file_path, config)))
        }),
    ) {
        Ok(_) => Ok(()),
        Err(e) => Err(anyhow!("Failed to run GUI: {}", e)),
    }
}
