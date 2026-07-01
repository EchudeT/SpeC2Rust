# Functional Specification: main_root_file_name_03

- **Project**: `pwd`
- **Module**: `main_root_file_name_03`
- **Category**: `main_cluster`
- **Source basis**: `pwd.c`
- **Rust target branch**: `003-main_root_file_name_03-rust-port`
- **Generation date**: 2026-06-07

## 1. Overview

This module implements the main executable behavior for reporting the current working directory path. It supports building the output path either from an environment-provided logical path or by reconstructing the path from filesystem traversal when needed.

The Rust rewrite must preserve the observable behavior evidenced by this module:

- parse command-line mode selection relevant to logical versus physical directory reporting,
- validate whether an environment-provided working directory can be reused,
- otherwise compute the current directory path robustly,
- print the resulting path,
- terminate with success or failure according to the outcome.

This specification is limited to functionality evidenced by the analyzed module functions and data structures.

## 2. Feature Specification

### 2.1 Command-line driven current-directory reporting

The module provides a command-line entry point that determines how the current working directory should be reported. The rewrite must preserve mode selection behavior represented by the `main` function.

Supported functional modes evidenced by the module:

- a mode that prefers or accepts a logical path source,
- a mode that requires physical filesystem-derived resolution,
- default execution that prints one current directory path line.

The Rust version must implement equivalent top-level behavior for selecting between these modes and producing exactly one pathname result on success.

### 2.2 Logical path acceptance when valid

The module may use an existing working-directory string instead of reconstructing the path, but only when it passes validation against filesystem state. This is evidenced by `main` using `stat` comparisons involving the current directory and a candidate path.

The Rust version must preserve this functional boundary:

- if a candidate logical working-directory path is present and matches the current directory identity as validated by filesystem metadata, it may be printed directly;
- if validation fails, the module must not trust that candidate and must fall back to robust reconstruction.

### 2.3 Robust reconstruction of the current path

When a reusable logical path is unavailable or invalid, the module constructs the absolute pathname of the current directory. This behavior is evidenced by `robust_getcwd` and `find_dir_entry`.

The Rust version must implement equivalent behavior that:

- determines whether the current directory is the filesystem root,
- if not at root, repeatedly identifies the directory entry name of the current directory within its parent,
- prepends each discovered component to a growing path,
- stops once root is reached,
- returns a path string beginning with `/`.

### 2.4 Root and parent traversal correctness

The reconstruction logic depends on distinguishing the current directory from its parent and finding the matching directory entry in the parent. This is evidenced by:

- comparison of current and parent directory identity using device/inode-style identity,
- scanning parent directory entries,
- matching the current directory entry by metadata.

The Rust version must preserve these behavioral guarantees:

- root detection must be based on filesystem identity comparison, not string heuristics;
- path components must correspond to actual parent directory entries that identify the traversed directory;
- the final path must represent the current directory reached by traversal.

### 2.5 Dynamic path assembly

The module uses an internal pathname accumulator that supports initialization, prefix insertion, and cleanup. This is evidenced by `file_name_init`, `file_name_prepend`, and `file_name_free`.

The Rust rewrite must preserve the functional role of this entity:

- start with an empty or initial path buffer state,
- support adding a discovered path component at the front of the current path,
- produce the completed path for output,
- release owned resources through Rust ownership rather than manual freeing.

### 2.6 Failure behavior

The module performs filesystem-dependent operations whose failure prevents correct path reporting. The Rust version must preserve command-level failure behavior:

- when path validation or reconstruction cannot be completed successfully, the program must terminate as a failure rather than printing an unverified path;
- success output must only occur after a valid final path is obtained.

## 3. User Scenarios & Testing

### 3.1 Default invocation in a normal directory

**Scenario**: A user runs the program with no special mode selection while inside a non-root directory.

**Expected behavior**:
- the program prints the current working directory as a single absolute pathname line;
- the output corresponds to the actual current directory;
- the process exits successfully.

**Traceability**: `main`, `robust_getcwd`, `find_dir_entry`.

### 3.2 Invocation while the current directory is the filesystem root

**Scenario**: A user runs the program when the current directory is `/`.

**Expected behavior**:
- the program prints `/`;
- no extra path components are added;
- the process exits successfully.

**Traceability**: `robust_getcwd`, `find_dir_entry`.

### 3.3 Logical path reuse when valid

**Scenario**: A user invokes the program in a mode that permits logical path use, and the environment-provided working-directory string identifies the same directory as the actual current directory.

**Expected behavior**:
- the program accepts the logical path;
- it prints that validated path instead of reconstructing a different physical one;
- the process exits successfully.

**Traceability**: `main`, `stat`-based validation in `pwd.c`.

### 3.4 Fallback when logical path is invalid

**Scenario**: A user invokes the program in a mode that permits logical path use, but the environment-provided working-directory value is stale, missing, or does not match the current directory.

**Expected behavior**:
- the program rejects the invalid logical path;
- it reconstructs the path using filesystem traversal;
- it prints the reconstructed absolute path;
- the process exits successfully if reconstruction succeeds.

**Traceability**: `main`, `robust_getcwd`, `find_dir_entry`.

### 3.5 Physical mode requiring filesystem-derived output

**Scenario**: A user invokes the program in physical mode.

**Expected behavior**:
- the program does not rely on a merely available logical path unless it satisfies the module’s validation rules for the selected behavior;
- it outputs a path derived consistently with physical directory identity;
- the process exits successfully if resolution succeeds.

**Traceability**: `main`, `robust_getcwd`, `find_dir_entry`.

### 3.6 Failure during directory traversal

**Scenario**: A required filesystem operation fails while identifying parent entries or reconstructing the path.

**Expected behavior**:
- the program does not print an unverified or partial path as success output;
- the process exits with failure.

**Traceability**: `find_dir_entry`, `robust_getcwd`, `main`.

### 3.7 Testing guidance

The Rust version must support tests covering at least:

- output at `/`,
- output in nested directories,
- acceptance of a valid logical working-directory path,
- rejection of an invalid logical working-directory path with fallback reconstruction,
- failure propagation when traversal cannot be completed.

## 4. Requirements

### 4.1 Functional Requirements

#### FR-1: Program entry behavior
The module shall provide the executable behavior for current-directory reporting, including argument-driven selection of logical versus physical handling and final path output.

**Traceability**: `main`.

#### FR-2: Current directory identity validation
The module shall validate any candidate reusable working-directory path against the actual current directory using filesystem metadata before treating it as authoritative output.

**Traceability**: `main`, `stat`, `dev_ino`.

#### FR-3: Robust current-directory reconstruction
The module shall reconstruct the current directory pathname when direct reuse of a validated path is not available.

**Traceability**: `robust_getcwd`.

#### FR-4: Root detection
The module shall detect when the current directory is the root of the traversal and terminate reconstruction at that point.

**Traceability**: `robust_getcwd`, `find_dir_entry`, `stat`, `dev_ino`.

#### FR-5: Parent entry discovery
For a non-root directory, the module shall locate the directory entry within the parent that refers to the current directory and use that entry name as the next pathname component.

**Traceability**: `find_dir_entry`, `dirent`, `stat`.

#### FR-6: Front-prepending path assembly
The module shall assemble the final pathname by prepending discovered directory component names onto an internal path accumulator until the full absolute path is formed.

**Traceability**: `file_name_init`, `file_name_prepend`, `robust_getcwd`, `find_dir_entry`, `file_name`.

#### FR-7: Success output
Upon successful path determination, the module shall output exactly one completed pathname suitable for command-line use.

**Traceability**: `main`, `file_name`.

#### FR-8: Failure on unrecoverable filesystem errors
The module shall terminate with failure if filesystem inspection required for validation or reconstruction cannot be completed correctly.

**Traceability**: `find_dir_entry`, `robust_getcwd`, `main`.

### 4.2 Key Entities

#### `file_name`
An internal pathname accumulator used to hold the directory path under construction. It supports initialization, front insertion of path segments, and cleanup.

**Relationships**:
- populated by `robust_getcwd`,
- updated by `find_dir_entry`,
- managed by `file_name_init`, `file_name_prepend`, and `file_name_free`,
- consumed by `main` for final output.

#### `stat`
Filesystem metadata used to compare directory identity and validate whether two paths refer to the same directory.

**Relationships**:
- used in `main` to validate candidate logical paths,
- used in `find_dir_entry` and `robust_getcwd` to compare current and parent directories.

#### `dev_ino`
A directory identity concept referenced by the module to compare directories by device/inode-style identity rather than pathname text.

**Relationships**:
- supports root detection and directory matching in `robust_getcwd` and `main`.

#### `dirent`
A parent-directory entry examined during traversal to identify the name corresponding to the current directory.

**Relationships**:
- used by `find_dir_entry` while scanning the parent directory.

## 5. Success Criteria

### 5.1 Functional correctness

- The Rust module prints `/` when executed with the current directory at root.
  **Traceability**: `robust_getcwd`, `find_dir_entry`.

- The Rust module prints an absolute path representing the actual current directory when executed from nested directories.
  **Traceability**: `robust_getcwd`, `find_dir_entry`, `main`.

- When a candidate logical working-directory path is valid for the current directory under the selected mode, the Rust module accepts and outputs it.
  **Traceability**: `main`, `stat`.

- When a candidate logical working-directory path is invalid, the Rust module rejects it and successfully falls back to reconstructed output if traversal succeeds.
  **Traceability**: `main`, `robust_getcwd`, `find_dir_entry`.

### 5.2 Behavioral integrity

- The Rust module determines root and directory identity using filesystem metadata comparisons rather than path-string assumptions.
  **Traceability**: `main`, `robust_getcwd`, `find_dir_entry`, `stat`, `dev_ino`.

- The Rust module forms the reconstructed path from parent directory entry names discovered during traversal.
  **Traceability**: `find_dir_entry`, `file_name_prepend`.

- The Rust module produces one final pathname on success and does not emit a partial pathname as a successful result after traversal failure.
  **Traceability**: `main`, `robust_getcwd`, `find_dir_entry`.

### 5.3 Port completion criteria

- The Rust implementation on branch `003-main_root_file_name_03-rust-port` covers all functional requirements in this specification.
- Each requirement is demonstrably mapped to behavior exercised by tests for the scenarios listed above.
- No additional externally visible capabilities are introduced beyond the behavior evidenced by `pwd.c`.