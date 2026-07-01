# Interface Facts: module_src_parser.c_31

This document is intended for the later Rust repository-level rewrite and keeps only the interface facts directly observed in the current source-analysis stage.
Header files, macros, error codes, and configuration items that do not appear in the current parse results will not be added or assumed.

## Module Scope
- Module category: `module_cluster`
- Directory: `src`
- File list: src/parser.c
- Candidate header files: src/cflow.h, src/parser.h
- Exported functions observed: 15
- Struct definitions observed: 37
- Type names referenced but not defined locally: 0
- Macros observed in related files: 18
- Global variables observed: 0

## Header Evidence
- `src/cflow.h` [src/cflow.h]
- `src/parser.h` [src/parser.h]

## Functions
### `skip_struct`
- Definition location: [src/parser.c:953-974]
- Source file: `src/parser.c`
- Observed declaration: `void skip_struct();`
- Approximate function body length: 22 lines
### `parse_typedef`
- Definition location: [src/parser.c:976-994]
- Source file: `src/parser.c`
- Observed declaration: `void parse_typedef();`
- Approximate function body length: 19 lines
### `parse_dcl`
- Definition location: [src/parser.c:996-1008]
- Source file: `src/parser.c`
- Observed declaration: `void parse_dcl(Ident *ident, int maybe_knr);`
- Approximate function body length: 13 lines
### `dcl`
- Definition location: [src/parser.c:1010-1041]
- Source file: `src/parser.c`
- Observed declaration: `int dcl(Ident *idptr);`
- Approximate function body length: 32 lines
### `getident`
- Definition location: [src/parser.c:1043-1086]
- Source file: `src/parser.c`
- Observed declaration: `int getident(Ident *idptr, int **parm_ptr);`
- Approximate function body length: 44 lines
### `dirdcl`
- Definition location: [src/parser.c:1088-1149]
- Source file: `src/parser.c`
- Observed declaration: `int dirdcl(Ident *idptr);`
- Approximate function body length: 62 lines
### `parmdcl`
- Definition location: [src/parser.c:1151-1173]
- Source file: `src/parser.c`
- Observed declaration: `int parmdcl(Ident *idptr);`
- Approximate function body length: 23 lines
### `maybe_parm_list`
- Definition location: [src/parser.c:1176-1223]
- Source file: `src/parser.c`
- Observed declaration: `void maybe_parm_list(int *parm_cnt_return);`
- Approximate function body length: 48 lines
### `func_body`
- Definition location: [src/parser.c:1225-1281]
- Source file: `src/parser.c`
- Observed declaration: `void func_body();`
- Approximate function body length: 57 lines
### `get_knr_args`
- Definition location: [src/parser.c:1283-1336]
- Source file: `src/parser.c`
- Observed declaration: `int get_knr_args(Ident *ident);`
- Approximate function body length: 54 lines
### `declare`
- Definition location: [src/parser.c:1338-1400]
- Source file: `src/parser.c`
- Observed declaration: `void declare(Ident *ident, int maybe_knr);`
- Approximate function body length: 63 lines
### `declare_type`
- Definition location: [src/parser.c:1402-1422]
- Source file: `src/parser.c`
- Observed declaration: `void declare_type(Ident *ident);`
- Approximate function body length: 21 lines
### `get_symbol`
- Definition location: [src/parser.c:1424-1445]
- Source file: `src/parser.c`
- Observed declaration: `Symbol * get_symbol(char *name);`
- Approximate function body length: 22 lines
### `add_reference`
- Definition location: [src/parser.c:1447-1463]
- Source file: `src/parser.c`
- Observed declaration: `Symbol * add_reference(char *name, int line);`
- Approximate function body length: 17 lines
### `call`
- Definition location: [src/parser.c:1466-1482]
- Source file: `src/parser.c`
- Observed declaration: `void call(char *name, int line);`
- Approximate function body length: 17 lines

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
- Definition location: [src/parser.c:21-27]
- Source file: `src/parser.c`
- Observed declaration prefix: `struct`
### `anonymous`
- Definition location: [src/parser.c:53]
- Source file: `src/parser.c`
- Observed declaration prefix: `struct obstack`
### `anonymous`
- Definition location: [src/parser.c:57-61]
- Source file: `src/parser.c`
- Observed declaration prefix: `struct`
### `anonymous`
- Definition location: [src/parser.c:475-479]
- Source file: `src/parser.c`
- Observed declaration prefix: `struct balance_state`
### `anonymous`
- Definition location: [src/parser.c:478]
- Source file: `src/parser.c`
- Observed declaration prefix: `struct balance_state`
### `anonymous`
- Definition location: [src/parser.c:482]
- Source file: `src/parser.c`
- Observed declaration prefix: `struct balance_state`
### `anonymous`
- Definition location: [src/parser.c:484]
- Source file: `src/parser.c`
- Observed declaration prefix: `struct balance_state`
### `anonymous`
- Definition location: [src/parser.c:492]
- Source file: `src/parser.c`
- Observed declaration prefix: `struct balance_state`
### `anonymous`
- Definition location: [src/parser.c:494]
- Source file: `src/parser.c`
- Observed declaration prefix: `struct balance_state`
### `anonymous`
- Definition location: [src/parser.c:502]
- Source file: `src/parser.c`
- Observed declaration prefix: `struct balance_state`
### `anonymous`
- Definition location: [src/parser.c:516]
- Source file: `src/parser.c`
- Observed declaration prefix: `struct balance_state`
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
