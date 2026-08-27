# install.ps1 (Distribuição End-User)
# Script de instalação standalone para o pacote pré-compilado do cdd.

param (
    [switch]$Uninstall,
    [switch]$Quiet,
    [switch]$Force
)

$ErrorActionPreference = "Stop"

if (!$Quiet) {
    Clear-Host
    function Get-AppOemBoxChar([byte]$Code) {
        return [System.Text.Encoding]::GetEncoding(437).GetString([byte[]]@($Code))
    }
    $tl = Get-AppOemBoxChar 218; $tr = Get-AppOemBoxChar 191
    $bl = Get-AppOemBoxChar 192; $br = Get-AppOemBoxChar 217
    $h  = Get-AppOemBoxChar 196; $v  = Get-AppOemBoxChar 179

    $leftW = 18; $rightW = 56
    $ruleTop = $tl + ($h * $leftW) + $h + ($h * $rightW) + $tr
    $ruleBot = $bl + ($h * $leftW) + $h + ($h * $rightW) + $br
    
    Write-Host $ruleTop -ForegroundColor Cyan
    Write-Host "$v Projeto          $v cdd (Change Directory Directly)                        $v" -ForegroundColor Cyan
    if ($Uninstall) {
        Write-Host "$v Acao             $v Desinstalacao do End-User Package                      $v" -ForegroundColor Cyan
    } else {
        Write-Host "$v Acao             $v Instalacao do Binario Autonomo                         $v" -ForegroundColor Cyan
    }
    Write-Host $ruleBot -ForegroundColor Cyan
    Write-Host ""
}

$ScriptPath = if ($MyInvocation.MyCommand.Path) { $MyInvocation.MyCommand.Path } else { $PSCommandPath }
$ScriptDir = Split-Path -Parent $ScriptPath
$Dest = Join-Path $env:LOCALAPPDATA "cdd"
$ProfilePath = $PROFILE
$InstallMarker = "# CDD_INSTALL_MARKER"
$SourceCmd = ". `"$Dest\cdd.ps1`""

if ($Uninstall) {
    if (!$Quiet -and !$Force) {
        Write-Host "Aviso: Isso ira apagar os arquivos em $Dest e remover a entrada do seu perfil."
        $ans = Read-Host "Continuar? (1 = Sim / 0 = Nao) [1]"
        if ($ans -eq "" -or $ans -eq "1") { } else {
            Write-Host "Abortado."
            exit 0
        }
    }

    if (Test-Path $Dest) { Remove-Item -Recurse -Force $Dest }

    if (Test-Path $ProfilePath) {
        $content = Get-Content $ProfilePath
        $newContent = $content | Where-Object { $_ -notmatch $InstallMarker -and $_ -notmatch "\.cdd\\cdd\.ps1" }
        Set-Content -Path $ProfilePath -Value $newContent
        Write-Host "Desinstalacao concluida. Os arquivos e a injecao de perfil foram removidos." -ForegroundColor Green
    }
    exit 0
}

Write-Host "Copiando arquivos binarios para $Dest..." -ForegroundColor Yellow
if (!(Test-Path $Dest)) { New-Item -ItemType Directory -Force -Path $Dest | Out-Null }
Copy-Item (Join-Path $ScriptDir "cdd.exe") -Destination $Dest -Force
Copy-Item (Join-Path $ScriptDir "cdd.ps1") -Destination $Dest -Force

Write-Host "Atualizando arquivo de perfil ($ProfilePath)..." -ForegroundColor Yellow
if (!(Test-Path $ProfilePath)) {
    $ProfileDir = Split-Path -Parent $ProfilePath
    if (!(Test-Path $ProfileDir)) { New-Item -ItemType Directory -Force -Path $ProfileDir | Out-Null }
    New-Item -ItemType File -Force -Path $ProfilePath | Out-Null
}

$content = Get-Content $ProfilePath
$alreadyInstalled = $content | Where-Object { $_ -match $InstallMarker }

if ($alreadyInstalled) {
    Write-Host "SKIP: O wrapper do cdd ja esta instalado no \$PROFILE." -ForegroundColor DarkGray
} else {
    Add-Content -Path $ProfilePath -Value ""
    Add-Content -Path $ProfilePath -Value $InstallMarker
    Add-Content -Path $ProfilePath -Value $SourceCmd
    Write-Host "OK: cdd injetado no \$PROFILE." -ForegroundColor Green
}

Write-Host "Instalacao concluida com sucesso!" -ForegroundColor Green
Write-Host "Abra um novo terminal ou digite '. `$PROFILE' para comecar a usar." -ForegroundColor Cyan
