# Interface Facts: main_root_c-strcasecmp.c_15

This document is intended for the later Rust repository-level rewrite and keeps only the interface facts directly observed in the current source-analysis stage.
Header files, macros, error codes, and configuration items that do not appear in the current parse results will not be added or assumed.

## Module Scope
- Module category: `main_cluster`
- Directory: `root`
- File list: c-strcasecmp.c
- Candidate header files: include/c-ctype.h, include/c-strcase.h, include/config.h, include/limits.h
- Exported functions observed: 1
- Struct definitions observed: 0
- Type names referenced but not defined locally: 0
- Macros observed in related files: 20
- Global variables observed: 0

## Header Evidence
- `include/c-ctype.h` [include/c-ctype.h]
- `include/c-strcase.h` [include/c-strcase.h]
- `include/config.h` [include/config.h]
- `include/limits.h` [include/limits.h]

## Functions
### `c_strcasecmp`
- Definition location: [c-strcasecmp.c:26-56]
- Source file: `c-strcasecmp.c`
- Observed declaration: `int c_strcasecmp (const char *s1, const char *s2);`
- Approximate function body length: 31 lines

## Structs and Types
- No struct definitions were observed in the current module slice.

## Referenced External Types
- No external struct or type references beyond local definitions were recorded.

## Macros and Constants
- `C_CTYPE_H` [include/c-ctype.h:24]: `#define C_CTYPE_H`
- `_C_CTYPE_CNTRL` [include/c-ctype.h:87-90]: `#define _C_CTYPE_CNTRL \ case '\a': case '\b': case '\f': case '\n': \ case '\r': case '\t': case '\v': \ _C_CTYPE_OTHER_CNTRL`
- `_C_CTYPE_LOWER_A_THRU_F_N` [include/c-ctype.h:118-120]: `#define _C_CTYPE_LOWER_A_THRU_F_N(N) \ case 'a' + (N): case 'b' + (N): case 'c' + (N): case 'd' + (N): \ case 'e' + (N): case 'f' + (N)`
- `_C_CTYPE_LOWER_N` [include/c-ctype.h:121-127]: `#define _C_CTYPE_LOWER_N(N) \ _C_CTYPE_LOWER_A_THRU_F_N(N): \ case 'g' + (N): case 'h' + (N): case 'i' + (N): case 'j' + (N): \ case 'k' + (N): case 'l' + (N): case 'm' + (N): c...`
- `_C_CTYPE_A_THRU_F` [include/c-ctype.h:131-133]: `#define _C_CTYPE_A_THRU_F \ _C_CTYPE_LOWER_A_THRU_F_N (0): \ _C_CTYPE_LOWER_A_THRU_F_N ('A' - 'a')`
- `_C_CTYPE_DIGIT` [include/c-ctype.h:134-137]: `#define _C_CTYPE_DIGIT \ case '0': case '1': case '2': case '3': \ case '4': case '5': case '6': case '7': \ case '8': case '9'`
- `_C_CTYPE_LOWER` [include/c-ctype.h:138]: `#define _C_CTYPE_LOWER _C_CTYPE_LOWER_N (0)`
- `_C_CTYPE_PUNCT` [include/c-ctype.h:139-147]: `#define _C_CTYPE_PUNCT \ case '!': case '"': case '#': case '$': \ case '%': case '&': case '\'': case '(': \ case ')': case '*': case '+': case ',': \ case '-': case '.': case...`
- `_C_CTYPE_UPPER` [include/c-ctype.h:148]: `#define _C_CTYPE_UPPER _C_CTYPE_LOWER_N ('A' - 'a')`
- `C_STRCASE_H` [include/c-strcase.h:19]: `#define C_STRCASE_H`
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

## Global Variables
- No global variable definitions were observed in the current module's `.c` files.

## Known Gaps
- This document is generated from parsed results for functions, structs, macros, and global variables; it does not infer declaration signatures from `.h` files that were not parsed.
- If a function appears in the "Functions" section without an explicit header binding, the later Rust migration should re-check the corresponding source `#include` relationships and build scripts.
- Error codes, configuration items, and input/output protocols are recorded only when explicit symbols appear in the source; missing entries do not mean the semantics do not exist, only that the current fact extraction did not observe them.
