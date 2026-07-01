# Minimal test-framework shim for extracted shell tests.
# This intentionally implements only common helpers used by small benchmark
# datasets. It is not a full replacement for gnulib/coreutils tests/init.sh.

: "${fail:=0}"

path_prepend_()
{
  # The runner exposes the program under test through exported functions and
  # ./<program> wrappers. Avoid mutating PATH by default; PATH-sensitive tools
  # such as which must see the test's own PATH changes.
  :
}

framework_failure_()
{
  echo "framework failure" >&2
  exit 99
}

skip_()
{
  echo "skipped: $*" >&2
  exit 77
}

mkfifo_or_skip_()
{
  name=$1
  if command -v mkfifo >/dev/null 2>&1; then
    mkfifo "$name" 2>/dev/null && return 0
  fi
  if command -v python3 >/dev/null 2>&1; then
    python3 -c 'import os,sys; os.mkfifo(sys.argv[1])' "$name" 2>/dev/null && return 0
  fi
  if command -v python >/dev/null 2>&1; then
    python -c 'import os,sys; os.mkfifo(sys.argv[1])' "$name" 2>/dev/null && return 0
  fi
  skip_ "cannot create fifo $name"
}

retry_delay_()
{
  func=$1
  delay=$2
  retries=$3
  shift 3

  i=0
  while :; do
    "$func" "$delay" "$@" && return 0
    i=$((i + 1))
    test "$i" -ge "$retries" && return 1
    sleep "$delay"
  done
}

compare()
{
  diff -u "$@"
}

returns_()
{
  expected=$1
  shift
  "$@"
  status=$?
  test "$status" -eq "$expected"
}

Exit()
{
  exit "$1"
}

getlimits_()
{
  : "${SSIZE_MAX:=9223372036854775807}"
  export SSIZE_MAX
}

get_min_ulimit_v_()
{
  # Conservative default: enough for small tests, low enough for allocation
  # guard checks that add a small margin.
  echo 65536
}

# Perl support: many coreutils tests embed Perl scripts.
# Locate perl and export $PERL; skip the test if unavailable.
if command -v perl >/dev/null 2>&1; then
  PERL=$(command -v perl)
else
  PERL=
fi
export PERL

require_perl_()
{
  if test -z "$PERL"; then
    skip_ "this test requires perl"
  fi
}

require_readable_root_()
{
  test -r / || skip_ "/ is not readable"
}

require_root_()
{
  test "$(id -u)" -eq 0 || skip_ "this test requires root"
}

_cgr_cleanup_on_exit_()
{
  status=$?
  if command -v cleanup_ >/dev/null 2>&1; then
    cleanup_
  fi
  exit "$status"
}
trap _cgr_cleanup_on_exit_ 0
