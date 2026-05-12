use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct LocalesConfig {
    pub force_locale: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EguiConfig {
    pub initial_window_size: (f32, f32),
    pub vsync: bool,
    pub centered: bool,
    pub hardware_acceleration: bool,
}

impl Default for EguiConfig {
    fn default() -> Self {
        Self {
            initial_window_size: (600.0, 100.0),
            vsync: true,
            centered: true,
            hardware_acceleration: true,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FontsConfig {
    pub scale: f32,
    pub use_system_fonts: bool,
    pub enable_hinting: bool,
}

impl Default for FontsConfig {
    fn default() -> Self {
        Self {
            scale: 1.0,
            use_system_fonts: true,
            enable_hinting: true,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AudioConfig {
    pub default_volume: f32,
    pub default_loop: bool,
}

impl Default for AudioConfig {
    fn default() -> Self {
        Self {
            default_volume: 1.0,
            default_loop: false,
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Config {
    pub locales: LocalesConfig,
    pub egui: EguiConfig,
    pub fonts: FontsConfig,
    pub audio: AudioConfig,
}

fn get_config_path() -> Result<PathBuf> {
    let config_dir = if let Some(config_dir) = dirs::config_dir() {
        config_dir
    } else {
        return Err(anyhow!("Config directory not found"));
    };

    let app_config_dir = config_dir.join(env!("CARGO_PKG_NAME"));
    if app_config_dir.exists() && !app_config_dir.is_dir() {
        return Err(anyhow!("{} is not a directory", app_config_dir.display()));
    }

    if !app_config_dir.exists() {
        fs::create_dir_all(&app_config_dir)?;
    }

    let config_path = app_config_dir.join("config.toml");

    Ok(config_path)
}

pub fn load_config() -> Result<Config> {
    let config_path = get_config_path()?;

    let config = if config_path.exists() {
        let config_data = fs::read(&config_path)?;
        let config: Config = toml::from_slice(&config_data)?;
        config
    } else {
        let config = Config::default();
        config
    };

    save_config(&config)?;

    Ok(config)
}

pub fn save_config(config: &Config) -> Result<()> {
    let config_path = get_config_path()?;
    let config_data = toml::to_string_pretty(&config)?;

    fs::write(&config_path, config_data)?;

    Ok(())
}
