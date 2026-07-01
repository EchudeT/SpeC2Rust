# Interface Facts: module_src_output.c_27

This document is intended for the later Rust repository-level rewrite and keeps only the interface facts directly observed in the current source-analysis stage.
Header files, macros, error codes, and configuration items that do not appear in the current parse results will not be added or assumed.

## Module Scope
- Module category: `module_cluster`
- Directory: `src`
- File list: src/output.c
- Candidate header files: src/cflow.h, src/parser.h
- Exported functions observed: 15
- Struct definitions observed: 36
- Type names referenced but not defined locally: 0
- Macros observed in related files: 18
- Global variables observed: 0

## Header Evidence
- `src/cflow.h` [src/cflow.h]
- `src/parser.h` [src/parser.h]

## Functions
### `print_level`
- Definition location: [src/output.c:42-55]
- Source file: `src/output.c`
- Observed declaration: `void print_level(int lev, int last);`
- Approximate function body length: 14 lines
### `register_output`
- Definition location: [src/output.c:72-85]
- Source file: `src/output.c`
- Observed declaration: `int register_output(const char *name, int (*handler) (cflow_output_command cmd, FILE *outfile, int line, void *data, void *handler_data), void *handler_data);`
- Approximate function body length: 14 lines
### `select_output_driver`
- Definition location: [src/output.c:87-97]
- Source file: `src/output.c`
- Observed declaration: `int select_output_driver(const char *name);`
- Approximate function body length: 11 lines
### `output_init`
- Definition location: [src/output.c:99-106]
- Source file: `src/output.c`
- Observed declaration: `void output_init();`
- Approximate function body length: 8 lines
### `newline`
- Definition location: [src/output.c:108-116]
- Source file: `src/output.c`
- Observed declaration: `void newline();`
- Approximate function body length: 9 lines
### `begin`
- Definition location: [src/output.c:118-125]
- Source file: `src/output.c`
- Observed declaration: `static void begin();`
- Approximate function body length: 8 lines
### `end`
- Definition location: [src/output.c:127-134]
- Source file: `src/output.c`
- Observed declaration: `static void end();`
- Approximate function body length: 8 lines
### `separator`
- Definition location: [src/output.c:136-143]
- Source file: `src/output.c`
- Observed declaration: `static void separator();`
- Approximate function body length: 8 lines
### `print_text`
- Definition location: [src/output.c:146-153]
- Source file: `src/output.c`
- Observed declaration: `static void print_text(char *buf);`
- Approximate function body length: 8 lines
### `compare`
- Definition location: [src/output.c:173-179]
- Source file: `src/output.c`
- Observed declaration: `static int compare(const void *ap, const void *bp);`
- Approximate function body length: 7 lines
### `is_var`
- Definition location: [src/output.c:181-192]
- Source file: `src/output.c`
- Observed declaration: `static int is_var(Symbol *symp);`
- Approximate function body length: 12 lines
### `symbol_is_function`
- Definition location: [src/output.c:194-198]
- Source file: `src/output.c`
- Observed declaration: `int symbol_is_function(Symbol *symp);`
- Approximate function body length: 5 lines
### `clear_active`
- Definition location: [src/output.c:200-204]
- Source file: `src/output.c`
- Observed declaration: `static void clear_active(Symbol *sym);`
- Approximate function body length: 5 lines
### `print_type`
- Definition location: [src/output.c:236-244]
- Source file: `src/output.c`
- Observed declaration: `static void print_type(Symbol *symp);`
- Approximate function body length: 9 lines
### `xref_output`
- Definition location: [src/output.c:246-270]
- Source file: `src/output.c`
- Observed declaration: `void xref_output();`
- Approximate function body length: 25 lines

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
- Definition location: [src/output.c:60-66]
- Source file: `src/output.c`
- Observed declaration prefix: `struct output_driver`
### `anonymous`
- Definition location: [src/output.c:70]
- Source file: `src/output.c`
- Observed declaration prefix: `struct output_driver`
### `anonymous`
- Definition location: [src/output.c:159]
- Source file: `src/output.c`
- Observed declaration prefix: `struct output_symbol`
### `anonymous`
- Definition location: [src/output.c:209]
- Source file: `src/output.c`
- Observed declaration prefix: `struct linked_list`
### `anonymous`
- Definition location: [src/output.c:212]
- Source file: `src/output.c`
- Observed declaration prefix: `struct linked_list_entry`
### `anonymous`
- Definition location: [src/output.c:283]
- Source file: `src/output.c`
- Observed declaration prefix: `struct linked_list_entry`
### `anonymous`
- Definition location: [src/output.c:289]
- Source file: `src/output.c`
- Observed declaration prefix: `struct linked_list_entry`
### `anonymous`
- Definition location: [src/output.c:302]
- Source file: `src/output.c`
- Observed declaration prefix: `struct linked_list_entry`
### `anonymous`
- Definition location: [src/output.c:327]
- Source file: `src/output.c`
- Observed declaration prefix: `struct linked_list_entry`
### `anonymous`
- Definition location: [src/output.c:362]
- Source file: `src/output.c`
- Observed declaration prefix: `struct linked_list_entry`
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

## Global Variables
- No global variable definitions were observed in the current module's `.c` files.

## Known Gaps
- This document is generated from parsed results for functions, structs, macros, and global variables; it does not infer declaration signatures from `.h` files that were not parsed.
- If a function appears in the "Functions" section without an explicit header binding, the later Rust migration should re-check the corresponding source `#include` relationships and build scripts.
- Error codes, configuration items, and input/output protocols are recorded only when explicit symbols appear in the source; missing entries do not mean the semantics do not exist, only that the current fact extraction did not observe them.
