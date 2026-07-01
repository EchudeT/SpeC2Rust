# Interface Facts: module_src_parseopt_parseopt_01

This document is intended for the later Rust repository-level rewrite and keeps only the interface facts directly observed in the current source-analysis stage.
Header files, macros, error codes, and configuration items that do not appear in the current parse results will not be added or assumed.

## Module Scope
- Module category: `module_cluster`
- Directory: `src/parseopt`
- File list: src/parseopt/help.c, src/parseopt/optset.c, src/parseopt/parseopt.c
- Candidate header files: gnu/assert.h, gnu/inttypes.h, gnu/limits.h, gnu/stdio.h, gnu/stdlib.h, gnu/string.h, gnu/sys/types.h, src/parseopt/parseconf.h, src/parseopt/parseopt.h, src/parseopt/wordwrap.h
- Exported functions observed: 15
- Struct definitions observed: 206
- Type names referenced but not defined locally: 4
- Macros observed in related files: 20
- Global variables observed: 0

## Header Evidence
- `gnu/assert.h` [gnu/assert.h]
- `gnu/inttypes.h` [gnu/inttypes.h]
- `gnu/limits.h` [gnu/limits.h]
- `gnu/stdio.h` [gnu/stdio.h]
- `gnu/stdlib.h` [gnu/stdlib.h]
- `gnu/string.h` [gnu/string.h]
- `gnu/sys/types.h` [gnu/sys/types.h]
- `src/parseopt/parseconf.h` [src/parseopt/parseconf.h]
- `src/parseopt/parseopt.h` [src/parseopt/parseopt.h]
- `src/parseopt/wordwrap.h` [src/parseopt/wordwrap.h]

## Functions
### `set_usage_var`
- Definition location: [src/parseopt/help.c:62-128]
- Source file: `src/parseopt/help.c`
- Observed declaration: `static void set_usage_var (struct parseopt *po, char const *text, char **end);`
- Approximate function body length: 67 lines
### `init_usage_vars`
- Definition location: [src/parseopt/help.c:130-154]
- Source file: `src/parseopt/help.c`
- Observed declaration: `static void init_usage_vars (struct parseopt *po);`
- Approximate function body length: 25 lines
### `parseopt_usage_std`
- Definition location: [src/parseopt/help.c:177-256]
- Source file: `src/parseopt/help.c`
- Observed declaration: `static void parseopt_usage_std (struct parseopt *po, WORDWRAP_FILE wf);`
- Approximate function body length: 80 lines
### `parseopt_usage_sdash`
- Definition location: [src/parseopt/help.c:258-290]
- Source file: `src/parseopt/help.c`
- Observed declaration: `static void parseopt_usage_sdash (struct parseopt *po, WORDWRAP_FILE wf);`
- Approximate function body length: 33 lines
### `parseopt_usage_fd`
- Definition location: [src/parseopt/help.c:292-326]
- Source file: `src/parseopt/help.c`
- Observed declaration: `void parseopt_usage_fd (struct parseopt *po, int fd);`
- Approximate function body length: 35 lines
### `parseopt_help_fd`
- Definition location: [src/parseopt/help.c:725-812]
- Source file: `src/parseopt/help.c`
- Observed declaration: `void parseopt_help_fd (struct parseopt *po, int fd);`
- Approximate function body length: 88 lines
### `parseopt_version_fd`
- Definition location: [src/parseopt/help.c:814-826]
- Source file: `src/parseopt/help.c`
- Observed declaration: `void parseopt_version_fd (struct parseopt *po, int fd);`
- Approximate function body length: 13 lines
### `optset_incr`
- Definition location: [src/parseopt/optset.c:94-101]
- Source file: `src/parseopt/optset.c`
- Observed declaration: `int optset_incr (struct parseopt *po, struct optdef *opt, char *arg);`
- Approximate function body length: 8 lines
### `optset_string_copy`
- Definition location: [src/parseopt/optset.c:103-109]
- Source file: `src/parseopt/optset.c`
- Observed declaration: `int optset_string_copy (struct parseopt *po, struct optdef *opt, char *arg);`
- Approximate function body length: 7 lines
### `optset_string`
- Definition location: [src/parseopt/optset.c:111-125]
- Source file: `src/parseopt/optset.c`
- Observed declaration: `int optset_string (struct parseopt *po, struct optdef *opt, char *arg);`
- Approximate function body length: 15 lines
### `optset_string_alloc`
- Definition location: [src/parseopt/optset.c:127-142]
- Source file: `src/parseopt/optset.c`
- Observed declaration: `int optset_string_alloc (struct parseopt *po, struct optdef *opt, char *arg);`
- Approximate function body length: 16 lines
### `optset_true`
- Definition location: [src/parseopt/optset.c:144-149]
- Source file: `src/parseopt/optset.c`
- Observed declaration: `int optset_true (struct parseopt *po, struct optdef *opt, char *arg);`
- Approximate function body length: 6 lines
### `optset_false`
- Definition location: [src/parseopt/optset.c:151-156]
- Source file: `src/parseopt/optset.c`
- Observed declaration: `int optset_false (struct parseopt *po, struct optdef *opt, char *arg);`
- Approximate function body length: 6 lines
### `optset_bool`
- Definition location: [src/parseopt/optset.c:158-163]
- Source file: `src/parseopt/optset.c`
- Observed declaration: `int optset_bool (struct parseopt *po, struct optdef *opt, char *arg);`
- Approximate function body length: 6 lines
### `option_dash`
- Definition location: [src/parseopt/parseopt.c:27-32]
- Source file: `src/parseopt/parseopt.c`
- Observed declaration: `char const * option_dash (struct parseopt *po, struct optdef *opt);`
- Approximate function body length: 6 lines

## Structs and Types
### `anonymous`
- Definition location: [gnu/inttypes.h:1459]
- Source file: `gnu/inttypes.h`
- Observed declaration prefix: `struct`
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
- Definition location: [gnu/sys/types.h:85]
- Source file: `gnu/sys/types.h`
- Observed declaration prefix: `struct`
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
- Definition location: [src/parseopt/optset.c:95]
- Source file: `src/parseopt/optset.c`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/optset.c:95]
- Source file: `src/parseopt/optset.c`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/optset.c:104]
- Source file: `src/parseopt/optset.c`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/optset.c:104]
- Source file: `src/parseopt/optset.c`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/optset.c:112]
- Source file: `src/parseopt/optset.c`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/optset.c:112]
- Source file: `src/parseopt/optset.c`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/optset.c:128]
- Source file: `src/parseopt/optset.c`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/optset.c:128]
- Source file: `src/parseopt/optset.c`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/optset.c:145]
- Source file: `src/parseopt/optset.c`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/optset.c:145]
- Source file: `src/parseopt/optset.c`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/optset.c:152]
- Source file: `src/parseopt/optset.c`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/optset.c:152]
- Source file: `src/parseopt/optset.c`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/parseopt/optset.c:159]
- Source file: `src/parseopt/optset.c`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/parseopt/optset.c:159]
- Source file: `src/parseopt/optset.c`
- Observed declaration prefix: `struct optdef`
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
- `parseopt`: this name came from clustering metadata or nearby call analysis, but no local definition was observed in the current module files.
- `usage_var_def`: this name came from clustering metadata or nearby call analysis, but no local definition was observed in the current module files.

## Macros and Constants
- `_GL_STATIC_ASSERT_H` [gnu/assert.h:53]: `#define _GL_STATIC_ASSERT_H`
- `_GL_CONCAT0` [gnu/assert.h:135]: `#define _GL_CONCAT0(x, y) x##y`
- `_GL_CONCAT` [gnu/assert.h:136]: `#define _GL_CONCAT(x, y) _GL_CONCAT0 (x, y)`
- `_GL_CONCAT` [gnu/assert.h:187]: `#define _GL_CONCAT(x, y) _GL_CONCAT0 (x, y)`
- `_GL_CONCAT0` [gnu/assert.h:188]: `#define _GL_CONCAT0(x, y) x##y`
- `_GL_GENSYM` [gnu/assert.h:202]: `#define _GL_GENSYM(prefix) _GL_CONCAT (prefix, _GL_COUNTER)`
- `_GL_STATIC_ASSERT_TRUE` [gnu/assert.h:208-209]: `#define _GL_STATIC_ASSERT_TRUE(R, DIAGNOSTIC) \ (!!sizeof (_GL_STATIC_ASSERT_TYPE (R, DIAGNOSTIC)))`
- `INTTYPES_H` [gnu/inttypes.h:50]: `#define INTTYPES_H`
- `_GL_CXXDEFS_H` [gnu/inttypes.h:93]: `#define _GL_CXXDEFS_H`
- `_GL_FUNCDECL_RPL` [gnu/inttypes.h:184-185]: `#define _GL_FUNCDECL_RPL(func,rettype,parameters_and_attributes) \ _GL_FUNCDECL_RPL_1 (rpl_##func, rettype, parameters_and_attributes)`
- `_GL_FUNCDECL_RPL_1` [gnu/inttypes.h:186-187]: `#define _GL_FUNCDECL_RPL_1(rpl_func,rettype,parameters_and_attributes) \ _GL_EXTERN_C rettype rpl_func parameters_and_attributes`
- `_GL_FUNCDECL_SYS` [gnu/inttypes.h:196-197]: `#define _GL_FUNCDECL_SYS(func,rettype,parameters_and_attributes) \ _GL_EXTERN_C rettype func parameters_and_attributes`
- `_GL_CXXALIAS_RPL` [gnu/inttypes.h:208-209]: `#define _GL_CXXALIAS_RPL(func,rettype,parameters) \ _GL_CXXALIAS_RPL_1 (func, rpl_##func, rettype, parameters)`
- `_GL_CXXALIAS_MDA` [gnu/inttypes.h:237-238]: `#define _GL_CXXALIAS_MDA(func,rettype,parameters) \ _GL_CXXALIAS_RPL_1 (func, _##func, rettype, parameters)`
- `_GL_CXXALIAS_MDA_CAST` [gnu/inttypes.h:270-271]: `#define _GL_CXXALIAS_MDA_CAST(func,rettype,parameters) \ _GL_CXXALIAS_RPL_CAST_1 (func, _##func, rettype, parameters)`
- `_GL_LIMITS_H` [gnu/limits.h:48]: `#define _GL_LIMITS_H`
- `_GL_INTEGER_WIDTH` [gnu/limits.h:84]: `#define _GL_INTEGER_WIDTH(min, max) (((min) < 0) + _GL_COB128 (max))`
- `_GL_COB128` [gnu/limits.h:85]: `#define _GL_COB128(n) (_GL_COB64 ((n) >> 31 >> 31 >> 2) + _GL_COB64 (n))`
- `_GL_COB64` [gnu/limits.h:86]: `#define _GL_COB64(n) (_GL_COB32 ((n) >> 31 >> 1) + _GL_COB32 (n))`
- `_GL_COB32` [gnu/limits.h:87]: `#define _GL_COB32(n) (_GL_COB16 ((n) >> 16) + _GL_COB16 (n))`

## Global Variables
- No global variable definitions were observed in the current module's `.c` files.

## Known Gaps
- This document is generated from parsed results for functions, structs, macros, and global variables; it does not infer declaration signatures from `.h` files that were not parsed.
- If a function appears in the "Functions" section without an explicit header binding, the later Rust migration should re-check the corresponding source `#include` relationships and build scripts.
- Error codes, configuration items, and input/output protocols are recorded only when explicit symbols appear in the source; missing entries do not mean the semantics do not exist, only that the current fact extraction did not observe them.
