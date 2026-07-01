# Module Summary

This document is generated only from module partition results and parsed source facts; it does not turn "insufficient information" into "empty implementation" or "design mistake".

## 1. Module Responsibilities
- Observed focus: organized around functions with the `print_symbol` prefix
- Module category: `module_cluster`
- Directory scope: `src`

## 2. Inputs and Outputs
- At this stage, do not speculate about runtime I/O; define interface boundaries by the observed function signatures and source files.
- File input boundary: src/gnu.c, src/output.c, src/posix.c
- Function count: 3

## 3. Core Interface List
- `print_symbol` [src/gnu.c:41-59]: `static int print_symbol(FILE *outfile, int line, struct output_symbol *s);`
- `print_symbol` [src/output.c:156-170]: `static int print_symbol (int direct, int level, int last, Symbol *sym);`
- `print_symbol` [src/posix.c:32-47]: `static int print_symbol(FILE *outfile, int line, struct output_symbol *s);`

## 4. Dependencies on Other Modules
- Internal call count: 0
- External call count: 12
- Cohesion score: 0.00
- Related headers: not recorded in the current module metadata.

## 5. Key Behaviors That Must Be Preserved
- At minimum, preserve the control flow and return conventions from the source code where these function definitions live; check the implementation body directly rather than inferring from the summary.
- Data structures defined in this module's files: `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`

## 6. Module Partition Signals
- This module was split out of parent module `module_src`; cluster type: `struct_based`.
- Actual reasons the parent module was split: 函数数过多(221); 职责不明确且目录范围较大

## Conclusion
- If function definitions really exist in the source, they should not be described as "empty implementations"; this document is grounded in source locations and declaration excerpts.
- "Module partitioning is unreasonable" should only come from real split signals from the partitioner, not from a summary model drawing conclusions when information is insufficient.
