# validate-structure.ps1 — confere a estrutura mínima do template
[CmdletBinding()]
param(
    [switch]$Help
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

function Show-Usage {
    @"
Uso: validate-structure.ps1 [-Help]

Valida a presença dos arquivos e pastas mínimos do template.

Opções:
  -Help    Mostra esta ajuda
"@
}

function Find-RepoRoot {
    $dir = (Get-Location).Path
    while ($true) {
        $spec = Join-Path $dir 'spec-root.md'
        $prompt = Join-Path $dir '.prompt-status'
        if ((Test-Path -LiteralPath $spec) -and (Test-Path -LiteralPath $prompt)) {
            return $dir
        }
        $parent = Split-Path -Parent $dir
        if ([string]::IsNullOrEmpty($parent) -or $parent -eq $dir) {
            return $null
        }
        $dir = $parent
    }
}

if ($Help) {
    Show-Usage
    exit 0
}

if ($Host.UI.SupportsVirtualTerminal) {
    Clear-Host
}

$root = Find-RepoRoot
if (-not $root) {
    Write-Error "FAIL: raiz do repositório não encontrada (marcadores: spec-root.md + .prompt-status)"
    exit 1
}

Write-Host "========================================"
Write-Host " Script : validate-structure.ps1"
Write-Host " Propósito: validar estrutura mínima do template"
Write-Host " Modo   : validate"
Write-Host " Raiz   : $root"
Write-Host "========================================"
Write-Host ""

$requiredFiles = @(
    '.gitignore',
    'readme.md',
    'spec-root.md',
    'flow.md',
    'rules.md',
    'status.md',
    'timeline.md',
    'setup.md',
    'tools-linux.md',
    'tools-windows.md',
    '.cursorrules',
    '.prompt-status',
    'spec-template.md',
    'rules-scripts.md'
)

$requiredDirs = @(
    'docs',
    'ideas',
    'specs',
    'specs/to-do',
    'specs/done',
    'references',
    'scripts',
    'scripts/bootstrap',
    'scripts/validation',
    'scripts/automation',
    'scripts/maintenance',
    'reports',
    'prompts',
    'resources',
    'core'
)

$missing = 0

foreach ($f in $requiredFiles) {
    $path = Join-Path $root $f
    if (Test-Path -LiteralPath $path -PathType Leaf) {
        Write-Host "OK   file  $f"
    }
    else {
        Write-Host "FAIL file  $f"
        $missing++
    }
}

foreach ($d in $requiredDirs) {
    $path = Join-Path $root $d
    if (Test-Path -LiteralPath $path -PathType Container) {
        Write-Host "OK   dir   $d/"
    }
    else {
        Write-Host "FAIL dir   $d/"
        $missing++
    }
}

Write-Host ""
$total = $requiredFiles.Count + $requiredDirs.Count
if ($missing -eq 0) {
    Write-Host "OK: estrutura mínima completa ($total itens)."
    exit 0
}

Write-Error "FAIL: $missing item(ns) ausente(s)."
exit 1
