# Module Summary

This document is generated only from module partition results and parsed source facts; it does not turn "insufficient information" into "empty implementation" or "design mistake".

## 1. Module Responsibilities
- Observed focus: organized around functions with the `setlocale_null` prefix
- Module category: `main_cluster`
- Directory scope: `root`

## 2. Inputs and Outputs
- At this stage, do not speculate about runtime I/O; define interface boundaries by the observed function signatures and source files.
- File input boundary: setlocale_null-unlocked.c, setlocale_null.c
- Function count: 7

## 3. Core Interface List
- `setlocale_null_unlocked` [setlocale_null-unlocked.c:34-63]: `const char * setlocale_null_unlocked (int category);`
- `setlocale_null_r_unlocked` [setlocale_null-unlocked.c:65-149]: `int setlocale_null_r_unlocked (int category, char *buf, size_t bufsize);`
- `setlocale_null_r_with_lock` [setlocale_null.c:76-87]: `static int setlocale_null_r_with_lock (int category, char *buf, size_t bufsize);`
- `setlocale_null_r_with_lock` [setlocale_null.c:113-131]: `static int setlocale_null_r_with_lock (int category, char *buf, size_t bufsize);`
- `setlocale_null_r_with_lock` [setlocale_null.c:137-150]: `static int setlocale_null_r_with_lock (int category, char *buf, size_t bufsize);`
- `setlocale_null_r` [setlocale_null.c:156-186]: `int setlocale_null_r (int category, char *buf, size_t bufsize);`
- `setlocale_null` [setlocale_null.c:188-299]: `const char * setlocale_null (int category);`

## 4. Dependencies on Other Modules
- Internal call count: 8
- External call count: 0
- Cohesion score: 1.00
- Related headers: not recorded in the current module metadata.

## 5. Key Behaviors That Must Be Preserved
- At minimum, preserve the control flow and return conventions from the source code where these function definitions live; check the implementation body directly rather than inferring from the summary.
- No struct definitions were parsed for the current module.

## 6. Module Partition Signals
- This module was split out of parent module `main_root`; cluster type: `prefix_based`.
- Actual reasons the parent module was split: 文件数过多(38); 函数数过多(113)

## Conclusion
- If function definitions really exist in the source, they should not be described as "empty implementations"; this document is grounded in source locations and declaration excerpts.
- "Module partitioning is unreasonable" should only come from real split signals from the partitioner, not from a summary model drawing conclusions when information is insufficient.
