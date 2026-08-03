$ErrorActionPreference = "Stop"

$identity = [Security.Principal.WindowsIdentity]::GetCurrent()
$principal = New-Object Security.Principal.WindowsPrincipal($identity)
$admin = [Security.Principal.WindowsBuiltInRole]::Administrator
if (-not $principal.IsInRole($admin)) {
    throw "Run PowerShell as Administrator."
}

powercfg.exe /hibernate off
if ($LASTEXITCODE -ne 0) { throw "Could not disable hibernation." }

Dism.exe /Online /Cleanup-Image /StartComponentCleanup
if ($LASTEXITCODE -ne 0) { throw "Windows component cleanup failed." }

Start-Process "ms-settings:storagesense"
Write-Host "Hibernation disabled and component store cleaned."
Write-Host "Review Temporary files manually; do not remove Downloads."
