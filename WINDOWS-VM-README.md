# Error Examiner: Windows 10 VM

## Prepare Windows

Create a VM snapshot. Open Windows PowerShell as Administrator and run:

```powershell
Set-ExecutionPolicy -Scope Process Bypass
.\windows\keep-windows-10.ps1
.\windows\clean-test-vm.ps1
```

The first script pins feature updates to Windows 10 22H2. It does not disable
Windows Update. The cleanup disables hibernation, cleans superseded Windows
components, and opens Temporary files for manual review.

## Build

Install Visual Studio Build Tools with **Desktop development with C++** and a
Windows SDK. Install Rust with the default MSVC toolchain, then run:

```powershell
.\windows\build-release.ps1
```

The result is `dist\Error-Examiner-Windows-x86_64\Error-Examiner.exe`.

## Offline smoke test

1. Start the EXE and confirm one process owns both the window and tray.
2. Check minimize, maximize, close-to-tray, resize, pin, and window lock.
3. Check Ctrl+A/C/V/Z on English and Ukrainian or Russian keyboard layouts.
4. Insert the included Docker log and inspect Raw, Parsed, and Compare scroll.
5. In Schema, disable and re-enable one profile; verify the active count.
6. Select a non-Docker profile, press Update, and choose the Docker fixture.
   Error Examiner must reject it before any AI request.
7. Check themes, opacity down to 0.3, chat tabs, Clear, and tray restore.

AI provider tests may be skipped in an offline VM.
