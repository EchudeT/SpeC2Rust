# spec.md

## Title

Functional Specification for `main_root_pwd.c_23` Rust Port

## Metadata

- Project: `pwd`
- Module: `main_root_pwd.c_23`
- Category: `main_cluster`
- Source file: `pwd.c`
- Rust branch: `023-main_root_pwd.c_23-rust-port`
- Generation date: 2026-06-09

## Overview

This module provides the core top-level behavior for the `pwd` program area represented in `pwd.c`, including:

- emitting usage/help text through `usage`
- computing a path string for an ancestor directory through `nth_parent`

The Rust rewrite must preserve the observable behavior of these responsibilities as evidenced by the analyzed source elements. In particular, the module must support producing user-facing usage output and constructing a parent-path string based on a requested ancestor depth.

## Scope

In scope for this module specification:

- behavior associated with `usage(int status)`
- behavior associated with `nth_parent(size_t n)`
- handling of pathname-oriented data represented by `struct file_name`
- interaction with directory and filesystem identity data where required by the source module’s path logic (`struct stat`, `struct dirent`, `struct dev_ino`, `struct option` as referenced by the module)

Out of scope:

- any new CLI features not evidenced by the source module
- any new public API beyond what is needed to preserve the module’s existing behavior
- performance, concurrency, serialization, FFI, or recovery guarantees not evidenced by the source

## Feature Specification

### 1. Usage/help behavior

The module must provide behavior equivalent to `usage(int status)`.

Required behavior:

- Produce user-facing usage information for the program/module context.
- Support at least two status-driven outcomes:
  - a successful help/usage path
  - a non-success path for invalid invocation or similar error-triggered usage display
- The emitted content must be suitable for command-line use and must clearly communicate how the program is intended to be invoked.

Traceability:
- `pwd.c:48-75` — `usage`

### 2. Ancestor path construction

The module must provide behavior equivalent to `nth_parent(size_t n)`.

Required behavior:

- Accept a non-negative ancestor count `n`.
- Return a pathname string representing the `n`th parent relationship.
- The result must be a valid path-form string usable by the rest of the module’s path-handling logic.
- The behavior must be deterministic for a given `n`.

Minimum evidenced interpretation from the source role:

- when asked for a parent depth, the module constructs a textual path describing ascent through parent directories rather than resolving a filesystem query by itself

Traceability:
- `pwd.c:126-139` — `nth_parent`

### 3. Filesystem/path identity participation

The module operates in a context that uses filesystem and pathname entities to determine or validate current-directory-related output.

The Rust rewrite must preserve the module’s ability to work with:

- mutable pathname buffers or pathname objects
- filesystem metadata comparisons
- directory entry information
- device/inode identity pairs where needed by surrounding path computation behavior

This requirement is limited to preserving the functional boundaries evidenced by the source types and their use context; it does not require exposing those C types directly in Rust.

Traceability:
- `struct file_name` references throughout `pwd.c`
- `struct stat` references throughout `pwd.c`
- `struct dirent` at `pwd.c:183`
- `struct dev_ino` at `pwd.c:271-272`
- `struct option` at `pwd.c:39`

## User Scenarios & Testing

### Scenario 1: User requests help/usage output

A user invokes the program in a way that requires displaying usage/help text.

Expected behavior:

- the module emits usage information
- the module distinguishes successful help display from error-associated usage display via status
- the observable result matches command-line utility conventions for success vs. failure signaling

Suggested tests:

- invoke the usage path with a success status and verify usage text is emitted
- invoke the usage path with a failure status and verify usage text is emitted with non-success signaling

Traceability:
- `usage`

### Scenario 2: Internal logic needs a path to the immediate parent

The program needs a string representing one directory ascent.

Expected behavior:

- requesting parent depth `1` yields a parent-path string expressing one upward traversal

Suggested tests:

- call the Rust equivalent of `nth_parent(1)` and verify the returned string denotes one parent ascent

Traceability:
- `nth_parent`

### Scenario 3: Internal logic needs a path to a deeper ancestor

The program needs a string representing multiple directory ascents.

Expected behavior:

- requesting parent depth `n > 1` yields a path string expressing exactly `n` upward traversals in order

Suggested tests:

- verify `n = 2` returns a string denoting two parent ascents
- verify `n = 3` returns a string denoting three parent ascents
- verify no extra path segments are introduced

Traceability:
- `nth_parent`

### Scenario 4: Internal logic needs the zero-depth ancestor form

The program requests an ancestor path with depth `0`.

Expected behavior:

- the module returns the path form corresponding to zero ascents, consistent with the source module’s ancestor-path construction behavior

Suggested tests:

- call the Rust equivalent of `nth_parent(0)` and verify it returns the module’s zero-depth path form consistently across repeated calls

Traceability:
- `nth_parent`

### Scenario 5: Path logic participates in current-directory determination

The broader `pwd` flow uses pathname objects and filesystem identity information while determining the printed working directory.

Expected behavior:

- the module’s Rust implementation accepts or internally manages path data in a way that supports comparison or traversal logic equivalent to the source module’s usage of pathname, `stat`, directory-entry, and device/inode identity information

Suggested tests:

- integration tests exercising current-directory reporting through normal directory layouts
- integration tests in nested directories to ensure ancestor path construction remains compatible with overall path determination

Traceability:
- `struct file_name`
- `struct stat`
- `struct dirent`
- `struct dev_ino`

## Requirements

### Functional Requirements

#### FR-1: Usage output
The Rust module shall provide behavior equivalent to `usage(int status)` that emits command-line usage/help text for this program area.

Traceability:
- `pwd.c:48-75`

#### FR-2: Status-sensitive usage outcome
The Rust module shall preserve status-sensitive behavior for usage emission, distinguishing successful and non-successful invocation outcomes.

Traceability:
- `usage(int status)`

#### FR-3: Ancestor path generation
The Rust module shall provide behavior equivalent to `nth_parent(size_t n)` that constructs and returns a pathname string for the requested ancestor depth `n`.

Traceability:
- `pwd.c:126-139`

#### FR-4: Exact ascent count
For any supported non-negative `n`, the generated ancestor path shall encode exactly `n` parent traversals and no additional traversal segments.

Traceability:
- `nth_parent(size_t n)`

#### FR-5: Deterministic path construction
Repeated requests with the same ancestor depth shall produce equivalent path strings.

Traceability:
- `nth_parent(size_t n)`

#### FR-6: Path data compatibility
The Rust module shall represent and manipulate pathname data in a way that preserves the functional role of the source module’s `struct file_name`.

Traceability:
- `struct file_name` references in `pwd.c`

#### FR-7: Filesystem metadata compatibility
The Rust module shall preserve the module’s ability to participate in filesystem metadata-based decisions comparable to the source module’s use of `struct stat`.

Traceability:
- `struct stat` references in `pwd.c`

#### FR-8: Directory entry compatibility
The Rust module shall preserve the module’s ability to participate in directory-entry-based traversal or lookup comparable to the source module’s use of `struct dirent`.

Traceability:
- `pwd.c:183`

#### FR-9: Device/inode identity compatibility
The Rust module shall preserve the module’s ability to compare or carry filesystem identity pairs comparable to the source module’s use of `struct dev_ino`.

Traceability:
- `pwd.c:271-272`

#### FR-10: Option/usage integration compatibility
The Rust module shall preserve compatibility with option-driven program usage behavior comparable to the source module’s use of `struct option`, to the extent required for usage handling in this module.

Traceability:
- `pwd.c:39`

### Key Entities

#### `file_name`
A pathname-carrying entity used throughout the module to hold or build filesystem path values. In the Rust rewrite, this must map to an internal representation capable of safe pathname construction and reuse.

Relationships:
- used by ancestor-path construction
- participates in broader current-directory path handling

Traceability:
- `struct file_name` references at `pwd.c:32-37, 78, 84, 87, 101, 153, 268, 387`

#### `stat`
Filesystem metadata used to identify or compare filesystem objects relevant to path determination.

Relationships:
- associated with pathname objects
- used with directory and identity comparisons

Traceability:
- `struct stat` references at `pwd.c:153, 158, 184, 273, 302, 303`

#### `dirent`
Directory entry information used during directory traversal or name discovery.

Relationships:
- works with `stat` and pathname data during path-oriented logic

Traceability:
- `struct dirent` at `pwd.c:183`

#### `dev_ino`
A filesystem identity pair representing device/inode-style identity.

Relationships:
- compared against filesystem metadata to support object identity decisions

Traceability:
- `struct dev_ino` at `pwd.c:271-272`

#### `option`
Option-description data associated with command-line usage/help behavior.

Relationships:
- supports the content and structure of usage presentation

Traceability:
- `struct option` at `pwd.c:39`

## Success Criteria

### Functional correctness

1. The Rust implementation provides a usage/help path that emits human-readable invocation guidance and preserves success vs. failure outcome distinction.
   - Verification: automated tests covering both status classes
   - Traceability: `usage`

2. The Rust implementation provides ancestor path generation for `n = 0`, `1`, and multiple larger values.
   - Verification: automated unit tests for representative depths
   - Traceability: `nth_parent`

3. For tested values of `n`, the generated path contains exactly the requested number of parent traversals and no extra segments.
   - Verification: string-form assertions on generated paths

4. Repeated generation for the same `n` yields equivalent results.
   - Verification: repeated-call equality tests

### Integration compatibility

5. The Rust module integrates with pathname and filesystem-metadata handling sufficient to support the surrounding `pwd.c` current-directory logic represented by the source entities.
   - Verification: integration tests in real directory hierarchies
   - Traceability: `file_name`, `stat`, `dirent`, `dev_ino`

6. The Rust port introduces no required user-visible capability beyond the source-evidenced usage/help and ancestor-path behavior.
   - Verification: review of exported behavior against this specification
   - Traceability: module scope and source function set

## Notes for Porting

- The Rust rewrite should preserve observable behavior, not C-specific memory layout.
- Source C structs named here define functional roles only; Rust may model them idiomatically as long as the same behavior is preserved.
- Any behavior not evidenced by `usage`, `nth_parent`, or the listed source entities should remain unspecified in this module-level port document.