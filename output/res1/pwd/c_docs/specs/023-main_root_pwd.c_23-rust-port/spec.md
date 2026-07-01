# spec.md

## Title
Rust Functional Specification for `main_root_pwd.c_23`

## Metadata
- Project: `pwd`
- Module: `main_root_pwd.c_23`
- Category: `main_cluster`
- Source file: `pwd.c`
- Rust branch: `023-main_root_pwd.c_23-rust-port`
- Generation date: `2026-06-07`

## Overview
This module provides the main path-reporting behavior for the `pwd` program portion represented by `pwd.c`. Based on the analyzed functions and referenced filesystem structures, the module is responsible for:

- presenting command usage/help text (`usage`)
- computing an ancestor path string by walking upward a requested number of directory levels (`nth_parent`)

The Rust rewrite must preserve these observable behaviors within the module boundary evidenced by `pwd.c`. The specification covers only functionality supported by the analyzed source evidence and referenced filesystem entities.

## Scope
In scope:

- formatting and emitting program usage/help information through the module’s usage path
- deriving a pathname that represents the `n`th parent of a current path context
- handling filesystem path relationships needed to identify parent directories
- interacting with filesystem metadata and directory entries as required by path reconstruction logic

Out of scope:

- any new command-line interface beyond behavior evidenced by the usage function
- any persistence, networking, FFI, concurrency, or non-filesystem extensions
- any guarantees not supported by the analyzed module inputs

## Feature Specification

### Feature 1: Usage/help reporting
The module shall provide a usage-reporting behavior corresponding to `usage(int status)`.

Required behavior:
- Emit program usage/help text appropriate to the supplied exit status.
- Support both successful informational output and non-success usage/error output through the `status` argument.
- Terminate the usage path in a manner consistent with the provided status.

Traceability:
- `usage` in `pwd.c:48-75`

### Feature 2: Parent path derivation
The module shall provide behavior corresponding to `nth_parent(size_t n)`.

Required behavior:
- Accept a non-negative parent depth `n`.
- Produce a path string representing the ancestor located `n` levels above the current location/path context used by the module.
- Return a path result suitable for further path assembly or output.
- Support repeated parent traversal up to the filesystem root boundary.
- Handle root-boundary traversal without inventing nonexistent parents above root.

Traceability:
- `nth_parent` in `pwd.c:126-139`

### Feature 3: Filesystem-based path resolution support
The module shall support the filesystem inspection needed by the path-building behavior evidenced by referenced types in `pwd.c`.

Required behavior:
- Inspect file metadata sufficient to compare filesystem identity during upward traversal.
- Inspect directory entries when needed to identify the current directory’s name within its parent.
- Build and maintain path fragments as pathname components are discovered.

This feature is required because the module references:
- file metadata (`struct stat`)
- directory enumeration (`struct dirent`)
- path fragment storage (`struct file_name`)
- device/inode identity comparison (`struct dev_ino`)

Traceability:
- `struct stat` references in `pwd.c`
- `struct dirent` reference in `pwd.c:183`
- `struct file_name` references across `pwd.c`
- `struct dev_ino` references in `pwd.c:271-272`

## User Scenarios & Testing

### Scenario 1: User requests help or usage text
A user invokes the program in a way that triggers the usage path. The module prints usage/help content and exits according to the provided status.

Test expectations:
- With a success-oriented status, usage/help text is emitted and the process exits successfully.
- With a failure-oriented status, usage/help or diagnostic usage text is emitted and the process exits unsuccessfully.

Traceability:
- `usage`

### Scenario 2: Caller asks for the immediate parent
The path-resolution logic requests `nth_parent(1)`. The module returns the pathname corresponding to one level above the current directory context.

Test expectations:
- For a nested working directory, the returned path identifies its direct parent.
- The returned string is usable as a valid path string.

Traceability:
- `nth_parent`

### Scenario 3: Caller asks for the current level ancestor
The path-resolution logic requests `nth_parent(0)`. The module returns the path representing zero levels of ascent from the current context.

Test expectations:
- The result is a valid path string representing the current level in the module’s parent-derivation convention.
- No upward traversal beyond the requested depth occurs.

Traceability:
- `nth_parent`

### Scenario 4: Caller asks for an ancestor above multiple levels
The module is used to derive a path several levels upward from a nested directory.

Test expectations:
- For depth `n > 1`, the returned path reflects exactly `n` parent traversals, unless root is reached first.
- The path components are assembled in correct directory order.

Traceability:
- `nth_parent`
- `struct file_name`

### Scenario 5: Traversal reaches filesystem root
The module processes a path whose ancestor chain reaches root before or at the requested depth.

Test expectations:
- The module does not produce a fictitious parent above root.
- The result remains a valid root-bounded path.

Traceability:
- `nth_parent`
- `struct stat`
- `struct dev_ino`

### Scenario 6: Directory identity must be resolved through metadata
The module needs to determine which directory entry in a parent corresponds to the current directory during path reconstruction.

Test expectations:
- Directory entry inspection and metadata comparison correctly identify the matching component.
- The resulting pathname matches filesystem identity rather than relying only on string assumptions.

Traceability:
- `struct dirent`
- `struct stat`
- `struct dev_ino`

## Requirements

### Functional Requirements

#### FR-1: Usage status handling
The Rust module shall implement a usage/help path that accepts an exit-status input and produces usage output consistent with that status.

Traceability:
- `usage`

#### FR-2: Usage output emission
The Rust module shall emit human-readable usage/help text when the usage path is invoked.

Traceability:
- `usage`

#### FR-3: Ancestor path computation
The Rust module shall implement computation of the `n`th parent path for a non-negative depth input.

Traceability:
- `nth_parent`

#### FR-4: Root-bounded traversal
The Rust module shall stop parent traversal at the filesystem root boundary and shall not represent ancestors above root.

Traceability:
- `nth_parent`
- `struct stat`
- `struct dev_ino`

#### FR-5: Path component assembly
The Rust module shall support building path strings from discovered directory components.

Traceability:
- `struct file_name`
- `nth_parent`

#### FR-6: Filesystem identity inspection
The Rust module shall inspect filesystem metadata sufficient to compare directories during path resolution.

Traceability:
- `struct stat`
- `struct dev_ino`

#### FR-7: Directory enumeration support
The Rust module shall inspect directory entries where needed to determine the name of a directory within its parent.

Traceability:
- `struct dirent`
- `struct stat`

### Key Entities

#### `file_name`
A path-construction entity used to hold and assemble pathname content. It is the central string/path container referenced repeatedly through the module and is used in parent-path derivation and path reconstruction.

Relationship:
- receives directory component content discovered during traversal
- contributes to final path string output

Traceability:
- `struct file_name` references across `pwd.c`

#### `stat`
Filesystem metadata describing a file or directory identity and attributes required for traversal decisions.

Relationship:
- compared between current and parent contexts
- used with directory entries and device/inode identity to recognize matching directories

Traceability:
- `struct stat` references across `pwd.c`

#### `dirent`
Directory entry data used when scanning a parent directory to find the entry corresponding to the current directory.

Relationship:
- enumerated from a directory
- matched against metadata from `stat`

Traceability:
- `struct dirent` in `pwd.c:183`

#### `dev_ino`
A filesystem identity pair representing device and inode information.

Relationship:
- derived from or compared with file metadata
- used to determine whether two references denote the same filesystem object during upward traversal

Traceability:
- `struct dev_ino` in `pwd.c:271-272`

#### `option`
Option-description data associated with command usage/help processing.

Relationship:
- supports the usage/reporting path

Traceability:
- `struct option` in `pwd.c:39`

## Success Criteria

1. Invoking the Rust module’s usage path with a success status emits usage/help text and completes with a success exit status.
   - Traceability: `usage`

2. Invoking the Rust module’s usage path with a failure status emits usage/help text and completes with a failure exit status.

3. For a directory with at least one parent, ancestor computation for depth `1` returns the direct parent path.
   - Traceability: `nth_parent`

4. Ancestor computation for depth `0` returns a valid path representation for zero-level ascent under the module’s preserved behavior.

5. For nested directories, ancestor computation returns paths corresponding to the requested number of parent levels in correct order until root is reached.
   - Traceability: `nth_parent`, `struct file_name`

6. When the requested depth reaches or exceeds the root boundary, the Rust module returns a root-bounded result and does not synthesize parents above root.
   - Traceability: `nth_parent`, `struct stat`, `struct dev_ino`

7. In path reconstruction cases requiring parent-directory scanning, the Rust module identifies the correct directory entry by filesystem identity and produces the matching pathname component.
   - Traceability: `struct dirent`, `struct stat`, `struct dev_ino`

8. Path strings produced by parent traversal are valid for output or further internal path assembly within the module.
   - Traceability: `struct file_name`, `nth_parent`

## Acceptance Notes
- Conformance is defined by preserved observable behavior of the analyzed module surface and its evidenced filesystem interactions.
- The Rust rewrite may change internal implementation, but it must not reduce the supported usage/help behavior or the correctness of ancestor path derivation evidenced by `pwd.c`.