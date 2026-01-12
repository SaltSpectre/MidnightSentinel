# Midnight Sentinel: Protect your OLED investment!

<div align="center">
  <img src="assets/logo/gh_logo.png" alt="Midnight Sentinel Logo">
</div>

## Description

Midnight Sentinel is a simple app that creates pure black, full-screen overlays on all active displays. The intended purpose is to protect OLED screens from burn-in* should you need to step away while leaving your computer running.

### Isn't that what a screensaver is for?

Sure, but screensavers are a deprecated feature of Windows, and while they still exist, they are subject to removal at Microsoft's discretion. Further, Midnight Sentinel is dismissed by double-clicking the mouse; this protects the process from being disrupted by curious, roaming cats.

### Can't I just turn off my monitors?

Yep, that's an option. The power buttons on my monitors are annoying to operate, and that's why I made this app. Additionally, you typically can't just turn off a laptop monitor if you're lucky enough to have one with an OLED screen.

> *\*OLED screens don't actually burn-in. The individual pixels have a maximum luminance that diminishes with use; the brighter the pixel, the faster it diminishes. With prolonged illumination in certain areas, pixels will diminish unevenly causing a burn-in-like effect.*

## Screenshot

<div align="center">
  <img src="assets/Screenshot.png" alt="Simulated black screen">
</div>

jkjk. This is just a pure-black image. But it's similar to the pure-black overlay that will be placed over your screen!

## Installation

### Option 1: Download MSI Installer (Recommended)

Download the latest MSI installer for your system architecture:

- **x64 (Intel/AMD 64-bit)**: `MidnightSentinel-x64.msi`
- **ARM64 (ARM-based systems)**: `MidnightSentinel-arm64.msi`

The MSI installer will:
- Install Midnight Sentinel to `%LOCALAPPDATA%\Midnight Sentinel` (no admin required)
- Add the installation directory to your user PATH (so you can run `midsent` from anywhere)
- Create Start Menu shortcuts
- Allow easy uninstallation via Windows Settings

### Option 2: Build from Source

Requirements:
- .NET 9.0 SDK or later
- Windows 10/11

```powershell
# Clone the repository
git clone https://github.com/SaltSpectre/MidnightSentinel.git
cd MidnightSentinel

# Build the application
cd src
dotnet build --configuration Release
```

The compiled executable will be in `src/bin/Release/net9.0-windows/`.

## Usage

Midnght Sentinel has a couple of ways you can use it.

### Standard Mode

First, is by executing the application and double-clicking on the System Tray icon to initiate the overlays. Double-click again (anywhere!--You won't see your mouse because it's hidden) and the overlays are dismissed. The app will continue to live in your tray to be called at your leisure.

### Advanced Mode (Command Line)

When calling `midsent` (or `midsent.exe`) with the `--run-now` argument, the overlays will immediately initiate. Dismissal is the same (double-clicking), but the app won't continue running after the overlays are dismissed. This is intended for programatic use (such as with AutoHotKey) or if your keyboard, mouse, or other device can be programmed to execute an application.

```cmd
midsent --run-now
```

## Conclusion

That's it! It's a very simple app for a very specific purpose.

## Contributing

I am always open to feedback and ideas. Feel free to create an issue, or, if you're feeling a little frisky, make a pull request! (I don't bite.)

## License

Midnight Sentinel is proudly open source under the MIT License.
