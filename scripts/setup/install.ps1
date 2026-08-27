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
$CddScriptPath = Join-Path $RepoRoot "scripts\shell\cdd.ps1"

# Unblock the wrapper script to prevent execution policy errors for RemoteSigned
if (Test-Path $CddScriptPath) {
    Unblock-File -Path $CddScriptPath -ErrorAction SilentlyContinue
}

$InstallMarker = "# CDD_INSTALL_MARKER"
$SourceCmd = ". `"$CddScriptPath`""

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
Write-Host "Restart PowerShell or run '. `$PROFILE' to start using the 'cdd' command." -ForegroundColor Cyan
