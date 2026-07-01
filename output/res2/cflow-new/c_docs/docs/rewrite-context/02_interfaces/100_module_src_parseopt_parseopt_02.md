# Interface Facts: module_src_parseopt_parseopt_02

This document is intended for the later Rust repository-level rewrite and keeps only the interface facts directly observed in the current source-analysis stage.
Header files, macros, error codes, and configuration items that do not appear in the current parse results will not be added or assumed.

## Module Scope
- Module category: `module_cluster`
- Directory: `src/parseopt`
- File list: src/parseopt/parseopt.c
- Candidate header files: gnu/assert.h, gnu/stdio.h, gnu/stdlib.h, gnu/string.h, src/parseopt/parseconf.h, src/parseopt/parseopt.h, src/parseopt/wordwrap.h
- Exported functions observed: 15
- Struct definitions observed: 144
- Type names referenced but not defined locally: 2
- Macros observed in related files: 20
- Global variables observed: 0

## Header Evidence
- `gnu/assert.h` [gnu/assert.h]
- `gnu/stdio.h` [gnu/stdio.h]
- `gnu/stdlib.h` [gnu/stdlib.h]
- `gnu/string.h` [gnu/string.h]
- `src/parseopt/parseconf.h` [src/parseopt/parseconf.h]
- `src/parseopt/parseopt.h` [src/parseopt/parseopt.h]
- `src/parseopt/wordwrap.h` [src/parseopt/wordwrap.h]

## Functions
### `option_find_short`
- Definition location: [src/parseopt/parseopt.c:34-74]
- Source file: `src/parseopt/parseopt.c`
- Observed declaration: `static struct optdef * option_find_short (struct parseopt *po, struct optdef **orig_opt, char **argptr, int *exparg);`
- Approximate function body length: 41 lines
### `negmatch`
- Definition location: [src/parseopt/parseopt.c:84-100]
- Source file: `src/parseopt/parseopt.c`
- Observed declaration: `static enum neg_match negmatch (struct parseopt *po, struct optdef const *opt, char const *optstr, size_t optlen);`
- Approximate function body length: 17 lines
### `option_find_long`
- Definition location: [src/parseopt/parseopt.c:102-202]
- Source file: `src/parseopt/parseopt.c`
- Observed declaration: `static struct optdef * option_find_long (struct parseopt *po, struct optdef **orig_opt, char **argptr, int *exparg);`
- Approximate function body length: 101 lines
### `permute`
- Definition location: [src/parseopt/parseopt.c:204-234]
- Source file: `src/parseopt/parseopt.c`
- Observed declaration: `static void permute (struct parseopt *po);`
- Approximate function body length: 31 lines
### `parseopt_lookahead`
- Definition location: [src/parseopt/parseopt.c:236-244]
- Source file: `src/parseopt/parseopt.c`
- Observed declaration: `char const * parseopt_lookahead (struct parseopt *po);`
- Approximate function body length: 9 lines
### `parseopt_skip`
- Definition location: [src/parseopt/parseopt.c:246-252]
- Source file: `src/parseopt/parseopt.c`
- Observed declaration: `void parseopt_skip (struct parseopt *po);`
- Approximate function body length: 7 lines
### `parseopt_next_internal`
- Definition location: [src/parseopt/parseopt.c:254-412]
- Source file: `src/parseopt/parseopt.c`
- Observed declaration: `static int parseopt_next_internal (struct parseopt *po, char **ret_arg);`
- Approximate function body length: 159 lines
### `parseopt_next`
- Definition location: [src/parseopt/parseopt.c:414-422]
- Source file: `src/parseopt/parseopt.c`
- Observed declaration: `int parseopt_next (struct parseopt *po, char **ret_arg);`
- Approximate function body length: 9 lines
### `parseopt_argv`
- Definition location: [src/parseopt/parseopt.c:424-429]
- Source file: `src/parseopt/parseopt.c`
- Observed declaration: `void parseopt_argv (struct parseopt *po, int *argc, char ***argv);`
- Approximate function body length: 6 lines
### `parseopt_error`
- Definition location: [src/parseopt/parseopt.c:431-445]
- Source file: `src/parseopt/parseopt.c`
- Observed declaration: `void parseopt_error (struct parseopt *po, int pri, char const *fmt, ...);`
- Approximate function body length: 15 lines
### `optidx_slot`
- Definition location: [src/parseopt/parseopt.c:459-478]
- Source file: `src/parseopt/parseopt.c`
- Observed declaration: `static int optidx_slot (struct parseopt *po, int n, struct optdef *opt);`
- Approximate function body length: 20 lines
### `collect_optdef`
- Definition location: [src/parseopt/parseopt.c:480-499]
- Source file: `src/parseopt/parseopt.c`
- Observed declaration: `static size_t collect_optdef (struct parseopt *po, struct optdef *opt, size_t n);`
- Approximate function body length: 20 lines
### `prepare_optdef`
- Definition location: [src/parseopt/parseopt.c:506-555]
- Source file: `src/parseopt/parseopt.c`
- Observed declaration: `static void prepare_optdef (struct parseopt *po, struct optdef *opt, int *scan_flags);`
- Approximate function body length: 50 lines
### `set_help`
- Definition location: [src/parseopt/parseopt.c:557-562]
- Source file: `src/parseopt/parseopt.c`
- Observed declaration: `static int set_help (struct parseopt *po, struct optdef *opt, char *arg);`
- Approximate function body length: 6 lines
### `set_usage`
- Definition location: [src/parseopt/parseopt.c:564-569]
- Source file: `src/parseopt/parseopt.c`
- Observed declaration: `static int set_usage (struct parseopt *po, struct optdef *opt, char *arg);`
- Approximate function body length: 6 lines

## Structs and Types
### `anonymous`
- Definition location: [gnu/stdio.h:1599]
- Source file: `gnu/stdio.h`
- Observed declaration prefix: `struct obstack`
### `anonymous`
- Definition location: [gnu/stdio.h:1610]
- Source file: `gnu/stdio.h`
- Observed declaration prefix: `struct obstack`
### `anonymous`
- Definition location: [gnu/stdio.h:1614]
- Source file: `gnu/stdio.h`
- Observed declaration prefix: `struct obstack`
### `anonymous`
- Definition location: [gnu/stdio.h:1618]
- Source file: `gnu/stdio.h`
- Observed declaration prefix: `struct obstack`
### `anonymous`
- Definition location: [gnu/stdio.h:1623]
- Source file: `gnu/stdio.h`
- Observed declaration prefix: `struct obstack`
### `anonymous`
- Definition location: [gnu/stdio.h:1631]
- Source file: `gnu/stdio.h`
- Observed declaration prefix: `struct obstack`
### `anonymous`
- Definition location: [gnu/stdio.h:1635]
- Source file: `gnu/stdio.h`
- Observed declaration prefix: `struct obstack`
### `anonymous`
- Definition location: [gnu/stdio.h:1639]
- Source file: `gnu/stdio.h`
- Observed declaration prefix: `struct obstack`
### `anonymous`
- Definition location: [gnu/stdio.h:1644]
- Source file: `gnu/stdio.h`
- Observed declaration prefix: `struct obstack`
### `anonymous`
- Definition location: [gnu/stdlib.h:86-95]
- Source file: `gnu/stdlib.h`
- Observed declaration prefix: `struct random_data`
### `anonymous`
- Definition location: [gnu/stdlib.h:1843]
- Source file: `gnu/stdlib.h`
- Observed declaration prefix: `struct random_data`
### `anonymous`
- Definition location: [gnu/stdlib.h:1845]
- Source file: `gnu/stdlib.h`
- Observed declaration prefix: `struct random_data`
### `anonymous`
- Definition location: [gnu/stdlib.h:1848]
- Source file: `gnu/stdlib.h`
- Observed declaration prefix: `struct random_data`
### `anonymous`
- Definition location: [gnu/stdlib.h:1851]
- Source file: `gnu/stdlib.h`
- Observed declaration prefix: `struct random_data`
### `anonymous`
- Definition location: [src/parseopt/parseopt.c:28]
- Source file: `src/parseopt/parseopt.c`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/parseopt.c:28]
- Source file: `src/parseopt/parseopt.c`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/parseopt.c:34]
- Source file: `src/parseopt/parseopt.c`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/parseopt.c:35]
- Source file: `src/parseopt/parseopt.c`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/parseopt.c:35]
- Source file: `src/parseopt/parseopt.c`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/parseopt.c:42]
- Source file: `src/parseopt/parseopt.c`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/parseopt.c:85]
- Source file: `src/parseopt/parseopt.c`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/parseopt.c:85]
- Source file: `src/parseopt/parseopt.c`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/parseopt.c:102]
- Source file: `src/parseopt/parseopt.c`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/parseopt.c:103]
- Source file: `src/parseopt/parseopt.c`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/parseopt.c:103]
- Source file: `src/parseopt/parseopt.c`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/parseopt.c:109]
- Source file: `src/parseopt/parseopt.c`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/parseopt.c:114]
- Source file: `src/parseopt/parseopt.c`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/parseopt.c:205]
- Source file: `src/parseopt/parseopt.c`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/parseopt.c:237]
- Source file: `src/parseopt/parseopt.c`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/parseopt.c:247]
- Source file: `src/parseopt/parseopt.c`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/parseopt.c:255]
- Source file: `src/parseopt/parseopt.c`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/parseopt.c:317]
- Source file: `src/parseopt/parseopt.c`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/parseopt.c:415]
- Source file: `src/parseopt/parseopt.c`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/parseopt.c:425]
- Source file: `src/parseopt/parseopt.c`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/parseopt.c:432]
- Source file: `src/parseopt/parseopt.c`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/parseopt.c:448]
- Source file: `src/parseopt/parseopt.c`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/parseopt.c:460]
- Source file: `src/parseopt/parseopt.c`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/parseopt.c:460]
- Source file: `src/parseopt/parseopt.c`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/parseopt.c:481]
- Source file: `src/parseopt/parseopt.c`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/parseopt.c:481]
- Source file: `src/parseopt/parseopt.c`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/parseopt.c:507]
- Source file: `src/parseopt/parseopt.c`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/parseopt.c:507]
- Source file: `src/parseopt/parseopt.c`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/parseopt.c:558]
- Source file: `src/parseopt/parseopt.c`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/parseopt.c:558]
- Source file: `src/parseopt/parseopt.c`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/parseopt.c:565]
- Source file: `src/parseopt/parseopt.c`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/parseopt.c:565]
- Source file: `src/parseopt/parseopt.c`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/parseopt.c:572]
- Source file: `src/parseopt/parseopt.c`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/parseopt.c:572]
- Source file: `src/parseopt/parseopt.c`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/parseopt.c:578]
- Source file: `src/parseopt/parseopt.c`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/parseopt.c:601]
- Source file: `src/parseopt/parseopt.c`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/parseopt.c:610]
- Source file: `src/parseopt/parseopt.c`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/parseopt.c:623]
- Source file: `src/parseopt/parseopt.c`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/parseopt.c:624]
- Source file: `src/parseopt/parseopt.c`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/parseopt.c:632]
- Source file: `src/parseopt/parseopt.c`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/parseopt.c:635]
- Source file: `src/parseopt/parseopt.c`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/parseopt.c:695]
- Source file: `src/parseopt/parseopt.c`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/parseopt.c:730]
- Source file: `src/parseopt/parseopt.c`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/parseopt.c:742]
- Source file: `src/parseopt/parseopt.c`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/parseopt.c:758]
- Source file: `src/parseopt/parseopt.c`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/parseopt.c:783]
- Source file: `src/parseopt/parseopt.c`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/parseopt.c:784]
- Source file: `src/parseopt/parseopt.c`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/parseopt.c:790]
- Source file: `src/parseopt/parseopt.c`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/parseopt.c:797]
- Source file: `src/parseopt/parseopt.c`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/parseopt.c:798]
- Source file: `src/parseopt/parseopt.c`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/parseopt.c:804]
- Source file: `src/parseopt/parseopt.c`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/parseopt.c:816]
- Source file: `src/parseopt/parseopt.c`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/parseopt.c:818]
- Source file: `src/parseopt/parseopt.c`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/parseopt.h:40]
- Source file: `src/parseopt/parseopt.h`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/parseopt.h:42-57]
- Source file: `src/parseopt/parseopt.h`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/parseopt.h:51]
- Source file: `src/parseopt/parseopt.h`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/parseopt.h:51]
- Source file: `src/parseopt/parseopt.h`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/parseopt.h:55]
- Source file: `src/parseopt/parseopt.h`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/parseopt.h:60]
- Source file: `src/parseopt/parseopt.h`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/parseopt.h:66]
- Source file: `src/parseopt/parseopt.h`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/parseopt.h:72]
- Source file: `src/parseopt/parseopt.h`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/parseopt.h:78]
- Source file: `src/parseopt/parseopt.h`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/parseopt.h:84]
- Source file: `src/parseopt/parseopt.h`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/parseopt.h:90]
- Source file: `src/parseopt/parseopt.h`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/parseopt.h:96]
- Source file: `src/parseopt/parseopt.h`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/parseopt.h:102]
- Source file: `src/parseopt/parseopt.h`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/parseopt.h:137-203]
- Source file: `src/parseopt/parseopt.h`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/parseopt.h:142]
- Source file: `src/parseopt/parseopt.h`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/parseopt.h:152]
- Source file: `src/parseopt/parseopt.h`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/parseopt.h:153]
- Source file: `src/parseopt/parseopt.h`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/parseopt.h:154]
- Source file: `src/parseopt/parseopt.h`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/parseopt.h:155]
- Source file: `src/parseopt/parseopt.h`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/parseopt.h:160]
- Source file: `src/parseopt/parseopt.h`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/parseopt.h:163]
- Source file: `src/parseopt/parseopt.h`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/parseopt.h:169]
- Source file: `src/parseopt/parseopt.h`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/parseopt.h:170]
- Source file: `src/parseopt/parseopt.h`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/parseopt.h:179]
- Source file: `src/parseopt/parseopt.h`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/parseopt.h:205]
- Source file: `src/parseopt/parseopt.h`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/parseopt.h:206]
- Source file: `src/parseopt/parseopt.h`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/parseopt.h:208]
- Source file: `src/parseopt/parseopt.h`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/parseopt.h:221]
- Source file: `src/parseopt/parseopt.h`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/parseopt.h:222]
- Source file: `src/parseopt/parseopt.h`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/parseopt.h:223]
- Source file: `src/parseopt/parseopt.h`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/parseopt.h:224]
- Source file: `src/parseopt/parseopt.h`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/parseopt.h:225]
- Source file: `src/parseopt/parseopt.h`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/parseopt.h:226]
- Source file: `src/parseopt/parseopt.h`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/parseopt.h:227]
- Source file: `src/parseopt/parseopt.h`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/parseopt.h:228]
- Source file: `src/parseopt/parseopt.h`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/parseopt.h:231]
- Source file: `src/parseopt/parseopt.h`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/parseopt.h:232]
- Source file: `src/parseopt/parseopt.h`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/parseopt.h:235]
- Source file: `src/parseopt/parseopt.h`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/parseopt.h:236]
- Source file: `src/parseopt/parseopt.h`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/parseopt.h:240]
- Source file: `src/parseopt/parseopt.h`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/parseopt.h:240]
- Source file: `src/parseopt/parseopt.h`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/parseopt.h:241]
- Source file: `src/parseopt/parseopt.h`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/parseopt.h:241]
- Source file: `src/parseopt/parseopt.h`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/parseopt.h:242]
- Source file: `src/parseopt/parseopt.h`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/parseopt.h:244]
- Source file: `src/parseopt/parseopt.h`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/parseopt.h:244]
- Source file: `src/parseopt/parseopt.h`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/parseopt.h:246]
- Source file: `src/parseopt/parseopt.h`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/parseopt.h:246]
- Source file: `src/parseopt/parseopt.h`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/parseopt.h:247]
- Source file: `src/parseopt/parseopt.h`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/parseopt.h:247]
- Source file: `src/parseopt/parseopt.h`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/parseopt.h:248]
- Source file: `src/parseopt/parseopt.h`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/parseopt.h:248]
- Source file: `src/parseopt/parseopt.h`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/parseopt.h:249]
- Source file: `src/parseopt/parseopt.h`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/parseopt.h:249]
- Source file: `src/parseopt/parseopt.h`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/parseopt.h:250]
- Source file: `src/parseopt/parseopt.h`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/parseopt.h:250]
- Source file: `src/parseopt/parseopt.h`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/parseopt.h:251]
- Source file: `src/parseopt/parseopt.h`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/parseopt.h:251]
- Source file: `src/parseopt/parseopt.h`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/parseopt.h:252]
- Source file: `src/parseopt/parseopt.h`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/parseopt.h:252]
- Source file: `src/parseopt/parseopt.h`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/parseopt.h:254]
- Source file: `src/parseopt/parseopt.h`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/parseopt.h:254]
- Source file: `src/parseopt/parseopt.h`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/parseopt.h:255]
- Source file: `src/parseopt/parseopt.h`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/parseopt.h:255]
- Source file: `src/parseopt/parseopt.h`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/parseopt.h:256]
- Source file: `src/parseopt/parseopt.h`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/parseopt.h:256]
- Source file: `src/parseopt/parseopt.h`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/parseopt.h:258]
- Source file: `src/parseopt/parseopt.h`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/parseopt.h:258]
- Source file: `src/parseopt/parseopt.h`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/parseopt.h:259]
- Source file: `src/parseopt/parseopt.h`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/parseopt.h:259]
- Source file: `src/parseopt/parseopt.h`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/parseopt.h:260]
- Source file: `src/parseopt/parseopt.h`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/parseopt.h:260]
- Source file: `src/parseopt/parseopt.h`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/parseopt.h:261]
- Source file: `src/parseopt/parseopt.h`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/parseopt.h:261]
- Source file: `src/parseopt/parseopt.h`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/parseopt.h:263-276]
- Source file: `src/parseopt/parseopt.h`
- Observed declaration prefix: `struct parseopt_help_format`
### `anonymous`
- Definition location: [src/parseopt/parseopt.h:292]
- Source file: `src/parseopt/parseopt.h`
- Observed declaration prefix: `struct parseopt_help_format`
### `anonymous`
- Definition location: [src/parseopt/wordwrap.h:25]
- Source file: `src/parseopt/wordwrap.h`
- Observed declaration prefix: `struct wordwrap_file`

## Referenced External Types
- `optdef`: this name came from clustering metadata or nearby call analysis, but no local definition was observed in the current module files.
- `parseopt`: this name came from clustering metadata or nearby call analysis, but no local definition was observed in the current module files.

## Macros and Constants
- `_GL_STATIC_ASSERT_H` [gnu/assert.h:53]: `#define _GL_STATIC_ASSERT_H`
- `_GL_CONCAT0` [gnu/assert.h:135]: `#define _GL_CONCAT0(x, y) x##y`
- `_GL_CONCAT` [gnu/assert.h:136]: `#define _GL_CONCAT(x, y) _GL_CONCAT0 (x, y)`
- `_GL_CONCAT` [gnu/assert.h:187]: `#define _GL_CONCAT(x, y) _GL_CONCAT0 (x, y)`
- `_GL_CONCAT0` [gnu/assert.h:188]: `#define _GL_CONCAT0(x, y) x##y`
- `_GL_GENSYM` [gnu/assert.h:202]: `#define _GL_GENSYM(prefix) _GL_CONCAT (prefix, _GL_COUNTER)`
- `_GL_STATIC_ASSERT_TRUE` [gnu/assert.h:208-209]: `#define _GL_STATIC_ASSERT_TRUE(R, DIAGNOSTIC) \ (!!sizeof (_GL_STATIC_ASSERT_TYPE (R, DIAGNOSTIC)))`
- `_GL_ALREADY_INCLUDING_STDIO_H` [gnu/stdio.h:54]: `#define _GL_ALREADY_INCLUDING_STDIO_H`
- `_GL_STDIO_H` [gnu/stdio.h:67]: `#define _GL_STDIO_H`
- `_GL_ATTRIBUTE_FORMAT_PRINTF_STANDARD` [gnu/stdio.h:206-207]: `#define _GL_ATTRIBUTE_FORMAT_PRINTF_STANDARD(formatstring_parameter, first_argument) \ _GL_ATTRIBUTE_FORMAT ((_GL_ATTRIBUTE_SPEC_PRINTF_STANDARD, formatstring_parameter, first_a...`
- `_GL_ATTRIBUTE_FORMAT_PRINTF_SYSTEM` [gnu/stdio.h:213-214]: `#define _GL_ATTRIBUTE_FORMAT_PRINTF_SYSTEM(formatstring_parameter, first_argument) \ _GL_ATTRIBUTE_FORMAT ((_GL_ATTRIBUTE_SPEC_PRINTF_SYSTEM, formatstring_parameter, first_argum...`
- `_GL_ATTRIBUTE_FORMAT_SCANF_SYSTEM` [gnu/stdio.h:232-233]: `#define _GL_ATTRIBUTE_FORMAT_SCANF_SYSTEM(formatstring_parameter, first_argument) \ _GL_ATTRIBUTE_FORMAT ((__scanf__, formatstring_parameter, first_argument))`
- `_GL_CXXDEFS_H` [gnu/stdio.h:255]: `#define _GL_CXXDEFS_H`
- `_GL_FUNCDECL_RPL` [gnu/stdio.h:346-347]: `#define _GL_FUNCDECL_RPL(func,rettype,parameters_and_attributes) \ _GL_FUNCDECL_RPL_1 (rpl_##func, rettype, parameters_and_attributes)`
- `_GL_FUNCDECL_RPL_1` [gnu/stdio.h:348-349]: `#define _GL_FUNCDECL_RPL_1(rpl_func,rettype,parameters_and_attributes) \ _GL_EXTERN_C rettype rpl_func parameters_and_attributes`
- `_GL_FUNCDECL_SYS` [gnu/stdio.h:358-359]: `#define _GL_FUNCDECL_SYS(func,rettype,parameters_and_attributes) \ _GL_EXTERN_C rettype func parameters_and_attributes`
- `_GL_CXXALIAS_RPL` [gnu/stdio.h:370-371]: `#define _GL_CXXALIAS_RPL(func,rettype,parameters) \ _GL_CXXALIAS_RPL_1 (func, rpl_##func, rettype, parameters)`
- `_GL_CXXALIAS_MDA` [gnu/stdio.h:399-400]: `#define _GL_CXXALIAS_MDA(func,rettype,parameters) \ _GL_CXXALIAS_RPL_1 (func, _##func, rettype, parameters)`
- `_GL_CXXALIAS_MDA_CAST` [gnu/stdio.h:432-433]: `#define _GL_CXXALIAS_MDA_CAST(func,rettype,parameters) \ _GL_CXXALIAS_RPL_CAST_1 (func, _##func, rettype, parameters)`
- `_GL_STDIO_STRINGIZE` [gnu/stdio.h:760]: `#define _GL_STDIO_STRINGIZE(token) #token`

## Global Variables
- No global variable definitions were observed in the current module's `.c` files.

## Known Gaps
- This document is generated from parsed results for functions, structs, macros, and global variables; it does not infer declaration signatures from `.h` files that were not parsed.
- If a function appears in the "Functions" section without an explicit header binding, the later Rust migration should re-check the corresponding source `#include` relationships and build scripts.
- Error codes, configuration items, and input/output protocols are recorded only when explicit symbols appear in the source; missing entries do not mean the semantics do not exist, only that the current fact extraction did not observe them.
