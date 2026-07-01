# Module Summary

This document is generated only from module partition results and parsed source facts; it does not turn "insufficient information" into "empty implementation" or "design mistake".

## 1. Module Responsibilities
- Observed focus: organized around source files related to `main`
- Module category: `module_cluster`
- Directory scope: `src`

## 2. Inputs and Outputs
- At this stage, do not speculate about runtime I/O; define interface boundaries by the observed function signatures and source files.
- File input boundary: src/main.c
- Function count: 12

## 3. Core Interface List
- `CHAR_TO_SM` [src/main.c:75-86]: `static inline int CHAR_TO_SM(int c);`
- `find_option_type` [src/main.c:106-119]: `static int find_option_type(struct option_type *optype, const char *str, int len);`
- `symbol_override` [src/main.c:138-190]: `static void symbol_override(const char *str);`
- `number` [src/main.c:197-217]: `static int number(const char **str_ptr, int base, int count);`
- `parse_level_string` [src/main.c:250-322]: `static void parse_level_string(const char *str, char **return_ptr);`
- `tildexpand` [src/main.c:1101-1133]: `static char * tildexpand(char const *str);`
- `parse_rc` [src/main.c:1289-1317]: `int parse_rc(void);`
- `globals_only` [src/main.c:1340-1344]: `int globals_only();`
- `include_symbol` [src/main.c:1346-1378]: `int include_symbol(Symbol *sym);`
- `xalloc_die` [src/main.c:1380-1385]: `void xalloc_die(void);`
- `init` [src/main.c:1387-1401]: `void init();`
- `main` [src/main.c:1403-1470]: `int main(int argc, char **argv);`

## 4. Dependencies on Other Modules
- Internal call count: 6
- External call count: 36
- Cohesion score: 0.14
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
