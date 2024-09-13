#define VerFile = FileOpen("version.txt")
#define MyAppVersion = FileRead(VerFile)
#expr FileClose(VerFile)
#undef VerFile

#define TarFile FileOpen("target.txt")
#define MyTarget FileRead(TarFile)
#expr FileClose(TarFile)
#undef TarFile

#define MyAppName "VPM"
#define MyAppPublisher "Instachip"
#define MyAppURL "https://getinstachip.com/"

[Setup]
AppId={{E3D813B5-C9DB-4FC0-957C-9D06371B378E}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
AppPublisher={#MyAppPublisher}
AppPublisherURL={#MyAppURL}
AppSupportURL={#MyAppURL}
AppUpdatesURL={#MyAppURL}
CreateAppDir=yes
PrivilegesRequired=lowest
PrivilegesRequiredOverridesAllowed=dialog
Compression=lzma
SolidCompression=yes
WizardStyle=modern
DefaultDirName={autopf}\{#MyAppName}
DisableDirPage=no
DirExistsWarning=no
OutputBaseFilename=vpm-installer-{#MyAppVersion}-{#MyTarget}
OutputDir=.

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"
Name: "armenian"; MessagesFile: "compiler:Languages\Armenian.isl"
Name: "brazilianportuguese"; MessagesFile: "compiler:Languages\BrazilianPortuguese.isl"
Name: "bulgarian"; MessagesFile: "compiler:Languages\Bulgarian.isl"
Name: "catalan"; MessagesFile: "compiler:Languages\Catalan.isl"
Name: "corsican"; MessagesFile: "compiler:Languages\Corsican.isl"
Name: "czech"; MessagesFile: "compiler:Languages\Czech.isl"
Name: "danish"; MessagesFile: "compiler:Languages\Danish.isl"
Name: "dutch"; MessagesFile: "compiler:Languages\Dutch.isl"
Name: "finnish"; MessagesFile: "compiler:Languages\Finnish.isl"
Name: "french"; MessagesFile: "compiler:Languages\French.isl"
Name: "german"; MessagesFile: "compiler:Languages\German.isl"
Name: "hebrew"; MessagesFile: "compiler:Languages\Hebrew.isl"
Name: "hungarian"; MessagesFile: "compiler:Languages\Hungarian.isl"
Name: "icelandic"; MessagesFile: "compiler:Languages\Icelandic.isl"
Name: "italian"; MessagesFile: "compiler:Languages\Italian.isl"
Name: "japanese"; MessagesFile: "compiler:Languages\Japanese.isl"
Name: "korean"; MessagesFile: "compiler:Languages\Korean.isl"
Name: "norwegian"; MessagesFile: "compiler:Languages\Norwegian.isl"
Name: "polish"; MessagesFile: "compiler:Languages\Polish.isl"
Name: "portuguese"; MessagesFile: "compiler:Languages\Portuguese.isl"
Name: "russian"; MessagesFile: "compiler:Languages\Russian.isl"
Name: "slovak"; MessagesFile: "compiler:Languages\Slovak.isl"
Name: "slovenian"; MessagesFile: "compiler:Languages\Slovenian.isl"
Name: "spanish"; MessagesFile: "compiler:Languages\Spanish.isl"
Name: "turkish"; MessagesFile: "compiler:Languages\Turkish.isl"
Name: "ukrainian"; MessagesFile: "compiler:Languages\Ukrainian.isl"

[Files]
Source: "target\{#MyTarget}\release\vpm.exe"; DestDir: "{app}"; Flags: ignoreversion

[Tasks]
Name: addtopath; Description: "Add application directory to PATH"; Flags: checkedonce

[Code]
const
    ModPathName = 'modifypath';
    ModPathType = 'user';

function ModPathDir(): TArrayOfString;
begin
    SetArrayLength(Result, 1);
    Result[0] := ExpandConstant('{app}');
end;

procedure ModPath();
var
    oldpath: string;
    newpath: string;
    updatepath: boolean;
begin
    if not RegQueryStringValue(HKEY_CURRENT_USER, 'Environment', 'Path', oldpath) then
        oldpath := '';

    updatepath := true;

    if (Pos(';' + UpperCase(ExpandConstant('{app}')) + ';', ';' + UpperCase(oldpath) + ';') > 0) then
        updatepath := false
    else if (Pos(';' + UpperCase(ExpandConstant('{app}')) + '\;', ';' + UpperCase(oldpath) + ';') > 0) then
        updatepath := false;

    if (updatepath) then begin
        newpath := oldpath;
        if (Pos(';', oldpath) > 0) then
            newpath := newpath + ';' + ExpandConstant('{app}')
        else
            newpath := ExpandConstant('{app}');

        if RegWriteStringValue(HKEY_CURRENT_USER, 'Environment', 'Path', newpath) then
        begin
            StringChangeEx(oldpath, ';', #13#10, True);
            StringChangeEx(newpath, ';', #13#10, True);
            Log('Old PATH:' + #13#10 + oldpath);
            Log('New PATH:' + #13#10 + newpath);
            RegWriteStringValue(HKEY_CURRENT_USER, 'Environment', 'Path', newpath);
        end else
            Log('Error: Failed to modify PATH');
    end;
end;

procedure CurStepChanged(CurStep: TSetupStep);
begin
    if (CurStep = ssPostInstall) and WizardIsTaskSelected('addtopath') then
        ModPath();
end;

procedure CurUninstallStepChanged(CurUninstallStep: TUninstallStep);
var
    oldpath: string;
    newpath: string;
    pathArr: TArrayOfString;
    i: Integer;
begin
    if (CurUninstallStep = usUninstall) then begin
        if RegQueryStringValue(HKEY_CURRENT_USER, 'Environment', 'Path', oldpath) then begin
            oldpath := oldpath + ';';
            i := 0;
            while (Pos(';', oldpath) > 0) do begin
                SetArrayLength(pathArr, i + 1);
                pathArr[i] := Copy(oldpath, 0, Pos(';', oldpath) - 1);
                oldpath := Copy(oldpath, Pos(';', oldpath) + 1, Length(oldpath));
                i := i + 1;

                if (pathArr[i - 1] <> ExpandConstant('{app}')) then begin
                    if (newpath = '') then
                        newpath := pathArr[i - 1]
                    else
                        newpath := newpath + ';' + pathArr[i - 1];
                end;
            end;

            RegWriteStringValue(HKEY_CURRENT_USER, 'Environment', 'Path', newpath);
        end;
    end;
end;

[UninstallDelete]
Type: filesandordirs; Name: "{app}"