# Module Summary

This document is generated only from module partition results and parsed source facts; it does not turn "insufficient information" into "empty implementation" or "design mistake".

## 1. Module Responsibilities
- Observed focus: At this point, only the file and symbol distribution indicates that this is a local source slice; its responsibilities still need to be confirmed from the source.
- Module category: `main`
- Directory scope: `root`

## 2. Inputs and Outputs
- At this stage, do not speculate about runtime I/O; define interface boundaries by the observed function signatures and source files.
- File input boundary: bash.c, getopt.c, getopt1.c, which.c
- Function count: 33

## 3. Core Interface List
- `uidget` [bash.c:81-102]: `int uidget ();`
- `getmaxgroups` [bash.c:120-146]: `int getmaxgroups ();`
- `initialize_group_array` [bash.c:149-199]: `static void initialize_group_array ();`
- `group_member` [bash.c:205-234]: `group_member (GID_T gid) #else group_member (gid) GID_T gid; #endif /* !__STDC__ && !_MINIX */;`
- `file_status` [bash.c:241-324]: `int file_status (char const* name);`
- `absolute_program` [bash.c:330-334]: `int absolute_program (char const* string);`
- `substring` [bash.c:339-350]: `char * substring (char const* string, int start, int end);`
- `extract_colon_unit` [bash.c:356-396]: `char* extract_colon_unit (char const* string, int* p_index);`
- `get_next_path_element` [bash.c:403-420]: `char* get_next_path_element (char const* path_list, int* path_index_pointer);`
- `make_full_pathname` [bash.c:425-437]: `char * make_full_pathname (const char *path, const char *name, int name_len);`
- `get_current_user_info` [bash.c:440-470]: `void get_current_user_info ();`
- `sh_get_env_value` [bash.c:473-476]: `char* sh_get_env_value (const char* v);`
- `sh_get_home_dir` [bash.c:478-483]: `char* sh_get_home_dir(void);`
- `store_args_and_env` [getopt.c:273-281]: `static void __attribute__ ((unused)) store_args_and_env (int argc, char *const *argv);`
- `exchange` [getopt.c:308-386]: `static void exchange (argv) char **argv;`
- `_getopt_internal` [getopt.c:518-970]: `int _getopt_internal (argc, argv, optstring, longopts, longind, long_only) int argc; char *const *argv; const char *optstring; const struct option *longopts; int *longind; int long_only;`
- `getopt` [getopt.c:972-982]: `int getopt (argc, argv, optstring) int argc; char *const *argv; const char *optstring;`
- `main` [getopt.c:992-1055]: `int main (argc, argv) int argc; char **argv;`
- `getopt_long` [getopt1.c:67-76]: `int getopt_long (argc, argv, options, long_options, opt_index) int argc; char *const *argv; const char *options; const struct option *long_options; int *opt_index;`
- `getopt_long_only` [getopt1.c:83-92]: `int getopt_long_only (argc, argv, options, long_options, opt_index) int argc; char *const *argv; const char *options; const struct option *long_options; int *opt_index;`

## 4. Dependencies on Other Modules
- Internal call count: 65
- External call count: 0
- Cohesion score: 1.00
- Related headers: not recorded in the current module metadata.

## 5. Key Behaviors That Must Be Preserved
- At minimum, preserve the control flow and return conventions from the source code where these function definitions live; check the implementation body directly rather than inferring from the summary.
- Data structures defined in this module's files: `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`

## 6. Module Partition Signals
- The current module is already a consumable unit after partitioner convergence, with no additional split signals.

## Conclusion
- If function definitions really exist in the source, they should not be described as "empty implementations"; this document is grounded in source locations and declaration excerpts.
- "Module partitioning is unreasonable" should only come from real split signals from the partitioner, not from a summary model drawing conclusions when information is insufficient.
