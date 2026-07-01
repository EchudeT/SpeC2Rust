# Module Summary

This document is generated only from module partition results and parsed source facts; it does not turn "insufficient information" into "empty implementation" or "design mistake".

## 1. Module Responsibilities
- Observed focus: organized around source files related to `pwd`
- Module category: `main_cluster`
- Directory scope: `root`

## 2. Inputs and Outputs
- At this stage, do not speculate about runtime I/O; define interface boundaries by the observed function signatures and source files.
- File input boundary: pwd.c
- Function count: 6

## 3. Core Interface List
- `file_name_free` [pwd.c:77-82]: `static void file_name_free (struct file_name *p);`
- `file_name_init` [pwd.c:84-97]: `static struct file_name * file_name_init (void);`
- `file_name_prepend` [pwd.c:100-123]: `static void file_name_prepend (struct file_name *p, char const *s, size_t s_len);`
- `find_dir_entry` [pwd.c:152-242]: `static void find_dir_entry (struct stat *dot_sb, struct file_name *file_name, size_t parent_height);`
- `robust_getcwd` [pwd.c:267-294]: `static void robust_getcwd (struct file_name *file_name);`
- `main` [pwd.c:326-394]: `int main (int argc, char **argv);`

## 4. Dependencies on Other Modules
- Internal call count: 6
- External call count: 10
- Cohesion score: 0.38
- Related headers: not recorded in the current module metadata.

## 5. Key Behaviors That Must Be Preserved
- At minimum, preserve the control flow and return conventions from the source code where these function definitions live; check the implementation body directly rather than inferring from the summary.
- Data structures defined in this module's files: `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`, `anonymous`

## 6. Module Partition Signals
- This module was split out of parent module `main_root`; cluster type: `struct_based`.
- Actual reasons the parent module was split: 文件数过多(28); 函数数过多(99)

## Conclusion
- If function definitions really exist in the source, they should not be described as "empty implementations"; this document is grounded in source locations and declaration excerpts.
- "Module partitioning is unreasonable" should only come from real split signals from the partitioner, not from a summary model drawing conclusions when information is insufficient.
