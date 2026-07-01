#!/bin/bash

run_ttest_shell() {
    local shell_path="$1"
    local shell_label="$2"
    local shc="${3:-}"
    local check_opts=("" "-r" "-v" "-D" "-S")
    local txtred='\e[0;31m'
    local txtgrn='\e[0;32m'
    local txtrst='\e[0m'
    local stat=0
    local pc=0
    local fc=0

    if [[ -z "$shc" ]]; then
        if [[ -x ./shc ]]; then
            shc=./shc
        elif [[ -n "${SCRIPT_DIR:-}" && -x "$SCRIPT_DIR/../shc" ]]; then
            shc="$SCRIPT_DIR/../shc"
        else
            shc=shc
        fi
    fi

    echo
    echo "== Running ttest for $shell_path ..."

    for opt in "${check_opts[@]}"; do
        local tmpd
        local tmpf
        local out
        local opt_label
        local shc_stdout
        local shc_stderr
        local shc_status
        local artifact_name
        local artifact_dir
        local run_stdout
        local run_stderr
        local run_status

        tmpd=$(mktemp -d)
        tmpf="$tmpd/test.$shell_label"
        shc_stdout="$tmpd/shc.stdout"
        shc_stderr="$tmpd/shc.stderr"
        run_stdout="$tmpd/a.out.stdout"
        run_stderr="$tmpd/a.out.stderr"
        opt_label="${opt:-default}"
        artifact_name="${shell_label}_${opt_label}"
        artifact_name="${artifact_name//[^A-Za-z0-9_.-]/_}"
        artifact_dir="ttest_artifacts/$artifact_name"

        {
            echo "#!$shell_path"
            echo "echo 'Hello World fp:'\$1 sp:\$2"
        } > "$tmpf"

        if [[ -n "$opt" ]]; then
            "$shc" "$opt" -f "$tmpf" -o "$tmpd/a.out" >"$shc_stdout" 2>"$shc_stderr"
            shc_status=$?
        else
            "$shc" -f "$tmpf" -o "$tmpd/a.out" >"$shc_stdout" 2>"$shc_stderr"
            shc_status=$?
        fi

        if [[ $shc_status -ne 0 || ! -x "$tmpd/a.out" ]]; then
            rm -rf "$artifact_dir"
            mkdir -p "$artifact_dir"
            cp -a "$tmpd"/. "$artifact_dir"/ 2>/dev/null || true

            echo    "===================================================="
            echo -e "=== $shell_path [with shc $opt_label]: ${txtred}FAILED${txtrst}"
            echo    "shc command failed or did not produce executable output"
            echo    "shc: $shc"
            echo    "status: $shc_status"
            echo    "artifacts: $artifact_dir"
            if [[ -s "$shc_stdout" ]]; then
                echo    "--- shc stdout ---"
                cat "$shc_stdout"
            fi
            if [[ -s "$shc_stderr" ]]; then
                echo    "--- shc stderr ---"
                cat "$shc_stderr"
            fi
            echo    "--- tmp files ---"
            find "$tmpd" -maxdepth 1 -type f -printf '%f %s bytes\n' | sort
            echo    "===================================================="
            stat=1
            ((fc++))
            rm -rf "$tmpd"
            continue
        fi

        "$tmpd/a.out" first second >"$run_stdout" 2>"$run_stderr"
        run_status=$?
        out=$(cat "$run_stdout")
        if [[ $run_status -eq 0 && "$out" = 'Hello World fp:first sp:second' ]]; then
            echo    "===================================================="
            echo -e "=== $shell_path [with shc $opt_label]: ${txtgrn}PASSED${txtrst}"
            echo    "===================================================="
            ((pc++))
        else
            rm -rf "$artifact_dir"
            mkdir -p "$artifact_dir"
            cp -a "$tmpd"/. "$artifact_dir"/ 2>/dev/null || true

            echo    "===================================================="
            echo -e "=== $shell_path [with shc $opt_label]: ${txtred}FAILED${txtrst}"
            echo    "generated executable output mismatch"
            echo    "status: $run_status"
            echo    "artifacts: $artifact_dir"
            echo    "Output: $out"
            if [[ -s "$run_stderr" ]]; then
                echo    "--- a.out stderr ---"
                cat "$run_stderr"
            fi
            echo    "===================================================="
            stat=1
            ((fc++))
        fi

        rm -rf "$tmpd"
    done

    echo
    echo "Test Summary ($shell_path)"
    echo "-------------------------"
    echo -e "PASSED: $pc"
    echo -e "FAILED: $fc"
    echo "-------------------------"
    echo

    exit "$stat"
}
