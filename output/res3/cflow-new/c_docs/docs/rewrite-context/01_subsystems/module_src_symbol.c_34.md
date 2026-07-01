# Module Summary

This document is generated only from module partition results and parsed source facts; it does not turn "insufficient information" into "empty implementation" or "design mistake".

## 1. Module Responsibilities
- Observed focus: organized around source files related to `symbol`
- Module category: `module_cluster`
- Directory scope: `src`

## 2. Inputs and Outputs
- At this stage, do not speculate about runtime I/O; define interface boundaries by the observed function signatures and source files.
- File input boundary: src/symbol.c
- Function count: 12

## 3. Core Interface List
- `symbol_unlink_from_list` [src/symbol.c:30-37]: `static void symbol_unlink_from_list(Symbol *sp);`
- `ident_change_storage` [src/symbol.c:139-160]: `void ident_change_storage(Symbol *sp, enum storage storage);`
- `init_ident` [src/symbol.c:162-175]: `void init_ident(Symbol *sp, enum storage storage);`
- `install_ident` [src/symbol.c:177-187]: `Symbol * install_ident(char *name, enum storage storage);`
- `delete_symbol` [src/symbol.c:212-224]: `static void delete_symbol(Symbol *sym);`
- `delete_statics` [src/symbol.c:260-268]: `void delete_statics();`
- `delete_autos` [src/symbol.c:298-303]: `void delete_autos(int level);`
- `delete_parms` [src/symbol.c:406-410]: `void delete_parms(int level);`
- `install_starter` [src/symbol.c:430-438]: `Symbol * install_starter(char *name);`
- `set_default_starter` [src/symbol.c:440-445]: `void set_default_starter(void);`
- `clear_starters` [src/symbol.c:447-451]: `void clear_starters(void);`
- `install_target` [src/symbol.c:489-497]: `Symbol * install_target(char *name);`

## 4. Dependencies on Other Modules
- Internal call count: 4
- External call count: 16
- Cohesion score: 0.20
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
