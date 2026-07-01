#!/bin/bash

SCRIPT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
. "$SCRIPT_DIR/ttest_case.inc"

run_ttest_shell "/bin/zsh" "zsh" "${1:-}"
