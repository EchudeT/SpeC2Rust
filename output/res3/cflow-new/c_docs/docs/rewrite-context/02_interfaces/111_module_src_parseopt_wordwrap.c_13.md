# Interface Facts: module_src_parseopt_wordwrap.c_13

This document is intended for the later Rust repository-level rewrite and keeps only the interface facts directly observed in the current source-analysis stage.
Header files, macros, error codes, and configuration items that do not appear in the current parse results will not be added or assumed.

## Module Scope
- Module category: `module_cluster`
- Directory: `src/parseopt`
- File list: src/parseopt/wordwrap.c
- Candidate header files: gnu/limits.h, gnu/stdio.h, gnu/stdlib.h, gnu/string.h, gnu/unistd.h, gnu/wchar.h, src/parseopt/parseconf.h, src/parseopt/parseopt.h, src/parseopt/wordwrap.h
- Exported functions observed: 15
- Struct definitions observed: 109
- Type names referenced but not defined locally: 2
- Macros observed in related files: 20
- Global variables observed: 0

## Header Evidence
- `gnu/limits.h` [gnu/limits.h]
- `gnu/stdio.h` [gnu/stdio.h]
- `gnu/stdlib.h` [gnu/stdlib.h]
- `gnu/string.h` [gnu/string.h]
- `gnu/unistd.h` [gnu/unistd.h]
- `gnu/wchar.h` [gnu/wchar.h]
- `src/parseopt/parseconf.h` [src/parseopt/parseconf.h]
- `src/parseopt/parseopt.h` [src/parseopt/parseopt.h]
- `src/parseopt/wordwrap.h` [src/parseopt/wordwrap.h]

## Functions
### `wordwrap_line_init`
- Definition location: [src/parseopt/wordwrap.c:100-109]
- Source file: `src/parseopt/wordwrap.c`
- Observed declaration: `static void wordwrap_line_init (WORDWRAP_FILE wf, int clrws);`
- Approximate function body length: 10 lines
### `detect_right_margin`
- Definition location: [src/parseopt/wordwrap.c:115-140]
- Source file: `src/parseopt/wordwrap.c`
- Observed declaration: `static unsigned detect_right_margin (WORDWRAP_FILE wf);`
- Approximate function body length: 26 lines
### `_ww_fd_writer`
- Definition location: [src/parseopt/wordwrap.c:142-147]
- Source file: `src/parseopt/wordwrap.c`
- Observed declaration: `static ssize_t _ww_fd_writer (void *data, const char *str, size_t n);`
- Approximate function body length: 6 lines
### `wordwrap_open`
- Definition location: [src/parseopt/wordwrap.c:154-179]
- Source file: `src/parseopt/wordwrap.c`
- Observed declaration: `WORDWRAP_FILE wordwrap_open (int fd, ssize_t (*writer) (void *, const char *, size_t), void *data);`
- Approximate function body length: 26 lines
### `wordwrap_fdopen`
- Definition location: [src/parseopt/wordwrap.c:181-187]
- Source file: `src/parseopt/wordwrap.c`
- Observed declaration: `WORDWRAP_FILE wordwrap_fdopen (int fd);`
- Approximate function body length: 7 lines
### `wordwrap_close`
- Definition location: [src/parseopt/wordwrap.c:192-203]
- Source file: `src/parseopt/wordwrap.c`
- Observed declaration: `int wordwrap_close (WORDWRAP_FILE wf);`
- Approximate function body length: 12 lines
### `full_write`
- Definition location: [src/parseopt/wordwrap.c:228-249]
- Source file: `src/parseopt/wordwrap.c`
- Observed declaration: `static ssize_t full_write (WORDWRAP_FILE wf, size_t size);`
- Approximate function body length: 22 lines
### `safe_mbrtowc`
- Definition location: [src/parseopt/wordwrap.c:255-268]
- Source file: `src/parseopt/wordwrap.c`
- Observed declaration: `static inline size_t safe_mbrtowc (WORDWRAP_FILE wf, wchar_t *wc, const char *s, mbstate_t *ps);`
- Approximate function body length: 14 lines
### `wsprefix`
- Definition location: [src/parseopt/wordwrap.c:273-292]
- Source file: `src/parseopt/wordwrap.c`
- Observed declaration: `static size_t wsprefix (WORDWRAP_FILE wf, char const *str, size_t size);`
- Approximate function body length: 20 lines
### `wordwrap_rescan`
- Definition location: [src/parseopt/wordwrap.c:298-324]
- Source file: `src/parseopt/wordwrap.c`
- Observed declaration: `static void wordwrap_rescan (WORDWRAP_FILE wf, size_t size);`
- Approximate function body length: 27 lines
### `wordwrap_flush`
- Definition location: [src/parseopt/wordwrap.c:436-442]
- Source file: `src/parseopt/wordwrap.c`
- Observed declaration: `int wordwrap_flush (WORDWRAP_FILE wf);`
- Approximate function body length: 7 lines
### `wordwrap_error`
- Definition location: [src/parseopt/wordwrap.c:447-451]
- Source file: `src/parseopt/wordwrap.c`
- Observed declaration: `int wordwrap_error (WORDWRAP_FILE wf);`
- Approximate function body length: 5 lines
### `wordwrap_next_left_margin`
- Definition location: [src/parseopt/wordwrap.c:502-515]
- Source file: `src/parseopt/wordwrap.c`
- Observed declaration: `int wordwrap_next_left_margin (WORDWRAP_FILE wf, unsigned left);`
- Approximate function body length: 14 lines
### `wordwrap_write`
- Definition location: [src/parseopt/wordwrap.c:586-643]
- Source file: `src/parseopt/wordwrap.c`
- Observed declaration: `int wordwrap_write (WORDWRAP_FILE wf, char const *str, size_t len);`
- Approximate function body length: 58 lines
### `wordwrap_puts`
- Definition location: [src/parseopt/wordwrap.c:649-653]
- Source file: `src/parseopt/wordwrap.c`
- Observed declaration: `int wordwrap_puts (WORDWRAP_FILE wf, char const *str);`
- Approximate function body length: 5 lines

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
- Definition location: [src/parseopt/wordwrap.c:38-42]
- Source file: `src/parseopt/wordwrap.c`
- Observed declaration prefix: `struct position`
### `anonymous`
- Definition location: [src/parseopt/wordwrap.c:47]
- Source file: `src/parseopt/wordwrap.c`
- Observed declaration prefix: `struct position`
### `anonymous`
- Definition location: [src/parseopt/wordwrap.c:53]
- Source file: `src/parseopt/wordwrap.c`
- Observed declaration prefix: `struct position`
### `anonymous`
- Definition location: [src/parseopt/wordwrap.c:60]
- Source file: `src/parseopt/wordwrap.c`
- Observed declaration prefix: `struct position`
### `anonymous`
- Definition location: [src/parseopt/wordwrap.c:60]
- Source file: `src/parseopt/wordwrap.c`
- Observed declaration prefix: `struct position`
### `anonymous`
- Definition location: [src/parseopt/wordwrap.c:67]
- Source file: `src/parseopt/wordwrap.c`
- Observed declaration prefix: `struct position`
### `anonymous`
- Definition location: [src/parseopt/wordwrap.c:67]
- Source file: `src/parseopt/wordwrap.c`
- Observed declaration prefix: `struct position`
### `anonymous`
- Definition location: [src/parseopt/wordwrap.c:72-95]
- Source file: `src/parseopt/wordwrap.c`
- Observed declaration prefix: `struct wordwrap_file`
### `anonymous`
- Definition location: [src/parseopt/wordwrap.c:81]
- Source file: `src/parseopt/wordwrap.c`
- Observed declaration prefix: `struct position`
### `anonymous`
- Definition location: [src/parseopt/wordwrap.c:82]
- Source file: `src/parseopt/wordwrap.c`
- Observed declaration prefix: `struct position`
### `anonymous`
- Definition location: [src/parseopt/wordwrap.c:84]
- Source file: `src/parseopt/wordwrap.c`
- Observed declaration prefix: `struct position`
### `anonymous`
- Definition location: [src/parseopt/wordwrap.c:118]
- Source file: `src/parseopt/wordwrap.c`
- Observed declaration prefix: `struct winsize`
### `anonymous`
- Definition location: [src/parseopt/wordwrap.c:158]
- Source file: `src/parseopt/wordwrap.c`
- Observed declaration prefix: `struct wordwrap_file`
### `anonymous`
- Definition location: [src/parseopt/wordwrap.c:326]
- Source file: `src/parseopt/wordwrap.c`
- Observed declaration prefix: `struct position`
### `anonymous`
- Definition location: [src/parseopt/wordwrap.c:327]
- Source file: `src/parseopt/wordwrap.c`
- Observed declaration prefix: `struct position`
### `anonymous`
- Definition location: [src/parseopt/wordwrap.c:331]
- Source file: `src/parseopt/wordwrap.c`
- Observed declaration prefix: `struct position`
### `anonymous`
- Definition location: [src/parseopt/wordwrap.c:332]
- Source file: `src/parseopt/wordwrap.c`
- Observed declaration prefix: `struct position`
### `anonymous`
- Definition location: [src/parseopt/wordwrap.c:366]
- Source file: `src/parseopt/wordwrap.c`
- Observed declaration prefix: `struct position`
### `anonymous`
- Definition location: [src/parseopt/wordwrap.h:25]
- Source file: `src/parseopt/wordwrap.h`
- Observed declaration prefix: `struct wordwrap_file`

## Referenced External Types
- `winsize`: this name came from clustering metadata or nearby call analysis, but no local definition was observed in the current module files.
- `wordwrap_file`: this name came from clustering metadata or nearby call analysis, but no local definition was observed in the current module files.

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
