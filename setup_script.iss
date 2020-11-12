[Setup]
AppId={{47B10700-5CC0-4AD3-8EE1-E84C1885C05F}
AppName=Remember0416
AppVersion=0.2a
AppPublisher=REGENTAG
AppPublisherURL=https://github.com/Regentag/Remember0416
AppSupportURL=https://github.com/Regentag/Remember0416
AppUpdatesURL=https://github.com/Regentag/Remember0416/releases
DefaultDirName={pf64}\Remember0416
DefaultGroupName=Remember0416
DisableProgramGroupPage=yes
OutputBaseFilename=0416setup
Compression=lzma
SolidCompression=yes
DisableDirPage=yes
AllowUNCPath=False
ShowLanguageDialog=no
AppContact=https://github.com/Regentag/Remember0416/issues
UninstallDisplaySize=42
UninstallDisplayIcon={app}\remember0416.exe
DisableReadyPage=True
DisableReadyMemo=True
DisableFinishedPage=True
UsePreviousSetupType=False
UsePreviousTasks=False
UsePreviousLanguage=False
SetupIconFile=ribbon.ico
UninstallDisplayName=Remember0416
ArchitecturesInstallIn64BitMode=x64
ArchitecturesAllowed=x64

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Files]
Source: "0416.exe"; DestDir: "{app}"; DestName: "remember0416.exe"; Flags: ignoreversion
Source: "ribbon.ico"; DestDir: "{app}"; DestName: "ribbon.ico"; Flags: ignoreversion

[Run]
Filename: "{app}\remember0416.exe"; WorkingDir: "{app}"; Flags: nowait postinstall; Description: "Run after installation"

[Registry]
;current user only
Root: HKCU; Subkey: "Software\Microsoft\Windows\CurrentVersion\Run"; ValueType: string; ValueName: "Remember0416"; ValueData: "{app}\remember0416.exe";

[UninstallRun]
Filename: "taskkill.exe"; Parameters: "/F /IM remember0416.exe"; WorkingDir: "{app}"; Flags: waituntilterminated skipifdoesntexist runhidden
