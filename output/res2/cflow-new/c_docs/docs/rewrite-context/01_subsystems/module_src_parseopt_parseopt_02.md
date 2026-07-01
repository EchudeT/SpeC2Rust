# Module Summary

This document is generated only from module partition results and parsed source facts; it does not turn "insufficient information" into "empty implementation" or "design mistake".

## 1. Module Responsibilities
- Observed focus: organized around source files related to `parseopt`
- Module category: `module_cluster`
- Directory scope: `src/parseopt`

## 2. Inputs and Outputs
- At this stage, do not speculate about runtime I/O; define interface boundaries by the observed function signatures and source files.
- File input boundary: src/parseopt/parseopt.c
- Function count: 15

## 3. Core Interface List
- `option_find_short` [src/parseopt/parseopt.c:34-74]: `static struct optdef * option_find_short (struct parseopt *po, struct optdef **orig_opt, char **argptr, int *exparg);`
- `negmatch` [src/parseopt/parseopt.c:84-100]: `static enum neg_match negmatch (struct parseopt *po, struct optdef const *opt, char const *optstr, size_t optlen);`
- `option_find_long` [src/parseopt/parseopt.c:102-202]: `static struct optdef * option_find_long (struct parseopt *po, struct optdef **orig_opt, char **argptr, int *exparg);`
- `permute` [src/parseopt/parseopt.c:204-234]: `static void permute (struct parseopt *po);`
- `parseopt_lookahead` [src/parseopt/parseopt.c:236-244]: `char const * parseopt_lookahead (struct parseopt *po);`
- `parseopt_skip` [src/parseopt/parseopt.c:246-252]: `void parseopt_skip (struct parseopt *po);`
- `parseopt_next_internal` [src/parseopt/parseopt.c:254-412]: `static int parseopt_next_internal (struct parseopt *po, char **ret_arg);`
- `parseopt_next` [src/parseopt/parseopt.c:414-422]: `int parseopt_next (struct parseopt *po, char **ret_arg);`
- `parseopt_argv` [src/parseopt/parseopt.c:424-429]: `void parseopt_argv (struct parseopt *po, int *argc, char ***argv);`
- `parseopt_error` [src/parseopt/parseopt.c:431-445]: `void parseopt_error (struct parseopt *po, int pri, char const *fmt, ...);`
- `optidx_slot` [src/parseopt/parseopt.c:459-478]: `static int optidx_slot (struct parseopt *po, int n, struct optdef *opt);`
- `collect_optdef` [src/parseopt/parseopt.c:480-499]: `static size_t collect_optdef (struct parseopt *po, struct optdef *opt, size_t n);`
- `prepare_optdef` [src/parseopt/parseopt.c:506-555]: `static void prepare_optdef (struct parseopt *po, struct optdef *opt, int *scan_flags);`
- `set_help` [src/parseopt/parseopt.c:557-562]: `static int set_help (struct parseopt *po, struct optdef *opt, char *arg);`
- `set_usage` [src/parseopt/parseopt.c:564-569]: `static int set_usage (struct parseopt *po, struct optdef *opt, char *arg);`

## 4. Dependencies on Other Modules
- Internal call count: 4
- External call count: 10
- Cohesion score: 0.29
- Related headers: not recorded in the current module metadata.

## 5. Key Behaviors That Must Be Preserved
- At minimum, preserve the control flow and return conventions from the source code where these function definitions live; check the implementation body directly rather than inferring from the summary.
- Data structures defined in this module's files: `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`

## 6. Module Partition Signals
- This module was split out of parent module `module_src_parseopt`; cluster type: `struct_based`.
- Actual reasons the parent module was split: 函数数过多(88)

## Conclusion
- If function definitions really exist in the source, they should not be described as "empty implementations"; this document is grounded in source locations and declaration excerpts.
- "Module partitioning is unreasonable" should only come from real split signals from the partitioner, not from a summary model drawing conclusions when information is insufficient.
