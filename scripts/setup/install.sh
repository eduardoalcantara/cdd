#!/bin/bash
# install.sh
# Script de instalação do utilitário cdd (Change Directory Directly) para Linux

set -euo pipefail

# Obtem a raiz do repositorio
REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

# Se estamos na pasta core, suba mais um nivel (o shell costuma se perder no drive se for chamado daqui)
if [ ! -f "$REPO_ROOT/spec-root.md" ]; then
    echo "Erro: Não foi possível detectar a raiz do repositório cdd."
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
    printf "\e[36m│ \e[0m%-16s\e[36m │ \e[0m%-54s\e[36m │\e[0m\n" "Projeto" "cdd (Change Directory Directly)"
    printf "\e[36m│ \e[0m%-16s\e[36m │ \e[0m%-54s\e[36m │\e[0m\n" "Ação" "Instalação / Atualização"
    printf "\e[36m├──────────────────┼────────────────────────────────────────────────────────┤\e[0m\n"
    printf "\e[36m│ \e[0m%-16s\e[36m │ \e[0m%-54s\e[36m │\e[0m\n" "Raiz detectada" "${REPO_ROOT:0:54}"
    printf "\e[36m└──────────────────┴────────────────────────────────────────────────────────┘\e[0m\n\n"
fi

BASHRC_FILE="$HOME/.bashrc"
ZSHRC_FILE="$HOME/.zshrc"
INSTALL_MARKER="# CDD_INSTALL_MARKER"
CDD_SOURCE_CMD="source \"$REPO_ROOT/scripts/shell/cdd.sh\""

do_uninstall() {
    if [ "$QUIET" -eq 0 ]; then
        echo "Aviso: Isso irá remover as entradas do comando 'cdd' dos seus arquivos de perfil (.bashrc / .zshrc)."
        if [ "$FORCE" -eq 0 ]; then
            read -r -p "Continuar? (1 = Sim / 0 = Não) [1]: " opt
            opt=${opt:-1}
            if [ "$opt" != "1" ]; then
                echo "Abortado."
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

    echo "Desinstalação concluída. O comando 'cdd' foi removido do seu shell."
    echo "Por favor, reinicie seu terminal para aplicar as mudanças."
}

do_install() {
    echo "Compilando o binário cdd (Rust)..."
    cd "$REPO_ROOT/core"
    cargo build --release

    if [ $? -ne 0 ]; then
        echo "FAIL: Erro ao compilar o cdd."
        exit 1
    fi

    echo "OK: Binário compilado com sucesso."

    # Adicionar ao bashrc se não existir
    if [ -f "$BASHRC_FILE" ]; then
        if ! grep -q "$INSTALL_MARKER" "$BASHRC_FILE"; then
            echo -e "\n$INSTALL_MARKER\n$CDD_SOURCE_CMD" >> "$BASHRC_FILE"
            echo "OK: Injetado no $BASHRC_FILE"
        else
            echo "SKIP: cdd já está presente no $BASHRC_FILE"
        fi
    fi

    # Adicionar ao zshrc se não existir
    if [ -f "$ZSHRC_FILE" ]; then
        if ! grep -q "$INSTALL_MARKER" "$ZSHRC_FILE"; then
            echo -e "\n$INSTALL_MARKER\n$CDD_SOURCE_CMD" >> "$ZSHRC_FILE"
            echo "OK: Injetado no $ZSHRC_FILE"
        else
            echo "SKIP: cdd já está presente no $ZSHRC_FILE"
        fi
    fi

    echo "Instalação concluída com sucesso!"
    echo "Execute 'source ~/.bashrc' ou reinicie o terminal para usar o comando 'cdd'."
}

if [ "$UNINSTALL" -eq 1 ]; then
    do_uninstall
else
    do_install
fi
