# Interface Facts: module_src

This document is intended for the later Rust repository-level rewrite and keeps only the interface facts directly observed in the current source-analysis stage.
Header files, macros, error codes, and configuration items that do not appear in the current parse results will not be added or assumed.

## Module Scope
- Module category: `module`
- Directory: `src`
- File list: src/shc.c
- Candidate header files: none
- Exported functions observed: 18
- Struct definitions observed: 5
- Type names referenced but not defined locally: 0
- Macros observed in related files: 1
- Global variables observed: 0

## Header Evidence
- No project header files were associated from directory, include graph, or file-name evidence.

## Functions
### `parse_an_arg`
- Definition location: [src/shc.c:759-878]
- Source file: `src/shc.c`
- Observed declaration: `static int parse_an_arg(int argc, char * argv[]);`
- Approximate function body length: 120 lines
### `parse_args`
- Definition location: [src/shc.c:880-903]
- Source file: `src/shc.c`
- Observed declaration: `static void parse_args(int argc, char * argv[]);`
- Approximate function body length: 24 lines
### `stte_0`
- Definition location: [src/shc.c:912-918]
- Source file: `src/shc.c`
- Observed declaration: `void stte_0(void);`
- Approximate function body length: 7 lines
### `key`
- Definition location: [src/shc.c:923-937]
- Source file: `src/shc.c`
- Observed declaration: `void key(void * str, int len);`
- Approximate function body length: 15 lines
### `arc4`
- Definition location: [src/shc.c:942-956]
- Source file: `src/shc.c`
- Observed declaration: `void arc4(void * str, int len);`
- Approximate function body length: 15 lines
### `key_with_file`
- Definition location: [src/shc.c:963-983]
- Source file: `src/shc.c`
- Observed declaration: `int key_with_file(char * file);`
- Approximate function body length: 21 lines
### `eval_shell`
- Definition location: [src/shc.c:1011-1075]
- Source file: `src/shc.c`
- Observed declaration: `int eval_shell(char * text);`
- Approximate function body length: 65 lines
### `read_script`
- Definition location: [src/shc.c:1077-1114]
- Source file: `src/shc.c`
- Observed declaration: `char * read_script(char * file);`
- Approximate function body length: 38 lines
### `rand_mod`
- Definition location: [src/shc.c:1116-1126]
- Source file: `src/shc.c`
- Observed declaration: `unsigned rand_mod(unsigned mod);`
- Approximate function body length: 11 lines
### `rand_chr`
- Definition location: [src/shc.c:1128-1131]
- Source file: `src/shc.c`
- Observed declaration: `char rand_chr(void);`
- Approximate function body length: 4 lines
### `noise`
- Definition location: [src/shc.c:1133-1143]
- Source file: `src/shc.c`
- Observed declaration: `int noise(char * ptr, unsigned min, unsigned xtra, int str);`
- Approximate function body length: 11 lines
### `prnt_bytes`
- Definition location: [src/shc.c:1147-1163]
- Source file: `src/shc.c`
- Observed declaration: `void prnt_bytes(FILE * o, char * ptr, int m, int l, int n);`
- Approximate function body length: 17 lines
### `prnt_array`
- Definition location: [src/shc.c:1165-1176]
- Source file: `src/shc.c`
- Observed declaration: `void prnt_array(FILE * o, void * ptr, char * name, int l, char * cast);`
- Approximate function body length: 12 lines
### `dump_array`
- Definition location: [src/shc.c:1178-1182]
- Source file: `src/shc.c`
- Observed declaration: `void dump_array(FILE * o, void * ptr, char * name, int l, char * cast);`
- Approximate function body length: 5 lines
### `write_C`
- Definition location: [src/shc.c:1184-1298]
- Source file: `src/shc.c`
- Observed declaration: `int write_C(char * file, char * argv[]);`
- Approximate function body length: 115 lines
### `make`
- Definition location: [src/shc.c:1300-1338]
- Source file: `src/shc.c`
- Observed declaration: `int make(void);`
- Approximate function body length: 39 lines
### `do_all`
- Definition location: [src/shc.c:1340-1353]
- Source file: `src/shc.c`
- Observed declaration: `void do_all(int argc, char * argv[]);`
- Approximate function body length: 14 lines
### `main`
- Definition location: [src/shc.c:1355-1363]
- Source file: `src/shc.c`
- Observed declaration: `int main(int argc, char * argv[]);`
- Approximate function body length: 9 lines

## Structs and Types
### `anonymous`
- Definition location: [src/shc.c:23]
- Source file: `src/shc.c`
- Observed declaration prefix: `struct`
### `anonymous`
- Definition location: [src/shc.c:763]
- Source file: `src/shc.c`
- Observed declaration prefix: `struct tm`
### `anonymous`
- Definition location: [src/shc.c:965]
- Source file: `src/shc.c`
- Observed declaration prefix: `struct stat`
### `anonymous`
- Definition location: [src/shc.c:966]
- Source file: `src/shc.c`
- Observed declaration prefix: `struct stat`
### `anonymous`
- Definition location: [src/shc.c:989-994]
- Source file: `src/shc.c`
- Observed declaration prefix: `struct`

## Referenced External Types
- No external struct or type references beyond local definitions were recorded.

## Macros and Constants
- `SIZE` [src/shc.c:120]: `#define SIZE 4096`

## Global Variables
- No global variable definitions were observed in the current module's `.c` files.

## Known Gaps
- This document is generated from parsed results for functions, structs, macros, and global variables; it does not infer declaration signatures from `.h` files that were not parsed.
- If a function appears in the "Functions" section without an explicit header binding, the later Rust migration should re-check the corresponding source `#include` relationships and build scripts.
- Error codes, configuration items, and input/output protocols are recorded only when explicit symbols appear in the source; missing entries do not mean the semantics do not exist, only that the current fact extraction did not observe them.
