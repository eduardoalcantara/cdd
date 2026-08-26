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
    printf "\e[36m│ \e[0m%-16s\e[36m │ \e[0m%-54s\e[36m │\e[0m\n" "Projeto" "cdd (Change Directory Directly)"
    printf "\e[36m│ \e[0m%-16s\e[36m │ \e[0m%-54s\e[36m │\e[0m\n" "Ação" "Instalação do Binário Autônomo"
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
        echo "Aviso: Isso irá apagar os binários em ~/.local/share/cdd e remover a entrada do seu perfil."
        if [ "$FORCE" -eq 0 ]; then
            read -r -p "Continuar? (1 = Sim / 0 = Não) [1]: " opt
            opt=${opt:-1}
            if [ "$opt" != "1" ]; then
                echo "Abortado."
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

    echo "Desinstalação concluída. Os arquivos e as injeções de perfil foram removidos."
}

do_install() {
    echo "Copiando binário e wrapper para $DEST..."
    mkdir -p "$DEST"
    cp "$DIR/cdd" "$DIR/cdd.sh" "$DEST/"
    chmod +x "$DEST/cdd" "$DEST/cdd.sh"

    # Injetando no bashrc
    if [ -f "$BASHRC_FILE" ]; then
        if ! grep -q "$INSTALL_MARKER" "$BASHRC_FILE"; then
            echo -e "\n$INSTALL_MARKER\n$CDD_SOURCE_CMD" >> "$BASHRC_FILE"
            echo "OK: cdd injetado no $BASHRC_FILE"
        else
            echo "SKIP: cdd já está presente no $BASHRC_FILE"
        fi
    fi

    # Injetando no zshrc
    if [ -f "$ZSHRC_FILE" ]; then
        if ! grep -q "$INSTALL_MARKER" "$ZSHRC_FILE"; then
            echo -e "\n$INSTALL_MARKER\n$CDD_SOURCE_CMD" >> "$ZSHRC_FILE"
            echo "OK: cdd injetado no $ZSHRC_FILE"
        else
            echo "SKIP: cdd já está presente no $ZSHRC_FILE"
        fi
    fi

    echo "Instalação finalizada!"
    echo "Execute 'source ~/.bashrc' ou abra um novo terminal para utilizar o 'cdd'."
}

if [ "$UNINSTALL" -eq 1 ]; then
    do_uninstall
else
    do_install
fi
