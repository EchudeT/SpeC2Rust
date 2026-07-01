# Interface Facts: module_src_parseopt_optdef_04

This document is intended for the later Rust repository-level rewrite and keeps only the interface facts directly observed in the current source-analysis stage.
Header files, macros, error codes, and configuration items that do not appear in the current parse results will not be added or assumed.

## Module Scope
- Module category: `module_cluster`
- Directory: `src/parseopt`
- File list: src/parseopt/help.c, src/parseopt/parseopt.c
- Candidate header files: gnu/assert.h, gnu/limits.h, gnu/stdio.h, gnu/stdlib.h, gnu/string.h, src/parseopt/parseconf.h, src/parseopt/parseopt.h, src/parseopt/wordwrap.h
- Exported functions observed: 10
- Struct definitions observed: 190
- Type names referenced but not defined locally: 3
- Macros observed in related files: 20
- Global variables observed: 0

## Header Evidence
- `gnu/assert.h` [gnu/assert.h]
- `gnu/limits.h` [gnu/limits.h]
- `gnu/stdio.h` [gnu/stdio.h]
- `gnu/stdlib.h` [gnu/stdlib.h]
- `gnu/string.h` [gnu/string.h]
- `src/parseopt/parseconf.h` [src/parseopt/parseconf.h]
- `src/parseopt/parseopt.h` [src/parseopt/parseopt.h]
- `src/parseopt/wordwrap.h` [src/parseopt/wordwrap.h]

## Functions
### `print_arg`
- Definition location: [src/parseopt/help.c:156-167]
- Source file: `src/parseopt/help.c`
- Observed declaration: `static inline void print_arg (WORDWRAP_FILE wf, struct optdef *opt, int delim, int *argsused);`
- Approximate function body length: 12 lines
### `opt_unalias`
- Definition location: [src/parseopt/help.c:169-175]
- Source file: `src/parseopt/help.c`
- Observed declaration: `static inline struct optdef * opt_unalias (struct optdef *opt);`
- Approximate function body length: 7 lines
### `merge`
- Definition location: [src/parseopt/help.c:345-362]
- Source file: `src/parseopt/help.c`
- Observed declaration: `static void merge (struct optdef const *optv, int *source, int *work, size_t left, size_t right, size_t end, OPTCMP cmp);`
- Approximate function body length: 18 lines
### `print_option_std`
- Definition location: [src/parseopt/help.c:407-468]
- Source file: `src/parseopt/help.c`
- Observed declaration: `static void print_option_std (WORDWRAP_FILE wf, struct help_context *ctx, struct optdef *cur_opt, size_t i, size_t next);`
- Approximate function body length: 62 lines
### `print_option_sdash`
- Definition location: [src/parseopt/help.c:470-496]
- Source file: `src/parseopt/help.c`
- Observed declaration: `static void print_option_sdash (WORDWRAP_FILE wf, struct help_context *ctx, struct optdef *cur_opt, size_t i, size_t next);`
- Approximate function body length: 27 lines
### `print_option`
- Definition location: [src/parseopt/help.c:498-579]
- Source file: `src/parseopt/help.c`
- Observed declaration: `static int print_option (WORDWRAP_FILE wf, struct help_context *ctx, int i);`
- Approximate function body length: 82 lines
### `optcmp`
- Definition location: [src/parseopt/help.c:588-621]
- Source file: `src/parseopt/help.c`
- Observed declaration: `static int optcmp (struct optdef const *optv, int *idx, int i, int j);`
- Approximate function body length: 34 lines
### `sethead`
- Definition location: [src/parseopt/help.c:642-648]
- Source file: `src/parseopt/help.c`
- Observed declaration: `static inline void sethead (struct optsort *ops, int i, int n);`
- Approximate function body length: 7 lines
### `sort_group`
- Definition location: [src/parseopt/help.c:650-690]
- Source file: `src/parseopt/help.c`
- Observed declaration: `static void sort_group (struct optsort *ops);`
- Approximate function body length: 41 lines
### `find_short_name`
- Definition location: [src/parseopt/parseopt.c:447-457]
- Source file: `src/parseopt/parseopt.c`
- Observed declaration: `static int find_short_name (struct optdef *opt);`
- Approximate function body length: 11 lines

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
- Definition location: [src/parseopt/help.c:29]
- Source file: `src/parseopt/help.c`
- Observed declaration prefix: `struct parseopt_help_format`
### `anonymous`
- Definition location: [src/parseopt/help.c:40-45]
- Source file: `src/parseopt/help.c`
- Observed declaration prefix: `struct usage_var_def`
### `anonymous`
- Definition location: [src/parseopt/help.c:47]
- Source file: `src/parseopt/help.c`
- Observed declaration prefix: `struct usage_var_def`
### `anonymous`
- Definition location: [src/parseopt/help.c:63]
- Source file: `src/parseopt/help.c`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/help.c:65]
- Source file: `src/parseopt/help.c`
- Observed declaration prefix: `struct usage_var_def`
### `anonymous`
- Definition location: [src/parseopt/help.c:131]
- Source file: `src/parseopt/help.c`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/help.c:157]
- Source file: `src/parseopt/help.c`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/help.c:169]
- Source file: `src/parseopt/help.c`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/help.c:170]
- Source file: `src/parseopt/help.c`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/help.c:178]
- Source file: `src/parseopt/help.c`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/help.c:181]
- Source file: `src/parseopt/help.c`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/help.c:259]
- Source file: `src/parseopt/help.c`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/help.c:265]
- Source file: `src/parseopt/help.c`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/help.c:268]
- Source file: `src/parseopt/help.c`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/help.c:293]
- Source file: `src/parseopt/help.c`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/help.c:328]
- Source file: `src/parseopt/help.c`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/help.c:330-337]
- Source file: `src/parseopt/help.c`
- Observed declaration prefix: `struct optsort`
### `anonymous`
- Definition location: [src/parseopt/help.c:332]
- Source file: `src/parseopt/help.c`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/help.c:346]
- Source file: `src/parseopt/help.c`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/help.c:365]
- Source file: `src/parseopt/help.c`
- Observed declaration prefix: `struct optsort`
### `anonymous`
- Definition location: [src/parseopt/help.c:397-405]
- Source file: `src/parseopt/help.c`
- Observed declaration prefix: `struct help_context`
### `anonymous`
- Definition location: [src/parseopt/help.c:399]
- Source file: `src/parseopt/help.c`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/help.c:400]
- Source file: `src/parseopt/help.c`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/help.c:408]
- Source file: `src/parseopt/help.c`
- Observed declaration prefix: `struct help_context`
### `anonymous`
- Definition location: [src/parseopt/help.c:409]
- Source file: `src/parseopt/help.c`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/help.c:412]
- Source file: `src/parseopt/help.c`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/help.c:471]
- Source file: `src/parseopt/help.c`
- Observed declaration prefix: `struct help_context`
### `anonymous`
- Definition location: [src/parseopt/help.c:472]
- Source file: `src/parseopt/help.c`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/help.c:479]
- Source file: `src/parseopt/help.c`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/help.c:499]
- Source file: `src/parseopt/help.c`
- Observed declaration prefix: `struct help_context`
### `anonymous`
- Definition location: [src/parseopt/help.c:501]
- Source file: `src/parseopt/help.c`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/help.c:503]
- Source file: `src/parseopt/help.c`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/help.c:589]
- Source file: `src/parseopt/help.c`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/help.c:591]
- Source file: `src/parseopt/help.c`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/help.c:592]
- Source file: `src/parseopt/help.c`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/help.c:624]
- Source file: `src/parseopt/help.c`
- Observed declaration prefix: `struct optsort`
### `anonymous`
- Definition location: [src/parseopt/help.c:643]
- Source file: `src/parseopt/help.c`
- Observed declaration prefix: `struct optsort`
### `anonymous`
- Definition location: [src/parseopt/help.c:645]
- Source file: `src/parseopt/help.c`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/help.c:651]
- Source file: `src/parseopt/help.c`
- Observed declaration prefix: `struct optsort`
### `anonymous`
- Definition location: [src/parseopt/help.c:654]
- Source file: `src/parseopt/help.c`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/help.c:693]
- Source file: `src/parseopt/help.c`
- Observed declaration prefix: `struct help_context`
### `anonymous`
- Definition location: [src/parseopt/help.c:695]
- Source file: `src/parseopt/help.c`
- Observed declaration prefix: `struct optsort`
### `anonymous`
- Definition location: [src/parseopt/help.c:713]
- Source file: `src/parseopt/help.c`
- Observed declaration prefix: `struct help_context`
### `anonymous`
- Definition location: [src/parseopt/help.c:726]
- Source file: `src/parseopt/help.c`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/help.c:729]
- Source file: `src/parseopt/help.c`
- Observed declaration prefix: `struct help_context`
### `anonymous`
- Definition location: [src/parseopt/help.c:815]
- Source file: `src/parseopt/help.c`
- Observed declaration prefix: `struct parseopt`
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
- `help_context`: this name came from clustering metadata or nearby call analysis, but no local definition was observed in the current module files.
- `optdef`: this name came from clustering metadata or nearby call analysis, but no local definition was observed in the current module files.
- `optsort`: this name came from clustering metadata or nearby call analysis, but no local definition was observed in the current module files.

## Macros and Constants
- `_GL_STATIC_ASSERT_H` [gnu/assert.h:53]: `#define _GL_STATIC_ASSERT_H`
- `_GL_CONCAT0` [gnu/assert.h:135]: `#define _GL_CONCAT0(x, y) x##y`
- `_GL_CONCAT` [gnu/assert.h:136]: `#define _GL_CONCAT(x, y) _GL_CONCAT0 (x, y)`
- `_GL_CONCAT` [gnu/assert.h:187]: `#define _GL_CONCAT(x, y) _GL_CONCAT0 (x, y)`
- `_GL_CONCAT0` [gnu/assert.h:188]: `#define _GL_CONCAT0(x, y) x##y`
- `_GL_GENSYM` [gnu/assert.h:202]: `#define _GL_GENSYM(prefix) _GL_CONCAT (prefix, _GL_COUNTER)`
- `_GL_STATIC_ASSERT_TRUE` [gnu/assert.h:208-209]: `#define _GL_STATIC_ASSERT_TRUE(R, DIAGNOSTIC) \ (!!sizeof (_GL_STATIC_ASSERT_TYPE (R, DIAGNOSTIC)))`
- `_GL_LIMITS_H` [gnu/limits.h:48]: `#define _GL_LIMITS_H`
- `_GL_INTEGER_WIDTH` [gnu/limits.h:84]: `#define _GL_INTEGER_WIDTH(min, max) (((min) < 0) + _GL_COB128 (max))`
- `_GL_COB128` [gnu/limits.h:85]: `#define _GL_COB128(n) (_GL_COB64 ((n) >> 31 >> 31 >> 2) + _GL_COB64 (n))`
- `_GL_COB64` [gnu/limits.h:86]: `#define _GL_COB64(n) (_GL_COB32 ((n) >> 31 >> 1) + _GL_COB32 (n))`
- `_GL_COB32` [gnu/limits.h:87]: `#define _GL_COB32(n) (_GL_COB16 ((n) >> 16) + _GL_COB16 (n))`
- `_GL_COB16` [gnu/limits.h:88]: `#define _GL_COB16(n) (_GL_COB8 ((n) >> 8) + _GL_COB8 (n))`
- `_GL_COB8` [gnu/limits.h:89]: `#define _GL_COB8(n) (_GL_COB4 ((n) >> 4) + _GL_COB4 (n))`
- `_GL_COB4` [gnu/limits.h:90]: `#define _GL_COB4(n) (!!((n) & 8) + !!((n) & 4) + !!((n) & 2) + !!((n) & 1))`
- `_GL_ALREADY_INCLUDING_STDIO_H` [gnu/stdio.h:54]: `#define _GL_ALREADY_INCLUDING_STDIO_H`
- `_GL_STDIO_H` [gnu/stdio.h:67]: `#define _GL_STDIO_H`
- `_GL_ATTRIBUTE_FORMAT_PRINTF_STANDARD` [gnu/stdio.h:206-207]: `#define _GL_ATTRIBUTE_FORMAT_PRINTF_STANDARD(formatstring_parameter, first_argument) \ _GL_ATTRIBUTE_FORMAT ((_GL_ATTRIBUTE_SPEC_PRINTF_STANDARD, formatstring_parameter, first_a...`
- `_GL_ATTRIBUTE_FORMAT_PRINTF_SYSTEM` [gnu/stdio.h:213-214]: `#define _GL_ATTRIBUTE_FORMAT_PRINTF_SYSTEM(formatstring_parameter, first_argument) \ _GL_ATTRIBUTE_FORMAT ((_GL_ATTRIBUTE_SPEC_PRINTF_SYSTEM, formatstring_parameter, first_argum...`
- `_GL_ATTRIBUTE_FORMAT_SCANF_SYSTEM` [gnu/stdio.h:232-233]: `#define _GL_ATTRIBUTE_FORMAT_SCANF_SYSTEM(formatstring_parameter, first_argument) \ _GL_ATTRIBUTE_FORMAT ((__scanf__, formatstring_parameter, first_argument))`

## Global Variables
- No global variable definitions were observed in the current module's `.c` files.

## Known Gaps
- This document is generated from parsed results for functions, structs, macros, and global variables; it does not infer declaration signatures from `.h` files that were not parsed.
- If a function appears in the "Functions" section without an explicit header binding, the later Rust migration should re-check the corresponding source `#include` relationships and build scripts.
- Error codes, configuration items, and input/output protocols are recorded only when explicit symbols appear in the source; missing entries do not mean the semantics do not exist, only that the current fact extraction did not observe them.
