# Module Summary

This document is generated only from module partition results and parsed source facts; it does not turn "insufficient information" into "empty implementation" or "design mistake".

## 1. Module Responsibilities
- Observed focus: organized around source files related to `stdio-read`
- Module category: `module_cluster`
- Directory scope: `gnu`

## 2. Inputs and Outputs
- At this stage, do not speculate about runtime I/O; define interface boundaries by the observed function signatures and source files.
- File input boundary: gnu/stdio-read.c
- Function count: 8

## 3. Core Interface List
- `scanf` [gnu/stdio-read.c:88-99]: `int scanf (const char *format, ...);`
- `fscanf` [gnu/stdio-read.c:105-116]: `int fscanf (FILE *stream, const char *format, ...);`
- `vscanf` [gnu/stdio-read.c:122-126]: `int vscanf (const char *format, va_list args);`
- `vfscanf` [gnu/stdio-read.c:132-137]: `int vfscanf (FILE *stream, const char *format, va_list args) #undef vfscanf;`
- `getchar` [gnu/stdio-read.c:140-144]: `int getchar (void);`
- `fgetc` [gnu/stdio-read.c:146-151]: `int fgetc (FILE *stream) #undef fgetc;`
- `fgets` [gnu/stdio-read.c:153-158]: `char * fgets (char *s, int n, FILE *stream) #undef fgets;`
- `fread` [gnu/stdio-read.c:162-167]: `size_t fread (void *ptr, size_t s, size_t n, FILE *stream) #undef fread;`

## 4. Dependencies on Other Modules
- Internal call count: 8
- External call count: 0
- Cohesion score: 1.00
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
