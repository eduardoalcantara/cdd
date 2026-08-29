# Wrapper de instalação para cdd (PowerShell)

param (
    [switch]$Uninstall,
    [switch]$Quiet,
    [switch]$Force
)

$ErrorActionPreference = "Stop"

# Descoberta da raiz
$ScriptPath = if ($MyInvocation.MyCommand.Path) { $MyInvocation.MyCommand.Path } else { $PSCommandPath }
$ScriptDir = Split-Path -Parent $ScriptPath
$RepoRoot = Resolve-Path (Join-Path $ScriptDir "..\..")

if (!(Test-Path (Join-Path $RepoRoot "spec-root.md"))) {
    Write-Host "Error: Could not detect cdd repository root." -ForegroundColor Red
    exit 1
}

# Limpeza e Tabela do Cabeçalho se não for silencioso
if (!$Quiet) {
    Clear-Host
    function Get-AppOemBoxChar([byte]$Code) {
        return [System.Text.Encoding]::GetEncoding(437).GetString([byte[]]@($Code))
    }
    $tl = Get-AppOemBoxChar 218; $tr = Get-AppOemBoxChar 191
    $bl = Get-AppOemBoxChar 192; $br = Get-AppOemBoxChar 217
    $h  = Get-AppOemBoxChar 196; $v  = Get-AppOemBoxChar 179
    $ml = Get-AppOemBoxChar 195; $mr = Get-AppOemBoxChar 180

    $leftW = 18
    $rightW = 56
    
    $ruleTop = $tl + ($h * $leftW) + $h + ($h * $rightW) + $tr
    $ruleMid = $ml + ($h * $leftW) + $h + ($h * $rightW) + $mr
    $ruleBot = $bl + ($h * $leftW) + $h + ($h * $rightW) + $br
    
    Write-Host $ruleTop -ForegroundColor Cyan
    Write-Host "$v Project          $v cdd (Change Directory Directly)                        $v" -ForegroundColor Cyan
    if ($Uninstall) {
        Write-Host "$v Action           $v PowerShell profile uninstallation                      $v" -ForegroundColor Cyan
    } else {
        Write-Host "$v Action           $v Compilation and Profile Installation                   $v" -ForegroundColor Cyan
    }
    Write-Host $ruleMid -ForegroundColor Cyan
    
    # Trunca o Root para não quebrar a tabela
    $RootTxt = $RepoRoot.Path
    if ($RootTxt.Length -gt 54) { $RootTxt = $RootTxt.Substring(0, 54) } else { $RootTxt = $RootTxt.PadRight(54) }
    
    Write-Host "$v Detected root    $v $RootTxt $v" -ForegroundColor Cyan
    Write-Host $ruleBot -ForegroundColor Cyan
    Write-Host ""
}

$ProfilePath = $PROFILE
$ShellDir = Join-Path $RepoRoot "scripts\shell"
$CddScriptPath = Join-Path $ShellDir "cdd.ps1"
$CddCmdPath = Join-Path $ShellDir "cdd.cmd"
$ReleaseBin = Join-Path $RepoRoot "core\target\release\cdd-bin.exe"
$ShellBin = Join-Path $ShellDir "cdd-bin.exe"

# Unblock wrapper scripts to prevent execution policy errors for RemoteSigned
foreach ($WrapperPath in @($CddScriptPath, $CddCmdPath)) {
    if (Test-Path $WrapperPath) {
        Unblock-File -Path $WrapperPath -ErrorAction SilentlyContinue
    }
}

$InstallMarker = "# CDD_INSTALL_MARKER"
$SourceCmd = ". `"$CddScriptPath`""

function Add-CddDevShellToUserPath {
    param([string]$Directory)

    $UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
    if ($UserPath -and $UserPath -match [regex]::Escape($Directory)) {
        Write-Host "SKIP: $Directory is already in User PATH." -ForegroundColor DarkGray
        return
    }

    if ([string]::IsNullOrWhiteSpace($UserPath)) {
        $NewPath = $Directory
    } elseif ($UserPath.EndsWith(";")) {
        $NewPath = $UserPath + $Directory
    } else {
        $NewPath = $UserPath + ";" + $Directory
    }

    [Environment]::SetEnvironmentVariable("Path", $NewPath, "User")
    Write-Host "OK: Added $Directory to User PATH for CMD support." -ForegroundColor Green
}

function Remove-CddDevShellFromUserPath {
    param([string]$Directory)

    $UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
    if (!$UserPath) {
        return
    }

    $Paths = $UserPath -split ";" | Where-Object {
        $_ -ne "" -and $_ -ne $Directory
    }
    $NewPath = ($Paths -join ";").TrimEnd(";")
    [Environment]::SetEnvironmentVariable("Path", $NewPath, "User")
    Write-Host "Removed $Directory from User PATH." -ForegroundColor Green
}

if ($Uninstall) {
    if (!$Quiet -and !$Force) {
        Write-Host "Warning: This will remove the cdd entry from your `$PROFILE ($ProfilePath)."
        $ans = Read-Host "Continue? (1 = Yes / 0 = No) [1]"
        if ($ans -eq "" -or $ans -eq "1") { } else {
            Write-Host "Aborted."
            exit 0
        }
    }

    if (Test-Path $ProfilePath) {
        $content = Get-Content $ProfilePath
        $newContent = $content | Where-Object { $_ -notmatch $InstallMarker -and $_ -notmatch "scripts\\shell\\cdd.ps1" }
        Set-Content -Path $ProfilePath -Value $newContent
        Write-Host "Uninstallation complete. cdd was removed from your Profile." -ForegroundColor Green
    } else {
        Write-Host "PROFILE not found, nothing to remove."
    }

    Remove-CddDevShellFromUserPath -Directory $ShellDir

    if (Test-Path $ShellBin) {
        Remove-Item -Path $ShellBin -Force
        Write-Host "Removed local cdd-bin.exe copy from scripts/shell." -ForegroundColor Green
    }

    exit 0
}

# Instalacao
Write-Host "Compiling cdd binary (Rust)..." -ForegroundColor Yellow
Set-Location (Join-Path $RepoRoot "core")
cargo build --release

if ($LASTEXITCODE -ne 0) {
    Write-Host "FAIL: Failed to compile cdd." -ForegroundColor Red
    exit 1
}
Write-Host "OK: Compiled successfully." -ForegroundColor Green

Write-Host "Updating CMD wrapper binary..." -ForegroundColor Yellow
if (!(Test-Path $ReleaseBin)) {
    Write-Host "FAIL: Release binary not found at $ReleaseBin" -ForegroundColor Red
    exit 1
}
Copy-Item -Path $ReleaseBin -Destination $ShellBin -Force
Write-Host "OK: Copied cdd-bin.exe to scripts/shell." -ForegroundColor Green

Write-Host "Updating User PATH for CMD support..." -ForegroundColor Yellow
Add-CddDevShellToUserPath -Directory $ShellDir

# Criar profile se não existir
if (!(Test-Path $ProfilePath)) {
    $ProfileDir = Split-Path -Parent $ProfilePath
    if (!(Test-Path $ProfileDir)) {
        New-Item -ItemType Directory -Force -Path $ProfileDir | Out-Null
    }
    New-Item -ItemType File -Force -Path $ProfilePath | Out-Null
}

$content = Get-Content $ProfilePath
$alreadyInstalled = $content | Where-Object { $_ -match $InstallMarker }

if ($alreadyInstalled) {
    Write-Host "SKIP: cdd is already installed in `$PROFILE." -ForegroundColor DarkGray
} else {
    Add-Content -Path $ProfilePath -Value ""
    Add-Content -Path $ProfilePath -Value $InstallMarker
    Add-Content -Path $ProfilePath -Value $SourceCmd
    Write-Host "OK: cdd injected into `$PROFILE." -ForegroundColor Green
}

Write-Host "Installation completed successfully!" -ForegroundColor Green
Write-Host "Restart your terminal (PowerShell or CMD) to start using the 'cdd' command." -ForegroundColor Cyan
Write-Host "PowerShell: run '. `$PROFILE' in the current session." -ForegroundColor Cyan
Write-Host "CMD: open a new Command Prompt window after PATH refresh." -ForegroundColor Cyan
