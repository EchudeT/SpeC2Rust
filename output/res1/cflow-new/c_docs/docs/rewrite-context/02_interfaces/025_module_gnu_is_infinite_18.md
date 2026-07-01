# Interface Facts: module_gnu_is_infinite_18

This document is intended for the later Rust repository-level rewrite and keeps only the interface facts directly observed in the current source-analysis stage.
Header files, macros, error codes, and configuration items that do not appear in the current parse results will not be added or assumed.

## Module Scope
- Module category: `module_cluster`
- Directory: `gnu`
- File list: gnu/vasnprintf.c
- Candidate header files: gnu/_Noreturn.h, gnu/alignof.h, gnu/alloca.h, gnu/alloca.in.h, gnu/arg-nonnull.h, gnu/assert.h, gnu/assert.in.h, gnu/attribute.h, gnu/basename-lgpl.h, gnu/bitrotate.h, gnu/c++defs.h, gnu/cloexec.h, gnu/errno.in.h, gnu/error.h, gnu/error.in.h, gnu/exitfail.h, gnu/fcntl.h, gnu/fcntl.in.h, gnu/fd-hook.h, gnu/filename.h, gnu/float+.h, gnu/float.in.h, gnu/getprogname.h, gnu/gettext.h, gnu/hash.h, gnu/ialloc.h, gnu/idx.h, gnu/intprops-internal.h, gnu/intprops.h, gnu/inttypes.h, gnu/inttypes.in.h, gnu/limits.h, gnu/limits.in.h, gnu/malloca.h, gnu/minmax.h, gnu/msvc-inval.h, gnu/msvc-nothrow.h, gnu/obstack.h, gnu/obstack.in.h, gnu/pathmax.h, gnu/printf-args.h, gnu/printf-parse.h, gnu/progname.h, gnu/size_max.h, gnu/stat-time.h, gnu/stat-w32.h, gnu/stdckdint.h, gnu/stdckdint.in.h, gnu/stddef.h, gnu/stddef.in.h, gnu/stdint.in.h, gnu/stdio.h, gnu/stdio.in.h, gnu/stdlib.h, gnu/stdlib.in.h, gnu/strerror-override.h, gnu/string.h, gnu/string.in.h, gnu/sys_stat.in.h, gnu/sys_types.in.h, gnu/time.h, gnu/time.in.h, gnu/unistd.h, gnu/unistd.in.h, gnu/vasnprintf.h, gnu/verify.h, gnu/warn-on-use.h, gnu/wchar.h, gnu/wchar.in.h, gnu/xalloc-oversized.h, gnu/xalloc.h, gnu/xsize.h
- Exported functions observed: 2
- Struct definitions observed: 128
- Type names referenced but not defined locally: 0
- Macros observed in related files: 20
- Global variables observed: 0

## Header Evidence
- `gnu/_Noreturn.h` [gnu/_Noreturn.h]
- `gnu/alignof.h` [gnu/alignof.h]
- `gnu/alloca.h` [gnu/alloca.h]
- `gnu/alloca.in.h` [gnu/alloca.in.h]
- `gnu/arg-nonnull.h` [gnu/arg-nonnull.h]
- `gnu/assert.h` [gnu/assert.h]
- `gnu/assert.in.h` [gnu/assert.in.h]
- `gnu/attribute.h` [gnu/attribute.h]
- `gnu/basename-lgpl.h` [gnu/basename-lgpl.h]
- `gnu/bitrotate.h` [gnu/bitrotate.h]
- `gnu/c++defs.h` [gnu/c++defs.h]
- `gnu/cloexec.h` [gnu/cloexec.h]
- `gnu/errno.in.h` [gnu/errno.in.h]
- `gnu/error.h` [gnu/error.h]
- `gnu/error.in.h` [gnu/error.in.h]
- `gnu/exitfail.h` [gnu/exitfail.h]
- `gnu/fcntl.h` [gnu/fcntl.h]
- `gnu/fcntl.in.h` [gnu/fcntl.in.h]
- `gnu/fd-hook.h` [gnu/fd-hook.h]
- `gnu/filename.h` [gnu/filename.h]
- `gnu/float+.h` [gnu/float+.h]
- `gnu/float.in.h` [gnu/float.in.h]
- `gnu/getprogname.h` [gnu/getprogname.h]
- `gnu/gettext.h` [gnu/gettext.h]
- `gnu/hash.h` [gnu/hash.h]
- `gnu/ialloc.h` [gnu/ialloc.h]
- `gnu/idx.h` [gnu/idx.h]
- `gnu/intprops-internal.h` [gnu/intprops-internal.h]
- `gnu/intprops.h` [gnu/intprops.h]
- `gnu/inttypes.h` [gnu/inttypes.h]
- `gnu/inttypes.in.h` [gnu/inttypes.in.h]
- `gnu/limits.h` [gnu/limits.h]
- `gnu/limits.in.h` [gnu/limits.in.h]
- `gnu/malloca.h` [gnu/malloca.h]
- `gnu/minmax.h` [gnu/minmax.h]
- `gnu/msvc-inval.h` [gnu/msvc-inval.h]
- `gnu/msvc-nothrow.h` [gnu/msvc-nothrow.h]
- `gnu/obstack.h` [gnu/obstack.h]
- `gnu/obstack.in.h` [gnu/obstack.in.h]
- `gnu/pathmax.h` [gnu/pathmax.h]
- `gnu/printf-args.h` [gnu/printf-args.h]
- `gnu/printf-parse.h` [gnu/printf-parse.h]
- `gnu/progname.h` [gnu/progname.h]
- `gnu/size_max.h` [gnu/size_max.h]
- `gnu/stat-time.h` [gnu/stat-time.h]
- `gnu/stat-w32.h` [gnu/stat-w32.h]
- `gnu/stdckdint.h` [gnu/stdckdint.h]
- `gnu/stdckdint.in.h` [gnu/stdckdint.in.h]
- `gnu/stddef.h` [gnu/stddef.h]
- `gnu/stddef.in.h` [gnu/stddef.in.h]
- `gnu/stdint.in.h` [gnu/stdint.in.h]
- `gnu/stdio.h` [gnu/stdio.h]
- `gnu/stdio.in.h` [gnu/stdio.in.h]
- `gnu/stdlib.h` [gnu/stdlib.h]
- `gnu/stdlib.in.h` [gnu/stdlib.in.h]
- `gnu/strerror-override.h` [gnu/strerror-override.h]
- `gnu/string.h` [gnu/string.h]
- `gnu/string.in.h` [gnu/string.in.h]
- `gnu/sys_stat.in.h` [gnu/sys_stat.in.h]
- `gnu/sys_types.in.h` [gnu/sys_types.in.h]
- `gnu/time.h` [gnu/time.h]
- `gnu/time.in.h` [gnu/time.in.h]
- `gnu/unistd.h` [gnu/unistd.h]
- `gnu/unistd.in.h` [gnu/unistd.in.h]
- `gnu/vasnprintf.h` [gnu/vasnprintf.h]
- `gnu/verify.h` [gnu/verify.h]
- `gnu/warn-on-use.h` [gnu/warn-on-use.h]
- `gnu/wchar.h` [gnu/wchar.h]
- `gnu/wchar.in.h` [gnu/wchar.in.h]
- `gnu/xalloc-oversized.h` [gnu/xalloc-oversized.h]
- `gnu/xalloc.h` [gnu/xalloc.h]
- `gnu/xsize.h` [gnu/xsize.h]

## Functions
### `is_infinite_or_zero`
- Definition location: [gnu/vasnprintf.c:392-396]
- Source file: `gnu/vasnprintf.c`
- Observed declaration: `static int is_infinite_or_zero (double x);`
- Approximate function body length: 5 lines
### `is_infinite_or_zerol`
- Definition location: [gnu/vasnprintf.c:403-407]
- Source file: `gnu/vasnprintf.c`
- Observed declaration: `static int is_infinite_or_zerol (long double x);`
- Approximate function body length: 5 lines

## Structs and Types
### `anonymous`
- Definition location: [gnu/fd-hook.h:44-60]
- Source file: `gnu/fd-hook.h`
- Observed declaration prefix: `struct fd_hook`
### `anonymous`
- Definition location: [gnu/fd-hook.h:47]
- Source file: `gnu/fd-hook.h`
- Observed declaration prefix: `struct fd_hook`
### `anonymous`
- Definition location: [gnu/fd-hook.h:48]
- Source file: `gnu/fd-hook.h`
- Observed declaration prefix: `struct fd_hook`
### `anonymous`
- Definition location: [gnu/fd-hook.h:51]
- Source file: `gnu/fd-hook.h`
- Observed declaration prefix: `struct fd_hook`
### `anonymous`
- Definition location: [gnu/fd-hook.h:57]
- Source file: `gnu/fd-hook.h`
- Observed declaration prefix: `struct fd_hook`
### `anonymous`
- Definition location: [gnu/fd-hook.h:68]
- Source file: `gnu/fd-hook.h`
- Observed declaration prefix: `struct fd_hook`
### `anonymous`
- Definition location: [gnu/fd-hook.h:74]
- Source file: `gnu/fd-hook.h`
- Observed declaration prefix: `struct fd_hook`
### `anonymous`
- Definition location: [gnu/fd-hook.h:88]
- Source file: `gnu/fd-hook.h`
- Observed declaration prefix: `struct fd_hook`
### `anonymous`
- Definition location: [gnu/fd-hook.h:94]
- Source file: `gnu/fd-hook.h`
- Observed declaration prefix: `struct fd_hook`
### `anonymous`
- Definition location: [gnu/fd-hook.h:108]
- Source file: `gnu/fd-hook.h`
- Observed declaration prefix: `struct fd_hook`
### `anonymous`
- Definition location: [gnu/fd-hook.h:111]
- Source file: `gnu/fd-hook.h`
- Observed declaration prefix: `struct fd_hook`
### `anonymous`
- Definition location: [gnu/float.in.h:101]
- Source file: `gnu/float.in.h`
- Observed declaration prefix: `struct`
### `anonymous`
- Definition location: [gnu/float.in.h:157]
- Source file: `gnu/float.in.h`
- Observed declaration prefix: `struct`
### `anonymous`
- Definition location: [gnu/hash.h:42-52]
- Source file: `gnu/hash.h`
- Observed declaration prefix: `struct hash_tuning`
### `anonymous`
- Definition location: [gnu/hash.h:54]
- Source file: `gnu/hash.h`
- Observed declaration prefix: `struct hash_tuning`
### `anonymous`
- Definition location: [gnu/hash.h:56]
- Source file: `gnu/hash.h`
- Observed declaration prefix: `struct hash_table`
### `anonymous`
- Definition location: [gnu/hash.h:58]
- Source file: `gnu/hash.h`
- Observed declaration prefix: `struct hash_table`
### `anonymous`
- Definition location: [gnu/inttypes.h:1459]
- Source file: `gnu/inttypes.h`
- Observed declaration prefix: `struct`
### `anonymous`
- Definition location: [gnu/inttypes.in.h:940]
- Source file: `gnu/inttypes.in.h`
- Observed declaration prefix: `struct`
### `anonymous`
- Definition location: [gnu/msvc-inval.h:156-165]
- Source file: `gnu/msvc-inval.h`
- Observed declaration prefix: `struct gl_msvc_inval_per_thread`
### `anonymous`
- Definition location: [gnu/msvc-inval.h:175]
- Source file: `gnu/msvc-inval.h`
- Observed declaration prefix: `struct gl_msvc_inval_per_thread`
### `anonymous`
- Definition location: [gnu/obstack.h:168-173]
- Source file: `gnu/obstack.h`
- Observed declaration prefix: `struct _obstack_chunk /* Lives at front of each chunk. */`
### `anonymous`
- Definition location: [gnu/obstack.h:171]
- Source file: `gnu/obstack.h`
- Observed declaration prefix: `struct _obstack_chunk`
### `anonymous`
- Definition location: [gnu/obstack.h:175-210]
- Source file: `gnu/obstack.h`
- Observed declaration prefix: `struct obstack /* control current object in current chunk */`
### `anonymous`
- Definition location: [gnu/obstack.h:178]
- Source file: `gnu/obstack.h`
- Observed declaration prefix: `struct _obstack_chunk`
### `anonymous`
- Definition location: [gnu/obstack.h:222]
- Source file: `gnu/obstack.h`
- Observed declaration prefix: `struct obstack`
### `anonymous`
- Definition location: [gnu/obstack.h:223]
- Source file: `gnu/obstack.h`
- Observed declaration prefix: `struct obstack`
### `anonymous`
- Definition location: [gnu/obstack.h:224]
- Source file: `gnu/obstack.h`
- Observed declaration prefix: `struct obstack`
### `anonymous`
- Definition location: [gnu/obstack.h:227]
- Source file: `gnu/obstack.h`
- Observed declaration prefix: `struct obstack`
### `anonymous`
- Definition location: [gnu/obstack.h:231]
- Source file: `gnu/obstack.h`
- Observed declaration prefix: `struct obstack`
### `anonymous`
- Definition location: [gnu/obstack.in.h:167-172]
- Source file: `gnu/obstack.in.h`
- Observed declaration prefix: `struct _obstack_chunk /* Lives at front of each chunk. */`
### `anonymous`
- Definition location: [gnu/obstack.in.h:170]
- Source file: `gnu/obstack.in.h`
- Observed declaration prefix: `struct _obstack_chunk`
### `anonymous`
- Definition location: [gnu/obstack.in.h:174-209]
- Source file: `gnu/obstack.in.h`
- Observed declaration prefix: `struct obstack /* control current object in current chunk */`
### `anonymous`
- Definition location: [gnu/obstack.in.h:177]
- Source file: `gnu/obstack.in.h`
- Observed declaration prefix: `struct _obstack_chunk`
### `anonymous`
- Definition location: [gnu/obstack.in.h:221]
- Source file: `gnu/obstack.in.h`
- Observed declaration prefix: `struct obstack`
### `anonymous`
- Definition location: [gnu/obstack.in.h:222]
- Source file: `gnu/obstack.in.h`
- Observed declaration prefix: `struct obstack`
### `anonymous`
- Definition location: [gnu/obstack.in.h:223]
- Source file: `gnu/obstack.in.h`
- Observed declaration prefix: `struct obstack`
### `anonymous`
- Definition location: [gnu/obstack.in.h:226]
- Source file: `gnu/obstack.in.h`
- Observed declaration prefix: `struct obstack`
### `anonymous`
- Definition location: [gnu/obstack.in.h:230]
- Source file: `gnu/obstack.in.h`
- Observed declaration prefix: `struct obstack`
### `anonymous`
- Definition location: [gnu/printf-args.h:120-184]
- Source file: `gnu/printf-args.h`
- Observed declaration prefix: `truct`
### `anonymous`
- Definition location: [gnu/printf-args.h:190-195]
- Source file: `gnu/printf-args.h`
- Observed declaration prefix: `truct`
### `anonymous`
- Definition location: [gnu/printf-parse.h:55-68]
- Source file: `gnu/printf-parse.h`
- Observed declaration prefix: `struct`
### `anonymous`
- Definition location: [gnu/printf-parse.h:72-79]
- Source file: `gnu/printf-parse.h`
- Observed declaration prefix: `struct`
### `anonymous`
- Definition location: [gnu/printf-parse.h:85-98]
- Source file: `gnu/printf-parse.h`
- Observed declaration prefix: `struct`
### `anonymous`
- Definition location: [gnu/printf-parse.h:102-109]
- Source file: `gnu/printf-parse.h`
- Observed declaration prefix: `struct`
### `anonymous`
- Definition location: [gnu/printf-parse.h:113-126]
- Source file: `gnu/printf-parse.h`
- Observed declaration prefix: `struct`
### `anonymous`
- Definition location: [gnu/printf-parse.h:130-137]
- Source file: `gnu/printf-parse.h`
- Observed declaration prefix: `struct`
### `anonymous`
- Definition location: [gnu/printf-parse.h:141-154]
- Source file: `gnu/printf-parse.h`
- Observed declaration prefix: `struct`
### `anonymous`
- Definition location: [gnu/printf-parse.h:158-165]
- Source file: `gnu/printf-parse.h`
- Observed declaration prefix: `struct`
### `anonymous`
- Definition location: [gnu/stat-time.h:72]
- Source file: `gnu/stat-time.h`
- Observed declaration prefix: `struct stat`
### `anonymous`
- Definition location: [gnu/stat-time.h:85]
- Source file: `gnu/stat-time.h`
- Observed declaration prefix: `struct stat`
### `anonymous`
- Definition location: [gnu/stat-time.h:98]
- Source file: `gnu/stat-time.h`
- Observed declaration prefix: `struct stat`
### `anonymous`
- Definition location: [gnu/stat-time.h:124]
- Source file: `gnu/stat-time.h`
- Observed declaration prefix: `struct stat`
### `anonymous`
- Definition location: [gnu/stat-time.h:129]
- Source file: `gnu/stat-time.h`
- Observed declaration prefix: `struct timespec`
### `anonymous`
- Definition location: [gnu/stat-time.h:136]
- Source file: `gnu/stat-time.h`
- Observed declaration prefix: `struct stat`
### `anonymous`
- Definition location: [gnu/stat-time.h:141]
- Source file: `gnu/stat-time.h`
- Observed declaration prefix: `struct timespec`
### `anonymous`
- Definition location: [gnu/stat-time.h:148]
- Source file: `gnu/stat-time.h`
- Observed declaration prefix: `struct stat`
### `anonymous`
- Definition location: [gnu/stat-time.h:153]
- Source file: `gnu/stat-time.h`
- Observed declaration prefix: `struct timespec`
### `anonymous`
- Definition location: [gnu/stat-time.h:163]
- Source file: `gnu/stat-time.h`
- Observed declaration prefix: `struct timespec`
### `anonymous`
- Definition location: [gnu/stat-time.h:169]
- Source file: `gnu/stat-time.h`
- Observed declaration prefix: `struct timespec`
### `anonymous`
- Definition location: [gnu/stat-time.h:178]
- Source file: `gnu/stat-time.h`
- Observed declaration prefix: `struct timespec`
### `anonymous`
- Definition location: [gnu/stat-time.h:182]
- Source file: `gnu/stat-time.h`
- Observed declaration prefix: `struct timespec`
### `anonymous`
- Definition location: [gnu/stat-time.h:194]
- Source file: `gnu/stat-time.h`
- Observed declaration prefix: `struct timespec`
### `anonymous`
- Definition location: [gnu/stat-time.h:219]
- Source file: `gnu/stat-time.h`
- Observed declaration prefix: `struct timespec`
### `anonymous`
- Definition location: [gnu/stat-time.h:219]
- Source file: `gnu/stat-time.h`
- Observed declaration prefix: `struct timespec`
### `anonymous`
- Definition location: [gnu/stat-w32.h:24]
- Source file: `gnu/stat-w32.h`
- Observed declaration prefix: `struct timespec`
### `anonymous`
- Definition location: [gnu/stat-w32.h:32]
- Source file: `gnu/stat-w32.h`
- Observed declaration prefix: `struct stat`
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
- Definition location: [gnu/stdio.in.h:1080]
- Source file: `gnu/stdio.in.h`
- Observed declaration prefix: `struct obstack`
### `anonymous`
- Definition location: [gnu/stdio.in.h:1091]
- Source file: `gnu/stdio.in.h`
- Observed declaration prefix: `struct obstack`
### `anonymous`
- Definition location: [gnu/stdio.in.h:1095]
- Source file: `gnu/stdio.in.h`
- Observed declaration prefix: `struct obstack`
### `anonymous`
- Definition location: [gnu/stdio.in.h:1125]
- Source file: `gnu/stdio.in.h`
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
- Definition location: [gnu/stdlib.in.h:85-94]
- Source file: `gnu/stdlib.in.h`
- Observed declaration prefix: `struct random_data`
### `anonymous`
- Definition location: [gnu/stdlib.in.h:1272]
- Source file: `gnu/stdlib.in.h`
- Observed declaration prefix: `struct random_data`
### `anonymous`
- Definition location: [gnu/stdlib.in.h:1274]
- Source file: `gnu/stdlib.in.h`
- Observed declaration prefix: `struct random_data`
### `anonymous`
- Definition location: [gnu/stdlib.in.h:1280]
- Source file: `gnu/stdlib.in.h`
- Observed declaration prefix: `struct random_data`
### `anonymous`
- Definition location: [gnu/sys_stat.in.h:890]
- Source file: `gnu/sys_stat.in.h`
- Observed declaration prefix: `struct stat`
### `anonymous`
- Definition location: [gnu/sys_types.in.h:84]
- Source file: `gnu/sys_types.in.h`
- Observed declaration prefix: `struct`
### `anonymous`
- Definition location: [gnu/time.h:611-615]
- Source file: `gnu/time.h`
- Observed declaration prefix: `struct timespec`
### `anonymous`
- Definition location: [gnu/time.h:632-634]
- Source file: `gnu/time.h`
- Observed declaration prefix: `struct __time_t_must_be_integral`
### `anonymous`
- Definition location: [gnu/time.h:654]
- Source file: `gnu/time.h`
- Observed declaration prefix: `struct timespec`
### `anonymous`
- Definition location: [gnu/time.h:656]
- Source file: `gnu/time.h`
- Observed declaration prefix: `struct timespec`
### `anonymous`
- Definition location: [gnu/time.h:659]
- Source file: `gnu/time.h`
- Observed declaration prefix: `struct timespec`
### `anonymous`
- Definition location: [gnu/time.h:662]
- Source file: `gnu/time.h`
- Observed declaration prefix: `struct timespec`
### `anonymous`
- Definition location: [gnu/time.h:679]
- Source file: `gnu/time.h`
- Observed declaration prefix: `struct timespec`
### `anonymous`
- Definition location: [gnu/time.h:682]
- Source file: `gnu/time.h`
- Observed declaration prefix: `struct timespec`
### `anonymous`
- Definition location: [gnu/time.h:723]
- Source file: `gnu/time.h`
- Observed declaration prefix: `struct timespec`
### `anonymous`
- Definition location: [gnu/time.h:726]
- Source file: `gnu/time.h`
- Observed declaration prefix: `struct timespec`
### `anonymous`
- Definition location: [gnu/time.h:730]
- Source file: `gnu/time.h`
- Observed declaration prefix: `struct timespec`
### `anonymous`
- Definition location: [gnu/time.h:734]
- Source file: `gnu/time.h`
- Observed declaration prefix: `struct timespec`
### `anonymous`
- Definition location: [gnu/time.h:792]
- Source file: `gnu/time.h`
- Observed declaration prefix: `struct tm`
### `anonymous`
- Definition location: [gnu/time.h:793]
- Source file: `gnu/time.h`
- Observed declaration prefix: `struct tm`
### `anonymous`
- Definition location: [gnu/time.h:795]
- Source file: `gnu/time.h`
- Observed declaration prefix: `struct tm`
### `anonymous`
- Definition location: [gnu/time.h:996]
- Source file: `gnu/time.h`
- Observed declaration prefix: `struct tm_zone`
### `anonymous`
- Definition location: [gnu/time.h:1053]
- Source file: `gnu/time.h`
- Observed declaration prefix: `struct tm`
### `anonymous`
- Definition location: [gnu/time.h:1054]
- Source file: `gnu/time.h`
- Observed declaration prefix: `struct tm`
### `anonymous`
- Definition location: [gnu/time.h:1057]
- Source file: `gnu/time.h`
- Observed declaration prefix: `struct tm`
### `anonymous`
- Definition location: [gnu/time.h:1059]
- Source file: `gnu/time.h`
- Observed declaration prefix: `struct tm`
### `anonymous`
- Definition location: [gnu/time.in.h:92-96]
- Source file: `gnu/time.in.h`
- Observed declaration prefix: `struct timespec`
### `anonymous`
- Definition location: [gnu/time.in.h:113-115]
- Source file: `gnu/time.in.h`
- Observed declaration prefix: `struct __time_t_must_be_integral`
### `anonymous`
- Definition location: [gnu/time.in.h:135]
- Source file: `gnu/time.in.h`
- Observed declaration prefix: `struct timespec`
### `anonymous`
- Definition location: [gnu/time.in.h:137]
- Source file: `gnu/time.in.h`
- Observed declaration prefix: `struct timespec`
### `anonymous`
- Definition location: [gnu/time.in.h:143]
- Source file: `gnu/time.in.h`
- Observed declaration prefix: `struct timespec`
### `anonymous`
- Definition location: [gnu/time.in.h:163]
- Source file: `gnu/time.in.h`
- Observed declaration prefix: `struct timespec`
### `anonymous`
- Definition location: [gnu/time.in.h:204]
- Source file: `gnu/time.in.h`
- Observed declaration prefix: `struct timespec`
### `anonymous`
- Definition location: [gnu/time.in.h:207]
- Source file: `gnu/time.in.h`
- Observed declaration prefix: `struct timespec`
### `anonymous`
- Definition location: [gnu/time.in.h:215]
- Source file: `gnu/time.in.h`
- Observed declaration prefix: `struct timespec`
### `anonymous`
- Definition location: [gnu/time.in.h:273]
- Source file: `gnu/time.in.h`
- Observed declaration prefix: `struct tm`
### `anonymous`
- Definition location: [gnu/time.in.h:274]
- Source file: `gnu/time.in.h`
- Observed declaration prefix: `struct tm`
### `anonymous`
- Definition location: [gnu/time.in.h:276]
- Source file: `gnu/time.in.h`
- Observed declaration prefix: `struct tm`
### `anonymous`
- Definition location: [gnu/time.in.h:477]
- Source file: `gnu/time.in.h`
- Observed declaration prefix: `struct tm_zone`
### `anonymous`
- Definition location: [gnu/time.in.h:534]
- Source file: `gnu/time.in.h`
- Observed declaration prefix: `struct tm`
### `anonymous`
- Definition location: [gnu/time.in.h:535]
- Source file: `gnu/time.in.h`
- Observed declaration prefix: `struct tm`
### `anonymous`
- Definition location: [gnu/time.in.h:540]
- Source file: `gnu/time.in.h`
- Observed declaration prefix: `struct tm`
### `anonymous`
- Definition location: [gnu/vasnprintf.c:426-430]
- Source file: `gnu/vasnprintf.c`
- Observed declaration prefix: `struct`

## Referenced External Types
- No external struct or type references beyond local definitions were recorded.

## Macros and Constants
- `_ALIGNOF_H` [gnu/alignof.h:20]: `#define _ALIGNOF_H`
- `_GL_ALLOCA_H` [gnu/alloca.h:25]: `#define _GL_ALLOCA_H`
- `_GL_ALLOCA_H` [gnu/alloca.in.h:24]: `#define _GL_ALLOCA_H`
- `_GL_STATIC_ASSERT_H` [gnu/assert.h:53]: `#define _GL_STATIC_ASSERT_H`
- `_GL_CONCAT0` [gnu/assert.h:135]: `#define _GL_CONCAT0(x, y) x##y`
- `_GL_CONCAT` [gnu/assert.h:136]: `#define _GL_CONCAT(x, y) _GL_CONCAT0 (x, y)`
- `_GL_CONCAT` [gnu/assert.h:187]: `#define _GL_CONCAT(x, y) _GL_CONCAT0 (x, y)`
- `_GL_CONCAT0` [gnu/assert.h:188]: `#define _GL_CONCAT0(x, y) x##y`
- `_GL_GENSYM` [gnu/assert.h:202]: `#define _GL_GENSYM(prefix) _GL_CONCAT (prefix, _GL_COUNTER)`
- `_GL_STATIC_ASSERT_TRUE` [gnu/assert.h:208-209]: `#define _GL_STATIC_ASSERT_TRUE(R, DIAGNOSTIC) \ (!!sizeof (_GL_STATIC_ASSERT_TYPE (R, DIAGNOSTIC)))`
- `_GL_ATTRIBUTE_H` [gnu/attribute.h:33]: `#define _GL_ATTRIBUTE_H`
- `DEPRECATED` [gnu/attribute.h:72]: `#define DEPRECATED _GL_ATTRIBUTE_DEPRECATED`
- `ATTRIBUTE_WARNING` [gnu/attribute.h:76]: `#define ATTRIBUTE_WARNING(msg) _GL_ATTRIBUTE_WARNING (msg)`
- `ATTRIBUTE_ERROR` [gnu/attribute.h:80]: `#define ATTRIBUTE_ERROR(msg) _GL_ATTRIBUTE_ERROR (msg)`
- `ATTRIBUTE_MALLOC` [gnu/attribute.h:87]: `#define ATTRIBUTE_MALLOC _GL_ATTRIBUTE_MALLOC`
- `ATTRIBUTE_ALLOC_SIZE` [gnu/attribute.h:94]: `#define ATTRIBUTE_ALLOC_SIZE(args) _GL_ATTRIBUTE_ALLOC_SIZE (args)`
- `ATTRIBUTE_DEALLOC` [gnu/attribute.h:102]: `#define ATTRIBUTE_DEALLOC(f, i) _GL_ATTRIBUTE_DEALLOC(f, i)`
- `ATTRIBUTE_DEALLOC_FREE` [gnu/attribute.h:103]: `#define ATTRIBUTE_DEALLOC_FREE _GL_ATTRIBUTE_DEALLOC_FREE`
- `ATTRIBUTE_SENTINEL` [gnu/attribute.h:111]: `#define ATTRIBUTE_SENTINEL(pos) _GL_ATTRIBUTE_SENTINEL (pos)`
- `ATTRIBUTE_FORMAT` [gnu/attribute.h:130]: `#define ATTRIBUTE_FORMAT(spec) _GL_ATTRIBUTE_FORMAT (spec)`

## Global Variables
- No global variable definitions were observed in the current module's `.c` files.

## Known Gaps
- This document is generated from parsed results for functions, structs, macros, and global variables; it does not infer declaration signatures from `.h` files that were not parsed.
- If a function appears in the "Functions" section without an explicit header binding, the later Rust migration should re-check the corresponding source `#include` relationships and build scripts.
- Error codes, configuration items, and input/output protocols are recorded only when explicit symbols appear in the source; missing entries do not mean the semantics do not exist, only that the current fact extraction did not observe them.
