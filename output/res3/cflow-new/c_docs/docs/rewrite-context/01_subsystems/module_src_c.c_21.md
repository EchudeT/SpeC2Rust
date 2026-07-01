# Module Summary

This document is generated only from module partition results and parsed source facts; it does not turn "insufficient information" into "empty implementation" or "design mistake".

## 1. Module Responsibilities
- Observed focus: organized around source files related to `c`
- Module category: `module_cluster`
- Directory scope: `src`

## 2. Inputs and Outputs
- At this stage, do not speculate about runtime I/O; define interface boundaries by the observed function signatures and source files.
- File input boundary: src/c.c
- Function count: 15

## 3. Core Interface List
- `yyget_text` [src/c.c:2576-2579]: `char *yyget_text (void);`
- `yyset_lineno` [src/c.c:2588-2592]: `void yyset_lineno (int _line_number );`
- `yyset_in` [src/c.c:2600-2603]: `void yyset_in (FILE * _in_str );`
- `yyset_out` [src/c.c:2605-2608]: `void yyset_out (FILE * _out_str );`
- `yyget_debug` [src/c.c:2610-2613]: `int yyget_debug (void);`
- `yyset_debug` [src/c.c:2615-2618]: `void yyset_debug (int _bdebug );`
- `yylex_destroy` [src/c.c:2659-2680]: `int yylex_destroy (void);`
- `yyalloc` [src/c.c:2708-2711]: `void *yyalloc (yy_size_t size );`
- `yyrealloc` [src/c.c:2713-2724]: `void *yyrealloc (void * ptr, yy_size_t size );`
- `yyfree` [src/c.c:2726-2729]: `void yyfree (void * ptr );`
- `init_tokens` [src/c.c:2777-2812]: `void init_tokens();`
- `init_lex` [src/c.c:2814-2820]: `void init_lex(int debug_level);`
- `ident` [src/c.c:2822-2844]: `int ident();`
- `set_preprocessor` [src/c.c:2852-2856]: `void set_preprocessor(const char *arg);`
- `pp_option` [src/c.c:2858-2871]: `void pp_option(int opt, const char *arg);`

## 4. Dependencies on Other Modules
- Internal call count: 2
- External call count: 7
- Cohesion score: 0.22
- Related headers: not recorded in the current module metadata.

## 5. Key Behaviors That Must Be Preserved
- At minimum, preserve the control flow and return conventions from the source code where these function definitions live; check the implementation body directly rather than inferring from the summary.
- Data structures defined in this module's files: `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`

## 6. Module Partition Signals
- This module was split out of parent module `module_src`; cluster type: `file_local`.
- Actual reasons the parent module was split: 函数数过多(221); 职责不明确且目录范围较大

## Conclusion
- If function definitions really exist in the source, they should not be described as "empty implementations"; this document is grounded in source locations and declaration excerpts.
- "Module partitioning is unreasonable" should only come from real split signals from the partitioner, not from a summary model drawing conclusions when information is insufficient.
