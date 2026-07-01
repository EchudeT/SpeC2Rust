# Interface Facts: module_src_wordsplit_wordsplit_c_07

This document is intended for the later Rust repository-level rewrite and keeps only the interface facts directly observed in the current source-analysis stage.
Header files, macros, error codes, and configuration items that do not appear in the current parse results will not be added or assumed.

## Module Scope
- Module category: `module_cluster`
- Directory: `src/wordsplit`
- File list: src/wordsplit/wordsplit.c
- Candidate header files: gnu/limits.h, gnu/stdio.h, gnu/stdlib.h, gnu/string.h, gnu/unistd.h, src/wordsplit/wordsplit.h
- Exported functions observed: 4
- Struct definitions observed: 161
- Type names referenced but not defined locally: 0
- Macros observed in related files: 20
- Global variables observed: 0

## Header Evidence
- `gnu/limits.h` [gnu/limits.h]
- `gnu/stdio.h` [gnu/stdio.h]
- `gnu/stdlib.h` [gnu/stdlib.h]
- `gnu/string.h` [gnu/string.h]
- `gnu/unistd.h` [gnu/unistd.h]
- `src/wordsplit/wordsplit.h` [src/wordsplit/wordsplit.h]

## Functions
### `wordsplit_c_quoted_length`
- Definition location: [src/wordsplit/wordsplit.c:2392-2420]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration: `size_t wordsplit_c_quoted_length (const char *str, int quote_hex, int *quote);`
- Approximate function body length: 29 lines
### `wordsplit_c_unquote_char`
- Definition location: [src/wordsplit/wordsplit.c:2445-2449]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration: `int wordsplit_c_unquote_char (int c);`
- Approximate function body length: 5 lines
### `wordsplit_c_quote_char`
- Definition location: [src/wordsplit/wordsplit.c:2451-2455]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration: `int wordsplit_c_quote_char (int c);`
- Approximate function body length: 5 lines
### `wordsplit_c_quote_copy`
- Definition location: [src/wordsplit/wordsplit.c:2535-2572]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration: `void wordsplit_c_quote_copy (char *dst, const char *src, int quote_hex);`
- Approximate function body length: 38 lines

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
- Definition location: [src/wordsplit/wordsplit.c:56]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:76]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:96]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:99]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:110]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:125]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:142]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:149]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:152]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:154]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:155]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:158]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:158]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:214]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:214]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:234]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:264]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:372]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:416-430]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit_node`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:418]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit_node`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:419]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit_node`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:423-427]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:469]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:469]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit_node`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:480]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit_node`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:491]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:491]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit_node`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:493]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit_node`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:501]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit_node`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:509]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:509]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit_node`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:521]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:521]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit_node`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:523]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit_node`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:544]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit_node`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:545]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit_node`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:553]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:553]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit_node`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:554]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit_node`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:567]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit_node`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:576]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit_node`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:577]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit_node`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:591]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:593]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit_node`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:609]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:611]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit_node`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:615]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit_node`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:623]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:625]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit_node`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:644]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:644]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit_node`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:646]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit_node`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:670]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit_node`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:697]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:702]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:704]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit_node`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:732]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:734]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit_node`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:746]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:746]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit_node`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:750]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit_node`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:762]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:765]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:767]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit_node`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:801]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit_node`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:823]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit_node`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:955]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:956]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit_node`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:957]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit_node`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:960]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit_node`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:1046]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:1085]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:1104]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:1111]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:1219]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:1285]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:1286]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit_node`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:1288]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit_node`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:1306]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:1306]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit_node`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:1309]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:1314]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit_node`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:1318]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit_node`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:1360]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit_node`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:1361]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit_node`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:1379]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:1380]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit_node`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:1385]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit_node`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:1388]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:1678]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:1732]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:1732]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit_node`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:1734]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:1736]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit_node`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:1745]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit_node`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:1787]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:1789]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit_node`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:1793]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit_node`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:1805]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:1807]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit_node`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:1811]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit_node`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:1829]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:1830]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit_node`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:1835]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit_node`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:1836]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:1895]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:1928]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:1930]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit_node`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:1934]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit_node`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:1949]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:1951]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit_node`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:1985]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:1987]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit_node`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:2003]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct passwd`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:2069]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:2071]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit_node`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:2098]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit_node`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:2149]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit_node`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:2207]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:2213]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:2219]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:2229]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:2258]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:2265]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit_node`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:2458]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:2576-2582]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct exptab`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:2581]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:2594]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct exptab`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:2615]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct exptab`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:2615]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:2629]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:2631]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct exptab`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:2696]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:2739]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:2746]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:2752]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:2769]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:2786]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:2803]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:2816]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:2832]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:2865]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit`
### `anonymous`
- Definition location: [src/wordsplit/wordsplit.c:2875]
- Source file: `src/wordsplit/wordsplit.c`
- Observed declaration prefix: `struct wordsplit`
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
- No external struct or type references beyond local definitions were recorded.

## Macros and Constants
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
- `_GL_CXXDEFS_H` [gnu/stdio.h:255]: `#define _GL_CXXDEFS_H`
- `_GL_FUNCDECL_RPL` [gnu/stdio.h:346-347]: `#define _GL_FUNCDECL_RPL(func,rettype,parameters_and_attributes) \ _GL_FUNCDECL_RPL_1 (rpl_##func, rettype, parameters_and_attributes)`
- `_GL_FUNCDECL_RPL_1` [gnu/stdio.h:348-349]: `#define _GL_FUNCDECL_RPL_1(rpl_func,rettype,parameters_and_attributes) \ _GL_EXTERN_C rettype rpl_func parameters_and_attributes`
- `_GL_FUNCDECL_SYS` [gnu/stdio.h:358-359]: `#define _GL_FUNCDECL_SYS(func,rettype,parameters_and_attributes) \ _GL_EXTERN_C rettype func parameters_and_attributes`
- `_GL_CXXALIAS_RPL` [gnu/stdio.h:370-371]: `#define _GL_CXXALIAS_RPL(func,rettype,parameters) \ _GL_CXXALIAS_RPL_1 (func, rpl_##func, rettype, parameters)`
- `_GL_CXXALIAS_MDA` [gnu/stdio.h:399-400]: `#define _GL_CXXALIAS_MDA(func,rettype,parameters) \ _GL_CXXALIAS_RPL_1 (func, _##func, rettype, parameters)`
- `_GL_CXXALIAS_MDA_CAST` [gnu/stdio.h:432-433]: `#define _GL_CXXALIAS_MDA_CAST(func,rettype,parameters) \ _GL_CXXALIAS_RPL_CAST_1 (func, _##func, rettype, parameters)`

## Global Variables
- No global variable definitions were observed in the current module's `.c` files.

## Known Gaps
- This document is generated from parsed results for functions, structs, macros, and global variables; it does not infer declaration signatures from `.h` files that were not parsed.
- If a function appears in the "Functions" section without an explicit header binding, the later Rust migration should re-check the corresponding source `#include` relationships and build scripts.
- Error codes, configuration items, and input/output protocols are recorded only when explicit symbols appear in the source; missing entries do not mean the semantics do not exist, only that the current fact extraction did not observe them.
