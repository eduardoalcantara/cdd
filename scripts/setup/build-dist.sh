#!/bin/bash
# build-dist.sh
# Script para compilar o cdd e gerar o pacote de distribuição autônomo (tar.gz) para Linux

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

echo "Compilando cdd em modo release..."
cd "$REPO_ROOT/core"
cargo build --release

DIST_DIR="$REPO_ROOT/dist"
PKG_DIR="$DIST_DIR/cdd-linux-x86_64"

echo "Preparando diretório de distribuição..."
rm -rf "$PKG_DIR"
mkdir -p "$PKG_DIR"

# Copiando artefatos
cp "$REPO_ROOT/core/target/release/cdd" "$PKG_DIR/"
cp "$REPO_ROOT/scripts/shell/cdd.sh" "$PKG_DIR/"
cp "$REPO_ROOT/scripts/setup/install-user.sh" "$PKG_DIR/install.sh"
chmod +x "$PKG_DIR/install.sh" "$PKG_DIR/cdd"

echo "Compactando o pacote cdd-linux-x86_64.tar.gz..."
cd "$DIST_DIR"
tar -czf cdd-linux-x86_64.tar.gz cdd-linux-x86_64/
rm -rf cdd-linux-x86_64/

echo "Build concluído! Pacote gerado em: dist/cdd-linux-x86_64.tar.gz"
