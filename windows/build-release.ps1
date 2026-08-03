$ErrorActionPreference = "Stop"
$root = Split-Path -Parent $PSScriptRoot
Set-Location $root

cargo build --release --bin error-examiner
if ($LASTEXITCODE -ne 0) { throw "Cargo release build failed." }

$package = Join-Path $root "dist\Error-Examiner-Windows-x86_64"
New-Item -ItemType Directory -Path $package -Force | Out-Null
Copy-Item "target\release\error-examiner.exe" `
    (Join-Path $package "Error-Examiner.exe") -Force
Copy-Item "Create Desktop Shortcut.cmd", "README.md", "LICENSE" `
    -Destination $package -Force
Copy-Item "fixtures\schema-guard\wrong-update-docker-buildkit.log" `
    -Destination $package -Force

$exe = Join-Path $package "Error-Examiner.exe"
$hash = (Get-FileHash $exe -Algorithm SHA256).Hash
Set-Content -Path "$exe.sha256" -Value "$hash  Error-Examiner.exe"
Write-Host "Built: $package"
