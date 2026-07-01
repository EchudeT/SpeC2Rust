# spec.md

## Overview

This module is the main execution cluster for `pwd`. It is responsible for producing the current working directory path for command-line use.

Traceable sources:
- `pwd.c`
- `main`
- `robust_getcwd`
- `find_dir_entry`
- `file_name_init`
- `file_name_prepend`
- `file_name_free`

The Rust rewrite must preserve the observed behavior of this module:
- initialize path-building state,
- determine the current directory path,
- reconstruct a path by walking parent directories when needed,
- manage command-line execution flow for the `pwd` program,
- emit the resulting path in the form expected by the original module.

## Feature Specification

### Feature: current working directory output

The module shall implement the main `pwd` program behavior of determining and printing the current working directory.

This includes:
- starting from process state at invocation time,
- selecting the module’s directory-resolution behavior through the existing main-program flow,
- producing a single path result for standard output when successful,
- terminating through the same success/failure style expected of a command-line utility.

Traceable sources:
- `main`
- `robust_getcwd`

### Feature: resilient directory path resolution

The module shall support robust construction of the current working directory path even when direct acquisition is not sufficient for the module’s needs.

Observed behavior indicates path construction based on directory identity and parent traversal. The Rust version must therefore support:
- obtaining metadata for the current directory,
- determining whether traversal has reached the filesystem root,
- walking upward through parent directories,
- identifying the entry name in a parent that corresponds to the child directory,
- prepending each discovered path component until the full absolute path is assembled.

Traceable sources:
- `robust_getcwd`
- `find_dir_entry`
- `stat`
- `dirent`
- `dev_ino`

### Feature: dynamic path assembly

The module shall maintain an internal path buffer used to assemble the final pathname.

The Rust version must preserve the functional behavior of this internal assembly facility:
- create an initially valid empty path state,
- prepend path components as parent traversal discovers them,
- release internal resources when processing is complete.

Traceable sources:
- `file_name_init`
- `file_name_prepend`
- `file_name_free`
- `file_name`

## User Scenarios & Testing

### Scenario 1: print the current directory from a normal working directory

A user runs `pwd` from a directory that can be resolved normally.

Expected behavior:
- the program exits successfully,
- exactly one directory path is emitted,
- the emitted path corresponds to the process current working directory.

Traceable sources:
- `main`
- `robust_getcwd`

Suggested test:
- change into a known directory,
- run the Rust binary,
- verify output matches the expected absolute current directory path.

### Scenario 2: resolve the path by parent traversal logic

A user runs `pwd` in a context where the module must reconstruct the path by examining parent directories and matching directory entries.

Expected behavior:
- the program successfully identifies each child name from its parent,
- path components are prepended in correct order,
- the final emitted path is the absolute path to the starting directory.

Traceable sources:
- `find_dir_entry`
- `robust_getcwd`
- `file_name_prepend`

Suggested test:
- exercise execution in nested directories,
- validate that the output is the full absolute path from root to the leaf directory,
- confirm that component order is correct.

### Scenario 3: execution at filesystem root

A user runs `pwd` while the process current directory is the filesystem root.

Expected behavior:
- traversal recognizes the root condition,
- no extra path components are invented,
- output is the root path.

Traceable sources:
- `robust_getcwd`
- `find_dir_entry`
- `stat`
- `dev_ino`

Suggested test:
- run the Rust binary with current directory set to `/`,
- verify output is `/`.

### Scenario 4: failure during directory inspection

A user runs `pwd` when required filesystem inspection or path construction cannot be completed.

Expected behavior:
- the program does not emit a successful pathname as if resolution had succeeded,
- the program terminates with failure behavior appropriate to the original command-line flow.

Traceable sources:
- `main`
- `robust_getcwd`
- `find_dir_entry`

Suggested test:
- induce a filesystem access failure in a controlled environment,
- verify non-success termination and absence of misleading successful output.

## Requirements

### Functional Requirements

#### FR-1: Main program flow
The module shall provide the main execution flow for the `pwd` program, including command-line entry, path determination, output of the resolved directory path on success, and process termination status consistent with the original module behavior.

Traceable sources:
- `main`

#### FR-2: Internal path-state initialization
The module shall create an internal pathname assembly state before path resolution begins.

Traceable sources:
- `file_name_init`
- `file_name`

#### FR-3: Prepend-based path construction
The module shall support building the final pathname by prepending newly discovered path segments to the existing internal path state.

Traceable sources:
- `file_name_prepend`
- `file_name`

#### FR-4: Internal path-state cleanup
The module shall release internal pathname assembly resources after use.

Traceable sources:
- `file_name_free`
- `file_name`

#### FR-5: Robust current-directory resolution
The module shall resolve the current working directory using the module’s robust resolution path, not solely by assuming a direct string is always sufficient.

Traceable sources:
- `robust_getcwd`

#### FR-6: Root detection during traversal
The module shall detect when traversal has reached the filesystem root by comparing directory identity information for the current directory and its parent-equivalent traversal state.

Traceable sources:
- `robust_getcwd`
- `stat`
- `dev_ino`

#### FR-7: Parent-directory child-name discovery
When reconstructing a path from parent traversal, the module shall locate the directory entry in the parent that corresponds to the current child directory and use that entry name as the next path component.

Traceable sources:
- `find_dir_entry`
- `dirent`
- `stat`

#### FR-8: Absolute path assembly
The module shall assemble the resolved current directory as an absolute path in root-to-leaf order.

Traceable sources:
- `find_dir_entry`
- `robust_getcwd`
- `file_name_prepend`

### Key Entities

#### `file_name`
Internal mutable path-assembly state used to accumulate the resulting directory path. Its lifecycle is:
- created before resolution,
- updated as path components are discovered,
- consumed for final output,
- released after use.

Traceable sources:
- `file_name_init`
- `file_name_prepend`
- `file_name_free`

#### `stat`
Filesystem identity and metadata used to compare directories during traversal and to determine root/parent relationships relevant to path reconstruction.

Traceable sources:
- `find_dir_entry`
- `robust_getcwd`

#### `dirent`
Directory entry information used when scanning a parent directory to find the name corresponding to the current child directory.

Traceable sources:
- `find_dir_entry`

#### `dev_ino`
Directory identity abstraction referenced by the module for comparing filesystem objects while determining traversal termination and matching directory relationships.

Traceable sources:
- `robust_getcwd`

Relationships:
- `robust_getcwd` uses `stat` and `dev_ino` to manage traversal and stopping conditions.
- `find_dir_entry` uses `dirent` and `stat` to map a child directory identity to its name within the parent.
- `file_name` receives discovered names from traversal and stores the progressively assembled absolute path.

## Success Criteria

1. When run from a known non-root directory, the Rust module prints the correct absolute current working directory path and exits successfully.
   - Traceable sources: `main`, `robust_getcwd`

2. When run from the filesystem root, the Rust module prints `/` and exits successfully.
   - Traceable sources: `robust_getcwd`, `stat`, `dev_ino`

3. In nested-directory cases requiring parent-based reconstruction, the Rust module produces the correct absolute path with components in the correct order from root to leaf.
   - Traceable sources: `find_dir_entry`, `file_name_prepend`, `robust_getcwd`

4. The Rust module identifies the correct child directory entry within a parent directory based on filesystem identity rather than assuming the name in advance.
   - Traceable sources: `find_dir_entry`, `dirent`, `stat`

5. The Rust module creates and cleans up its internal path-assembly state without leaking functional state across execution.
   - Traceable sources: `file_name_init`, `file_name_free`, `file_name_prepend`

6. If required directory inspection or reconstruction fails, the Rust module does not report a false successful pathname and instead follows failure termination behavior consistent with the original module flow.
   - Traceable sources: `main`, `find_dir_entry`, `robust_getcwd`