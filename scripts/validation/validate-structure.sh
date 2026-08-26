#!/usr/bin/env bash
# validate-structure.sh — confere a estrutura mínima do template
set -euo pipefail

clear_screen() {
  if [[ -t 1 ]] && command -v clear >/dev/null 2>&1; then
    clear
  fi
}

find_repo_root() {
  local dir="$PWD"
  while [[ "$dir" != "/" ]]; do
    if [[ -f "$dir/spec-root.md" && -f "$dir/.prompt-status" ]]; then
      printf '%s\n' "$dir"
      return 0
    fi
    dir="$(dirname "$dir")"
  done
  return 1
}

usage() {
  cat <<'EOF'
Uso: validate-structure.sh [--help]

Valida a presença dos arquivos e pastas mínimos do template.

Opções:
  --help    Mostra esta ajuda
EOF
}

main() {
  if [[ "${1:-}" == "--help" || "${1:-}" == "-h" ]]; then
    usage
    exit 0
  fi

  clear_screen

  local root
  if ! root="$(find_repo_root)"; then
    echo "FAIL: raiz do repositório não encontrada (marcadores: spec-root.md + .prompt-status)" >&2
    exit 1
  fi

  echo "========================================"
  echo " Script : validate-structure.sh"
  echo " Propósito: validar estrutura mínima do template"
  echo " Modo   : validate"
  echo " Raiz   : $root"
  echo "========================================"
  echo

  local -a required_files=(
    .gitignore
    readme.md
    spec-root.md
    flow.md
    rules.md
    status.md
    timeline.md
    setup.md
    tools-linux.md
    tools-windows.md
    .cursorrules
    .prompt-status
    spec-template.md
    rules-scripts.md
  )

  local -a required_dirs=(
    docs
    ideas
    specs
    specs/to-do
    specs/done
    references
    scripts
    scripts/bootstrap
    scripts/validation
    scripts/automation
    scripts/maintenance
    reports
    prompts
    resources
    core
  )

  local missing=0

  for f in "${required_files[@]}"; do
    if [[ -f "$root/$f" ]]; then
      echo "OK   file  $f"
    else
      echo "FAIL file  $f"
      missing=$((missing + 1))
    fi
  done

  for d in "${required_dirs[@]}"; do
    if [[ -d "$root/$d" ]]; then
      echo "OK   dir   $d/"
    else
      echo "FAIL dir   $d/"
      missing=$((missing + 1))
    fi
  done

  echo
  if [[ "$missing" -eq 0 ]]; then
    echo "OK: estrutura mínima completa ($((${#required_files[@]} + ${#required_dirs[@]})) itens)."
    exit 0
  fi

  echo "FAIL: $missing item(ns) ausente(s)." >&2
  exit 1
}

main "$@"
