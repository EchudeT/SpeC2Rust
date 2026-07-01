# Implementation Plan: main_root_version_etc_05

## Summary

This module ports the functionality from `version-etc.c` into Rust for the `cat` project branch `006-main_root_version_etc_05-rust-port`. The implementation should preserve the existing role of formatting and emitting version and author information, while translating C varargs- and array-oriented entry points into safe Rust function interfaces.

The Rust approach should keep the implementation narrowly aligned with the existing C file and functions:

- migrate the logic from `version_etc_arn`
- migrate the logic from `version_etc_ar`
- migrate the logic from `version_etc_va`
- migrate the logic from `version_etc`

The port should prefer:

- safe string handling with `&str` and slices
- explicit I/O targets through standard library writer types
- standard Rust error propagation instead of C-style implicit write/error handling
- minimal internal helper functions only where needed to share formatting logic among the four migrated functions

No additional capabilities should be introduced beyond what is required to replace the behavior of the existing C module.

## Technical Context

### Language/Version

- Rust stable
- Minimum recommended version: **Rust 1.75+**

### Primary Dependencies

Use the Rust standard library by default.

Recommended dependencies:

- **None required** for this module

Standard library components expected:

- `std::io::{self, Write}`
- `std::fmt`
- `std::slice`

### Testing

- `cargo test`

Testing focus:

- output formatting equivalence for the migrated entry points
- handling of empty, single, and multiple author lists
- consistent behavior across the slice-based replacement of C array/varargs forms
- write error propagation from `Write` implementors

### Performance Goals

This module is output-formatting code and is not performance-critical. Goals:

- no unnecessary heap allocation beyond what is needed for formatting
- avoid repeated formatting logic by sharing one internal formatting path
- maintain linear behavior relative to author count
- preserve straightforward write-through behavior to the destination writer

## Module Mapping

### C to Rust File Mapping

- `version-etc.c` -> `src/version_etc.rs`

If this module is invoked from the crate root or main program flow, expose it with a standard module declaration:

- `src/version_etc.rs`
- referenced from existing crate module declarations only as needed for current call sites

### Function Mapping

The C functions should map to Rust functions with interfaces adapted to Rust safety and ownership rules.

- `version_etc_arn`
  - Rust: `pub fn version_etc_arn<W: Write>(writer: &mut W, command_name: &str, package: &str, version: &str, authors: &[&str]) -> io::Result<()>`
  - Purpose: primary array/count-style entry point in Rust using a slice instead of pointer-plus-count

- `version_etc_ar`
  - Rust: `pub fn version_etc_ar<W: Write>(writer: &mut W, command_name: &str, package: &str, version: &str, authors: &[&str]) -> io::Result<()>`
  - Purpose: retained as a distinct migrated API if existing call sites require it; internally should delegate to the same shared formatter

- `version_etc_va`
  - Rust: replace direct C varargs behavior with an internal/shared non-varargs Rust helper taking `&[&str]`
  - Because Rust does not support C-style variadic safe functions for this use case, this function should not attempt to emulate C varargs literally unless existing crate structure strictly requires a similarly named internal function
  - If name retention is required for migration traceability, implement as a private helper accepting a slice

- `version_etc`
  - Rust: `pub fn version_etc<W: Write>(writer: &mut W, command_name: &str, package: &str, version: &str, authors: &[&str]) -> io::Result<()>`
  - Purpose: public convenience wrapper matching the C top-level role, delegating to the shared implementation

### Internal Structure

Keep implementation compact:

- one shared internal formatter for version/author emission
- optional helper for author-line formatting if needed
- no extra abstraction layers beyond what is needed to replace the original file

## Data Model

This module analysis reports no C data structures.

### Data-Structure Mapping

- C structs: none
- Rust structs/enums: none required

### Type Mapping for Function Inputs

C-style inputs should map as follows:

- `FILE *` or stream-like target -> `&mut impl std::io::Write`
- `const char *` -> `&str`
- `const char **` / author arrays -> `&[&str]`
- count parameters such as `size_t n_authors` -> encoded by slice length in Rust
- C varargs author lists -> explicit slice arguments in Rust

### Memory Management

Rust ownership removes the need for manual lifetime and pointer management used in C:

- borrowed string inputs avoid copying unless formatting requires temporary buffers
- author lists should be borrowed as slices
- writing should stream directly to the provided writer
- no raw pointers should be introduced

### Error Handling

C output functions may rely on stream state or unchecked writes; in Rust:

- all write operations should return `io::Result<()>`
- callers decide whether to propagate or handle write failures
- do not hide I/O errors behind boolean or integer status conversions unless an existing crate API already requires that pattern

## Implementation Phases

## Phase 1: Create Rust Module Skeleton and Shared Signature Shape

### Goals

- establish the Rust file replacing `version-etc.c`
- define the public migrated entry points
- align signatures with Rust-safe equivalents

### Tasks

- create `src/version_etc.rs`
- add the migrated public functions:
  - `version_etc_arn`
  - `version_etc_ar`
  - `version_etc`
- define one private shared helper that accepts:
  - writer
  - command/package/version strings
  - author slice
- decide whether `version_etc_va` remains as a private helper name for migration traceability or is folded into the shared helper
- wire module declaration into the existing crate only where current build structure requires it

### Deliverables

- compiling module skeleton
- placeholder tests for basic invocation signatures
- no additional modules or support layers beyond this file

## Phase 2: Port Formatting Logic from C to Shared Rust Implementation

### Goals

- migrate the formatting/output behavior from `version-etc.c`
- ensure all entry points route through the same formatting path

### Tasks

- port the version header emission logic into write calls using `write!` / `writeln!`
- port author-list rendering from C array/count handling to slice iteration
- preserve edge-case formatting behavior from the C implementation:
  - zero authors
  - one author
  - multiple authors
- implement `version_etc_arn` and `version_etc_ar` as thin delegators if their behavior is equivalent after slice conversion
- implement `version_etc` as the top-level convenience entry point
- keep temporary allocation minimal; prefer direct writing over building large intermediate strings

### Deliverables

- complete migrated logic in `src/version_etc.rs`
- shared helper used by all applicable entry points
- successful compilation with `cargo test`

## Phase 3: Error Semantics and Output Equivalence Tests

### Goals

- verify output behavior and I/O failure propagation
- lock in migration correctness for all exposed functions

### Tasks

- add unit tests comparing produced output against expected text for:
  - empty author slice
  - single author
  - two authors
  - longer author lists
- add tests covering each public entry point to confirm equivalent output
- add a custom failing writer in tests to verify `io::Result` propagation
- review newline placement and separator formatting against the C source behavior during port validation

### Deliverables

- passing `cargo test`
- coverage of formatting branches and error propagation
- verified function parity at the module boundary

## Phase 4: Integrate Existing Call Sites and Remove C-Oriented Assumptions

### Goals

- connect the Rust module to existing crate call sites
- finish migration from C-oriented calling patterns to Rust slice-based usage

### Tasks

- update existing internal call sites to pass `&[&str]` instead of count-plus-pointer or variadic assumptions
- remove any unnecessary placeholder compatibility code left from early scaffolding
- confirm no raw-pointer or unsafe interfaces remain in this module
- keep public surface limited to functions actually needed by current project structure

### Deliverables

- module integrated on branch `006-main_root_version_etc_05-rust-port`
- final API uses Rust-native borrowed inputs and `io::Result<()>`
- no extra migration scaffolding beyond required file/function replacements