# Slyh 🎵

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Slyh is a simple, lightweight, and cross-platform audio player built with Rust and [egui](https://github.com/emilk/egui). It focuses on providing a minimal yet powerful user interface for playing local audio files.

## Features ✨

- **Cross-Platform:** Works on Linux, Windows, and macOS.
- **Minimal UI:** Clean, intuitive interface powered by `egui`.
- **Lightweight:** Built in Rust for high performance and low resource usage.
- **Localization (i18n):** Automatically detects your system locale or can be configured manually.
- **Audio Support:** Uses `rodio` for playback and `lofty` for parsing metadata.

## Installation 📦

### Windows
Download .exe or .msi from the [releases page](https://github.com/arabianq/slyh/releases).

### macOS
Download .dmg from the [releases page](https://github.com/arabianq/slyh/releases).


### Linux

Slyh provides several ways to install on Linux depending on your distribution.

**Arch Linux (AUR)**
You can install `slyh` from the AUR using your preferred helper (e.g., `yay` or `paru`):
```bash
yay -S slyh
# or
yay -S slyh-bin
```

**Flatpak**
```bash
flatpak remote-add --user --if-not-exists slyh-repo https://arabianq.github.io/slyh/index.flatpakrepo

flatpak install --user arabianq-repo ru.arabianq.slyh//stable

# Or nightly version
flatpak install --user arabianq-repo ru.arabianq.slyh//nightly
```

**COPR Repository for Fedora and it's derivatives**
```bash
sudo dnf copr enable arabianq/slyh
sudo dnf install slyh
```

**Ubuntu, Debian, etc.**

Download .deb package from [releases page](https://github.com/arabianq/slyh/releases).


### Building from Source

To build from source, make sure you have [Rust and Cargo](https://rustup.rs/) installed along with the required dependencies (e.g., system libraries for audio and UI).

1. Clone the repository:
   ```bash
   git clone https://github.com/arabianq/slyh.git
   cd slyh
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. The compiled binary will be available at `target/release/slyh`.

## Usage 🚀

Launch Slyh from your application menu, or run it directly from the terminal. 

To open a specific file immediately upon launch:
```bash
slyh /path/to/your/audio_file.mp3
```

## Configuration ⚙️

Configuration files are automatically created on the first launch. They are stored in your system's default configuration directory (e.g., `~/.config/slyh/` on Linux). You can adjust settings like the application language (`force_locale`) through the config file.


## License 📄

This project is licensed under the [MIT License](LICENSE).