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
    Write-Host "$v Project          $v cdd (Change Directory Directly)                        $v" -ForegroundColor Cyan
    if ($Uninstall) {
        Write-Host "$v Action           $v End-User Package Uninstallation                        $v" -ForegroundColor Cyan
    } else {
        Write-Host "$v Action           $v Standalone Binary Installation                         $v" -ForegroundColor Cyan
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
        Write-Host "Warning: This will delete files in $Dest and remove the entry from your profile."
        $ans = Read-Host "Continue? (1 = Yes / 0 = No) [1]"
        if ($ans -eq "" -or $ans -eq "1") { } else {
            Write-Host "Aborted."
            exit 0
        }
    }

    if (Test-Path $Dest) { Remove-Item -Recurse -Force $Dest }

    if (Test-Path $ProfilePath) {
        $content = Get-Content $ProfilePath
        $newContent = $content | Where-Object { $_ -notmatch $InstallMarker -and $_ -notmatch "\.cdd\\cdd\.ps1" }
        Set-Content -Path $ProfilePath -Value $newContent
        Write-Host "Uninstallation complete. Files and profile injection were removed." -ForegroundColor Green
    }
    exit 0
}

Write-Host "Copying binary files to $Dest..." -ForegroundColor Yellow
if (!(Test-Path $Dest)) { New-Item -ItemType Directory -Force -Path $Dest | Out-Null }
Copy-Item (Join-Path $ScriptDir "cdd.exe") -Destination $Dest -Force
Copy-Item (Join-Path $ScriptDir "cdd.ps1") -Destination $Dest -Force

Write-Host "Updating profile file ($ProfilePath)..." -ForegroundColor Yellow
if (!(Test-Path $ProfilePath)) {
    $ProfileDir = Split-Path -Parent $ProfilePath
    if (!(Test-Path $ProfileDir)) { New-Item -ItemType Directory -Force -Path $ProfileDir | Out-Null }
    New-Item -ItemType File -Force -Path $ProfilePath | Out-Null
}

$content = Get-Content $ProfilePath
$alreadyInstalled = $content | Where-Object { $_ -match $InstallMarker }

if ($alreadyInstalled) {
    Write-Host "SKIP: cdd wrapper is already installed in `$PROFILE." -ForegroundColor DarkGray
} else {
    Add-Content -Path $ProfilePath -Value ""
    Add-Content -Path $ProfilePath -Value $InstallMarker
    Add-Content -Path $ProfilePath -Value $SourceCmd
    Write-Host "OK: cdd injected into `$PROFILE." -ForegroundColor Green
}

Write-Host "Installation completed successfully!" -ForegroundColor Green
Write-Host "Open a new terminal or run '. `$PROFILE' to start using it." -ForegroundColor Cyan
