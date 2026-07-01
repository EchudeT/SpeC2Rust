# spec.md

## Title
Functional Specification for `module_doc_d.c_03` Rust Port

## Summary
This module provides directory filtering and recursive directory display behavior for the documentation component represented by `doc/d.c`. Its observed responsibilities are:

- determine whether a filesystem path refers to a directory,
- identify directory entry names that should be ignored during traversal,
- print a directory tree recursively with indentation by depth.

The Rust rewrite must preserve this behavior as a filesystem-walking helper module for the `cflow-new` project branch `003-module_doc_d.c_03-rust-port`.

## Scope
In scope for this module:

- checking whether a named path is a directory,
- recognizing ignorable entry names,
- recursively traversing directories starting from a named path,
- printing visited entries in a depth-indented form.

Out of scope:

- defining new traversal policies beyond those evidenced,
- exposing additional public APIs not required by the observed functions,
- file content processing,
- non-filesystem data sources.

## Feature Specification

### Feature: Directory Classification
The module must support classification of a supplied filesystem path as either a directory or not a directory.

This behavior is evidenced by `isdir` in `doc/d.c`.

Expected behavior:
- Accept a path name.
- Inspect filesystem metadata for that path.
- Return a boolean-like result indicating whether the path is a directory.

### Feature: Ignorable Entry Detection
The module must support recognition of directory entry names that should not be traversed.

This behavior is evidenced by `ignorent` in `doc/d.c`.

Expected behavior:
- Accept a directory entry name.
- Identify whether the entry is considered ignorable.
- Return a boolean-like result used by traversal logic to skip such entries.

The Rust version must preserve the same effective filtering role so recursive traversal avoids entries intentionally excluded by the original module.

### Feature: Recursive Directory Printing
The module must support recursive printing of a directory hierarchy beginning from a supplied path and indentation level.

This behavior is evidenced by `printdir` in `doc/d.c`.

Expected behavior:
- Accept a nesting level and a path name.
- Print the current item in a form that reflects the supplied nesting level.
- If the named path is a directory, inspect its contents.
- Skip ignorable entries during traversal.
- Recurse into eligible child entries using increased nesting depth.
- Continue until reachable non-ignored descendants have been processed.

## User Scenarios & Testing

### Scenario 1: Check Whether a Path Is a Directory
A caller needs to know whether a path should be treated as a directory before traversing it.

Expected result:
- For a path naming an existing directory, the module reports directory status.
- For a path naming a non-directory, the module reports non-directory status.

Suggested tests:
- existing directory path returns true-like result,
- existing regular file path returns false-like result.

### Scenario 2: Skip Special or Ignored Directory Entries
A caller traversing a directory relies on the module to avoid descending into entries that must be ignored.

Expected result:
- Ignorable entry names are identified consistently.
- Non-ignorable names are not filtered out by this check.

Suggested tests:
- known ignorable names are reported as ignorable,
- ordinary file or subdirectory names are reported as non-ignorable.

### Scenario 3: Print a Single Non-Directory Entry
A caller requests printing for a path that is not a directory.

Expected result:
- The module prints the entry once using the provided indentation level.
- No recursive descent occurs.

Suggested tests:
- invoke recursive print on a regular file path and verify one appropriately indented output line.

### Scenario 4: Print a Nested Directory Tree
A caller requests printing from a directory containing nested subdirectories and files.

Expected result:
- The root path is printed.
- Descendants are printed in traversal order determined by directory reading.
- Child items appear at a greater indentation level than their parent.
- Ignorable entries are omitted from output and are not traversed.

Suggested tests:
- create a temporary directory with nested children and verify:
  - all non-ignored descendants appear,
  - ignored entries do not appear,
  - depth indentation increases by one level per recursion step.

### Scenario 5: Start Traversal at a Nonzero Level
A caller integrates this module into a larger formatted output and starts printing at an existing nesting depth.

Expected result:
- Output indentation reflects the supplied starting level.
- Descendant indentation is relative to that starting level.

Suggested tests:
- invoke recursive print with starting levels such as 0 and 2 and verify relative indentation.

## Requirements

### Functional Requirements

#### FR-1: Directory Status Evaluation
The module shall evaluate a provided path name and determine whether it refers to a directory.

Traceability:
- `doc/d.c`
- `isdir`

#### FR-2: Ignorable Name Evaluation
The module shall evaluate a provided directory entry name and determine whether traversal should skip that entry.

Traceability:
- `doc/d.c`
- `ignorent`

#### FR-3: Indented Output of Named Path
The module shall print a representation of the provided path using an indentation level supplied by the caller.

Traceability:
- `doc/d.c`
- `printdir`

#### FR-4: Recursive Descent for Directories
When the provided path is a directory, the module shall inspect contained entries and recursively print eligible descendants at one greater nesting level.

Traceability:
- `doc/d.c`
- `printdir`
- `isdir`

#### FR-5: Ignored Entries Shall Not Be Traversed
During directory traversal, the module shall exclude entries identified as ignorable from recursive processing and printed output.

Traceability:
- `doc/d.c`
- `printdir`
- `ignorent`

#### FR-6: Non-Directory Paths Shall Not Trigger Descent
When the provided path is not a directory, the module shall not attempt recursive descent beneath that path.

Traceability:
- `doc/d.c`
- `printdir`
- `isdir`

### Key Entities

#### Filesystem Path Name
A caller-supplied name identifying the filesystem object to classify or print.

Relationships:
- consumed by directory status evaluation,
- consumed by recursive printing as the traversal root or current node.

Traceability:
- `isdir`
- `printdir`

#### Directory Entry Name
A name read from a directory listing and evaluated for exclusion.

Relationships:
- evaluated by ignorable-name detection,
- used by recursive traversal to decide whether a child should be skipped.

Traceability:
- `ignorent`
- `printdir`
- `dirent`

#### Filesystem Metadata
Directory classification depends on filesystem metadata describing the named object.

Relationships:
- used to determine whether a path is a directory.

Traceability:
- `isdir`
- `stat`

## Success Criteria

### SC-1: Correct Directory Classification
For test inputs containing at least one existing directory and one existing non-directory, the Rust module returns correct directory/non-directory results for all tested paths.

Traceability:
- `isdir`
- `stat`

### SC-2: Correct Ignore Decisions
For a test set containing names intended to be ignored and names intended to be retained, the Rust module produces the same ignore decisions as the original module behavior.

Traceability:
- `ignorent`

### SC-3: Correct Non-Directory Print Behavior
Given a non-directory input path and a specified level, the Rust module emits exactly one entry for that path and performs no descendant output.

Traceability:
- `printdir`
- `isdir`

### SC-4: Correct Recursive Tree Output
Given a directory tree with nested non-ignored descendants, the Rust module outputs the root and all non-ignored reachable descendants with indentation increasing by one level per recursive step.

Traceability:
- `printdir`
- `ignorent`

### SC-5: Ignored Entries Are Excluded
Given a directory tree containing ignorable entries, the Rust module neither prints nor descends into those entries.

Traceability:
- `printdir`
- `ignorent`

### SC-6: Starting Level Is Honored
Given the same input tree and different starting levels, the Rust module preserves the same relative tree structure while shifting indentation according to the supplied initial level.

Traceability:
- `printdir`