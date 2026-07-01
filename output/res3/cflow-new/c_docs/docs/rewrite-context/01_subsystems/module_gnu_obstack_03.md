# Module Summary

This document is generated only from module partition results and parsed source facts; it does not turn "insufficient information" into "empty implementation" or "design mistake".

## 1. Module Responsibilities
- Observed focus: organized around source files related to `obstack`
- Module category: `module_cluster`
- Directory scope: `gnu`

## 2. Inputs and Outputs
- At this stage, do not speculate about runtime I/O; define interface boundaries by the observed function signatures and source files.
- File input boundary: gnu/obstack.c
- Function count: 9

## 3. Core Interface List
- `call_chunkfun` [gnu/obstack.c:64-71]: `static void * call_chunkfun (struct obstack *h, size_t size);`
- `call_freefun` [gnu/obstack.c:73-80]: `static void call_freefun (struct obstack *h, void *old_chunk);`
- `_obstack_begin_worker` [gnu/obstack.c:89-128]: `static int _obstack_begin_worker (struct obstack *h, _OBSTACK_SIZE_T size, _OBSTACK_SIZE_T alignment);`
- `_obstack_begin` [gnu/obstack.c:130-140]: `int _obstack_begin (struct obstack *h, _OBSTACK_SIZE_T size, _OBSTACK_SIZE_T alignment, void *(*chunkfun) (size_t), void (*freefun) (void *));`
- `_obstack_begin_1` [gnu/obstack.c:142-154]: `int _obstack_begin_1 (struct obstack *h, _OBSTACK_SIZE_T size, _OBSTACK_SIZE_T alignment, void *(*chunkfun) (void *, size_t), void (*freefun) (void *, void *), void *arg);`
- `_obstack_newchunk` [gnu/obstack.c:162-211]: `void _obstack_newchunk (struct obstack *h, _OBSTACK_SIZE_T length);`
- `_obstack_allocated_p` [gnu/obstack.c:221-237]: `int _obstack_allocated_p (struct obstack *h, void *obj);`
- `_obstack_free` [gnu/obstack.c:242-270]: `void _obstack_free (struct obstack *h, void *obj);`
- `_obstack_memory_used` [gnu/obstack.c:272-283]: `_OBSTACK_SIZE_T _obstack_memory_used (struct obstack *h);`

## 4. Dependencies on Other Modules
- Internal call count: 6
- External call count: 0
- Cohesion score: 1.00
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
