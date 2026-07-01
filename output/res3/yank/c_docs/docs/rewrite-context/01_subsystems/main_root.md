# Module Summary

This document is generated only from module partition results and parsed source facts; it does not turn "insufficient information" into "empty implementation" or "design mistake".

## 1. Module Responsibilities
- Observed focus: organized around source files related to `yank`
- Module category: `main`
- Directory scope: `root`

## 2. Inputs and Outputs
- At this stage, do not speculate about runtime I/O; define interface boundaries by the observed function signatures and source files.
- File input boundary: yank.c
- Function count: 13

## 3. Core Interface List
- `input` [yank.c:69-90]: `static void input(void);`
- `strtopat` [yank.c:96-112]: `static char * strtopat(const char *s);`
- `fcmp` [yank.c:119-128]: `static int fcmp(const struct field *f1, const struct field *f2);`
- `xwrite` [yank.c:130-147]: `static ssize_t xwrite(int fd, const char *s, size_t nmemb);`
- `yank` [yank.c:149-189]: `static void yank(const char *s, size_t nmemb);`
- `twrite` [yank.c:191-196]: `static void twrite(const char *s, size_t nmemb);`
- `tputs` [yank.c:198-205]: `static void tputs(const char *s);`
- `tsetup` [yank.c:207-281]: `static void tsetup(void);`
- `tend` [yank.c:283-294]: `static void tend(void);`
- `tgetc` [yank.c:296-339]: `static int tgetc(void);`
- `tmain` [yank.c:341-408]: `static const struct field * tmain(void);`
- `usage` [yank.c:410-416]: `static void usage(void);`
- `main` [yank.c:418-492]: `int main(int argc, char *argv[]);`

## 4. Dependencies on Other Modules
- Internal call count: 16
- External call count: 0
- Cohesion score: 1.00
- Related headers: not recorded in the current module metadata.

## 5. Key Behaviors That Must Be Preserved
- At minimum, preserve the control flow and return conventions from the source code where these function definitions live; check the implementation body directly rather than inferring from the summary.
- Data structures defined in this module's files: `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`

## 6. Module Partition Signals
- The current module is already a consumable unit after partitioner convergence, with no additional split signals.

## Conclusion
- If function definitions really exist in the source, they should not be described as "empty implementations"; this document is grounded in source locations and declaration excerpts.
- "Module partitioning is unreasonable" should only come from real split signals from the partitioner, not from a summary model drawing conclusions when information is insufficient.
