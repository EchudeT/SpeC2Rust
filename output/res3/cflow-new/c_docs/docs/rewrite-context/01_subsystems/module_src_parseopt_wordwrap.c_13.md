# Module Summary

This document is generated only from module partition results and parsed source facts; it does not turn "insufficient information" into "empty implementation" or "design mistake".

## 1. Module Responsibilities
- Observed focus: organized around source files related to `wordwrap`
- Module category: `module_cluster`
- Directory scope: `src/parseopt`

## 2. Inputs and Outputs
- At this stage, do not speculate about runtime I/O; define interface boundaries by the observed function signatures and source files.
- File input boundary: src/parseopt/wordwrap.c
- Function count: 15

## 3. Core Interface List
- `wordwrap_line_init` [src/parseopt/wordwrap.c:100-109]: `static void wordwrap_line_init (WORDWRAP_FILE wf, int clrws);`
- `detect_right_margin` [src/parseopt/wordwrap.c:115-140]: `static unsigned detect_right_margin (WORDWRAP_FILE wf);`
- `_ww_fd_writer` [src/parseopt/wordwrap.c:142-147]: `static ssize_t _ww_fd_writer (void *data, const char *str, size_t n);`
- `wordwrap_open` [src/parseopt/wordwrap.c:154-179]: `WORDWRAP_FILE wordwrap_open (int fd, ssize_t (*writer) (void *, const char *, size_t), void *data);`
- `wordwrap_fdopen` [src/parseopt/wordwrap.c:181-187]: `WORDWRAP_FILE wordwrap_fdopen (int fd);`
- `wordwrap_close` [src/parseopt/wordwrap.c:192-203]: `int wordwrap_close (WORDWRAP_FILE wf);`
- `full_write` [src/parseopt/wordwrap.c:228-249]: `static ssize_t full_write (WORDWRAP_FILE wf, size_t size);`
- `safe_mbrtowc` [src/parseopt/wordwrap.c:255-268]: `static inline size_t safe_mbrtowc (WORDWRAP_FILE wf, wchar_t *wc, const char *s, mbstate_t *ps);`
- `wsprefix` [src/parseopt/wordwrap.c:273-292]: `static size_t wsprefix (WORDWRAP_FILE wf, char const *str, size_t size);`
- `wordwrap_rescan` [src/parseopt/wordwrap.c:298-324]: `static void wordwrap_rescan (WORDWRAP_FILE wf, size_t size);`
- `wordwrap_flush` [src/parseopt/wordwrap.c:436-442]: `int wordwrap_flush (WORDWRAP_FILE wf);`
- `wordwrap_error` [src/parseopt/wordwrap.c:447-451]: `int wordwrap_error (WORDWRAP_FILE wf);`
- `wordwrap_next_left_margin` [src/parseopt/wordwrap.c:502-515]: `int wordwrap_next_left_margin (WORDWRAP_FILE wf, unsigned left);`
- `wordwrap_write` [src/parseopt/wordwrap.c:586-643]: `int wordwrap_write (WORDWRAP_FILE wf, char const *str, size_t len);`
- `wordwrap_puts` [src/parseopt/wordwrap.c:649-653]: `int wordwrap_puts (WORDWRAP_FILE wf, char const *str);`

## 4. Dependencies on Other Modules
- Internal call count: 7
- External call count: 8
- Cohesion score: 0.47
- Related headers: not recorded in the current module metadata.

## 5. Key Behaviors That Must Be Preserved
- At minimum, preserve the control flow and return conventions from the source code where these function definitions live; check the implementation body directly rather than inferring from the summary.
- Data structures defined in this module's files: `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`

## 6. Module Partition Signals
- This module was split out of parent module `module_src_parseopt`; cluster type: `file_local`.
- Actual reasons the parent module was split: 函数数过多(88)

## Conclusion
- If function definitions really exist in the source, they should not be described as "empty implementations"; this document is grounded in source locations and declaration excerpts.
- "Module partitioning is unreasonable" should only come from real split signals from the partitioner, not from a summary model drawing conclusions when information is insufficient.
