# Interface Facts: module_src_parseopt_03

This document is intended for the later Rust repository-level rewrite and keeps only the interface facts directly observed in the current source-analysis stage.
Header files, macros, error codes, and configuration items that do not appear in the current parse results will not be added or assumed.

## Module Scope
- Module category: `module_cluster`
- Directory: `src`
- File list: src/main.c
- Candidate header files: gnu/fcntl.h, gnu/progname.h, gnu/sys/stat.h, gnu/sys/types.h, gnu/unistd.h, src/cflow.h, src/parseopt/parseopt.h, src/parser.h, src/wordsplit/wordsplit.h
- Exported functions observed: 15
- Struct definitions observed: 175
- Type names referenced but not defined locally: 2
- Macros observed in related files: 20
- Global variables observed: 0

## Header Evidence
- `gnu/fcntl.h` [gnu/fcntl.h]
- `gnu/progname.h` [gnu/progname.h]
- `gnu/sys/stat.h` [gnu/sys/stat.h]
- `gnu/sys/types.h` [gnu/sys/types.h]
- `gnu/unistd.h` [gnu/unistd.h]
- `src/cflow.h` [src/cflow.h]
- `src/parseopt/parseopt.h` [src/parseopt/parseopt.h]
- `src/parser.h` [src/parser.h]
- `src/wordsplit/wordsplit.h` [src/wordsplit/wordsplit.h]

## Functions
### `optset_include_classes`
- Definition location: [src/main.c:369-401]
- Source file: `src/main.c`
- Observed declaration: `static int optset_include_classes(struct parseopt *po, struct optdef *opt, char *arg);`
- Approximate function body length: 33 lines
### `optset_output_driver`
- Definition location: [src/main.c:403-412]
- Source file: `src/main.c`
- Observed declaration: `static int optset_output_driver(struct parseopt *po, struct optdef *opt, char *arg);`
- Approximate function body length: 10 lines
### `optset_xref`
- Definition location: [src/main.c:414-420]
- Source file: `src/main.c`
- Observed declaration: `static int optset_xref(struct parseopt *po, struct optdef *opt, char *arg);`
- Approximate function body length: 7 lines
### `optset_symbol`
- Definition location: [src/main.c:422-427]
- Source file: `src/main.c`
- Observed declaration: `static int optset_symbol(struct parseopt *po, struct optdef *opt, char *arg);`
- Approximate function body length: 6 lines
### `optset_preproc_option`
- Definition location: [src/main.c:429-444]
- Source file: `src/main.c`
- Observed declaration: `static int optset_preproc_option(struct parseopt *po, struct optdef *opt, char *arg);`
- Approximate function body length: 16 lines
### `optset_preprocess`
- Definition location: [src/main.c:446-452]
- Source file: `src/main.c`
- Observed declaration: `static int optset_preprocess(struct parseopt *po, struct optdef *opt, char *arg);`
- Approximate function body length: 7 lines
### `optset_level_indent`
- Definition location: [src/main.c:454-459]
- Source file: `src/main.c`
- Observed declaration: `static int optset_level_indent(struct parseopt *po, struct optdef *opt, char *arg);`
- Approximate function body length: 6 lines
### `optset_main_symbol`
- Definition location: [src/main.c:461-466]
- Source file: `src/main.c`
- Observed declaration: `static int optset_main_symbol(struct parseopt *po, struct optdef *opt, char *arg);`
- Approximate function body length: 6 lines
### `optset_clear_main_symbol`
- Definition location: [src/main.c:468-474]
- Source file: `src/main.c`
- Observed declaration: `static int optset_clear_main_symbol(struct parseopt *po, struct optdef *opt, char *arg);`
- Approximate function body length: 7 lines
### `optset_install_target`
- Definition location: [src/main.c:476-481]
- Source file: `src/main.c`
- Observed declaration: `static int optset_install_target(struct parseopt *po, struct optdef *opt, char *arg);`
- Approximate function body length: 6 lines
### `optset_int_1`
- Definition location: [src/main.c:483-490]
- Source file: `src/main.c`
- Observed declaration: `static int optset_int_1(struct parseopt *po, struct optdef *opt, char *arg);`
- Approximate function body length: 8 lines
### `optset_prepend_path`
- Definition location: [src/main.c:492-504]
- Source file: `src/main.c`
- Observed declaration: `static int optset_prepend_path(struct parseopt *po, struct optdef *opt, char *arg);`
- Approximate function body length: 13 lines
### `version_hook`
- Definition location: [src/main.c:900-911]
- Source file: `src/main.c`
- Observed declaration: `void version_hook(WORDWRAP_FILE wf, struct parseopt *po);`
- Approximate function body length: 12 lines
### `help_hook`
- Definition location: [src/main.c:913-920]
- Source file: `src/main.c`
- Observed declaration: `static void help_hook(WORDWRAP_FILE wf, struct parseopt *po);`
- Approximate function body length: 8 lines
### `po_env_error`
- Definition location: [src/main.c:938-955]
- Source file: `src/main.c`
- Observed declaration: `static void po_env_error (struct parseopt *po, int pri, char const *fmt, ...);`
- Approximate function body length: 18 lines

## Structs and Types
### `anonymous`
- Definition location: [gnu/sys/stat.h:652-684]
- Source file: `gnu/sys/stat.h`
- Observed declaration prefix: `struct stat`
### `anonymous`
- Definition location: [gnu/sys/stat.h:676]
- Source file: `gnu/sys/stat.h`
- Observed declaration prefix: `struct timespec`
### `anonymous`
- Definition location: [gnu/sys/stat.h:677]
- Source file: `gnu/sys/stat.h`
- Observed declaration prefix: `struct timespec`
### `anonymous`
- Definition location: [gnu/sys/stat.h:678]
- Source file: `gnu/sys/stat.h`
- Observed declaration prefix: `struct timespec`
### `anonymous`
- Definition location: [gnu/sys/stat.h:1409]
- Source file: `gnu/sys/stat.h`
- Observed declaration prefix: `struct stat`
### `anonymous`
- Definition location: [gnu/sys/types.h:85]
- Source file: `gnu/sys/types.h`
- Observed declaration prefix: `struct`
### `anonymous`
- Definition location: [src/cflow.h:48-52]
- Source file: `src/cflow.h`
- Observed declaration prefix: `struct linked_list_entry`
### `anonymous`
- Definition location: [src/cflow.h:49]
- Source file: `src/cflow.h`
- Observed declaration prefix: `struct linked_list_entry`
### `anonymous`
- Definition location: [src/cflow.h:50]
- Source file: `src/cflow.h`
- Observed declaration prefix: `struct linked_list`
### `anonymous`
- Definition location: [src/cflow.h:56-59]
- Source file: `src/cflow.h`
- Observed declaration prefix: `struct linked_list`
### `anonymous`
- Definition location: [src/cflow.h:58]
- Source file: `src/cflow.h`
- Observed declaration prefix: `struct linked_list_entry`
### `anonymous`
- Definition location: [src/cflow.h:77-80]
- Source file: `src/cflow.h`
- Observed declaration prefix: `struct`
### `anonymous`
- Definition location: [src/cflow.h:92]
- Source file: `src/cflow.h`
- Observed declaration prefix: `struct symbol`
### `anonymous`
- Definition location: [src/cflow.h:94-134]
- Source file: `src/cflow.h`
- Observed declaration prefix: `struct symbol`
### `anonymous`
- Definition location: [src/cflow.h:95]
- Source file: `src/cflow.h`
- Observed declaration prefix: `struct table_entry`
### `anonymous`
- Definition location: [src/cflow.h:97]
- Source file: `src/cflow.h`
- Observed declaration prefix: `struct linked_list_entry`
### `anonymous`
- Definition location: [src/cflow.h:102]
- Source file: `src/cflow.h`
- Observed declaration prefix: `struct symbol`
### `anonymous`
- Definition location: [src/cflow.h:119]
- Source file: `src/cflow.h`
- Observed declaration prefix: `struct linked_list`
### `anonymous`
- Definition location: [src/cflow.h:132]
- Source file: `src/cflow.h`
- Observed declaration prefix: `struct linked_list`
### `anonymous`
- Definition location: [src/cflow.h:133]
- Source file: `src/cflow.h`
- Observed declaration prefix: `struct linked_list`
### `anonymous`
- Definition location: [src/cflow.h:208]
- Source file: `src/cflow.h`
- Observed declaration prefix: `struct linked_list`
### `anonymous`
- Definition location: [src/cflow.h:209]
- Source file: `src/cflow.h`
- Observed declaration prefix: `struct linked_list`
### `anonymous`
- Definition location: [src/cflow.h:210]
- Source file: `src/cflow.h`
- Observed declaration prefix: `struct linked_list`
### `anonymous`
- Definition location: [src/cflow.h:211]
- Source file: `src/cflow.h`
- Observed declaration prefix: `struct linked_list`
### `anonymous`
- Definition location: [src/cflow.h:212]
- Source file: `src/cflow.h`
- Observed declaration prefix: `struct linked_list`
### `anonymous`
- Definition location: [src/cflow.h:214]
- Source file: `src/cflow.h`
- Observed declaration prefix: `struct linked_list`
### `anonymous`
- Definition location: [src/cflow.h:215]
- Source file: `src/cflow.h`
- Observed declaration prefix: `struct linked_list_entry`
### `anonymous`
- Definition location: [src/cflow.h:216]
- Source file: `src/cflow.h`
- Observed declaration prefix: `struct linked_list`
### `anonymous`
- Definition location: [src/cflow.h:218]
- Source file: `src/cflow.h`
- Observed declaration prefix: `struct linked_list`
### `anonymous`
- Definition location: [src/cflow.h:249-254]
- Source file: `src/cflow.h`
- Observed declaration prefix: `struct output_symbol`
### `anonymous`
- Definition location: [src/cflow.h:275]
- Source file: `src/cflow.h`
- Observed declaration prefix: `struct cflow_depmap`
### `anonymous`
- Definition location: [src/main.c:31-35]
- Source file: `src/main.c`
- Observed declaration prefix: `struct option_type`
### `anonymous`
- Definition location: [src/main.c:107]
- Source file: `src/main.c`
- Observed declaration prefix: `struct option_type`
### `anonymous`
- Definition location: [src/main.c:122]
- Source file: `src/main.c`
- Observed declaration prefix: `struct option_type`
### `anonymous`
- Definition location: [src/main.c:241]
- Source file: `src/main.c`
- Observed declaration prefix: `struct option_type`
### `anonymous`
- Definition location: [src/main.c:370]
- Source file: `src/main.c`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/main.c:370]
- Source file: `src/main.c`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/main.c:404]
- Source file: `src/main.c`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/main.c:404]
- Source file: `src/main.c`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/main.c:415]
- Source file: `src/main.c`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/main.c:415]
- Source file: `src/main.c`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/main.c:423]
- Source file: `src/main.c`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/main.c:423]
- Source file: `src/main.c`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/main.c:430]
- Source file: `src/main.c`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/main.c:430]
- Source file: `src/main.c`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/main.c:447]
- Source file: `src/main.c`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/main.c:447]
- Source file: `src/main.c`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/main.c:455]
- Source file: `src/main.c`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/main.c:455]
- Source file: `src/main.c`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/main.c:462]
- Source file: `src/main.c`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/main.c:462]
- Source file: `src/main.c`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/main.c:469]
- Source file: `src/main.c`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/main.c:469]
- Source file: `src/main.c`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/main.c:477]
- Source file: `src/main.c`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/main.c:477]
- Source file: `src/main.c`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/main.c:484]
- Source file: `src/main.c`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/main.c:484]
- Source file: `src/main.c`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/main.c:493]
- Source file: `src/main.c`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/main.c:493]
- Source file: `src/main.c`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/main.c:506]
- Source file: `src/main.c`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/main.c:506]
- Source file: `src/main.c`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/main.c:508]
- Source file: `src/main.c`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/main.c:901]
- Source file: `src/main.c`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/main.c:914]
- Source file: `src/main.c`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/main.c:922]
- Source file: `src/main.c`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/main.c:924]
- Source file: `src/main.c`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/main.c:939]
- Source file: `src/main.c`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/main.c:964]
- Source file: `src/main.c`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/main.c:965]
- Source file: `src/main.c`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/main.c:983-989]
- Source file: `src/main.c`
- Observed declaration prefix: `struct parseopt_file`
### `anonymous`
- Definition location: [src/main.c:984]
- Source file: `src/main.c`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/main.c:992]
- Source file: `src/main.c`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/main.c:994]
- Source file: `src/main.c`
- Observed declaration prefix: `struct parseopt_file`
### `anonymous`
- Definition location: [src/main.c:994]
- Source file: `src/main.c`
- Observed declaration prefix: `struct parseopt_file`
### `anonymous`
- Definition location: [src/main.c:1015]
- Source file: `src/main.c`
- Observed declaration prefix: `struct parseopt_file`
### `anonymous`
- Definition location: [src/main.c:1020]
- Source file: `src/main.c`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/main.c:1042]
- Source file: `src/main.c`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/main.c:1049]
- Source file: `src/main.c`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/main.c:1063]
- Source file: `src/main.c`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/main.c:1071-1074]
- Source file: `src/main.c`
- Observed declaration prefix: `struct optfileid`
### `anonymous`
- Definition location: [src/main.c:1076]
- Source file: `src/main.c`
- Observed declaration prefix: `struct linked_list`
### `anonymous`
- Definition location: [src/main.c:1081]
- Source file: `src/main.c`
- Observed declaration prefix: `struct linked_list_entry`
### `anonymous`
- Definition location: [src/main.c:1082]
- Source file: `src/main.c`
- Observed declaration prefix: `struct optfileid`
### `anonymous`
- Definition location: [src/main.c:1107]
- Source file: `src/main.c`
- Observed declaration prefix: `struct passwd`
### `anonymous`
- Definition location: [src/main.c:1142]
- Source file: `src/main.c`
- Observed declaration prefix: `struct parseopt_file`
### `anonymous`
- Definition location: [src/main.c:1144]
- Source file: `src/main.c`
- Observed declaration prefix: `struct stat`
### `anonymous`
- Definition location: [src/main.c:1145]
- Source file: `src/main.c`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/main.c:1199]
- Source file: `src/main.c`
- Observed declaration prefix: `struct stat`
### `anonymous`
- Definition location: [src/main.c:1221]
- Source file: `src/main.c`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/main.c:1221]
- Source file: `src/main.c`
- Observed declaration prefix: `struct optdef`
### `anonymous`
- Definition location: [src/main.c:1223]
- Source file: `src/main.c`
- Observed declaration prefix: `struct parseopt_file`
### `anonymous`
- Definition location: [src/main.c:1256]
- Source file: `src/main.c`
- Observed declaration prefix: `struct parseopt_file`
### `anonymous`
- Definition location: [src/main.c:1320]
- Source file: `src/main.c`
- Observed declaration prefix: `struct parseopt`
### `anonymous`
- Definition location: [src/main.c:1322]
- Source file: `src/main.c`
- Observed declaration prefix: `struct parseopt_file`
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
- Definition location: [src/parser.h:41-43]
- Source file: `src/parser.h`
- Observed declaration prefix: `struct`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.h:22]
- Source file: `src/wordsplit/wordsplit.h`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.h:36-133]
- Source file: `src/wordsplit/wordsplit.h`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.h:129]
- Source file: `src/wordsplit/wordsplit.h`
- Observed declaration prefix: `struct wordsplit_node`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.h:298]
- Source file: `src/wordsplit/wordsplit.h`
- Observed declaration prefix: `struct wordsplit`

## Referenced External Types
- `optdef`: this name came from clustering metadata or nearby call analysis, but no local definition was observed in the current module files.
- `parseopt`: this name came from clustering metadata or nearby call analysis, but no local definition was observed in the current module files.

## Macros and Constants
- `_GL_FCNTL_H` [gnu/fcntl.h:78]: `#define _GL_FCNTL_H`
- `_GL_CXXDEFS_H` [gnu/fcntl.h:110]: `#define _GL_CXXDEFS_H`
- `_GL_FUNCDECL_RPL` [gnu/fcntl.h:201-202]: `#define _GL_FUNCDECL_RPL(func,rettype,parameters_and_attributes) \ _GL_FUNCDECL_RPL_1 (rpl_##func, rettype, parameters_and_attributes)`
- `_GL_FUNCDECL_RPL_1` [gnu/fcntl.h:203-204]: `#define _GL_FUNCDECL_RPL_1(rpl_func,rettype,parameters_and_attributes) \ _GL_EXTERN_C rettype rpl_func parameters_and_attributes`
- `_GL_FUNCDECL_SYS` [gnu/fcntl.h:213-214]: `#define _GL_FUNCDECL_SYS(func,rettype,parameters_and_attributes) \ _GL_EXTERN_C rettype func parameters_and_attributes`
- `_GL_CXXALIAS_RPL` [gnu/fcntl.h:225-226]: `#define _GL_CXXALIAS_RPL(func,rettype,parameters) \ _GL_CXXALIAS_RPL_1 (func, rpl_##func, rettype, parameters)`
- `_GL_CXXALIAS_MDA` [gnu/fcntl.h:254-255]: `#define _GL_CXXALIAS_MDA(func,rettype,parameters) \ _GL_CXXALIAS_RPL_1 (func, _##func, rettype, parameters)`
- `_GL_CXXALIAS_MDA_CAST` [gnu/fcntl.h:287-288]: `#define _GL_CXXALIAS_MDA_CAST(func,rettype,parameters) \ _GL_CXXALIAS_RPL_CAST_1 (func, _##func, rettype, parameters)`
- `_PROGNAME_H` [gnu/progname.h:21]: `#define _PROGNAME_H`
- `set_program_name` [gnu/progname.h:49-50]: `#define set_program_name(ARG0) \ set_program_name_and_installdir (ARG0, INSTALLPREFIX, INSTALLDIR)`
- `_GL_SYS_STAT_H` [gnu/sys/stat.h:59]: `#define _GL_SYS_STAT_H`
- `_GL_CXXDEFS_H` [gnu/sys/stat.h:109]: `#define _GL_CXXDEFS_H`
- `_GL_FUNCDECL_RPL` [gnu/sys/stat.h:200-201]: `#define _GL_FUNCDECL_RPL(func,rettype,parameters_and_attributes) \ _GL_FUNCDECL_RPL_1 (rpl_##func, rettype, parameters_and_attributes)`
- `_GL_FUNCDECL_RPL_1` [gnu/sys/stat.h:202-203]: `#define _GL_FUNCDECL_RPL_1(rpl_func,rettype,parameters_and_attributes) \ _GL_EXTERN_C rettype rpl_func parameters_and_attributes`
- `_GL_FUNCDECL_SYS` [gnu/sys/stat.h:212-213]: `#define _GL_FUNCDECL_SYS(func,rettype,parameters_and_attributes) \ _GL_EXTERN_C rettype func parameters_and_attributes`
- `_GL_CXXALIAS_RPL` [gnu/sys/stat.h:224-225]: `#define _GL_CXXALIAS_RPL(func,rettype,parameters) \ _GL_CXXALIAS_RPL_1 (func, rpl_##func, rettype, parameters)`
- `_GL_CXXALIAS_MDA` [gnu/sys/stat.h:253-254]: `#define _GL_CXXALIAS_MDA(func,rettype,parameters) \ _GL_CXXALIAS_RPL_1 (func, _##func, rettype, parameters)`
- `_GL_CXXALIAS_MDA_CAST` [gnu/sys/stat.h:286-287]: `#define _GL_CXXALIAS_MDA_CAST(func,rettype,parameters) \ _GL_CXXALIAS_RPL_CAST_1 (func, _##func, rettype, parameters)`
- `stat` [gnu/sys/stat.h:1414]: `#define stat stat_used_without_requesting_gnulib_module_stat`
- `_GL_SYS_TYPES_H` [gnu/sys/types.h:52]: `#define _GL_SYS_TYPES_H`

## Global Variables
- No global variable definitions were observed in the current module's `.c` files.

## Known Gaps
- This document is generated from parsed results for functions, structs, macros, and global variables; it does not infer declaration signatures from `.h` files that were not parsed.
- If a function appears in the "Functions" section without an explicit header binding, the later Rust migration should re-check the corresponding source `#include` relationships and build scripts.
- Error codes, configuration items, and input/output protocols are recorded only when explicit symbols appear in the source; missing entries do not mean the semantics do not exist, only that the current fact extraction did not observe them.
