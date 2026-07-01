# spec.md

## Title
Functional Specification for `module_doc_d.c_03` Rust Port

## Document Control
- Project: `cflow-new`
- Module: `module_doc_d.c_03`
- Category: `module_cluster`
- Source file: `doc/d.c`
- Target branch: `003-module_doc_d.c_03-rust-port`
- Generation date: `2026-06-17`

## Overview
This module provides directory-oriented support for documentation or tree-style output workflows. Its behavior is centered on:

- determining whether a path refers to a directory,
- deciding whether a directory entry name should be ignored,
- printing directory contents recursively with indentation based on nesting level.

The Rust rewrite must preserve this functional scope and observable behavior. The module is file-system facing and operates on path names and directory entries using standard directory metadata and directory enumeration concepts.

## Feature Specification

### Summary
The Rust version must implement the same three functional responsibilities evidenced in the source module:

1. **Directory classification**
   - Accept a path name and determine whether it identifies a directory.

2. **Directory entry filtering**
   - Accept a directory entry name and decide whether it should be ignored during traversal or display.

3. **Recursive directory printing**
   - Accept a nesting level and a path name, enumerate the directory contents under that path, and print them in a directory-tree style that reflects recursion depth.

### In-Scope Behavior
The Rust port must support the following behavior:

- Read file-system metadata for a named path to determine directory status.
- Apply a name-based ignore rule before processing directory entries.
- Traverse directory contents from a supplied starting path.
- Recurse into subdirectories.
- Use the provided recursion level to control output indentation or equivalent visible nesting representation.
- Produce printed output as the module’s observable result.

### Out of Scope
The Rust port must not introduce unsupported capabilities not evidenced by the source analysis, including:

- new public APIs beyond the module’s functional scope,
- configurable filtering rules beyond the existing name-based ignore decision,
- non-printing output formats,
- persistence, serialization, networking, or concurrency features.

## User Scenarios & Testing

### Scenario 1: Check whether a path is a directory
A caller needs to know whether a path should be handled as a directory before recursing into it.

**Expected behavior**
- When given a path that names a directory, the module reports directory status positively.
- When given a path that is not a directory, the module reports directory status negatively.

**Test coverage**
- Existing directory path.
- Existing non-directory path.
- Path that cannot be statted or inspected.

### Scenario 2: Ignore non-content directory entries
A caller or traversal routine encounters directory entry names that should not be processed further.

**Expected behavior**
- The module identifies ignorable names based on entry name alone.
- Ignored names are not printed and are not traversed recursively by the directory-printing behavior.

**Test coverage**
- Entry names that must be ignored.
- Ordinary entry names that must not be ignored.

### Scenario 3: Print a single directory level
A caller requests printing of a directory listing starting from a given directory at a given nesting level.

**Expected behavior**
- The module prints entries belonging to the supplied directory.
- The visual nesting of the output reflects the supplied level.
- Entries rejected by the ignore rule are excluded.

**Test coverage**
- Directory with files only.
- Directory with mixed files and subdirectories.
- Non-zero initial nesting level.

### Scenario 4: Print nested subdirectories recursively
A caller requests a tree-style printout for a directory hierarchy.

**Expected behavior**
- The module descends into child directories.
- Nested entries are printed with deeper indentation or equivalent depth representation than their parents.
- Recursive traversal respects the ignore rule.

**Test coverage**
- Two-level directory tree.
- Multi-level directory tree.
- Tree containing ignored entries at multiple depths.

### Scenario 5: Handle unreadable or non-directory inputs consistently
A caller passes a path that cannot be traversed as a directory.

**Expected behavior**
- The module does not treat an unreadable or non-directory path as a traversable directory.
- Observable behavior remains bounded to classification and printing responsibilities; no additional recovery features are required.

**Test coverage**
- Path to regular file supplied to recursive printing.
- Missing path.
- Directory with access failure during traversal, if reproducible in the target environment.

## Requirements

### Functional Requirements

#### FR-1: Directory status determination
The module shall determine whether a supplied path name refers to a directory.

**Traceability**
- `isdir` in `doc/d.c`

#### FR-2: Name-based ignore decision
The module shall determine whether a supplied directory entry name should be ignored.

**Traceability**
- `ignorent` in `doc/d.c`

#### FR-3: Directory enumeration
The module shall enumerate entries contained in a supplied directory path for printing purposes.

**Traceability**
- `printdir` in `doc/d.c`
- `struct dirent` usage in `doc/d.c`

#### FR-4: Recursive traversal of subdirectories
When an enumerated entry is a directory and is not ignored, the module shall support descending into that subdirectory during directory printing.

**Traceability**
- `printdir` in `doc/d.c`
- `isdir` interaction implied by traversal behavior in `doc/d.c`

#### FR-5: Depth-aware printed output
The module shall print directory content in a form that reflects traversal depth using the supplied level parameter.

**Traceability**
- `printdir` in `doc/d.c`

#### FR-6: Ignore rule enforcement during printing
The module shall exclude ignored entry names from recursive processing and printed output.

**Traceability**
- `ignorent` in `doc/d.c`
- `printdir` in `doc/d.c`

### Key Entities

#### Path name
A path name is the input used for directory classification and directory printing.

**Relationships**
- Consumed by directory status determination.
- Serves as the root location for directory enumeration and recursion.

**Traceability**
- `isdir`
- `printdir`

#### Directory entry name
A directory entry name is the unit evaluated by the ignore rule during traversal.

**Relationships**
- Produced by directory enumeration.
- Passed to ignore filtering before print or recursion decisions.

**Traceability**
- `ignorent`
- `printdir`
- `struct dirent`

#### File-system metadata
File-system metadata is the directory classification information used to distinguish directories from non-directories.

**Relationships**
- Obtained from a path name.
- Drives recursion eligibility.

**Traceability**
- `isdir`
- `struct stat`

#### Traversal level
Traversal level is the nesting depth value supplied to printing behavior.

**Relationships**
- Controls visible nesting in output.
- Increases as recursion descends into subdirectories.

**Traceability**
- `printdir`

## Success Criteria

### SC-1: Correct directory classification
For test inputs containing both directories and non-directories, the Rust port correctly distinguishes directory paths from non-directory paths in all covered cases.

**Traceability**
- `isdir`

### SC-2: Correct ignore decisions
For a test set containing ignorable and non-ignorable entry names, the Rust port returns the same ignore decisions as the source-defined behavior.

**Traceability**
- `ignorent`

### SC-3: Ignored entries are absent from printed traversal
In recursive directory-printing tests, entries classified as ignored do not appear in output and are not descended into.

**Traceability**
- `ignorent`
- `printdir`

### SC-4: Printed nesting reflects recursion depth
In directory-tree tests with at least three levels, the Rust port’s output visibly distinguishes each level according to the supplied and recursively increased level values.

**Traceability**
- `printdir`

### SC-5: Recursive traversal reaches eligible subdirectories
In test directory trees containing nested subdirectories that are not ignored, the Rust port prints content from those nested subdirectories.

**Traceability**
- `printdir`
- `isdir`

### SC-6: Non-directory or inaccessible paths are not traversed as directories
When given inputs that are not directories, or paths that cannot be inspected as directories, the Rust port does not incorrectly recurse into them.

**Traceability**
- `isdir`
- `printdir`