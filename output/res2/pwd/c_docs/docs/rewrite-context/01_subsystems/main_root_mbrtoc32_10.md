# Module Summary

This document is generated only from module partition results and parsed source facts; it does not turn "insufficient information" into "empty implementation" or "design mistake".

## 1. Module Responsibilities
- Observed focus: organized around functions with the `mbrtoc32` prefix
- Module category: `main_cluster`
- Directory scope: `root`

## 2. Inputs and Outputs
- At this stage, do not speculate about runtime I/O; define interface boundaries by the observed function signatures and source files.
- File input boundary: mbrtoc32.c
- Function count: 2

## 3. Core Interface List
- `mbrtoc32` [mbrtoc32.c:76-81]: `size_t mbrtoc32 (char32_t *pwc, const char *s, size_t n, mbstate_t *ps);`
- `mbrtoc32` [mbrtoc32.c:99-286]: `size_t mbrtoc32 (char32_t *pwc, const char *s, size_t n, mbstate_t *ps) # undef mbrtoc32;`

## 4. Dependencies on Other Modules
- Internal call count: 2
- External call count: 6
- Cohesion score: 0.25
- Related headers: not recorded in the current module metadata.

## 5. Key Behaviors That Must Be Preserved
- At minimum, preserve the control flow and return conventions from the source code where these function definitions live; check the implementation body directly rather than inferring from the summary.
- No struct definitions were parsed for the current module.

## 6. Module Partition Signals
- This module was split out of parent module `main_root`; cluster type: `prefix_based`.
- Actual reasons the parent module was split: 文件数过多(28); 函数数过多(99)

## Conclusion
- If function definitions really exist in the source, they should not be described as "empty implementations"; this document is grounded in source locations and declaration excerpts.
- "Module partitioning is unreasonable" should only come from real split signals from the partitioner, not from a summary model drawing conclusions when information is insufficient.
