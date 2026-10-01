# Midnight Sentinel: Protect your OLED investment!

<div align="center">
  <img src="assets/logo/gh_logo.png" alt="Midnight Sentinel Logo">
</div>

## Description

Midnight Sentinel is a simple app that creates pure black, full-screen overlays on all active displays. The intended purpose is to protect OLED screens from burn-in* should you need to step away while leaving your computer running.

> [!NOTE]\
> **AI Disclosure:** This project unapologetically uses AI-assisted development. AI is used as part of the coding process, but the resulting code is reviewed, evaluated, and tested by human eyes and hands before being published to GitHub. No, I did not simply ask a chatbot for an EXE and upload whatever fell out.
>
> I understand that AI-assisted code is a nonstarter for some users, so I would rather disclose that plainly. Midnight Sentinel is also intentionally tiny in scope: it requires no administrative privileges and does not read, inspect, or otherwise access files on disk or data in memory. Its grand technological ambition is, quite literally, to make your screens black.
>
> Ultimately, this is a personal pet project I built to protect my own OLED monitors when I need to leave my computer unattended and putting the system to sleep isn't an option.

### Isn't that what a screensaver is for?

Sure, but screensavers are a legacy feature of Windows, and while they still exist, they are subject to removal at Microsoft's discretion. Further, Midnight Sentinel is dismissed by double-clicking the mouse; this protects the process from being disrupted by curious, roaming cats.

### Can't I just turn off my monitors?

Yep, that's an option. The power buttons on my monitors are annoying to operate, and that's why I made this app. Additionally, you typically can't just turn off a laptop monitor if you're lucky enough to have one with an OLED screen.

> [!TIP]
> OLED screens don't actually burn-in. The individual pixels have a maximum luminance that diminishes with use; the brighter the pixel, the faster it diminishes. With prolonged illumination in certain areas, pixels will diminish unevenly causing a burn-in-like effect.

## Screenshot

<div align="center">
  <img src="assets/Screenshot.png" alt="Simulated black screen">
</div>

jkjk. This is just a pure-black image. But it's similar to the pure-black overlay that will be placed over your screen!

## Installation

### Option 1: Download Installer (Recommended)

Download `MidnightSentinel-Universal-{version}.exe` from the [latest release](https://github.com/SaltSpectre/MidnightSentinel/releases/latest) and run it. It's a single installer covering both x64 and ARM64 systems — it detects your system's architecture and installs the matching binaries automatically.

The installer will:
- Install Midnight Sentinel to `%LOCALAPPDATA%\SaltSpectre\Midnight Sentinel` (no admin required)
- Create Start Menu shortcuts
- Optionally add the installation directory to your user PATH (selected by default)
- Optionally start Midnight Sentinel automatically at logon (selected by default)
- Allow easy uninstallation via Windows Settings

> [!WARNING]
> If you have a version from **26.1.15+24 or earlier** installed (built with the old MSI-based installer), uninstall it first via Settings > Apps before installing this version. The new installer uses different installer technology and can't upgrade or remove the old one automatically — it will warn you if it detects the old install still present, but can't do anything about it for you.

### Option 2: Portable (No Installer)

Prefer not to install anything? Download the portable ZIP for your architecture from the [latest release](https://github.com/SaltSpectre/MidnightSentinel/releases/latest):

- `MidnightSentinel-Portable-x64-{version}.zip`
- `MidnightSentinel-Portable-arm64-{version}.zip`

Unzip it anywhere and run `midsent.exe`/`midsentcli.exe` directly from there. No install, no PATH or registry changes, no admin — just the two executables, which you can delete whenever you like.

### Option 3: Build from Source

Requirements:
- [Rust](https://rustup.rs/) (stable toolchain)
- Windows 10/11

```powershell
# Clone the repository
git clone https://github.com/SaltSpectre/MidnightSentinel.git
cd MidnightSentinel

# Build both executables
cd src
cargo build --release --workspace
```
The compiled executables will be in `src/target/release/`:
- `midsent.exe` — the system tray controller
- `midsentcli.exe` — the standalone overlay

## Usage

Midnight Sentinel is split into two independent executables.

### `midsent.exe`: System Tray Controller

Run `midsent.exe` and it will live in your system tray. Left-double-click the tray icon to raise the overlays; double-click anywhere (any mouse button) on an overlay to dismiss it — you won't see your mouse because it's hidden. Right-click the tray icon for an About/Exit menu.

Calling `midsent.exe --run-now` skips the tray entirely and toggles the overlay directly (raises it if it's not active, dismisses it if it is), exiting once dismissed. This flag is kept for compatibility with earlier versions; new integrations should prefer `midsentcli.exe` instead.

### `midsentcli.exe`: Standalone Overlay

`midsentcli.exe` is a fully independent executable that can be launched directly by a script, a keyboard/mouse macro tool (such as AutoHotKey), or any third-party controller — with or without `midsent.exe` running. It accepts one optional, mutually exclusive flag:

- No flag, or `--toggle`: raise the overlay if it's not active, or dismiss it (same double-click-anywhere gesture) if it already is. Prints nothing beyond error messages.
- `--start`: make sure the overlay is active. Prints the PID of the process that owns it — either this one, or an already-running one — to stdout, so a script can monitor it.
- `--stop`: make sure the overlay is **not** active, dismissing it if needed. Prints human-readable feedback and exits with `0` if an overlay was found and dismissed, or non-zero if none was active.

```cmd
midsentcli.exe
midsentcli.exe --start
midsentcli.exe --stop
```

Only one overlay display can be active at a time, regardless of which of the three ways above (tray, `--run-now`, or `midsentcli.exe`) was used to raise or dismiss it.

## Conclusion

That's it! It's a very simple app for a very specific purpose.

## Contributing

I am always open to feedback and ideas. Feel free to create an issue, or, if you're feeling a little frisky, make a pull request! (I don't bite.)

## Acknowledgments

Dialogs in `midsent.exe` (About, warning) are built with [Slint](https://slint.dev/), used under its [Royalty-free license](https://github.com/slint-ui/slint/blob/master/LICENSES/LicenseRef-Slint-Royalty-free-2.0.md).

<div align="center">
  <img src="https://raw.githubusercontent.com/slint-ui/slint/master/logo/slint-logo-simple-light.svg" alt="Made with Slint" width="200">
</div>

## License

Midnight Sentinel is proudly open source under the MIT License.
