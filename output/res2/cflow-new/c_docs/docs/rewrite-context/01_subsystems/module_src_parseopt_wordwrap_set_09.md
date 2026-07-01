# Module Summary

This document is generated only from module partition results and parsed source facts; it does not turn "insufficient information" into "empty implementation" or "design mistake".

## 1. Module Responsibilities
- Observed focus: organized around functions with the `wordwrap_set` prefix
- Module category: `module_cluster`
- Directory scope: `src/parseopt`

## 2. Inputs and Outputs
- At this stage, do not speculate about runtime I/O; define interface boundaries by the observed function signatures and source files.
- File input boundary: src/parseopt/wordwrap.c
- Function count: 2

## 3. Core Interface List
- `wordwrap_set_left_margin` [src/parseopt/wordwrap.c:456-496]: `int wordwrap_set_left_margin (WORDWRAP_FILE wf, unsigned left);`
- `wordwrap_set_right_margin` [src/parseopt/wordwrap.c:520-558]: `int wordwrap_set_right_margin (WORDWRAP_FILE wf, unsigned right);`

## 4. Dependencies on Other Modules
- Internal call count: 0
- External call count: 7
- Cohesion score: 0.00
- Related headers: not recorded in the current module metadata.

## 5. Key Behaviors That Must Be Preserved
- At minimum, preserve the control flow and return conventions from the source code where these function definitions live; check the implementation body directly rather than inferring from the summary.
- Data structures defined in this module's files: `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`

## 6. Module Partition Signals
- This module was split out of parent module `module_src_parseopt`; cluster type: `prefix_based`.
- Actual reasons the parent module was split: 函数数过多(88)

## Conclusion
- If function definitions really exist in the source, they should not be described as "empty implementations"; this document is grounded in source locations and declaration excerpts.
- "Module partitioning is unreasonable" should only come from real split signals from the partitioner, not from a summary model drawing conclusions when information is insufficient.
