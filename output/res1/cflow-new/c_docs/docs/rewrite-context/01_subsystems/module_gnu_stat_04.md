# Module Summary

This document is generated only from module partition results and parsed source facts; it does not turn "insufficient information" into "empty implementation" or "design mistake".

## 1. Module Responsibilities
- Observed focus: At this point, only the file and symbol distribution indicates that this is a local source slice; its responsibilities still need to be confirmed from the source.
- Module category: `module_cluster`
- Directory scope: `gnu`

## 2. Inputs and Outputs
- At this stage, do not speculate about runtime I/O; define interface boundaries by the observed function signatures and source files.
- File input boundary: gnu/dup2.c, gnu/fcntl.c, gnu/fstat.c, gnu/open.c, gnu/stat-w32.c, gnu/stat.c
- Function count: 7

## 3. Core Interface List
- `klibc_dup2` [gnu/dup2.c:140-156]: `static int klibc_dup2 (int fd, int desired_fd);`
- `klibc_fcntl` [gnu/fcntl.c:552-629]: `static int klibc_fcntl (int fd, int action, /* arg */...);`
- `orig_fstat` [gnu/fstat.c:36-40]: `static int orig_fstat (int fd, struct stat *buf);`
- `rpl_fstat` [gnu/fstat.c:69-96]: `int rpl_fstat (int fd, struct stat *buf);`
- `open` [gnu/open.c:64-215]: `int open (const char *filename, int flags, ...);`
- `_gl_fstat_by_handle` [gnu/stat-w32.c:162-454]: `int _gl_fstat_by_handle (HANDLE h, const char *path, struct stat *buf);`
- `orig_stat` [gnu/stat.c:38-42]: `static int orig_stat (const char *filename, struct stat *buf);`

## 4. Dependencies on Other Modules
- Internal call count: 3
- External call count: 8
- Cohesion score: 0.27
- Related headers: not recorded in the current module metadata.

## 5. Key Behaviors That Must Be Preserved
- At minimum, preserve the control flow and return conventions from the source code where these function definitions live; check the implementation body directly rather than inferring from the summary.
- Data structures defined in this module's files: `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`

## 6. Module Partition Signals
- This module was split out of parent module `module_gnu`; cluster type: `struct_based`.
- Actual reasons the parent module was split: 文件数过多(45); 函数数过多(174); 职责不明确且目录范围较大

## Conclusion
- If function definitions really exist in the source, they should not be described as "empty implementations"; this document is grounded in source locations and declaration excerpts.
- "Module partitioning is unreasonable" should only come from real split signals from the partitioner, not from a summary model drawing conclusions when information is insufficient.
