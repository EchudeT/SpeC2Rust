# spec.md

## Title
Rust Port Functional Specification: `module_doc_d.c_03`

## Metadata
- Project: `cflow-new`
- Source module: `doc/d.c`
- Module category: `module_cluster`
- Target Rust branch: `003-module_doc_d.c_03-rust-port`
- Generation date: `2026-06-17`

## Overview
This module provides directory-oriented support behavior for documentation-related traversal output. Its evidenced responsibilities are:

- determine whether a given path refers to a directory,
- decide whether a directory entry name should be ignored,
- print directory content recursively with level-aware formatting.

The Rust rewrite must preserve these observable behaviors and module boundaries as represented by the source module.

## Scope
In scope for the Rust version:

- path classification as directory or non-directory,
- filtering of ignorable entry names,
- recursive directory printing driven by a starting level and path name.

Out of scope unless required by direct behavioral equivalence:

- new traversal options,
- configurable filtering rules beyond those evidenced,
- non-printing traversal APIs,
- metadata enrichment beyond what is needed to match current behavior.

## Feature Specification

### Feature 1: Directory Classification
The module must be able to evaluate a provided path name and determine whether it names a directory. This classification is used by the module’s traversal behavior and must reflect filesystem state at the time of the call.

Observed source evidence:
- `isdir` in `doc/d.c`

Required Rust behavior:
- accept a path-like input corresponding to the source `char *name`,
- return a boolean-equivalent result indicating directory versus not-directory,
- treat non-directory paths and paths that cannot be classified as “not a directory” unless the original module’s visible behavior requires otherwise.

### Feature 2: Ignorable Entry Detection
The module must determine whether an entry name should be skipped during traversal or printing. This is a name-based decision applied to directory entries before recursive descent or output.

Observed source evidence:
- `ignorent` in `doc/d.c`

Required Rust behavior:
- accept an entry name,
- return a boolean-equivalent result indicating whether the name is ignorable,
- preserve the original skip set used by the C module.

### Feature 3: Recursive Directory Printing
The module must print directory structure information beginning at a specified path and indentation or depth level. It must inspect directory entries, exclude ignorable names, and recurse into nested directories.

Observed source evidence:
- `printdir` in `doc/d.c`

Required Rust behavior:
- accept a starting level and path name,
- enumerate entries for directory inputs,
- apply ignorable-entry filtering before further processing,
- recurse into subdirectories using incremented or otherwise depth-aware level handling consistent with the source behavior,
- emit output for traversal in a manner preserving the source module’s user-visible behavior.

## User Scenarios & Testing

### Scenario 1: Skip Special Directory Entries
A caller supplies entry names encountered during directory traversal. The module identifies names that should not be processed further, such as traversal-control entries evidenced by the C implementation’s ignore logic.

Acceptance checks:
- known ignorable names are classified as ignorable,
- ordinary non-ignored names are not falsely skipped.

Traceability:
- `ignorent` in `doc/d.c`

### Scenario 2: Distinguish Files from Directories
A caller checks a path before deciding whether recursive processing is appropriate. The module reports whether the path is a directory.

Acceptance checks:
- an existing directory path is reported as a directory,
- a regular file path is reported as not a directory,
- a nonexistent or inaccessible path does not produce a false positive directory result.

Traceability:
- `isdir` in `doc/d.c`
- `struct stat` usage

### Scenario 3: Print a Single-Level Directory
A caller invokes directory printing on a directory containing only non-directory entries. The module prints entries using the supplied initial level and does not recurse further.

Acceptance checks:
- directory contents are enumerated,
- ignorable names are omitted,
- no recursive descent occurs for non-directory entries.

Traceability:
- `printdir` in `doc/d.c`
- `struct dirent` usage

### Scenario 4: Print a Nested Directory Tree
A caller invokes directory printing on a directory containing subdirectories. The module prints the current directory’s entries and descends into nested directories while carrying forward depth information.

Acceptance checks:
- nested directories are traversed recursively,
- recursive descent excludes ignorable entries,
- output reflects level-sensitive traversal rather than flattening all entries into one level.

Traceability:
- `printdir` in `doc/d.c`
- `isdir` in `doc/d.c`
- `ignorent` in `doc/d.c`

## Requirements

### Functional Requirements

#### FR-1: Directory Status Evaluation
The module shall evaluate a provided path name and determine whether it refers to a directory.

Traceability:
- `isdir` in `doc/d.c`

#### FR-2: Non-Positive Classification for Non-Directories
The module shall report a non-directory result for paths that are regular files or otherwise do not satisfy directory classification.

Traceability:
- `isdir` in `doc/d.c`
- `struct stat` usage

#### FR-3: Ignorable Name Classification
The module shall evaluate a directory entry name and determine whether it belongs to the ignore set used by traversal logic.

Traceability:
- `ignorent` in `doc/d.c`

#### FR-4: Filter Before Processing
The module shall exclude ignorable entry names from traversal-related processing and output.

Traceability:
- `ignorent` in `doc/d.c`
- `printdir` in `doc/d.c`

#### FR-5: Directory Enumeration
The module shall inspect the contents of a directory identified by the supplied path when performing print traversal.

Traceability:
- `printdir` in `doc/d.c`
- `struct dirent` usage

#### FR-6: Recursive Descent
The module shall recurse into entries that are directories and are not ignored.

Traceability:
- `printdir` in `doc/d.c`
- `isdir` in `doc/d.c`
- `ignorent` in `doc/d.c`

#### FR-7: Level-Aware Output
The module shall use the supplied `level` argument to control depth-aware printing behavior during traversal.

Traceability:
- `printdir` in `doc/d.c`

#### FR-8: Start-Path Driven Traversal
The module shall perform printing relative to the supplied starting path name rather than an implicit working set.

Traceability:
- `printdir` in `doc/d.c`

### Key Entities

#### Path Name
A path name identifies a filesystem object to classify or traverse.

Relationships:
- consumed by directory classification,
- consumed by recursive directory printing.

Traceability:
- `isdir(char *name)`
- `printdir(int level, char *name)`

#### Entry Name
An entry name is the per-directory item name evaluated by ignore logic.

Relationships:
- produced during directory enumeration,
- checked by ignorable-entry detection before printing or recursion.

Traceability:
- `ignorent(char *name)`
- `struct dirent` usage in `printdir`

#### Directory Entry
A directory entry represents one enumerated item in a directory traversal step.

Relationships:
- supplies the entry name used by ignore filtering,
- may correspond to a path that is checked for directory status.

Traceability:
- `struct dirent` usage in `printdir`

#### File Status
File status data supports determination of whether a path refers to a directory.

Relationships:
- used by path classification,
- indirectly controls recursive descent decisions.

Traceability:
- `struct stat` usage in `isdir`

#### Traversal Level
Traversal level is the depth or formatting context passed into recursive printing.

Relationships:
- provided by the caller for the initial invocation,
- propagated through recursive traversal to preserve structure-aware output.

Traceability:
- `printdir(int level, char *name)`

## Success Criteria

### SC-1: Correct Ignore Decisions
Given the entry names ignored by the source module, the Rust port returns an ignorable result for those names and a non-ignorable result for ordinary names.

Traceability:
- `ignorent` in `doc/d.c`

### SC-2: Correct Directory Detection
For test fixtures containing at least one directory, one regular file, and one nonexistent path, the Rust port classifies only the actual directory as a directory.

Traceability:
- `isdir` in `doc/d.c`

### SC-3: Ignored Entries Do Not Appear in Traversal Output
When traversing a directory containing ignorable and non-ignorable names, output excludes the ignored names.

Traceability:
- `ignorent` in `doc/d.c`
- `printdir` in `doc/d.c`

### SC-4: Recursive Traversal Reaches Nested Directories
When given a directory tree with nested subdirectories, the Rust port emits output showing that nested directories were traversed rather than stopping at the first level.

Traceability:
- `printdir` in `doc/d.c`
- `isdir` in `doc/d.c`

### SC-5: Depth-Aware Behavior Is Preserved
When the same directory tree is printed with different starting `level` values, the output changes consistently with level-sensitive formatting or placement behavior expected from the source module.

Traceability:
- `printdir` in `doc/d.c`

### SC-6: Starting Path Controls Scope
When invoked on two different root paths, the Rust port limits traversal output to content reachable from the provided starting path for each invocation.

Traceability:
- `printdir` in `doc/d.c`

## Notes for Port Validation
Behavioral validation should compare the Rust port against the C module at the observable level of:

- directory/non-directory classification,
- ignored-name decisions,
- recursive traversal and printed structure.

No additional capabilities are required beyond these evidenced behaviors.