# Interface Facts: main_root

This document is intended for the later Rust repository-level rewrite and keeps only the interface facts directly observed in the current source-analysis stage.
Header files, macros, error codes, and configuration items that do not appear in the current parse results will not be added or assumed.

## Module Scope
- Module category: `main`
- Directory: `root`
- File list: sds.c
- Candidate header files: sds.h, sdsalloc.h, testhelp.h
- Exported functions observed: 45
- Struct definitions observed: 10
- Type names referenced but not defined locally: 0
- Macros observed in related files: 20
- Global variables observed: 0

## Header Evidence
- `sds.h` [sds.h]
- `sdsalloc.h` [sdsalloc.h]
- `testhelp.h` [testhelp.h]

## Functions
### `sdsHdrSize`
- Definition location: [sds.c:44-58]
- Source file: `sds.c`
- Observed declaration: `static inline int sdsHdrSize(char type);`
- Approximate function body length: 15 lines
### `sdsReqType`
- Definition location: [sds.c:60-74]
- Source file: `sds.c`
- Observed declaration: `static inline char sdsReqType(size_t string_size);`
- Approximate function body length: 15 lines
### `sdsnewlen`
- Definition location: [sds.c:89-145]
- Source file: `sds.c`
- Observed declaration: `sds sdsnewlen(const void *init, size_t initlen);`
- Approximate function body length: 57 lines
### `sdsempty`
- Definition location: [sds.c:149-151]
- Source file: `sds.c`
- Observed declaration: `sds sdsempty(void);`
- Approximate function body length: 3 lines
### `sdsnew`
- Definition location: [sds.c:154-157]
- Source file: `sds.c`
- Observed declaration: `sds sdsnew(const char *init);`
- Approximate function body length: 4 lines
### `sdsdup`
- Definition location: [sds.c:160-162]
- Source file: `sds.c`
- Observed declaration: `sds sdsdup(const sds s);`
- Approximate function body length: 3 lines
### `sdsfree`
- Definition location: [sds.c:165-168]
- Source file: `sds.c`
- Observed declaration: `void sdsfree(sds s);`
- Approximate function body length: 4 lines
### `sdsupdatelen`
- Definition location: [sds.c:184-187]
- Source file: `sds.c`
- Observed declaration: `void sdsupdatelen(sds s);`
- Approximate function body length: 4 lines
### `sdsclear`
- Definition location: [sds.c:193-196]
- Source file: `sds.c`
- Observed declaration: `void sdsclear(sds s);`
- Approximate function body length: 4 lines
### `sdsMakeRoomFor`
- Definition location: [sds.c:204-248]
- Source file: `sds.c`
- Observed declaration: `sds sdsMakeRoomFor(sds s, size_t addlen);`
- Approximate function body length: 45 lines
### `sdsRemoveFreeSpace`
- Definition location: [sds.c:256-291]
- Source file: `sds.c`
- Observed declaration: `sds sdsRemoveFreeSpace(sds s);`
- Approximate function body length: 36 lines
### `sdsAllocSize`
- Definition location: [sds.c:300-303]
- Source file: `sds.c`
- Observed declaration: `size_t sdsAllocSize(sds s);`
- Approximate function body length: 4 lines
### `sdsAllocPtr`
- Definition location: [sds.c:307-309]
- Source file: `sds.c`
- Observed declaration: `void *sdsAllocPtr(sds s);`
- Approximate function body length: 3 lines
### `sdsIncrLen`
- Definition location: [sds.c:334-373]
- Source file: `sds.c`
- Observed declaration: `void sdsIncrLen(sds s, ssize_t incr);`
- Approximate function body length: 40 lines
### `sdsgrowzero`
- Definition location: [sds.c:380-391]
- Source file: `sds.c`
- Observed declaration: `sds sdsgrowzero(sds s, size_t len);`
- Approximate function body length: 12 lines
### `sdscatlen`
- Definition location: [sds.c:398-407]
- Source file: `sds.c`
- Observed declaration: `sds sdscatlen(sds s, const void *t, size_t len);`
- Approximate function body length: 10 lines
### `sdscat`
- Definition location: [sds.c:413-415]
- Source file: `sds.c`
- Observed declaration: `sds sdscat(sds s, const char *t);`
- Approximate function body length: 3 lines
### `sdscatsds`
- Definition location: [sds.c:421-423]
- Source file: `sds.c`
- Observed declaration: `sds sdscatsds(sds s, const sds t);`
- Approximate function body length: 3 lines
### `sdscpylen`
- Definition location: [sds.c:427-436]
- Source file: `sds.c`
- Observed declaration: `sds sdscpylen(sds s, const char *t, size_t len);`
- Approximate function body length: 10 lines
### `sdscpy`
- Definition location: [sds.c:440-442]
- Source file: `sds.c`
- Observed declaration: `sds sdscpy(sds s, const char *t);`
- Approximate function body length: 3 lines
### `sdsll2str`
- Definition location: [sds.c:451-491]
- Source file: `sds.c`
- Observed declaration: `int sdsll2str(char *s, long long value);`
- Approximate function body length: 41 lines
### `sdsull2str`
- Definition location: [sds.c:494-520]
- Source file: `sds.c`
- Observed declaration: `int sdsull2str(char *s, unsigned long long v);`
- Approximate function body length: 27 lines
### `sdsfromlonglong`
- Definition location: [sds.c:526-531]
- Source file: `sds.c`
- Observed declaration: `sds sdsfromlonglong(long long value);`
- Approximate function body length: 6 lines
### `sdscatvprintf`
- Definition location: [sds.c:534-573]
- Source file: `sds.c`
- Observed declaration: `sds sdscatvprintf(sds s, const char *fmt, va_list ap);`
- Approximate function body length: 40 lines
### `sdscatprintf`
- Definition location: [sds.c:591-598]
- Source file: `sds.c`
- Observed declaration: `sds sdscatprintf(sds s, const char *fmt, ...);`
- Approximate function body length: 8 lines
### `sdscatfmt`
- Definition location: [sds.c:616-709]
- Source file: `sds.c`
- Observed declaration: `sds sdscatfmt(sds s, char const *fmt, ...);`
- Approximate function body length: 94 lines
### `sdstrim`
- Definition location: [sds.c:725-738]
- Source file: `sds.c`
- Observed declaration: `sds sdstrim(sds s, const char *cset);`
- Approximate function body length: 14 lines
### `sdsrange`
- Definition location: [sds.c:756-780]
- Source file: `sds.c`
- Observed declaration: `void sdsrange(sds s, ssize_t start, ssize_t end);`
- Approximate function body length: 25 lines
### `sdstolower`
- Definition location: [sds.c:783-787]
- Source file: `sds.c`
- Observed declaration: `void sdstolower(sds s);`
- Approximate function body length: 5 lines
### `sdstoupper`
- Definition location: [sds.c:790-794]
- Source file: `sds.c`
- Observed declaration: `void sdstoupper(sds s);`
- Approximate function body length: 5 lines
### `sdscmp`
- Definition location: [sds.c:807-817]
- Source file: `sds.c`
- Observed declaration: `int sdscmp(const sds s1, const sds s2);`
- Approximate function body length: 11 lines
### `sdssplitlen`
- Definition location: [sds.c:835-882]
- Source file: `sds.c`
- Observed declaration: `sds *sdssplitlen(const char *s, ssize_t len, const char *sep, int seplen, int *count);`
- Approximate function body length: 48 lines
### `sdsfreesplitres`
- Definition location: [sds.c:885-890]
- Source file: `sds.c`
- Observed declaration: `void sdsfreesplitres(sds *tokens, int count);`
- Approximate function body length: 6 lines
### `sdscatrepr`
- Definition location: [sds.c:898-921]
- Source file: `sds.c`
- Observed declaration: `sds sdscatrepr(sds s, const char *p, size_t len);`
- Approximate function body length: 24 lines
### `is_hex_digit`
- Definition location: [sds.c:925-928]
- Source file: `sds.c`
- Observed declaration: `int is_hex_digit(char c);`
- Approximate function body length: 4 lines
### `hex_digit_to_int`
- Definition location: [sds.c:932-952]
- Source file: `sds.c`
- Observed declaration: `int hex_digit_to_int(char c);`
- Approximate function body length: 21 lines
### `sdssplitargs`
- Definition location: [sds.c:973-1081]
- Source file: `sds.c`
- Observed declaration: `sds *sdssplitargs(const char *line, int *argc);`
- Approximate function body length: 109 lines
### `sdsmapchars`
- Definition location: [sds.c:1092-1104]
- Source file: `sds.c`
- Observed declaration: `sds sdsmapchars(sds s, const char *from, const char *to, size_t setlen);`
- Approximate function body length: 13 lines
### `sdsjoin`
- Definition location: [sds.c:1108-1117]
- Source file: `sds.c`
- Observed declaration: `sds sdsjoin(char **argv, int argc, char *sep);`
- Approximate function body length: 10 lines
### `sdsjoinsds`
- Definition location: [sds.c:1120-1129]
- Source file: `sds.c`
- Observed declaration: `sds sdsjoinsds(sds *argv, int argc, const char *sep, size_t seplen);`
- Approximate function body length: 10 lines
### `sds_malloc`
- Definition location: [sds.c:1136]
- Source file: `sds.c`
- Observed declaration: `void *sds_malloc(size_t size);`
- Approximate function body length: 1 lines
### `sds_realloc`
- Definition location: [sds.c:1137]
- Source file: `sds.c`
- Observed declaration: `void *sds_realloc(void *ptr, size_t size);`
- Approximate function body length: 1 lines
### `sds_free`
- Definition location: [sds.c:1138]
- Source file: `sds.c`
- Observed declaration: `void sds_free(void *ptr);`
- Approximate function body length: 1 lines
### `sdsTest`
- Definition location: [sds.c:1146-1321]
- Source file: `sds.c`
- Observed declaration: `int sdsTest(void);`
- Approximate function body length: 176 lines
### `main`
- Definition location: [sds.c:1325-1327]
- Source file: `sds.c`
- Observed declaration: `int main(void);`
- Approximate function body length: 3 lines

## Structs and Types
### `anonymous`
- Definition location: [sds.c:47]
- Source file: `sds.c`
- Observed declaration prefix: `struct sdshdr5`
### `anonymous`
- Definition location: [sds.c:49]
- Source file: `sds.c`
- Observed declaration prefix: `struct sdshdr8`
### `anonymous`
- Definition location: [sds.c:51]
- Source file: `sds.c`
- Observed declaration prefix: `struct sdshdr16`
### `anonymous`
- Definition location: [sds.c:53]
- Source file: `sds.c`
- Observed declaration prefix: `struct sdshdr32`
### `anonymous`
- Definition location: [sds.c:55]
- Source file: `sds.c`
- Observed declaration prefix: `struct sdshdr64`
### `anonymous`
- Definition location: [sds.h:47-50]
- Source file: `sds.h`
- Observed declaration prefix: `struct __attribute__ ((__packed__)) sdshdr5`
### `anonymous`
- Definition location: [sds.h:51-56]
- Source file: `sds.h`
- Observed declaration prefix: `struct __attribute__ ((__packed__)) sdshdr8`
### `anonymous`
- Definition location: [sds.h:57-62]
- Source file: `sds.h`
- Observed declaration prefix: `struct __attribute__ ((__packed__)) sdshdr16`
### `anonymous`
- Definition location: [sds.h:63-68]
- Source file: `sds.h`
- Observed declaration prefix: `struct __attribute__ ((__packed__)) sdshdr32`
### `anonymous`
- Definition location: [sds.h:69-74]
- Source file: `sds.h`
- Observed declaration prefix: `struct __attribute__ ((__packed__)) sdshdr64`

## Referenced External Types
- No external struct or type references beyond local definitions were recorded.

## Macros and Constants
- `SDS_LLSTR_SIZE` [sds.c:450]: `#define SDS_LLSTR_SIZE 21`
- `UNUSED` [sds.c:1145]: `#define UNUSED(x) (void)(x)`
- `__SDS_H` [sds.h:34]: `#define __SDS_H`
- `SDS_MAX_PREALLOC` [sds.h:36]: `#define SDS_MAX_PREALLOC (1024*1024)`
- `SDS_TYPE_5` [sds.h:76]: `#define SDS_TYPE_5 0`
- `SDS_TYPE_8` [sds.h:77]: `#define SDS_TYPE_8 1`
- `SDS_TYPE_16` [sds.h:78]: `#define SDS_TYPE_16 2`
- `SDS_TYPE_32` [sds.h:79]: `#define SDS_TYPE_32 3`
- `SDS_TYPE_64` [sds.h:80]: `#define SDS_TYPE_64 4`
- `SDS_TYPE_MASK` [sds.h:81]: `#define SDS_TYPE_MASK 7`
- `SDS_TYPE_BITS` [sds.h:82]: `#define SDS_TYPE_BITS 3`
- `SDS_HDR_VAR` [sds.h:83]: `#define SDS_HDR_VAR(T,s) struct sdshdr##T *sh = (void*)((s)-(sizeof(struct sdshdr##T)));`
- `SDS_HDR` [sds.h:84]: `#define SDS_HDR(T,s) ((struct sdshdr##T *)((s)-(sizeof(struct sdshdr##T))))`
- `SDS_TYPE_5_LEN` [sds.h:85]: `#define SDS_TYPE_5_LEN(f) ((f)>>SDS_TYPE_BITS)`
- `s_malloc` [sdsalloc.h:40]: `#define s_malloc malloc`
- `s_realloc` [sdsalloc.h:41]: `#define s_realloc realloc`
- `s_free` [sdsalloc.h:42]: `#define s_free free`
- `__TESTHELP_H` [testhelp.h:40]: `#define __TESTHELP_H`
- `test_cond` [testhelp.h:44-47]: `#define test_cond(descr,_c) do { \ __test_num++; printf("%d - %s: ", __test_num, descr); \ if(_c) printf("PASSED\n"); else {printf("FAILED\n"); __failed_tests++;} \ } while(0);`
- `test_report` [testhelp.h:48-55]: `#define test_report() do { \ printf("%d tests, %d passed, %d failed\n", __test_num, \ __test_num-__failed_tests, __failed_tests); \ if (__failed_tests) { \ printf("=== WARNING =...`

## Global Variables
- No global variable definitions were observed in the current module's `.c` files.

## Known Gaps
- This document is generated from parsed results for functions, structs, macros, and global variables; it does not infer declaration signatures from `.h` files that were not parsed.
- If a function appears in the "Functions" section without an explicit header binding, the later Rust migration should re-check the corresponding source `#include` relationships and build scripts.
- Error codes, configuration items, and input/output protocols are recorded only when explicit symbols appear in the source; missing entries do not mean the semantics do not exist, only that the current fact extraction did not observe them.
