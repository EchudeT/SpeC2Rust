# cat Repository Inventory

This document records only the facts directly observed during the current repository scan. It does not fill in missing directory trees or guess at artifacts that have not appeared yet.

## Snapshot
- Project name: `cat`
- Build system: `Makefile`
- C file count: 39
- Header file count: 84
- Other file count: 47
- Build files: Makefile
- Entry files: none
- Executables observed at the repository root: cat
- Library files observed at the repository root: none

## Directory Inventory
- `include`: 79 files total (1 C files, 78 header files, 0 other files). Examples: include/alignalloc.h, include/alloca.h, include/arg-nonnull.h, include/attribute.h, include/basename-lgpl.h, include/binary-io.h
- `include/sys`: 6 files total (0 C files, 6 header files, 0 other files). Examples: include/sys/ioctl.h, include/sys/select.h, include/sys/stat.h, include/sys/time.h, include/sys/types.h, include/sys/utsname.h
- `root`: 81 files total (38 C files, 0 header files, 43 other files). Examples: .gitignore, Makefile, alignalloc.c, alignalloc.o, binary-io.c, binary-io.o
- `test`: 4 files total (0 C files, 0 header files, 4 other files). Examples: test/cat-E.sh, test/cat-buf.sh, test/cat-proc.sh, test/cat-self.sh

## Source File Inventory
- `hard-locale.c`
- `fclose.c`
- `xmalloc.c`
- `stdbit.c`
- `safe-write.c`
- `version.c`
- `propername-lite.c`
- `setlocale_null-unlocked.c`
- `quotearg.c`
- `safe-read.c`
- `xalignalloc.c`
- `binary-io.c`
- `localcharset.c`
- `version-etc-fsf.c`
- `version-etc.c`
- `mbrtoc32.c`
- `stdc_leading_zeros.c`
- `full-write.c`
- `closeout.c`
- `c-ctype.c`
- `fcntl.c`
- `fflush.c`
- `mbszero.c`
- `fseeko.c`
- `xalloc-die.c`
- `progname.c`
- `xbinary-io.c`
- `fadvise.c`
- `setlocale_null.c`
- `alignalloc.c`
- `c-strcasecmp.c`
- `ialloc.c`
- `cat.c`
- `exitfail.c`
- `c32isprint.c`
- `fpurge.c`
- `copy-file-range.c`
- `close-stream.c`
- `include/safe-read.c`

## Header Files by Directory
### `include`
- `include/alignalloc.h`
- `include/alloca.h`
- `include/arg-nonnull.h`
- `include/attribute.h`
- `include/basename-lgpl.h`
- `include/binary-io.h`
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
- `include/dirent.h`
- `include/dirname.h`
- `include/error.h`
- `include/exitfail.h`
- `include/fadvise.h`
- `include/fcntl.h`
- `include/filename.h`
- `include/fpending.h`
- `include/freading.h`
- `include/full-write.h`
- `include/gettext.h`
- `include/hard-locale.h`
- `include/ialloc.h`
- `include/idx.h`
- `include/ignore-value.h`
- `include/intprops-internal.h`
- `include/intprops.h`
- `include/inttostr.h`
- `include/inttypes.h`
- `include/ioblksize.h`
- `include/langinfo.h`
- `include/limits.h`
- `include/localcharset.h`
- `include/locale.h`
### `include/sys`
- `include/sys/ioctl.h`
- `include/sys/select.h`
- `include/sys/stat.h`
- `include/sys/time.h`
- `include/sys/types.h`
- `include/sys/utsname.h`

## README Excerpt
No README file was found.

## Evidence Boundaries
- This inventory comes from a filesystem scan and does not represent the final installation layout.
- Executables, libraries, or generated directories that were not observed will not be inferred to exist.
- File responsibilities and module behavior must be understood together with the later `01_subsystems` / `03_behaviors` documents.
