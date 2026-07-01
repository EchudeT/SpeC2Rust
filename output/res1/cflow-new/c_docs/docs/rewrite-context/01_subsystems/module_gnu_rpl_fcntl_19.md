# Module Summary

This document is generated only from module partition results and parsed source facts; it does not turn "insufficient information" into "empty implementation" or "design mistake".

## 1. Module Responsibilities
- Observed focus: organized around functions with the `rpl_fcntl_DUPFD` prefix
- Module category: `module_cluster`
- Directory scope: `gnu`

## 2. Inputs and Outputs
- At this stage, do not speculate about runtime I/O; define interface boundaries by the observed function signatures and source files.
- File input boundary: gnu/fcntl.c
- Function count: 2

## 3. Core Interface List
- `rpl_fcntl_DUPFD` [gnu/fcntl.c:448-487]: `static int rpl_fcntl_DUPFD (int fd, int target);`
- `rpl_fcntl_DUPFD_CLOEXEC` [gnu/fcntl.c:489-546]: `static int rpl_fcntl_DUPFD_CLOEXEC (int fd, int target);`

## 4. Dependencies on Other Modules
- Internal call count: 1
- External call count: 3
- Cohesion score: 0.25
- Related headers: not recorded in the current module metadata.

## 5. Key Behaviors That Must Be Preserved
- At minimum, preserve the control flow and return conventions from the source code where these function definitions live; check the implementation body directly rather than inferring from the summary.
- Data structures defined in this module's files: `anonymous`

## 6. Module Partition Signals
- This module was split out of parent module `module_gnu`; cluster type: `prefix_based`.
- Actual reasons the parent module was split: 文件数过多(45); 函数数过多(174); 职责不明确且目录范围较大

## Conclusion
- If function definitions really exist in the source, they should not be described as "empty implementations"; this document is grounded in source locations and declaration excerpts.
- "Module partitioning is unreasonable" should only come from real split signals from the partitioner, not from a summary model drawing conclusions when information is insufficient.
