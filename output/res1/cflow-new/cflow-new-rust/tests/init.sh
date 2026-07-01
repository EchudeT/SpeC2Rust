# Minimal init.sh for cflow tests
export PATH_ORIG="$PATH"

path_prepend_() {
    dir="$1"
    case "$dir" in
        /*) ;;
        *) dir="$(pwd)/$dir" ;;
    esac
    PATH="$dir:$PATH"
}

framework_failure_() {
    echo "$*" >&2
    exit 99
}

skip_() {
    echo "$*" >&2
    exit 77
}
