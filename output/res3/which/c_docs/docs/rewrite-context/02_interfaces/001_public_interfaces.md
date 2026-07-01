# which Public Interface Overview

This index is intended for the later Rust rewrite stage. It preserves module-level interface entry points instead of compressing all interface facts into a single overly long document.

- Interface module count: 2

## main_root
- Module category: main
- Related headers: bash.h, config.h, getopt.h, posixstat.h, sys.h, tilde/tilde.h
- Representative functions: uidget, getmaxgroups, initialize_group_array, group_member, file_status, absolute_program, substring, extract_colon_unit
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 002_main_root.md

## module_tilde
- Module category: module
- Related headers: tilde/ansi_stdlib.h, tilde/bashansi.h, tilde/tilde.h, tilde/xmalloc.h
- Representative functions: get_home_dir, tilde_find_prefix, tilde_find_suffix, memory_error_and_abort
- Representative structs: anonymous, anonymous, anonymous, anonymous, anonymous
- Detailed document: 003_module_tilde.md
