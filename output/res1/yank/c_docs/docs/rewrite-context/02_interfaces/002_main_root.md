# Interface Facts: main_root

This document is intended for the later Rust repository-level rewrite and keeps only the interface facts directly observed in the current source-analysis stage.
Header files, macros, error codes, and configuration items that do not appear in the current parse results will not be added or assumed.

## Module Scope
- Module category: `main`
- Directory: `root`
- File list: yank.c
- Candidate header files: none
- Exported functions observed: 13
- Struct definitions observed: 15
- Type names referenced but not defined locally: 0
- Macros observed in related files: 11
- Global variables observed: 0

## Header Evidence
- No project header files were associated from directory, include graph, or file-name evidence.

## Functions
### `input`
- Definition location: [yank.c:69-90]
- Source file: `yank.c`
- Observed declaration: `static void input(void);`
- Approximate function body length: 22 lines
### `strtopat`
- Definition location: [yank.c:96-112]
- Source file: `yank.c`
- Observed declaration: `static char * strtopat(const char *s);`
- Approximate function body length: 17 lines
### `fcmp`
- Definition location: [yank.c:119-128]
- Source file: `yank.c`
- Observed declaration: `static int fcmp(const struct field *f1, const struct field *f2);`
- Approximate function body length: 10 lines
### `xwrite`
- Definition location: [yank.c:130-147]
- Source file: `yank.c`
- Observed declaration: `static ssize_t xwrite(int fd, const char *s, size_t nmemb);`
- Approximate function body length: 18 lines
### `yank`
- Definition location: [yank.c:149-189]
- Source file: `yank.c`
- Observed declaration: `static void yank(const char *s, size_t nmemb);`
- Approximate function body length: 41 lines
### `twrite`
- Definition location: [yank.c:191-196]
- Source file: `yank.c`
- Observed declaration: `static void twrite(const char *s, size_t nmemb);`
- Approximate function body length: 6 lines
### `tputs`
- Definition location: [yank.c:198-205]
- Source file: `yank.c`
- Observed declaration: `static void tputs(const char *s);`
- Approximate function body length: 8 lines
### `tsetup`
- Definition location: [yank.c:207-281]
- Source file: `yank.c`
- Observed declaration: `static void tsetup(void);`
- Approximate function body length: 75 lines
### `tend`
- Definition location: [yank.c:283-294]
- Source file: `yank.c`
- Observed declaration: `static void tend(void);`
- Approximate function body length: 12 lines
### `tgetc`
- Definition location: [yank.c:296-339]
- Source file: `yank.c`
- Observed declaration: `static int tgetc(void);`
- Approximate function body length: 44 lines
### `tmain`
- Definition location: [yank.c:341-408]
- Source file: `yank.c`
- Observed declaration: `static const struct field * tmain(void);`
- Approximate function body length: 68 lines
### `usage`
- Definition location: [yank.c:410-416]
- Source file: `yank.c`
- Observed declaration: `static void usage(void);`
- Approximate function body length: 7 lines
### `main`
- Definition location: [yank.c:418-492]
- Source file: `yank.c`
- Observed declaration: `int main(int argc, char *argv[]);`
- Approximate function body length: 75 lines

## Structs and Types
### `anonymous`
- Definition location: [yank.c:40-44]
- Source file: `yank.c`
- Observed declaration prefix: `struct field`
### `anonymous`
- Definition location: [yank.c:50-54]
- Source file: `yank.c`
- Observed declaration prefix: `struct`
### `anonymous`
- Definition location: [yank.c:53]
- Source file: `yank.c`
- Observed declaration prefix: `struct field`
### `anonymous`
- Definition location: [yank.c:56-60]
- Source file: `yank.c`
- Observed declaration prefix: `struct`
### `anonymous`
- Definition location: [yank.c:62-67]
- Source file: `yank.c`
- Observed declaration prefix: `struct`
### `anonymous`
- Definition location: [yank.c:66]
- Source file: `yank.c`
- Observed declaration prefix: `struct termios`
### `anonymous`
- Definition location: [yank.c:120]
- Source file: `yank.c`
- Observed declaration prefix: `struct field`
### `anonymous`
- Definition location: [yank.c:120]
- Source file: `yank.c`
- Observed declaration prefix: `struct field`
### `anonymous`
- Definition location: [yank.c:210]
- Source file: `yank.c`
- Observed declaration prefix: `struct termios`
### `anonymous`
- Definition location: [yank.c:211]
- Source file: `yank.c`
- Observed declaration prefix: `struct winsize`
### `anonymous`
- Definition location: [yank.c:226]
- Source file: `yank.c`
- Observed declaration prefix: `struct field`
### `anonymous`
- Definition location: [yank.c:240]
- Source file: `yank.c`
- Observed declaration prefix: `struct field`
### `anonymous`
- Definition location: [yank.c:299-302]
- Source file: `yank.c`
- Observed declaration prefix: `struct`
### `anonymous`
- Definition location: [yank.c:341]
- Source file: `yank.c`
- Observed declaration prefix: `struct field`
### `anonymous`
- Definition location: [yank.c:421]
- Source file: `yank.c`
- Observed declaration prefix: `struct field`

## Referenced External Types
- No external struct or type references beyond local definitions were recorded.

## Macros and Constants
- `T_CLR_EOS` [yank.c:16]: `#define T_CLR_EOS "\033[J"`
- `T_CURSOR_INVISIBLE` [yank.c:17]: `#define T_CURSOR_INVISIBLE "\033[?25l"`
- `T_CURSOR_VISIBLE` [yank.c:18]: `#define T_CURSOR_VISIBLE "\033[?25h"`
- `T_ENTER_CA_MODE` [yank.c:19]: `#define T_ENTER_CA_MODE "\033[?1049h"`
- `T_ENTER_STANDOUT_MODE` [yank.c:20]: `#define T_ENTER_STANDOUT_MODE "\033[7m"`
- `T_EXIT_CA_MODE` [yank.c:21]: `#define T_EXIT_CA_MODE "\033[?1049l"`
- `T_EXIT_STANDOUT_MODE` [yank.c:22]: `#define T_EXIT_STANDOUT_MODE "\033[0m"`
- `T_RESTORE_CURSOR` [yank.c:23]: `#define T_RESTORE_CURSOR "\0338"`
- `T_SAVE_CURSOR` [yank.c:24]: `#define T_SAVE_CURSOR "\0337"`
- `MAX` [yank.c:26]: `#define MAX(x, y) ((x) > (y) ? (x) : (y))`
- `MIN` [yank.c:27]: `#define MIN(x, y) ((x) < (y) ? (x) : (y))`

## Global Variables
- No global variable definitions were observed in the current module's `.c` files.

## Known Gaps
- This document is generated from parsed results for functions, structs, macros, and global variables; it does not infer declaration signatures from `.h` files that were not parsed.
- If a function appears in the "Functions" section without an explicit header binding, the later Rust migration should re-check the corresponding source `#include` relationships and build scripts.
- Error codes, configuration items, and input/output protocols are recorded only when explicit symbols appear in the source; missing entries do not mean the semantics do not exist, only that the current fact extraction did not observe them.
