# Interface Facts: main_root_stat_03

This document is intended for the later Rust repository-level rewrite and keeps only the interface facts directly observed in the current source-analysis stage.
Header files, macros, error codes, and configuration items that do not appear in the current parse results will not be added or assumed.

## Module Scope
- Module category: `main_cluster`
- Directory: `root`
- File list: cat.c, fcntl.c
- Candidate header files: include/alignalloc.h, include/config.h, include/fadvise.h, include/fcntl.h, include/full-write.h, include/ioblksize.h, include/limits.h, include/safe-read.h, include/stdio.h, include/stdlib.h, include/sys/ioctl.h, include/sys/types.h, include/system.h, include/unistd.h, include/xbinary-io.h
- Exported functions observed: 2
- Struct definitions observed: 26
- Type names referenced but not defined locally: 2
- Macros observed in related files: 20
- Global variables observed: 0

## Header Evidence
- `include/alignalloc.h` [include/alignalloc.h]
- `include/config.h` [include/config.h]
- `include/fadvise.h` [include/fadvise.h]
- `include/fcntl.h` [include/fcntl.h]
- `include/full-write.h` [include/full-write.h]
- `include/ioblksize.h` [include/ioblksize.h]
- `include/limits.h` [include/limits.h]
- `include/safe-read.h` [include/safe-read.h]
- `include/stdio.h` [include/stdio.h]
- `include/stdlib.h` [include/stdlib.h]
- `include/sys/ioctl.h` [include/sys/ioctl.h]
- `include/sys/types.h` [include/sys/types.h]
- `include/system.h` [include/system.h]
- `include/unistd.h` [include/unistd.h]
- `include/xbinary-io.h` [include/xbinary-io.h]

## Functions
### `main`
- Definition location: [cat.c:535-813]
- Source file: `cat.c`
- Observed declaration: `nt main (int argc, char **argv);`
- Approximate function body length: 279 lines
### `klibc_fcntl`
- Definition location: [fcntl.c:550-627]
- Source file: `fcntl.c`
- Observed declaration: `static int klibc_fcntl (int fd, int action, /* arg */...);`
- Approximate function body length: 78 lines

## Structs and Types
### `anonymous`
- Definition location: [cat.c:541]
- Source file: `cat.c`
- Observed declaration prefix: `truct stat`
### `anonymous`
- Definition location: [cat.c:552]
- Source file: `cat.c`
- Observed declaration prefix: `truct option`
### `anonymous`
- Definition location: [fcntl.c:555]
- Source file: `fcntl.c`
- Observed declaration prefix: `struct stat`
### `anonymous`
- Definition location: [include/ioblksize.h:80]
- Source file: `include/ioblksize.h`
- Observed declaration prefix: `struct stat`
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

## Referenced External Types
- `option`: this name came from clustering metadata or nearby call analysis, but no local definition was observed in the current module files.
- `stat`: this name came from clustering metadata or nearby call analysis, but no local definition was observed in the current module files.

## Macros and Constants
- `PROGRAM_NAME` [cat.c:44]: `#define PROGRAM_NAME "cat"`
- `AUTHORS` [cat.c:46-48]: `#define AUTHORS \ proper_name_lite ("Torbjorn Granlund", "Torbj\303\266rn Granlund"), \ proper_name ("Richard M. Stallman")`
- `LINE_COUNTER_BUF_LEN` [cat.c:59]: `#define LINE_COUNTER_BUF_LEN 20`
- `ALIGNALLOC_H_` [include/alignalloc.h:21]: `#define ALIGNALLOC_H_`
- `_GL_CONFIG_H_INCLUDED` [include/config.h:5]: `#define _GL_CONFIG_H_INCLUDED 1`
- `ARGMATCH_DIE` [include/config.h:15]: `#define ARGMATCH_DIE usage (EXIT_FAILURE)`
- `ARGMATCH_DIE_DECL` [include/config.h:18]: `#define ARGMATCH_DIE_DECL void usage (int _e)`
- `BITSIZEOF_WCHAR_T` [include/config.h:36]: `#define BITSIZEOF_WCHAR_T 32`
- `CHECK_PRINTF_SAFE` [include/config.h:46]: `#define CHECK_PRINTF_SAFE 1`
- `DBL_EXPBIT0_BIT` [include/config.h:66]: `#define DBL_EXPBIT0_BIT 20`
- `DBL_EXPBIT0_WORD` [include/config.h:69]: `#define DBL_EXPBIT0_WORD 1`
- `DLOPEN_LIBCRYPTO` [include/config.h:110]: `#define DLOPEN_LIBCRYPTO 1`
- `D_INO_IN_DIRENT` [include/config.h:116]: `#define D_INO_IN_DIRENT 1`
- `ENABLE_NLS` [include/config.h:120]: `#define ENABLE_NLS 1`
- `FLEXIBLE_ARRAY_MEMBER` [include/config.h:138]: `#define FLEXIBLE_ARRAY_MEMBER /**/`
- `FLOAT16_SUPPORTED` [include/config.h:141]: `#define FLOAT16_SUPPORTED 1`
- `FLT_EXPBIT0_BIT` [include/config.h:145]: `#define FLT_EXPBIT0_BIT 23`
- `FLT_EXPBIT0_WORD` [include/config.h:148]: `#define FLT_EXPBIT0_WORD 0`
- `FUNC_FFLUSH_STDIN` [include/config.h:174]: `#define FUNC_FFLUSH_STDIN 0`
- `FUNC_NL_LANGINFO_YESEXPR_WORKS` [include/config.h:181]: `#define FUNC_NL_LANGINFO_YESEXPR_WORKS 1`

## Global Variables
- No global variable definitions were observed in the current module's `.c` files.

## Known Gaps
- This document is generated from parsed results for functions, structs, macros, and global variables; it does not infer declaration signatures from `.h` files that were not parsed.
- If a function appears in the "Functions" section without an explicit header binding, the later Rust migration should re-check the corresponding source `#include` relationships and build scripts.
- Error codes, configuration items, and input/output protocols are recorded only when explicit symbols appear in the source; missing entries do not mean the semantics do not exist, only that the current fact extraction did not observe them.
