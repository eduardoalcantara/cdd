#!/bin/bash
# install.sh (Distribuição End-User)
# Script de instalação standalone para o pacote pré-compilado do cdd.

set -euo pipefail

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
    printf "\e[36m┌──────────────────┬────────────────────────────────────────────────────────┐\e[0m\n"
    printf "\e[36m│ \e[0m%-16s\e[36m │ \e[0m%-54s\e[36m │\e[0m\n" "Project" "cdd (Change Directory Directly)"
    printf "\e[36m│ \e[0m%-16s\e[36m │ \e[0m%-54s\e[36m │\e[0m\n" "Action" "Standalone Binary Installation"
    printf "\e[36m└──────────────────┴────────────────────────────────────────────────────────┘\e[0m\n\n"
fi

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DEST="$HOME/.local/share/cdd"
BASHRC_FILE="$HOME/.bashrc"
ZSHRC_FILE="$HOME/.zshrc"
INSTALL_MARKER="# CDD_INSTALL_MARKER"
CDD_SOURCE_CMD="source \"$DEST/cdd.sh\""

do_uninstall() {
    if [ "$QUIET" -eq 0 ]; then
        echo "Warning: This will delete binaries in ~/.local/share/cdd and remove the entry from your profile."
        if [ "$FORCE" -eq 0 ]; then
            read -r -p "Continue? (1 = Yes / 0 = No) [1]: " opt
            opt=${opt:-1}
            if [ "$opt" != "1" ]; then
                echo "Aborted."
                exit 0
            fi
        fi
    fi

    rm -rf "$DEST"

    if [ -f "$BASHRC_FILE" ]; then
        sed -i "/$INSTALL_MARKER/d" "$BASHRC_FILE"
        sed -i "\|source \"$DEST/cdd.sh\"|d" "$BASHRC_FILE"
    fi

    if [ -f "$ZSHRC_FILE" ]; then
        sed -i "/$INSTALL_MARKER/d" "$ZSHRC_FILE"
        sed -i "\|source \"$DEST/cdd.sh\"|d" "$ZSHRC_FILE"
    fi

    echo "Uninstallation complete. Files and profile injections were removed."
}

do_install() {
    echo "Copying binary and wrapper to $DEST..."
    mkdir -p "$DEST"
    cp "$DIR/cdd" "$DIR/cdd.sh" "$DEST/"
    chmod +x "$DEST/cdd" "$DEST/cdd.sh"

    # Injetando no bashrc
    if [ -f "$BASHRC_FILE" ]; then
        if ! grep -q "$INSTALL_MARKER" "$BASHRC_FILE"; then
            echo -e "\n$INSTALL_MARKER\n$CDD_SOURCE_CMD" >> "$BASHRC_FILE"
            echo "OK: cdd injected into $BASHRC_FILE"
        else
            echo "SKIP: cdd is already present in $BASHRC_FILE"
        fi
    fi

    # Injetando no zshrc
    if [ -f "$ZSHRC_FILE" ]; then
        if ! grep -q "$INSTALL_MARKER" "$ZSHRC_FILE"; then
            echo -e "\n$INSTALL_MARKER\n$CDD_SOURCE_CMD" >> "$ZSHRC_FILE"
            echo "OK: cdd injected into $ZSHRC_FILE"
        else
            echo "SKIP: cdd is already present in $ZSHRC_FILE"
        fi
    fi

    echo "Installation finished!"
    echo "Run 'source ~/.bashrc' or open a new terminal to use 'cdd'."
}

if [ "$UNINSTALL" -eq 1 ]; then
    do_uninstall
else
    do_install
fi
