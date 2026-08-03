@echo off
set "APP=%~dp0Error-Examiner.exe"
powershell -NoProfile -WindowStyle Hidden -Command ^
 "$s=(New-Object -ComObject WScript.Shell).CreateShortcut('%USERPROFILE%\Desktop\Error Examiner.lnk');$s.TargetPath='%APP%';$s.WorkingDirectory='%~dp0';$s.IconLocation='%APP%,0';$s.Save()"
