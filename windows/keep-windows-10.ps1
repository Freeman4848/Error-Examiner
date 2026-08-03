$ErrorActionPreference = "Stop"

$identity = [Security.Principal.WindowsIdentity]::GetCurrent()
$principal = New-Object Security.Principal.WindowsPrincipal($identity)
$admin = [Security.Principal.WindowsBuiltInRole]::Administrator
if (-not $principal.IsInRole($admin)) {
    throw "Run PowerShell as Administrator."
}

$policy = "HKLM:\SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate"
New-Item -Path $policy -Force | Out-Null
New-ItemProperty -Path $policy -Name ProductVersion `
    -PropertyType String -Value "Windows 10" -Force | Out-Null
New-ItemProperty -Path $policy -Name TargetReleaseVersion `
    -PropertyType DWord -Value 1 -Force | Out-Null
New-ItemProperty -Path $policy -Name TargetReleaseVersionInfo `
    -PropertyType String -Value "22H2" -Force | Out-Null

Write-Host "Pinned to Windows 10 22H2. Reboot the VM before checking updates."
