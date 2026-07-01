# Module Summary

This document is generated only from module partition results and parsed source facts; it does not turn "insufficient information" into "empty implementation" or "design mistake".

## 1. Module Responsibilities
- Observed focus: organized around source files related to `xmalloc`
- Module category: `module_cluster`
- Directory scope: `gnu`

## 2. Inputs and Outputs
- At this stage, do not speculate about runtime I/O; define interface boundaries by the observed function signatures and source files.
- File input boundary: gnu/xmalloc.c
- Function count: 15

## 3. Core Interface List
- `xmalloc` [gnu/xmalloc.c:44-48]: `void * xmalloc (size_t s);`
- `ximalloc` [gnu/xmalloc.c:50-54]: `void * ximalloc (idx_t s);`
- `xcharalloc` [gnu/xmalloc.c:56-60]: `char * xcharalloc (size_t n);`
- `xrealloc` [gnu/xmalloc.c:65-72]: `void * xrealloc (void *p, size_t s);`
- `xirealloc` [gnu/xmalloc.c:74-78]: `void * xirealloc (void *p, idx_t s);`
- `xreallocarray` [gnu/xmalloc.c:83-90]: `void * xreallocarray (void *p, size_t n, size_t s);`
- `xireallocarray` [gnu/xmalloc.c:92-96]: `void * xireallocarray (void *p, idx_t n, idx_t s);`
- `xnmalloc` [gnu/xmalloc.c:101-105]: `void * xnmalloc (size_t n, size_t s);`
- `xinmalloc` [gnu/xmalloc.c:107-111]: `void * xinmalloc (idx_t n, idx_t s);`
- `x2realloc` [gnu/xmalloc.c:119-123]: `void * x2realloc (void *p, size_t *ps);`
- `x2nrealloc` [gnu/xmalloc.c:179-208]: `void * x2nrealloc (void *p, size_t *pn, size_t s);`
- `xpalloc` [gnu/xmalloc.c:226-276]: `void * xpalloc (void *pa, idx_t *pn, idx_t n_incr_min, ptrdiff_t n_max, idx_t s);`
- `xzalloc` [gnu/xmalloc.c:282-286]: `void * xzalloc (size_t s);`
- `xizalloc` [gnu/xmalloc.c:288-292]: `void * xizalloc (idx_t s);`
- `xcalloc` [gnu/xmalloc.c:297-301]: `void * xcalloc (size_t n, size_t s);`

## 4. Dependencies on Other Modules
- Internal call count: 10
- External call count: 9
- Cohesion score: 0.53
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
