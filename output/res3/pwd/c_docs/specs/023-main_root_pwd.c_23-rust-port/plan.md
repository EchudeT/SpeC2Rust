# Implementation Plan: pwd

## Summary

Port `pwd.c` from the C implementation into an idiomatic Rust module focused on the functionality represented by `usage` and `nth_parent`. The Rust work should stay narrowly aligned with the existing module boundary and behavior, preserving command-line utility semantics rather than redesigning the program structure.

The technical approach is to map the current file into a single Rust source module, use `std::path` and `std::ffi` facilities for pathname handling, and represent fallible operations with `Result` instead of C-style sentinel returns where internal APIs permit. Any externally visible command behavior should remain compatible with the current implementation, with error reporting and exit behavior kept at the command layer. Memory ownership will be handled through Rust’s owned string and path types, eliminating manual allocation and cleanup patterns from the C code.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**:
  - Rust standard library
  - No third-party crates recommended based on the available module evidence
- **Testing**:
  - `cargo test`
  - Unit tests for path-parent computation logic
  - Output-oriented tests for usage text formatting where practical
- **Performance Goals**:
  - Match the existing utility’s expected constant-space/linear-path processing characteristics
  - Avoid unnecessary path cloning in `nth_parent`
  - Keep startup and execution overhead minimal, consistent with a small command-line utility
  - Preserve straightforward single-pass path traversal behavior

## Module Mapping

| C File | Rust File | Notes |
|---|---|---|
| `pwd.c` | `src/bin/pwd.rs` or `src/pwd.rs` | Keep implementation in one Rust file unless the existing project layout already dictates a binary/lib split. Do not introduce extra helper modules beyond what the current Rust project structure requires. |

### Function Mapping

| C Function | Rust Function | Notes |
|---|---|---|
| `usage` | `fn usage(...) -> !` or `fn usage(...)` | Implement as a small command-layer function responsible for printing usage/help text and terminating or returning control according to the surrounding application pattern. |
| `nth_parent` | `fn nth_parent(path: &Path, n: usize) -> Option<PathBuf>` | Use `std::path::Path`/`PathBuf` and parent traversal; preserve original edge-case behavior as closely as possible. |

## Data Model

The analysis reports only anonymous C data structures and does not identify named structs used by this module. The implementation plan should therefore avoid inventing new persistent data models unless required by the existing Rust crate structure.

### Data-Structure Mapping

| C Data Structure | Rust Mapping | Notes |
|---|---|---|
| anonymous | Inline local variables / tuples | Prefer function-local values when the C structure was only incidental to control flow. |
| anonymous | `Path` / `PathBuf` | Use for pathname state instead of raw C string buffers. |
| anonymous | `Option<PathBuf>` | Use for “parent exists / does not exist” outcomes in `nth_parent`. |
| anonymous | `Result<T, E>` | Use for internal fallible operations instead of integer error codes. |
| anonymous | `String` / `OsString` | Use based on whether text must be UTF-8 or platform-native path-compatible. |

### Memory Management and Error Handling

- Replace manual buffer allocation and ownership transfer with Rust-owned values (`PathBuf`, `String`, `OsString`).
- Avoid borrowing path components across mutation boundaries; derive new parent paths as owned values when necessary.
- Convert C null/error return patterns into `Option` or `Result` internally.
- Keep process exit decisions at the top-level command logic rather than embedding them into path-computation helpers.
- If usage output in C writes directly to stdout/stderr, preserve stream choice in Rust with `std::io`.

## Implementation Phases

## Phase 1: Establish File-Level Port Skeleton

- Create the Rust destination for `pwd.c` within the existing crate layout.
- Port the command entry flow only as needed to host `usage` and `nth_parent`.
- Add function signatures for:
  - `usage`
  - `nth_parent`
- Identify current C include-driven dependencies and map them to standard-library imports.
- Preserve current command-line behavior boundaries without introducing new abstractions.

### Deliverables
- Rust file created and compiling with placeholder or partial implementations
- Function signatures aligned with expected callers
- Basic imports and module wiring in place

## Phase 2: Port Core Logic

- Implement `nth_parent` using `Path::parent()` traversal or equivalent component-based logic.
- Validate edge behavior for:
  - zero parent depth
  - root paths
  - paths with fewer ancestors than requested
  - trailing separators as interpreted by Rust path APIs
- Implement `usage` with output text and exit/return pattern matching the surrounding command architecture.
- Replace any C string manipulation with safe Rust path/string handling.

### Deliverables
- Working Rust implementations of `usage` and `nth_parent`
- No manual memory management remaining from the C patterns
- Command-layer error and output handling aligned with existing utility conventions

## Phase 3: Compatibility Review and Cleanup

- Compare Rust behavior against the C module’s existing outputs and path traversal semantics.
- Adjust any mismatches caused by Rust path normalization differences, but only where needed to preserve current behavior.
- Remove transitional code and ensure concise idiomatic ownership handling.
- Confirm that no extra module decomposition or unevidenced helper framework has been added.

### Deliverables
- Behaviorally aligned Rust port of the module
- Clean compile with no unused transitional items
- Finalized function and file mapping matching the C source scope

## Phase 4: Test Coverage

- Add unit tests for `nth_parent` covering representative ancestor traversal cases.
- Add tests for boundary conditions:
  - empty-like path handling if applicable in current callers
  - root-only path
  - ancestor request exceeding available depth
  - unchanged path for zero-depth requests
- Add lightweight verification for `usage` output or invocation path if the project’s test style supports it.
- Run `cargo test` and fix any behavioral regressions.

### Deliverables
- Focused tests for migrated functions
- Passing `cargo test`
- Verified parity for the module’s migrated behavior