# which Repository Inventory

This document records only the facts directly observed during the current repository scan. It does not fill in missing directory trees or guess at artifacts that have not appeared yet.

## Snapshot
- Project name: `which`
- Build system: `Makefile.in`
- C file count: 6
- Header file count: 9
- Other file count: 209
- Build files: Makefile, configure, Makefile.am, Makefile.in
- Entry files: none
- Executables observed at the repository root: configure, config.status, which
- Library files observed at the repository root: none

## Directory Inventory
- `compare_results`: 100 files total (0 C files, 0 header files, 100 other files). Examples: compare_results/test1.log, compare_results/test10.log, compare_results/test10_c.norm, compare_results/test10_c.raw, compare_results/test10_rust.norm, compare_results/test10_rust.raw
- `flow_results`: 39 files total (0 C files, 0 header files, 39 other files). Examples: flow_results/test10_fail.log, flow_results/test10_success.log, flow_results/test11_fail.log, flow_results/test11_success.log, flow_results/test12_fail.log, flow_results/test12_success.log
- `results`: 20 files total (0 C files, 0 header files, 20 other files). Examples: results/test10_success.log, results/test11_success.log, results/test12_success.log, results/test13_success.log, results/test14_success.log, results/test15_success.log
- `root`: 50 files total (4 C files, 5 header files, 41 other files). Examples: AUTHORS, COPYING, EXAMPLES, INSTALL, Makefile, Makefile.am
- `test`: 2 files total (0 C files, 0 header files, 2 other files). Examples: test/base_test.sh, test/run_test.sh
- `tilde`: 13 files total (2 C files, 4 header files, 7 other files). Examples: tilde/Makefile, tilde/Makefile.am, tilde/Makefile.in, tilde/README, tilde/ansi_stdlib.h, tilde/bashansi.h

## Source File Inventory
- `getopt.c`
- `getopt1.c`
- `which.c`
- `bash.c`
- `tilde/tilde.c`
- `tilde/shell.c`

## Header Files by Directory
### `root`
- `bash.h`
- `config.h`
- `getopt.h`
- `posixstat.h`
- `sys.h`
### `tilde`
- `tilde/ansi_stdlib.h`
- `tilde/bashansi.h`
- `tilde/tilde.h`
- `tilde/xmalloc.h`

## README Excerpt
Install
=======

You will need an ANSI C compiler (like gcc) to compile this package.

Just type `make', followed by `make install'.

History
=======

The main difference with version 1.0 by Paul Vixie is that this
version will not return directory names as being executables
and that by default it will expand a leading "./" and "~/" to
its full path on output.

The -all option has been added in example of a version of which
on Ultrix.  They use `-a' as option.

The --read-alias idea has been copied from a version of which by
Maarten Litmaath called `which-v6', he was using `-i' as option
which stands for `interactive'.

Manual page
===========


NAME
       which - shows the full path of (shell) commands.

SYNOPSIS
       which [options] [--] programname [...]

DESCRIPTION
       Which takes one or more arguments. For each of its arguments it prints
       to stdout the full path of the executables that would have been exe-
       cuted when this argument had been entered at the shell prompt. It does
       this by searching for an executable or script in the directories listed
       in the environment variable PATH using the same algorithm as bash(1).

       This man page is generated from the file which.texinfo.

OPTIONS
       --all, -a
           Print all matching executables in PATH, not just the first.

       --read-alias, -i
           Read aliases from stdin, reporting matching ones on stdout. This is
           useful in combination with using an alias for which itself. For
           example
           alias which='alias | which -i'.

       --skip-alias
           Ignore option `--read-alias', if any. This is useful to explicity
           search for normal binaries, while using the `--read-alias' option
           in an alias or function for which.

       --read-functions
           Read shell function definitions from stdin, reporting matching ones
           on stdout. This is useful in combination with using a shell func-
           tion for whic
...[truncated]

## Evidence Boundaries
- This inventory comes from a filesystem scan and does not represent the final installation layout.
- Executables, libraries, or generated directories that were not observed will not be inferred to exist.
- File responsibilities and module behavior must be understood together with the later `01_subsystems` / `03_behaviors` documents.
