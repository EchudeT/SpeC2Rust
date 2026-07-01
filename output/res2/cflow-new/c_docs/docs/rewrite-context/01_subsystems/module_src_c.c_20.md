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
- `if` [src/c.c:1598-1618]: `YY_RESTORE_YY_MORE_OFFSET if ( YY_CURRENT_BUFFER_LVALUE->yy_buffer_status == YY_BUFFER_NEW );`
- `yy_try_NUL_trans` [src/c.c:1923-1948]: `static yy_state_type yy_try_NUL_trans (yy_state_type yy_current_state ) /* %endif */ /* %if-c++-only */ /* %endif */;`
- `yyunput` [src/c.c:1953-1993]: `static void yyunput (int c, char * yy_bp ) /* %endif */ /* %if-c++-only */ /* %endif */;`
- `yyrestart` [src/c.c:2088-2102]: `void yyrestart (FILE * input_file ) /* %endif */ /* %if-c++-only */ /* %endif */;`
- `yy_switch_to_buffer` [src/c.c:2112-2144]: `void yy_switch_to_buffer (YY_BUFFER_STATE new_buffer ) /* %endif */ /* %if-c++-only */ /* %endif */;`
- `yy_load_buffer_state` [src/c.c:2147-2160]: `static void yy_load_buffer_state (void) /* %endif */ /* %if-c++-only */ /* %endif */;`
- `yy_delete_buffer` [src/c.c:2204-2220]: `void yy_delete_buffer (YY_BUFFER_STATE b ) /* %endif */ /* %if-c++-only */ /* %endif */;`
- `yy_flush_buffer` [src/c.c:2268-2292]: `void yy_flush_buffer (YY_BUFFER_STATE b ) /* %endif */ /* %if-c++-only */ /* %endif */;`
- `yypush_buffer_state` [src/c.c:2302-2329]: `void yypush_buffer_state (YY_BUFFER_STATE new_buffer ) /* %endif */ /* %if-c++-only */ /* %endif */;`
- `yypop_buffer_state` [src/c.c:2338-2355]: `void yypop_buffer_state (void) /* %endif */ /* %if-c++-only */ /* %endif */;`
- `yy_fatal_error` [src/c.c:2507-2511]: `static void yynoreturn yy_fatal_error (const char* msg );`
- `yyget_lineno` [src/c.c:2542-2546]: `int yyget_lineno (void);`
- `yyget_in` [src/c.c:2551-2554]: `FILE *yyget_in (void);`
- `yyget_out` [src/c.c:2559-2562]: `FILE *yyget_out (void);`
- `yyget_leng` [src/c.c:2567-2570]: `int yyget_leng (void);`

## 4. Dependencies on Other Modules
- Internal call count: 6
- External call count: 15
- Cohesion score: 0.29
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
