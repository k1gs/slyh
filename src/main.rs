mod app;

use anyhow::Result;
use clap::Parser;
use rust_i18n::i18n;
use std::path::PathBuf;

i18n!("locales", fallback = "en");

#[derive(Parser)]
struct Cli {
    /// Path to audio file
    file_path: Option<PathBuf>,
}

fn main() -> Result<()> {
    let config = app::config::load_config()?;

    let preffered_locale = config
        .locales
        .force_locale
        .unwrap_or(sys_locale::get_locale().unwrap_or(String::from("en-US")));
    rust_i18n::set_locale(&preffered_locale);

    let args = Cli::parse();
    let file_path = args.file_path;

    app::gui::run_gui(file_path)
}
