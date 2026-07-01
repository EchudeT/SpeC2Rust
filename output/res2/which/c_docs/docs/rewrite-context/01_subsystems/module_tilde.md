# Module Summary

This document is generated only from module partition results and parsed source facts; it does not turn "insufficient information" into "empty implementation" or "design mistake".

## 1. Module Responsibilities
- Observed focus: organized around source files related to `shell`, `tilde`
- Module category: `module`
- Directory scope: `tilde`

## 2. Inputs and Outputs
- At this stage, do not speculate about runtime I/O; define interface boundaries by the observed function signatures and source files.
- File input boundary: tilde/shell.c, tilde/tilde.c
- Function count: 4

## 3. Core Interface List
- `get_home_dir` [tilde/shell.c:59-70]: `char * get_home_dir ();`
- `tilde_find_prefix` [tilde/tilde.c:127-158]: `static int tilde_find_prefix (string, len) const char *string; int *len;`
- `tilde_find_suffix` [tilde/tilde.c:162-188]: `static int tilde_find_suffix (string) const char *string;`
- `memory_error_and_abort` [tilde/tilde.c:490-495]: `static void memory_error_and_abort ();`

## 4. Dependencies on Other Modules
- Internal call count: 0
- External call count: 0
- Cohesion score: 1.00
- Related headers: not recorded in the current module metadata.

## 5. Key Behaviors That Must Be Preserved
- At minimum, preserve the control flow and return conventions from the source code where these function definitions live; check the implementation body directly rather than inferring from the summary.
- Data structures defined in this module's files: `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`

## 6. Module Partition Signals
- The current module is already a consumable unit after partitioner convergence, with no additional split signals.

## Conclusion
- If function definitions really exist in the source, they should not be described as "empty implementations"; this document is grounded in source locations and declaration excerpts.
- "Module partitioning is unreasonable" should only come from real split signals from the partitioner, not from a summary model drawing conclusions when information is insufficient.
