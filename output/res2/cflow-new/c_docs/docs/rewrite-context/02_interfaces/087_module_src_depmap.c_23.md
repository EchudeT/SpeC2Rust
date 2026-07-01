# Interface Facts: module_src_depmap.c_23

This document is intended for the later Rust repository-level rewrite and keeps only the interface facts directly observed in the current source-analysis stage.
Header files, macros, error codes, and configuration items that do not appear in the current parse results will not be added or assumed.

## Module Scope
- Module category: `module_cluster`
- Directory: `src`
- File list: src/depmap.c
- Candidate header files: src/cflow.h, src/parser.h
- Exported functions observed: 6
- Struct definitions observed: 27
- Type names referenced but not defined locally: 0
- Macros observed in related files: 20
- Global variables observed: 0

## Header Evidence
- `src/cflow.h` [src/cflow.h]
- `src/parser.h` [src/parser.h]

## Functions
### `transitive_closure`
- Definition location: [src/depmap.c:29-74]
- Source file: `src/depmap.c`
- Observed declaration: `static void transitive_closure(unsigned *R, int n);`
- Approximate function body length: 46 lines
### `depmap_alloc`
- Definition location: [src/depmap.c:82-91]
- Source file: `src/depmap.c`
- Observed declaration: `cflow_depmap_t depmap_alloc(size_t count);`
- Approximate function body length: 10 lines
### `depmap_rowptr`
- Definition location: [src/depmap.c:93-97]
- Source file: `src/depmap.c`
- Observed declaration: `static unsigned * depmap_rowptr(cflow_depmap_t dmap, size_t row);`
- Approximate function body length: 5 lines
### `depmap_set`
- Definition location: [src/depmap.c:99-104]
- Source file: `src/depmap.c`
- Observed declaration: `void depmap_set(cflow_depmap_t dmap, size_t row, size_t col);`
- Approximate function body length: 6 lines
### `depmap_isset`
- Definition location: [src/depmap.c:106-111]
- Source file: `src/depmap.c`
- Observed declaration: `int depmap_isset(cflow_depmap_t dmap, size_t row, size_t col);`
- Approximate function body length: 6 lines
### `depmap_tc`
- Definition location: [src/depmap.c:113-117]
- Source file: `src/depmap.c`
- Observed declaration: `void depmap_tc(cflow_depmap_t dmap);`
- Approximate function body length: 5 lines

## Structs and Types
### `anonymous`
- Definition location: [src/cflow.h:48-52]
- Source file: `src/cflow.h`
- Observed declaration prefix: `struct linked_list_entry`
### `anonymous`
- Definition location: [src/cflow.h:49]
- Source file: `src/cflow.h`
- Observed declaration prefix: `struct linked_list_entry`
### `anonymous`
- Definition location: [src/cflow.h:50]
- Source file: `src/cflow.h`
- Observed declaration prefix: `struct linked_list`
### `anonymous`
- Definition location: [src/cflow.h:56-59]
- Source file: `src/cflow.h`
- Observed declaration prefix: `struct linked_list`
### `anonymous`
- Definition location: [src/cflow.h:58]
- Source file: `src/cflow.h`
- Observed declaration prefix: `struct linked_list_entry`
### `anonymous`
- Definition location: [src/cflow.h:77-80]
- Source file: `src/cflow.h`
- Observed declaration prefix: `struct`
### `anonymous`
- Definition location: [src/cflow.h:92]
- Source file: `src/cflow.h`
- Observed declaration prefix: `struct symbol`
### `anonymous`
- Definition location: [src/cflow.h:94-134]
- Source file: `src/cflow.h`
- Observed declaration prefix: `struct symbol`
### `anonymous`
- Definition location: [src/cflow.h:95]
- Source file: `src/cflow.h`
- Observed declaration prefix: `struct table_entry`
### `anonymous`
- Definition location: [src/cflow.h:97]
- Source file: `src/cflow.h`
- Observed declaration prefix: `struct linked_list_entry`
### `anonymous`
- Definition location: [src/cflow.h:102]
- Source file: `src/cflow.h`
- Observed declaration prefix: `struct symbol`
### `anonymous`
- Definition location: [src/cflow.h:119]
- Source file: `src/cflow.h`
- Observed declaration prefix: `struct linked_list`
### `anonymous`
- Definition location: [src/cflow.h:132]
- Source file: `src/cflow.h`
- Observed declaration prefix: `struct linked_list`
### `anonymous`
- Definition location: [src/cflow.h:133]
- Source file: `src/cflow.h`
- Observed declaration prefix: `struct linked_list`
### `anonymous`
- Definition location: [src/cflow.h:208]
- Source file: `src/cflow.h`
- Observed declaration prefix: `struct linked_list`
### `anonymous`
- Definition location: [src/cflow.h:209]
- Source file: `src/cflow.h`
- Observed declaration prefix: `struct linked_list`
### `anonymous`
- Definition location: [src/cflow.h:210]
- Source file: `src/cflow.h`
- Observed declaration prefix: `struct linked_list`
### `anonymous`
- Definition location: [src/cflow.h:211]
- Source file: `src/cflow.h`
- Observed declaration prefix: `struct linked_list`
### `anonymous`
- Definition location: [src/cflow.h:212]
- Source file: `src/cflow.h`
- Observed declaration prefix: `struct linked_list`
### `anonymous`
- Definition location: [src/cflow.h:214]
- Source file: `src/cflow.h`
- Observed declaration prefix: `struct linked_list`
### `anonymous`
- Definition location: [src/cflow.h:215]
- Source file: `src/cflow.h`
- Observed declaration prefix: `struct linked_list_entry`
### `anonymous`
- Definition location: [src/cflow.h:216]
- Source file: `src/cflow.h`
- Observed declaration prefix: `struct linked_list`
### `anonymous`
- Definition location: [src/cflow.h:218]
- Source file: `src/cflow.h`
- Observed declaration prefix: `struct linked_list`
### `anonymous`
- Definition location: [src/cflow.h:249-254]
- Source file: `src/cflow.h`
- Observed declaration prefix: `struct output_symbol`
### `anonymous`
- Definition location: [src/cflow.h:275]
- Source file: `src/cflow.h`
- Observed declaration prefix: `struct cflow_depmap`
### `anonymous`
- Definition location: [src/depmap.c:76-80]
- Source file: `src/depmap.c`
- Observed declaration prefix: `struct cflow_depmap`
### `anonymous`
- Definition location: [src/parser.h:41-43]
- Source file: `src/parser.h`
- Observed declaration prefix: `struct`

## Referenced External Types
- No external struct or type references beyond local definitions were recorded.

## Macros and Constants
- `obstack_chunk_alloc` [src/cflow.h:27]: `#define obstack_chunk_alloc xmalloc`
- `obstack_chunk_free` [src/cflow.h:28]: `#define obstack_chunk_free free`
- `_` [src/cflow.h:34]: `#define _(c) gettext(c)`
- `N_` [src/cflow.h:35]: `#define N_(c) c`
- `EX_OK` [src/cflow.h:40]: `#define EX_OK 0 /* Success */`
- `EX_FATAL` [src/cflow.h:41]: `#define EX_FATAL 1 /* Fatal error */`
- `EX_SOFT` [src/cflow.h:42]: `#define EX_SOFT 2 /* Some input files cannot be read or parsed */`
- `EX_USAGE` [src/cflow.h:43]: `#define EX_USAGE 3 /* Command line usage error */`
- `EX_SOFTWARE` [src/cflow.h:44]: `#define EX_SOFTWARE 4`
- `NUMITEMS` [src/cflow.h:46]: `#define NUMITEMS(a) sizeof(a)/sizeof((a)[0])`
- `linked_list_head` [src/cflow.h:61]: `#define linked_list_head(list) ((list) ? (list)->head : NULL)`
- `PRINT_XREF` [src/cflow.h:137]: `#define PRINT_XREF 0x01`
- `PRINT_TREE` [src/cflow.h:138]: `#define PRINT_TREE 0x02`
- `MAX_OUTPUT_DRIVERS` [src/cflow.h:144]: `#define MAX_OUTPUT_DRIVERS 8`
- `INSTALL_DEFAULT` [src/cflow.h:182]: `#define INSTALL_DEFAULT 0x00`
- `INSTALL_OVERWRITE` [src/cflow.h:183]: `#define INSTALL_OVERWRITE 0x01`
- `INSTALL_CHECK_LOCAL` [src/cflow.h:184]: `#define INSTALL_CHECK_LOCAL 0x02`
- `INSTALL_UNIT_LOCAL` [src/cflow.h:185]: `#define INSTALL_UNIT_LOCAL 0x04`
- `BITS_PER_WORD` [src/depmap.c:22]: `#define BITS_PER_WORD (sizeof(unsigned)*CHAR_BIT)`
- `WORDSIZE` [src/depmap.c:24]: `#define WORDSIZE(n) (((n) + BITS_PER_WORD - 1) / BITS_PER_WORD)`

## Global Variables
- No global variable definitions were observed in the current module's `.c` files.

## Known Gaps
- This document is generated from parsed results for functions, structs, macros, and global variables; it does not infer declaration signatures from `.h` files that were not parsed.
- If a function appears in the "Functions" section without an explicit header binding, the later Rust migration should re-check the corresponding source `#include` relationships and build scripts.
- Error codes, configuration items, and input/output protocols are recorded only when explicit symbols appear in the source; missing entries do not mean the semantics do not exist, only that the current fact extraction did not observe them.
