# Interface Facts: main_root_quotearg_custom_13

This document is intended for the later Rust repository-level rewrite and keeps only the interface facts directly observed in the current source-analysis stage.
Header files, macros, error codes, and configuration items that do not appear in the current parse results will not be added or assumed.

## Module Scope
- Module category: `main_cluster`
- Directory: `root`
- File list: quotearg.c
- Candidate header files: include/attribute.h, include/c-strcaseeq.h, include/config.h, include/ctype.h, include/gettext.h, include/limits.h, include/localcharset.h, include/minmax.h, include/quote.h, include/quotearg.h, include/stdlib.h, include/string.h, include/uchar.h, include/wchar.h, include/xalloc.h
- Exported functions observed: 2
- Struct definitions observed: 46
- Type names referenced but not defined locally: 0
- Macros observed in related files: 20
- Global variables observed: 0

## Header Evidence
- `include/attribute.h` [include/attribute.h]
- `include/c-strcaseeq.h` [include/c-strcaseeq.h]
- `include/config.h` [include/config.h]
- `include/ctype.h` [include/ctype.h]
- `include/gettext.h` [include/gettext.h]
- `include/limits.h` [include/limits.h]
- `include/localcharset.h` [include/localcharset.h]
- `include/minmax.h` [include/minmax.h]
- `include/quote.h` [include/quote.h]
- `include/quotearg.h` [include/quotearg.h]
- `include/stdlib.h` [include/stdlib.h]
- `include/string.h` [include/string.h]
- `include/uchar.h` [include/uchar.h]
- `include/wchar.h` [include/wchar.h]
- `include/xalloc.h` [include/xalloc.h]

## Functions
### `quotearg_custom`
- Definition location: [quotearg.c:1030-1035]
- Source file: `quotearg.c`
- Observed declaration: `har * quotearg_custom (char const *left_quote, char const *right_quote, char const *arg);`
- Approximate function body length: 6 lines
### `quotearg_custom_mem`
- Definition location: [quotearg.c:1037-1043]
- Source file: `quotearg.c`
- Observed declaration: `har * quotearg_custom_mem (char const *left_quote, char const *right_quote, char const *arg, size_t argsize);`
- Approximate function body length: 7 lines

## Structs and Types
### `anonymous`
- Definition location: [include/quote.h:30]
- Source file: `include/quote.h`
- Observed declaration prefix: `struct quoting_options`
### `anonymous`
- Definition location: [include/quotearg.h:286]
- Source file: `include/quotearg.h`
- Observed declaration prefix: `struct quoting_options`
### `anonymous`
- Definition location: [include/quotearg.h:294]
- Source file: `include/quotearg.h`
- Observed declaration prefix: `struct quoting_options`
### `anonymous`
- Definition location: [include/quotearg.h:294]
- Source file: `include/quotearg.h`
- Observed declaration prefix: `struct quoting_options`
### `anonymous`
- Definition location: [include/quotearg.h:299]
- Source file: `include/quotearg.h`
- Observed declaration prefix: `struct quoting_options`
### `anonymous`
- Definition location: [include/quotearg.h:303]
- Source file: `include/quotearg.h`
- Observed declaration prefix: `struct quoting_options`
### `anonymous`
- Definition location: [include/quotearg.h:312]
- Source file: `include/quotearg.h`
- Observed declaration prefix: `struct quoting_options`
### `anonymous`
- Definition location: [include/quotearg.h:318]
- Source file: `include/quotearg.h`
- Observed declaration prefix: `struct quoting_options`
### `anonymous`
- Definition location: [include/quotearg.h:329]
- Source file: `include/quotearg.h`
- Observed declaration prefix: `struct quoting_options`
### `anonymous`
- Definition location: [include/quotearg.h:346]
- Source file: `include/quotearg.h`
- Observed declaration prefix: `struct quoting_options`
### `anonymous`
- Definition location: [include/quotearg.h:352]
- Source file: `include/quotearg.h`
- Observed declaration prefix: `struct quoting_options`
### `anonymous`
- Definition location: [include/quotearg.h:363]
- Source file: `include/quotearg.h`
- Observed declaration prefix: `struct quoting_options`
### `anonymous`
- Definition location: [include/stdlib.h:84-93]
- Source file: `include/stdlib.h`
- Observed declaration prefix: `struct random_data`
### `anonymous`
- Definition location: [include/stdlib.h:1839]
- Source file: `include/stdlib.h`
- Observed declaration prefix: `struct random_data`
### `anonymous`
- Definition location: [include/stdlib.h:1841]
- Source file: `include/stdlib.h`
- Observed declaration prefix: `struct random_data`
### `anonymous`
- Definition location: [include/stdlib.h:1844]
- Source file: `include/stdlib.h`
- Observed declaration prefix: `struct random_data`
### `anonymous`
- Definition location: [include/stdlib.h:1847]
- Source file: `include/stdlib.h`
- Observed declaration prefix: `struct random_data`
### `anonymous`
- Definition location: [quotearg.c:57-74]
- Source file: `quotearg.c`
- Observed declaration prefix: `struct quoting_options`
### `anonymous`
- Definition location: [quotearg.c:108]
- Source file: `quotearg.c`
- Observed declaration prefix: `struct quoting_options`
### `anonymous`
- Definition location: [quotearg.c:113]
- Source file: `quotearg.c`
- Observed declaration prefix: `struct quoting_options`
### `anonymous`
- Definition location: [quotearg.c:114]
- Source file: `quotearg.c`
- Observed declaration prefix: `struct quoting_options`
### `anonymous`
- Definition location: [quotearg.c:117]
- Source file: `quotearg.c`
- Observed declaration prefix: `struct quoting_options`
### `anonymous`
- Definition location: [quotearg.c:125]
- Source file: `quotearg.c`
- Observed declaration prefix: `struct quoting_options`
### `anonymous`
- Definition location: [quotearg.c:133]
- Source file: `quotearg.c`
- Observed declaration prefix: `struct quoting_options`
### `anonymous`
- Definition location: [quotearg.c:144]
- Source file: `quotearg.c`
- Observed declaration prefix: `struct quoting_options`
### `anonymous`
- Definition location: [quotearg.c:160]
- Source file: `quotearg.c`
- Observed declaration prefix: `struct quoting_options`
### `anonymous`
- Definition location: [quotearg.c:171]
- Source file: `quotearg.c`
- Observed declaration prefix: `struct quoting_options`
### `anonymous`
- Definition location: [quotearg.c:184]
- Source file: `quotearg.c`
- Observed declaration prefix: `struct quoting_options`
### `anonymous`
- Definition location: [quotearg.c:187]
- Source file: `quotearg.c`
- Observed declaration prefix: `struct quoting_options`
### `anonymous`
- Definition location: [quotearg.c:782]
- Source file: `quotearg.c`
- Observed declaration prefix: `truct quoting_options`
### `anonymous`
- Definition location: [quotearg.c:784]
- Source file: `quotearg.c`
- Observed declaration prefix: `truct quoting_options`
### `anonymous`
- Definition location: [quotearg.c:795]
- Source file: `quotearg.c`
- Observed declaration prefix: `truct quoting_options`
### `anonymous`
- Definition location: [quotearg.c:808]
- Source file: `quotearg.c`
- Observed declaration prefix: `truct quoting_options`
### `anonymous`
- Definition location: [quotearg.c:810]
- Source file: `quotearg.c`
- Observed declaration prefix: `truct quoting_options`
### `anonymous`
- Definition location: [quotearg.c:829-833]
- Source file: `quotearg.c`
- Observed declaration prefix: `truct slotvec`
### `anonymous`
- Definition location: [quotearg.c:839]
- Source file: `quotearg.c`
- Observed declaration prefix: `truct slotvec`
### `anonymous`
- Definition location: [quotearg.c:840]
- Source file: `quotearg.c`
- Observed declaration prefix: `truct slotvec`
### `anonymous`
- Definition location: [quotearg.c:845]
- Source file: `quotearg.c`
- Observed declaration prefix: `truct slotvec`
### `anonymous`
- Definition location: [quotearg.c:874]
- Source file: `quotearg.c`
- Observed declaration prefix: `truct quoting_options`
### `anonymous`
- Definition location: [quotearg.c:878]
- Source file: `quotearg.c`
- Observed declaration prefix: `truct slotvec`
### `anonymous`
- Definition location: [quotearg.c:952]
- Source file: `quotearg.c`
- Observed declaration prefix: `truct quoting_options`
### `anonymous`
- Definition location: [quotearg.c:960]
- Source file: `quotearg.c`
- Observed declaration prefix: `truct quoting_options`
### `anonymous`
- Definition location: [quotearg.c:979]
- Source file: `quotearg.c`
- Observed declaration prefix: `truct quoting_options`
### `anonymous`
- Definition location: [quotearg.c:1006]
- Source file: `quotearg.c`
- Observed declaration prefix: `truct quoting_options`
### `anonymous`
- Definition location: [quotearg.c:1025]
- Source file: `quotearg.c`
- Observed declaration prefix: `truct quoting_options`
### `anonymous`
- Definition location: [quotearg.c:1047]
- Source file: `quotearg.c`
- Observed declaration prefix: `truct quoting_options`

## Referenced External Types
- No external struct or type references beyond local definitions were recorded.

## Macros and Constants
- `_GL_ATTRIBUTE_H` [include/attribute.h:31]: `#define _GL_ATTRIBUTE_H`
- `DEPRECATED` [include/attribute.h:70]: `#define DEPRECATED _GL_ATTRIBUTE_DEPRECATED`
- `ATTRIBUTE_WARNING` [include/attribute.h:74]: `#define ATTRIBUTE_WARNING(msg) _GL_ATTRIBUTE_WARNING (msg)`
- `ATTRIBUTE_ERROR` [include/attribute.h:78]: `#define ATTRIBUTE_ERROR(msg) _GL_ATTRIBUTE_ERROR (msg)`
- `ATTRIBUTE_MALLOC` [include/attribute.h:85]: `#define ATTRIBUTE_MALLOC _GL_ATTRIBUTE_MALLOC`
- `ATTRIBUTE_ALLOC_SIZE` [include/attribute.h:92]: `#define ATTRIBUTE_ALLOC_SIZE(args) _GL_ATTRIBUTE_ALLOC_SIZE (args)`
- `ATTRIBUTE_DEALLOC` [include/attribute.h:100]: `#define ATTRIBUTE_DEALLOC(f, i) _GL_ATTRIBUTE_DEALLOC(f, i)`
- `ATTRIBUTE_DEALLOC_FREE` [include/attribute.h:101]: `#define ATTRIBUTE_DEALLOC_FREE _GL_ATTRIBUTE_DEALLOC_FREE`
- `ATTRIBUTE_SENTINEL` [include/attribute.h:109]: `#define ATTRIBUTE_SENTINEL(pos) _GL_ATTRIBUTE_SENTINEL (pos)`
- `ATTRIBUTE_FORMAT` [include/attribute.h:128]: `#define ATTRIBUTE_FORMAT(spec) _GL_ATTRIBUTE_FORMAT (spec)`
- `ATTRIBUTE_NONNULL` [include/attribute.h:133]: `#define ATTRIBUTE_NONNULL(args) _GL_ATTRIBUTE_NONNULL (args)`
- `ATTRIBUTE_RETURNS_NONNULL` [include/attribute.h:137]: `#define ATTRIBUTE_RETURNS_NONNULL _GL_ATTRIBUTE_RETURNS_NONNULL`
- `NODISCARD` [include/attribute.h:142]: `#define NODISCARD _GL_ATTRIBUTE_NODISCARD`
- `MAYBE_UNUSED` [include/attribute.h:155]: `#define MAYBE_UNUSED _GL_ATTRIBUTE_MAYBE_UNUSED`
- `ATTRIBUTE_NONSTRING` [include/attribute.h:160]: `#define ATTRIBUTE_NONSTRING _GL_ATTRIBUTE_NONSTRING`
- `FALLTHROUGH` [include/attribute.h:165]: `#define FALLTHROUGH _GL_ATTRIBUTE_FALLTHROUGH`
- `ATTRIBUTE_ARTIFICIAL` [include/attribute.h:174]: `#define ATTRIBUTE_ARTIFICIAL _GL_ATTRIBUTE_ARTIFICIAL`
- `ATTRIBUTE_EXTERNALLY_VISIBLE` [include/attribute.h:178]: `#define ATTRIBUTE_EXTERNALLY_VISIBLE _GL_ATTRIBUTE_EXTERNALLY_VISIBLE`
- `ATTRIBUTE_NOTHROW` [include/attribute.h:187]: `#define ATTRIBUTE_NOTHROW _GL_ATTRIBUTE_NOTHROW`
- `ATTRIBUTE_NOINLINE` [include/attribute.h:191]: `#define ATTRIBUTE_NOINLINE _GL_ATTRIBUTE_NOINLINE`

## Global Variables
- No global variable definitions were observed in the current module's `.c` files.

## Known Gaps
- This document is generated from parsed results for functions, structs, macros, and global variables; it does not infer declaration signatures from `.h` files that were not parsed.
- If a function appears in the "Functions" section without an explicit header binding, the later Rust migration should re-check the corresponding source `#include` relationships and build scripts.
- Error codes, configuration items, and input/output protocols are recorded only when explicit symbols appear in the source; missing entries do not mean the semantics do not exist, only that the current fact extraction did not observe them.
