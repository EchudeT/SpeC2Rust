# Module Summary

This document is generated only from module partition results and parsed source facts; it does not turn "insufficient information" into "empty implementation" or "design mistake".

## 1. Module Responsibilities
- Observed focus: organized around functions with the `rpl_fcntl_DUPFD` prefix
- Module category: `main_cluster`
- Directory scope: `root`

## 2. Inputs and Outputs
- At this stage, do not speculate about runtime I/O; define interface boundaries by the observed function signatures and source files.
- File input boundary: fcntl.c
- Function count: 2

## 3. Core Interface List
- `rpl_fcntl_DUPFD` [fcntl.c:446-485]: `static int rpl_fcntl_DUPFD (int fd, int target);`
- `rpl_fcntl_DUPFD_CLOEXEC` [fcntl.c:487-544]: `static int rpl_fcntl_DUPFD_CLOEXEC (int fd, int target);`

## 4. Dependencies on Other Modules
- Internal call count: 1
- External call count: 2
- Cohesion score: 0.33
- Related headers: not recorded in the current module metadata.

## 5. Key Behaviors That Must Be Preserved
- At minimum, preserve the control flow and return conventions from the source code where these function definitions live; check the implementation body directly rather than inferring from the summary.
- Data structures defined in this module's files: `anonymous`

## 6. Module Partition Signals
- This module was split out of parent module `main_root`; cluster type: `prefix_based`.
- Actual reasons the parent module was split: 文件数过多(38); 函数数过多(113)

## Conclusion
- If function definitions really exist in the source, they should not be described as "empty implementations"; this document is grounded in source locations and declaration excerpts.
- "Module partitioning is unreasonable" should only come from real split signals from the partitioner, not from a summary model drawing conclusions when information is insufficient.
