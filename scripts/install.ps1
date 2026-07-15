# Open Design Lite installer (Windows).
# Usage: irm https://raw.githubusercontent.com/Bistu-OSSDT-2026/OpenDesignLite/master/scripts/install.ps1 | iex
# Downloads the latest release binary to %LOCALAPPDATA%\OpenDesignLite\bin\odl.exe.
# Spec: docs/specs/setup.md

$ErrorActionPreference = "Stop"

$Repo = "Bistu-OSSDT-2026/OpenDesignLite"
$Asset = "odl-windows-x64.exe"
$InstallDir = Join-Path $env:LOCALAPPDATA "OpenDesignLite\bin"
$Target = Join-Path $InstallDir "odl.exe"
$Url = "https://github.com/$Repo/releases/latest/download/$Asset"

Write-Host "Downloading $Asset from the latest release..."
New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
Invoke-WebRequest -Uri $Url -OutFile $Target -UseBasicParsing

Write-Host "Installed: $Target"
& $Target --version

# PATH guidance only - we do not modify the user's environment silently.
$onPath = ($env:Path -split ";") | Where-Object { $_.TrimEnd("\") -ieq $InstallDir.TrimEnd("\") }
if (-not $onPath) {
    Write-Host ""
    Write-Host "NOTE: $InstallDir is not on your PATH."
    Write-Host "Add it (one-time) with:"
    Write-Host "  [Environment]::SetEnvironmentVariable('Path', `$env:Path + ';$InstallDir', 'User')"
    Write-Host "or invoke the binary by full path."
}

Write-Host ""
Write-Host "Next step: wire up your coding agent with:"
Write-Host "  odl setup"
