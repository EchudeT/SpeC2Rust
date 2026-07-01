# cflow-new Public Interface Overview

This index is intended for the later Rust rewrite stage. It preserves module-level interface entry points instead of compressing all interface facts into a single overly long document.

- Interface module count: 120

## module_doc_main_01
- Module category: module_cluster
- Related headers: gnu/stdio.h, gnu/stdlib.h, gnu/string.h, gnu/sys/stat.h, gnu/sys/types.h, gnu/unistd.h
- Representative functions: main, main, main, main
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 002_module_doc_main_01.md

## module_doc_ack.c_02
- Module category: module_cluster
- Related headers: none
- Representative functions: ack
- Representative structs: none
- Detailed document: 003_module_doc_ack.c_02.md

## module_doc_d.c_03
- Module category: module_cluster
- Related headers: gnu/stdio.h, gnu/stdlib.h, gnu/string.h, gnu/sys/stat.h, gnu/sys/types.h, gnu/unistd.h
- Representative functions: isdir, ignorent, printdir
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 004_module_doc_d.c_03.md

## module_doc_foo.c_04
- Module category: module_cluster
- Related headers: none
- Representative functions: f
- Representative structs: none
- Detailed document: 005_module_doc_foo.c_04.md

## module_doc_wc.c_05
- Module category: module_cluster
- Related headers: gnu/stdio.h, gnu/stdlib.h
- Representative functions: error_print, errf, perrf, report, isword, getword, counter
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 006_module_doc_wc.c_05.md

## module_doc_whoami.c_06
- Module category: module_cluster
- Related headers: gnu/stdio.h, gnu/stdlib.h, gnu/sys/types.h
- Representative functions: who_am_i
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 007_module_doc_whoami.c_06.md

## module_gnu_hash_entry_01
- Module category: module_cluster
- Related headers: config.h, gnu/_Noreturn.h, gnu/alignof.h, gnu/alloca.h, gnu/alloca.in.h, gnu/arg-nonnull.h, gnu/assert.h, gnu/assert.in.h, gnu/attribute.h, gnu/basename-lgpl.h, gnu/bitrotate.h, gnu/c++defs.h, gnu/cloexec.h, gnu/errno.in.h, gnu/error.h, gnu/error.in.h, gnu/exitfail.h, gnu/fcntl.h, gnu/fcntl.in.h, gnu/fd-hook.h, gnu/filename.h, gnu/float+.h, gnu/float.in.h, gnu/getprogname.h, gnu/gettext.h, gnu/hash.h, gnu/ialloc.h, gnu/idx.h, gnu/intprops-internal.h, gnu/intprops.h, gnu/inttypes.h, gnu/inttypes.in.h, gnu/limits.h, gnu/limits.in.h, gnu/malloca.h, gnu/minmax.h, gnu/msvc-inval.h, gnu/msvc-nothrow.h, gnu/obstack.h, gnu/obstack.in.h, gnu/pathmax.h, gnu/printf-args.h, gnu/printf-parse.h, gnu/progname.h, gnu/size_max.h, gnu/stat-time.h, gnu/stat-w32.h, gnu/stdckdint.h, gnu/stdckdint.in.h, gnu/stddef.h, gnu/stddef.in.h, gnu/stdint.in.h, gnu/stdio.h, gnu/stdio.in.h, gnu/stdlib.h, gnu/stdlib.in.h, gnu/strerror-override.h, gnu/string.h, gnu/string.in.h, gnu/sys_stat.in.h, gnu/sys_types.in.h, gnu/time.h, gnu/time.in.h, gnu/unistd.h, gnu/unistd.in.h, gnu/vasnprintf.h, gnu/verify.h, gnu/warn-on-use.h, gnu/wchar.h, gnu/wchar.in.h, gnu/xalloc-oversized.h, gnu/xalloc.h, gnu/xsize.h
- Representative functions: hash_get_max_bucket_length, hash_table_ok, safe_hasher, hash_lookup, hash_get_first, hash_get_next, hash_get_entries, hash_do_for_each
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 008_module_gnu_hash_entry_01.md

## module_gnu_hash_entry_02
- Module category: module_cluster
- Related headers: config.h, gnu/_Noreturn.h, gnu/alignof.h, gnu/alloca.h, gnu/alloca.in.h, gnu/arg-nonnull.h, gnu/assert.h, gnu/assert.in.h, gnu/attribute.h, gnu/basename-lgpl.h, gnu/bitrotate.h, gnu/c++defs.h, gnu/cloexec.h, gnu/errno.in.h, gnu/error.h, gnu/error.in.h, gnu/exitfail.h, gnu/fcntl.h, gnu/fcntl.in.h, gnu/fd-hook.h, gnu/filename.h, gnu/float+.h, gnu/float.in.h, gnu/getprogname.h, gnu/gettext.h, gnu/hash.h, gnu/ialloc.h, gnu/idx.h, gnu/intprops-internal.h, gnu/intprops.h, gnu/inttypes.h, gnu/inttypes.in.h, gnu/limits.h, gnu/limits.in.h, gnu/malloca.h, gnu/minmax.h, gnu/msvc-inval.h, gnu/msvc-nothrow.h, gnu/obstack.h, gnu/obstack.in.h, gnu/pathmax.h, gnu/printf-args.h, gnu/printf-parse.h, gnu/progname.h, gnu/size_max.h, gnu/stat-time.h, gnu/stat-w32.h, gnu/stdckdint.h, gnu/stdckdint.in.h, gnu/stddef.h, gnu/stddef.in.h, gnu/stdint.in.h, gnu/stdio.h, gnu/stdio.in.h, gnu/stdlib.h, gnu/stdlib.in.h, gnu/strerror-override.h, gnu/string.h, gnu/string.in.h, gnu/sys_stat.in.h, gnu/sys_types.in.h, gnu/time.h, gnu/time.in.h, gnu/unistd.h, gnu/unistd.in.h, gnu/vasnprintf.h, gnu/verify.h, gnu/warn-on-use.h, gnu/wchar.h, gnu/wchar.in.h, gnu/xalloc-oversized.h, gnu/xalloc.h, gnu/xsize.h
- Representative functions: hash_insert_if_absent, hash_remove, hash_print
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 009_module_gnu_hash_entry_02.md

## module_gnu_obstack_03
- Module category: module_cluster
- Related headers: gnu/_Noreturn.h, gnu/alignof.h, gnu/alloca.h, gnu/alloca.in.h, gnu/arg-nonnull.h, gnu/assert.h, gnu/assert.in.h, gnu/attribute.h, gnu/basename-lgpl.h, gnu/bitrotate.h, gnu/c++defs.h, gnu/cloexec.h, gnu/errno.in.h, gnu/error.h, gnu/error.in.h, gnu/exitfail.h, gnu/fcntl.h, gnu/fcntl.in.h, gnu/fd-hook.h, gnu/filename.h, gnu/float+.h, gnu/float.in.h, gnu/getprogname.h, gnu/gettext.h, gnu/hash.h, gnu/ialloc.h, gnu/idx.h, gnu/intprops-internal.h, gnu/intprops.h, gnu/inttypes.h, gnu/inttypes.in.h, gnu/limits.h, gnu/limits.in.h, gnu/malloca.h, gnu/minmax.h, gnu/msvc-inval.h, gnu/msvc-nothrow.h, gnu/obstack.h, gnu/obstack.in.h, gnu/pathmax.h, gnu/printf-args.h, gnu/printf-parse.h, gnu/progname.h, gnu/size_max.h, gnu/stat-time.h, gnu/stat-w32.h, gnu/stdckdint.h, gnu/stdckdint.in.h, gnu/stddef.h, gnu/stddef.in.h, gnu/stdint.in.h, gnu/stdio.h, gnu/stdio.in.h, gnu/stdlib.h, gnu/stdlib.in.h, gnu/strerror-override.h, gnu/string.h, gnu/string.in.h, gnu/sys_stat.in.h, gnu/sys_types.in.h, gnu/time.h, gnu/time.in.h, gnu/unistd.h, gnu/unistd.in.h, gnu/vasnprintf.h, gnu/verify.h, gnu/warn-on-use.h, gnu/wchar.h, gnu/wchar.in.h, gnu/xalloc-oversized.h, gnu/xalloc.h, gnu/xsize.h
- Representative functions: call_chunkfun, call_freefun, _obstack_begin_worker, _obstack_begin, _obstack_begin_1, _obstack_newchunk, _obstack_allocated_p, _obstack_free
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 010_module_gnu_obstack_03.md

## module_gnu_stat_04
- Module category: module_cluster
- Related headers: config.h, gnu/_Noreturn.h, gnu/alignof.h, gnu/alloca.h, gnu/alloca.in.h, gnu/arg-nonnull.h, gnu/assert.h, gnu/assert.in.h, gnu/attribute.h, gnu/basename-lgpl.h, gnu/bitrotate.h, gnu/c++defs.h, gnu/cloexec.h, gnu/errno.in.h, gnu/error.h, gnu/error.in.h, gnu/exitfail.h, gnu/fcntl.h, gnu/fcntl.in.h, gnu/fd-hook.h, gnu/filename.h, gnu/float+.h, gnu/float.in.h, gnu/getprogname.h, gnu/gettext.h, gnu/hash.h, gnu/ialloc.h, gnu/idx.h, gnu/intprops-internal.h, gnu/intprops.h, gnu/inttypes.h, gnu/inttypes.in.h, gnu/limits.h, gnu/limits.in.h, gnu/malloca.h, gnu/minmax.h, gnu/msvc-inval.h, gnu/msvc-nothrow.h, gnu/obstack.h, gnu/obstack.in.h, gnu/pathmax.h, gnu/printf-args.h, gnu/printf-parse.h, gnu/progname.h, gnu/size_max.h, gnu/stat-time.h, gnu/stat-w32.h, gnu/stdckdint.h, gnu/stdckdint.in.h, gnu/stddef.h, gnu/stddef.in.h, gnu/stdint.in.h, gnu/stdio.h, gnu/stdio.in.h, gnu/stdlib.h, gnu/stdlib.in.h, gnu/strerror-override.h, gnu/string.h, gnu/string.in.h, gnu/sys/stat.h, gnu/sys/types.h, gnu/sys_stat.in.h, gnu/sys_types.in.h, gnu/time.h, gnu/time.in.h, gnu/unistd.h, gnu/unistd.in.h, gnu/vasnprintf.h, gnu/verify.h, gnu/warn-on-use.h, gnu/wchar.h, gnu/wchar.in.h, gnu/xalloc-oversized.h, gnu/xalloc.h, gnu/xsize.h
- Representative functions: klibc_dup2, klibc_fcntl, orig_fstat, rpl_fstat, open, _gl_fstat_by_handle, orig_stat
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 011_module_gnu_stat_04.md

## module_gnu_stat_05
- Module category: module_cluster
- Related headers: config.h, gnu/_Noreturn.h, gnu/alignof.h, gnu/alloca.h, gnu/alloca.in.h, gnu/arg-nonnull.h, gnu/assert.h, gnu/assert.in.h, gnu/attribute.h, gnu/basename-lgpl.h, gnu/bitrotate.h, gnu/c++defs.h, gnu/cloexec.h, gnu/errno.in.h, gnu/error.h, gnu/error.in.h, gnu/exitfail.h, gnu/fcntl.h, gnu/fcntl.in.h, gnu/fd-hook.h, gnu/filename.h, gnu/float+.h, gnu/float.in.h, gnu/getprogname.h, gnu/gettext.h, gnu/hash.h, gnu/ialloc.h, gnu/idx.h, gnu/intprops-internal.h, gnu/intprops.h, gnu/inttypes.h, gnu/inttypes.in.h, gnu/limits.h, gnu/limits.in.h, gnu/malloca.h, gnu/minmax.h, gnu/msvc-inval.h, gnu/msvc-nothrow.h, gnu/obstack.h, gnu/obstack.in.h, gnu/pathmax.h, gnu/printf-args.h, gnu/printf-parse.h, gnu/progname.h, gnu/size_max.h, gnu/stat-time.h, gnu/stat-w32.h, gnu/stdckdint.h, gnu/stdckdint.in.h, gnu/stddef.h, gnu/stddef.in.h, gnu/stdint.in.h, gnu/stdio.h, gnu/stdio.in.h, gnu/stdlib.h, gnu/stdlib.in.h, gnu/strerror-override.h, gnu/string.h, gnu/string.in.h, gnu/sys/stat.h, gnu/sys/types.h, gnu/sys_stat.in.h, gnu/sys_types.in.h, gnu/time.h, gnu/time.in.h, gnu/unistd.h, gnu/unistd.in.h, gnu/vasnprintf.h, gnu/verify.h, gnu/warn-on-use.h, gnu/wchar.h, gnu/wchar.in.h, gnu/xalloc-oversized.h, gnu/xalloc.h, gnu/xsize.h
- Representative functions: rpl_stat, _GL_ATTRIBUTE_PURE
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 012_module_gnu_stat_05.md

## module_gnu_fd_hook_06
- Module category: module_cluster
- Related headers: config.h, gnu/_Noreturn.h, gnu/alignof.h, gnu/alloca.h, gnu/alloca.in.h, gnu/arg-nonnull.h, gnu/assert.h, gnu/assert.in.h, gnu/attribute.h, gnu/basename-lgpl.h, gnu/bitrotate.h, gnu/c++defs.h, gnu/cloexec.h, gnu/errno.in.h, gnu/error.h, gnu/error.in.h, gnu/exitfail.h, gnu/fcntl.h, gnu/fcntl.in.h, gnu/fd-hook.h, gnu/filename.h, gnu/float+.h, gnu/float.in.h, gnu/getprogname.h, gnu/gettext.h, gnu/hash.h, gnu/ialloc.h, gnu/idx.h, gnu/intprops-internal.h, gnu/intprops.h, gnu/inttypes.h, gnu/inttypes.in.h, gnu/limits.h, gnu/limits.in.h, gnu/malloca.h, gnu/minmax.h, gnu/msvc-inval.h, gnu/msvc-nothrow.h, gnu/obstack.h, gnu/obstack.in.h, gnu/pathmax.h, gnu/printf-args.h, gnu/printf-parse.h, gnu/progname.h, gnu/size_max.h, gnu/stat-time.h, gnu/stat-w32.h, gnu/stdckdint.h, gnu/stdckdint.in.h, gnu/stddef.h, gnu/stddef.in.h, gnu/stdint.in.h, gnu/stdio.h, gnu/stdio.in.h, gnu/stdlib.h, gnu/stdlib.in.h, gnu/strerror-override.h, gnu/string.h, gnu/string.in.h, gnu/sys_stat.in.h, gnu/sys_types.in.h, gnu/time.h, gnu/time.in.h, gnu/unistd.h, gnu/unistd.in.h, gnu/vasnprintf.h, gnu/verify.h, gnu/warn-on-use.h, gnu/wchar.h, gnu/wchar.in.h, gnu/xalloc-oversized.h, gnu/xalloc.h, gnu/xsize.h
- Representative functions: execute_close_hooks, execute_ioctl_hooks, register_fd_hook, unregister_fd_hook
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 013_module_gnu_fd_hook_06.md

## module_gnu_gl_msvc_inval_per_thread_07
- Module category: module_cluster
- Related headers: config.h, gnu/_Noreturn.h, gnu/alignof.h, gnu/alloca.h, gnu/alloca.in.h, gnu/arg-nonnull.h, gnu/assert.h, gnu/assert.in.h, gnu/attribute.h, gnu/basename-lgpl.h, gnu/bitrotate.h, gnu/c++defs.h, gnu/cloexec.h, gnu/errno.in.h, gnu/error.h, gnu/error.in.h, gnu/exitfail.h, gnu/fcntl.h, gnu/fcntl.in.h, gnu/fd-hook.h, gnu/filename.h, gnu/float+.h, gnu/float.in.h, gnu/getprogname.h, gnu/gettext.h, gnu/hash.h, gnu/ialloc.h, gnu/idx.h, gnu/intprops-internal.h, gnu/intprops.h, gnu/inttypes.h, gnu/inttypes.in.h, gnu/limits.h, gnu/limits.in.h, gnu/malloca.h, gnu/minmax.h, gnu/msvc-inval.h, gnu/msvc-nothrow.h, gnu/obstack.h, gnu/obstack.in.h, gnu/pathmax.h, gnu/printf-args.h, gnu/printf-parse.h, gnu/progname.h, gnu/size_max.h, gnu/stat-time.h, gnu/stat-w32.h, gnu/stdckdint.h, gnu/stdckdint.in.h, gnu/stddef.h, gnu/stddef.in.h, gnu/stdint.in.h, gnu/stdio.h, gnu/stdio.in.h, gnu/stdlib.h, gnu/stdlib.in.h, gnu/strerror-override.h, gnu/string.h, gnu/string.in.h, gnu/sys_stat.in.h, gnu/sys_types.in.h, gnu/time.h, gnu/time.in.h, gnu/unistd.h, gnu/unistd.in.h, gnu/vasnprintf.h, gnu/verify.h, gnu/warn-on-use.h, gnu/wchar.h, gnu/wchar.in.h, gnu/xalloc-oversized.h, gnu/xalloc.h, gnu/xsize.h
- Representative functions: gl_msvc_invalid_parameter_handler, gl_msvc_invalid_parameter_handler, gl_msvc_inval_current, gl_msvc_invalid_parameter_handler
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 014_module_gnu_gl_msvc_inval_per_thread_07.md

## module_gnu_rlimit_08
- Module category: module_cluster
- Related headers: config.h, gnu/_Noreturn.h, gnu/alignof.h, gnu/alloca.h, gnu/alloca.in.h, gnu/arg-nonnull.h, gnu/assert.h, gnu/assert.in.h, gnu/attribute.h, gnu/basename-lgpl.h, gnu/bitrotate.h, gnu/c++defs.h, gnu/cloexec.h, gnu/errno.in.h, gnu/error.h, gnu/error.in.h, gnu/exitfail.h, gnu/fcntl.h, gnu/fcntl.in.h, gnu/fd-hook.h, gnu/filename.h, gnu/float+.h, gnu/float.in.h, gnu/getprogname.h, gnu/gettext.h, gnu/hash.h, gnu/ialloc.h, gnu/idx.h, gnu/intprops-internal.h, gnu/intprops.h, gnu/inttypes.h, gnu/inttypes.in.h, gnu/limits.h, gnu/limits.in.h, gnu/malloca.h, gnu/minmax.h, gnu/msvc-inval.h, gnu/msvc-nothrow.h, gnu/obstack.h, gnu/obstack.in.h, gnu/pathmax.h, gnu/printf-args.h, gnu/printf-parse.h, gnu/progname.h, gnu/size_max.h, gnu/stat-time.h, gnu/stat-w32.h, gnu/stdckdint.h, gnu/stdckdint.in.h, gnu/stddef.h, gnu/stddef.in.h, gnu/stdint.in.h, gnu/stdio.h, gnu/stdio.in.h, gnu/stdlib.h, gnu/stdlib.in.h, gnu/strerror-override.h, gnu/string.h, gnu/string.in.h, gnu/sys_stat.in.h, gnu/sys_types.in.h, gnu/time.h, gnu/time.in.h, gnu/unistd.h, gnu/unistd.in.h, gnu/vasnprintf.h, gnu/verify.h, gnu/warn-on-use.h, gnu/wchar.h, gnu/wchar.in.h, gnu/xalloc-oversized.h, gnu/xalloc.h, gnu/xsize.h
- Representative functions: getdtablesize, getdtablesize
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 015_module_gnu_rlimit_08.md

## module_gnu_if_09
- Module category: module_cluster
- Related headers: gnu/_Noreturn.h, gnu/alignof.h, gnu/alloca.h, gnu/alloca.in.h, gnu/arg-nonnull.h, gnu/assert.h, gnu/assert.in.h, gnu/attribute.h, gnu/basename-lgpl.h, gnu/bitrotate.h, gnu/c++defs.h, gnu/cloexec.h, gnu/errno.in.h, gnu/error.h, gnu/error.in.h, gnu/exitfail.h, gnu/fcntl.h, gnu/fcntl.in.h, gnu/fd-hook.h, gnu/filename.h, gnu/float+.h, gnu/float.in.h, gnu/getprogname.h, gnu/gettext.h, gnu/hash.h, gnu/ialloc.h, gnu/idx.h, gnu/intprops-internal.h, gnu/intprops.h, gnu/inttypes.h, gnu/inttypes.in.h, gnu/limits.h, gnu/limits.in.h, gnu/malloca.h, gnu/minmax.h, gnu/msvc-inval.h, gnu/msvc-nothrow.h, gnu/obstack.h, gnu/obstack.in.h, gnu/pathmax.h, gnu/printf-args.h, gnu/printf-parse.h, gnu/progname.h, gnu/size_max.h, gnu/stat-time.h, gnu/stat-w32.h, gnu/stdckdint.h, gnu/stdckdint.in.h, gnu/stddef.h, gnu/stddef.in.h, gnu/stdint.in.h, gnu/stdio.h, gnu/stdio.in.h, gnu/stdlib.h, gnu/stdlib.in.h, gnu/strerror-override.h, gnu/string.h, gnu/string.in.h, gnu/sys_stat.in.h, gnu/sys_types.in.h, gnu/time.h, gnu/time.in.h, gnu/unistd.h, gnu/unistd.in.h, gnu/vasnprintf.h, gnu/verify.h, gnu/warn-on-use.h, gnu/wchar.h, gnu/wchar.in.h, gnu/xalloc-oversized.h, gnu/xalloc.h, gnu/xsize.h
- Representative functions: if, if
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 016_module_gnu_if_09.md

## module_gnu_if_10
- Module category: module_cluster
- Related headers: gnu/_Noreturn.h, gnu/alignof.h, gnu/alloca.h, gnu/alloca.in.h, gnu/arg-nonnull.h, gnu/assert.h, gnu/assert.in.h, gnu/attribute.h, gnu/basename-lgpl.h, gnu/bitrotate.h, gnu/c++defs.h, gnu/cloexec.h, gnu/errno.in.h, gnu/error.h, gnu/error.in.h, gnu/exitfail.h, gnu/fcntl.h, gnu/fcntl.in.h, gnu/fd-hook.h, gnu/filename.h, gnu/float+.h, gnu/float.in.h, gnu/getprogname.h, gnu/gettext.h, gnu/hash.h, gnu/ialloc.h, gnu/idx.h, gnu/intprops-internal.h, gnu/intprops.h, gnu/inttypes.h, gnu/inttypes.in.h, gnu/limits.h, gnu/limits.in.h, gnu/malloca.h, gnu/minmax.h, gnu/msvc-inval.h, gnu/msvc-nothrow.h, gnu/obstack.h, gnu/obstack.in.h, gnu/pathmax.h, gnu/printf-args.h, gnu/printf-parse.h, gnu/progname.h, gnu/size_max.h, gnu/stat-time.h, gnu/stat-w32.h, gnu/stdckdint.h, gnu/stdckdint.in.h, gnu/stddef.h, gnu/stddef.in.h, gnu/stdint.in.h, gnu/stdio.h, gnu/stdio.in.h, gnu/stdlib.h, gnu/stdlib.in.h, gnu/strerror-override.h, gnu/string.h, gnu/string.in.h, gnu/sys_stat.in.h, gnu/sys_types.in.h, gnu/time.h, gnu/time.in.h, gnu/unistd.h, gnu/unistd.in.h, gnu/vasnprintf.h, gnu/verify.h, gnu/warn-on-use.h, gnu/wchar.h, gnu/wchar.in.h, gnu/xalloc-oversized.h, gnu/xalloc.h, gnu/xsize.h
- Representative functions: if, if
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 017_module_gnu_if_10.md

## module_gnu_if_11
- Module category: module_cluster
- Related headers: gnu/_Noreturn.h, gnu/alignof.h, gnu/alloca.h, gnu/alloca.in.h, gnu/arg-nonnull.h, gnu/assert.h, gnu/assert.in.h, gnu/attribute.h, gnu/basename-lgpl.h, gnu/bitrotate.h, gnu/c++defs.h, gnu/cloexec.h, gnu/errno.in.h, gnu/error.h, gnu/error.in.h, gnu/exitfail.h, gnu/fcntl.h, gnu/fcntl.in.h, gnu/fd-hook.h, gnu/filename.h, gnu/float+.h, gnu/float.in.h, gnu/getprogname.h, gnu/gettext.h, gnu/hash.h, gnu/ialloc.h, gnu/idx.h, gnu/intprops-internal.h, gnu/intprops.h, gnu/inttypes.h, gnu/inttypes.in.h, gnu/limits.h, gnu/limits.in.h, gnu/malloca.h, gnu/minmax.h, gnu/msvc-inval.h, gnu/msvc-nothrow.h, gnu/obstack.h, gnu/obstack.in.h, gnu/pathmax.h, gnu/printf-args.h, gnu/printf-parse.h, gnu/progname.h, gnu/size_max.h, gnu/stat-time.h, gnu/stat-w32.h, gnu/stdckdint.h, gnu/stdckdint.in.h, gnu/stddef.h, gnu/stddef.in.h, gnu/stdint.in.h, gnu/stdio.h, gnu/stdio.in.h, gnu/stdlib.h, gnu/stdlib.in.h, gnu/strerror-override.h, gnu/string.h, gnu/string.in.h, gnu/sys_stat.in.h, gnu/sys_types.in.h, gnu/time.h, gnu/time.in.h, gnu/unistd.h, gnu/unistd.in.h, gnu/vasnprintf.h, gnu/verify.h, gnu/warn-on-use.h, gnu/wchar.h, gnu/wchar.in.h, gnu/xalloc-oversized.h, gnu/xalloc.h, gnu/xsize.h
- Representative functions: if, if
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 018_module_gnu_if_11.md

## module_gnu_GL_ATTRIBUTE_12
- Module category: module_cluster
- Related headers: config.h, gnu/_Noreturn.h, gnu/alignof.h, gnu/alloca.h, gnu/alloca.in.h, gnu/arg-nonnull.h, gnu/assert.h, gnu/assert.in.h, gnu/attribute.h, gnu/basename-lgpl.h, gnu/bitrotate.h, gnu/c++defs.h, gnu/cloexec.h, gnu/errno.in.h, gnu/error.h, gnu/error.in.h, gnu/exitfail.h, gnu/fcntl.h, gnu/fcntl.in.h, gnu/fd-hook.h, gnu/filename.h, gnu/float+.h, gnu/float.in.h, gnu/getprogname.h, gnu/gettext.h, gnu/hash.h, gnu/ialloc.h, gnu/idx.h, gnu/intprops-internal.h, gnu/intprops.h, gnu/inttypes.h, gnu/inttypes.in.h, gnu/limits.h, gnu/limits.in.h, gnu/malloca.h, gnu/minmax.h, gnu/msvc-inval.h, gnu/msvc-nothrow.h, gnu/obstack.h, gnu/obstack.in.h, gnu/pathmax.h, gnu/printf-args.h, gnu/printf-parse.h, gnu/progname.h, gnu/size_max.h, gnu/stat-time.h, gnu/stat-w32.h, gnu/stdckdint.h, gnu/stdckdint.in.h, gnu/stddef.h, gnu/stddef.in.h, gnu/stdint.in.h, gnu/stdio.h, gnu/stdio.in.h, gnu/stdlib.h, gnu/stdlib.in.h, gnu/strerror-override.h, gnu/string.h, gnu/string.in.h, gnu/sys_stat.in.h, gnu/sys_types.in.h, gnu/time.h, gnu/time.in.h, gnu/unistd.h, gnu/unistd.in.h, gnu/vasnprintf.h, gnu/verify.h, gnu/warn-on-use.h, gnu/wchar.h, gnu/wchar.in.h, gnu/xalloc-oversized.h, gnu/xalloc.h, gnu/xsize.h
- Representative functions: _GL_ATTRIBUTE_FORMAT_PRINTF_STANDARD, _GL_ATTRIBUTE_CONST, _GL_ATTRIBUTE_CONST
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 019_module_gnu_GL_ATTRIBUTE_12.md

## module_gnu_hash_get_13
- Module category: module_cluster
- Related headers: config.h, gnu/_Noreturn.h, gnu/alignof.h, gnu/alloca.h, gnu/alloca.in.h, gnu/arg-nonnull.h, gnu/assert.h, gnu/assert.in.h, gnu/attribute.h, gnu/basename-lgpl.h, gnu/bitrotate.h, gnu/c++defs.h, gnu/cloexec.h, gnu/errno.in.h, gnu/error.h, gnu/error.in.h, gnu/exitfail.h, gnu/fcntl.h, gnu/fcntl.in.h, gnu/fd-hook.h, gnu/filename.h, gnu/float+.h, gnu/float.in.h, gnu/getprogname.h, gnu/gettext.h, gnu/hash.h, gnu/ialloc.h, gnu/idx.h, gnu/intprops-internal.h, gnu/intprops.h, gnu/inttypes.h, gnu/inttypes.in.h, gnu/limits.h, gnu/limits.in.h, gnu/malloca.h, gnu/minmax.h, gnu/msvc-inval.h, gnu/msvc-nothrow.h, gnu/obstack.h, gnu/obstack.in.h, gnu/pathmax.h, gnu/printf-args.h, gnu/printf-parse.h, gnu/progname.h, gnu/size_max.h, gnu/stat-time.h, gnu/stat-w32.h, gnu/stdckdint.h, gnu/stdckdint.in.h, gnu/stddef.h, gnu/stddef.in.h, gnu/stdint.in.h, gnu/stdio.h, gnu/stdio.in.h, gnu/stdlib.h, gnu/stdlib.in.h, gnu/strerror-override.h, gnu/string.h, gnu/string.in.h, gnu/sys_stat.in.h, gnu/sys_types.in.h, gnu/time.h, gnu/time.in.h, gnu/unistd.h, gnu/unistd.in.h, gnu/vasnprintf.h, gnu/verify.h, gnu/warn-on-use.h, gnu/wchar.h, gnu/wchar.in.h, gnu/xalloc-oversized.h, gnu/xalloc.h, gnu/xsize.h
- Representative functions: hash_get_n_buckets, hash_get_n_buckets_used, hash_get_n_entries
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 020_module_gnu_hash_get_13.md

## module_gnu_scale10_round_14
- Module category: module_cluster
- Related headers: gnu/_Noreturn.h, gnu/alignof.h, gnu/alloca.h, gnu/alloca.in.h, gnu/arg-nonnull.h, gnu/assert.h, gnu/assert.in.h, gnu/attribute.h, gnu/basename-lgpl.h, gnu/bitrotate.h, gnu/c++defs.h, gnu/cloexec.h, gnu/errno.in.h, gnu/error.h, gnu/error.in.h, gnu/exitfail.h, gnu/fcntl.h, gnu/fcntl.in.h, gnu/fd-hook.h, gnu/filename.h, gnu/float+.h, gnu/float.in.h, gnu/getprogname.h, gnu/gettext.h, gnu/hash.h, gnu/ialloc.h, gnu/idx.h, gnu/intprops-internal.h, gnu/intprops.h, gnu/inttypes.h, gnu/inttypes.in.h, gnu/limits.h, gnu/limits.in.h, gnu/malloca.h, gnu/minmax.h, gnu/msvc-inval.h, gnu/msvc-nothrow.h, gnu/obstack.h, gnu/obstack.in.h, gnu/pathmax.h, gnu/printf-args.h, gnu/printf-parse.h, gnu/progname.h, gnu/size_max.h, gnu/stat-time.h, gnu/stat-w32.h, gnu/stdckdint.h, gnu/stdckdint.in.h, gnu/stddef.h, gnu/stddef.in.h, gnu/stdint.in.h, gnu/stdio.h, gnu/stdio.in.h, gnu/stdlib.h, gnu/stdlib.in.h, gnu/strerror-override.h, gnu/string.h, gnu/string.in.h, gnu/sys_stat.in.h, gnu/sys_types.in.h, gnu/time.h, gnu/time.in.h, gnu/unistd.h, gnu/unistd.in.h, gnu/vasnprintf.h, gnu/verify.h, gnu/warn-on-use.h, gnu/wchar.h, gnu/wchar.in.h, gnu/xalloc-oversized.h, gnu/xalloc.h, gnu/xsize.h
- Representative functions: scale10_round_decimal_decoded, scale10_round_decimal_long_double, scale10_round_decimal_double
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 021_module_gnu_scale10_round_14.md

## module_gnu_execute_all_15
- Module category: module_cluster
- Related headers: config.h, gnu/_Noreturn.h, gnu/alignof.h, gnu/alloca.h, gnu/alloca.in.h, gnu/arg-nonnull.h, gnu/assert.h, gnu/assert.in.h, gnu/attribute.h, gnu/basename-lgpl.h, gnu/bitrotate.h, gnu/c++defs.h, gnu/cloexec.h, gnu/errno.in.h, gnu/error.h, gnu/error.in.h, gnu/exitfail.h, gnu/fcntl.h, gnu/fcntl.in.h, gnu/fd-hook.h, gnu/filename.h, gnu/float+.h, gnu/float.in.h, gnu/getprogname.h, gnu/gettext.h, gnu/hash.h, gnu/ialloc.h, gnu/idx.h, gnu/intprops-internal.h, gnu/intprops.h, gnu/inttypes.h, gnu/inttypes.in.h, gnu/limits.h, gnu/limits.in.h, gnu/malloca.h, gnu/minmax.h, gnu/msvc-inval.h, gnu/msvc-nothrow.h, gnu/obstack.h, gnu/obstack.in.h, gnu/pathmax.h, gnu/printf-args.h, gnu/printf-parse.h, gnu/progname.h, gnu/size_max.h, gnu/stat-time.h, gnu/stat-w32.h, gnu/stdckdint.h, gnu/stdckdint.in.h, gnu/stddef.h, gnu/stddef.in.h, gnu/stdint.in.h, gnu/stdio.h, gnu/stdio.in.h, gnu/stdlib.h, gnu/stdlib.in.h, gnu/strerror-override.h, gnu/string.h, gnu/string.in.h, gnu/sys_stat.in.h, gnu/sys_types.in.h, gnu/time.h, gnu/time.in.h, gnu/unistd.h, gnu/unistd.in.h, gnu/vasnprintf.h, gnu/verify.h, gnu/warn-on-use.h, gnu/wchar.h, gnu/wchar.in.h, gnu/xalloc-oversized.h, gnu/xalloc.h, gnu/xsize.h
- Representative functions: execute_all_close_hooks, execute_all_ioctl_hooks
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 022_module_gnu_execute_all_15.md

## module_gnu_gl_convert_16
- Module category: module_cluster
- Related headers: config.h, gnu/_Noreturn.h, gnu/alignof.h, gnu/alloca.h, gnu/alloca.in.h, gnu/arg-nonnull.h, gnu/assert.h, gnu/assert.in.h, gnu/attribute.h, gnu/basename-lgpl.h, gnu/bitrotate.h, gnu/c++defs.h, gnu/cloexec.h, gnu/errno.in.h, gnu/error.h, gnu/error.in.h, gnu/exitfail.h, gnu/fcntl.h, gnu/fcntl.in.h, gnu/fd-hook.h, gnu/filename.h, gnu/float+.h, gnu/float.in.h, gnu/getprogname.h, gnu/gettext.h, gnu/hash.h, gnu/ialloc.h, gnu/idx.h, gnu/intprops-internal.h, gnu/intprops.h, gnu/inttypes.h, gnu/inttypes.in.h, gnu/limits.h, gnu/limits.in.h, gnu/malloca.h, gnu/minmax.h, gnu/msvc-inval.h, gnu/msvc-nothrow.h, gnu/obstack.h, gnu/obstack.in.h, gnu/pathmax.h, gnu/printf-args.h, gnu/printf-parse.h, gnu/progname.h, gnu/size_max.h, gnu/stat-time.h, gnu/stat-w32.h, gnu/stdckdint.h, gnu/stdckdint.in.h, gnu/stddef.h, gnu/stddef.in.h, gnu/stdint.in.h, gnu/stdio.h, gnu/stdio.in.h, gnu/stdlib.h, gnu/stdlib.in.h, gnu/strerror-override.h, gnu/string.h, gnu/string.in.h, gnu/sys/stat.h, gnu/sys/types.h, gnu/sys_stat.in.h, gnu/sys_types.in.h, gnu/time.h, gnu/time.in.h, gnu/unistd.h, gnu/unistd.in.h, gnu/vasnprintf.h, gnu/verify.h, gnu/warn-on-use.h, gnu/wchar.h, gnu/wchar.in.h, gnu/xalloc-oversized.h, gnu/xalloc.h, gnu/xsize.h
- Representative functions: _gl_convert_FILETIME_to_timespec, _gl_convert_FILETIME_to_POSIX
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 023_module_gnu_gl_convert_16.md

## module_gnu_hash_string_17
- Module category: module_cluster
- Related headers: config.h, gnu/_Noreturn.h, gnu/alignof.h, gnu/alloca.h, gnu/alloca.in.h, gnu/arg-nonnull.h, gnu/assert.h, gnu/assert.in.h, gnu/attribute.h, gnu/basename-lgpl.h, gnu/bitrotate.h, gnu/c++defs.h, gnu/cloexec.h, gnu/errno.in.h, gnu/error.h, gnu/error.in.h, gnu/exitfail.h, gnu/fcntl.h, gnu/fcntl.in.h, gnu/fd-hook.h, gnu/filename.h, gnu/float+.h, gnu/float.in.h, gnu/getprogname.h, gnu/gettext.h, gnu/hash.h, gnu/ialloc.h, gnu/idx.h, gnu/intprops-internal.h, gnu/intprops.h, gnu/inttypes.h, gnu/inttypes.in.h, gnu/limits.h, gnu/limits.in.h, gnu/malloca.h, gnu/minmax.h, gnu/msvc-inval.h, gnu/msvc-nothrow.h, gnu/obstack.h, gnu/obstack.in.h, gnu/pathmax.h, gnu/printf-args.h, gnu/printf-parse.h, gnu/progname.h, gnu/size_max.h, gnu/stat-time.h, gnu/stat-w32.h, gnu/stdckdint.h, gnu/stdckdint.in.h, gnu/stddef.h, gnu/stddef.in.h, gnu/stdint.in.h, gnu/stdio.h, gnu/stdio.in.h, gnu/stdlib.h, gnu/stdlib.in.h, gnu/strerror-override.h, gnu/string.h, gnu/string.in.h, gnu/sys_stat.in.h, gnu/sys_types.in.h, gnu/time.h, gnu/time.in.h, gnu/unistd.h, gnu/unistd.in.h, gnu/vasnprintf.h, gnu/verify.h, gnu/warn-on-use.h, gnu/wchar.h, gnu/wchar.in.h, gnu/xalloc-oversized.h, gnu/xalloc.h, gnu/xsize.h
- Representative functions: hash_string, hash_string
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 024_module_gnu_hash_string_17.md

## module_gnu_is_infinite_18
- Module category: module_cluster
- Related headers: gnu/_Noreturn.h, gnu/alignof.h, gnu/alloca.h, gnu/alloca.in.h, gnu/arg-nonnull.h, gnu/assert.h, gnu/assert.in.h, gnu/attribute.h, gnu/basename-lgpl.h, gnu/bitrotate.h, gnu/c++defs.h, gnu/cloexec.h, gnu/errno.in.h, gnu/error.h, gnu/error.in.h, gnu/exitfail.h, gnu/fcntl.h, gnu/fcntl.in.h, gnu/fd-hook.h, gnu/filename.h, gnu/float+.h, gnu/float.in.h, gnu/getprogname.h, gnu/gettext.h, gnu/hash.h, gnu/ialloc.h, gnu/idx.h, gnu/intprops-internal.h, gnu/intprops.h, gnu/inttypes.h, gnu/inttypes.in.h, gnu/limits.h, gnu/limits.in.h, gnu/malloca.h, gnu/minmax.h, gnu/msvc-inval.h, gnu/msvc-nothrow.h, gnu/obstack.h, gnu/obstack.in.h, gnu/pathmax.h, gnu/printf-args.h, gnu/printf-parse.h, gnu/progname.h, gnu/size_max.h, gnu/stat-time.h, gnu/stat-w32.h, gnu/stdckdint.h, gnu/stdckdint.in.h, gnu/stddef.h, gnu/stddef.in.h, gnu/stdint.in.h, gnu/stdio.h, gnu/stdio.in.h, gnu/stdlib.h, gnu/stdlib.in.h, gnu/strerror-override.h, gnu/string.h, gnu/string.in.h, gnu/sys_stat.in.h, gnu/sys_types.in.h, gnu/time.h, gnu/time.in.h, gnu/unistd.h, gnu/unistd.in.h, gnu/vasnprintf.h, gnu/verify.h, gnu/warn-on-use.h, gnu/wchar.h, gnu/wchar.in.h, gnu/xalloc-oversized.h, gnu/xalloc.h, gnu/xsize.h
- Representative functions: is_infinite_or_zero, is_infinite_or_zerol
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 025_module_gnu_is_infinite_18.md

## module_gnu_rpl_fcntl_19
- Module category: module_cluster
- Related headers: config.h, gnu/_Noreturn.h, gnu/alignof.h, gnu/alloca.h, gnu/alloca.in.h, gnu/arg-nonnull.h, gnu/assert.h, gnu/assert.in.h, gnu/attribute.h, gnu/basename-lgpl.h, gnu/bitrotate.h, gnu/c++defs.h, gnu/cloexec.h, gnu/errno.in.h, gnu/error.h, gnu/error.in.h, gnu/exitfail.h, gnu/fcntl.h, gnu/fcntl.in.h, gnu/fd-hook.h, gnu/filename.h, gnu/float+.h, gnu/float.in.h, gnu/getprogname.h, gnu/gettext.h, gnu/hash.h, gnu/ialloc.h, gnu/idx.h, gnu/intprops-internal.h, gnu/intprops.h, gnu/inttypes.h, gnu/inttypes.in.h, gnu/limits.h, gnu/limits.in.h, gnu/malloca.h, gnu/minmax.h, gnu/msvc-inval.h, gnu/msvc-nothrow.h, gnu/obstack.h, gnu/obstack.in.h, gnu/pathmax.h, gnu/printf-args.h, gnu/printf-parse.h, gnu/progname.h, gnu/size_max.h, gnu/stat-time.h, gnu/stat-w32.h, gnu/stdckdint.h, gnu/stdckdint.in.h, gnu/stddef.h, gnu/stddef.in.h, gnu/stdint.in.h, gnu/stdio.h, gnu/stdio.in.h, gnu/stdlib.h, gnu/stdlib.in.h, gnu/strerror-override.h, gnu/string.h, gnu/string.in.h, gnu/sys_stat.in.h, gnu/sys_types.in.h, gnu/time.h, gnu/time.in.h, gnu/unistd.h, gnu/unistd.in.h, gnu/vasnprintf.h, gnu/verify.h, gnu/warn-on-use.h, gnu/wchar.h, gnu/wchar.in.h, gnu/xalloc-oversized.h, gnu/xalloc.h, gnu/xsize.h
- Representative functions: rpl_fcntl_DUPFD, rpl_fcntl_DUPFD_CLOEXEC
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 026_module_gnu_rpl_fcntl_19.md

## module_gnu_asnprintf.c_20
- Module category: module_cluster
- Related headers: config.h, gnu/_Noreturn.h, gnu/alignof.h, gnu/alloca.h, gnu/alloca.in.h, gnu/arg-nonnull.h, gnu/assert.h, gnu/assert.in.h, gnu/attribute.h, gnu/basename-lgpl.h, gnu/bitrotate.h, gnu/c++defs.h, gnu/cloexec.h, gnu/errno.in.h, gnu/error.h, gnu/error.in.h, gnu/exitfail.h, gnu/fcntl.h, gnu/fcntl.in.h, gnu/fd-hook.h, gnu/filename.h, gnu/float+.h, gnu/float.in.h, gnu/getprogname.h, gnu/gettext.h, gnu/hash.h, gnu/ialloc.h, gnu/idx.h, gnu/intprops-internal.h, gnu/intprops.h, gnu/inttypes.h, gnu/inttypes.in.h, gnu/limits.h, gnu/limits.in.h, gnu/malloca.h, gnu/minmax.h, gnu/msvc-inval.h, gnu/msvc-nothrow.h, gnu/obstack.h, gnu/obstack.in.h, gnu/pathmax.h, gnu/printf-args.h, gnu/printf-parse.h, gnu/progname.h, gnu/size_max.h, gnu/stat-time.h, gnu/stat-w32.h, gnu/stdckdint.h, gnu/stdckdint.in.h, gnu/stddef.h, gnu/stddef.in.h, gnu/stdint.in.h, gnu/stdio.h, gnu/stdio.in.h, gnu/stdlib.h, gnu/stdlib.in.h, gnu/strerror-override.h, gnu/string.h, gnu/string.in.h, gnu/sys_stat.in.h, gnu/sys_types.in.h, gnu/time.h, gnu/time.in.h, gnu/unistd.h, gnu/unistd.in.h, gnu/vasnprintf.h, gnu/verify.h, gnu/warn-on-use.h, gnu/wchar.h, gnu/wchar.in.h, gnu/xalloc-oversized.h, gnu/xalloc.h, gnu/xsize.h
- Representative functions: asnprintf
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 027_module_gnu_asnprintf.c_20.md

## module_gnu_basename-lgpl.c_21
- Module category: module_cluster
- Related headers: config.h, gnu/_Noreturn.h, gnu/alignof.h, gnu/alloca.h, gnu/alloca.in.h, gnu/arg-nonnull.h, gnu/assert.h, gnu/assert.in.h, gnu/attribute.h, gnu/basename-lgpl.h, gnu/bitrotate.h, gnu/c++defs.h, gnu/cloexec.h, gnu/errno.in.h, gnu/error.h, gnu/error.in.h, gnu/exitfail.h, gnu/fcntl.h, gnu/fcntl.in.h, gnu/fd-hook.h, gnu/filename.h, gnu/float+.h, gnu/float.in.h, gnu/getprogname.h, gnu/gettext.h, gnu/hash.h, gnu/ialloc.h, gnu/idx.h, gnu/intprops-internal.h, gnu/intprops.h, gnu/inttypes.h, gnu/inttypes.in.h, gnu/limits.h, gnu/limits.in.h, gnu/malloca.h, gnu/minmax.h, gnu/msvc-inval.h, gnu/msvc-nothrow.h, gnu/obstack.h, gnu/obstack.in.h, gnu/pathmax.h, gnu/printf-args.h, gnu/printf-parse.h, gnu/progname.h, gnu/size_max.h, gnu/stat-time.h, gnu/stat-w32.h, gnu/stdckdint.h, gnu/stdckdint.in.h, gnu/stddef.h, gnu/stddef.in.h, gnu/stdint.in.h, gnu/stdio.h, gnu/stdio.in.h, gnu/stdlib.h, gnu/stdlib.in.h, gnu/strerror-override.h, gnu/string.h, gnu/string.in.h, gnu/sys_stat.in.h, gnu/sys_types.in.h, gnu/time.h, gnu/time.in.h, gnu/unistd.h, gnu/unistd.in.h, gnu/vasnprintf.h, gnu/verify.h, gnu/warn-on-use.h, gnu/wchar.h, gnu/wchar.in.h, gnu/xalloc-oversized.h, gnu/xalloc.h, gnu/xsize.h
- Representative functions: last_component, base_len
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 028_module_gnu_basename-lgpl.c_21.md

## module_gnu_calloc.c_22
- Module category: module_cluster
- Related headers: config.h, gnu/_Noreturn.h, gnu/alignof.h, gnu/alloca.h, gnu/alloca.in.h, gnu/arg-nonnull.h, gnu/assert.h, gnu/assert.in.h, gnu/attribute.h, gnu/basename-lgpl.h, gnu/bitrotate.h, gnu/c++defs.h, gnu/cloexec.h, gnu/errno.in.h, gnu/error.h, gnu/error.in.h, gnu/exitfail.h, gnu/fcntl.h, gnu/fcntl.in.h, gnu/fd-hook.h, gnu/filename.h, gnu/float+.h, gnu/float.in.h, gnu/getprogname.h, gnu/gettext.h, gnu/hash.h, gnu/ialloc.h, gnu/idx.h, gnu/intprops-internal.h, gnu/intprops.h, gnu/inttypes.h, gnu/inttypes.in.h, gnu/limits.h, gnu/limits.in.h, gnu/malloca.h, gnu/minmax.h, gnu/msvc-inval.h, gnu/msvc-nothrow.h, gnu/obstack.h, gnu/obstack.in.h, gnu/pathmax.h, gnu/printf-args.h, gnu/printf-parse.h, gnu/progname.h, gnu/size_max.h, gnu/stat-time.h, gnu/stat-w32.h, gnu/stdckdint.h, gnu/stdckdint.in.h, gnu/stddef.h, gnu/stddef.in.h, gnu/stdint.in.h, gnu/stdio.h, gnu/stdio.in.h, gnu/stdlib.h, gnu/stdlib.in.h, gnu/strerror-override.h, gnu/string.h, gnu/string.in.h, gnu/sys_stat.in.h, gnu/sys_types.in.h, gnu/time.h, gnu/time.in.h, gnu/unistd.h, gnu/unistd.in.h, gnu/vasnprintf.h, gnu/verify.h, gnu/warn-on-use.h, gnu/wchar.h, gnu/wchar.in.h, gnu/xalloc-oversized.h, gnu/xalloc.h, gnu/xsize.h
- Representative functions: rpl_calloc
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 029_module_gnu_calloc.c_22.md

## module_gnu_cloexec.c_23
- Module category: module_cluster
- Related headers: config.h, gnu/_Noreturn.h, gnu/alignof.h, gnu/alloca.h, gnu/alloca.in.h, gnu/arg-nonnull.h, gnu/assert.h, gnu/assert.in.h, gnu/attribute.h, gnu/basename-lgpl.h, gnu/bitrotate.h, gnu/c++defs.h, gnu/cloexec.h, gnu/errno.in.h, gnu/error.h, gnu/error.in.h, gnu/exitfail.h, gnu/fcntl.h, gnu/fcntl.in.h, gnu/fd-hook.h, gnu/filename.h, gnu/float+.h, gnu/float.in.h, gnu/getprogname.h, gnu/gettext.h, gnu/hash.h, gnu/ialloc.h, gnu/idx.h, gnu/intprops-internal.h, gnu/intprops.h, gnu/inttypes.h, gnu/inttypes.in.h, gnu/limits.h, gnu/limits.in.h, gnu/malloca.h, gnu/minmax.h, gnu/msvc-inval.h, gnu/msvc-nothrow.h, gnu/obstack.h, gnu/obstack.in.h, gnu/pathmax.h, gnu/printf-args.h, gnu/printf-parse.h, gnu/progname.h, gnu/size_max.h, gnu/stat-time.h, gnu/stat-w32.h, gnu/stdckdint.h, gnu/stdckdint.in.h, gnu/stddef.h, gnu/stddef.in.h, gnu/stdint.in.h, gnu/stdio.h, gnu/stdio.in.h, gnu/stdlib.h, gnu/stdlib.in.h, gnu/strerror-override.h, gnu/string.h, gnu/string.in.h, gnu/sys_stat.in.h, gnu/sys_types.in.h, gnu/time.h, gnu/time.in.h, gnu/unistd.h, gnu/unistd.in.h, gnu/vasnprintf.h, gnu/verify.h, gnu/warn-on-use.h, gnu/wchar.h, gnu/wchar.in.h, gnu/xalloc-oversized.h, gnu/xalloc.h, gnu/xsize.h
- Representative functions: set_cloexec_flag, dup_cloexec
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 030_module_gnu_cloexec.c_23.md

## module_gnu_close.c_24
- Module category: module_cluster
- Related headers: config.h, gnu/_Noreturn.h, gnu/alignof.h, gnu/alloca.h, gnu/alloca.in.h, gnu/arg-nonnull.h, gnu/assert.h, gnu/assert.in.h, gnu/attribute.h, gnu/basename-lgpl.h, gnu/bitrotate.h, gnu/c++defs.h, gnu/cloexec.h, gnu/errno.in.h, gnu/error.h, gnu/error.in.h, gnu/exitfail.h, gnu/fcntl.h, gnu/fcntl.in.h, gnu/fd-hook.h, gnu/filename.h, gnu/float+.h, gnu/float.in.h, gnu/getprogname.h, gnu/gettext.h, gnu/hash.h, gnu/ialloc.h, gnu/idx.h, gnu/intprops-internal.h, gnu/intprops.h, gnu/inttypes.h, gnu/inttypes.in.h, gnu/limits.h, gnu/limits.in.h, gnu/malloca.h, gnu/minmax.h, gnu/msvc-inval.h, gnu/msvc-nothrow.h, gnu/obstack.h, gnu/obstack.in.h, gnu/pathmax.h, gnu/printf-args.h, gnu/printf-parse.h, gnu/progname.h, gnu/size_max.h, gnu/stat-time.h, gnu/stat-w32.h, gnu/stdckdint.h, gnu/stdckdint.in.h, gnu/stddef.h, gnu/stddef.in.h, gnu/stdint.in.h, gnu/stdio.h, gnu/stdio.in.h, gnu/stdlib.h, gnu/stdlib.in.h, gnu/strerror-override.h, gnu/string.h, gnu/string.in.h, gnu/sys_stat.in.h, gnu/sys_types.in.h, gnu/time.h, gnu/time.in.h, gnu/unistd.h, gnu/unistd.in.h, gnu/vasnprintf.h, gnu/verify.h, gnu/warn-on-use.h, gnu/wchar.h, gnu/wchar.in.h, gnu/xalloc-oversized.h, gnu/xalloc.h, gnu/xsize.h
- Representative functions: close_nothrow, rpl_close
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 031_module_gnu_close.c_24.md

## module_gnu_dup2.c_25
- Module category: module_cluster
- Related headers: config.h, gnu/_Noreturn.h, gnu/alignof.h, gnu/alloca.h, gnu/alloca.in.h, gnu/arg-nonnull.h, gnu/assert.h, gnu/assert.in.h, gnu/attribute.h, gnu/basename-lgpl.h, gnu/bitrotate.h, gnu/c++defs.h, gnu/cloexec.h, gnu/errno.in.h, gnu/error.h, gnu/error.in.h, gnu/exitfail.h, gnu/fcntl.h, gnu/fcntl.in.h, gnu/fd-hook.h, gnu/filename.h, gnu/float+.h, gnu/float.in.h, gnu/getprogname.h, gnu/gettext.h, gnu/hash.h, gnu/ialloc.h, gnu/idx.h, gnu/intprops-internal.h, gnu/intprops.h, gnu/inttypes.h, gnu/inttypes.in.h, gnu/limits.h, gnu/limits.in.h, gnu/malloca.h, gnu/minmax.h, gnu/msvc-inval.h, gnu/msvc-nothrow.h, gnu/obstack.h, gnu/obstack.in.h, gnu/pathmax.h, gnu/printf-args.h, gnu/printf-parse.h, gnu/progname.h, gnu/size_max.h, gnu/stat-time.h, gnu/stat-w32.h, gnu/stdckdint.h, gnu/stdckdint.in.h, gnu/stddef.h, gnu/stddef.in.h, gnu/stdint.in.h, gnu/stdio.h, gnu/stdio.in.h, gnu/stdlib.h, gnu/stdlib.in.h, gnu/strerror-override.h, gnu/string.h, gnu/string.in.h, gnu/sys_stat.in.h, gnu/sys_types.in.h, gnu/time.h, gnu/time.in.h, gnu/unistd.h, gnu/unistd.in.h, gnu/vasnprintf.h, gnu/verify.h, gnu/warn-on-use.h, gnu/wchar.h, gnu/wchar.in.h, gnu/xalloc-oversized.h, gnu/xalloc.h, gnu/xsize.h
- Representative functions: dup2_nothrow, ms_windows_dup2, klibc_dup2dirfd, rpl_dup2
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 032_module_gnu_dup2.c_25.md

## module_gnu_error.c_26
- Module category: module_cluster
- Related headers: gnu/_Noreturn.h, gnu/alignof.h, gnu/alloca.h, gnu/alloca.in.h, gnu/arg-nonnull.h, gnu/assert.h, gnu/assert.in.h, gnu/attribute.h, gnu/basename-lgpl.h, gnu/bitrotate.h, gnu/c++defs.h, gnu/cloexec.h, gnu/errno.in.h, gnu/error.h, gnu/error.in.h, gnu/exitfail.h, gnu/fcntl.h, gnu/fcntl.in.h, gnu/fd-hook.h, gnu/filename.h, gnu/float+.h, gnu/float.in.h, gnu/getprogname.h, gnu/gettext.h, gnu/hash.h, gnu/ialloc.h, gnu/idx.h, gnu/intprops-internal.h, gnu/intprops.h, gnu/inttypes.h, gnu/inttypes.in.h, gnu/limits.h, gnu/limits.in.h, gnu/malloca.h, gnu/minmax.h, gnu/msvc-inval.h, gnu/msvc-nothrow.h, gnu/obstack.h, gnu/obstack.in.h, gnu/pathmax.h, gnu/printf-args.h, gnu/printf-parse.h, gnu/progname.h, gnu/size_max.h, gnu/stat-time.h, gnu/stat-w32.h, gnu/stdckdint.h, gnu/stdckdint.in.h, gnu/stddef.h, gnu/stddef.in.h, gnu/stdint.in.h, gnu/stdio.h, gnu/stdio.in.h, gnu/stdlib.h, gnu/stdlib.in.h, gnu/strerror-override.h, gnu/string.h, gnu/string.in.h, gnu/sys_stat.in.h, gnu/sys_types.in.h, gnu/time.h, gnu/time.in.h, gnu/unistd.h, gnu/unistd.in.h, gnu/vasnprintf.h, gnu/verify.h, gnu/warn-on-use.h, gnu/wchar.h, gnu/wchar.in.h, gnu/xalloc-oversized.h, gnu/xalloc.h, gnu/xsize.h
- Representative functions: is_open, flush_stdout, print_errno_message, error, error_at_line
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 033_module_gnu_error.c_26.md

## module_gnu_fcntl.c_27
- Module category: module_cluster
- Related headers: config.h, gnu/_Noreturn.h, gnu/alignof.h, gnu/alloca.h, gnu/alloca.in.h, gnu/arg-nonnull.h, gnu/assert.h, gnu/assert.in.h, gnu/attribute.h, gnu/basename-lgpl.h, gnu/bitrotate.h, gnu/c++defs.h, gnu/cloexec.h, gnu/errno.in.h, gnu/error.h, gnu/error.in.h, gnu/exitfail.h, gnu/fcntl.h, gnu/fcntl.in.h, gnu/fd-hook.h, gnu/filename.h, gnu/float+.h, gnu/float.in.h, gnu/getprogname.h, gnu/gettext.h, gnu/hash.h, gnu/ialloc.h, gnu/idx.h, gnu/intprops-internal.h, gnu/intprops.h, gnu/inttypes.h, gnu/inttypes.in.h, gnu/limits.h, gnu/limits.in.h, gnu/malloca.h, gnu/minmax.h, gnu/msvc-inval.h, gnu/msvc-nothrow.h, gnu/obstack.h, gnu/obstack.in.h, gnu/pathmax.h, gnu/printf-args.h, gnu/printf-parse.h, gnu/progname.h, gnu/size_max.h, gnu/stat-time.h, gnu/stat-w32.h, gnu/stdckdint.h, gnu/stdckdint.in.h, gnu/stddef.h, gnu/stddef.in.h, gnu/stdint.in.h, gnu/stdio.h, gnu/stdio.in.h, gnu/stdlib.h, gnu/stdlib.in.h, gnu/strerror-override.h, gnu/string.h, gnu/string.in.h, gnu/sys_stat.in.h, gnu/sys_types.in.h, gnu/time.h, gnu/time.in.h, gnu/unistd.h, gnu/unistd.in.h, gnu/vasnprintf.h, gnu/verify.h, gnu/warn-on-use.h, gnu/wchar.h, gnu/wchar.in.h, gnu/xalloc-oversized.h, gnu/xalloc.h, gnu/xsize.h
- Representative functions: dupfd
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 034_module_gnu_fcntl.c_27.md

## module_gnu_free.c_28
- Module category: module_cluster
- Related headers: config.h, gnu/_Noreturn.h, gnu/alignof.h, gnu/alloca.h, gnu/alloca.in.h, gnu/arg-nonnull.h, gnu/assert.h, gnu/assert.in.h, gnu/attribute.h, gnu/basename-lgpl.h, gnu/bitrotate.h, gnu/c++defs.h, gnu/cloexec.h, gnu/errno.in.h, gnu/error.h, gnu/error.in.h, gnu/exitfail.h, gnu/fcntl.h, gnu/fcntl.in.h, gnu/fd-hook.h, gnu/filename.h, gnu/float+.h, gnu/float.in.h, gnu/getprogname.h, gnu/gettext.h, gnu/hash.h, gnu/ialloc.h, gnu/idx.h, gnu/intprops-internal.h, gnu/intprops.h, gnu/inttypes.h, gnu/inttypes.in.h, gnu/limits.h, gnu/limits.in.h, gnu/malloca.h, gnu/minmax.h, gnu/msvc-inval.h, gnu/msvc-nothrow.h, gnu/obstack.h, gnu/obstack.in.h, gnu/pathmax.h, gnu/printf-args.h, gnu/printf-parse.h, gnu/progname.h, gnu/size_max.h, gnu/stat-time.h, gnu/stat-w32.h, gnu/stdckdint.h, gnu/stdckdint.in.h, gnu/stddef.h, gnu/stddef.in.h, gnu/stdint.in.h, gnu/stdio.h, gnu/stdio.in.h, gnu/stdlib.h, gnu/stdlib.in.h, gnu/strerror-override.h, gnu/string.h, gnu/string.in.h, gnu/sys_stat.in.h, gnu/sys_types.in.h, gnu/time.h, gnu/time.in.h, gnu/unistd.h, gnu/unistd.in.h, gnu/vasnprintf.h, gnu/verify.h, gnu/warn-on-use.h, gnu/wchar.h, gnu/wchar.in.h, gnu/xalloc-oversized.h, gnu/xalloc.h, gnu/xsize.h
- Representative functions: rpl_free
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 035_module_gnu_free.c_28.md

## module_gnu_getdtablesize.c_29
- Module category: module_cluster
- Related headers: config.h, gnu/_Noreturn.h, gnu/alignof.h, gnu/alloca.h, gnu/alloca.in.h, gnu/arg-nonnull.h, gnu/assert.h, gnu/assert.in.h, gnu/attribute.h, gnu/basename-lgpl.h, gnu/bitrotate.h, gnu/c++defs.h, gnu/cloexec.h, gnu/errno.in.h, gnu/error.h, gnu/error.in.h, gnu/exitfail.h, gnu/fcntl.h, gnu/fcntl.in.h, gnu/fd-hook.h, gnu/filename.h, gnu/float+.h, gnu/float.in.h, gnu/getprogname.h, gnu/gettext.h, gnu/hash.h, gnu/ialloc.h, gnu/idx.h, gnu/intprops-internal.h, gnu/intprops.h, gnu/inttypes.h, gnu/inttypes.in.h, gnu/limits.h, gnu/limits.in.h, gnu/malloca.h, gnu/minmax.h, gnu/msvc-inval.h, gnu/msvc-nothrow.h, gnu/obstack.h, gnu/obstack.in.h, gnu/pathmax.h, gnu/printf-args.h, gnu/printf-parse.h, gnu/progname.h, gnu/size_max.h, gnu/stat-time.h, gnu/stat-w32.h, gnu/stdckdint.h, gnu/stdckdint.in.h, gnu/stddef.h, gnu/stddef.in.h, gnu/stdint.in.h, gnu/stdio.h, gnu/stdio.in.h, gnu/stdlib.h, gnu/stdlib.in.h, gnu/strerror-override.h, gnu/string.h, gnu/string.in.h, gnu/sys_stat.in.h, gnu/sys_types.in.h, gnu/time.h, gnu/time.in.h, gnu/unistd.h, gnu/unistd.in.h, gnu/vasnprintf.h, gnu/verify.h, gnu/warn-on-use.h, gnu/wchar.h, gnu/wchar.in.h, gnu/xalloc-oversized.h, gnu/xalloc.h, gnu/xsize.h
- Representative functions: _setmaxstdio_nothrow
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 036_module_gnu_getdtablesize.c_29.md

## module_gnu_getprogname.c_30
- Module category: module_cluster
- Related headers: config.h, gnu/_Noreturn.h, gnu/alignof.h, gnu/alloca.h, gnu/alloca.in.h, gnu/arg-nonnull.h, gnu/assert.h, gnu/assert.in.h, gnu/attribute.h, gnu/basename-lgpl.h, gnu/bitrotate.h, gnu/c++defs.h, gnu/cloexec.h, gnu/errno.in.h, gnu/error.h, gnu/error.in.h, gnu/exitfail.h, gnu/fcntl.h, gnu/fcntl.in.h, gnu/fd-hook.h, gnu/filename.h, gnu/float+.h, gnu/float.in.h, gnu/getprogname.h, gnu/gettext.h, gnu/hash.h, gnu/ialloc.h, gnu/idx.h, gnu/intprops-internal.h, gnu/intprops.h, gnu/inttypes.h, gnu/inttypes.in.h, gnu/limits.h, gnu/limits.in.h, gnu/malloca.h, gnu/minmax.h, gnu/msvc-inval.h, gnu/msvc-nothrow.h, gnu/obstack.h, gnu/obstack.in.h, gnu/pathmax.h, gnu/printf-args.h, gnu/printf-parse.h, gnu/progname.h, gnu/size_max.h, gnu/stat-time.h, gnu/stat-w32.h, gnu/stdckdint.h, gnu/stdckdint.in.h, gnu/stddef.h, gnu/stddef.in.h, gnu/stdint.in.h, gnu/stdio.h, gnu/stdio.in.h, gnu/stdlib.h, gnu/stdlib.in.h, gnu/strerror-override.h, gnu/string.h, gnu/string.in.h, gnu/sys_stat.in.h, gnu/sys_types.in.h, gnu/time.h, gnu/time.in.h, gnu/unistd.h, gnu/unistd.in.h, gnu/vasnprintf.h, gnu/verify.h, gnu/warn-on-use.h, gnu/wchar.h, gnu/wchar.in.h, gnu/xalloc-oversized.h, gnu/xalloc.h, gnu/xsize.h
- Representative functions: getprogname
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 037_module_gnu_getprogname.c_30.md

## module_gnu_hash.c_31
- Module category: module_cluster
- Related headers: config.h, gnu/_Noreturn.h, gnu/alignof.h, gnu/alloca.h, gnu/alloca.in.h, gnu/arg-nonnull.h, gnu/assert.h, gnu/assert.in.h, gnu/attribute.h, gnu/basename-lgpl.h, gnu/bitrotate.h, gnu/c++defs.h, gnu/cloexec.h, gnu/errno.in.h, gnu/error.h, gnu/error.in.h, gnu/exitfail.h, gnu/fcntl.h, gnu/fcntl.in.h, gnu/fd-hook.h, gnu/filename.h, gnu/float+.h, gnu/float.in.h, gnu/getprogname.h, gnu/gettext.h, gnu/hash.h, gnu/ialloc.h, gnu/idx.h, gnu/intprops-internal.h, gnu/intprops.h, gnu/inttypes.h, gnu/inttypes.in.h, gnu/limits.h, gnu/limits.in.h, gnu/malloca.h, gnu/minmax.h, gnu/msvc-inval.h, gnu/msvc-nothrow.h, gnu/obstack.h, gnu/obstack.in.h, gnu/pathmax.h, gnu/printf-args.h, gnu/printf-parse.h, gnu/progname.h, gnu/size_max.h, gnu/stat-time.h, gnu/stat-w32.h, gnu/stdckdint.h, gnu/stdckdint.in.h, gnu/stddef.h, gnu/stddef.in.h, gnu/stdint.in.h, gnu/stdio.h, gnu/stdio.in.h, gnu/stdlib.h, gnu/stdlib.in.h, gnu/strerror-override.h, gnu/string.h, gnu/string.in.h, gnu/sys_stat.in.h, gnu/sys_types.in.h, gnu/time.h, gnu/time.in.h, gnu/unistd.h, gnu/unistd.in.h, gnu/vasnprintf.h, gnu/verify.h, gnu/warn-on-use.h, gnu/wchar.h, gnu/wchar.in.h, gnu/xalloc-oversized.h, gnu/xalloc.h, gnu/xsize.h
- Representative functions: hash_print_statistics, hash_reset_tuning, raw_hasher, raw_comparator, check_tuning, hash_initialize, hash_rehash, hash_insert
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 038_module_gnu_hash.c_31.md

## module_gnu_itold.c_32
- Module category: module_cluster
- Related headers: config.h, gnu/_Noreturn.h, gnu/alignof.h, gnu/alloca.h, gnu/alloca.in.h, gnu/arg-nonnull.h, gnu/assert.h, gnu/assert.in.h, gnu/attribute.h, gnu/basename-lgpl.h, gnu/bitrotate.h, gnu/c++defs.h, gnu/cloexec.h, gnu/errno.in.h, gnu/error.h, gnu/error.in.h, gnu/exitfail.h, gnu/fcntl.h, gnu/fcntl.in.h, gnu/fd-hook.h, gnu/filename.h, gnu/float+.h, gnu/float.in.h, gnu/getprogname.h, gnu/gettext.h, gnu/hash.h, gnu/ialloc.h, gnu/idx.h, gnu/intprops-internal.h, gnu/intprops.h, gnu/inttypes.h, gnu/inttypes.in.h, gnu/limits.h, gnu/limits.in.h, gnu/malloca.h, gnu/minmax.h, gnu/msvc-inval.h, gnu/msvc-nothrow.h, gnu/obstack.h, gnu/obstack.in.h, gnu/pathmax.h, gnu/printf-args.h, gnu/printf-parse.h, gnu/progname.h, gnu/size_max.h, gnu/stat-time.h, gnu/stat-w32.h, gnu/stdckdint.h, gnu/stdckdint.in.h, gnu/stddef.h, gnu/stddef.in.h, gnu/stdint.in.h, gnu/stdio.h, gnu/stdio.in.h, gnu/stdlib.h, gnu/stdlib.in.h, gnu/strerror-override.h, gnu/string.h, gnu/string.in.h, gnu/sys_stat.in.h, gnu/sys_types.in.h, gnu/time.h, gnu/time.in.h, gnu/unistd.h, gnu/unistd.in.h, gnu/vasnprintf.h, gnu/verify.h, gnu/warn-on-use.h, gnu/wchar.h, gnu/wchar.in.h, gnu/xalloc-oversized.h, gnu/xalloc.h, gnu/xsize.h
- Representative functions: _Qp_itoq
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 039_module_gnu_itold.c_32.md

## module_gnu_malloc.c_33
- Module category: module_cluster
- Related headers: config.h, gnu/_Noreturn.h, gnu/alignof.h, gnu/alloca.h, gnu/alloca.in.h, gnu/arg-nonnull.h, gnu/assert.h, gnu/assert.in.h, gnu/attribute.h, gnu/basename-lgpl.h, gnu/bitrotate.h, gnu/c++defs.h, gnu/cloexec.h, gnu/errno.in.h, gnu/error.h, gnu/error.in.h, gnu/exitfail.h, gnu/fcntl.h, gnu/fcntl.in.h, gnu/fd-hook.h, gnu/filename.h, gnu/float+.h, gnu/float.in.h, gnu/getprogname.h, gnu/gettext.h, gnu/hash.h, gnu/ialloc.h, gnu/idx.h, gnu/intprops-internal.h, gnu/intprops.h, gnu/inttypes.h, gnu/inttypes.in.h, gnu/limits.h, gnu/limits.in.h, gnu/malloca.h, gnu/minmax.h, gnu/msvc-inval.h, gnu/msvc-nothrow.h, gnu/obstack.h, gnu/obstack.in.h, gnu/pathmax.h, gnu/printf-args.h, gnu/printf-parse.h, gnu/progname.h, gnu/size_max.h, gnu/stat-time.h, gnu/stat-w32.h, gnu/stdckdint.h, gnu/stdckdint.in.h, gnu/stddef.h, gnu/stddef.in.h, gnu/stdint.in.h, gnu/stdio.h, gnu/stdio.in.h, gnu/stdlib.h, gnu/stdlib.in.h, gnu/strerror-override.h, gnu/string.h, gnu/string.in.h, gnu/sys_stat.in.h, gnu/sys_types.in.h, gnu/time.h, gnu/time.in.h, gnu/unistd.h, gnu/unistd.in.h, gnu/vasnprintf.h, gnu/verify.h, gnu/warn-on-use.h, gnu/wchar.h, gnu/wchar.in.h, gnu/xalloc-oversized.h, gnu/xalloc.h, gnu/xsize.h
- Representative functions: rpl_malloc
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 040_module_gnu_malloc.c_33.md

## module_gnu_malloca.c_34
- Module category: module_cluster
- Related headers: config.h, gnu/_Noreturn.h, gnu/alignof.h, gnu/alloca.h, gnu/alloca.in.h, gnu/arg-nonnull.h, gnu/assert.h, gnu/assert.in.h, gnu/attribute.h, gnu/basename-lgpl.h, gnu/bitrotate.h, gnu/c++defs.h, gnu/cloexec.h, gnu/errno.in.h, gnu/error.h, gnu/error.in.h, gnu/exitfail.h, gnu/fcntl.h, gnu/fcntl.in.h, gnu/fd-hook.h, gnu/filename.h, gnu/float+.h, gnu/float.in.h, gnu/getprogname.h, gnu/gettext.h, gnu/hash.h, gnu/ialloc.h, gnu/idx.h, gnu/intprops-internal.h, gnu/intprops.h, gnu/inttypes.h, gnu/inttypes.in.h, gnu/limits.h, gnu/limits.in.h, gnu/malloca.h, gnu/minmax.h, gnu/msvc-inval.h, gnu/msvc-nothrow.h, gnu/obstack.h, gnu/obstack.in.h, gnu/pathmax.h, gnu/printf-args.h, gnu/printf-parse.h, gnu/progname.h, gnu/size_max.h, gnu/stat-time.h, gnu/stat-w32.h, gnu/stdckdint.h, gnu/stdckdint.in.h, gnu/stddef.h, gnu/stddef.in.h, gnu/stdint.in.h, gnu/stdio.h, gnu/stdio.in.h, gnu/stdlib.h, gnu/stdlib.in.h, gnu/strerror-override.h, gnu/string.h, gnu/string.in.h, gnu/sys_stat.in.h, gnu/sys_types.in.h, gnu/time.h, gnu/time.in.h, gnu/unistd.h, gnu/unistd.in.h, gnu/vasnprintf.h, gnu/verify.h, gnu/warn-on-use.h, gnu/wchar.h, gnu/wchar.in.h, gnu/xalloc-oversized.h, gnu/xalloc.h, gnu/xsize.h
- Representative functions: mmalloca, freea
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 041_module_gnu_malloca.c_34.md

## module_gnu_memchr.c_35
- Module category: module_cluster
- Related headers: gnu/_Noreturn.h, gnu/alignof.h, gnu/alloca.h, gnu/alloca.in.h, gnu/arg-nonnull.h, gnu/assert.h, gnu/assert.in.h, gnu/attribute.h, gnu/basename-lgpl.h, gnu/bitrotate.h, gnu/c++defs.h, gnu/cloexec.h, gnu/errno.in.h, gnu/error.h, gnu/error.in.h, gnu/exitfail.h, gnu/fcntl.h, gnu/fcntl.in.h, gnu/fd-hook.h, gnu/filename.h, gnu/float+.h, gnu/float.in.h, gnu/getprogname.h, gnu/gettext.h, gnu/hash.h, gnu/ialloc.h, gnu/idx.h, gnu/intprops-internal.h, gnu/intprops.h, gnu/inttypes.h, gnu/inttypes.in.h, gnu/limits.h, gnu/limits.in.h, gnu/malloca.h, gnu/minmax.h, gnu/msvc-inval.h, gnu/msvc-nothrow.h, gnu/obstack.h, gnu/obstack.in.h, gnu/pathmax.h, gnu/printf-args.h, gnu/printf-parse.h, gnu/progname.h, gnu/size_max.h, gnu/stat-time.h, gnu/stat-w32.h, gnu/stdckdint.h, gnu/stdckdint.in.h, gnu/stddef.h, gnu/stddef.in.h, gnu/stdint.in.h, gnu/stdio.h, gnu/stdio.in.h, gnu/stdlib.h, gnu/stdlib.in.h, gnu/strerror-override.h, gnu/string.h, gnu/string.in.h, gnu/sys_stat.in.h, gnu/sys_types.in.h, gnu/time.h, gnu/time.in.h, gnu/unistd.h, gnu/unistd.in.h, gnu/vasnprintf.h, gnu/verify.h, gnu/warn-on-use.h, gnu/wchar.h, gnu/wchar.in.h, gnu/xalloc-oversized.h, gnu/xalloc.h, gnu/xsize.h
- Representative functions: __memchr
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 042_module_gnu_memchr.c_35.md

## module_gnu_msvc-inval.c_36
- Module category: module_cluster
- Related headers: config.h, gnu/_Noreturn.h, gnu/alignof.h, gnu/alloca.h, gnu/alloca.in.h, gnu/arg-nonnull.h, gnu/assert.h, gnu/assert.in.h, gnu/attribute.h, gnu/basename-lgpl.h, gnu/bitrotate.h, gnu/c++defs.h, gnu/cloexec.h, gnu/errno.in.h, gnu/error.h, gnu/error.in.h, gnu/exitfail.h, gnu/fcntl.h, gnu/fcntl.in.h, gnu/fd-hook.h, gnu/filename.h, gnu/float+.h, gnu/float.in.h, gnu/getprogname.h, gnu/gettext.h, gnu/hash.h, gnu/ialloc.h, gnu/idx.h, gnu/intprops-internal.h, gnu/intprops.h, gnu/inttypes.h, gnu/inttypes.in.h, gnu/limits.h, gnu/limits.in.h, gnu/malloca.h, gnu/minmax.h, gnu/msvc-inval.h, gnu/msvc-nothrow.h, gnu/obstack.h, gnu/obstack.in.h, gnu/pathmax.h, gnu/printf-args.h, gnu/printf-parse.h, gnu/progname.h, gnu/size_max.h, gnu/stat-time.h, gnu/stat-w32.h, gnu/stdckdint.h, gnu/stdckdint.in.h, gnu/stddef.h, gnu/stddef.in.h, gnu/stdint.in.h, gnu/stdio.h, gnu/stdio.in.h, gnu/stdlib.h, gnu/stdlib.in.h, gnu/strerror-override.h, gnu/string.h, gnu/string.in.h, gnu/sys_stat.in.h, gnu/sys_types.in.h, gnu/time.h, gnu/time.in.h, gnu/unistd.h, gnu/unistd.in.h, gnu/vasnprintf.h, gnu/verify.h, gnu/warn-on-use.h, gnu/wchar.h, gnu/wchar.in.h, gnu/xalloc-oversized.h, gnu/xalloc.h, gnu/xsize.h
- Representative functions: gl_msvc_inval_ensure_handler
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 043_module_gnu_msvc-inval.c_36.md

## module_gnu_msvc-nothrow.c_37
- Module category: module_cluster
- Related headers: config.h, gnu/_Noreturn.h, gnu/alignof.h, gnu/alloca.h, gnu/alloca.in.h, gnu/arg-nonnull.h, gnu/assert.h, gnu/assert.in.h, gnu/attribute.h, gnu/basename-lgpl.h, gnu/bitrotate.h, gnu/c++defs.h, gnu/cloexec.h, gnu/errno.in.h, gnu/error.h, gnu/error.in.h, gnu/exitfail.h, gnu/fcntl.h, gnu/fcntl.in.h, gnu/fd-hook.h, gnu/filename.h, gnu/float+.h, gnu/float.in.h, gnu/getprogname.h, gnu/gettext.h, gnu/hash.h, gnu/ialloc.h, gnu/idx.h, gnu/intprops-internal.h, gnu/intprops.h, gnu/inttypes.h, gnu/inttypes.in.h, gnu/limits.h, gnu/limits.in.h, gnu/malloca.h, gnu/minmax.h, gnu/msvc-inval.h, gnu/msvc-nothrow.h, gnu/obstack.h, gnu/obstack.in.h, gnu/pathmax.h, gnu/printf-args.h, gnu/printf-parse.h, gnu/progname.h, gnu/size_max.h, gnu/stat-time.h, gnu/stat-w32.h, gnu/stdckdint.h, gnu/stdckdint.in.h, gnu/stddef.h, gnu/stddef.in.h, gnu/stdint.in.h, gnu/stdio.h, gnu/stdio.in.h, gnu/stdlib.h, gnu/stdlib.in.h, gnu/strerror-override.h, gnu/string.h, gnu/string.in.h, gnu/sys_stat.in.h, gnu/sys_types.in.h, gnu/time.h, gnu/time.in.h, gnu/unistd.h, gnu/unistd.in.h, gnu/vasnprintf.h, gnu/verify.h, gnu/warn-on-use.h, gnu/wchar.h, gnu/wchar.in.h, gnu/xalloc-oversized.h, gnu/xalloc.h, gnu/xsize.h
- Representative functions: _gl_nothrow_get_osfhandle
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 044_module_gnu_msvc-nothrow.c_37.md

## module_gnu_obstack.c_38
- Module category: module_cluster
- Related headers: gnu/_Noreturn.h, gnu/alignof.h, gnu/alloca.h, gnu/alloca.in.h, gnu/arg-nonnull.h, gnu/assert.h, gnu/assert.in.h, gnu/attribute.h, gnu/basename-lgpl.h, gnu/bitrotate.h, gnu/c++defs.h, gnu/cloexec.h, gnu/errno.in.h, gnu/error.h, gnu/error.in.h, gnu/exitfail.h, gnu/fcntl.h, gnu/fcntl.in.h, gnu/fd-hook.h, gnu/filename.h, gnu/float+.h, gnu/float.in.h, gnu/getprogname.h, gnu/gettext.h, gnu/hash.h, gnu/ialloc.h, gnu/idx.h, gnu/intprops-internal.h, gnu/intprops.h, gnu/inttypes.h, gnu/inttypes.in.h, gnu/limits.h, gnu/limits.in.h, gnu/malloca.h, gnu/minmax.h, gnu/msvc-inval.h, gnu/msvc-nothrow.h, gnu/obstack.h, gnu/obstack.in.h, gnu/pathmax.h, gnu/printf-args.h, gnu/printf-parse.h, gnu/progname.h, gnu/size_max.h, gnu/stat-time.h, gnu/stat-w32.h, gnu/stdckdint.h, gnu/stdckdint.in.h, gnu/stddef.h, gnu/stddef.in.h, gnu/stdint.in.h, gnu/stdio.h, gnu/stdio.in.h, gnu/stdlib.h, gnu/stdlib.in.h, gnu/strerror-override.h, gnu/string.h, gnu/string.in.h, gnu/sys_stat.in.h, gnu/sys_types.in.h, gnu/time.h, gnu/time.in.h, gnu/unistd.h, gnu/unistd.in.h, gnu/vasnprintf.h, gnu/verify.h, gnu/warn-on-use.h, gnu/wchar.h, gnu/wchar.in.h, gnu/xalloc-oversized.h, gnu/xalloc.h, gnu/xsize.h
- Representative functions: print_and_abort
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 045_module_gnu_obstack.c_38.md

## module_gnu_open.c_39
- Module category: module_cluster
- Related headers: config.h, gnu/_Noreturn.h, gnu/alignof.h, gnu/alloca.h, gnu/alloca.in.h, gnu/arg-nonnull.h, gnu/assert.h, gnu/assert.in.h, gnu/attribute.h, gnu/basename-lgpl.h, gnu/bitrotate.h, gnu/c++defs.h, gnu/cloexec.h, gnu/errno.in.h, gnu/error.h, gnu/error.in.h, gnu/exitfail.h, gnu/fcntl.h, gnu/fcntl.in.h, gnu/fd-hook.h, gnu/filename.h, gnu/float+.h, gnu/float.in.h, gnu/getprogname.h, gnu/gettext.h, gnu/hash.h, gnu/ialloc.h, gnu/idx.h, gnu/intprops-internal.h, gnu/intprops.h, gnu/inttypes.h, gnu/inttypes.in.h, gnu/limits.h, gnu/limits.in.h, gnu/malloca.h, gnu/minmax.h, gnu/msvc-inval.h, gnu/msvc-nothrow.h, gnu/obstack.h, gnu/obstack.in.h, gnu/pathmax.h, gnu/printf-args.h, gnu/printf-parse.h, gnu/progname.h, gnu/size_max.h, gnu/stat-time.h, gnu/stat-w32.h, gnu/stdckdint.h, gnu/stdckdint.in.h, gnu/stddef.h, gnu/stddef.in.h, gnu/stdint.in.h, gnu/stdio.h, gnu/stdio.in.h, gnu/stdlib.h, gnu/stdlib.in.h, gnu/strerror-override.h, gnu/string.h, gnu/string.in.h, gnu/sys/stat.h, gnu/sys/types.h, gnu/sys_stat.in.h, gnu/sys_types.in.h, gnu/time.h, gnu/time.in.h, gnu/unistd.h, gnu/unistd.in.h, gnu/vasnprintf.h, gnu/verify.h, gnu/warn-on-use.h, gnu/wchar.h, gnu/wchar.in.h, gnu/xalloc-oversized.h, gnu/xalloc.h, gnu/xsize.h
- Representative functions: orig_open
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 046_module_gnu_open.c_39.md

## module_gnu_printf-args.c_40
- Module category: module_cluster
- Related headers: gnu/_Noreturn.h, gnu/alignof.h, gnu/alloca.h, gnu/alloca.in.h, gnu/arg-nonnull.h, gnu/assert.h, gnu/assert.in.h, gnu/attribute.h, gnu/basename-lgpl.h, gnu/bitrotate.h, gnu/c++defs.h, gnu/cloexec.h, gnu/errno.in.h, gnu/error.h, gnu/error.in.h, gnu/exitfail.h, gnu/fcntl.h, gnu/fcntl.in.h, gnu/fd-hook.h, gnu/filename.h, gnu/float+.h, gnu/float.in.h, gnu/getprogname.h, gnu/gettext.h, gnu/hash.h, gnu/ialloc.h, gnu/idx.h, gnu/intprops-internal.h, gnu/intprops.h, gnu/inttypes.h, gnu/inttypes.in.h, gnu/limits.h, gnu/limits.in.h, gnu/malloca.h, gnu/minmax.h, gnu/msvc-inval.h, gnu/msvc-nothrow.h, gnu/obstack.h, gnu/obstack.in.h, gnu/pathmax.h, gnu/printf-args.h, gnu/printf-parse.h, gnu/progname.h, gnu/size_max.h, gnu/stat-time.h, gnu/stat-w32.h, gnu/stdckdint.h, gnu/stdckdint.in.h, gnu/stddef.h, gnu/stddef.in.h, gnu/stdint.in.h, gnu/stdio.h, gnu/stdio.in.h, gnu/stdlib.h, gnu/stdlib.in.h, gnu/strerror-override.h, gnu/string.h, gnu/string.in.h, gnu/sys_stat.in.h, gnu/sys_types.in.h, gnu/time.h, gnu/time.in.h, gnu/unistd.h, gnu/unistd.in.h, gnu/vasnprintf.h, gnu/verify.h, gnu/warn-on-use.h, gnu/wchar.h, gnu/wchar.in.h, gnu/xalloc-oversized.h, gnu/xalloc.h, gnu/xsize.h
- Representative functions: PRINTF_FETCHARGS
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 047_module_gnu_printf-args.c_40.md

## module_gnu_printf-parse.c_41
- Module category: module_cluster
- Related headers: gnu/_Noreturn.h, gnu/alignof.h, gnu/alloca.h, gnu/alloca.in.h, gnu/arg-nonnull.h, gnu/assert.h, gnu/assert.in.h, gnu/attribute.h, gnu/basename-lgpl.h, gnu/bitrotate.h, gnu/c++defs.h, gnu/cloexec.h, gnu/errno.in.h, gnu/error.h, gnu/error.in.h, gnu/exitfail.h, gnu/fcntl.h, gnu/fcntl.in.h, gnu/fd-hook.h, gnu/filename.h, gnu/float+.h, gnu/float.in.h, gnu/getprogname.h, gnu/gettext.h, gnu/hash.h, gnu/ialloc.h, gnu/idx.h, gnu/intprops-internal.h, gnu/intprops.h, gnu/inttypes.h, gnu/inttypes.in.h, gnu/limits.h, gnu/limits.in.h, gnu/malloca.h, gnu/minmax.h, gnu/msvc-inval.h, gnu/msvc-nothrow.h, gnu/obstack.h, gnu/obstack.in.h, gnu/pathmax.h, gnu/printf-args.h, gnu/printf-parse.h, gnu/progname.h, gnu/size_max.h, gnu/stat-time.h, gnu/stat-w32.h, gnu/stdckdint.h, gnu/stdckdint.in.h, gnu/stddef.h, gnu/stddef.in.h, gnu/stdint.in.h, gnu/stdio.h, gnu/stdio.in.h, gnu/stdlib.h, gnu/stdlib.in.h, gnu/strerror-override.h, gnu/string.h, gnu/string.in.h, gnu/sys_stat.in.h, gnu/sys_types.in.h, gnu/time.h, gnu/time.in.h, gnu/unistd.h, gnu/unistd.in.h, gnu/vasnprintf.h, gnu/verify.h, gnu/warn-on-use.h, gnu/wchar.h, gnu/wchar.in.h, gnu/xalloc-oversized.h, gnu/xalloc.h, gnu/xsize.h
- Representative functions: PRINTF_PARSE
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 048_module_gnu_printf-parse.c_41.md

## module_gnu_progname.c_42
- Module category: module_cluster
- Related headers: config.h, gnu/_Noreturn.h, gnu/alignof.h, gnu/alloca.h, gnu/alloca.in.h, gnu/arg-nonnull.h, gnu/assert.h, gnu/assert.in.h, gnu/attribute.h, gnu/basename-lgpl.h, gnu/bitrotate.h, gnu/c++defs.h, gnu/cloexec.h, gnu/errno.in.h, gnu/error.h, gnu/error.in.h, gnu/exitfail.h, gnu/fcntl.h, gnu/fcntl.in.h, gnu/fd-hook.h, gnu/filename.h, gnu/float+.h, gnu/float.in.h, gnu/getprogname.h, gnu/gettext.h, gnu/hash.h, gnu/ialloc.h, gnu/idx.h, gnu/intprops-internal.h, gnu/intprops.h, gnu/inttypes.h, gnu/inttypes.in.h, gnu/limits.h, gnu/limits.in.h, gnu/malloca.h, gnu/minmax.h, gnu/msvc-inval.h, gnu/msvc-nothrow.h, gnu/obstack.h, gnu/obstack.in.h, gnu/pathmax.h, gnu/printf-args.h, gnu/printf-parse.h, gnu/progname.h, gnu/size_max.h, gnu/stat-time.h, gnu/stat-w32.h, gnu/stdckdint.h, gnu/stdckdint.in.h, gnu/stddef.h, gnu/stddef.in.h, gnu/stdint.in.h, gnu/stdio.h, gnu/stdio.in.h, gnu/stdlib.h, gnu/stdlib.in.h, gnu/strerror-override.h, gnu/string.h, gnu/string.in.h, gnu/sys_stat.in.h, gnu/sys_types.in.h, gnu/time.h, gnu/time.in.h, gnu/unistd.h, gnu/unistd.in.h, gnu/vasnprintf.h, gnu/verify.h, gnu/warn-on-use.h, gnu/wchar.h, gnu/wchar.in.h, gnu/xalloc-oversized.h, gnu/xalloc.h, gnu/xsize.h
- Representative functions: set_program_name
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 049_module_gnu_progname.c_42.md

## module_gnu_realloc.c_43
- Module category: module_cluster
- Related headers: config.h, gnu/_Noreturn.h, gnu/alignof.h, gnu/alloca.h, gnu/alloca.in.h, gnu/arg-nonnull.h, gnu/assert.h, gnu/assert.in.h, gnu/attribute.h, gnu/basename-lgpl.h, gnu/bitrotate.h, gnu/c++defs.h, gnu/cloexec.h, gnu/errno.in.h, gnu/error.h, gnu/error.in.h, gnu/exitfail.h, gnu/fcntl.h, gnu/fcntl.in.h, gnu/fd-hook.h, gnu/filename.h, gnu/float+.h, gnu/float.in.h, gnu/getprogname.h, gnu/gettext.h, gnu/hash.h, gnu/ialloc.h, gnu/idx.h, gnu/intprops-internal.h, gnu/intprops.h, gnu/inttypes.h, gnu/inttypes.in.h, gnu/limits.h, gnu/limits.in.h, gnu/malloca.h, gnu/minmax.h, gnu/msvc-inval.h, gnu/msvc-nothrow.h, gnu/obstack.h, gnu/obstack.in.h, gnu/pathmax.h, gnu/printf-args.h, gnu/printf-parse.h, gnu/progname.h, gnu/size_max.h, gnu/stat-time.h, gnu/stat-w32.h, gnu/stdckdint.h, gnu/stdckdint.in.h, gnu/stddef.h, gnu/stddef.in.h, gnu/stdint.in.h, gnu/stdio.h, gnu/stdio.in.h, gnu/stdlib.h, gnu/stdlib.in.h, gnu/strerror-override.h, gnu/string.h, gnu/string.in.h, gnu/sys_stat.in.h, gnu/sys_types.in.h, gnu/time.h, gnu/time.in.h, gnu/unistd.h, gnu/unistd.in.h, gnu/vasnprintf.h, gnu/verify.h, gnu/warn-on-use.h, gnu/wchar.h, gnu/wchar.in.h, gnu/xalloc-oversized.h, gnu/xalloc.h, gnu/xsize.h
- Representative functions: rpl_realloc
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 050_module_gnu_realloc.c_43.md

## module_gnu_reallocarray.c_44
- Module category: module_cluster
- Related headers: config.h, gnu/_Noreturn.h, gnu/alignof.h, gnu/alloca.h, gnu/alloca.in.h, gnu/arg-nonnull.h, gnu/assert.h, gnu/assert.in.h, gnu/attribute.h, gnu/basename-lgpl.h, gnu/bitrotate.h, gnu/c++defs.h, gnu/cloexec.h, gnu/errno.in.h, gnu/error.h, gnu/error.in.h, gnu/exitfail.h, gnu/fcntl.h, gnu/fcntl.in.h, gnu/fd-hook.h, gnu/filename.h, gnu/float+.h, gnu/float.in.h, gnu/getprogname.h, gnu/gettext.h, gnu/hash.h, gnu/ialloc.h, gnu/idx.h, gnu/intprops-internal.h, gnu/intprops.h, gnu/inttypes.h, gnu/inttypes.in.h, gnu/limits.h, gnu/limits.in.h, gnu/malloca.h, gnu/minmax.h, gnu/msvc-inval.h, gnu/msvc-nothrow.h, gnu/obstack.h, gnu/obstack.in.h, gnu/pathmax.h, gnu/printf-args.h, gnu/printf-parse.h, gnu/progname.h, gnu/size_max.h, gnu/stat-time.h, gnu/stat-w32.h, gnu/stdckdint.h, gnu/stdckdint.in.h, gnu/stddef.h, gnu/stddef.in.h, gnu/stdint.in.h, gnu/stdio.h, gnu/stdio.in.h, gnu/stdlib.h, gnu/stdlib.in.h, gnu/strerror-override.h, gnu/string.h, gnu/string.in.h, gnu/sys_stat.in.h, gnu/sys_types.in.h, gnu/time.h, gnu/time.in.h, gnu/unistd.h, gnu/unistd.in.h, gnu/vasnprintf.h, gnu/verify.h, gnu/warn-on-use.h, gnu/wchar.h, gnu/wchar.in.h, gnu/xalloc-oversized.h, gnu/xalloc.h, gnu/xsize.h
- Representative functions: reallocarray
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 051_module_gnu_reallocarray.c_44.md

## module_gnu_snprintf.c_45
- Module category: module_cluster
- Related headers: config.h, gnu/_Noreturn.h, gnu/alignof.h, gnu/alloca.h, gnu/alloca.in.h, gnu/arg-nonnull.h, gnu/assert.h, gnu/assert.in.h, gnu/attribute.h, gnu/basename-lgpl.h, gnu/bitrotate.h, gnu/c++defs.h, gnu/cloexec.h, gnu/errno.in.h, gnu/error.h, gnu/error.in.h, gnu/exitfail.h, gnu/fcntl.h, gnu/fcntl.in.h, gnu/fd-hook.h, gnu/filename.h, gnu/float+.h, gnu/float.in.h, gnu/getprogname.h, gnu/gettext.h, gnu/hash.h, gnu/ialloc.h, gnu/idx.h, gnu/intprops-internal.h, gnu/intprops.h, gnu/inttypes.h, gnu/inttypes.in.h, gnu/limits.h, gnu/limits.in.h, gnu/malloca.h, gnu/minmax.h, gnu/msvc-inval.h, gnu/msvc-nothrow.h, gnu/obstack.h, gnu/obstack.in.h, gnu/pathmax.h, gnu/printf-args.h, gnu/printf-parse.h, gnu/progname.h, gnu/size_max.h, gnu/stat-time.h, gnu/stat-w32.h, gnu/stdckdint.h, gnu/stdckdint.in.h, gnu/stddef.h, gnu/stddef.in.h, gnu/stdint.in.h, gnu/stdio.h, gnu/stdio.in.h, gnu/stdlib.h, gnu/stdlib.in.h, gnu/strerror-override.h, gnu/string.h, gnu/string.in.h, gnu/sys_stat.in.h, gnu/sys_types.in.h, gnu/time.h, gnu/time.in.h, gnu/unistd.h, gnu/unistd.in.h, gnu/vasnprintf.h, gnu/verify.h, gnu/warn-on-use.h, gnu/wchar.h, gnu/wchar.in.h, gnu/xalloc-oversized.h, gnu/xalloc.h, gnu/xsize.h
- Representative functions: snprintf
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 052_module_gnu_snprintf.c_45.md

## module_gnu_stat-w32.c_46
- Module category: module_cluster
- Related headers: config.h, gnu/_Noreturn.h, gnu/alignof.h, gnu/alloca.h, gnu/alloca.in.h, gnu/arg-nonnull.h, gnu/assert.h, gnu/assert.in.h, gnu/attribute.h, gnu/basename-lgpl.h, gnu/bitrotate.h, gnu/c++defs.h, gnu/cloexec.h, gnu/errno.in.h, gnu/error.h, gnu/error.in.h, gnu/exitfail.h, gnu/fcntl.h, gnu/fcntl.in.h, gnu/fd-hook.h, gnu/filename.h, gnu/float+.h, gnu/float.in.h, gnu/getprogname.h, gnu/gettext.h, gnu/hash.h, gnu/ialloc.h, gnu/idx.h, gnu/intprops-internal.h, gnu/intprops.h, gnu/inttypes.h, gnu/inttypes.in.h, gnu/limits.h, gnu/limits.in.h, gnu/malloca.h, gnu/minmax.h, gnu/msvc-inval.h, gnu/msvc-nothrow.h, gnu/obstack.h, gnu/obstack.in.h, gnu/pathmax.h, gnu/printf-args.h, gnu/printf-parse.h, gnu/progname.h, gnu/size_max.h, gnu/stat-time.h, gnu/stat-w32.h, gnu/stdckdint.h, gnu/stdckdint.in.h, gnu/stddef.h, gnu/stddef.in.h, gnu/stdint.in.h, gnu/stdio.h, gnu/stdio.in.h, gnu/stdlib.h, gnu/stdlib.in.h, gnu/strerror-override.h, gnu/string.h, gnu/string.in.h, gnu/sys/stat.h, gnu/sys/types.h, gnu/sys_stat.in.h, gnu/sys_types.in.h, gnu/time.h, gnu/time.in.h, gnu/unistd.h, gnu/unistd.in.h, gnu/vasnprintf.h, gnu/verify.h, gnu/warn-on-use.h, gnu/wchar.h, gnu/wchar.in.h, gnu/xalloc-oversized.h, gnu/xalloc.h, gnu/xsize.h
- Representative functions: initialize
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 053_module_gnu_stat-w32.c_46.md

## module_gnu_stat.c_47
- Module category: module_cluster
- Related headers: config.h, gnu/_Noreturn.h, gnu/alignof.h, gnu/alloca.h, gnu/alloca.in.h, gnu/arg-nonnull.h, gnu/assert.h, gnu/assert.in.h, gnu/attribute.h, gnu/basename-lgpl.h, gnu/bitrotate.h, gnu/c++defs.h, gnu/cloexec.h, gnu/errno.in.h, gnu/error.h, gnu/error.in.h, gnu/exitfail.h, gnu/fcntl.h, gnu/fcntl.in.h, gnu/fd-hook.h, gnu/filename.h, gnu/float+.h, gnu/float.in.h, gnu/getprogname.h, gnu/gettext.h, gnu/hash.h, gnu/ialloc.h, gnu/idx.h, gnu/intprops-internal.h, gnu/intprops.h, gnu/inttypes.h, gnu/inttypes.in.h, gnu/limits.h, gnu/limits.in.h, gnu/malloca.h, gnu/minmax.h, gnu/msvc-inval.h, gnu/msvc-nothrow.h, gnu/obstack.h, gnu/obstack.in.h, gnu/pathmax.h, gnu/printf-args.h, gnu/printf-parse.h, gnu/progname.h, gnu/size_max.h, gnu/stat-time.h, gnu/stat-w32.h, gnu/stdckdint.h, gnu/stdckdint.in.h, gnu/stddef.h, gnu/stddef.in.h, gnu/stdint.in.h, gnu/stdio.h, gnu/stdio.in.h, gnu/stdlib.h, gnu/stdlib.in.h, gnu/strerror-override.h, gnu/string.h, gnu/string.in.h, gnu/sys/stat.h, gnu/sys/types.h, gnu/sys_stat.in.h, gnu/sys_types.in.h, gnu/time.h, gnu/time.in.h, gnu/unistd.h, gnu/unistd.in.h, gnu/vasnprintf.h, gnu/verify.h, gnu/warn-on-use.h, gnu/wchar.h, gnu/wchar.in.h, gnu/xalloc-oversized.h, gnu/xalloc.h, gnu/xsize.h
- Representative functions: is_unc_root
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 054_module_gnu_stat.c_47.md

## module_gnu_stdio-read.c_48
- Module category: module_cluster
- Related headers: config.h, gnu/_Noreturn.h, gnu/alignof.h, gnu/alloca.h, gnu/alloca.in.h, gnu/arg-nonnull.h, gnu/assert.h, gnu/assert.in.h, gnu/attribute.h, gnu/basename-lgpl.h, gnu/bitrotate.h, gnu/c++defs.h, gnu/cloexec.h, gnu/errno.in.h, gnu/error.h, gnu/error.in.h, gnu/exitfail.h, gnu/fcntl.h, gnu/fcntl.in.h, gnu/fd-hook.h, gnu/filename.h, gnu/float+.h, gnu/float.in.h, gnu/getprogname.h, gnu/gettext.h, gnu/hash.h, gnu/ialloc.h, gnu/idx.h, gnu/intprops-internal.h, gnu/intprops.h, gnu/inttypes.h, gnu/inttypes.in.h, gnu/limits.h, gnu/limits.in.h, gnu/malloca.h, gnu/minmax.h, gnu/msvc-inval.h, gnu/msvc-nothrow.h, gnu/obstack.h, gnu/obstack.in.h, gnu/pathmax.h, gnu/printf-args.h, gnu/printf-parse.h, gnu/progname.h, gnu/size_max.h, gnu/stat-time.h, gnu/stat-w32.h, gnu/stdckdint.h, gnu/stdckdint.in.h, gnu/stddef.h, gnu/stddef.in.h, gnu/stdint.in.h, gnu/stdio.h, gnu/stdio.in.h, gnu/stdlib.h, gnu/stdlib.in.h, gnu/strerror-override.h, gnu/string.h, gnu/string.in.h, gnu/sys_stat.in.h, gnu/sys_types.in.h, gnu/time.h, gnu/time.in.h, gnu/unistd.h, gnu/unistd.in.h, gnu/vasnprintf.h, gnu/verify.h, gnu/warn-on-use.h, gnu/wchar.h, gnu/wchar.in.h, gnu/xalloc-oversized.h, gnu/xalloc.h, gnu/xsize.h
- Representative functions: scanf, fscanf, vscanf, vfscanf, getchar, fgetc, fgets, fread
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 055_module_gnu_stdio-read.c_48.md

## module_gnu_stdio-write.c_49
- Module category: module_cluster
- Related headers: config.h, gnu/_Noreturn.h, gnu/alignof.h, gnu/alloca.h, gnu/alloca.in.h, gnu/arg-nonnull.h, gnu/assert.h, gnu/assert.in.h, gnu/attribute.h, gnu/basename-lgpl.h, gnu/bitrotate.h, gnu/c++defs.h, gnu/cloexec.h, gnu/errno.in.h, gnu/error.h, gnu/error.in.h, gnu/exitfail.h, gnu/fcntl.h, gnu/fcntl.in.h, gnu/fd-hook.h, gnu/filename.h, gnu/float+.h, gnu/float.in.h, gnu/getprogname.h, gnu/gettext.h, gnu/hash.h, gnu/ialloc.h, gnu/idx.h, gnu/intprops-internal.h, gnu/intprops.h, gnu/inttypes.h, gnu/inttypes.in.h, gnu/limits.h, gnu/limits.in.h, gnu/malloca.h, gnu/minmax.h, gnu/msvc-inval.h, gnu/msvc-nothrow.h, gnu/obstack.h, gnu/obstack.in.h, gnu/pathmax.h, gnu/printf-args.h, gnu/printf-parse.h, gnu/progname.h, gnu/size_max.h, gnu/stat-time.h, gnu/stat-w32.h, gnu/stdckdint.h, gnu/stdckdint.in.h, gnu/stddef.h, gnu/stddef.in.h, gnu/stdint.in.h, gnu/stdio.h, gnu/stdio.in.h, gnu/stdlib.h, gnu/stdlib.in.h, gnu/strerror-override.h, gnu/string.h, gnu/string.in.h, gnu/sys_stat.in.h, gnu/sys_types.in.h, gnu/time.h, gnu/time.in.h, gnu/unistd.h, gnu/unistd.in.h, gnu/vasnprintf.h, gnu/verify.h, gnu/warn-on-use.h, gnu/wchar.h, gnu/wchar.in.h, gnu/xalloc-oversized.h, gnu/xalloc.h, gnu/xsize.h
- Representative functions: printf, fprintf, vprintf, vfprintf, putchar, fputc, fputs, puts
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 056_module_gnu_stdio-write.c_49.md

## module_gnu_strerror-override.c_50
- Module category: module_cluster
- Related headers: config.h, gnu/_Noreturn.h, gnu/alignof.h, gnu/alloca.h, gnu/alloca.in.h, gnu/arg-nonnull.h, gnu/assert.h, gnu/assert.in.h, gnu/attribute.h, gnu/basename-lgpl.h, gnu/bitrotate.h, gnu/c++defs.h, gnu/cloexec.h, gnu/errno.in.h, gnu/error.h, gnu/error.in.h, gnu/exitfail.h, gnu/fcntl.h, gnu/fcntl.in.h, gnu/fd-hook.h, gnu/filename.h, gnu/float+.h, gnu/float.in.h, gnu/getprogname.h, gnu/gettext.h, gnu/hash.h, gnu/ialloc.h, gnu/idx.h, gnu/intprops-internal.h, gnu/intprops.h, gnu/inttypes.h, gnu/inttypes.in.h, gnu/limits.h, gnu/limits.in.h, gnu/malloca.h, gnu/minmax.h, gnu/msvc-inval.h, gnu/msvc-nothrow.h, gnu/obstack.h, gnu/obstack.in.h, gnu/pathmax.h, gnu/printf-args.h, gnu/printf-parse.h, gnu/progname.h, gnu/size_max.h, gnu/stat-time.h, gnu/stat-w32.h, gnu/stdckdint.h, gnu/stdckdint.in.h, gnu/stddef.h, gnu/stddef.in.h, gnu/stdint.in.h, gnu/stdio.h, gnu/stdio.in.h, gnu/stdlib.h, gnu/stdlib.in.h, gnu/strerror-override.h, gnu/string.h, gnu/string.in.h, gnu/sys_stat.in.h, gnu/sys_types.in.h, gnu/time.h, gnu/time.in.h, gnu/unistd.h, gnu/unistd.in.h, gnu/vasnprintf.h, gnu/verify.h, gnu/warn-on-use.h, gnu/wchar.h, gnu/wchar.in.h, gnu/xalloc-oversized.h, gnu/xalloc.h, gnu/xsize.h
- Representative functions: strerror_override
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 057_module_gnu_strerror-override.c_50.md

## module_gnu_strerror.c_51
- Module category: module_cluster
- Related headers: config.h, gnu/_Noreturn.h, gnu/alignof.h, gnu/alloca.h, gnu/alloca.in.h, gnu/arg-nonnull.h, gnu/assert.h, gnu/assert.in.h, gnu/attribute.h, gnu/basename-lgpl.h, gnu/bitrotate.h, gnu/c++defs.h, gnu/cloexec.h, gnu/errno.in.h, gnu/error.h, gnu/error.in.h, gnu/exitfail.h, gnu/fcntl.h, gnu/fcntl.in.h, gnu/fd-hook.h, gnu/filename.h, gnu/float+.h, gnu/float.in.h, gnu/getprogname.h, gnu/gettext.h, gnu/hash.h, gnu/ialloc.h, gnu/idx.h, gnu/intprops-internal.h, gnu/intprops.h, gnu/inttypes.h, gnu/inttypes.in.h, gnu/limits.h, gnu/limits.in.h, gnu/malloca.h, gnu/minmax.h, gnu/msvc-inval.h, gnu/msvc-nothrow.h, gnu/obstack.h, gnu/obstack.in.h, gnu/pathmax.h, gnu/printf-args.h, gnu/printf-parse.h, gnu/progname.h, gnu/size_max.h, gnu/stat-time.h, gnu/stat-w32.h, gnu/stdckdint.h, gnu/stdckdint.in.h, gnu/stddef.h, gnu/stddef.in.h, gnu/stdint.in.h, gnu/stdio.h, gnu/stdio.in.h, gnu/stdlib.h, gnu/stdlib.in.h, gnu/strerror-override.h, gnu/string.h, gnu/string.in.h, gnu/sys_stat.in.h, gnu/sys_types.in.h, gnu/time.h, gnu/time.in.h, gnu/unistd.h, gnu/unistd.in.h, gnu/vasnprintf.h, gnu/verify.h, gnu/warn-on-use.h, gnu/wchar.h, gnu/wchar.in.h, gnu/xalloc-oversized.h, gnu/xalloc.h, gnu/xsize.h
- Representative functions: strerror
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 058_module_gnu_strerror.c_51.md

## module_gnu_vasnprintf.c_52
- Module category: module_cluster
- Related headers: gnu/_Noreturn.h, gnu/alignof.h, gnu/alloca.h, gnu/alloca.in.h, gnu/arg-nonnull.h, gnu/assert.h, gnu/assert.in.h, gnu/attribute.h, gnu/basename-lgpl.h, gnu/bitrotate.h, gnu/c++defs.h, gnu/cloexec.h, gnu/errno.in.h, gnu/error.h, gnu/error.in.h, gnu/exitfail.h, gnu/fcntl.h, gnu/fcntl.in.h, gnu/fd-hook.h, gnu/filename.h, gnu/float+.h, gnu/float.in.h, gnu/getprogname.h, gnu/gettext.h, gnu/hash.h, gnu/ialloc.h, gnu/idx.h, gnu/intprops-internal.h, gnu/intprops.h, gnu/inttypes.h, gnu/inttypes.in.h, gnu/limits.h, gnu/limits.in.h, gnu/malloca.h, gnu/minmax.h, gnu/msvc-inval.h, gnu/msvc-nothrow.h, gnu/obstack.h, gnu/obstack.in.h, gnu/pathmax.h, gnu/printf-args.h, gnu/printf-parse.h, gnu/progname.h, gnu/size_max.h, gnu/stat-time.h, gnu/stat-w32.h, gnu/stdckdint.h, gnu/stdckdint.in.h, gnu/stddef.h, gnu/stddef.in.h, gnu/stdint.in.h, gnu/stdio.h, gnu/stdio.in.h, gnu/stdlib.h, gnu/stdlib.in.h, gnu/strerror-override.h, gnu/string.h, gnu/string.in.h, gnu/sys_stat.in.h, gnu/sys_types.in.h, gnu/time.h, gnu/time.in.h, gnu/unistd.h, gnu/unistd.in.h, gnu/vasnprintf.h, gnu/verify.h, gnu/warn-on-use.h, gnu/wchar.h, gnu/wchar.in.h, gnu/xalloc-oversized.h, gnu/xalloc.h, gnu/xsize.h
- Representative functions: local_strnlen, local_wcslen, local_wcsnlen, wctomb_fallback, local_wcrtomb, local_wctomb, decimal_point_char, multiply
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 059_module_gnu_vasnprintf.c_52.md

## module_gnu_vasnprintf.c_53
- Module category: module_cluster
- Related headers: gnu/_Noreturn.h, gnu/alignof.h, gnu/alloca.h, gnu/alloca.in.h, gnu/arg-nonnull.h, gnu/assert.h, gnu/assert.in.h, gnu/attribute.h, gnu/basename-lgpl.h, gnu/bitrotate.h, gnu/c++defs.h, gnu/cloexec.h, gnu/errno.in.h, gnu/error.h, gnu/error.in.h, gnu/exitfail.h, gnu/fcntl.h, gnu/fcntl.in.h, gnu/fd-hook.h, gnu/filename.h, gnu/float+.h, gnu/float.in.h, gnu/getprogname.h, gnu/gettext.h, gnu/hash.h, gnu/ialloc.h, gnu/idx.h, gnu/intprops-internal.h, gnu/intprops.h, gnu/inttypes.h, gnu/inttypes.in.h, gnu/limits.h, gnu/limits.in.h, gnu/malloca.h, gnu/minmax.h, gnu/msvc-inval.h, gnu/msvc-nothrow.h, gnu/obstack.h, gnu/obstack.in.h, gnu/pathmax.h, gnu/printf-args.h, gnu/printf-parse.h, gnu/progname.h, gnu/size_max.h, gnu/stat-time.h, gnu/stat-w32.h, gnu/stdckdint.h, gnu/stdckdint.in.h, gnu/stddef.h, gnu/stddef.in.h, gnu/stdint.in.h, gnu/stdio.h, gnu/stdio.in.h, gnu/stdlib.h, gnu/stdlib.in.h, gnu/strerror-override.h, gnu/string.h, gnu/string.in.h, gnu/sys_stat.in.h, gnu/sys_types.in.h, gnu/time.h, gnu/time.in.h, gnu/unistd.h, gnu/unistd.in.h, gnu/vasnprintf.h, gnu/verify.h, gnu/warn-on-use.h, gnu/wchar.h, gnu/wchar.in.h, gnu/xalloc-oversized.h, gnu/xalloc.h, gnu/xsize.h
- Representative functions: decode_long_double, decode_double, floorlog10l, floorlog10, is_borderline
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 060_module_gnu_vasnprintf.c_53.md

## module_gnu_vasnprintf.c_54
- Module category: module_cluster
- Related headers: gnu/_Noreturn.h, gnu/alignof.h, gnu/alloca.h, gnu/alloca.in.h, gnu/arg-nonnull.h, gnu/assert.h, gnu/assert.in.h, gnu/attribute.h, gnu/basename-lgpl.h, gnu/bitrotate.h, gnu/c++defs.h, gnu/cloexec.h, gnu/errno.in.h, gnu/error.h, gnu/error.in.h, gnu/exitfail.h, gnu/fcntl.h, gnu/fcntl.in.h, gnu/fd-hook.h, gnu/filename.h, gnu/float+.h, gnu/float.in.h, gnu/getprogname.h, gnu/gettext.h, gnu/hash.h, gnu/ialloc.h, gnu/idx.h, gnu/intprops-internal.h, gnu/intprops.h, gnu/inttypes.h, gnu/inttypes.in.h, gnu/limits.h, gnu/limits.in.h, gnu/malloca.h, gnu/minmax.h, gnu/msvc-inval.h, gnu/msvc-nothrow.h, gnu/obstack.h, gnu/obstack.in.h, gnu/pathmax.h, gnu/printf-args.h, gnu/printf-parse.h, gnu/progname.h, gnu/size_max.h, gnu/stat-time.h, gnu/stat-w32.h, gnu/stdckdint.h, gnu/stdckdint.in.h, gnu/stddef.h, gnu/stddef.in.h, gnu/stdint.in.h, gnu/stdio.h, gnu/stdio.in.h, gnu/stdlib.h, gnu/stdlib.in.h, gnu/strerror-override.h, gnu/string.h, gnu/string.in.h, gnu/sys_stat.in.h, gnu/sys_types.in.h, gnu/time.h, gnu/time.in.h, gnu/unistd.h, gnu/unistd.in.h, gnu/vasnprintf.h, gnu/verify.h, gnu/warn-on-use.h, gnu/wchar.h, gnu/wchar.in.h, gnu/xalloc-oversized.h, gnu/xalloc.h, gnu/xsize.h
- Representative functions: MAX_ROOM_NEEDED
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 061_module_gnu_vasnprintf.c_54.md

## module_gnu_xalloc-die.c_55
- Module category: module_cluster
- Related headers: config.h, gnu/_Noreturn.h, gnu/alignof.h, gnu/alloca.h, gnu/alloca.in.h, gnu/arg-nonnull.h, gnu/assert.h, gnu/assert.in.h, gnu/attribute.h, gnu/basename-lgpl.h, gnu/bitrotate.h, gnu/c++defs.h, gnu/cloexec.h, gnu/errno.in.h, gnu/error.h, gnu/error.in.h, gnu/exitfail.h, gnu/fcntl.h, gnu/fcntl.in.h, gnu/fd-hook.h, gnu/filename.h, gnu/float+.h, gnu/float.in.h, gnu/getprogname.h, gnu/gettext.h, gnu/hash.h, gnu/ialloc.h, gnu/idx.h, gnu/intprops-internal.h, gnu/intprops.h, gnu/inttypes.h, gnu/inttypes.in.h, gnu/limits.h, gnu/limits.in.h, gnu/malloca.h, gnu/minmax.h, gnu/msvc-inval.h, gnu/msvc-nothrow.h, gnu/obstack.h, gnu/obstack.in.h, gnu/pathmax.h, gnu/printf-args.h, gnu/printf-parse.h, gnu/progname.h, gnu/size_max.h, gnu/stat-time.h, gnu/stat-w32.h, gnu/stdckdint.h, gnu/stdckdint.in.h, gnu/stddef.h, gnu/stddef.in.h, gnu/stdint.in.h, gnu/stdio.h, gnu/stdio.in.h, gnu/stdlib.h, gnu/stdlib.in.h, gnu/strerror-override.h, gnu/string.h, gnu/string.in.h, gnu/sys_stat.in.h, gnu/sys_types.in.h, gnu/time.h, gnu/time.in.h, gnu/unistd.h, gnu/unistd.in.h, gnu/vasnprintf.h, gnu/verify.h, gnu/warn-on-use.h, gnu/wchar.h, gnu/wchar.in.h, gnu/xalloc-oversized.h, gnu/xalloc.h, gnu/xsize.h
- Representative functions: xalloc_die
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 062_module_gnu_xalloc-die.c_55.md

## module_gnu_xmalloc.c_56
- Module category: module_cluster
- Related headers: config.h, gnu/_Noreturn.h, gnu/alignof.h, gnu/alloca.h, gnu/alloca.in.h, gnu/arg-nonnull.h, gnu/assert.h, gnu/assert.in.h, gnu/attribute.h, gnu/basename-lgpl.h, gnu/bitrotate.h, gnu/c++defs.h, gnu/cloexec.h, gnu/errno.in.h, gnu/error.h, gnu/error.in.h, gnu/exitfail.h, gnu/fcntl.h, gnu/fcntl.in.h, gnu/fd-hook.h, gnu/filename.h, gnu/float+.h, gnu/float.in.h, gnu/getprogname.h, gnu/gettext.h, gnu/hash.h, gnu/ialloc.h, gnu/idx.h, gnu/intprops-internal.h, gnu/intprops.h, gnu/inttypes.h, gnu/inttypes.in.h, gnu/limits.h, gnu/limits.in.h, gnu/malloca.h, gnu/minmax.h, gnu/msvc-inval.h, gnu/msvc-nothrow.h, gnu/obstack.h, gnu/obstack.in.h, gnu/pathmax.h, gnu/printf-args.h, gnu/printf-parse.h, gnu/progname.h, gnu/size_max.h, gnu/stat-time.h, gnu/stat-w32.h, gnu/stdckdint.h, gnu/stdckdint.in.h, gnu/stddef.h, gnu/stddef.in.h, gnu/stdint.in.h, gnu/stdio.h, gnu/stdio.in.h, gnu/stdlib.h, gnu/stdlib.in.h, gnu/strerror-override.h, gnu/string.h, gnu/string.in.h, gnu/sys_stat.in.h, gnu/sys_types.in.h, gnu/time.h, gnu/time.in.h, gnu/unistd.h, gnu/unistd.in.h, gnu/vasnprintf.h, gnu/verify.h, gnu/warn-on-use.h, gnu/wchar.h, gnu/wchar.in.h, gnu/xalloc-oversized.h, gnu/xalloc.h, gnu/xsize.h
- Representative functions: xmalloc, ximalloc, xcharalloc, xrealloc, xirealloc, xreallocarray, xireallocarray, xnmalloc
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 063_module_gnu_xmalloc.c_56.md

## module_gnu_xmalloc.c_57
- Module category: module_cluster
- Related headers: config.h, gnu/_Noreturn.h, gnu/alignof.h, gnu/alloca.h, gnu/alloca.in.h, gnu/arg-nonnull.h, gnu/assert.h, gnu/assert.in.h, gnu/attribute.h, gnu/basename-lgpl.h, gnu/bitrotate.h, gnu/c++defs.h, gnu/cloexec.h, gnu/errno.in.h, gnu/error.h, gnu/error.in.h, gnu/exitfail.h, gnu/fcntl.h, gnu/fcntl.in.h, gnu/fd-hook.h, gnu/filename.h, gnu/float+.h, gnu/float.in.h, gnu/getprogname.h, gnu/gettext.h, gnu/hash.h, gnu/ialloc.h, gnu/idx.h, gnu/intprops-internal.h, gnu/intprops.h, gnu/inttypes.h, gnu/inttypes.in.h, gnu/limits.h, gnu/limits.in.h, gnu/malloca.h, gnu/minmax.h, gnu/msvc-inval.h, gnu/msvc-nothrow.h, gnu/obstack.h, gnu/obstack.in.h, gnu/pathmax.h, gnu/printf-args.h, gnu/printf-parse.h, gnu/progname.h, gnu/size_max.h, gnu/stat-time.h, gnu/stat-w32.h, gnu/stdckdint.h, gnu/stdckdint.in.h, gnu/stddef.h, gnu/stddef.in.h, gnu/stdint.in.h, gnu/stdio.h, gnu/stdio.in.h, gnu/stdlib.h, gnu/stdlib.in.h, gnu/strerror-override.h, gnu/string.h, gnu/string.in.h, gnu/sys_stat.in.h, gnu/sys_types.in.h, gnu/time.h, gnu/time.in.h, gnu/unistd.h, gnu/unistd.in.h, gnu/vasnprintf.h, gnu/verify.h, gnu/warn-on-use.h, gnu/wchar.h, gnu/wchar.in.h, gnu/xalloc-oversized.h, gnu/xalloc.h, gnu/xsize.h
- Representative functions: xicalloc, xmemdup, ximemdup, ximemdup0, xstrdup
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 064_module_gnu_xmalloc.c_57.md

## module_src_linked_list_entry_01
- Module category: module_cluster
- Related headers: gnu/fcntl.h, gnu/progname.h, gnu/sys/stat.h, gnu/sys/types.h, gnu/unistd.h, src/cflow.h, src/parseopt/parseopt.h, src/parser.h, src/wordsplit/wordsplit.h
- Representative functions: dot_print_symbol, linked_list_append, linked_list_prepend, linked_list_destroy, linked_list_unlink, linked_list_iterate, data_in_list, linked_list_size
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 065_module_src_linked_list_entry_01.md

## module_src_linked_list_entry_02
- Module category: module_cluster
- Related headers: gnu/hash.h, src/cflow.h, src/parser.h
- Representative functions: collect_functions, move_parms, first_starter, next_starter, mark_callers, eliminate_non_targets
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 066_module_src_linked_list_entry_02.md

## module_src_parseopt_03
- Module category: module_cluster
- Related headers: gnu/fcntl.h, gnu/progname.h, gnu/sys/stat.h, gnu/sys/types.h, gnu/unistd.h, src/cflow.h, src/parseopt/parseopt.h, src/parser.h, src/wordsplit/wordsplit.h
- Representative functions: optset_include_classes, optset_output_driver, optset_xref, optset_symbol, optset_preproc_option, optset_preprocess, optset_level_indent, optset_main_symbol
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 067_module_src_parseopt_03.md

## module_src_parseopt_04
- Module category: module_cluster
- Related headers: gnu/fcntl.h, gnu/progname.h, gnu/sys/stat.h, gnu/sys/types.h, gnu/unistd.h, src/cflow.h, src/parseopt/parseopt.h, src/parser.h, src/wordsplit/wordsplit.h
- Representative functions: parseopt_from_env, fromfile_error, fromfile, optset_profile, init_hook
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 068_module_src_parseopt_04.md

## module_src_linked_list_05
- Module category: module_cluster
- Related headers: gnu/hash.h, src/cflow.h, src/parser.h
- Representative functions: deref_linked_list, linked_list_create, append_symbol
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 069_module_src_linked_list_05.md

## module_src_table_entry_06
- Module category: module_cluster
- Related headers: gnu/hash.h, src/cflow.h, src/parser.h
- Representative functions: hash_symbol_hasher, hash_symbol_compare, lookup, install, unlink_symbol, static_free, collect_processor, delete_parms_itr
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 070_module_src_table_entry_06.md

## module_src_parseopt_file_07
- Module category: module_cluster
- Related headers: gnu/fcntl.h, gnu/progname.h, gnu/sys/stat.h, gnu/sys/types.h, gnu/unistd.h, src/cflow.h, src/parseopt/parseopt.h, src/parser.h, src/wordsplit/wordsplit.h
- Representative functions: optfile_lookup, parseopt_from_rc
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 071_module_src_parseopt_file_07.md

## module_src_balance_state_08
- Module category: module_cluster
- Related headers: src/cflow.h, src/parser.h
- Representative functions: push_balance_state, pop_balance_state, free_balance_stack, find_closing_paren
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 072_module_src_balance_state_08.md

## module_src_output_symbol_09
- Module category: module_cluster
- Related headers: src/cflow.h, src/parser.h
- Representative functions: print_symbol, print_symbol, print_symbol
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 073_module_src_output_symbol_09.md

## module_src_collect_data_10
- Module category: module_cluster
- Related headers: gnu/hash.h, src/cflow.h, src/parser.h
- Representative functions: collect_list_entry, collect_symbols
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 074_module_src_collect_data_10.md

## module_src_yy_buffer_state_11
- Module category: module_cluster
- Related headers: gnu/inttypes.h, gnu/stdio.h, gnu/stdlib.h, gnu/string.h, gnu/unistd.h, src/cflow.h, src/parser.h
- Representative functions: yy_create_buffer, yyensure_buffer_stack, yy_scan_buffer
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 075_module_src_yy_buffer_state_11.md

## module_src_delete_level_12
- Module category: module_cluster
- Related headers: gnu/hash.h, src/cflow.h, src/parser.h
- Representative functions: delete_level_autos, delete_level_statics
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 076_module_src_delete_level_12.md

## module_src_print_function_13
- Module category: module_cluster
- Related headers: src/cflow.h, src/parser.h
- Representative functions: print_function_name, print_function
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 077_module_src_print_function_13.md

## module_src_save_stack_14
- Module category: module_cluster
- Related headers: src/cflow.h, src/parser.h
- Representative functions: save_stack, save_stack_is_empty
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 078_module_src_save_stack_14.md

## module_src_set_level_15
- Module category: module_cluster
- Related headers: gnu/fcntl.h, gnu/progname.h, gnu/sys/stat.h, gnu/sys/types.h, gnu/unistd.h, src/cflow.h, src/parseopt/parseopt.h, src/parser.h, src/wordsplit/wordsplit.h
- Representative functions: set_level_indent, set_level_mark
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 079_module_src_set_level_15.md

## module_src_yy_flex_16
- Module category: module_cluster
- Related headers: gnu/inttypes.h, gnu/stdio.h, gnu/stdlib.h, gnu/string.h, gnu/unistd.h, src/cflow.h, src/parser.h
- Representative functions: yy_flex_strncpy, yy_flex_strlen
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 080_module_src_yy_flex_16.md

## module_src_yy_get_17
- Module category: module_cluster
- Related headers: gnu/inttypes.h, gnu/stdio.h, gnu/stdlib.h, gnu/string.h, gnu/unistd.h, src/cflow.h, src/parser.h
- Representative functions: yy_get_next_buffer, yy_get_previous_state
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 081_module_src_yy_get_17.md

## module_src_yy_init_18
- Module category: module_cluster
- Related headers: gnu/inttypes.h, gnu/stdio.h, gnu/stdlib.h, gnu/string.h, gnu/unistd.h, src/cflow.h, src/parser.h
- Representative functions: yy_init_buffer, yy_init_globals
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 082_module_src_yy_init_18.md

## module_src_yy_scan_19
- Module category: module_cluster
- Related headers: gnu/inttypes.h, gnu/stdio.h, gnu/stdlib.h, gnu/string.h, gnu/unistd.h, src/cflow.h, src/parser.h
- Representative functions: yy_scan_string, yy_scan_bytes
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 083_module_src_yy_scan_19.md

## module_src_c.c_20
- Module category: module_cluster
- Related headers: gnu/inttypes.h, gnu/stdio.h, gnu/stdlib.h, gnu/string.h, gnu/unistd.h, src/cflow.h, src/parser.h
- Representative functions: if, yy_try_NUL_trans, yyunput, yyrestart, yy_switch_to_buffer, yy_load_buffer_state, yy_delete_buffer, yy_flush_buffer
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 084_module_src_c.c_20.md

## module_src_c.c_21
- Module category: module_cluster
- Related headers: gnu/inttypes.h, gnu/stdio.h, gnu/stdlib.h, gnu/string.h, gnu/unistd.h, src/cflow.h, src/parser.h
- Representative functions: yyget_text, yyset_lineno, yyset_in, yyset_out, yyget_debug, yyset_debug, yylex_destroy, yyalloc
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 085_module_src_c.c_21.md

## module_src_c.c_22
- Module category: module_cluster
- Related headers: gnu/inttypes.h, gnu/stdio.h, gnu/stdlib.h, gnu/string.h, gnu/unistd.h, src/cflow.h, src/parser.h
- Representative functions: pp_finalize, pp_open, pp_close, yywrap, get_token, source, getnum, backslash
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 086_module_src_c.c_22.md

## module_src_depmap.c_23
- Module category: module_cluster
- Related headers: src/cflow.h, src/parser.h
- Representative functions: transitive_closure, depmap_alloc, depmap_rowptr, depmap_set, depmap_isset, depmap_tc
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 087_module_src_depmap.c_23.md

## module_src_dot.c_24
- Module category: module_cluster
- Related headers: src/cflow.h, src/parser.h
- Representative functions: dot_begin, declare_node, dot_output_handler
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 088_module_src_dot.c_24.md

## module_src_gnu.c_25
- Module category: module_cluster
- Related headers: src/cflow.h, src/parser.h
- Representative functions: gnu_output_handler
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 089_module_src_gnu.c_25.md

## module_src_main.c_26
- Module category: module_cluster
- Related headers: gnu/fcntl.h, gnu/progname.h, gnu/sys/stat.h, gnu/sys/types.h, gnu/unistd.h, src/cflow.h, src/parseopt/parseopt.h, src/parser.h, src/wordsplit/wordsplit.h
- Representative functions: CHAR_TO_SM, find_option_type, symbol_override, number, parse_level_string, tildexpand, parse_rc, globals_only
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 090_module_src_main.c_26.md

## module_src_output.c_27
- Module category: module_cluster
- Related headers: src/cflow.h, src/parser.h
- Representative functions: print_level, register_output, select_output_driver, output_init, newline, begin, end, separator
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 091_module_src_output.c_27.md

## module_src_output.c_28
- Module category: module_cluster
- Related headers: src/cflow.h, src/parser.h
- Representative functions: set_active, output
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 092_module_src_output.c_28.md

## module_src_parser.c_29
- Module category: module_cluster
- Related headers: src/cflow.h, src/parser.h
- Representative functions: print_token, token_type_str, dbgtok, debugtoken, file_error, mark, restore, tokdel
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 093_module_src_parser.c_29.md

## module_src_parser.c_30
- Module category: module_cluster
- Related headers: src/cflow.h, src/parser.h
- Representative functions: save_token, undo_save_stack, finish_save_stack, skip_to, skip_balanced, yyparse, is_function, parse_declaration
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 094_module_src_parser.c_30.md

## module_src_parser.c_31
- Module category: module_cluster
- Related headers: src/cflow.h, src/parser.h
- Representative functions: skip_struct, parse_typedef, parse_dcl, dcl, getident, dirdcl, parmdcl, maybe_parm_list
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 095_module_src_parser.c_31.md

## module_src_parser.c_32
- Module category: module_cluster
- Related headers: src/cflow.h, src/parser.h
- Representative functions: reference, reset_static_caller
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 096_module_src_parser.c_32.md

## module_src_posix.c_33
- Module category: module_cluster
- Related headers: src/cflow.h, src/parser.h
- Representative functions: print_symbol_type, posix_output_handler
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 097_module_src_posix.c_33.md

## module_src_symbol.c_34
- Module category: module_cluster
- Related headers: gnu/hash.h, src/cflow.h, src/parser.h
- Representative functions: symbol_unlink_from_list, ident_change_storage, init_ident, install_ident, delete_symbol, delete_statics, delete_autos, delete_parms
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 098_module_src_symbol.c_34.md

## module_src_parseopt_parseopt_01
- Module category: module_cluster
- Related headers: gnu/assert.h, gnu/inttypes.h, gnu/limits.h, gnu/stdio.h, gnu/stdlib.h, gnu/string.h, gnu/sys/types.h, src/parseopt/parseconf.h, src/parseopt/parseopt.h, src/parseopt/wordwrap.h
- Representative functions: set_usage_var, init_usage_vars, parseopt_usage_std, parseopt_usage_sdash, parseopt_usage_fd, parseopt_help_fd, parseopt_version_fd, optset_incr
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 099_module_src_parseopt_parseopt_01.md

## module_src_parseopt_parseopt_02
- Module category: module_cluster
- Related headers: gnu/assert.h, gnu/stdio.h, gnu/stdlib.h, gnu/string.h, src/parseopt/parseconf.h, src/parseopt/parseopt.h, src/parseopt/wordwrap.h
- Representative functions: option_find_short, negmatch, option_find_long, permute, parseopt_lookahead, parseopt_skip, parseopt_next_internal, parseopt_next
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 100_module_src_parseopt_parseopt_02.md

## module_src_parseopt_parseopt_03
- Module category: module_cluster
- Related headers: gnu/assert.h, gnu/stdio.h, gnu/stdlib.h, gnu/string.h, src/parseopt/parseconf.h, src/parseopt/parseopt.h, src/parseopt/wordwrap.h
- Representative functions: set_version, _parseopt_optgroup, parseopt_init0, parseopt_init, parseopt_free, parseopt_parse, parseopt_getopt, parseopt_optdef_by_code
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 101_module_src_parseopt_parseopt_03.md

## module_src_parseopt_optdef_04
- Module category: module_cluster
- Related headers: gnu/assert.h, gnu/limits.h, gnu/stdio.h, gnu/stdlib.h, gnu/string.h, src/parseopt/parseconf.h, src/parseopt/parseopt.h, src/parseopt/wordwrap.h
- Representative functions: print_arg, opt_unalias, merge, print_option_std, print_option_sdash, print_option, optcmp, sethead
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 102_module_src_parseopt_optdef_04.md

## module_src_parseopt_help_context_05
- Module category: module_cluster
- Related headers: gnu/assert.h, gnu/limits.h, gnu/stdio.h, gnu/stdlib.h, gnu/string.h, src/parseopt/parseconf.h, src/parseopt/parseopt.h, src/parseopt/wordwrap.h
- Representative functions: sort_options, print_option_group
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 103_module_src_parseopt_help_context_05.md

## module_src_parseopt_position_06
- Module category: module_cluster
- Related headers: gnu/limits.h, gnu/stdio.h, gnu/stdlib.h, gnu/string.h, gnu/unistd.h, gnu/wchar.h, src/parseopt/parseconf.h, src/parseopt/parseopt.h, src/parseopt/wordwrap.h
- Representative functions: position_init, position_incr, position_add, position_eq, wordwrap_last_ws, flush_line
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 104_module_src_parseopt_position_06.md

## module_src_parseopt_optsort_07
- Module category: module_cluster
- Related headers: gnu/assert.h, gnu/limits.h, gnu/stdio.h, gnu/stdlib.h, gnu/string.h, src/parseopt/parseconf.h, src/parseopt/parseopt.h, src/parseopt/wordwrap.h
- Representative functions: optsort, sortnames
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 105_module_src_parseopt_optsort_07.md

## module_src_parseopt_wordwrap_at_08
- Module category: module_cluster
- Related headers: gnu/limits.h, gnu/stdio.h, gnu/stdlib.h, gnu/string.h, gnu/unistd.h, gnu/wchar.h, src/parseopt/parseconf.h, src/parseopt/parseopt.h, src/parseopt/wordwrap.h
- Representative functions: wordwrap_at_bol, wordwrap_at_eol
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 106_module_src_parseopt_wordwrap_at_08.md

## module_src_parseopt_wordwrap_set_09
- Module category: module_cluster
- Related headers: gnu/limits.h, gnu/stdio.h, gnu/stdlib.h, gnu/string.h, gnu/unistd.h, gnu/wchar.h, src/parseopt/parseconf.h, src/parseopt/parseopt.h, src/parseopt/wordwrap.h
- Representative functions: wordwrap_set_left_margin, wordwrap_set_right_margin
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 107_module_src_parseopt_wordwrap_set_09.md

## module_src_parseopt_wordwrap_word_10
- Module category: module_cluster
- Related headers: gnu/limits.h, gnu/stdio.h, gnu/stdlib.h, gnu/string.h, gnu/unistd.h, gnu/wchar.h, src/parseopt/parseconf.h, src/parseopt/parseopt.h, src/parseopt/wordwrap.h
- Representative functions: wordwrap_word_start, wordwrap_word_end
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 108_module_src_parseopt_wordwrap_word_10.md

## module_src_parseopt_help.c_11
- Module category: module_cluster
- Related headers: gnu/assert.h, gnu/limits.h, gnu/stdio.h, gnu/stdlib.h, gnu/string.h, src/parseopt/parseconf.h, src/parseopt/parseopt.h, src/parseopt/wordwrap.h
- Representative functions: min
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 109_module_src_parseopt_help.c_11.md

## module_src_parseopt_optset.c_12
- Module category: module_cluster
- Related headers: gnu/assert.h, gnu/inttypes.h, gnu/limits.h, gnu/stdlib.h, gnu/string.h, gnu/sys/types.h, src/parseopt/parseconf.h, src/parseopt/parseopt.h, src/parseopt/wordwrap.h
- Representative functions: get_signed_int, get_unsigned_int
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 110_module_src_parseopt_optset.c_12.md

## module_src_parseopt_wordwrap.c_13
- Module category: module_cluster
- Related headers: gnu/limits.h, gnu/stdio.h, gnu/stdlib.h, gnu/string.h, gnu/unistd.h, gnu/wchar.h, src/parseopt/parseconf.h, src/parseopt/parseopt.h, src/parseopt/wordwrap.h
- Representative functions: wordwrap_line_init, detect_right_margin, _ww_fd_writer, wordwrap_open, wordwrap_fdopen, wordwrap_close, full_write, safe_mbrtowc
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 111_module_src_parseopt_wordwrap.c_13.md

## module_src_parseopt_wordwrap.c_14
- Module category: module_cluster
- Related headers: gnu/limits.h, gnu/stdio.h, gnu/stdlib.h, gnu/string.h, gnu/unistd.h, gnu/wchar.h, src/parseopt/parseconf.h, src/parseopt/parseopt.h, src/parseopt/wordwrap.h
- Representative functions: wordwrap_putc, wordwrap_para, wordwrap_vprintf, wordwrap_printf
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 112_module_src_parseopt_wordwrap.c_14.md

## module_src_wordsplit_wordsplit_01
- Module category: module_cluster
- Related headers: gnu/limits.h, gnu/stdio.h, gnu/stdlib.h, gnu/string.h, gnu/unistd.h, src/wordsplit/wordsplit.h
- Representative functions: is_name_char, _wsplt_alloc_die, _wsplt_seterr, _wsplt_nomem, _wsplt_store_errctx, _wsplt_setctxerr, _wsplt_subsplit, _wsplt_seterr_sub
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 113_module_src_wordsplit_wordsplit_01.md

## module_src_wordsplit_wordsplit_02
- Module category: module_cluster
- Related headers: gnu/limits.h, gnu/stdio.h, gnu/stdlib.h, gnu/string.h, gnu/unistd.h, src/wordsplit/wordsplit.h
- Representative functions: wsnode_insert, wordsplit_add_segm, wordsplit_free_nodes, wordsplit_dump_nodes, coalesce_segment, wsnode_quoteremoval, wsnode_coalesce, wsnode_tail_coalesce
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 114_module_src_wordsplit_wordsplit_02.md

## module_src_wordsplit_wordsplit_03
- Module category: module_cluster
- Related headers: gnu/limits.h, gnu/stdio.h, gnu/stdlib.h, gnu/string.h, gnu/unistd.h, src/wordsplit/wordsplit.h
- Representative functions: expvar_recover, expand_paramv, expvar, node_expand, wsnode_nullelim, wordsplit_varexp, expcmd, wordsplit_cmdexp
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 115_module_src_wordsplit_wordsplit_03.md

## module_src_wordsplit_wordsplit_04
- Module category: module_cluster
- Related headers: gnu/limits.h, gnu/stdio.h, gnu/stdlib.h, gnu/string.h, gnu/unistd.h, src/wordsplit/wordsplit.h
- Representative functions: wordsplit_tildexpand, wordsplit_pathexpand, skip_delim_internal, skip_delim, skip_delim_real, scan_qstring, scan_word, wordsplit_string_unquote_copy
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 116_module_src_wordsplit_wordsplit_04.md

## module_src_wordsplit_wordsplit_05
- Module category: module_cluster
- Related headers: gnu/limits.h, gnu/stdio.h, gnu/stdlib.h, gnu/string.h, gnu/unistd.h, src/wordsplit/wordsplit.h
- Representative functions: wordsplit_free_parambuf, wordsplit_clearerr, wordsplit_free, wordsplit_get_words, wordsplit_strerror, wordsplit_perror
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 117_module_src_wordsplit_wordsplit_05.md

## module_src_wordsplit_wordsplit_node_06
- Module category: module_cluster
- Related headers: gnu/limits.h, gnu/stdio.h, gnu/stdlib.h, gnu/string.h, gnu/unistd.h, src/wordsplit/wordsplit.h
- Representative functions: wsnode_len, wsnode_free, wsnode_tail
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 118_module_src_wordsplit_wordsplit_node_06.md

## module_src_wordsplit_wordsplit_c_07
- Module category: module_cluster
- Related headers: gnu/limits.h, gnu/stdio.h, gnu/stdlib.h, gnu/string.h, gnu/unistd.h, src/wordsplit/wordsplit.h
- Representative functions: wordsplit_c_quoted_length, wordsplit_c_unquote_char, wordsplit_c_quote_char, wordsplit_c_quote_copy
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 119_module_src_wordsplit_wordsplit_c_07.md

## module_src_wordsplit_wordsplit.c_08
- Module category: module_cluster
- Related headers: gnu/limits.h, gnu/stdio.h, gnu/stdlib.h, gnu/string.h, gnu/unistd.h, src/wordsplit/wordsplit.h
- Representative functions: _wsplt_error, wsnode_flagstr, wordsplit_append, find_closing_paren, begin_var_p, begin_cmd_p, isglob, skip_sed_expr
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 120_module_src_wordsplit_wordsplit.c_08.md

## module_test
- Module category: module
- Related headers: gnu/stdio.h
- Representative functions: helper, twice, run, fib, fact, main, add, mul
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 121_module_test.md
