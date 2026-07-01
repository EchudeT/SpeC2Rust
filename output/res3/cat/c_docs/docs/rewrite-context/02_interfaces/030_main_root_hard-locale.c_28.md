# Interface Facts: main_root_hard-locale.c_28

This document is intended for the later Rust repository-level rewrite and keeps only the interface facts directly observed in the current source-analysis stage.
Header files, macros, error codes, and configuration items that do not appear in the current parse results will not be added or assumed.

## Module Scope
- Module category: `main_cluster`
- Directory: `root`
- File list: hard-locale.c
- Candidate header files: include/config.h, include/hard-locale.h, include/locale.h, include/stdlib.h, include/string.h
- Exported functions observed: 1
- Struct definitions observed: 6
- Type names referenced but not defined locally: 0
- Macros observed in related files: 20
- Global variables observed: 0

## Header Evidence
- `include/config.h` [include/config.h]
- `include/hard-locale.h` [include/hard-locale.h]
- `include/locale.h` [include/locale.h]
- `include/stdlib.h` [include/stdlib.h]
- `include/string.h` [include/string.h]

## Functions
### `hard_locale`
- Definition location: [hard-locale.c:27-47]
- Source file: `hard-locale.c`
- Observed declaration: `bool hard_locale (int category);`
- Approximate function body length: 21 lines

## Structs and Types
### `anonymous`
- Definition location: [include/locale.h:604-679]
- Source file: `include/locale.h`
- Observed declaration prefix: `struct lconv`
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

## Referenced External Types
- No external struct or type references beyond local definitions were recorded.

## Macros and Constants
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
- `FUNC_REALPATH_WORKS` [include/config.h:189]: `#define FUNC_REALPATH_WORKS 1`
- `GETGROUPS_T` [include/config.h:199]: `#define GETGROUPS_T gid_t`
- `GETTIMEOFDAY_TIMEZONE` [include/config.h:207]: `#define GETTIMEOFDAY_TIMEZONE void`
- `GNULIB_AREADLINKAT` [include/config.h:211]: `#define GNULIB_AREADLINKAT 1`

## Global Variables
- No global variable definitions were observed in the current module's `.c` files.

## Known Gaps
- This document is generated from parsed results for functions, structs, macros, and global variables; it does not infer declaration signatures from `.h` files that were not parsed.
- If a function appears in the "Functions" section without an explicit header binding, the later Rust migration should re-check the corresponding source `#include` relationships and build scripts.
- Error codes, configuration items, and input/output protocols are recorded only when explicit symbols appear in the source; missing entries do not mean the semantics do not exist, only that the current fact extraction did not observe them.
