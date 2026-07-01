# spec.md

## Title

Rust Functional Specification for `main_root_file_name_03`

## Metadata

- Project: `pwd`
- Module: `main_root_file_name_03`
- Category: `main_cluster`
- Source files analyzed: `pwd.c`
- Target Rust branch: `003-main_root_file_name_03-rust-port`
- Generation date: `2026-06-07`

## Overview

This module implements the command-line behavior for printing the current working directory, including logic for constructing the directory path by walking parent directories when needed. The Rust rewrite must preserve the observable behavior of the analyzed module: initialize and manage a dynamically built pathname, determine path components by inspecting parent directories, and produce the final current-directory path for program output.

The module includes:
- pathname buffer lifecycle management,
- pathname construction by prepending components,
- directory-entry discovery for the current directory within its parent,
- robust current-working-directory resolution,
- and the main program flow that selects and prints the resulting path.

## Feature Specification

### Summary

The Rust version must implement the module behavior needed to obtain and print the current working directory path. It must support two behavior paths evidenced by the source:
1. using an available working-directory source directly when valid for the requested mode,
2. otherwise computing the path robustly by traversing parent directories and locating the current directory entry name inside each parent.

### Required Functional Behavior

#### 1. Manage an incrementally built pathname

The module must maintain a mutable pathname object that supports:
- initialization to an empty or root-ready state,
- prepending a path segment to the front of the current path,
- and cleanup of any associated resources.

This behavior is evidenced by:
- `file_name_init`
- `file_name_prepend`
- `file_name_free`

#### 2. Discover the current directory name from its parent

The module must be able to determine the directory entry name corresponding to the current directory when examining its parent directory. This behavior must rely on filesystem identity comparison, using directory metadata for the current directory and candidate entries in the parent.

This behavior is evidenced by:
- `find_dir_entry`

The Rust rewrite must preserve the functional effect:
- given metadata for the current directory and access to its parent, identify the matching entry name,
- prepend that name to the pathname under construction,
- support repeated use during upward traversal.

#### 3. Construct the current working directory robustly

The module must support building the absolute current working directory path without depending solely on a direct library call or environment-provided value. When this mode is used, the module must:
- examine the current directory and ancestor directories,
- stop when the filesystem root is reached,
- build the final absolute path from discovered entry names.

This behavior is evidenced by:
- `robust_getcwd`
- `find_dir_entry`
- pathname support via `file_name_*`

#### 4. Execute main program flow for `pwd`

The module must implement the command behavior that:
- accepts command-line arguments relevant to working-directory output,
- determines which directory-resolution mode to use,
- prints the resulting path,
- and exits with success or failure according to outcome.

This behavior is evidenced by:
- `main`

The Rust rewrite must preserve observable command behavior for this module’s responsibilities:
- produce one pathname line on success,
- report failure when the path cannot be determined or output cannot be completed.

## User Scenarios & Testing

### Scenario 1: Print the current working directory successfully

A user runs the program from a normal accessible working directory. The module resolves the current directory path and prints exactly that absolute path followed by a newline.

Supported by:
- `main`
- `robust_getcwd`

Test expectations:
- exit status indicates success,
- stdout contains one absolute pathname line,
- no error output is produced.

### Scenario 2: Build the path by parent traversal

A user runs the program in a context where direct retrieval is not the selected path source, so the module must compute the path by walking upward through parent directories. The module identifies the current directory entry within each parent and prepends components until root is reached.

Supported by:
- `robust_getcwd`
- `find_dir_entry`
- `file_name_prepend`

Test expectations:
- output path is absolute,
- output path names the actual current directory,
- path components appear in correct order from root to leaf.

### Scenario 3: Handle the filesystem root directory

A user runs the program while the current working directory is the filesystem root. The module recognizes the root condition and prints `/`.

Supported by:
- `robust_getcwd`
- metadata comparison in traversal logic

Test expectations:
- stdout is exactly `/` followed by newline,
- no duplicate separators or empty extra components appear,
- exit status indicates success.

### Scenario 4: Use a valid direct working-directory source when allowed

A user runs the program in a mode where a directly available current-directory path is acceptable. The module uses that value instead of reconstructing the path, provided it is valid for the requested behavior.

Supported by:
- `main`

Test expectations:
- output matches the accepted direct path,
- no traversal is required for the successful case,
- exit status indicates success.

### Scenario 5: Fail when directory resolution cannot be completed

A user runs the program in an environment where required directory inspection or output fails. The module must terminate with failure rather than printing an incorrect path.

Supported by:
- `find_dir_entry`
- `robust_getcwd`
- `main`

Test expectations:
- exit status indicates failure,
- stdout does not contain a misleading completed pathname,
- an error is reported.

## Requirements

### Functional Requirements

#### FR-1: Pathname object lifecycle
The Rust module shall provide internal pathname state that can be created before resolution begins, extended as components are discovered, and released after use.

Traceability:
- `pwd.c`: `file_name_init`, `file_name_prepend`, `file_name_free`
- type: `struct file_name`

#### FR-2: Front-prepend pathname construction
The pathname state shall support prepending a directory name segment of known length so that repeated prepend operations produce a correctly ordered absolute path.

Traceability:
- `pwd.c`: `file_name_prepend`
- type: `struct file_name`

#### FR-3: Parent-entry matching by filesystem identity
The module shall be able to inspect a parent directory and find the entry corresponding to the current directory by comparing filesystem identity information from metadata rather than relying only on names already known.

Traceability:
- `pwd.c`: `find_dir_entry`
- types: `struct stat`, `struct dirent`, `dev_ino`

#### FR-4: Root detection during upward traversal
The module shall detect when the current traversal point has reached the filesystem root and stop adding further parent components.

Traceability:
- `pwd.c`: `robust_getcwd`
- types: `struct stat`, `dev_ino`

#### FR-5: Robust absolute path reconstruction
When direct path use is not sufficient for the selected behavior, the module shall reconstruct the current working directory as an absolute path by repeated parent traversal and component discovery.

Traceability:
- `pwd.c`: `robust_getcwd`, `find_dir_entry`, `file_name_prepend`

#### FR-6: Command-line driven mode selection
The main program flow shall process the supported invocation mode(s) for current-directory output and choose between accepted direct-path use and robust reconstruction according to the requested behavior.

Traceability:
- `pwd.c`: `main`
- type: `struct option`

#### FR-7: Standard output of resulting path
On successful resolution, the module shall print the resolved current working directory path to standard output as a single line.

Traceability:
- `pwd.c`: `main`

#### FR-8: Failure signaling on unrecoverable resolution or output error
If the module cannot determine a correct path or cannot complete output, it shall report failure rather than silently succeeding with incorrect data.

Traceability:
- `pwd.c`: `find_dir_entry`, `robust_getcwd`, `main`

### Key Entities

#### `file_name`
A mutable internal pathname accumulator used while determining the final current working directory string. It is initialized before resolution, updated as directory components are discovered, and released after the final output step.

Traceability:
- `pwd.c`: `struct file_name`
- functions: `file_name_init`, `file_name_prepend`, `file_name_free`

#### Filesystem metadata record
Metadata for directories and candidate entries is used to compare identity across traversal steps, determine root conditions, and match the current directory inside a parent directory.

Traceability:
- `pwd.c`: `struct stat`
- functions: `find_dir_entry`, `robust_getcwd`

#### Directory entry record
A parent directory entry candidate examined during lookup of the current directory’s name within that parent.

Traceability:
- `pwd.c`: `struct dirent`
- function: `find_dir_entry`

#### Device/inode identity pair
A filesystem identity abstraction used in the module’s traversal and root/match decisions.

Traceability:
- `pwd.c`: `dev_ino`
- function: `robust_getcwd`

#### Command option descriptor
Represents supported command-line options used by the main program flow to select output behavior.

Traceability:
- `pwd.c`: `struct option`
- function: `main`

## Success Criteria

### Functional Correctness

1. From an accessible non-root working directory, the Rust module prints the correct absolute current working directory path and exits successfully.
   - Traceability: `main`, `robust_getcwd`

2. From the filesystem root directory, the Rust module prints `/` and exits successfully.
   - Traceability: `robust_getcwd`, `main`

3. When operating in the traversal-based path resolution path, the Rust module reconstructs the full path in correct root-to-leaf order.
   - Traceability: `robust_getcwd`, `find_dir_entry`, `file_name_prepend`

4. The Rust module identifies the current directory within its parent using filesystem identity comparison sufficient to select the correct directory entry name.
   - Traceability: `find_dir_entry`, `struct stat`, `struct dirent`, `dev_ino`

5. When a valid direct current-directory source is permitted by the selected mode, the Rust module can use it and still produce the correct single-line output.
   - Traceability: `main`

### Failure Behavior

6. If path resolution cannot be completed, the Rust module exits with failure and does not claim success with an incorrect pathname.
   - Traceability: `find_dir_entry`, `robust_getcwd`, `main`

7. If final path output cannot be written successfully, the Rust module exits with failure.
   - Traceability: `main`

### Behavioral Parity

8. The Rust rewrite preserves the module’s user-visible responsibility as the `pwd` main command path printer, including mode-driven path selection and final output behavior.
   - Traceability: `main`