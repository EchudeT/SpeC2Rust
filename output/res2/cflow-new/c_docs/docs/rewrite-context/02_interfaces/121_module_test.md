# Interface Facts: module_test

This document is intended for the later Rust repository-level rewrite and keeps only the interface facts directly observed in the current source-analysis stage.
Header files, macros, error codes, and configuration items that do not appear in the current parse results will not be added or assumed.

## Module Scope
- Module category: `module`
- Directory: `test`
- File list: test/multi.c, test/recursion.c, test/simple.c
- Candidate header files: gnu/stdio.h
- Exported functions observed: 11
- Struct definitions observed: 9
- Type names referenced but not defined locally: 0
- Macros observed in related files: 14
- Global variables observed: 0

## Header Evidence
- `gnu/stdio.h` [gnu/stdio.h]

## Functions
### `helper`
- Definition location: [test/multi.c:4-6]
- Source file: `test/multi.c`
- Observed declaration: `(void);`
- Approximate function body length: 3 lines
### `twice`
- Definition location: [test/multi.c:8-10]
- Source file: `test/multi.c`
- Observed declaration: `nt x);`
- Approximate function body length: 3 lines
### `run`
- Definition location: [test/multi.c:12-16]
- Source file: `test/multi.c`
- Observed declaration: `helper(); int r = twice(42); printf("%d\n", r); };`
- Approximate function body length: 5 lines
### `fib`
- Definition location: [test/recursion.c:2-6]
- Source file: `test/recursion.c`
- Observed declaration: `n);`
- Approximate function body length: 5 lines
### `fact`
- Definition location: [test/recursion.c:8-12]
- Source file: `test/recursion.c`
- Observed declaration: `n);`
- Approximate function body length: 5 lines
### `main`
- Definition location: [test/recursion.c:14-16]
- Source file: `test/recursion.c`
- Observed declaration: `d);`
- Approximate function body length: 3 lines
### `add`
- Definition location: [test/simple.c:2-4]
- Source file: `test/simple.c`
- Observed declaration: `b);`
- Approximate function body length: 3 lines
### `mul`
- Definition location: [test/simple.c:6-8]
- Source file: `test/simple.c`
- Observed declaration: `y);`
- Approximate function body length: 3 lines
### `orphan`
- Definition location: [test/simple.c:10-12]
- Source file: `test/simple.c`
- Observed declaration: `return 42; } int compute(int;`
- Approximate function body length: 3 lines
### `compute`
- Definition location: [test/simple.c:14-19]
- Source file: `test/simple.c`
- Observed declaration: `{;`
- Approximate function body length: 6 lines
### `main`
- Definition location: [test/simple.c:21-23]
- Source file: `test/simple.c`
- Observed declaration: `char **argv);`
- Approximate function body length: 3 lines

## Structs and Types
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

## Referenced External Types
- No external struct or type references beyond local definitions were recorded.

## Macros and Constants
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
- `_GL_STDIO_STRINGIZE` [gnu/stdio.h:760]: `#define _GL_STDIO_STRINGIZE(token) #token`
- `_GL_STDIO_MACROEXPAND_AND_STRINGIZE` [gnu/stdio.h:761]: `#define _GL_STDIO_MACROEXPAND_AND_STRINGIZE(token) _GL_STDIO_STRINGIZE(token)`

## Global Variables
- No global variable definitions were observed in the current module's `.c` files.

## Known Gaps
- This document is generated from parsed results for functions, structs, macros, and global variables; it does not infer declaration signatures from `.h` files that were not parsed.
- If a function appears in the "Functions" section without an explicit header binding, the later Rust migration should re-check the corresponding source `#include` relationships and build scripts.
- Error codes, configuration items, and input/output protocols are recorded only when explicit symbols appear in the source; missing entries do not mean the semantics do not exist, only that the current fact extraction did not observe them.
