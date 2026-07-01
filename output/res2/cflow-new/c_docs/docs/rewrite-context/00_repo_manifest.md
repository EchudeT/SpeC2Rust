# cflow-new Repository Inventory

This document records only the facts directly observed during the current repository scan. It does not fill in missing directory trees or guess at artifacts that have not appeared yet.

## Snapshot
- Project name: `cflow-new`
- Build system: `Makefile.in`
- C file count: 68
- Header file count: 81
- Other file count: 263
- Build files: Makefile, configure, Makefile.am, Makefile.in
- Entry files: src/main.c
- Executables observed at the repository root: cflow
- Library files observed at the repository root: none

## Directory Inventory
- `autom4te.cache`: 3 files total (0 C files, 0 header files, 3 other files). Examples: autom4te.cache/output.0t, autom4te.cache/requests, autom4te.cache/traces.0t
- `build-aux`: 11 files total (0 C files, 0 header files, 11 other files). Examples: build-aux/compile, build-aux/config.guess, build-aux/config.rpath, build-aux/config.sub, build-aux/depcomp, build-aux/gitlog-to-changelog
- `doc`: 16 files total (5 C files, 0 header files, 11 other files). Examples: doc/Makefile, doc/Makefile.am, doc/Makefile.in, doc/ack.c, doc/cflow.1, doc/cflow.info
- `doc/imprimatur`: 11 files total (0 C files, 0 header files, 11 other files). Examples: doc/imprimatur/Makefile, doc/imprimatur/Makefile.am, doc/imprimatur/Makefile.in, doc/imprimatur/README, doc/imprimatur/check-docs.sh, doc/imprimatur/emptynodes.awk
- `elisp`: 4 files total (0 C files, 0 header files, 4 other files). Examples: elisp/Makefile, elisp/Makefile.am, elisp/Makefile.in, elisp/cflow-mode.el
- `gnu`: 144 files total (45 C files, 72 header files, 27 other files). Examples: gnu/Makefile, gnu/Makefile.am, gnu/Makefile.in, gnu/_Noreturn.h, gnu/alignof.h, gnu/alloca.h
- `gnu/sys`: 2 files total (0 C files, 2 header files, 0 other files). Examples: gnu/sys/stat.h, gnu/sys/types.h
- `m4`: 92 files total (0 C files, 0 header files, 92 other files). Examples: m4/00gnulib.m4, m4/absolute-header.m4, m4/alloca.m4, m4/assert_h.m4, m4/c-bool.m4, m4/calloc.m4
- `po`: 53 files total (0 C files, 0 header files, 53 other files). Examples: po/ChangeLog, po/LINGUAS, po/Makefile, po/Makefile.in, po/Makefile.in.in, po/Makevars
- `root`: 25 files total (0 C files, 1 header files, 24 other files). Examples: ABOUT-NLS, AUTHORS, COPYING, ChangeLog, ChangeLog.2007, INSTALL
- `src`: 27 files total (10 C files, 2 header files, 15 other files). Examples: src/Makefile, src/Makefile.am, src/Makefile.in, src/c.c, src/c.l, src/c.o
- `src/init`: 2 files total (0 C files, 0 header files, 2 other files). Examples: src/init/c11.cfo, src/init/gcc.cfo
- `src/parseopt`: 12 files total (4 C files, 3 header files, 5 other files). Examples: src/parseopt/.dirstamp, src/parseopt/help.c, src/parseopt/help.o, src/parseopt/optset.c, src/parseopt/optset.o, src/parseopt/parseconf.h
- `src/wordsplit`: 4 files total (1 C files, 1 header files, 2 other files). Examples: src/wordsplit/.dirstamp, src/wordsplit/wordsplit.c, src/wordsplit/wordsplit.h, src/wordsplit/wordsplit.o
- `test`: 6 files total (3 C files, 0 header files, 3 other files). Examples: test/cflow-basic.sh, test/cflow-flags.sh, test/cflow-formats.sh, test/multi.c, test/recursion.c, test/simple.c

## Source File Inventory
- `src/symbol.c`
- `src/c.c`
- `src/main.c` (entry candidate)
- `src/parser.c`
- `src/posix.c`
- `src/gnu.c`
- `src/dot.c`
- `src/depmap.c`
- `src/output.c`
- `src/linked-list.c`
- `src/wordsplit/wordsplit.c`
- `src/parseopt/wordwrap.c`
- `src/parseopt/parseopt.c`
- `src/parseopt/optset.c`
- `src/parseopt/help.c`
- `test/multi.c`
- `test/simple.c`
- `test/recursion.c`
- `gnu/dup2.c`
- `gnu/xmalloc.c`
- `gnu/realloc.c`
- `gnu/hash.c`
- `gnu/fd-hook.c`
- `gnu/basename-lgpl.c`
- `gnu/printf-args.c`
- `gnu/xsize.c`
- `gnu/snprintf.c`
- `gnu/close.c`
- `gnu/strerror.c`
- `gnu/cloexec.c`
- `gnu/vasnprintf.c`
- `gnu/asnprintf.c`
- `gnu/msvc-inval.c`
- `gnu/free.c`
- `gnu/bitrotate.c`
- `gnu/memchr.c`
- `gnu/open.c`
- `gnu/stat.c`
- `gnu/stdio-read.c`
- `gnu/msvc-nothrow.c`
- `gnu/malloca.c`
- `gnu/getdtablesize.c`
- `gnu/stat-time.c`
- `gnu/calloc.c`
- `gnu/malloc.c`
- `gnu/strerror-override.c`
- `gnu/fcntl.c`
- `gnu/mbszero.c`
- `gnu/xalloc-die.c`
- `gnu/unistd.c`
- `gnu/getprogname.c`
- `gnu/stat-w32.c`
- `gnu/progname.c`
- `gnu/ialloc.c`
- `gnu/printf-parse.c`
- `gnu/obstack.c`
- `gnu/itold.c`
- `gnu/stdio-write.c`
- `gnu/exitfail.c`
- `gnu/reallocarray.c`
- `gnu/float.c`
- `gnu/fstat.c`
- `gnu/error.c`
- `doc/d.c`
- `doc/whoami.c`
- `doc/ack.c`
- `doc/foo.c`
- `doc/wc.c`

## Header Files by Directory
### `gnu`
- `gnu/_Noreturn.h`
- `gnu/alignof.h`
- `gnu/alloca.h`
- `gnu/alloca.in.h`
- `gnu/arg-nonnull.h`
- `gnu/assert.h`
- `gnu/assert.in.h`
- `gnu/attribute.h`
- `gnu/basename-lgpl.h`
- `gnu/bitrotate.h`
- `gnu/c++defs.h`
- `gnu/cloexec.h`
- `gnu/errno.in.h`
- `gnu/error.h`
- `gnu/error.in.h`
- `gnu/exitfail.h`
- `gnu/fcntl.h`
- `gnu/fcntl.in.h`
- `gnu/fd-hook.h`
- `gnu/filename.h`
- `gnu/float+.h`
- `gnu/float.in.h`
- `gnu/getprogname.h`
- `gnu/gettext.h`
- `gnu/hash.h`
- `gnu/ialloc.h`
- `gnu/idx.h`
- `gnu/intprops-internal.h`
- `gnu/intprops.h`
- `gnu/inttypes.h`
- `gnu/inttypes.in.h`
- `gnu/limits.h`
- `gnu/limits.in.h`
- `gnu/malloca.h`
- `gnu/minmax.h`
- `gnu/msvc-inval.h`
- `gnu/msvc-nothrow.h`
- `gnu/obstack.h`
- `gnu/obstack.in.h`
- `gnu/pathmax.h`
### `gnu/sys`
- `gnu/sys/stat.h`
- `gnu/sys/types.h`
### `root`
- `config.h`
### `src`
- `src/cflow.h`
- `src/parser.h`
### `src/parseopt`
- `src/parseopt/parseconf.h`
- `src/parseopt/parseopt.h`
- `src/parseopt/wordwrap.h`
### `src/wordsplit`
- `src/wordsplit/wordsplit.h`

## README Excerpt
GNU cflow README
Copyright (C) 2005-2025 Sergey Poznyakoff
See the end of file for copying conditions.

* Introduction

This file contains brief information about configuring, and installing
GNU cflow. It is *not* intended as a replacement for the
documentation, instead it is provided as a brief reference only.
Please be sure to read the accompanuing documentation before using the
utility. See section `Documentation' below.

For the generic configuration options, see the file `INSTALL'.
Refer to file `ABOUT-NLS' for information regarding internationalization.

* History

The GNU cflow was initially written in 1997, when I needed a utility
that could display the control flow in a C program. The latest updates
in its old repo date back to 1999. Then, as I no longer needed the
package, it fell into oblivion.

Six years later a thread in gnu-devel mailing list had shown that
there is a kind of demand for that sort of utility, so I decided to
resurrect it and make a full-fledged package of it. On 2005-04-12 it
was dubbed a GNU package.

* Current Status

The package is fully operational. It compiles and runs on any
GNU/Linux and most decent UNIX-like systems. It supports all command
line switches required by POSIX, plus a number of extended ones. It is
able to produce output in two formats: in GNU cflow format (which is
the default) and in POSIX format.

Currently the utility is only able to process C sources. It is the
only deviation from POSIX specs, which requires ability to process
YACC and LEX sources as well as binary object files. This support will
appear in future versions.

Emacs module cflow-mode.el works with files in GNU cflow format (as
opposed to POSIX format). It has been tested with Emacs 24.2.1.

* Compilation

Please see the INSTALL file in this directory for the generic instructions
on how to use configure. There is currently only one package-specific
configuration option: --enable-debug, which compiles the package with
-g (or -ggdb if appropriate) opt
...[truncated]

## Evidence Boundaries
- This inventory comes from a filesystem scan and does not represent the final installation layout.
- Executables, libraries, or generated directories that were not observed will not be inferred to exist.
- File responsibilities and module behavior must be understood together with the later `01_subsystems` / `03_behaviors` documents.
