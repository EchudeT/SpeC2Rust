# spec.md

## Title

Functional Specification for `main_root_pwd.c_23` Rust Port

## Metadata

- **Project**: `pwd`
- **Module**: `main_root_pwd.c_23`
- **Category**: `main_cluster`
- **Source files**: `pwd.c`
- **Rust branch**: `023-main_root_pwd.c_23-rust-port`
- **Generation date**: `2026-06-07`

## Overview

This module provides the main user-facing behavior for reporting the current working directory path. It includes:

- command-line usage/help output behavior, evidenced by `usage` in `pwd.c`
- path parent construction support used while building a pathname, evidenced by `nth_parent` in `pwd.c`
- path-oriented state management using linked filename segments, evidenced by repeated `struct file_name` usage in `pwd.c`
- filesystem identity comparisons using device/inode and file status data, evidenced by `struct dev_ino` and `struct stat` usage in `pwd.c`
- directory entry inspection during pathname reconstruction, evidenced by `struct dirent` usage in `pwd.c`
- command-line option handling, evidenced by `struct option` usage in `pwd.c`

The Rust rewrite must preserve the observable behavior of this module as a command-line `pwd` implementation component, including help/usage presentation and correct current-directory path reporting behavior driven by filesystem inspection.

## Feature Specification

### 1. Current working directory reporting

The module shall provide behavior to determine and emit the current working directory path as text.

This behavior is evidenced by the module’s pathname-building helpers and filesystem traversal state in `pwd.c`, including:

- `nth_parent`
- `struct file_name`
- `struct stat`
- `struct dirent`
- `struct dev_ino`

The Rust version must implement equivalent functional behavior sufficient to:

- obtain the current directory path
- construct ancestor references as needed while traversing toward parent directories
- identify directory relationships using filesystem metadata
- produce a pathname representation for the current working directory

### 2. Parent path generation support

The module shall support generating a parent-directory reference representing repeated ascent by `n` levels.

This behavior is directly evidenced by:

- `static char * nth_parent (size_t n);` in `pwd.c:126-139`

The Rust version must implement equivalent behavior that functionally yields a path corresponding to:

- `"."` when zero parent ascents are requested, if that is the source behavior
- repeated `"../"`-style ascent when one or more parent levels are requested

The exact textual form must match the source module’s externally relevant behavior where used in pathname reconstruction.

### 3. Usage/help output

The module shall provide command-line usage output.

This behavior is directly evidenced by:

- `void usage (int status);` in `pwd.c:48-75`
- `struct option` usage in `pwd.c`

The Rust version must implement equivalent user-visible behavior for:

- displaying usage/help text
- honoring the status-dependent exit path associated with usage output

This requirement is limited to the behavior evidenced by the module and does not imply new CLI features beyond those supported by the source.

### 4. Filesystem-based path reconstruction

The module shall support reconstructing a pathname using filesystem traversal and identity comparison rather than relying only on preexisting environment text.

This behavior is evidenced by the presence and relationships of:

- `struct stat`
- `struct dirent`
- `struct dev_ino`
- `struct file_name`
- `nth_parent`

The Rust version must implement behavior that:

- inspects the current directory and parent directories
- compares filesystem identity information to recognize when directories correspond
- locates the current directory’s entry name within its parent when needed
- accumulates path segments into the final absolute or reconstructed path form used by the source module

### 5. Path segment accumulation

The module shall maintain pathname fragments while constructing the result.

This behavior is evidenced by repeated use of:

- `struct file_name`

The Rust version must preserve the functional role of this entity by supporting:

- storage of path components or component chains
- assembly of the final textual path from those components

## User Scenarios & Testing

### Scenario 1: User requests help

A user invokes the program with a help/usage-triggering option.

Expected behavior:

- the module prints usage/help text
- the process exits with the status associated with the help path implemented by `usage`

Testing focus:

- verify usage text is emitted
- verify the exit status matches the source behavior for help output

Traceability:

- `usage`
- `struct option`

### Scenario 2: User runs the program in a normal directory

A user invokes the program from a standard directory location with no special failure conditions.

Expected behavior:

- the module determines the current working directory
- it prints the path in the same functional form as the source module

Testing focus:

- verify a non-empty path is printed
- verify the path resolves to the invoking process’s current directory
- verify behavior matches the source for representative nested directories

Traceability:

- `nth_parent`
- `struct file_name`
- `struct stat`
- `struct dirent`
- `struct dev_ino`

### Scenario 3: User runs the program in nested directories

A user invokes the program from a directory several levels below the filesystem root.

Expected behavior:

- the module can conceptually ascend parent levels as needed
- it reconstructs the correct pathname by identifying each directory entry name in its parent chain

Testing focus:

- verify correct output from directories at multiple nesting depths
- verify no path segments are lost or reordered

Traceability:

- `nth_parent`
- `struct file_name`
- `struct stat`
- `struct dirent`
- `struct dev_ino`

### Scenario 4: Root or top-of-filesystem traversal boundary

A user invokes the program from a directory at or effectively equivalent to filesystem root.

Expected behavior:

- traversal stops at the correct root boundary
- output is the correct root pathname form for the platform behavior represented by the source module

Testing focus:

- verify root detection through filesystem identity comparisons
- verify no extra parent components are introduced above root

Traceability:

- `struct stat`
- `struct dev_ino`
- `struct file_name`

### Scenario 5: Directory identity must be matched via metadata

A user invokes the program where pathname reconstruction requires matching the current directory against entries in the parent directory.

Expected behavior:

- the module uses filesystem identity data to find the correct entry name
- the output path corresponds to the actual current directory

Testing focus:

- verify directory entry matching is based on correct metadata relationship
- verify output remains correct in directories containing many entries

Traceability:

- `struct dirent`
- `struct stat`
- `struct dev_ino`

## Requirements

### Functional Requirements

#### FR-1: Usage/help behavior
The module shall provide a usage/help output path controlled by command-line parsing and a status argument to the usage routine.

Traceability:

- `pwd.c:48-75` `usage`
- `pwd.c:39` `struct option`

#### FR-2: Parent reference generation
The module shall generate a pathname fragment representing ascent by an arbitrary nonnegative number of parent levels.

Traceability:

- `pwd.c:126-139` `nth_parent`

#### FR-3: Current-directory path determination
The module shall determine the current working directory pathname and produce it as program output.

Traceability:

- `pwd.c`
- `nth_parent`
- `struct file_name`
- `struct stat`
- `struct dirent`
- `struct dev_ino`

#### FR-4: Filesystem traversal toward root
The module shall support repeated inspection of parent directories until the root boundary is detected.

Traceability:

- `pwd.c`
- `struct stat`
- `struct dev_ino`
- `struct file_name`

#### FR-5: Root-boundary detection
The module shall detect when traversal has reached the root by comparing filesystem identity information for directory levels.

Traceability:

- `pwd.c:271-273`
- `struct dev_ino`
- `struct stat`

#### FR-6: Directory entry name recovery
The module shall recover the basename of the current directory within its parent by scanning directory entries and comparing identity metadata.

Traceability:

- `pwd.c:183-184`
- `struct dirent`
- `struct stat`

#### FR-7: Path segment accumulation and final assembly
The module shall accumulate recovered directory names and assemble them into the final pathname output.

Traceability:

- `pwd.c:32-37, 78, 84, 87, 101, 153, 268, 387`
- `struct file_name`

### Key Entities

#### `file_name`
A pathname-construction entity used to hold and link filename or path-segment data during reconstruction of the current working directory path.

Role and relationships:

- participates in accumulation of path components
- is used alongside filesystem metadata gathered from `stat`
- contributes to final pathname assembly

Traceability:

- `pwd.c:32-37`
- additional uses at `pwd.c:78, 84, 87, 101, 153, 268, 387`

#### `dev_ino`
A filesystem identity entity representing device/inode information used to compare directories and determine equivalence or root boundaries.

Role and relationships:

- derived from or compared with `stat` data
- used during upward traversal and identity matching

Traceability:

- `pwd.c:271-272`

#### `stat`
Filesystem status metadata used to identify directories and compare entries during path reconstruction.

Role and relationships:

- supplies metadata for current directory, parent directory, and candidate directory entries
- works with `dev_ino` for identity comparison
- supports root detection and entry matching

Traceability:

- `pwd.c:153, 158, 184, 273, 302, 303`

#### `dirent`
Directory-entry metadata used while scanning a parent directory to find the entry corresponding to the current directory.

Role and relationships:

- provides candidate names during parent-directory scan
- paired with `stat` comparisons to confirm identity
- contributes recovered names to `file_name`

Traceability:

- `pwd.c:183`

#### `option`
Command-line option description data supporting usage/help behavior.

Role and relationships:

- supports CLI option parsing
- ties directly to invocation of `usage`

Traceability:

- `pwd.c:39`

## Success Criteria

1. **Help output parity**
   When invoked through a help/usage path supported by the source module, the Rust version emits usage/help text and exits with the same success/failure class as the source behavior.

   Traceability:
   - `usage`
   - `struct option`

2. **Correct current-directory output**
   For representative executions in existing directories, the Rust version outputs a path that resolves to the process’s actual current working directory.

   Traceability:
   - `pwd.c`
   - `struct file_name`
   - `struct stat`
   - `struct dirent`
   - `struct dev_ino`

3. **Correct behavior at multiple nesting depths**
   For directories at several parent depths below root, the Rust version returns the same effective pathname as the source module.

   Traceability:
   - `nth_parent`

4. **Correct root handling**
   When executed at filesystem root, the Rust version detects the traversal boundary correctly and does not prepend nonexistent parent segments.

   Traceability:

5. **Correct parent-entry identification**
   During pathname reconstruction, the Rust version correctly identifies the current directory’s name within its parent using filesystem metadata comparison equivalent in effect to the source module.

   Traceability:

6. **Parent fragment generation equivalence**
   The Rust version’s behavior for generating parent-directory references for nonnegative counts is functionally equivalent to `nth_parent` where used by the module.

   Traceability: