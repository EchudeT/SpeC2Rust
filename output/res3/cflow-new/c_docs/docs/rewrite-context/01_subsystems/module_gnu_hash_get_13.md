# Module Summary

This document is generated only from module partition results and parsed source facts; it does not turn "insufficient information" into "empty implementation" or "design mistake".

## 1. Module Responsibilities
- Observed focus: organized around functions with the `hash_get_n` prefix
- Module category: `module_cluster`
- Directory scope: `gnu`

## 2. Inputs and Outputs
- At this stage, do not speculate about runtime I/O; define interface boundaries by the observed function signatures and source files.
- File input boundary: gnu/hash.c
- Function count: 3

## 3. Core Interface List
- `hash_get_n_buckets` [gnu/hash.c:144-148]: `size_t hash_get_n_buckets (const Hash_table *table);`
- `hash_get_n_buckets_used` [gnu/hash.c:150-154]: `size_t hash_get_n_buckets_used (const Hash_table *table);`
- `hash_get_n_entries` [gnu/hash.c:156-160]: `size_t hash_get_n_entries (const Hash_table *table);`

## 4. Dependencies on Other Modules
- Internal call count: 0
- External call count: 0
- Cohesion score: 1.00
- Related headers: not recorded in the current module metadata.

## 5. Key Behaviors That Must Be Preserved
- At minimum, preserve the control flow and return conventions from the source code where these function definitions live; check the implementation body directly rather than inferring from the summary.
- Data structures defined in this module's files: `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`

## 6. Module Partition Signals
- This module was split out of parent module `module_gnu`; cluster type: `prefix_based`.
- Actual reasons the parent module was split: 文件数过多(45); 函数数过多(174); 职责不明确且目录范围较大

## Conclusion
- If function definitions really exist in the source, they should not be described as "empty implementations"; this document is grounded in source locations and declaration excerpts.
- "Module partitioning is unreasonable" should only come from real split signals from the partitioner, not from a summary model drawing conclusions when information is insufficient.
