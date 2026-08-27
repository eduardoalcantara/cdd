# Wrapper for cdd (PowerShell)
# Usage:
# Add `. "C:\path\to\cdd\scripts\shell\cdd.ps1"` to your $PROFILE

$global:CddScriptPath = if ($MyInvocation.MyCommand.Path) { $MyInvocation.MyCommand.Path } else { $PSCommandPath }
$global:CddScriptDir = Split-Path -Parent $global:CddScriptPath

function cdd {
    $DebugBin = Join-Path $global:CddScriptDir "..\..\core\target\debug\cdd.exe"
    $ReleaseBin = Join-Path $global:CddScriptDir "..\..\core\target\release\cdd.exe"
    $SameDirBin = Join-Path $global:CddScriptDir "cdd.exe"
    
    $CddBin = "cdd.exe"
    
    if (Test-Path $SameDirBin) {
        $CddBin = $SameDirBin
    } elseif (Test-Path $ReleaseBin) {
        $CddBin = $ReleaseBin
    } elseif (Test-Path $DebugBin) {
        $CddBin = $DebugBin
    } else {
        if (!(Get-Command $CddBin -ErrorAction SilentlyContinue)) {
            Write-Host "cdd: command not found. Ensure the Rust binary is compiled." -ForegroundColor Red
            $global:LASTEXITCODE = 127
            return
        }
    }

    # Use a temporary file for communication
    $TmpFile = [System.IO.Path]::GetTempFileName()
    $CddExitCode = 1

    try {
        # Pass arguments to the rust binary
        & $CddBin $args --cdd-out-file $TmpFile
        $CddExitCode = $LASTEXITCODE

        if ($CddExitCode -eq 0 -and (Get-Item $TmpFile).Length -gt 0) {
            $TargetDir = (Get-Content $TmpFile -Raw).TrimEnd([char[]]"`r`n")
            if (Test-Path -LiteralPath $TargetDir -PathType Container) {
                Set-Location -LiteralPath $TargetDir
            }
        }
    } finally {
        Remove-Item $TmpFile -Force -ErrorAction SilentlyContinue
    }

    $global:LASTEXITCODE = $CddExitCode
}
