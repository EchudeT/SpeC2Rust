# Implementation Plan: module_gnu_open.c_39

## Summary

This module ports the logic currently contained in `gnu/open.c` into a focused Rust implementation that preserves the existing behavior of `orig_open` without adding new capabilities or restructuring unrelated code paths.

The Rust approach should center on:
- translating the single C entry point into a Rust function with equivalent call semantics as closely as practical,
- using `std` facilities for file opening and descriptor-oriented behavior where possible,
- isolating any platform-specific low-level open behavior behind a small internal function if direct flag fidelity is required,
- replacing C-style error propagation (`errno`/sentinel return values) with Rust `Result` internally while preserving the module’s expected outward behavior at integration boundaries.

Because this module contains one function and only anonymous data-structure references, the migration should remain narrow: one Rust source file, one exported function mapping, and only the minimum supporting types needed to represent C parameters and return values safely.

## Technical Context

- **Language/Version:** Rust 1.78+ stable
- **Primary Dependencies:** Rust standard library only (`std::fs`, `std::io`, `std::os::unix::*` as needed for Unix file-descriptor handling)
- **Testing:** `cargo test`
- **Performance Goals:**
  - Maintain behavior and runtime characteristics close to the C implementation
  - Avoid unnecessary allocations and copies in pathname handling
  - Preserve direct OS-backed open behavior with minimal abstraction overhead
  - Keep descriptor creation and error translation constant-time relative to the original implementation

## Module Mapping

### Source File Mapping
- `gnu/open.c` → `src/module_gnu_open.rs`

### Function Mapping
- `orig_open` → `pub(crate) fn orig_open(...) -> ...`

### Responsibility Mapping
- C module open logic → Rust module-local open wrapper logic
- C error return conventions → Rust internal `Result` plus boundary-compatible return conversion
- C pathname and flag handling → Rust string/OS-string and flag translation logic

## Data Model

The analysis identifies only anonymous structures, so the Rust port should avoid inventing named public models unless required by the migrated code.

### Data-structure Mapping
- `anonymous` → no standalone Rust type unless the C implementation uses a compound local state object that must be preserved; prefer local variables or a private struct
- `anonymous` → same treatment; introduce a private Rust struct only if needed to preserve grouped state during translation

### Primitive and ABI-oriented Mapping
- C pathname pointer types (`const char *`, if present) → `&CStr`, `&Path`, or `&OsStr` depending on the surrounding call path
- C integer flags/mode values → `i32`/`u32` or platform aliases as required
- C file descriptor return (`int`) → `std::os::unix::io::RawFd` internally or `i32` at the integration boundary
- C sentinel errors (`-1` + `errno`) → `std::io::Result<_>` internally, translated back only if the surrounding code requires it

### Memory Management Notes
- Borrow path inputs where possible; do not allocate owned strings unless encoding conversion forces it
- Avoid heap-backed wrapper objects for descriptor state
- Ensure ownership transfer of any opened file descriptor is explicit to prevent leaks or double-close behavior
- If `std::fs::File` is used internally, convert carefully with raw-descriptor extraction only at the exact boundary needed

## Implementation Phases

## Phase 1: Analyze and Scaffold the Rust Module

### Goals
Create the Rust module skeleton and lock down the exact signature and dependency surface needed for `orig_open`.

### Tasks
- Add `src/module_gnu_open.rs`
- Inspect the C implementation of `orig_open` and record:
  - exact parameter list,
  - return type,
  - flag handling requirements,
  - mode handling requirements,
  - any conditional logic around pathnames or descriptors
- Define the Rust function signature to mirror the C usage as closely as possible within the project’s Rust architecture
- Determine whether `std::fs::OpenOptions` is sufficient or whether Unix-specific lower-level handling is required for flag fidelity
- Keep all helper items private to this module unless another already-migrated module explicitly requires visibility

### Deliverables
- Compilable Rust module file with placeholder implementation
- Confirmed function signature and import list
- Written mapping notes for flags, modes, and return conventions

### Acceptance Criteria
- The crate builds with the new module stub in place
- No extra modules or abstractions are introduced beyond what `orig_open` requires

## Phase 2: Port `orig_open` Logic

### Goals
Translate the actual open behavior from C into Rust while preserving semantics and keeping the implementation narrow.

### Tasks
- Implement pathname handling using borrowed Rust/Unix path types appropriate to the original call shape
- Port open flag processing directly from the C logic
- Port file mode handling for creation paths if the C function uses mode arguments
- Implement the open operation using:
  - `std::fs::OpenOptions` when it can faithfully express the original flags, or
  - a small Unix-specific internal path using standard-library Unix extensions if required
- Translate C-style error handling into `std::io::Result` internally
- Convert the result back into the boundary form expected by the rest of the ported project
- Preserve edge-case behavior from the original function, especially around invalid inputs and open failures

### Deliverables
- Functional Rust implementation of `orig_open`
- Minimal private helper logic for flag or descriptor conversion only if required

### Acceptance Criteria
- The Rust function matches the observed C control flow and return behavior
- No descriptor leaks occur on success or failure paths
- Error cases are mapped consistently with the surrounding project expectations

## Phase 3: Integrate and Validate Behavior

### Goals
Connect the migrated module into the Rust crate and verify that the implementation matches the original behavior for expected inputs.

### Tasks
- Wire `src/module_gnu_open.rs` into the crate module tree
- Add unit tests for:
  - successful open of an existing file where applicable,
  - failure on nonexistent path without create flags,
  - create-mode behavior if supported by `orig_open`,
  - flag-dependent behavior that is visible from the original implementation
- Add tests ensuring returned errors or sentinel values match the intended contract
- Review ownership and cleanup paths for raw descriptors or file handles
- Remove any temporary scaffolding left from the earlier phases

### Deliverables
- Integrated Rust module
- `cargo test` coverage for the migrated function’s main paths

### Acceptance Criteria
- `cargo test` passes
- The module is integrated on branch `045-module_gnu_open.c_39-rust-port`
- The implementation remains limited to the scope of `gnu/open.c` and `orig_open`

## Notes and Constraints

- Prefer `std` over external crates; no third-party dependency is planned from the available evidence.
- Keep the port Unix-oriented if the original module depends on Unix file-descriptor semantics.
- Do not introduce generalized I/O abstraction layers, compatibility wrappers, or new helper subsystems not required by the migrated function.
- Any anonymous C aggregate should remain private and only be materialized in Rust if direct translation makes it necessary.