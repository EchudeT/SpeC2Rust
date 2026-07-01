# Module Summary

This document is generated only from module partition results and parsed source facts; it does not turn "insufficient information" into "empty implementation" or "design mistake".

## 1. Module Responsibilities
- Observed focus: organized around source files related to `main`
- Module category: `module_cluster`
- Directory scope: `src`

## 2. Inputs and Outputs
- At this stage, do not speculate about runtime I/O; define interface boundaries by the observed function signatures and source files.
- File input boundary: src/main.c
- Function count: 15

## 3. Core Interface List
- `optset_include_classes` [src/main.c:369-401]: `static int optset_include_classes(struct parseopt *po, struct optdef *opt, char *arg);`
- `optset_output_driver` [src/main.c:403-412]: `static int optset_output_driver(struct parseopt *po, struct optdef *opt, char *arg);`
- `optset_xref` [src/main.c:414-420]: `static int optset_xref(struct parseopt *po, struct optdef *opt, char *arg);`
- `optset_symbol` [src/main.c:422-427]: `static int optset_symbol(struct parseopt *po, struct optdef *opt, char *arg);`
- `optset_preproc_option` [src/main.c:429-444]: `static int optset_preproc_option(struct parseopt *po, struct optdef *opt, char *arg);`
- `optset_preprocess` [src/main.c:446-452]: `static int optset_preprocess(struct parseopt *po, struct optdef *opt, char *arg);`
- `optset_level_indent` [src/main.c:454-459]: `static int optset_level_indent(struct parseopt *po, struct optdef *opt, char *arg);`
- `optset_main_symbol` [src/main.c:461-466]: `static int optset_main_symbol(struct parseopt *po, struct optdef *opt, char *arg);`
- `optset_clear_main_symbol` [src/main.c:468-474]: `static int optset_clear_main_symbol(struct parseopt *po, struct optdef *opt, char *arg);`
- `optset_install_target` [src/main.c:476-481]: `static int optset_install_target(struct parseopt *po, struct optdef *opt, char *arg);`
- `optset_int_1` [src/main.c:483-490]: `static int optset_int_1(struct parseopt *po, struct optdef *opt, char *arg);`
- `optset_prepend_path` [src/main.c:492-504]: `static int optset_prepend_path(struct parseopt *po, struct optdef *opt, char *arg);`
- `version_hook` [src/main.c:900-911]: `void version_hook(WORDWRAP_FILE wf, struct parseopt *po);`
- `help_hook` [src/main.c:913-920]: `static void help_hook(WORDWRAP_FILE wf, struct parseopt *po);`
- `po_env_error` [src/main.c:938-955]: `static void po_env_error (struct parseopt *po, int pri, char const *fmt, ...);`

## 4. Dependencies on Other Modules
- Internal call count: 0
- External call count: 18
- Cohesion score: 0.00
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
