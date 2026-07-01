# Interface Facts: main_root

This document is intended for the later Rust repository-level rewrite and keeps only the interface facts directly observed in the current source-analysis stage.
Header files, macros, error codes, and configuration items that do not appear in the current parse results will not be added or assumed.

## Module Scope
- Module category: `main`
- Directory: `root`
- File list: bash.c, getopt.c, getopt1.c, which.c
- Candidate header files: bash.h, config.h, getopt.h, posixstat.h, sys.h, tilde/tilde.h
- Exported functions observed: 33
- Struct definitions observed: 24
- Type names referenced but not defined locally: 0
- Macros observed in related files: 20
- Global variables observed: 0

## Header Evidence
- `bash.h` [bash.h]
- `config.h` [config.h]
- `getopt.h` [getopt.h]
- `posixstat.h` [posixstat.h]
- `sys.h` [sys.h]
- `tilde/tilde.h` [tilde/tilde.h]

## Functions
### `uidget`
- Definition location: [bash.c:81-102]
- Source file: `bash.c`
- Observed declaration: `int uidget ();`
- Approximate function body length: 22 lines
### `getmaxgroups`
- Definition location: [bash.c:120-146]
- Source file: `bash.c`
- Observed declaration: `int getmaxgroups ();`
- Approximate function body length: 27 lines
### `initialize_group_array`
- Definition location: [bash.c:149-199]
- Source file: `bash.c`
- Observed declaration: `static void initialize_group_array ();`
- Approximate function body length: 51 lines
### `group_member`
- Definition location: [bash.c:205-234]
- Source file: `bash.c`
- Observed declaration: `group_member (GID_T gid) #else group_member (gid) GID_T gid; #endif /* !__STDC__ && !_MINIX */;`
- Approximate function body length: 30 lines
### `file_status`
- Definition location: [bash.c:241-324]
- Source file: `bash.c`
- Observed declaration: `int file_status (char const* name);`
- Approximate function body length: 84 lines
### `absolute_program`
- Definition location: [bash.c:330-334]
- Source file: `bash.c`
- Observed declaration: `int absolute_program (char const* string);`
- Approximate function body length: 5 lines
### `substring`
- Definition location: [bash.c:339-350]
- Source file: `bash.c`
- Observed declaration: `char * substring (char const* string, int start, int end);`
- Approximate function body length: 12 lines
### `extract_colon_unit`
- Definition location: [bash.c:356-396]
- Source file: `bash.c`
- Observed declaration: `char* extract_colon_unit (char const* string, int* p_index);`
- Approximate function body length: 41 lines
### `get_next_path_element`
- Definition location: [bash.c:403-420]
- Source file: `bash.c`
- Observed declaration: `char* get_next_path_element (char const* path_list, int* path_index_pointer);`
- Approximate function body length: 18 lines
### `make_full_pathname`
- Definition location: [bash.c:425-437]
- Source file: `bash.c`
- Observed declaration: `char * make_full_pathname (const char *path, const char *name, int name_len);`
- Approximate function body length: 13 lines
### `get_current_user_info`
- Definition location: [bash.c:440-470]
- Source file: `bash.c`
- Observed declaration: `void get_current_user_info ();`
- Approximate function body length: 31 lines
### `sh_get_env_value`
- Definition location: [bash.c:473-476]
- Source file: `bash.c`
- Observed declaration: `char* sh_get_env_value (const char* v);`
- Approximate function body length: 4 lines
### `sh_get_home_dir`
- Definition location: [bash.c:478-483]
- Source file: `bash.c`
- Observed declaration: `char* sh_get_home_dir(void);`
- Approximate function body length: 6 lines
### `store_args_and_env`
- Definition location: [getopt.c:273-281]
- Source file: `getopt.c`
- Observed declaration: `static void __attribute__ ((unused)) store_args_and_env (int argc, char *const *argv);`
- Approximate function body length: 9 lines
### `exchange`
- Definition location: [getopt.c:308-386]
- Source file: `getopt.c`
- Observed declaration: `static void exchange (argv) char **argv;`
- Approximate function body length: 79 lines
### `_getopt_internal`
- Definition location: [getopt.c:518-970]
- Source file: `getopt.c`
- Observed declaration: `int _getopt_internal (argc, argv, optstring, longopts, longind, long_only) int argc; char *const *argv; const char *optstring; const struct option *longopts; int *longind; int long_only;`
- Approximate function body length: 453 lines
### `getopt`
- Definition location: [getopt.c:972-982]
- Source file: `getopt.c`
- Observed declaration: `int getopt (argc, argv, optstring) int argc; char *const *argv; const char *optstring;`
- Approximate function body length: 11 lines
### `main`
- Definition location: [getopt.c:992-1055]
- Source file: `getopt.c`
- Observed declaration: `int main (argc, argv) int argc; char **argv;`
- Approximate function body length: 64 lines
### `getopt_long`
- Definition location: [getopt1.c:67-76]
- Source file: `getopt1.c`
- Observed declaration: `int getopt_long (argc, argv, options, long_options, opt_index) int argc; char *const *argv; const char *options; const struct option *long_options; int *opt_index;`
- Approximate function body length: 10 lines
### `getopt_long_only`
- Definition location: [getopt1.c:83-92]
- Source file: `getopt1.c`
- Observed declaration: `int getopt_long_only (argc, argv, options, long_options, opt_index) int argc; char *const *argv; const char *options; const struct option *long_options; int *opt_index;`
- Approximate function body length: 10 lines
### `main`
- Definition location: [getopt1.c:102-188]
- Source file: `getopt1.c`
- Observed declaration: `int main (argc, argv) int argc; char **argv;`
- Approximate function body length: 87 lines
### `print_usage`
- Definition location: [which.c:28-51]
- Source file: `which.c`
- Observed declaration: `static void print_usage(FILE *out);`
- Approximate function body length: 24 lines
### `print_version`
- Definition location: [which.c:53-59]
- Source file: `which.c`
- Observed declaration: `static void print_version(void);`
- Approximate function body length: 7 lines
### `print_fail`
- Definition location: [which.c:61-64]
- Source file: `which.c`
- Observed declaration: `static void print_fail(const char *name, const char *path_list);`
- Approximate function body length: 4 lines
### `find_command_in_path`
- Definition location: [which.c:77-163]
- Source file: `which.c`
- Observed declaration: `static char *find_command_in_path(const char *name, const char *path_list, int *path_index);`
- Approximate function body length: 87 lines
### `get_current_working_directory`
- Definition location: [which.c:168-193]
- Source file: `which.c`
- Observed declaration: `static void get_current_working_directory(void);`
- Approximate function body length: 26 lines
### `path_clean_up`
- Definition location: [which.c:195-245]
- Source file: `which.c`
- Observed declaration: `static char *path_clean_up(const char *path);`
- Approximate function body length: 51 lines
### `func_search`
- Definition location: [which.c:262-286]
- Source file: `which.c`
- Observed declaration: `int func_search(int indent, const char *cmd, struct function_st *func_list, int function_start_type);`
- Approximate function body length: 25 lines
### `path_search`
- Definition location: [which.c:288-337]
- Source file: `which.c`
- Observed declaration: `int path_search(int indent, const char *cmd, const char *path_list);`
- Approximate function body length: 50 lines
### `process_alias`
- Definition location: [which.c:339-407]
- Source file: `which.c`
- Observed declaration: `void process_alias(const char *str, int argc, char *argv[], const char *path_list, int function_start_type);`
- Approximate function body length: 69 lines
### `main`
- Definition location: [which.c:429-675]
- Source file: `which.c`
- Observed declaration: `int main(int argc, char *argv[]);`
- Approximate function body length: 247 lines
### `xmalloc`
- Definition location: [which.c:678-687]
- Source file: `which.c`
- Observed declaration: `void *xmalloc(size_t size);`
- Approximate function body length: 10 lines
### `xrealloc`
- Definition location: [which.c:689-700]
- Source file: `which.c`
- Observed declaration: `void *xrealloc(void *ptr, size_t size);`
- Approximate function body length: 12 lines

## Structs and Types
### `anonymous`
- Definition location: [bash.c:59-65]
- Source file: `bash.c`
- Observed declaration prefix: `struct user_info`
### `anonymous`
- Definition location: [bash.c:69]
- Source file: `bash.c`
- Observed declaration prefix: `struct user_info`
### `anonymous`
- Definition location: [bash.c:244]
- Source file: `bash.c`
- Observed declaration prefix: `struct stat`
### `anonymous`
- Definition location: [bash.c:443]
- Source file: `bash.c`
- Observed declaration prefix: `struct passwd`
### `anonymous`
- Definition location: [getopt.c:523]
- Source file: `getopt.c`
- Observed declaration prefix: `struct option`
### `anonymous`
- Definition location: [getopt.c:646]
- Source file: `getopt.c`
- Observed declaration prefix: `struct option`
### `anonymous`
- Definition location: [getopt.c:647]
- Source file: `getopt.c`
- Observed declaration prefix: `struct option`
### `anonymous`
- Definition location: [getopt.c:802]
- Source file: `getopt.c`
- Observed declaration prefix: `struct option`
### `anonymous`
- Definition location: [getopt.c:803]
- Source file: `getopt.c`
- Observed declaration prefix: `struct option`
### `anonymous`
- Definition location: [getopt.c:979]
- Source file: `getopt.c`
- Observed declaration prefix: `struct option`
### `anonymous`
- Definition location: [getopt.h:79-91]
- Source file: `getopt.h`
- Observed declaration prefix: `struct option`
### `anonymous`
- Definition location: [getopt.h:109]
- Source file: `getopt.h`
- Observed declaration prefix: `struct option`
### `anonymous`
- Definition location: [getopt.h:112]
- Source file: `getopt.h`
- Observed declaration prefix: `struct option`
### `anonymous`
- Definition location: [getopt.h:117]
- Source file: `getopt.h`
- Observed declaration prefix: `struct option`
### `anonymous`
- Definition location: [getopt1.c:72]
- Source file: `getopt1.c`
- Observed declaration prefix: `struct option`
### `anonymous`
- Definition location: [getopt1.c:88]
- Source file: `getopt1.c`
- Observed declaration prefix: `struct option`
### `anonymous`
- Definition location: [getopt1.c:114]
- Source file: `getopt1.c`
- Observed declaration prefix: `struct option`
### `anonymous`
- Definition location: [which.c:247-252]
- Source file: `which.c`
- Observed declaration prefix: `struct function_st`
### `anonymous`
- Definition location: [which.c:254]
- Source file: `which.c`
- Observed declaration prefix: `struct function_st`
### `anonymous`
- Definition location: [which.c:262]
- Source file: `which.c`
- Observed declaration prefix: `struct function_st`
### `anonymous`
- Definition location: [which.c:434]
- Source file: `which.c`
- Observed declaration prefix: `struct option`
### `anonymous`
- Definition location: [which.c:604]
- Source file: `which.c`
- Observed declaration prefix: `struct function_st`
### `anonymous`
- Definition location: [which.c:625]
- Source file: `which.c`
- Observed declaration prefix: `struct function_st`
### `anonymous`
- Definition location: [which.c:625]
- Source file: `which.c`
- Observed declaration prefix: `struct function_st`

## Referenced External Types
- No external struct or type references beyond local definitions were recorded.

## Macros and Constants
- `GID_T` [bash.c:29]: `#define GID_T GETGROUPS_T`
- `FREE` [bash.c:76]: `#define FREE(s) do { if (s) free (s); } while (0)`
- `DEFAULT_MAXGROUPS` [bash.c:117]: `#define DEFAULT_MAXGROUPS 64`
- `FS_EXISTS` [bash.h:3]: `#define FS_EXISTS 0x1`
- `FS_EXECABLE` [bash.h:4]: `#define FS_EXECABLE 0x2`
- `FS_EXEC_PREFERRED` [bash.h:5]: `#define FS_EXEC_PREFERRED 0x4`
- `FS_EXEC_ONLY` [bash.h:6]: `#define FS_EXEC_ONLY 0x8`
- `FS_DIRECTORY` [bash.h:7]: `#define FS_DIRECTORY 0x10`
- `FS_NODIRS` [bash.h:8]: `#define FS_NODIRS 0x20`
- `FS_READABLE` [bash.h:9]: `#define FS_READABLE 0x40`
- `savestring` [bash.h:12]: `#define savestring(x) (char *)strcpy(xmalloc(1 + strlen (x)), (x))`
- `GETGROUPS_T` [config.h:6]: `#define GETGROUPS_T gid_t`
- `HAVE_EACCESS` [config.h:9]: `#define HAVE_EACCESS 1`
- `HAVE_GETCWD` [config.h:12]: `#define HAVE_GETCWD 1`
- `HAVE_GETGROUPS` [config.h:15]: `#define HAVE_GETGROUPS 1`
- `HAVE_GETOPT_LONG` [config.h:18]: `#define HAVE_GETOPT_LONG 1`
- `HAVE_GETPWENT` [config.h:21]: `#define HAVE_GETPWENT 1`
- `HAVE_GETPWNAM` [config.h:24]: `#define HAVE_GETPWNAM 1`
- `HAVE_GETPWUID` [config.h:27]: `#define HAVE_GETPWUID 1`
- `HAVE_GETWD` [config.h:33]: `#define HAVE_GETWD 1`

## Global Variables
- No global variable definitions were observed in the current module's `.c` files.

## Known Gaps
- This document is generated from parsed results for functions, structs, macros, and global variables; it does not infer declaration signatures from `.h` files that were not parsed.
- If a function appears in the "Functions" section without an explicit header binding, the later Rust migration should re-check the corresponding source `#include` relationships and build scripts.
- Error codes, configuration items, and input/output protocols are recorded only when explicit symbols appear in the source; missing entries do not mean the semantics do not exist, only that the current fact extraction did not observe them.
