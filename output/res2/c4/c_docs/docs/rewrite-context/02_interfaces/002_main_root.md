# Interface Facts: main_root

This document is intended for the later Rust repository-level rewrite and keeps only the interface facts directly observed in the current source-analysis stage.
Header files, macros, error codes, and configuration items that do not appear in the current parse results will not be added or assumed.

## Module Scope
- Module category: `main`
- Directory: `root`
- File list: c4.c, hello.c
- Candidate header files: none
- Exported functions observed: 10
- Struct definitions observed: 0
- Type names referenced but not defined locally: 0
- Macros observed in related files: 1
- Global variables observed: 0

## Header Evidence
- No project header files were associated from directory, include graph, or file-name evidence.

## Functions
### `next`
- Definition location: [c4.c:48-132]
- Source file: `c4.c`
- Observed declaration: `void next();`
- Approximate function body length: 85 lines
### `expr`
- Definition location: [c4.c:134-282]
- Source file: `c4.c`
- Observed declaration: `void expr(int lev);`
- Approximate function body length: 149 lines
### `stmt`
- Definition location: [c4.c:284-331]
- Source file: `c4.c`
- Observed declaration: `void stmt();`
- Approximate function body length: 48 lines
### `main`
- Definition location: [c4.c:333-528]
- Source file: `c4.c`
- Observed declaration: `int main(int argc, char **argv);`
- Approximate function body length: 196 lines
### `main`
- Definition location: [hello.c:3-7]
- Source file: `hello.c`
- Observed declaration: `int main();`
- Approximate function body length: 5 lines
### `next`
- Definition location: [test/c4.c:48-132]
- Source file: `test/c4.c`
- Observed declaration: `void next();`
- Approximate function body length: 85 lines
### `expr`
- Definition location: [test/c4.c:134-282]
- Source file: `test/c4.c`
- Observed declaration: `void expr(int lev);`
- Approximate function body length: 149 lines
### `stmt`
- Definition location: [test/c4.c:284-331]
- Source file: `test/c4.c`
- Observed declaration: `void stmt();`
- Approximate function body length: 48 lines
### `main`
- Definition location: [test/c4.c:333-528]
- Source file: `test/c4.c`
- Observed declaration: `int main(int argc, char **argv);`
- Approximate function body length: 196 lines
### `main`
- Definition location: [test/hello.c:3-7]
- Source file: `test/hello.c`
- Observed declaration: `int main();`
- Approximate function body length: 5 lines

## Structs and Types
- No struct definitions were observed in the current module slice.

## Referenced External Types
- No external struct or type references beyond local definitions were recorded.

## Macros and Constants
- `int` [c4.c:14]: `#define int long long`

## Global Variables
- No global variable definitions were observed in the current module's `.c` files.

## Known Gaps
- This document is generated from parsed results for functions, structs, macros, and global variables; it does not infer declaration signatures from `.h` files that were not parsed.
- If a function appears in the "Functions" section without an explicit header binding, the later Rust migration should re-check the corresponding source `#include` relationships and build scripts.
- Error codes, configuration items, and input/output protocols are recorded only when explicit symbols appear in the source; missing entries do not mean the semantics do not exist, only that the current fact extraction did not observe them.
