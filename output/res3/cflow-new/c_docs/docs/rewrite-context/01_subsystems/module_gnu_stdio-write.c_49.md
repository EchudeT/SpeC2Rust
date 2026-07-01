# Module Summary

This document is generated only from module partition results and parsed source facts; it does not turn "insufficient information" into "empty implementation" or "design mistake".

## 1. Module Responsibilities
- Observed focus: organized around source files related to `stdio-write`
- Module category: `module_cluster`
- Directory scope: `gnu`

## 2. Inputs and Outputs
- At this stage, do not speculate about runtime I/O; define interface boundaries by the observed function signatures and source files.
- File input boundary: gnu/stdio-write.c
- Function count: 9

## 3. Core Interface List
- `printf` [gnu/stdio-write.c:126-137]: `int printf (const char *format, ...);`
- `fprintf` [gnu/stdio-write.c:141-152]: `int fprintf (FILE *stream, const char *format, ...);`
- `vprintf` [gnu/stdio-write.c:156-160]: `int vprintf (const char *format, va_list args);`
- `vfprintf` [gnu/stdio-write.c:164-169]: `int vfprintf (FILE *stream, const char *format, va_list args) #undef vfprintf;`
- `putchar` [gnu/stdio-write.c:172-176]: `int putchar (int c);`
- `fputc` [gnu/stdio-write.c:178-183]: `int fputc (int c, FILE *stream) #undef fputc;`
- `fputs` [gnu/stdio-write.c:185-190]: `int fputs (const char *string, FILE *stream) #undef fputs;`
- `puts` [gnu/stdio-write.c:192-198]: `int puts (const char *string) #undef puts;`
- `fwrite` [gnu/stdio-write.c:200-205]: `size_t fwrite (const void *ptr, size_t s, size_t n, FILE *stream) #undef fwrite;`

## 4. Dependencies on Other Modules
- Internal call count: 9
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
