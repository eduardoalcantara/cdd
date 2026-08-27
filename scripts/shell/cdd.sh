#!/bin/bash
# Wrapper for cdd (Linux / WSL)
# Usage:
# Add `source /path/to/cdd/scripts/shell/cdd.sh` to your ~/.bashrc or ~/.zshrc

function cdd() {
    # Set the path to the cdd rust binary. 
    # For production, this should point to the installed bin, e.g. ~/.cargo/bin/cdd
    # For this repository, we can try to find it relative to this script
    local CDD_SCRIPT_DIR
    CDD_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
    local CDD_BIN_SAME_DIR="$CDD_SCRIPT_DIR/cdd-bin"
    local CDD_BIN_DEBUG="$CDD_SCRIPT_DIR/../../core/target/debug/cdd-bin"
    local CDD_BIN_RELEASE="$CDD_SCRIPT_DIR/../../core/target/release/cdd-bin"
    
    local CDD_BIN="cdd-bin"
    
    if [ -f "$CDD_BIN_SAME_DIR" ]; then
        CDD_BIN="$CDD_BIN_SAME_DIR"
    elif [ -f "$CDD_BIN_RELEASE" ]; then
        CDD_BIN="$CDD_BIN_RELEASE"
    elif [ -f "$CDD_BIN_DEBUG" ]; then
        CDD_BIN="$CDD_BIN_DEBUG"
    elif ! command -v "$CDD_BIN" >/dev/null 2>&1; then
        echo "cdd: command not found. Ensure the Rust binary is compiled."
        return 1
    fi

    # Create a temporary file to store the result
    local TMP_FILE
    if ! TMP_FILE=$(mktemp -t cdd_result.XXXXXX); then
        echo "cdd: could not create temporary file." >&2
        return 1
    fi

    # Execute the Rust binary passing all arguments and the temp file path
    "$CDD_BIN" "$@" --cdd-out-file "$TMP_FILE"

    local EXIT_CODE=$?

    if [ "$EXIT_CODE" -eq 0 ] && [ -s "$TMP_FILE" ]; then
        local TARGET_DIR
        IFS= read -r TARGET_DIR < "$TMP_FILE"
        if [ -d "$TARGET_DIR" ]; then
            cd "$TARGET_DIR" || EXIT_CODE=1
        fi
    fi

    rm -f "$TMP_FILE"
    return "$EXIT_CODE"
}
