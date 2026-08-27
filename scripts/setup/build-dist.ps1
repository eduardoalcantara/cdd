# Wrapper para compilar o cdd e gerar o pacote de distribuição autônomo (zip) para Windows

param (
    [Alias("h")]
    [switch]$Help
)

$ErrorActionPreference = "Stop"

if ($Help) {
    Write-Host "Usage: .\scripts\setup\build-dist.ps1"
    Write-Host ""
    Write-Host "Generates dist\cdd-windows-x86_64.zip and its SHA-256 checksum."
    exit 0
}

$ScriptPath = if ($MyInvocation.MyCommand.Path) { $MyInvocation.MyCommand.Path } else { $PSCommandPath }
$ScriptDir = Split-Path -Parent $ScriptPath
$RepoRoot = Resolve-Path (Join-Path $ScriptDir "..\..")

Write-Host "Compiling cdd in release mode..." -ForegroundColor Yellow
Set-Location (Join-Path $RepoRoot "core")
cargo build --release

$DistDir = Join-Path $RepoRoot "dist"
$PkgDir = Join-Path $DistDir "cdd-windows-x86_64"

Write-Host "Preparing distribution directory..." -ForegroundColor Yellow
if (!(Test-Path $DistDir)) { New-Item -ItemType Directory -Force -Path $DistDir | Out-Null }
if (Test-Path $PkgDir) { Remove-Item -Recurse -Force $PkgDir }
New-Item -ItemType Directory -Force -Path $PkgDir | Out-Null

# Copiando artefatos
Copy-Item (Join-Path $RepoRoot "core\target\release\cdd.exe") -Destination $PkgDir -Force
Copy-Item (Join-Path $RepoRoot "scripts\shell\cdd.ps1") -Destination $PkgDir -Force
Copy-Item (Join-Path $RepoRoot "scripts\setup\install-user.ps1") -Destination (Join-Path $PkgDir "install.ps1") -Force
Copy-Item (Join-Path $RepoRoot "scripts\setup\install-user.cmd") -Destination (Join-Path $PkgDir "install.cmd") -Force
Copy-Item (Join-Path $RepoRoot "readme.md") -Destination (Join-Path $PkgDir "README.md") -Force
Copy-Item (Join-Path $RepoRoot "docs\HOW_TO_USE.md") -Destination $PkgDir -Force
Copy-Item (Join-Path $RepoRoot "docs\HOW_TO_INSTALL.md") -Destination $PkgDir -Force
Copy-Item (Join-Path $RepoRoot "docs\HOW_IT_WORKS.md") -Destination $PkgDir -Force

$ZipPath = Join-Path $DistDir "cdd-windows-x86_64.zip"
$ChecksumPath = "$ZipPath.sha256"
if (Test-Path $ZipPath) { Remove-Item -Force $ZipPath }
if (Test-Path $ChecksumPath) { Remove-Item -Force $ChecksumPath }

Write-Host "Compressing package cdd-windows-x86_64.zip..." -ForegroundColor Yellow
Compress-Archive -Path (Join-Path $PkgDir "*") -DestinationPath $ZipPath
$Hash = (Get-FileHash -Algorithm SHA256 -Path $ZipPath).Hash.ToLower()
"$Hash  $(Split-Path -Leaf $ZipPath)" | Set-Content -Encoding ascii -Path $ChecksumPath
Remove-Item -Recurse -Force $PkgDir

Write-Host "Build complete! Package generated at: $ZipPath" -ForegroundColor Green
Write-Host "Checksum generated at: $ChecksumPath" -ForegroundColor Green
