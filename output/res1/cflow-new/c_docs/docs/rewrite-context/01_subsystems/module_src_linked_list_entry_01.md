# Module Summary

This document is generated only from module partition results and parsed source facts; it does not turn "insufficient information" into "empty implementation" or "design mistake".

## 1. Module Responsibilities
- Observed focus: At this point, only the file and symbol distribution indicates that this is a local source slice; its responsibilities still need to be confirmed from the source.
- Module category: `module_cluster`
- Directory scope: `src`

## 2. Inputs and Outputs
- At this stage, do not speculate about runtime I/O; define interface boundaries by the observed function signatures and source files.
- File input boundary: src/dot.c, src/linked-list.c, src/main.c, src/output.c
- Function count: 15

## 3. Core Interface List
- `dot_print_symbol` [src/dot.c:38-58]: `static void dot_print_symbol(FILE *fp, int line, struct output_symbol *s);`
- `linked_list_append` [src/linked-list.c:41-56]: `void linked_list_append(struct linked_list **plist, void *data);`
- `linked_list_prepend` [src/linked-list.c:59-74]: `void linked_list_prepend(struct linked_list **plist, void *data);`
- `linked_list_destroy` [src/linked-list.c:77-94]: `void linked_list_destroy(struct linked_list **plist);`
- `linked_list_unlink` [src/linked-list.c:96-113]: `void linked_list_unlink(struct linked_list *list, struct linked_list_entry *ent);`
- `linked_list_iterate` [src/linked-list.c:115-135]: `void linked_list_iterate(struct linked_list **plist, int (*itr) (void *, void *), void *data);`
- `data_in_list` [src/linked-list.c:137-146]: `int data_in_list(void *data, struct linked_list *list);`
- `linked_list_size` [src/linked-list.c:148-158]: `size_t linked_list_size(struct linked_list *list);`
- `optfile_register` [src/main.c:1078-1099]: `static int optfile_register(dev_t dev, ino_t ino);`
- `print_refs` [src/output.c:208-221]: `void print_refs(char *name, struct linked_list *reflist);`
- `is_printable` [src/output.c:282-286]: `static int is_printable(struct linked_list_entry *p);`
- `is_last` [src/output.c:288-295]: `static int is_last(struct linked_list_entry *p);`
- `direct_tree` [src/output.c:299-320]: `static void direct_tree(int lev, int last, Symbol *sym);`
- `inverted_tree` [src/output.c:324-344]: `static void inverted_tree(int lev, int last, Symbol *sym);`
- `tree_output` [src/output.c:346-422]: `static void tree_output();`

## 4. Dependencies on Other Modules
- Internal call count: 10
- External call count: 35
- Cohesion score: 0.22
- Related headers: not recorded in the current module metadata.

## 5. Key Behaviors That Must Be Preserved
- At minimum, preserve the control flow and return conventions from the source code where these function definitions live; check the implementation body directly rather than inferring from the summary.
- Data structures defined in this module's files: `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`

## 6. Module Partition Signals
- This module was split out of parent module `module_src`; cluster type: `struct_based`.
- Actual reasons the parent module was split: 函数数过多(221); 职责不明确且目录范围较大

## Conclusion
- If function definitions really exist in the source, they should not be described as "empty implementations"; this document is grounded in source locations and declaration excerpts.
- "Module partitioning is unreasonable" should only come from real split signals from the partitioner, not from a summary model drawing conclusions when information is insufficient.
