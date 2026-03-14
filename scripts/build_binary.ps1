# Build a self-contained peakrdl binary using PyInstaller.
#
# Usage:
#   .\scripts\build_binary.ps1                        # output to dist\
#   .\scripts\build_binary.ps1 -OutputDir C:\tmp\out
#
# The resulting binary will be at <output-dir>/peakrdl.exe.
# A SHA-256 checksum file is written alongside it.

param(
    [string]$OutputDir = "dist"
)

$ErrorActionPreference = "Stop"

New-Item -ItemType Directory -Force -Path $OutputDir | Out-Null
$BinaryName = "peakrdl-rust.exe"

Write-Host "Building peakrdl binary..."
Write-Host "  Output dir  : $OutputDir"

uv run --with pyinstaller pyinstaller `
    --onefile `
    --name $BinaryName `
    --distpath $OutputDir `
    --workpath "$OutputDir\.build" `
    --specpath "$OutputDir\.build" `
    --collect-all peakrdl_rust `
    --collect-all peakrdl `
    --collect-all systemrdl `
    --copy-metadata peakrdl-rust `
    scripts\entrypoint.py

if ($LASTEXITCODE -ne 0) {
    Write-Error "PyInstaller failed"
    exit 1
}

$Binary = Join-Path $OutputDir $BinaryName

Write-Host ""
Write-Host "Binary built: $Binary"

# Write checksum.
$Hash = (Get-FileHash $Binary -Algorithm SHA256).Hash.ToLower()
$ChecksumLine = "$Hash  peakrdl-rust.exe"
$ChecksumLine | Out-File -Encoding ASCII "$Binary.sha256"
Write-Host $ChecksumLine
