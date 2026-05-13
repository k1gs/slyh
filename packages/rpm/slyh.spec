%bcond check 1

# prevent library files from being installed
%global cargo_install_lib 0

Name:            slyh
Version:         1.0.0
Release:         %autorelease
Summary:         Simple yet powerful audio player

License:         MIT

URL:             https://github.com/arabianq/slyh
Source:          https://github.com/arabianq/slyh/archive/refs/tags/v%{version}.tar.gz

BuildRequires: rust
BuildRequires: cargo
BuildRequires: clang-devel
BuildRequires: cmake
BuildRequires: alsa-lib-devel

%global _description %{expand:
Slyh is a simple audio player built with Rust and EGUI. It is focused on minimal yet powerful UI.}

%description %{_description}

%prep
%autosetup -n slyh-%{version} -p1

%build
cargo build --release --locked

%install
install -Dm755 target/release/slyh %{buildroot}%{_bindir}/slyh

install -Dm644 assets/slyh.desktop %{buildroot}%{_datadir}/applications/slyh.desktop
install -Dm644 assets/icon.png %{buildroot}%{_datadir}/icons/hicolor/128x128/apps/ru.arabianq.slyh.png

%files
%license LICENSE
%doc README.md
%{_bindir}/slyh
%{_datadir}/applications/slyh.desktop
%{_datadir}/icons/hicolor/128x128/apps/ru.arabianq.slyh.png

%changelog
%autochangelog
