# install_stop_nagging.ps1
# Install Stop-Nagging on Windows via PowerShell
param(
    [string]$InstallDir = "$HOME\.local\bin"
)

# Exit on error
$ErrorActionPreference = "Stop"

Write-Host "Stop-Nagging Windows Installer"

if (!(Test-Path -Path $InstallDir)) {
    New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
}

Write-Host "Selected install directory: $InstallDir"

# Detect architecture
$arch = $ENV:PROCESSOR_ARCHITECTURE
switch ($arch) {
    "AMD64" { $target = "x86_64-pc-windows-msvc" }
    "ARM64" { $target = "aarch64-pc-windows-msvc" }
    default {
        Write-Host "Unsupported or unknown architecture: $arch"
        Write-Host "Please build from source or check for a compatible artifact."
        exit 1
    }
}

$repoOwner = "bodo-run"
$repoName  = "stop-nagging"
$assetName = "stop-nagging-$target.zip"

Write-Host "OS/ARCH => Windows / $arch"
Write-Host "Asset name => $assetName"

Write-Host "Fetching latest release info from GitHub..."
$releasesUrl  = "https://api.github.com/repos/$repoOwner/$repoName/releases/latest"
try {
    $releaseData = Invoke-RestMethod -Uri $releasesUrl
} catch {
    Write-Host "Failed to fetch release info from GitHub."
    Write-Host "Please build from source or check back later."
    exit 0
}

# Find the asset download URL
$asset = $releaseData.assets | Where-Object { $_.name -eq $assetName }
if (!$asset) {
    Write-Host "Failed to find an asset named $assetName in the latest release."
    Write-Host "Check that your OS/ARCH is built or consider building from source."
    exit 0
}

$downloadUrl = $asset.browser_download_url
Write-Host "Downloading from: $downloadUrl"

$zipPath = Join-Path $env:TEMP $assetName
Invoke-WebRequest -Uri $downloadUrl -OutFile $zipPath -UseBasicParsing

Write-Host "Extracting archive..."
$extractDir = Join-Path $env:TEMP "stop-nagging-$($arch)"
if (Test-Path $extractDir) {
    Remove-Item -Recurse -Force $extractDir
}
Expand-Archive -Path $zipPath -DestinationPath $extractDir

Write-Host "Moving binary to $InstallDir..."
$binaryPath = Join-Path $extractDir "stop-nagging-$target" "stop-nagging.exe"
if (!(Test-Path $binaryPath)) {
    Write-Host "stop-nagging.exe not found in the extracted folder."
    exit 1
}
Move-Item -Force $binaryPath $InstallDir

Write-Host "Cleanup temporary files..."
Remove-Item -Force $zipPath
Remove-Item -Recurse -Force $extractDir

Write-Host "Installation complete!"

# Check if $InstallDir is in PATH
$pathDirs = $ENV:PATH -split ";"
if ($pathDirs -notcontains (Resolve-Path $InstallDir)) {
    Write-Host "NOTE: $InstallDir is not in your PATH. Add it by running something like:"
    Write-Host "`$env:Path += `";$(Resolve-Path $InstallDir)`""
    Write-Host "Or update your system's environment variables to persist this."
}

Write-Host "Now you can run: stop-nagging --help" 