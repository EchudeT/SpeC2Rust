# Module Summary

This document is generated only from module partition results and parsed source facts; it does not turn "insufficient information" into "empty implementation" or "design mistake".

## 1. Module Responsibilities
- Observed focus: organized around source files related to `multi`, `recursion`, `simple`
- Module category: `module`
- Directory scope: `test`

## 2. Inputs and Outputs
- At this stage, do not speculate about runtime I/O; define interface boundaries by the observed function signatures and source files.
- File input boundary: test/multi.c, test/recursion.c, test/simple.c
- Function count: 11

## 3. Core Interface List
- `helper` [test/multi.c:4-6]: `(void);`
- `twice` [test/multi.c:8-10]: `nt x);`
- `run` [test/multi.c:12-16]: `helper(); int r = twice(42); printf("%d\n", r); };`
- `fib` [test/recursion.c:2-6]: `n);`
- `fact` [test/recursion.c:8-12]: `n);`
- `main` [test/recursion.c:14-16]: `d);`
- `add` [test/simple.c:2-4]: `b);`
- `mul` [test/simple.c:6-8]: `y);`
- `orphan` [test/simple.c:10-12]: `return 42; } int compute(int;`
- `compute` [test/simple.c:14-19]: `{;`
- `main` [test/simple.c:21-23]: `char **argv);`

## 4. Dependencies on Other Modules
- Internal call count: 12
- External call count: 48
- Cohesion score: 0.20
- Related headers: not recorded in the current module metadata.

## 5. Key Behaviors That Must Be Preserved
- At minimum, preserve the control flow and return conventions from the source code where these function definitions live; check the implementation body directly rather than inferring from the summary.
- No struct definitions were parsed for the current module.

## 6. Module Partition Signals
- The current module is already a consumable unit after partitioner convergence, with no additional split signals.

## Conclusion
- If function definitions really exist in the source, they should not be described as "empty implementations"; this document is grounded in source locations and declaration excerpts.
- "Module partitioning is unreasonable" should only come from real split signals from the partitioner, not from a summary model drawing conclusions when information is insufficient.
