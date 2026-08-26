# Wrapper para compilar o cdd e gerar o pacote de distribuição autônomo (zip) para Windows

$ErrorActionPreference = "Stop"

$ScriptPath = if ($MyInvocation.MyCommand.Path) { $MyInvocation.MyCommand.Path } else { $PSCommandPath }
$ScriptDir = Split-Path -Parent $ScriptPath
$RepoRoot = Resolve-Path (Join-Path $ScriptDir "..\..")

Write-Host "Compilando cdd em modo release..." -ForegroundColor Yellow
Set-Location (Join-Path $RepoRoot "core")
cargo build --release

$DistDir = Join-Path $RepoRoot "dist"
$PkgDir = Join-Path $DistDir "cdd-windows-x86_64"

Write-Host "Preparando diretorio de distribuicao..." -ForegroundColor Yellow
if (!(Test-Path $DistDir)) { New-Item -ItemType Directory -Force -Path $DistDir | Out-Null }
if (Test-Path $PkgDir) { Remove-Item -Recurse -Force $PkgDir }
New-Item -ItemType Directory -Force -Path $PkgDir | Out-Null

# Copiando artefatos
Copy-Item (Join-Path $RepoRoot "core\target\release\cdd.exe") -Destination $PkgDir -Force
Copy-Item (Join-Path $RepoRoot "scripts\shell\cdd.ps1") -Destination $PkgDir -Force
Copy-Item (Join-Path $RepoRoot "scripts\setup\install-user.ps1") -Destination (Join-Path $PkgDir "install.ps1") -Force

$ZipPath = Join-Path $DistDir "cdd-windows-x86_64.zip"
if (Test-Path $ZipPath) { Remove-Item -Force $ZipPath }

Write-Host "Compactando o pacote cdd-windows-x86_64.zip..." -ForegroundColor Yellow
Compress-Archive -Path (Join-Path $PkgDir "*") -DestinationPath $ZipPath
Remove-Item -Recurse -Force $PkgDir

Write-Host "Build concluido! Pacote gerado em: $ZipPath" -ForegroundColor Green
