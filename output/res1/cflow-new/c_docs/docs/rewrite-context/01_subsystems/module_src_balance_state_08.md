# Module Summary

This document is generated only from module partition results and parsed source facts; it does not turn "insufficient information" into "empty implementation" or "design mistake".

## 1. Module Responsibilities
- Observed focus: organized around source files related to `parser`
- Module category: `module_cluster`
- Directory scope: `src`

## 2. Inputs and Outputs
- At this stage, do not speculate about runtime I/O; define interface boundaries by the observed function signatures and source files.
- File input boundary: src/parser.c
- Function count: 4

## 3. Core Interface List
- `push_balance_state` [src/parser.c:481-489]: `static void push_balance_state(struct balance_state **ptos, int idx, int level);`
- `pop_balance_state` [src/parser.c:491-499]: `static void pop_balance_state(struct balance_state **ptos, int *idx, int *level);`
- `free_balance_stack` [src/parser.c:501-507]: `static void free_balance_stack(struct balance_state **ptos);`
- `find_closing_paren` [src/parser.c:509-554]: `static int find_closing_paren(int open_tok, int level);`

## 4. Dependencies on Other Modules
- Internal call count: 4
- External call count: 3
- Cohesion score: 0.57
- Related headers: not recorded in the current module metadata.

## 5. Key Behaviors That Must Be Preserved
- At minimum, preserve the control flow and return conventions from the source code where these function definitions live; check the implementation body directly rather than inferring from the summary.
- Data structures defined in this module's files: `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`

## 6. Module Partition Signals
- This module was split out of parent module `module_src`; cluster type: `struct_based`.
- Actual reasons the parent module was split: 函数数过多(221); 职责不明确且目录范围较大

## Conclusion
- If function definitions really exist in the source, they should not be described as "empty implementations"; this document is grounded in source locations and declaration excerpts.
- "Module partitioning is unreasonable" should only come from real split signals from the partitioner, not from a summary model drawing conclusions when information is insufficient.
