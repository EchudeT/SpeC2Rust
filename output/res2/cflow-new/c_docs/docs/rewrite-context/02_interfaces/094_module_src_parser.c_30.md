# Interface Facts: module_src_parser.c_30

This document is intended for the later Rust repository-level rewrite and keeps only the interface facts directly observed in the current source-analysis stage.
Header files, macros, error codes, and configuration items that do not appear in the current parse results will not be added or assumed.

## Module Scope
- Module category: `module_cluster`
- Directory: `src`
- File list: src/parser.c
- Candidate header files: src/cflow.h, src/parser.h
- Exported functions observed: 15
- Struct definitions observed: 37
- Type names referenced but not defined locally: 1
- Macros observed in related files: 18
- Global variables observed: 0

## Header Evidence
- `src/cflow.h` [src/cflow.h]
- `src/parser.h` [src/parser.h]

## Functions
### `save_token`
- Definition location: [src/parser.c:333-406]
- Source file: `src/parser.c`
- Observed declaration: `void save_token(TOKSTK *tokptr);`
- Approximate function body length: 74 lines
### `undo_save_stack`
- Definition location: [src/parser.c:418-422]
- Source file: `src/parser.c`
- Observed declaration: `void undo_save_stack();`
- Approximate function body length: 5 lines
### `finish_save_stack`
- Definition location: [src/parser.c:430-464]
- Source file: `src/parser.c`
- Observed declaration: `char * finish_save_stack(char *name);`
- Approximate function body length: 35 lines
### `skip_to`
- Definition location: [src/parser.c:466-473]
- Source file: `src/parser.c`
- Observed declaration: `void skip_to(int c);`
- Approximate function body length: 8 lines
### `skip_balanced`
- Definition location: [src/parser.c:556-563]
- Source file: `src/parser.c`
- Observed declaration: `static inline int skip_balanced(int open_tok, int level);`
- Approximate function body length: 8 lines
### `yyparse`
- Definition location: [src/parser.c:565-602]
- Source file: `src/parser.c`
- Observed declaration: `int yyparse();`
- Approximate function body length: 38 lines
### `is_function`
- Definition location: [src/parser.c:604-640]
- Source file: `src/parser.c`
- Observed declaration: `static int is_function();`
- Approximate function body length: 37 lines
### `parse_declaration`
- Definition location: [src/parser.c:642-650]
- Source file: `src/parser.c`
- Observed declaration: `void parse_declaration(Ident *ident, int parm);`
- Approximate function body length: 9 lines
### `skip_declaration`
- Definition location: [src/parser.c:652-659]
- Source file: `src/parser.c`
- Observed declaration: `void skip_declaration(void);`
- Approximate function body length: 8 lines
### `expression`
- Definition location: [src/parser.c:661-751]
- Source file: `src/parser.c`
- Observed declaration: `void expression();`
- Approximate function body length: 91 lines
### `parse_function_declaration`
- Definition location: [src/parser.c:753-793]
- Source file: `src/parser.c`
- Observed declaration: `void parse_function_declaration(Ident *ident, int parm);`
- Approximate function body length: 41 lines
### `fake_struct`
- Definition location: [src/parser.c:795-827]
- Source file: `src/parser.c`
- Observed declaration: `int fake_struct(Ident *ident);`
- Approximate function body length: 33 lines
### `parse_variable_declaration`
- Definition location: [src/parser.c:829-913]
- Source file: `src/parser.c`
- Observed declaration: `void parse_variable_declaration(Ident *ident, int parm);`
- Approximate function body length: 85 lines
### `initializer_list`
- Definition location: [src/parser.c:915-944]
- Source file: `src/parser.c`
- Observed declaration: `void initializer_list();`
- Approximate function body length: 30 lines
### `parse_knr_dcl`
- Definition location: [src/parser.c:946-951]
- Source file: `src/parser.c`
- Observed declaration: `void parse_knr_dcl(Ident *ident);`
- Approximate function body length: 6 lines

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
- `declaration`: this name came from clustering metadata or nearby call analysis, but no local definition was observed in the current module files.

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
