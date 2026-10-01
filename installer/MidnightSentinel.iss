#ifndef MyAppVersion
  #define MyAppVersion "0.0.0-dev"
#endif

#define MyAppName "Midnight Sentinel"
#define MyAppPublisher "SaltSpectre"
#define MyAppURL "https://github.com/SaltSpectre/MidnightSentinel"
#define MyAppExeName "midsent.exe"

[Setup]
; This GUID is permanent and must NEVER change: it's how Inno recognizes an
; existing installation (under this AppId) to upgrade in place. It is NOT
; related to the old MSI installer's UpgradeCode — those two installer
; technologies don't recognize each other's install records at all (see the
; legacy-MSI detection code below).
AppId={{963e3742-47e5-4799-91d8-49b0cbf2b67c}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
AppPublisher={#MyAppPublisher}
AppPublisherURL={#MyAppURL}
AppSupportURL={#MyAppURL}
AppUpdatesURL={#MyAppURL}
DefaultDirName={localappdata}\SaltSpectre\Midnight Sentinel
DefaultGroupName=Midnight Sentinel
DisableProgramGroupPage=yes
PrivilegesRequired=lowest
; Universal installer: ships both x64 and Arm64 binaries (see [Files] below)
; and runs on either architecture family.
ArchitecturesAllowed=x64compatible or arm64
ArchitecturesInstallIn64BitMode=x64compatible or arm64
OutputDir=bin
OutputBaseFilename=MidnightSentinel-Universal-{#MyAppVersion}
SetupIconFile=..\src\controller\resources\MidnightSentinel.ico
UninstallDisplayIcon={app}\{#MyAppExeName}
LicenseFile=License.rtf
Compression=lzma2
SolidCompression=yes
WizardStyle=modern

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "addtopath"; Description: "Add Midnight Sentinel to your PATH (lets you run midsent/midsentcli from a terminal)"
Name: "autostart"; Description: "Start Midnight Sentinel automatically at logon"

[Files]
; One binary set per architecture; Inno installs only the matching set.
Source: "..\publish\x64\midsent.exe"; DestDir: "{app}"; Check: not IsArm64
Source: "..\publish\x64\midsentcli.exe"; DestDir: "{app}"; Check: not IsArm64
Source: "..\publish\arm64\midsent.exe"; DestDir: "{app}"; Check: IsArm64
Source: "..\publish\arm64\midsentcli.exe"; DestDir: "{app}"; Check: IsArm64

[Icons]
Name: "{group}\Midnight Sentinel"; Filename: "{app}\{#MyAppExeName}"
Name: "{group}\Uninstall Midnight Sentinel"; Filename: "{uninstallexe}"

[Registry]
Root: HKCU; Subkey: "Software\Microsoft\Windows\CurrentVersion\Run"; ValueType: string; ValueName: "MidnightSentinel"; ValueData: """{app}\{#MyAppExeName}"""; Tasks: autostart; Flags: uninsdeletevalue

[Code]
const
  { Registry key the old MSI installer always wrote to (regardless of which
    optional features were selected), used here only to detect its presence. }
  LegacyMsiMarkerKey = 'Software\SaltSpectre\MidnightSentinel';
  LegacyMsiMarkerValue = 'installed';

  WM_SETTINGCHANGE_MSG = $001A;
  HWND_BROADCAST_MSG = $FFFF;
  SMTO_ABORTIFHUNG_FLAG = $0002;
  EnvironmentKey = 'Environment';

function SendMessageTimeoutA(hWnd: Longint; Msg: Longint; wParam: Longint;
  lParam: AnsiString; fuFlags, uTimeout: Longint; var lpdwResult: Longint): Longint;
  external 'SendMessageTimeoutA@user32.dll stdcall';

{ Warns (without blocking) if the old MSI-based release (version 26.1.15+24
  or earlier — i.e. any version built before this installer replaced it) is
  still installed, since this installer has no way to remove it and running
  both would leave duplicate Start Menu entries / PATH entries behind. }
function InitializeSetup(): Boolean;
begin
  Result := True;
  if RegValueExists(HKCU, LegacyMsiMarkerKey, LegacyMsiMarkerValue) then
  begin
    if MsgBox('An older MSI-based installation of Midnight Sentinel (version 26.1.15+24 or earlier) was detected.' + #13#10 + #13#10 +
              'This installer cannot remove it automatically, and having both installed at once can leave duplicate Start Menu entries and PATH entries behind.' + #13#10 + #13#10 +
              'It is strongly recommended to uninstall the old version first via Settings > Apps, then run this installer again.' + #13#10 + #13#10 +
              'Continue anyway?',
              mbConfirmation, MB_YESNO) = IDNO then
      Result := False;
  end;
end;

procedure RefreshEnvironment;
var
  ResultCode: Longint;
begin
  SendMessageTimeoutA(HWND_BROADCAST_MSG, WM_SETTINGCHANGE_MSG, 0, EnvironmentKey,
    SMTO_ABORTIFHUNG_FLAG, 5000, ResultCode);
end;

procedure EnvAddPath(Path: string);
var
  Paths: string;
begin
  if not RegQueryStringValue(HKCU, EnvironmentKey, 'Path', Paths) then
    Paths := '';

  if Pos(';' + Uppercase(Path) + ';', ';' + Uppercase(Paths) + ';') > 0 then
    exit;

  if (Paths <> '') and (Paths[Length(Paths)] <> ';') then
    Paths := Paths + ';';
  Paths := Paths + Path;

  if RegWriteStringValue(HKCU, EnvironmentKey, 'Path', Paths) then
    RefreshEnvironment;
end;

procedure EnvRemovePath(Path: string);
var
  Paths: string;
  P: Integer;
begin
  if not RegQueryStringValue(HKCU, EnvironmentKey, 'Path', Paths) then
    exit;

  P := Pos(';' + Uppercase(Path) + ';', ';' + Uppercase(Paths) + ';');
  if P = 0 then
    exit;

  Delete(Paths, P, Length(Path) + 1);

  if RegWriteStringValue(HKCU, EnvironmentKey, 'Path', Paths) then
    RefreshEnvironment;
end;

procedure CurStepChanged(CurStep: TSetupStep);
begin
  if (CurStep = ssPostInstall) and WizardIsTaskSelected('addtopath') then
    EnvAddPath(ExpandConstant('{app}'));
end;

procedure CurUninstallStepChanged(CurUninstallStep: TUninstallStep);
begin
  if CurUninstallStep = usPostUninstall then
    EnvRemovePath(ExpandConstant('{app}'));
end;
