# Module Summary

This document is generated only from module partition results and parsed source facts; it does not turn "insufficient information" into "empty implementation" or "design mistake".

## 1. Module Responsibilities
- Observed focus: organized around source files related to `wc`
- Module category: `module_cluster`
- Directory scope: `doc`

## 2. Inputs and Outputs
- At this stage, do not speculate about runtime I/O; define interface boundaries by the observed function signatures and source files.
- File input boundary: doc/wc.c
- Function count: 7

## 3. Core Interface List
- `error_print` [doc/wc.c:21-30]: `static void error_print (int perr, char *fmt, va_list ap);`
- `errf` [doc/wc.c:33-41]: `static void errf (char *fmt, ...);`
- `perrf` [doc/wc.c:45-53]: `static void perrf (char *fmt, ...);`
- `report` [doc/wc.c:56-60]: `void report (char *file, count_t ccount, count_t wcount, count_t lcount);`
- `isword` [doc/wc.c:63-67]: `static int isword (unsigned char c);`
- `getword` [doc/wc.c:77-104]: `int getword (FILE *fp);`
- `counter` [doc/wc.c:107-124]: `void counter (char *file);`

## 4. Dependencies on Other Modules
- Internal call count: 6
- External call count: 3
- Cohesion score: 0.67
- Related headers: not recorded in the current module metadata.

## 5. Key Behaviors That Must Be Preserved
- At minimum, preserve the control flow and return conventions from the source code where these function definitions live; check the implementation body directly rather than inferring from the summary.
- No struct definitions were parsed for the current module.

## 6. Module Partition Signals
- This module was split out of parent module `module_doc`; cluster type: `file_local`.
- Actual reasons the parent module was split: 内聚度较低(0.28)

## Conclusion
- If function definitions really exist in the source, they should not be described as "empty implementations"; this document is grounded in source locations and declaration excerpts.
- "Module partitioning is unreasonable" should only come from real split signals from the partitioner, not from a summary model drawing conclusions when information is insufficient.
