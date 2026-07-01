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
- `skip_struct` [src/parser.c:953-974]: `void skip_struct();`
- `parse_typedef` [src/parser.c:976-994]: `void parse_typedef();`
- `parse_dcl` [src/parser.c:996-1008]: `void parse_dcl(Ident *ident, int maybe_knr);`
- `dcl` [src/parser.c:1010-1041]: `int dcl(Ident *idptr);`
- `getident` [src/parser.c:1043-1086]: `int getident(Ident *idptr, int **parm_ptr);`
- `dirdcl` [src/parser.c:1088-1149]: `int dirdcl(Ident *idptr);`
- `parmdcl` [src/parser.c:1151-1173]: `int parmdcl(Ident *idptr);`
- `maybe_parm_list` [src/parser.c:1176-1223]: `void maybe_parm_list(int *parm_cnt_return);`
- `func_body` [src/parser.c:1225-1281]: `void func_body();`
- `get_knr_args` [src/parser.c:1283-1336]: `int get_knr_args(Ident *ident);`
- `declare` [src/parser.c:1338-1400]: `void declare(Ident *ident, int maybe_knr);`
- `declare_type` [src/parser.c:1402-1422]: `void declare_type(Ident *ident);`
- `get_symbol` [src/parser.c:1424-1445]: `Symbol * get_symbol(char *name);`
- `add_reference` [src/parser.c:1447-1463]: `Symbol * add_reference(char *name, int line);`
- `call` [src/parser.c:1466-1482]: `void call(char *name, int line);`

## 4. Dependencies on Other Modules
- Internal call count: 14
- External call count: 60
- Cohesion score: 0.19
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
