# Module Summary

This document is generated only from module partition results and parsed source facts; it does not turn "insufficient information" into "empty implementation" or "design mistake".

## 1. Module Responsibilities
- Observed focus: organized around source files related to `c4`, `hello`
- Module category: `main`
- Directory scope: `root`

## 2. Inputs and Outputs
- At this stage, do not speculate about runtime I/O; define interface boundaries by the observed function signatures and source files.
- File input boundary: c4.c, hello.c
- Function count: 10

## 3. Core Interface List
- `next` [c4.c:48-132]: `void next();`
- `expr` [c4.c:134-282]: `void expr(int lev);`
- `stmt` [c4.c:284-331]: `void stmt();`
- `main` [c4.c:333-528]: `int main(int argc, char **argv);`
- `main` [hello.c:3-7]: `int main();`
- `next` [test/c4.c:48-132]: `void next();`
- `expr` [test/c4.c:134-282]: `void expr(int lev);`
- `stmt` [test/c4.c:284-331]: `void stmt();`
- `main` [test/c4.c:333-528]: `int main(int argc, char **argv);`
- `main` [test/hello.c:3-7]: `int main();`

## 4. Dependencies on Other Modules
- Internal call count: 18
- External call count: 0
- Cohesion score: 1.00
- Related headers: not recorded in the current module metadata.

## 5. Key Behaviors That Must Be Preserved
- At minimum, preserve the control flow and return conventions from the source code where these function definitions live; check the implementation body directly rather than inferring from the summary.
- No struct definitions were parsed for the current module.

## 6. Module Partition Signals
- The current module is already a consumable unit after partitioner convergence, with no additional split signals.

## Conclusion
- If function definitions really exist in the source, they should not be described as "empty implementations"; this document is grounded in source locations and declaration excerpts.
- "Module partitioning is unreasonable" should only come from real split signals from the partitioner, not from a summary model drawing conclusions when information is insufficient.
