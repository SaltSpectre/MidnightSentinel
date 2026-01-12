# Midnight Sentinel MSI Installer

This directory contains the WiX Toolset configuration for building MSI installers for Midnight Sentinel.

## Automated Builds

MSI installers are automatically built by GitHub Actions for both x64 and ARM64 architectures whenever:

- Code is pushed to `main` or `develop` branches
- A pull request is opened against `main`
- A tag starting with `v` is created (e.g., `v1.0.0`)
- The workflow is manually triggered

### Build Artifacts

The workflow produces:
- `MidnightSentinel-x64.msi` - Installer for x64 (64-bit Intel/AMD) systems
- `MidnightSentinel-arm64.msi` - Installer for ARM64 systems

Artifacts are:
- Uploaded to GitHub Actions (retained for 90 days)
- Automatically attached to GitHub releases when a version tag is pushed

## Manual Building

### Prerequisites

1. **.NET 9.0 SDK** - Download from https://dot.net
2. **WiX Toolset v4** - Installed automatically via dotnet tool

### Build Steps

```powershell
# 1. Restore and build the application
dotnet restore ../src/MidnightSentinel.sln
dotnet build ../src/MidnightSentinel.sln --configuration Release

# 2. Publish for target architecture (x64 or arm64)
dotnet publish ../src/MidnightSentinel.csproj --configuration Release --runtime win-x64 --self-contained true -p:PublishSingleFile=false --output ../publish/x64

# 3. Install WiX Toolset
dotnet tool install --global wix --version 4.0.5
wix extension add WixToolset.Heat.wixext/4.0.5
wix extension add WixToolset.Util.wixext/4.0.5
wix extension add WixToolset.UI.wixext/4.0.5

# 4. Harvest published files
wix heat dir ../publish/x64 -cg PublishedFiles -dr INSTALLFOLDER -gg -sfrag -srd -out HarvestedFiles.wxs

# 5. Build MSI
wix build -arch x64 -ext WixToolset.Util.wixext -ext WixToolset.UI.wixext -out MidnightSentinel-x64.msi Product.wxs HarvestedFiles.wxs
```

For ARM64, replace `x64` with `arm64` in steps 2, 4, and 5.

## Installer Features

- **Install Location**: `C:\Program Files\Midnight Sentinel\`
- **Start Menu Shortcuts**: Application launcher and uninstaller
- **Upgrade Support**: Automatically upgrades previous versions
- **Clean Uninstall**: Removes all files and registry entries
- **Per-Machine Installation**: Available to all users on the system

## Configuration Files

- **Product.wxs**: Main installer configuration
  - Product information (name, version, manufacturer)
  - Directory structure
  - Start menu shortcuts
  - Upgrade logic

- **HarvestedFiles.wxs**: Auto-generated file list
  - Created by WiX Heat tool
  - Contains all files from the publish directory
  - Regenerated for each build

## Versioning

The installer version is currently set to `1.0.0.0` in Product.wxs. Update this for new releases:

```xml
<?define ProductVersion = "1.0.0.0" ?>
```

## Upgrade Code

The `UpgradeCode` GUID in Product.wxs is permanent and should **NEVER** be changed:

```xml
<?define UpgradeCode = "A9E8B7C6-5D4E-3F2A-1B0C-9D8E7F6A5B4C" ?>
```

This GUID links all versions of the product together for upgrade detection. Changing it will prevent automatic upgrades.

## Troubleshooting

### Build Fails: "Cannot find file"

Ensure the publish directory exists and contains the application files:
```powershell
dir ../publish/x64
```

### Heat Fails: "Directory not found"

The publish directory must be created before running heat:
```powershell
dotnet publish ../src/MidnightSentinel.csproj --output ../publish/x64
```

### MSI Install Fails

Check Windows Event Viewer → Windows Logs → Application for detailed error messages.

## References

- [WiX Toolset v4 Documentation](https://wixtoolset.org/docs/v4/)
- [.NET 9.0 Publishing](https://learn.microsoft.com/dotnet/core/deploying/)
- [GitHub Actions for .NET](https://docs.github.com/actions/guides/building-and-testing-net)
