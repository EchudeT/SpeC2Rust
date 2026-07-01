# Module Summary

This document is generated only from module partition results and parsed source facts; it does not turn "insufficient information" into "empty implementation" or "design mistake".

## 1. Module Responsibilities
- Observed focus: organized around source files related to `depmap`
- Module category: `module_cluster`
- Directory scope: `src`

## 2. Inputs and Outputs
- At this stage, do not speculate about runtime I/O; define interface boundaries by the observed function signatures and source files.
- File input boundary: src/depmap.c
- Function count: 6

## 3. Core Interface List
- `transitive_closure` [src/depmap.c:29-74]: `static void transitive_closure(unsigned *R, int n);`
- `depmap_alloc` [src/depmap.c:82-91]: `cflow_depmap_t depmap_alloc(size_t count);`
- `depmap_rowptr` [src/depmap.c:93-97]: `static unsigned * depmap_rowptr(cflow_depmap_t dmap, size_t row);`
- `depmap_set` [src/depmap.c:99-104]: `void depmap_set(cflow_depmap_t dmap, size_t row, size_t col);`
- `depmap_isset` [src/depmap.c:106-111]: `int depmap_isset(cflow_depmap_t dmap, size_t row, size_t col);`
- `depmap_tc` [src/depmap.c:113-117]: `void depmap_tc(cflow_depmap_t dmap);`

## 4. Dependencies on Other Modules
- Internal call count: 3
- External call count: 1
- Cohesion score: 0.75
- Related headers: not recorded in the current module metadata.

## 5. Key Behaviors That Must Be Preserved
- At minimum, preserve the control flow and return conventions from the source code where these function definitions live; check the implementation body directly rather than inferring from the summary.
- Data structures defined in this module's files: `anonymous`

## 6. Module Partition Signals
- This module was split out of parent module `module_src`; cluster type: `file_local`.
- Actual reasons the parent module was split: 函数数过多(221); 职责不明确且目录范围较大

## Conclusion
- If function definitions really exist in the source, they should not be described as "empty implementations"; this document is grounded in source locations and declaration excerpts.
- "Module partitioning is unreasonable" should only come from real split signals from the partitioner, not from a summary model drawing conclusions when information is insufficient.
