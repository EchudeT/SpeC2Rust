# Interface Facts: module_src_linked_list_05

This document is intended for the later Rust repository-level rewrite and keeps only the interface facts directly observed in the current source-analysis stage.
Header files, macros, error codes, and configuration items that do not appear in the current parse results will not be added or assumed.

## Module Scope
- Module category: `module_cluster`
- Directory: `src`
- File list: src/linked-list.c, src/symbol.c
- Candidate header files: gnu/hash.h, src/cflow.h, src/parser.h
- Exported functions observed: 3
- Struct definitions observed: 83
- Type names referenced but not defined locally: 1
- Macros observed in related files: 18
- Global variables observed: 0

## Header Evidence
- `gnu/hash.h` [gnu/hash.h]
- `src/cflow.h` [src/cflow.h]
- `src/parser.h` [src/parser.h]

## Functions
### `deref_linked_list`
- Definition location: [src/linked-list.c:19-29]
- Source file: `src/linked-list.c`
- Observed declaration: `static struct linked_list * deref_linked_list(struct linked_list **plist);`
- Approximate function body length: 11 lines
### `linked_list_create`
- Definition location: [src/linked-list.c:32-39]
- Source file: `src/linked-list.c`
- Observed declaration: `struct linked_list * linked_list_create(linked_list_free_data_fp fun);`
- Approximate function body length: 8 lines
### `append_symbol`
- Definition location: [src/symbol.c:39-47]
- Source file: `src/symbol.c`
- Observed declaration: `static void append_symbol(struct linked_list **plist, Symbol *sp);`
- Approximate function body length: 9 lines

## Structs and Types
### `anonymous`
- Definition location: [gnu/hash.h:42-52]
- Source file: `gnu/hash.h`
- Observed declaration prefix: `struct hash_tuning`
### `anonymous`
- Definition location: [gnu/hash.h:54]
- Source file: `gnu/hash.h`
- Observed declaration prefix: `struct hash_tuning`
### `anonymous`
- Definition location: [gnu/hash.h:56]
- Source file: `gnu/hash.h`
- Observed declaration prefix: `struct hash_table`
### `anonymous`
- Definition location: [gnu/hash.h:58]
- Source file: `gnu/hash.h`
- Observed declaration prefix: `struct hash_table`
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
- Definition location: [src/linked-list.c:19]
- Source file: `src/linked-list.c`
- Observed declaration prefix: `struct linked_list`
### `anonymous`
- Definition location: [src/linked-list.c:20]
- Source file: `src/linked-list.c`
- Observed declaration prefix: `struct linked_list`
### `anonymous`
- Definition location: [src/linked-list.c:23]
- Source file: `src/linked-list.c`
- Observed declaration prefix: `struct linked_list`
### `anonymous`
- Definition location: [src/linked-list.c:32]
- Source file: `src/linked-list.c`
- Observed declaration prefix: `struct linked_list`
### `anonymous`
- Definition location: [src/linked-list.c:35]
- Source file: `src/linked-list.c`
- Observed declaration prefix: `struct linked_list`
### `anonymous`
- Definition location: [src/linked-list.c:42]
- Source file: `src/linked-list.c`
- Observed declaration prefix: `struct linked_list`
### `anonymous`
- Definition location: [src/linked-list.c:44]
- Source file: `src/linked-list.c`
- Observed declaration prefix: `struct linked_list`
### `anonymous`
- Definition location: [src/linked-list.c:45]
- Source file: `src/linked-list.c`
- Observed declaration prefix: `struct linked_list_entry`
### `anonymous`
- Definition location: [src/linked-list.c:60]
- Source file: `src/linked-list.c`
- Observed declaration prefix: `struct linked_list`
### `anonymous`
- Definition location: [src/linked-list.c:62]
- Source file: `src/linked-list.c`
- Observed declaration prefix: `struct linked_list`
### `anonymous`
- Definition location: [src/linked-list.c:63]
- Source file: `src/linked-list.c`
- Observed declaration prefix: `struct linked_list_entry`
### `anonymous`
- Definition location: [src/linked-list.c:78]
- Source file: `src/linked-list.c`
- Observed declaration prefix: `struct linked_list`
### `anonymous`
- Definition location: [src/linked-list.c:81]
- Source file: `src/linked-list.c`
- Observed declaration prefix: `struct linked_list`
### `anonymous`
- Definition location: [src/linked-list.c:82]
- Source file: `src/linked-list.c`
- Observed declaration prefix: `struct linked_list_entry`
### `anonymous`
- Definition location: [src/linked-list.c:85]
- Source file: `src/linked-list.c`
- Observed declaration prefix: `struct linked_list_entry`
### `anonymous`
- Definition location: [src/linked-list.c:97]
- Source file: `src/linked-list.c`
- Observed declaration prefix: `struct linked_list`
### `anonymous`
- Definition location: [src/linked-list.c:97]
- Source file: `src/linked-list.c`
- Observed declaration prefix: `struct linked_list_entry`
### `anonymous`
- Definition location: [src/linked-list.c:99]
- Source file: `src/linked-list.c`
- Observed declaration prefix: `struct linked_list_entry`
### `anonymous`
- Definition location: [src/linked-list.c:116]
- Source file: `src/linked-list.c`
- Observed declaration prefix: `struct linked_list`
### `anonymous`
- Definition location: [src/linked-list.c:119]
- Source file: `src/linked-list.c`
- Observed declaration prefix: `struct linked_list`
### `anonymous`
- Definition location: [src/linked-list.c:120]
- Source file: `src/linked-list.c`
- Observed declaration prefix: `struct linked_list_entry`
### `anonymous`
- Definition location: [src/linked-list.c:126]
- Source file: `src/linked-list.c`
- Observed declaration prefix: `struct linked_list_entry`
### `anonymous`
- Definition location: [src/linked-list.c:138]
- Source file: `src/linked-list.c`
- Observed declaration prefix: `struct linked_list`
### `anonymous`
- Definition location: [src/linked-list.c:140]
- Source file: `src/linked-list.c`
- Observed declaration prefix: `struct linked_list_entry`
### `anonymous`
- Definition location: [src/linked-list.c:149]
- Source file: `src/linked-list.c`
- Observed declaration prefix: `struct linked_list`
### `anonymous`
- Definition location: [src/linked-list.c:153]
- Source file: `src/linked-list.c`
- Observed declaration prefix: `struct linked_list_entry`
### `anonymous`
- Definition location: [src/parser.h:41-43]
- Source file: `src/parser.h`
- Observed declaration prefix: `struct`
### `anonymous`
- Definition location: [src/symbol.c:23]
- Source file: `src/symbol.c`
- Observed declaration prefix: `struct linked_list`
### `anonymous`
- Definition location: [src/symbol.c:24]
- Source file: `src/symbol.c`
- Observed declaration prefix: `struct linked_list`
### `anonymous`
- Definition location: [src/symbol.c:25]
- Source file: `src/symbol.c`
- Observed declaration prefix: `struct linked_list`
### `anonymous`
- Definition location: [src/symbol.c:26]
- Source file: `src/symbol.c`
- Observed declaration prefix: `struct linked_list`
### `anonymous`
- Definition location: [src/symbol.c:27]
- Source file: `src/symbol.c`
- Observed declaration prefix: `struct linked_list`
### `anonymous`
- Definition location: [src/symbol.c:28]
- Source file: `src/symbol.c`
- Observed declaration prefix: `struct linked_list`
### `anonymous`
- Definition location: [src/symbol.c:40]
- Source file: `src/symbol.c`
- Observed declaration prefix: `struct linked_list`
### `anonymous`
- Definition location: [src/symbol.c:49-51]
- Source file: `src/symbol.c`
- Observed declaration prefix: `struct table_entry`
### `anonymous`
- Definition location: [src/symbol.c:57]
- Source file: `src/symbol.c`
- Observed declaration prefix: `struct table_entry`
### `anonymous`
- Definition location: [src/symbol.c:67]
- Source file: `src/symbol.c`
- Observed declaration prefix: `struct table_entry`
### `anonymous`
- Definition location: [src/symbol.c:68]
- Source file: `src/symbol.c`
- Observed declaration prefix: `struct table_entry`
### `anonymous`
- Definition location: [src/symbol.c:76]
- Source file: `src/symbol.c`
- Observed declaration prefix: `struct table_entry`
### `anonymous`
- Definition location: [src/symbol.c:98]
- Source file: `src/symbol.c`
- Observed declaration prefix: `struct table_entry`
### `anonymous`
- Definition location: [src/symbol.c:194]
- Source file: `src/symbol.c`
- Observed declaration prefix: `struct table_entry`
### `anonymous`
- Definition location: [src/symbol.c:239]
- Source file: `src/symbol.c`
- Observed declaration prefix: `struct table_entry`
### `anonymous`
- Definition location: [src/symbol.c:305-310]
- Source file: `src/symbol.c`
- Observed declaration prefix: `struct collect_data`
### `anonymous`
- Definition location: [src/symbol.c:315]
- Source file: `src/symbol.c`
- Observed declaration prefix: `struct table_entry`
### `anonymous`
- Definition location: [src/symbol.c:316]
- Source file: `src/symbol.c`
- Observed declaration prefix: `struct collect_data`
### `anonymous`
- Definition location: [src/symbol.c:333]
- Source file: `src/symbol.c`
- Observed declaration prefix: `struct collect_data`
### `anonymous`
- Definition location: [src/symbol.c:347]
- Source file: `src/symbol.c`
- Observed declaration prefix: `struct collect_data`
### `anonymous`
- Definition location: [src/symbol.c:368]
- Source file: `src/symbol.c`
- Observed declaration prefix: `struct linked_list_entry`
### `anonymous`
- Definition location: [src/symbol.c:393]
- Source file: `src/symbol.c`
- Observed declaration prefix: `struct table_entry`
### `anonymous`
- Definition location: [src/symbol.c:417]
- Source file: `src/symbol.c`
- Observed declaration prefix: `struct linked_list_entry`
### `anonymous`
- Definition location: [src/symbol.c:456]
- Source file: `src/symbol.c`
- Observed declaration prefix: `struct linked_list_entry`
### `anonymous`
- Definition location: [src/symbol.c:472]
- Source file: `src/symbol.c`
- Observed declaration prefix: `struct linked_list_entry`
### `anonymous`
- Definition location: [src/symbol.c:502]
- Source file: `src/symbol.c`
- Observed declaration prefix: `struct linked_list_entry`
### `anonymous`
- Definition location: [src/symbol.c:518]
- Source file: `src/symbol.c`
- Observed declaration prefix: `struct linked_list_entry`

## Referenced External Types
- `linked_list`: this name came from clustering metadata or nearby call analysis, but no local definition was observed in the current module files.

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

## Global Variables
- No global variable definitions were observed in the current module's `.c` files.

## Known Gaps
- This document is generated from parsed results for functions, structs, macros, and global variables; it does not infer declaration signatures from `.h` files that were not parsed.
- If a function appears in the "Functions" section without an explicit header binding, the later Rust migration should re-check the corresponding source `#include` relationships and build scripts.
- Error codes, configuration items, and input/output protocols are recorded only when explicit symbols appear in the source; missing entries do not mean the semantics do not exist, only that the current fact extraction did not observe them.
