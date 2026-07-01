# Implementation Plan: module_gnu_fcntl.c_27

## Summary

This module ports `gnu/fcntl.c` into Rust with a narrow scope centered on the `dupfd` function. The Rust implementation should preserve the existing behavior of duplicating or validating file descriptors while translating C-style return/error conventions into idiomatic Rust internals and stable OS-facing results.

The technical approach is to migrate the single C source file into one Rust module with minimal surrounding structure. Because the functionality is OS/file-descriptor oriented and not represented by safe standard-library abstractions alone, the implementation should use direct Unix file-descriptor operations through Rust’s standard library OS types and, where needed, a small libc-compatible crate for constants or system calls. Memory management requirements are minimal because the analyzed module exposes no named complex data structures; the main migration concern is correct integer handling, syscall interaction, and preservation of error semantics.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**:
  - Rust standard library
  - `libc` crate, only if required for `fcntl`/descriptor constants or direct syscall access not exposed cleanly by `std`
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve near-system-call-level overhead for descriptor duplication paths
  - Avoid heap allocation in the migrated logic
  - Keep conversion and validation logic constant-time and minimal
  - Match the C implementation’s operational cost profile closely

## Module Mapping

- **C source file**: `gnu/fcntl.c`
- **Rust target module**: `src/gnu/fcntl.rs`

Suggested crate-local layout:

- `src/gnu/mod.rs`
  - exposes `fcntl`
- `src/gnu/fcntl.rs`
  - contains the Rust port of `dupfd`

Function mapping:

- `dupfd` → `pub(crate)` or private Rust function in `src/gnu/fcntl.rs`, with visibility chosen only to satisfy existing crate call sites

The migration should remain file-for-file in intent: one C implementation unit becomes one Rust module, without introducing extra abstraction layers beyond what is necessary for safe syscall wrapping and error translation.

## Data Model

The analysis identifies only an **anonymous** data structure and no named persistent struct that must be modeled directly.

Data mapping guidance:

- **Anonymous C data / temporary locals** → Rust local bindings and primitive types
- **C file descriptor integers (`int`)** → `std::os::fd::RawFd` or `libc::c_int`, depending on the syscall boundary
- **C error returns (`-1` with `errno`)** → internal `std::io::Result<T>` during implementation, with final outward shape adapted to surrounding project expectations
- **C integral flags/constants** → Rust integer constants from `libc` if not available through `std`

No standalone Rust struct or enum should be introduced unless a small private enum is required purely to encode a local branching condition from the original C logic.

## Implementation Phases

### Phase 1: Module Skeleton and Interface Mapping

- Create `src/gnu/fcntl.rs` and wire it into the crate module tree.
- Identify the exact call signature expected for `dupfd` from the C source and surrounding usage.
- Choose Rust parameter and return types that preserve descriptor-level behavior with minimal translation:
  - use `RawFd`/`c_int` at syscall boundaries
  - use `io::Result` internally if helpful for error propagation
- Record the required Unix-specific imports and gate usage appropriately if the crate already follows platform-specific compilation patterns.

### Phase 2: Core Function Port

- Port the body of `dupfd` directly from `gnu/fcntl.c` into Rust, preserving:
  - validation order
  - flag/constant handling
  - descriptor duplication semantics
  - exact failure conditions as closely as Rust permits
- Use standard library facilities first; if the needed `fcntl` duplication behavior is not directly available, call the underlying OS API through `libc`.
- Translate C integer/error handling carefully:
  - preserve `-1`-style syscall failure detection at the boundary
  - convert `errno` into `io::Error` during internal handling
  - convert back to the project-required outward form only if necessary
- Keep the implementation allocation-free and avoid introducing helper layers unless they directly support parity with the original function.

### Phase 3: Error Semantics and Edge-Case Alignment

- Verify behavior for invalid file descriptors, boundary descriptor values, and duplication target constraints reflected in the original C logic.
- Check integer conversions explicitly to avoid accidental sign or width issues between Rust and C types.
- Confirm that any use of unsafe code is limited to the syscall boundary and documented inline with the exact preconditions being upheld.
- Ensure resource ownership is unchanged from the C implementation:
  - duplicated descriptors remain raw descriptors
  - no accidental `File` ownership wrappers that would close descriptors on drop

### Phase 4: Tests and Final Integration

- Add focused unit tests in the Rust module or adjacent test module covering:
  - successful duplication of a valid descriptor
  - failure on invalid descriptor input
  - behavior around minimum requested duplicated descriptor, if applicable to the original logic
- Use `cargo test` to validate the migrated module.
- Confirm the new Rust file fully replaces the C module’s functional role in the branch scope, without adding unrelated utilities or APIs.