#!/bin/bash
# install.sh
# Script de instalação do utilitário cdd (Change Directory Directly) para Linux

set -euo pipefail

# Obtem a raiz do repositorio
REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

# Se estamos na pasta core, suba mais um nivel (o shell costuma se perder no drive se for chamado daqui)
if [ ! -f "$REPO_ROOT/spec-root.md" ]; then
    echo "Error: Could not detect cdd repository root."
    exit 1
fi

UNINSTALL=0
QUIET=0
FORCE=0

for arg in "$@"; do
    case $arg in
        --uninstall) UNINSTALL=1 ;;
        --quiet|-q) QUIET=1 ;;
        --force|-f) FORCE=1 ;;
    esac
done

if [ "$QUIET" -eq 0 ]; then
    clear
    # Print cyan header
    printf "\e[36m┌──────────────────┬────────────────────────────────────────────────────────┐\e[0m\n"
    printf "\e[36m│ \e[0m%-16s\e[36m │ \e[0m%-54s\e[36m │\e[0m\n" "Project" "cdd (Change Directory Directly)"
    printf "\e[36m│ \e[0m%-16s\e[36m │ \e[0m%-54s\e[36m │\e[0m\n" "Action" "Installation / Update"
    printf "\e[36m├──────────────────┼────────────────────────────────────────────────────────┤\e[0m\n"
    printf "\e[36m│ \e[0m%-16s\e[36m │ \e[0m%-54s\e[36m │\e[0m\n" "Detected root" "${REPO_ROOT:0:54}"
    printf "\e[36m└──────────────────┴────────────────────────────────────────────────────────┘\e[0m\n\n"
fi

BASHRC_FILE="$HOME/.bashrc"
ZSHRC_FILE="$HOME/.zshrc"
INSTALL_MARKER="# CDD_INSTALL_MARKER"
CDD_SOURCE_CMD="source \"$REPO_ROOT/scripts/shell/cdd.sh\""

do_uninstall() {
    if [ "$QUIET" -eq 0 ]; then
        echo "Warning: This will remove 'cdd' entries from your profile files (.bashrc / .zshrc)."
        if [ "$FORCE" -eq 0 ]; then
            read -r -p "Continue? (1 = Yes / 0 = No) [1]: " opt
            opt=${opt:-1}
            if [ "$opt" != "1" ]; then
                echo "Aborted."
                exit 0
            fi
        fi
    fi

    # Remover do bashrc
    if [ -f "$BASHRC_FILE" ]; then
        sed -i "/$INSTALL_MARKER/d" "$BASHRC_FILE"
        sed -i "\|source \"$REPO_ROOT/scripts/shell/cdd.sh\"|d" "$BASHRC_FILE"
    fi

    # Remover do zshrc
    if [ -f "$ZSHRC_FILE" ]; then
        sed -i "/$INSTALL_MARKER/d" "$ZSHRC_FILE"
        sed -i "\|source \"$REPO_ROOT/scripts/shell/cdd.sh\"|d" "$ZSHRC_FILE"
    fi

    echo "Uninstallation complete. The 'cdd' command was removed from your shell."
    echo "Please restart your terminal to apply the changes."
}

do_install() {
    echo "Compiling cdd binary (Rust)..."
    cd "$REPO_ROOT/core"
    cargo build --release

    if [ $? -ne 0 ]; then
        echo "FAIL: Failed to compile cdd."
        exit 1
    fi

    echo "OK: Binary compiled successfully."

    # Adicionar ao bashrc se não existir
    if [ -f "$BASHRC_FILE" ]; then
        if ! grep -q "$INSTALL_MARKER" "$BASHRC_FILE"; then
            echo -e "\n$INSTALL_MARKER\n$CDD_SOURCE_CMD" >> "$BASHRC_FILE"
            echo "OK: Injected into $BASHRC_FILE"
        else
            echo "SKIP: cdd is already present in $BASHRC_FILE"
        fi
    fi

    # Adicionar ao zshrc se não existir
    if [ -f "$ZSHRC_FILE" ]; then
        if ! grep -q "$INSTALL_MARKER" "$ZSHRC_FILE"; then
            echo -e "\n$INSTALL_MARKER\n$CDD_SOURCE_CMD" >> "$ZSHRC_FILE"
            echo "OK: Injected into $ZSHRC_FILE"
        else
            echo "SKIP: cdd is already present in $ZSHRC_FILE"
        fi
    fi

    echo "Installation completed successfully!"
    echo "Run 'source ~/.bashrc' or restart the terminal to use the 'cdd' command."
}

if [ "$UNINSTALL" -eq 1 ]; then
    do_uninstall
else
    do_install
fi
