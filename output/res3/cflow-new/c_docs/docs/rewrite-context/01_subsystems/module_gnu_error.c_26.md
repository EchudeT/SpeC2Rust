# Module Summary

This document is generated only from module partition results and parsed source facts; it does not turn "insufficient information" into "empty implementation" or "design mistake".

## 1. Module Responsibilities
- Observed focus: organized around source files related to `error`
- Module category: `module_cluster`
- Directory scope: `gnu`

## 2. Inputs and Outputs
- At this stage, do not speculate about runtime I/O; define interface boundaries by the observed function signatures and source files.
- File input boundary: gnu/error.c
- Function count: 5

## 3. Core Interface List
- `is_open` [gnu/error.c:132-147]: `static int is_open (int fd);`
- `flush_stdout` [gnu/error.c:150-173]: `static void flush_stdout (void);`
- `print_errno_message` [gnu/error.c:175-204]: `static void print_errno_message (int errnum);`
- `error` [gnu/error.c:295-333]: `void error (int status, int errnum, const char *message, ...);`
- `error_at_line` [gnu/error.c:339-404]: `void error_at_line (int status, int errnum, const char *file_name, unsigned int line_number, const char *message, ...);`

## 4. Dependencies on Other Modules
- Internal call count: 3
- External call count: 4
- Cohesion score: 0.43
- Related headers: not recorded in the current module metadata.

## 5. Key Behaviors That Must Be Preserved
- At minimum, preserve the control flow and return conventions from the source code where these function definitions live; check the implementation body directly rather than inferring from the summary.
- No struct definitions were parsed for the current module.

## 6. Module Partition Signals
- This module was split out of parent module `module_gnu`; cluster type: `file_local`.
- Actual reasons the parent module was split: 文件数过多(45); 函数数过多(174); 职责不明确且目录范围较大

## Conclusion
- If function definitions really exist in the source, they should not be described as "empty implementations"; this document is grounded in source locations and declaration excerpts.
- "Module partitioning is unreasonable" should only come from real split signals from the partitioner, not from a summary model drawing conclusions when information is insufficient.
