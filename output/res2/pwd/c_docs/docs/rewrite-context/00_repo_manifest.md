# pwd Repository Inventory

This document records only the facts directly observed during the current repository scan. It does not fill in missing directory trees or guess at artifacts that have not appeared yet.

## Snapshot
- Project name: `pwd`
- Build system: `Makefile`
- C file count: 28
- Header file count: 73
- Other file count: 33
- Build files: Makefile
- Entry files: none
- Executables observed at the repository root: pwd
- Library files observed at the repository root: none

## Directory Inventory
- `include`: 69 files total (0 C files, 69 header files, 0 other files). Examples: include/alloca.h, include/arg-nonnull.h, include/attribute.h, include/basename-lgpl.h, include/c-ctype.h, include/c-strcase.h
- `include/sys`: 4 files total (0 C files, 4 header files, 0 other files). Examples: include/sys/select.h, include/sys/stat.h, include/sys/time.h, include/sys/types.h
- `root`: 59 files total (28 C files, 0 header files, 31 other files). Examples: .gitignore, Makefile, c-ctype.c, c-ctype.o, c-strcasecmp.c, c-strcasecmp.o
- `test`: 2 files total (0 C files, 0 header files, 2 other files). Examples: test/pwd-long.sh, test/pwd-option.sh

## Source File Inventory
- `hard-locale.c`
- `fclose.c`
- `xmalloc.c`
- `version.c`
- `propername-lite.c`
- `setlocale_null-unlocked.c`
- `quotearg.c`
- `localcharset.c`
- `same-inode.c`
- `version-etc-fsf.c`
- `root-dev-ino.c`
- `version-etc.c`
- `mbrtoc32.c`
- `closeout.c`
- `c-ctype.c`
- `fflush.c`
- `mbszero.c`
- `fseeko.c`
- `xalloc-die.c`
- `xgetcwd.c`
- `pwd.c`
- `progname.c`
- `setlocale_null.c`
- `c-strcasecmp.c`
- `ialloc.c`
- `exitfail.c`
- `c32isprint.c`
- `close-stream.c`

## Header Files by Directory
### `include`
- `include/alloca.h`
- `include/arg-nonnull.h`
- `include/attribute.h`
- `include/basename-lgpl.h`
- `include/c-ctype.h`
- `include/c-strcase.h`
- `include/c-strcaseeq.h`
- `include/c32is-impl.h`
- `include/close-stream.h`
- `include/closein.h`
- `include/closeout.h`
- `include/config.h`
- `include/configmake.h`
- `include/ctype.h`
- `include/dev-ino.h`
- `include/dirent.h`
- `include/dirname.h`
- `include/error.h`
- `include/exitfail.h`
- `include/fcntl.h`
- `include/filename.h`
- `include/fpending.h`
- `include/freading.h`
- `include/gettext.h`
- `include/hard-locale.h`
- `include/ialloc.h`
- `include/idx.h`
- `include/intprops-internal.h`
- `include/intprops.h`
- `include/inttostr.h`
- `include/inttypes.h`
- `include/langinfo.h`
- `include/limits.h`
- `include/localcharset.h`
- `include/locale.h`
- `include/minmax.h`
- `include/openat.h`
- `include/pathmax.h`
- `include/progname.h`
- `include/propername.h`
### `include/sys`
- `include/sys/select.h`
- `include/sys/stat.h`
- `include/sys/time.h`
- `include/sys/types.h`

## README Excerpt
No README file was found.

## Evidence Boundaries
- This inventory comes from a filesystem scan and does not represent the final installation layout.
- Executables, libraries, or generated directories that were not observed will not be inferred to exist.
- File responsibilities and module behavior must be understood together with the later `01_subsystems` / `03_behaviors` documents.
