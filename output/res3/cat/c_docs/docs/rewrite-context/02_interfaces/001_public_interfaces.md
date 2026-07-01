# cat Public Interface Overview

This index is intended for the later Rust rewrite stage. It preserves module-level interface entry points instead of compressing all interface facts into a single overly long document.

- Interface module count: 39

## module_include
- Module category: module
- Related headers: include/alignalloc.h, include/alloca.h, include/arg-nonnull.h, include/attribute.h, include/basename-lgpl.h, include/binary-io.h, include/c-ctype.h, include/c-strcase.h, include/c-strcaseeq.h, include/c32is-impl.h, include/close-stream.h, include/closein.h, include/closeout.h, include/config.h, include/configmake.h, include/ctype.h, include/dirent.h, include/dirname.h, include/error.h, include/exitfail.h, include/fadvise.h, include/fcntl.h, include/filename.h, include/fpending.h, include/freading.h, include/full-write.h, include/gettext.h, include/hard-locale.h, include/ialloc.h, include/idx.h, include/ignore-value.h, include/intprops-internal.h, include/intprops.h, include/inttostr.h, include/inttypes.h, include/ioblksize.h, include/langinfo.h, include/limits.h, include/localcharset.h, include/locale.h, include/minmax.h, include/openat.h, include/pathmax.h, include/progname.h, include/propername.h, include/quote.h, include/quotearg.h, include/safe-read.h, include/safe-write.h, include/same-inode.h, include/setlocale_null.h, include/signal.h, include/stat-macros.h, include/stat-size.h, include/stdbit.h, include/stdckdint.h, include/stddef.h, include/stdio-impl.h, include/stdio.h, include/stdlib.h, include/streq.h, include/string.h, include/sys-limits.h, include/sys/types.h, include/system.h, include/time.h, include/timespec.h, include/uchar.h, include/unictype.h, include/unistd.h, include/unitypes.h, include/unlocked-io.h, include/verify.h, include/version-etc.h, include/version.h, include/wchar.h, include/wctype.h, include/xalloc.h, include/xbinary-io.h
- Representative functions: safe_rw
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 002_module_include.md

## main_root_quoting_options_01
- Module category: main_cluster
- Related headers: include/attribute.h, include/c-strcaseeq.h, include/config.h, include/ctype.h, include/gettext.h, include/limits.h, include/localcharset.h, include/minmax.h, include/quote.h, include/quotearg.h, include/stdlib.h, include/string.h, include/uchar.h, include/wchar.h, include/xalloc.h
- Representative functions: clone_quoting_options, get_quoting_style, set_quoting_style, set_char_quoting, set_quoting_flags, set_custom_quoting, quoting_options_from_style, quotearg_buffer
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 003_main_root_quoting_options_01.md

## main_root_quoting_options_02
- Module category: main_cluster
- Related headers: include/attribute.h, include/c-strcaseeq.h, include/config.h, include/ctype.h, include/gettext.h, include/limits.h, include/localcharset.h, include/minmax.h, include/quote.h, include/quotearg.h, include/stdlib.h, include/string.h, include/uchar.h, include/wchar.h, include/xalloc.h
- Representative functions: quotearg_n_custom_mem
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 004_main_root_quoting_options_02.md

## main_root_stat_03
- Module category: main_cluster
- Related headers: include/alignalloc.h, include/config.h, include/fadvise.h, include/fcntl.h, include/full-write.h, include/ioblksize.h, include/limits.h, include/safe-read.h, include/stdio.h, include/stdlib.h, include/sys/ioctl.h, include/sys/types.h, include/system.h, include/unistd.h, include/xbinary-io.h
- Representative functions: main, klibc_fcntl
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 005_main_root_stat_03.md

## main_root_setlocale_null_04
- Module category: main_cluster
- Related headers: include/config.h, include/locale.h, include/setlocale_null.h, include/stdlib.h, include/string.h
- Representative functions: setlocale_null_unlocked, setlocale_null_r_unlocked, setlocale_null_r_with_lock, setlocale_null_r_with_lock, setlocale_null_r_with_lock, setlocale_null_r, setlocale_null
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 006_main_root_setlocale_null_04.md

## main_root_version_etc_05
- Module category: main_cluster
- Related headers: include/config.h, include/gettext.h, include/stdio.h, include/version-etc.h
- Representative functions: version_etc_arn, version_etc_ar, version_etc_va, version_etc
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 007_main_root_version_etc_05.md

## main_root_close_stdout_06
- Module category: main_cluster
- Related headers: include/close-stream.h, include/closeout.h, include/config.h, include/error.h, include/exitfail.h, include/gettext.h, include/quotearg.h, include/stdio.h, include/unistd.h
- Representative functions: close_stdout_set_file_name, close_stdout_set_ignore_EPIPE, close_stdout
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 008_main_root_close_stdout_06.md

## main_root_quotearg_n_07
- Module category: main_cluster
- Related headers: include/attribute.h, include/c-strcaseeq.h, include/config.h, include/ctype.h, include/gettext.h, include/limits.h, include/localcharset.h, include/minmax.h, include/quote.h, include/quotearg.h, include/stdlib.h, include/string.h, include/uchar.h, include/wchar.h, include/xalloc.h
- Representative functions: quotearg_n, quotearg_n_mem, quotearg_n_custom
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 009_main_root_quotearg_n_07.md

## main_root_clear_ungetc_08
- Module category: main_cluster
- Related headers: include/config.h, include/freading.h, include/stdio-impl.h, include/stdio.h, include/unistd.h
- Representative functions: clear_ungetc_buffer_preserving_position, clear_ungetc_buffer
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 010_main_root_clear_ungetc_08.md

## main_root_mbrtoc32_09
- Module category: main_cluster
- Related headers: include/attribute.h, include/config.h, include/stdlib.h, include/uchar.h
- Representative functions: mbrtoc32, mbrtoc32
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 011_main_root_mbrtoc32_09.md

## main_root_quote_n_10
- Module category: main_cluster
- Related headers: include/attribute.h, include/c-strcaseeq.h, include/config.h, include/ctype.h, include/gettext.h, include/limits.h, include/localcharset.h, include/minmax.h, include/quote.h, include/quotearg.h, include/stdlib.h, include/string.h, include/uchar.h, include/wchar.h, include/xalloc.h
- Representative functions: quote_n_mem, quote_n
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 012_main_root_quote_n_10.md

## main_root_quotearg_colon_11
- Module category: main_cluster
- Related headers: include/attribute.h, include/c-strcaseeq.h, include/config.h, include/ctype.h, include/gettext.h, include/limits.h, include/localcharset.h, include/minmax.h, include/quote.h, include/quotearg.h, include/stdlib.h, include/string.h, include/uchar.h, include/wchar.h, include/xalloc.h
- Representative functions: quotearg_colon, quotearg_colon_mem
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 013_main_root_quotearg_colon_11.md

## main_root_quotearg_custom_12
- Module category: main_cluster
- Related headers: include/attribute.h, include/c-strcaseeq.h, include/config.h, include/ctype.h, include/gettext.h, include/limits.h, include/localcharset.h, include/minmax.h, include/quote.h, include/quotearg.h, include/stdlib.h, include/string.h, include/uchar.h, include/wchar.h, include/xalloc.h
- Representative functions: quotearg_custom, quotearg_custom_mem
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 014_main_root_quotearg_custom_12.md

## main_root_quotearg_style_13
- Module category: main_cluster
- Related headers: include/attribute.h, include/c-strcaseeq.h, include/config.h, include/ctype.h, include/gettext.h, include/limits.h, include/localcharset.h, include/minmax.h, include/quote.h, include/quotearg.h, include/stdlib.h, include/string.h, include/uchar.h, include/wchar.h, include/xalloc.h
- Representative functions: quotearg_style, quotearg_style_mem
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 015_main_root_quotearg_style_13.md

## main_root_rpl_fcntl_14
- Module category: main_cluster
- Related headers: include/config.h, include/fcntl.h, include/limits.h, include/stdlib.h, include/unistd.h
- Representative functions: rpl_fcntl_DUPFD, rpl_fcntl_DUPFD_CLOEXEC
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 016_main_root_rpl_fcntl_14.md

## main_root_safe_rw_15
- Module category: main_cluster
- Related headers: include/config.h, include/safe-read.h, include/sys-limits.h, include/sys/types.h, include/unistd.h
- Representative functions: safe_rw, safe_rw
- Representative structs: anonymous
- Detailed document: 017_main_root_safe_rw_15.md

## main_root_alignalloc.c_16
- Module category: main_cluster
- Related headers: include/alignalloc.h, include/config.h, include/limits.h, include/stdckdint.h
- Representative functions: align_down, address_of_pointer_to_malloced, alignalloc, alignfree
- Representative structs: none
- Detailed document: 018_main_root_alignalloc.c_16.md

## main_root_binary-io.c_17
- Module category: main_cluster
- Related headers: include/binary-io.h, include/config.h
- Representative functions: set_binary_mode
- Representative structs: none
- Detailed document: 019_main_root_binary-io.c_17.md

## main_root_c-strcasecmp.c_18
- Module category: main_cluster
- Related headers: include/c-ctype.h, include/c-strcase.h, include/config.h, include/limits.h
- Representative functions: c_strcasecmp
- Representative structs: none
- Detailed document: 020_main_root_c-strcasecmp.c_18.md

## main_root_cat.c_19
- Module category: main_cluster
- Related headers: include/alignalloc.h, include/config.h, include/fadvise.h, include/full-write.h, include/ioblksize.h, include/safe-read.h, include/stdio.h, include/sys/ioctl.h, include/sys/types.h, include/system.h, include/xbinary-io.h
- Representative functions: usage, next_line_num, simple_cat, write_pending, cat, copy_cat
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 021_main_root_cat.c_19.md

## main_root_close-stream.c_20
- Module category: main_cluster
- Related headers: include/close-stream.h, include/config.h, include/fpending.h
- Representative functions: close_stream
- Representative structs: none
- Detailed document: 022_main_root_close-stream.c_20.md

## main_root_copy-file-range.c_21
- Module category: main_cluster
- Related headers: include/config.h, include/unistd.h
- Representative functions: copy_file_range
- Representative structs: anonymous
- Detailed document: 023_main_root_copy-file-range.c_21.md

## main_root_fadvise.c_22
- Module category: main_cluster
- Related headers: include/config.h, include/fadvise.h, include/fcntl.h, include/ignore-value.h, include/stdio.h
- Representative functions: fdadvise, fadvise
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 024_main_root_fadvise.c_22.md

## main_root_fclose.c_23
- Module category: main_cluster
- Related headers: include/config.h, include/freading.h, include/stdio.h, include/unistd.h
- Representative functions: fclose_nothrow, rpl_fclose
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 025_main_root_fclose.c_23.md

## main_root_fcntl.c_24
- Module category: main_cluster
- Related headers: include/config.h, include/fcntl.h, include/limits.h, include/stdlib.h, include/unistd.h
- Representative functions: dupfd
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 026_main_root_fcntl.c_24.md

## main_root_fflush.c_25
- Module category: main_cluster
- Related headers: include/config.h, include/freading.h, include/stdio-impl.h, include/stdio.h, include/unistd.h
- Representative functions: disable_seek_optimization, restore_seek_optimization, update_fpos_cache, rpl_fflush
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 027_main_root_fflush.c_25.md

## main_root_fpurge.c_26
- Module category: main_cluster
- Related headers: include/config.h, include/stdio-impl.h, include/stdio.h, include/stdlib.h
- Representative functions: fpurge
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 028_main_root_fpurge.c_26.md

## main_root_full-write.c_27
- Module category: main_cluster
- Related headers: include/config.h, include/full-write.h
- Representative functions: full_rw
- Representative structs: none
- Detailed document: 029_main_root_full-write.c_27.md

## main_root_hard-locale.c_28
- Module category: main_cluster
- Related headers: include/config.h, include/hard-locale.h, include/locale.h, include/stdlib.h, include/string.h
- Representative functions: hard_locale
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 030_main_root_hard-locale.c_28.md

## main_root_localcharset.c_29
- Module category: main_cluster
- Related headers: include/config.h, include/localcharset.h, include/stddef.h, include/stdio.h, include/stdlib.h, include/string.h
- Representative functions: locale_charset
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 031_main_root_localcharset.c_29.md

## main_root_progname.c_30
- Module category: main_cluster
- Related headers: include/config.h, include/progname.h, include/stdio.h, include/stdlib.h, include/string.h
- Representative functions: set_program_name
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 032_main_root_progname.c_30.md

## main_root_propername-lite.c_31
- Module category: main_cluster
- Related headers: include/c-strcase.h, include/config.h, include/gettext.h, include/localcharset.h, include/propername.h
- Representative functions: proper_name_lite
- Representative structs: none
- Detailed document: 033_main_root_propername-lite.c_31.md

## main_root_quotearg.c_32
- Module category: main_cluster
- Related headers: include/attribute.h, include/c-strcaseeq.h, include/config.h, include/ctype.h, include/gettext.h, include/limits.h, include/localcharset.h, include/minmax.h, include/quote.h, include/quotearg.h, include/stdlib.h, include/string.h, include/uchar.h, include/wchar.h, include/xalloc.h
- Representative functions: gettext_quote, quotearg_buffer_restyled, quotearg_free, quotearg, quotearg_mem, quotearg_char, quote_mem, quote
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 034_main_root_quotearg.c_32.md

## main_root_version-etc.c_33
- Module category: main_cluster
- Related headers: include/config.h, include/gettext.h, include/stdio.h, include/version-etc.h
- Representative functions: emit_bug_reporting_address
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 035_main_root_version-etc.c_33.md

## main_root_xalignalloc.c_34
- Module category: main_cluster
- Related headers: include/alignalloc.h, include/config.h, include/xalloc.h
- Representative functions: xalignalloc
- Representative structs: none
- Detailed document: 036_main_root_xalignalloc.c_34.md

## main_root_xalloc-die.c_35
- Module category: main_cluster
- Related headers: include/config.h, include/error.h, include/exitfail.h, include/gettext.h, include/stdlib.h, include/xalloc.h
- Representative functions: xalloc_die
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 037_main_root_xalloc-die.c_35.md

## main_root_xbinary-io.c_36
- Module category: main_cluster
- Related headers: include/config.h, include/error.h, include/exitfail.h, include/gettext.h, include/verify.h, include/xbinary-io.h
- Representative functions: xset_binary_mode_error
- Representative structs: none
- Detailed document: 038_main_root_xbinary-io.c_36.md

## main_root_xmalloc.c_37
- Module category: main_cluster
- Related headers: include/config.h, include/ialloc.h, include/minmax.h, include/stdckdint.h, include/stdlib.h, include/string.h, include/xalloc.h
- Representative functions: _GL_ATTRIBUTE_PURE, xmalloc, ximalloc, xcharalloc, xrealloc, xirealloc, xreallocarray, xireallocarray
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 039_main_root_xmalloc.c_37.md

## main_root_xmalloc.c_38
- Module category: main_cluster
- Related headers: include/config.h, include/ialloc.h, include/minmax.h, include/stdckdint.h, include/stdlib.h, include/string.h, include/xalloc.h
- Representative functions: xcalloc, xicalloc, xmemdup, ximemdup, ximemdup0, xstrdup
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 040_main_root_xmalloc.c_38.md
