# Module Summary

This document is generated only from module partition results and parsed source facts; it does not turn "insufficient information" into "empty implementation" or "design mistake".

## 1. Module Responsibilities
- Observed focus: organized around source files related to `output`
- Module category: `module_cluster`
- Directory scope: `src`

## 2. Inputs and Outputs
- At this stage, do not speculate about runtime I/O; define interface boundaries by the observed function signatures and source files.
- File input boundary: src/output.c
- Function count: 15

## 3. Core Interface List
- `print_level` [src/output.c:42-55]: `void print_level(int lev, int last);`
- `register_output` [src/output.c:72-85]: `int register_output(const char *name, int (*handler) (cflow_output_command cmd, FILE *outfile, int line, void *data, void *handler_data), void *handler_data);`
- `select_output_driver` [src/output.c:87-97]: `int select_output_driver(const char *name);`
- `output_init` [src/output.c:99-106]: `void output_init();`
- `newline` [src/output.c:108-116]: `void newline();`
- `begin` [src/output.c:118-125]: `static void begin();`
- `end` [src/output.c:127-134]: `static void end();`
- `separator` [src/output.c:136-143]: `static void separator();`
- `print_text` [src/output.c:146-153]: `static void print_text(char *buf);`
- `compare` [src/output.c:173-179]: `static int compare(const void *ap, const void *bp);`
- `is_var` [src/output.c:181-192]: `static int is_var(Symbol *symp);`
- `symbol_is_function` [src/output.c:194-198]: `int symbol_is_function(Symbol *symp);`
- `clear_active` [src/output.c:200-204]: `static void clear_active(Symbol *sym);`
- `print_type` [src/output.c:236-244]: `static void print_type(Symbol *symp);`
- `xref_output` [src/output.c:246-270]: `void xref_output();`

## 4. Dependencies on Other Modules
- Internal call count: 1
- External call count: 5
- Cohesion score: 0.17
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
