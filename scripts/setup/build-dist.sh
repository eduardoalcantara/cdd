#!/bin/bash
# build-dist.sh
# Script para compilar o cdd e gerar o pacote de distribuição autônomo (tar.gz) para Linux

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

if [[ "${1:-}" == "--help" || "${1:-}" == "-h" ]]; then
    cat <<'EOF'
Uso: build-dist.sh

Compila o cdd para Linux x86_64 musl e gera:
  dist/cdd-linux-x86_64.tar.gz
  dist/cdd-linux-x86_64.tar.gz.sha256
EOF
    exit 0
fi

if [[ "$#" -gt 0 ]]; then
    echo "FAIL: opção desconhecida: $1" >&2
    exit 2
fi

echo "Compilando cdd em modo release..."
cd "$REPO_ROOT/core"
cargo build --release --target x86_64-unknown-linux-musl

DIST_DIR="$REPO_ROOT/dist"
PKG_DIR="$DIST_DIR/cdd-linux-x86_64"
ARCHIVE="$DIST_DIR/cdd-linux-x86_64.tar.gz"

echo "Preparando diretório de distribuição..."
rm -rf "$PKG_DIR"
mkdir -p "$PKG_DIR"

# Copiando artefatos
cp "$REPO_ROOT/core/target/x86_64-unknown-linux-musl/release/cdd" "$PKG_DIR/"
cp "$REPO_ROOT/scripts/shell/cdd.sh" "$PKG_DIR/"
cp "$REPO_ROOT/scripts/setup/install-user.sh" "$PKG_DIR/install.sh"
cp "$REPO_ROOT/readme.md" "$PKG_DIR/README.md"
cp "$REPO_ROOT/docs/HOW_TO_USE.md" "$PKG_DIR/"
cp "$REPO_ROOT/docs/HOW_TO_INSTALL.md" "$PKG_DIR/"
cp "$REPO_ROOT/docs/HOW_IT_WORKS.md" "$PKG_DIR/"
chmod +x "$PKG_DIR/install.sh" "$PKG_DIR/cdd"

echo "Compactando o pacote cdd-linux-x86_64.tar.gz..."
cd "$DIST_DIR"
rm -f "$ARCHIVE" "$ARCHIVE.sha256"
tar -czf "$ARCHIVE" cdd-linux-x86_64/
sha256sum "$(basename "$ARCHIVE")" > "$ARCHIVE.sha256"
rm -rf cdd-linux-x86_64/

echo "Build concluído!"
echo "Pacote: dist/cdd-linux-x86_64.tar.gz"
echo "Checksum: dist/cdd-linux-x86_64.tar.gz.sha256"
