# Module Summary

This document is generated only from module partition results and parsed source facts; it does not turn "insufficient information" into "empty implementation" or "design mistake".

## 1. Module Responsibilities
- Observed focus: organized around source files related to `parser`
- Module category: `module_cluster`
- Directory scope: `src`

## 2. Inputs and Outputs
- At this stage, do not speculate about runtime I/O; define interface boundaries by the observed function signatures and source files.
- File input boundary: src/parser.c
- Function count: 15

## 3. Core Interface List
- `print_token` [src/parser.c:78-115]: `static void print_token(TOKSTK *tokptr);`
- `token_type_str` [src/parser.c:117-168]: `static char * token_type_str(int t);`
- `dbgtok` [src/parser.c:170-179]: `static void dbgtok(TOKSTK *t, int delim);`
- `debugtoken` [src/parser.c:181-203]: `static void debugtoken(TOKSTK *t, char *fmt, ...);`
- `file_error` [src/parser.c:205-214]: `static void file_error(char *msg, TOKSTK *tokptr);`
- `mark` [src/parser.c:216-222]: `void mark(Stackpos pos);`
- `restore` [src/parser.c:224-232]: `void restore(Stackpos pos);`
- `tokdel` [src/parser.c:234-243]: `void tokdel(int beg, int end);`
- `tokins` [src/parser.c:245-259]: `void tokins(int pos, int type, int line, char *token);`
- `tokpush` [src/parser.c:261-272]: `void tokpush(int type, int line, char *token);`
- `cleanup_stack` [src/parser.c:274-285]: `void cleanup_stack();`
- `clearstack` [src/parser.c:287-291]: `void clearstack();`
- `nexttoken` [src/parser.c:293-307]: `int nexttoken();`
- `putback` [src/parser.c:309-323]: `int putback();`
- `init_parse` [src/parser.c:325-331]: `void init_parse();`

## 4. Dependencies on Other Modules
- Internal call count: 9
- External call count: 13
- Cohesion score: 0.41
- Related headers: not recorded in the current module metadata.

## 5. Key Behaviors That Must Be Preserved
- At minimum, preserve the control flow and return conventions from the source code where these function definitions live; check the implementation body directly rather than inferring from the summary.
- Data structures defined in this module's files: `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`

## 6. Module Partition Signals
- This module was split out of parent module `module_src`; cluster type: `file_local`.
- Actual reasons the parent module was split: 函数数过多(221); 职责不明确且目录范围较大

## Conclusion
- If function definitions really exist in the source, they should not be described as "empty implementations"; this document is grounded in source locations and declaration excerpts.
- "Module partitioning is unreasonable" should only come from real split signals from the partitioner, not from a summary model drawing conclusions when information is insufficient.
