# Module Summary

This document is generated only from module partition results and parsed source facts; it does not turn "insufficient information" into "empty implementation" or "design mistake".

## 1. Module Responsibilities
- Observed focus: organized around source files related to `d`
- Module category: `module_cluster`
- Directory scope: `doc`

## 2. Inputs and Outputs
- At this stage, do not speculate about runtime I/O; define interface boundaries by the observed function signatures and source files.
- File input boundary: doc/d.c
- Function count: 3

## 3. Core Interface List
- `isdir` [doc/d.c:11-22]: `static int isdir (char *name);`
- `ignorent` [doc/d.c:27-35]: `int ignorent (char *name);`
- `printdir` [doc/d.c:41-82]: `void printdir (int level, char *name);`

## 4. Dependencies on Other Modules
- Internal call count: 3
- External call count: 2
- Cohesion score: 0.60
- Related headers: not recorded in the current module metadata.

## 5. Key Behaviors That Must Be Preserved
- At minimum, preserve the control flow and return conventions from the source code where these function definitions live; check the implementation body directly rather than inferring from the summary.
- Data structures defined in this module's files: `anonymous`, `anonymous`

## 6. Module Partition Signals
- This module was split out of parent module `module_doc`; cluster type: `file_local`.
- Actual reasons the parent module was split: 内聚度较低(0.28)

## Conclusion
- If function definitions really exist in the source, they should not be described as "empty implementations"; this document is grounded in source locations and declaration excerpts.
- "Module partitioning is unreasonable" should only come from real split signals from the partitioner, not from a summary model drawing conclusions when information is insufficient.
