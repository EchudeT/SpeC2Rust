# pwd Public Interface Overview

This index is intended for the later Rust rewrite stage. It preserves module-level interface entry points instead of compressing all interface facts into a single overly long document.

- Interface module count: 29

## main_root_quoting_options_01
- Module category: main_cluster
- Related headers: include/attribute.h, include/c-strcaseeq.h, include/config.h, include/ctype.h, include/gettext.h, include/limits.h, include/localcharset.h, include/minmax.h, include/quote.h, include/quotearg.h, include/stdlib.h, include/string.h, include/uchar.h, include/wchar.h, include/xalloc.h
- Representative functions: clone_quoting_options, get_quoting_style, set_quoting_style, set_char_quoting, set_quoting_flags, set_custom_quoting, quoting_options_from_style, quotearg_buffer
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 002_main_root_quoting_options_01.md

## main_root_quoting_options_02
- Module category: main_cluster
- Related headers: include/attribute.h, include/c-strcaseeq.h, include/config.h, include/ctype.h, include/gettext.h, include/limits.h, include/localcharset.h, include/minmax.h, include/quote.h, include/quotearg.h, include/stdlib.h, include/string.h, include/uchar.h, include/wchar.h, include/xalloc.h
- Representative functions: quotearg_n_custom_mem
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 003_main_root_quoting_options_02.md

## main_root_file_name_03
- Module category: main_cluster
- Related headers: include/config.h, include/quote.h, include/root-dev-ino.h, include/stdio.h, include/sys/types.h, include/system.h, include/xgetcwd.h
- Representative functions: file_name_free, file_name_init, file_name_prepend, find_dir_entry, robust_getcwd, main
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 004_main_root_file_name_03.md

## main_root_stat_04
- Module category: main_cluster
- Related headers: include/config.h, include/quote.h, include/root-dev-ino.h, include/stdio.h, include/stdlib.h, include/sys/types.h, include/system.h, include/xgetcwd.h
- Representative functions: logical_getcwd, get_root_dev_ino
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 005_main_root_stat_04.md

## main_root_setlocale_null_05
- Module category: main_cluster
- Related headers: include/config.h, include/locale.h, include/setlocale_null.h, include/stdlib.h, include/string.h
- Representative functions: setlocale_null_unlocked, setlocale_null_r_unlocked, setlocale_null_r_with_lock, setlocale_null_r_with_lock, setlocale_null_r_with_lock, setlocale_null_r, setlocale_null
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 006_main_root_setlocale_null_05.md

## main_root_version_etc_06
- Module category: main_cluster
- Related headers: include/config.h, include/gettext.h, include/stdio.h, include/version-etc.h
- Representative functions: version_etc_arn, version_etc_ar, version_etc_va, version_etc
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 007_main_root_version_etc_06.md

## main_root_close_stdout_07
- Module category: main_cluster
- Related headers: include/close-stream.h, include/closeout.h, include/config.h, include/error.h, include/exitfail.h, include/gettext.h, include/quotearg.h, include/stdio.h, include/unistd.h
- Representative functions: close_stdout_set_file_name, close_stdout_set_ignore_EPIPE, close_stdout
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 008_main_root_close_stdout_07.md

## main_root_quotearg_n_08
- Module category: main_cluster
- Related headers: include/attribute.h, include/c-strcaseeq.h, include/config.h, include/ctype.h, include/gettext.h, include/limits.h, include/localcharset.h, include/minmax.h, include/quote.h, include/quotearg.h, include/stdlib.h, include/string.h, include/uchar.h, include/wchar.h, include/xalloc.h
- Representative functions: quotearg_n, quotearg_n_mem, quotearg_n_custom
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 009_main_root_quotearg_n_08.md

## main_root_clear_ungetc_09
- Module category: main_cluster
- Related headers: include/config.h, include/freading.h, include/stdio-impl.h, include/stdio.h, include/unistd.h
- Representative functions: clear_ungetc_buffer_preserving_position, clear_ungetc_buffer
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 010_main_root_clear_ungetc_09.md

## main_root_mbrtoc32_10
- Module category: main_cluster
- Related headers: include/attribute.h, include/config.h, include/stdlib.h, include/uchar.h
- Representative functions: mbrtoc32, mbrtoc32
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 011_main_root_mbrtoc32_10.md

## main_root_quote_n_11
- Module category: main_cluster
- Related headers: include/attribute.h, include/c-strcaseeq.h, include/config.h, include/ctype.h, include/gettext.h, include/limits.h, include/localcharset.h, include/minmax.h, include/quote.h, include/quotearg.h, include/stdlib.h, include/string.h, include/uchar.h, include/wchar.h, include/xalloc.h
- Representative functions: quote_n_mem, quote_n
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 012_main_root_quote_n_11.md

## main_root_quotearg_colon_12
- Module category: main_cluster
- Related headers: include/attribute.h, include/c-strcaseeq.h, include/config.h, include/ctype.h, include/gettext.h, include/limits.h, include/localcharset.h, include/minmax.h, include/quote.h, include/quotearg.h, include/stdlib.h, include/string.h, include/uchar.h, include/wchar.h, include/xalloc.h
- Representative functions: quotearg_colon, quotearg_colon_mem
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 013_main_root_quotearg_colon_12.md

## main_root_quotearg_custom_13
- Module category: main_cluster
- Related headers: include/attribute.h, include/c-strcaseeq.h, include/config.h, include/ctype.h, include/gettext.h, include/limits.h, include/localcharset.h, include/minmax.h, include/quote.h, include/quotearg.h, include/stdlib.h, include/string.h, include/uchar.h, include/wchar.h, include/xalloc.h
- Representative functions: quotearg_custom, quotearg_custom_mem
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 014_main_root_quotearg_custom_13.md

## main_root_quotearg_style_14
- Module category: main_cluster
- Related headers: include/attribute.h, include/c-strcaseeq.h, include/config.h, include/ctype.h, include/gettext.h, include/limits.h, include/localcharset.h, include/minmax.h, include/quote.h, include/quotearg.h, include/stdlib.h, include/string.h, include/uchar.h, include/wchar.h, include/xalloc.h
- Representative functions: quotearg_style, quotearg_style_mem
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 015_main_root_quotearg_style_14.md

## main_root_c-strcasecmp.c_15
- Module category: main_cluster
- Related headers: include/c-ctype.h, include/c-strcase.h, include/config.h, include/limits.h
- Representative functions: c_strcasecmp
- Representative structs: none
- Detailed document: 016_main_root_c-strcasecmp.c_15.md

## main_root_close-stream.c_16
- Module category: main_cluster
- Related headers: include/close-stream.h, include/config.h, include/fpending.h
- Representative functions: close_stream
- Representative structs: none
- Detailed document: 017_main_root_close-stream.c_16.md

## main_root_fclose.c_17
- Module category: main_cluster
- Related headers: include/config.h, include/freading.h, include/stdio.h, include/unistd.h
- Representative functions: fclose_nothrow, rpl_fclose
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 018_main_root_fclose.c_17.md

## main_root_fflush.c_18
- Module category: main_cluster
- Related headers: include/config.h, include/freading.h, include/stdio-impl.h, include/stdio.h, include/unistd.h
- Representative functions: disable_seek_optimization, restore_seek_optimization, update_fpos_cache, rpl_fflush
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 019_main_root_fflush.c_18.md

## main_root_hard-locale.c_19
- Module category: main_cluster
- Related headers: include/config.h, include/hard-locale.h, include/locale.h, include/stdlib.h, include/string.h
- Representative functions: hard_locale
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 020_main_root_hard-locale.c_19.md

## main_root_localcharset.c_20
- Module category: main_cluster
- Related headers: include/config.h, include/localcharset.h, include/stddef.h, include/stdio.h, include/stdlib.h, include/string.h
- Representative functions: locale_charset
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 021_main_root_localcharset.c_20.md

## main_root_progname.c_21
- Module category: main_cluster
- Related headers: include/config.h, include/progname.h, include/stdio.h, include/stdlib.h, include/string.h
- Representative functions: set_program_name
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 022_main_root_progname.c_21.md

## main_root_propername-lite.c_22
- Module category: main_cluster
- Related headers: include/c-strcase.h, include/config.h, include/gettext.h, include/localcharset.h, include/propername.h
- Representative functions: proper_name_lite
- Representative structs: none
- Detailed document: 023_main_root_propername-lite.c_22.md

## main_root_pwd.c_23
- Module category: main_cluster
- Related headers: include/config.h, include/quote.h, include/root-dev-ino.h, include/stdio.h, include/sys/types.h, include/system.h, include/xgetcwd.h
- Representative functions: usage, nth_parent
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 024_main_root_pwd.c_23.md

## main_root_quotearg.c_24
- Module category: main_cluster
- Related headers: include/attribute.h, include/c-strcaseeq.h, include/config.h, include/ctype.h, include/gettext.h, include/limits.h, include/localcharset.h, include/minmax.h, include/quote.h, include/quotearg.h, include/stdlib.h, include/string.h, include/uchar.h, include/wchar.h, include/xalloc.h
- Representative functions: gettext_quote, quotearg_buffer_restyled, quotearg_free, quotearg, quotearg_mem, quotearg_char, quote_mem, quote
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 025_main_root_quotearg.c_24.md

## main_root_version-etc.c_25
- Module category: main_cluster
- Related headers: include/config.h, include/gettext.h, include/stdio.h, include/version-etc.h
- Representative functions: emit_bug_reporting_address
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 026_main_root_version-etc.c_25.md

## main_root_xalloc-die.c_26
- Module category: main_cluster
- Related headers: include/config.h, include/error.h, include/exitfail.h, include/gettext.h, include/stdlib.h, include/xalloc.h
- Representative functions: xalloc_die
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 027_main_root_xalloc-die.c_26.md

## main_root_xgetcwd.c_27
- Module category: main_cluster
- Related headers: include/config.h, include/unistd.h, include/xalloc.h, include/xgetcwd.h
- Representative functions: xgetcwd
- Representative structs: none
- Detailed document: 028_main_root_xgetcwd.c_27.md

## main_root_xmalloc.c_28
- Module category: main_cluster
- Related headers: include/config.h, include/ialloc.h, include/minmax.h, include/stdckdint.h, include/stdlib.h, include/string.h, include/xalloc.h
- Representative functions: _GL_ATTRIBUTE_PURE, xmalloc, ximalloc, xcharalloc, xrealloc, xirealloc, xreallocarray, xireallocarray
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 029_main_root_xmalloc.c_28.md

## main_root_xmalloc.c_29
- Module category: main_cluster
- Related headers: include/config.h, include/ialloc.h, include/minmax.h, include/stdckdint.h, include/stdlib.h, include/string.h, include/xalloc.h
- Representative functions: xcalloc, xicalloc, xmemdup, ximemdup, ximemdup0, xstrdup
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 030_main_root_xmalloc.c_29.md
