# Interface Facts: module_tilde

This document is intended for the later Rust repository-level rewrite and keeps only the interface facts directly observed in the current source-analysis stage.
Header files, macros, error codes, and configuration items that do not appear in the current parse results will not be added or assumed.

## Module Scope
- Module category: `module`
- Directory: `tilde`
- File list: tilde/shell.c, tilde/tilde.c
- Candidate header files: tilde/ansi_stdlib.h, tilde/bashansi.h, tilde/tilde.h, tilde/xmalloc.h
- Exported functions observed: 4
- Struct definitions observed: 5
- Type names referenced but not defined locally: 0
- Macros observed in related files: 4
- Global variables observed: 0

## Header Evidence
- `tilde/ansi_stdlib.h` [tilde/ansi_stdlib.h]
- `tilde/bashansi.h` [tilde/bashansi.h]
- `tilde/tilde.h` [tilde/tilde.h]
- `tilde/xmalloc.h` [tilde/xmalloc.h]

## Functions
### `get_home_dir`
- Definition location: [tilde/shell.c:59-70]
- Source file: `tilde/shell.c`
- Observed declaration: `char * get_home_dir ();`
- Approximate function body length: 12 lines
### `tilde_find_prefix`
- Definition location: [tilde/tilde.c:127-158]
- Source file: `tilde/tilde.c`
- Observed declaration: `static int tilde_find_prefix (string, len) const char *string; int *len;`
- Approximate function body length: 32 lines
### `tilde_find_suffix`
- Definition location: [tilde/tilde.c:162-188]
- Source file: `tilde/tilde.c`
- Observed declaration: `static int tilde_find_suffix (string) const char *string;`
- Approximate function body length: 27 lines
### `memory_error_and_abort`
- Definition location: [tilde/tilde.c:490-495]
- Source file: `tilde/tilde.c`
- Observed declaration: `static void memory_error_and_abort ();`
- Approximate function body length: 6 lines

## Structs and Types
### `anonymous`
- Definition location: [tilde/shell.c:49]
- Source file: `tilde/shell.c`
- Observed declaration prefix: `struct passwd`
### `anonymous`
- Definition location: [tilde/shell.c:63]
- Source file: `tilde/shell.c`
- Observed declaration prefix: `struct passwd`
### `anonymous`
- Definition location: [tilde/tilde.c:60]
- Source file: `tilde/tilde.c`
- Observed declaration prefix: `struct passwd`
### `anonymous`
- Definition location: [tilde/tilde.c:63]
- Source file: `tilde/tilde.c`
- Observed declaration prefix: `struct passwd`
### `anonymous`
- Definition location: [tilde/tilde.c:348]
- Source file: `tilde/tilde.c`
- Observed declaration prefix: `struct passwd`

## Referenced External Types
- No external struct or type references beyond local definitions were recorded.

## Macros and Constants
- `_STDLIB_H_` [tilde/ansi_stdlib.h:24]: `#define _STDLIB_H_ 1`
- `_BASHANSI_H_` [tilde/bashansi.h:22]: `#define _BASHANSI_H_`
- `savestring` [tilde/tilde.c:68]: `#define savestring(x) strcpy ((char *)xmalloc (1 + strlen (x)), (x))`
- `_XMALLOC_H_` [tilde/xmalloc.h:24]: `#define _XMALLOC_H_`

## Global Variables
- No global variable definitions were observed in the current module's `.c` files.

## Known Gaps
- This document is generated from parsed results for functions, structs, macros, and global variables; it does not infer declaration signatures from `.h` files that were not parsed.
- If a function appears in the "Functions" section without an explicit header binding, the later Rust migration should re-check the corresponding source `#include` relationships and build scripts.
- Error codes, configuration items, and input/output protocols are recorded only when explicit symbols appear in the source; missing entries do not mean the semantics do not exist, only that the current fact extraction did not observe them.
