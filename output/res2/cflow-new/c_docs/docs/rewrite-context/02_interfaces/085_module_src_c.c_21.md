# Interface Facts: module_src_c.c_21

This document is intended for the later Rust repository-level rewrite and keeps only the interface facts directly observed in the current source-analysis stage.
Header files, macros, error codes, and configuration items that do not appear in the current parse results will not be added or assumed.

## Module Scope
- Module category: `module_cluster`
- Directory: `src`
- File list: src/c.c
- Candidate header files: gnu/inttypes.h, gnu/stdio.h, gnu/stdlib.h, gnu/string.h, gnu/unistd.h, src/cflow.h, src/parser.h
- Exported functions observed: 15
- Struct definitions observed: 54
- Type names referenced but not defined locally: 1
- Macros observed in related files: 20
- Global variables observed: 0

## Header Evidence
- `gnu/inttypes.h` [gnu/inttypes.h]
- `gnu/stdio.h` [gnu/stdio.h]
- `gnu/stdlib.h` [gnu/stdlib.h]
- `gnu/string.h` [gnu/string.h]
- `gnu/unistd.h` [gnu/unistd.h]
- `src/cflow.h` [src/cflow.h]
- `src/parser.h` [src/parser.h]

## Functions
### `yyget_text`
- Definition location: [src/c.c:2576-2579]
- Source file: `src/c.c`
- Observed declaration: `char *yyget_text (void);`
- Approximate function body length: 4 lines
### `yyset_lineno`
- Definition location: [src/c.c:2588-2592]
- Source file: `src/c.c`
- Observed declaration: `void yyset_lineno (int _line_number );`
- Approximate function body length: 5 lines
### `yyset_in`
- Definition location: [src/c.c:2600-2603]
- Source file: `src/c.c`
- Observed declaration: `void yyset_in (FILE * _in_str );`
- Approximate function body length: 4 lines
### `yyset_out`
- Definition location: [src/c.c:2605-2608]
- Source file: `src/c.c`
- Observed declaration: `void yyset_out (FILE * _out_str );`
- Approximate function body length: 4 lines
### `yyget_debug`
- Definition location: [src/c.c:2610-2613]
- Source file: `src/c.c`
- Observed declaration: `int yyget_debug (void);`
- Approximate function body length: 4 lines
### `yyset_debug`
- Definition location: [src/c.c:2615-2618]
- Source file: `src/c.c`
- Observed declaration: `void yyset_debug (int _bdebug );`
- Approximate function body length: 4 lines
### `yylex_destroy`
- Definition location: [src/c.c:2659-2680]
- Source file: `src/c.c`
- Observed declaration: `int yylex_destroy (void);`
- Approximate function body length: 22 lines
### `yyalloc`
- Definition location: [src/c.c:2708-2711]
- Source file: `src/c.c`
- Observed declaration: `void *yyalloc (yy_size_t size );`
- Approximate function body length: 4 lines
### `yyrealloc`
- Definition location: [src/c.c:2713-2724]
- Source file: `src/c.c`
- Observed declaration: `void *yyrealloc (void * ptr, yy_size_t size );`
- Approximate function body length: 12 lines
### `yyfree`
- Definition location: [src/c.c:2726-2729]
- Source file: `src/c.c`
- Observed declaration: `void yyfree (void * ptr );`
- Approximate function body length: 4 lines
### `init_tokens`
- Definition location: [src/c.c:2777-2812]
- Source file: `src/c.c`
- Observed declaration: `void init_tokens();`
- Approximate function body length: 36 lines
### `init_lex`
- Definition location: [src/c.c:2814-2820]
- Source file: `src/c.c`
- Observed declaration: `void init_lex(int debug_level);`
- Approximate function body length: 7 lines
### `ident`
- Definition location: [src/c.c:2822-2844]
- Source file: `src/c.c`
- Observed declaration: `int ident();`
- Approximate function body length: 23 lines
### `set_preprocessor`
- Definition location: [src/c.c:2852-2856]
- Source file: `src/c.c`
- Observed declaration: `void set_preprocessor(const char *arg);`
- Approximate function body length: 5 lines
### `pp_option`
- Definition location: [src/c.c:2858-2871]
- Source file: `src/c.c`
- Observed declaration: `void pp_option(int opt, const char *arg);`
- Approximate function body length: 14 lines

## Structs and Types
### `anonymous`
- Definition location: [gnu/inttypes.h:1459]
- Source file: `gnu/inttypes.h`
- Observed declaration prefix: `struct`
### `anonymous`
- Definition location: [gnu/stdio.h:1599]
- Source file: `gnu/stdio.h`
- Observed declaration prefix: `struct obstack`
### `anonymous`
- Definition location: [gnu/stdio.h:1610]
- Source file: `gnu/stdio.h`
- Observed declaration prefix: `struct obstack`
### `anonymous`
- Definition location: [gnu/stdio.h:1614]
- Source file: `gnu/stdio.h`
- Observed declaration prefix: `struct obstack`
### `anonymous`
- Definition location: [gnu/stdio.h:1618]
- Source file: `gnu/stdio.h`
- Observed declaration prefix: `struct obstack`
### `anonymous`
- Definition location: [gnu/stdio.h:1623]
- Source file: `gnu/stdio.h`
- Observed declaration prefix: `struct obstack`
### `anonymous`
- Definition location: [gnu/stdio.h:1631]
- Source file: `gnu/stdio.h`
- Observed declaration prefix: `struct obstack`
### `anonymous`
- Definition location: [gnu/stdio.h:1635]
- Source file: `gnu/stdio.h`
- Observed declaration prefix: `struct obstack`
### `anonymous`
- Definition location: [gnu/stdio.h:1639]
- Source file: `gnu/stdio.h`
- Observed declaration prefix: `struct obstack`
### `anonymous`
- Definition location: [gnu/stdio.h:1644]
- Source file: `gnu/stdio.h`
- Observed declaration prefix: `struct obstack`
### `anonymous`
- Definition location: [gnu/stdlib.h:86-95]
- Source file: `gnu/stdlib.h`
- Observed declaration prefix: `struct random_data`
### `anonymous`
- Definition location: [gnu/stdlib.h:1843]
- Source file: `gnu/stdlib.h`
- Observed declaration prefix: `struct random_data`
### `anonymous`
- Definition location: [gnu/stdlib.h:1845]
- Source file: `gnu/stdlib.h`
- Observed declaration prefix: `struct random_data`
### `anonymous`
- Definition location: [gnu/stdlib.h:1848]
- Source file: `gnu/stdlib.h`
- Observed declaration prefix: `struct random_data`
### `anonymous`
- Definition location: [gnu/stdlib.h:1851]
- Source file: `gnu/stdlib.h`
- Observed declaration prefix: `struct random_data`
### `anonymous`
- Definition location: [src/c.c:191]
- Source file: `src/c.c`
- Observed declaration prefix: `struct yy_buffer_state`
### `anonymous`
- Definition location: [src/c.c:233-298]
- Source file: `src/c.c`
- Observed declaration prefix: `struct yy_buffer_state`
### `anonymous`
- Definition location: [src/c.c:440-444]
- Source file: `src/c.c`
- Observed declaration prefix: `struct yy_trans_info`
### `anonymous`
- Definition location: [src/c.c:760]
- Source file: `src/c.c`
- Observed declaration prefix: `struct obstack`
### `anonymous`
- Definition location: [src/c.c:2176]
- Source file: `src/c.c`
- Observed declaration prefix: `struct yy_buffer_state`
### `anonymous`
- Definition location: [src/c.c:2377]
- Source file: `src/c.c`
- Observed declaration prefix: `struct yy_buffer_state`
### `anonymous`
- Definition location: [src/c.c:2378]
- Source file: `src/c.c`
- Observed declaration prefix: `struct yy_buffer_state`
### `anonymous`
- Definition location: [src/c.c:2383]
- Source file: `src/c.c`
- Observed declaration prefix: `struct yy_buffer_state`
### `anonymous`
- Definition location: [src/c.c:2396]
- Source file: `src/c.c`
- Observed declaration prefix: `struct yy_buffer_state`
### `anonymous`
- Definition location: [src/c.c:2398]
- Source file: `src/c.c`
- Observed declaration prefix: `struct yy_buffer_state`
### `anonymous`
- Definition location: [src/c.c:2404]
- Source file: `src/c.c`
- Observed declaration prefix: `struct yy_buffer_state`
### `anonymous`
- Definition location: [src/c.c:2427]
- Source file: `src/c.c`
- Observed declaration prefix: `struct yy_buffer_state`
### `anonymous`
- Definition location: [src/c.c:2850]
- Source file: `src/c.c`
- Observed declaration prefix: `struct obstack`
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
- Definition location: [src/parser.h:41-43]
- Source file: `src/parser.h`
- Observed declaration prefix: `struct`

## Referenced External Types
- `foo`: this name came from clustering metadata or nearby call analysis, but no local definition was observed in the current module files.

## Macros and Constants
- `INTTYPES_H` [gnu/inttypes.h:50]: `#define INTTYPES_H`
- `_GL_CXXDEFS_H` [gnu/inttypes.h:93]: `#define _GL_CXXDEFS_H`
- `_GL_FUNCDECL_RPL` [gnu/inttypes.h:184-185]: `#define _GL_FUNCDECL_RPL(func,rettype,parameters_and_attributes) \ _GL_FUNCDECL_RPL_1 (rpl_##func, rettype, parameters_and_attributes)`
- `_GL_FUNCDECL_RPL_1` [gnu/inttypes.h:186-187]: `#define _GL_FUNCDECL_RPL_1(rpl_func,rettype,parameters_and_attributes) \ _GL_EXTERN_C rettype rpl_func parameters_and_attributes`
- `_GL_FUNCDECL_SYS` [gnu/inttypes.h:196-197]: `#define _GL_FUNCDECL_SYS(func,rettype,parameters_and_attributes) \ _GL_EXTERN_C rettype func parameters_and_attributes`
- `_GL_CXXALIAS_RPL` [gnu/inttypes.h:208-209]: `#define _GL_CXXALIAS_RPL(func,rettype,parameters) \ _GL_CXXALIAS_RPL_1 (func, rpl_##func, rettype, parameters)`
- `_GL_CXXALIAS_MDA` [gnu/inttypes.h:237-238]: `#define _GL_CXXALIAS_MDA(func,rettype,parameters) \ _GL_CXXALIAS_RPL_1 (func, _##func, rettype, parameters)`
- `_GL_CXXALIAS_MDA_CAST` [gnu/inttypes.h:270-271]: `#define _GL_CXXALIAS_MDA_CAST(func,rettype,parameters) \ _GL_CXXALIAS_RPL_CAST_1 (func, _##func, rettype, parameters)`
- `_GL_ALREADY_INCLUDING_STDIO_H` [gnu/stdio.h:54]: `#define _GL_ALREADY_INCLUDING_STDIO_H`
- `_GL_STDIO_H` [gnu/stdio.h:67]: `#define _GL_STDIO_H`
- `_GL_ATTRIBUTE_FORMAT_PRINTF_STANDARD` [gnu/stdio.h:206-207]: `#define _GL_ATTRIBUTE_FORMAT_PRINTF_STANDARD(formatstring_parameter, first_argument) \ _GL_ATTRIBUTE_FORMAT ((_GL_ATTRIBUTE_SPEC_PRINTF_STANDARD, formatstring_parameter, first_a...`
- `_GL_ATTRIBUTE_FORMAT_PRINTF_SYSTEM` [gnu/stdio.h:213-214]: `#define _GL_ATTRIBUTE_FORMAT_PRINTF_SYSTEM(formatstring_parameter, first_argument) \ _GL_ATTRIBUTE_FORMAT ((_GL_ATTRIBUTE_SPEC_PRINTF_SYSTEM, formatstring_parameter, first_argum...`
- `_GL_ATTRIBUTE_FORMAT_SCANF_SYSTEM` [gnu/stdio.h:232-233]: `#define _GL_ATTRIBUTE_FORMAT_SCANF_SYSTEM(formatstring_parameter, first_argument) \ _GL_ATTRIBUTE_FORMAT ((__scanf__, formatstring_parameter, first_argument))`
- `_GL_CXXDEFS_H` [gnu/stdio.h:255]: `#define _GL_CXXDEFS_H`
- `_GL_FUNCDECL_RPL` [gnu/stdio.h:346-347]: `#define _GL_FUNCDECL_RPL(func,rettype,parameters_and_attributes) \ _GL_FUNCDECL_RPL_1 (rpl_##func, rettype, parameters_and_attributes)`
- `_GL_FUNCDECL_RPL_1` [gnu/stdio.h:348-349]: `#define _GL_FUNCDECL_RPL_1(rpl_func,rettype,parameters_and_attributes) \ _GL_EXTERN_C rettype rpl_func parameters_and_attributes`
- `_GL_FUNCDECL_SYS` [gnu/stdio.h:358-359]: `#define _GL_FUNCDECL_SYS(func,rettype,parameters_and_attributes) \ _GL_EXTERN_C rettype func parameters_and_attributes`
- `_GL_CXXALIAS_RPL` [gnu/stdio.h:370-371]: `#define _GL_CXXALIAS_RPL(func,rettype,parameters) \ _GL_CXXALIAS_RPL_1 (func, rpl_##func, rettype, parameters)`
- `_GL_CXXALIAS_MDA` [gnu/stdio.h:399-400]: `#define _GL_CXXALIAS_MDA(func,rettype,parameters) \ _GL_CXXALIAS_RPL_1 (func, _##func, rettype, parameters)`
- `_GL_CXXALIAS_MDA_CAST` [gnu/stdio.h:432-433]: `#define _GL_CXXALIAS_MDA_CAST(func,rettype,parameters) \ _GL_CXXALIAS_RPL_CAST_1 (func, _##func, rettype, parameters)`

## Global Variables
- No global variable definitions were observed in the current module's `.c` files.

## Known Gaps
- This document is generated from parsed results for functions, structs, macros, and global variables; it does not infer declaration signatures from `.h` files that were not parsed.
- If a function appears in the "Functions" section without an explicit header binding, the later Rust migration should re-check the corresponding source `#include` relationships and build scripts.
- Error codes, configuration items, and input/output protocols are recorded only when explicit symbols appear in the source; missing entries do not mean the semantics do not exist, only that the current fact extraction did not observe them.
