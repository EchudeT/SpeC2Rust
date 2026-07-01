# Module Summary

This document is generated only from module partition results and parsed source facts; it does not turn "insufficient information" into "empty implementation" or "design mistake".

## 1. Module Responsibilities
- Observed focus: organized around functions with the `wordwrap` prefix
- Module category: `module_cluster`
- Directory scope: `src/parseopt`

## 2. Inputs and Outputs
- At this stage, do not speculate about runtime I/O; define interface boundaries by the observed function signatures and source files.
- File input boundary: src/parseopt/wordwrap.c
- Function count: 4

## 3. Core Interface List
- `wordwrap_putc` [src/parseopt/wordwrap.c:658-663]: `int wordwrap_putc (WORDWRAP_FILE wf, int c);`
- `wordwrap_para` [src/parseopt/wordwrap.c:668-675]: `int wordwrap_para (WORDWRAP_FILE wf);`
- `wordwrap_vprintf` [src/parseopt/wordwrap.c:680-731]: `int wordwrap_vprintf (WORDWRAP_FILE wf, char const *fmt, va_list ap);`
- `wordwrap_printf` [src/parseopt/wordwrap.c:737-747]: `int wordwrap_printf (WORDWRAP_FILE wf, char const *fmt, ...);`

## 4. Dependencies on Other Modules
- Internal call count: 1
- External call count: 4
- Cohesion score: 0.20
- Related headers: not recorded in the current module metadata.

## 5. Key Behaviors That Must Be Preserved
- At minimum, preserve the control flow and return conventions from the source code where these function definitions live; check the implementation body directly rather than inferring from the summary.
- Data structures defined in this module's files: `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`

## 6. Module Partition Signals
- This module was split out of parent module `module_src_parseopt`; cluster type: `file_local`.
- Actual reasons the parent module was split: 函数数过多(88)

## Conclusion
- If function definitions really exist in the source, they should not be described as "empty implementations"; this document is grounded in source locations and declaration excerpts.
- "Module partitioning is unreasonable" should only come from real split signals from the partitioner, not from a summary model drawing conclusions when information is insufficient.
