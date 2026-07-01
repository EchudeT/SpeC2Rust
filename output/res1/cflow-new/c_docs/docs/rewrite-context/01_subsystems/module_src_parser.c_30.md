# Module Summary

This document is generated only from module partition results and parsed source facts; it does not turn "insufficient information" into "empty implementation" or "design mistake".

## 1. Module Responsibilities
- Observed focus: organized around source files related to `parser`
- Module category: `module_cluster`
- Directory scope: `src`

## 2. Inputs and Outputs
- At this stage, do not speculate about runtime I/O; define interface boundaries by the observed function signatures and source files.
- File input boundary: src/parser.c
- Function count: 15

## 3. Core Interface List
- `save_token` [src/parser.c:333-406]: `void save_token(TOKSTK *tokptr);`
- `undo_save_stack` [src/parser.c:418-422]: `void undo_save_stack();`
- `finish_save_stack` [src/parser.c:430-464]: `char * finish_save_stack(char *name);`
- `skip_to` [src/parser.c:466-473]: `void skip_to(int c);`
- `skip_balanced` [src/parser.c:556-563]: `static inline int skip_balanced(int open_tok, int level);`
- `yyparse` [src/parser.c:565-602]: `int yyparse();`
- `is_function` [src/parser.c:604-640]: `static int is_function();`
- `parse_declaration` [src/parser.c:642-650]: `void parse_declaration(Ident *ident, int parm);`
- `skip_declaration` [src/parser.c:652-659]: `void skip_declaration(void);`
- `expression` [src/parser.c:661-751]: `void expression();`
- `parse_function_declaration` [src/parser.c:753-793]: `void parse_function_declaration(Ident *ident, int parm);`
- `fake_struct` [src/parser.c:795-827]: `int fake_struct(Ident *ident);`
- `parse_variable_declaration` [src/parser.c:829-913]: `void parse_variable_declaration(Ident *ident, int parm);`
- `initializer_list` [src/parser.c:915-944]: `void initializer_list();`
- `parse_knr_dcl` [src/parser.c:946-951]: `void parse_knr_dcl(Ident *ident);`

## 4. Dependencies on Other Modules
- Internal call count: 13
- External call count: 42
- Cohesion score: 0.24
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
