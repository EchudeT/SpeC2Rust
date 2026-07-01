# Interface Facts: module_include

This document is intended for the later Rust repository-level rewrite and keeps only the interface facts directly observed in the current source-analysis stage.
Header files, macros, error codes, and configuration items that do not appear in the current parse results will not be added or assumed.

## Module Scope
- Module category: `module`
- Directory: `include`
- File list: include/safe-read.c
- Candidate header files: include/alignalloc.h, include/alloca.h, include/arg-nonnull.h, include/attribute.h, include/basename-lgpl.h, include/binary-io.h, include/c-ctype.h, include/c-strcase.h, include/c-strcaseeq.h, include/c32is-impl.h, include/close-stream.h, include/closein.h, include/closeout.h, include/config.h, include/configmake.h, include/ctype.h, include/dirent.h, include/dirname.h, include/error.h, include/exitfail.h, include/fadvise.h, include/fcntl.h, include/filename.h, include/fpending.h, include/freading.h, include/full-write.h, include/gettext.h, include/hard-locale.h, include/ialloc.h, include/idx.h, include/ignore-value.h, include/intprops-internal.h, include/intprops.h, include/inttostr.h, include/inttypes.h, include/ioblksize.h, include/langinfo.h, include/limits.h, include/localcharset.h, include/locale.h, include/minmax.h, include/openat.h, include/pathmax.h, include/progname.h, include/propername.h, include/quote.h, include/quotearg.h, include/safe-read.h, include/safe-write.h, include/same-inode.h, include/setlocale_null.h, include/signal.h, include/stat-macros.h, include/stat-size.h, include/stdbit.h, include/stdckdint.h, include/stddef.h, include/stdio-impl.h, include/stdio.h, include/stdlib.h, include/streq.h, include/string.h, include/sys-limits.h, include/sys/types.h, include/system.h, include/time.h, include/timespec.h, include/uchar.h, include/unictype.h, include/unistd.h, include/unitypes.h, include/unlocked-io.h, include/verify.h, include/version-etc.h, include/version.h, include/wchar.h, include/wctype.h, include/xalloc.h, include/xbinary-io.h
- Exported functions observed: 1
- Struct definitions observed: 103
- Type names referenced but not defined locally: 0
- Macros observed in related files: 20
- Global variables observed: 0

## Header Evidence
- `include/alignalloc.h` [include/alignalloc.h]
- `include/alloca.h` [include/alloca.h]
- `include/arg-nonnull.h` [include/arg-nonnull.h]
- `include/attribute.h` [include/attribute.h]
- `include/basename-lgpl.h` [include/basename-lgpl.h]
- `include/binary-io.h` [include/binary-io.h]
- `include/c-ctype.h` [include/c-ctype.h]
- `include/c-strcase.h` [include/c-strcase.h]
- `include/c-strcaseeq.h` [include/c-strcaseeq.h]
- `include/c32is-impl.h` [include/c32is-impl.h]
- `include/close-stream.h` [include/close-stream.h]
- `include/closein.h` [include/closein.h]
- `include/closeout.h` [include/closeout.h]
- `include/config.h` [include/config.h]
- `include/configmake.h` [include/configmake.h]
- `include/ctype.h` [include/ctype.h]
- `include/dirent.h` [include/dirent.h]
- `include/dirname.h` [include/dirname.h]
- `include/error.h` [include/error.h]
- `include/exitfail.h` [include/exitfail.h]
- `include/fadvise.h` [include/fadvise.h]
- `include/fcntl.h` [include/fcntl.h]
- `include/filename.h` [include/filename.h]
- `include/fpending.h` [include/fpending.h]
- `include/freading.h` [include/freading.h]
- `include/full-write.h` [include/full-write.h]
- `include/gettext.h` [include/gettext.h]
- `include/hard-locale.h` [include/hard-locale.h]
- `include/ialloc.h` [include/ialloc.h]
- `include/idx.h` [include/idx.h]
- `include/ignore-value.h` [include/ignore-value.h]
- `include/intprops-internal.h` [include/intprops-internal.h]
- `include/intprops.h` [include/intprops.h]
- `include/inttostr.h` [include/inttostr.h]
- `include/inttypes.h` [include/inttypes.h]
- `include/ioblksize.h` [include/ioblksize.h]
- `include/langinfo.h` [include/langinfo.h]
- `include/limits.h` [include/limits.h]
- `include/localcharset.h` [include/localcharset.h]
- `include/locale.h` [include/locale.h]
- `include/minmax.h` [include/minmax.h]
- `include/openat.h` [include/openat.h]
- `include/pathmax.h` [include/pathmax.h]
- `include/progname.h` [include/progname.h]
- `include/propername.h` [include/propername.h]
- `include/quote.h` [include/quote.h]
- `include/quotearg.h` [include/quotearg.h]
- `include/safe-read.h` [include/safe-read.h]
- `include/safe-write.h` [include/safe-write.h]
- `include/same-inode.h` [include/same-inode.h]
- `include/setlocale_null.h` [include/setlocale_null.h]
- `include/signal.h` [include/signal.h]
- `include/stat-macros.h` [include/stat-macros.h]
- `include/stat-size.h` [include/stat-size.h]
- `include/stdbit.h` [include/stdbit.h]
- `include/stdckdint.h` [include/stdckdint.h]
- `include/stddef.h` [include/stddef.h]
- `include/stdio-impl.h` [include/stdio-impl.h]
- `include/stdio.h` [include/stdio.h]
- `include/stdlib.h` [include/stdlib.h]
- `include/streq.h` [include/streq.h]
- `include/string.h` [include/string.h]
- `include/sys-limits.h` [include/sys-limits.h]
- `include/sys/types.h` [include/sys/types.h]
- `include/system.h` [include/system.h]
- `include/time.h` [include/time.h]
- `include/timespec.h` [include/timespec.h]
- `include/uchar.h` [include/uchar.h]
- `include/unictype.h` [include/unictype.h]
- `include/unistd.h` [include/unistd.h]
- `include/unitypes.h` [include/unitypes.h]
- `include/unlocked-io.h` [include/unlocked-io.h]
- `include/verify.h` [include/verify.h]
- `include/version-etc.h` [include/version-etc.h]
- `include/version.h` [include/version.h]
- `include/wchar.h` [include/wchar.h]
- `include/wctype.h` [include/wctype.h]
- `include/xalloc.h` [include/xalloc.h]
- `include/xbinary-io.h` [include/xbinary-io.h]

## Functions
### `safe_rw`
- Definition location: [include/safe-read.c:55-71]
- Source file: `include/safe-read.c`
- Observed declaration: `size_t safe_rw (int fd, void const *buf, size_t count);`
- Approximate function body length: 17 lines

## Structs and Types
### `anonymous`
- Definition location: [include/dirent.h:45-49]
- Source file: `include/dirent.h`
- Observed declaration prefix: `struct dirent`
### `anonymous`
- Definition location: [include/dirent.h:68]
- Source file: `include/dirent.h`
- Observed declaration prefix: `struct gl_directory`
### `anonymous`
- Definition location: [include/dirent.h:72]
- Source file: `include/dirent.h`
- Observed declaration prefix: `struct gl_directory`
### `anonymous`
- Definition location: [include/dirent.h:850]
- Source file: `include/dirent.h`
- Observed declaration prefix: `struct dirent`
### `anonymous`
- Definition location: [include/dirent.h:850]
- Source file: `include/dirent.h`
- Observed declaration prefix: `struct dirent`
### `anonymous`
- Definition location: [include/dirent.h:857]
- Source file: `include/dirent.h`
- Observed declaration prefix: `struct dirent`
### `anonymous`
- Definition location: [include/dirent.h:857]
- Source file: `include/dirent.h`
- Observed declaration prefix: `struct dirent`
### `anonymous`
- Definition location: [include/inttypes.h:1455]
- Source file: `include/inttypes.h`
- Observed declaration prefix: `struct`
### `anonymous`
- Definition location: [include/ioblksize.h:80]
- Source file: `include/ioblksize.h`
- Observed declaration prefix: `struct stat`
### `anonymous`
- Definition location: [include/locale.h:604-679]
- Source file: `include/locale.h`
- Observed declaration prefix: `struct lconv`
### `anonymous`
- Definition location: [include/openat.h:110]
- Source file: `include/openat.h`
- Observed declaration prefix: `struct stat`
### `anonymous`
- Definition location: [include/openat.h:117]
- Source file: `include/openat.h`
- Observed declaration prefix: `struct stat`
### `anonymous`
- Definition location: [include/quote.h:30]
- Source file: `include/quote.h`
- Observed declaration prefix: `struct quoting_options`
### `anonymous`
- Definition location: [include/quotearg.h:286]
- Source file: `include/quotearg.h`
- Observed declaration prefix: `struct quoting_options`
### `anonymous`
- Definition location: [include/quotearg.h:294]
- Source file: `include/quotearg.h`
- Observed declaration prefix: `struct quoting_options`
### `anonymous`
- Definition location: [include/quotearg.h:294]
- Source file: `include/quotearg.h`
- Observed declaration prefix: `struct quoting_options`
### `anonymous`
- Definition location: [include/quotearg.h:299]
- Source file: `include/quotearg.h`
- Observed declaration prefix: `struct quoting_options`
### `anonymous`
- Definition location: [include/quotearg.h:303]
- Source file: `include/quotearg.h`
- Observed declaration prefix: `struct quoting_options`
### `anonymous`
- Definition location: [include/quotearg.h:312]
- Source file: `include/quotearg.h`
- Observed declaration prefix: `struct quoting_options`
### `anonymous`
- Definition location: [include/quotearg.h:318]
- Source file: `include/quotearg.h`
- Observed declaration prefix: `struct quoting_options`
### `anonymous`
- Definition location: [include/quotearg.h:329]
- Source file: `include/quotearg.h`
- Observed declaration prefix: `struct quoting_options`
### `anonymous`
- Definition location: [include/quotearg.h:346]
- Source file: `include/quotearg.h`
- Observed declaration prefix: `struct quoting_options`
### `anonymous`
- Definition location: [include/quotearg.h:352]
- Source file: `include/quotearg.h`
- Observed declaration prefix: `struct quoting_options`
### `anonymous`
- Definition location: [include/quotearg.h:363]
- Source file: `include/quotearg.h`
- Observed declaration prefix: `struct quoting_options`
### `anonymous`
- Definition location: [include/same-inode.h:83]
- Source file: `include/same-inode.h`
- Observed declaration prefix: `struct stat`
### `anonymous`
- Definition location: [include/same-inode.h:83]
- Source file: `include/same-inode.h`
- Observed declaration prefix: `struct stat`
### `anonymous`
- Definition location: [include/signal.h:939-950]
- Source file: `include/signal.h`
- Observed declaration prefix: `struct siginfo_t`
### `anonymous`
- Definition location: [include/signal.h:951]
- Source file: `include/signal.h`
- Observed declaration prefix: `struct siginfo_t`
### `anonymous`
- Definition location: [include/signal.h:963-977]
- Source file: `include/signal.h`
- Observed declaration prefix: `struct sigaction`
### `anonymous`
- Definition location: [include/signal.h:988]
- Source file: `include/signal.h`
- Observed declaration prefix: `struct sigaction`
### `anonymous`
- Definition location: [include/signal.h:989]
- Source file: `include/signal.h`
- Observed declaration prefix: `struct sigaction`
### `anonymous`
- Definition location: [include/signal.h:997]
- Source file: `include/signal.h`
- Observed declaration prefix: `struct sigaction`
### `anonymous`
- Definition location: [include/signal.h:998]
- Source file: `include/signal.h`
- Observed declaration prefix: `struct sigaction`
### `anonymous`
- Definition location: [include/stdio-impl.h:116-120]
- Source file: `include/stdio-impl.h`
- Observed declaration prefix: `struct __sfileext`
### `anonymous`
- Definition location: [include/stdio-impl.h:118]
- Source file: `include/stdio-impl.h`
- Observed declaration prefix: `struct __sbuf`
### `anonymous`
- Definition location: [include/stdio-impl.h:123-127]
- Source file: `include/stdio-impl.h`
- Observed declaration prefix: `struct __sfileext`
### `anonymous`
- Definition location: [include/stdio-impl.h:125]
- Source file: `include/stdio-impl.h`
- Observed declaration prefix: `struct`
### `anonymous`
- Definition location: [include/stdio-impl.h:196-207]
- Source file: `include/stdio-impl.h`
- Observed declaration prefix: `struct _gl_real_FILE`
### `anonymous`
- Definition location: [include/stdio.h:1596]
- Source file: `include/stdio.h`
- Observed declaration prefix: `struct obstack`
### `anonymous`
- Definition location: [include/stdio.h:1607]
- Source file: `include/stdio.h`
- Observed declaration prefix: `struct obstack`
### `anonymous`
- Definition location: [include/stdio.h:1611]
- Source file: `include/stdio.h`
- Observed declaration prefix: `struct obstack`
### `anonymous`
- Definition location: [include/stdio.h:1615]
- Source file: `include/stdio.h`
- Observed declaration prefix: `struct obstack`
### `anonymous`
- Definition location: [include/stdio.h:1620]
- Source file: `include/stdio.h`
- Observed declaration prefix: `struct obstack`
### `anonymous`
- Definition location: [include/stdio.h:1628]
- Source file: `include/stdio.h`
- Observed declaration prefix: `struct obstack`
### `anonymous`
- Definition location: [include/stdio.h:1632]
- Source file: `include/stdio.h`
- Observed declaration prefix: `struct obstack`
### `anonymous`
- Definition location: [include/stdio.h:1636]
- Source file: `include/stdio.h`
- Observed declaration prefix: `struct obstack`
### `anonymous`
- Definition location: [include/stdio.h:1641]
- Source file: `include/stdio.h`
- Observed declaration prefix: `struct obstack`
### `anonymous`
- Definition location: [include/stdlib.h:84-93]
- Source file: `include/stdlib.h`
- Observed declaration prefix: `struct random_data`
### `anonymous`
- Definition location: [include/stdlib.h:1839]
- Source file: `include/stdlib.h`
- Observed declaration prefix: `struct random_data`
### `anonymous`
- Definition location: [include/stdlib.h:1841]
- Source file: `include/stdlib.h`
- Observed declaration prefix: `struct random_data`
### `anonymous`
- Definition location: [include/stdlib.h:1844]
- Source file: `include/stdlib.h`
- Observed declaration prefix: `struct random_data`
### `anonymous`
- Definition location: [include/stdlib.h:1847]
- Source file: `include/stdlib.h`
- Observed declaration prefix: `struct random_data`
### `anonymous`
- Definition location: [include/sys/types.h:92]
- Source file: `include/sys/types.h`
- Observed declaration prefix: `struct`
### `anonymous`
- Definition location: [include/system.h:207]
- Source file: `include/system.h`
- Observed declaration prefix: `struct passwd`
### `anonymous`
- Definition location: [include/system.h:211]
- Source file: `include/system.h`
- Observed declaration prefix: `struct group`
### `anonymous`
- Definition location: [include/system.h:273]
- Source file: `include/system.h`
- Observed declaration prefix: `struct dirent`
### `anonymous`
- Definition location: [include/system.h:278]
- Source file: `include/system.h`
- Observed declaration prefix: `struct dirent`
### `anonymous`
- Definition location: [include/system.h:659]
- Source file: `include/system.h`
- Observed declaration prefix: `struct infomap`
### `anonymous`
- Definition location: [include/system.h:670]
- Source file: `include/system.h`
- Observed declaration prefix: `struct infomap`
### `anonymous`
- Definition location: [include/system.h:732]
- Source file: `include/system.h`
- Observed declaration prefix: `struct stat`
### `anonymous`
- Definition location: [include/time.h:607-611]
- Source file: `include/time.h`
- Observed declaration prefix: `struct timespec`
### `anonymous`
- Definition location: [include/time.h:628-630]
- Source file: `include/time.h`
- Observed declaration prefix: `struct __time_t_must_be_integral`
### `anonymous`
- Definition location: [include/time.h:650]
- Source file: `include/time.h`
- Observed declaration prefix: `struct timespec`
### `anonymous`
- Definition location: [include/time.h:652]
- Source file: `include/time.h`
- Observed declaration prefix: `struct timespec`
### `anonymous`
- Definition location: [include/time.h:655]
- Source file: `include/time.h`
- Observed declaration prefix: `struct timespec`
### `anonymous`
- Definition location: [include/time.h:658]
- Source file: `include/time.h`
- Observed declaration prefix: `struct timespec`
### `anonymous`
- Definition location: [include/time.h:679]
- Source file: `include/time.h`
- Observed declaration prefix: `struct timespec`
### `anonymous`
- Definition location: [include/time.h:681]
- Source file: `include/time.h`
- Observed declaration prefix: `struct timespec`
### `anonymous`
- Definition location: [include/time.h:684]
- Source file: `include/time.h`
- Observed declaration prefix: `struct timespec`
### `anonymous`
- Definition location: [include/time.h:687]
- Source file: `include/time.h`
- Observed declaration prefix: `struct timespec`
### `anonymous`
- Definition location: [include/time.h:729]
- Source file: `include/time.h`
- Observed declaration prefix: `struct timespec`
### `anonymous`
- Definition location: [include/time.h:732]
- Source file: `include/time.h`
- Observed declaration prefix: `struct timespec`
### `anonymous`
- Definition location: [include/time.h:736]
- Source file: `include/time.h`
- Observed declaration prefix: `struct timespec`
### `anonymous`
- Definition location: [include/time.h:740]
- Source file: `include/time.h`
- Observed declaration prefix: `struct timespec`
### `anonymous`
- Definition location: [include/time.h:798]
- Source file: `include/time.h`
- Observed declaration prefix: `struct tm`
### `anonymous`
- Definition location: [include/time.h:799]
- Source file: `include/time.h`
- Observed declaration prefix: `struct tm`
### `anonymous`
- Definition location: [include/time.h:801]
- Source file: `include/time.h`
- Observed declaration prefix: `struct tm`
### `anonymous`
- Definition location: [include/time.h:998]
- Source file: `include/time.h`
- Observed declaration prefix: `struct tm_zone`
### `anonymous`
- Definition location: [include/time.h:1055]
- Source file: `include/time.h`
- Observed declaration prefix: `struct tm`
### `anonymous`
- Definition location: [include/time.h:1056]
- Source file: `include/time.h`
- Observed declaration prefix: `struct tm`
### `anonymous`
- Definition location: [include/time.h:1059]
- Source file: `include/time.h`
- Observed declaration prefix: `struct tm`
### `anonymous`
- Definition location: [include/time.h:1061]
- Source file: `include/time.h`
- Observed declaration prefix: `struct tm`
### `anonymous`
- Definition location: [include/timespec.h:58]
- Source file: `include/timespec.h`
- Observed declaration prefix: `struct timespec`
### `anonymous`
- Definition location: [include/timespec.h:64]
- Source file: `include/timespec.h`
- Observed declaration prefix: `struct timespec`
### `anonymous`
- Definition location: [include/timespec.h:64]
- Source file: `include/timespec.h`
- Observed declaration prefix: `struct timespec`
### `anonymous`
- Definition location: [include/timespec.h:72]
- Source file: `include/timespec.h`
- Observed declaration prefix: `struct timespec`
### `anonymous`
- Definition location: [include/timespec.h:77]
- Source file: `include/timespec.h`
- Observed declaration prefix: `struct timespec`
### `anonymous`
- Definition location: [include/timespec.h:77]
- Source file: `include/timespec.h`
- Observed declaration prefix: `struct timespec`
### `anonymous`
- Definition location: [include/timespec.h:77]
- Source file: `include/timespec.h`
- Observed declaration prefix: `struct timespec`
### `anonymous`
- Definition location: [include/timespec.h:79]
- Source file: `include/timespec.h`
- Observed declaration prefix: `struct timespec`
### `anonymous`
- Definition location: [include/timespec.h:79]
- Source file: `include/timespec.h`
- Observed declaration prefix: `struct timespec`
### `anonymous`
- Definition location: [include/timespec.h:79]
- Source file: `include/timespec.h`
- Observed declaration prefix: `struct timespec`
### `anonymous`
- Definition location: [include/timespec.h:81]
- Source file: `include/timespec.h`
- Observed declaration prefix: `struct timespec`
### `anonymous`
- Definition location: [include/timespec.h:86]
- Source file: `include/timespec.h`
- Observed declaration prefix: `struct timespec`
### `anonymous`
- Definition location: [include/timespec.h:92]
- Source file: `include/timespec.h`
- Observed declaration prefix: `struct timespec`
### `anonymous`
- Definition location: [include/timespec.h:93]
- Source file: `include/timespec.h`
- Observed declaration prefix: `struct timespec`
### `anonymous`
- Definition location: [include/timespec.h:94]
- Source file: `include/timespec.h`
- Observed declaration prefix: `struct timespec`
### `anonymous`
- Definition location: [include/unictype.h:52-61]
- Source file: `include/unictype.h`
- Observed declaration prefix: `struct`
### `anonymous`
- Definition location: [include/unictype.h:399-403]
- Source file: `include/unictype.h`
- Observed declaration prefix: `struct`
### `anonymous`
- Definition location: [include/unictype.h:604-607]
- Source file: `include/unictype.h`
- Observed declaration prefix: `struct`
### `anonymous`
- Definition location: [include/unictype.h:961-966]
- Source file: `include/unictype.h`
- Observed declaration prefix: `struct`
### `anonymous`
- Definition location: [include/unictype.h:968-973]
- Source file: `include/unictype.h`
- Observed declaration prefix: `struct`
### `anonymous`
- Definition location: [include/unictype.h:999-1004]
- Source file: `include/unictype.h`
- Observed declaration prefix: `struct`

## Referenced External Types
- No external struct or type references beyond local definitions were recorded.

## Macros and Constants
- `ALIGNALLOC_H_` [include/alignalloc.h:21]: `#define ALIGNALLOC_H_`
- `_GL_ALLOCA_H` [include/alloca.h:23]: `#define _GL_ALLOCA_H`
- `_GL_ATTRIBUTE_H` [include/attribute.h:31]: `#define _GL_ATTRIBUTE_H`
- `DEPRECATED` [include/attribute.h:70]: `#define DEPRECATED _GL_ATTRIBUTE_DEPRECATED`
- `ATTRIBUTE_WARNING` [include/attribute.h:74]: `#define ATTRIBUTE_WARNING(msg) _GL_ATTRIBUTE_WARNING (msg)`
- `ATTRIBUTE_ERROR` [include/attribute.h:78]: `#define ATTRIBUTE_ERROR(msg) _GL_ATTRIBUTE_ERROR (msg)`
- `ATTRIBUTE_MALLOC` [include/attribute.h:85]: `#define ATTRIBUTE_MALLOC _GL_ATTRIBUTE_MALLOC`
- `ATTRIBUTE_ALLOC_SIZE` [include/attribute.h:92]: `#define ATTRIBUTE_ALLOC_SIZE(args) _GL_ATTRIBUTE_ALLOC_SIZE (args)`
- `ATTRIBUTE_DEALLOC` [include/attribute.h:100]: `#define ATTRIBUTE_DEALLOC(f, i) _GL_ATTRIBUTE_DEALLOC(f, i)`
- `ATTRIBUTE_DEALLOC_FREE` [include/attribute.h:101]: `#define ATTRIBUTE_DEALLOC_FREE _GL_ATTRIBUTE_DEALLOC_FREE`
- `ATTRIBUTE_SENTINEL` [include/attribute.h:109]: `#define ATTRIBUTE_SENTINEL(pos) _GL_ATTRIBUTE_SENTINEL (pos)`
- `ATTRIBUTE_FORMAT` [include/attribute.h:128]: `#define ATTRIBUTE_FORMAT(spec) _GL_ATTRIBUTE_FORMAT (spec)`
- `ATTRIBUTE_NONNULL` [include/attribute.h:133]: `#define ATTRIBUTE_NONNULL(args) _GL_ATTRIBUTE_NONNULL (args)`
- `ATTRIBUTE_RETURNS_NONNULL` [include/attribute.h:137]: `#define ATTRIBUTE_RETURNS_NONNULL _GL_ATTRIBUTE_RETURNS_NONNULL`
- `NODISCARD` [include/attribute.h:142]: `#define NODISCARD _GL_ATTRIBUTE_NODISCARD`
- `MAYBE_UNUSED` [include/attribute.h:155]: `#define MAYBE_UNUSED _GL_ATTRIBUTE_MAYBE_UNUSED`
- `ATTRIBUTE_NONSTRING` [include/attribute.h:160]: `#define ATTRIBUTE_NONSTRING _GL_ATTRIBUTE_NONSTRING`
- `FALLTHROUGH` [include/attribute.h:165]: `#define FALLTHROUGH _GL_ATTRIBUTE_FALLTHROUGH`
- `ATTRIBUTE_ARTIFICIAL` [include/attribute.h:174]: `#define ATTRIBUTE_ARTIFICIAL _GL_ATTRIBUTE_ARTIFICIAL`
- `ATTRIBUTE_EXTERNALLY_VISIBLE` [include/attribute.h:178]: `#define ATTRIBUTE_EXTERNALLY_VISIBLE _GL_ATTRIBUTE_EXTERNALLY_VISIBLE`

## Global Variables
- No global variable definitions were observed in the current module's `.c` files.

## Known Gaps
- This document is generated from parsed results for functions, structs, macros, and global variables; it does not infer declaration signatures from `.h` files that were not parsed.
- If a function appears in the "Functions" section without an explicit header binding, the later Rust migration should re-check the corresponding source `#include` relationships and build scripts.
- Error codes, configuration items, and input/output protocols are recorded only when explicit symbols appear in the source; missing entries do not mean the semantics do not exist, only that the current fact extraction did not observe them.
