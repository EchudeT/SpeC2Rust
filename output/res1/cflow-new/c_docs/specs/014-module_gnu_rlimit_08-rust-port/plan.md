# Implementation Plan: module_gnu_rlimit_08

## Summary

Port `gnu/getdtablesize.c` to a Rust module that preserves the existing behavior of retrieving the process file descriptor table size using the platform resource limit interface. The Rust implementation should remain narrowly scoped to the current C file and function, with a direct translation of control flow and return semantics rather than introducing broader abstractions.

The implementation approach is to expose a Rust function corresponding to `getdtablesize`, using standard Rust types where possible and a minimal libc-facing layer only where system calls or constants are required. Error handling should mirror the C behavior closely, including integer conversion boundaries and fallback behavior dictated by the original implementation.

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**:
  - Rust standard library
  - `libc` crate for `getrlimit`, `rlimit`, `RLIMIT_NOFILE`, and related platform constants/types
- **Testing**: `cargo test`
- **Performance Goals**:
  - Single system-call path equivalent to the C implementation
  - No heap allocation
  - Constant-time wrapper overhead
  - Return-path cost effectively identical to the original C code

## Module Mapping

| C Source File | Rust Module/File | Notes |
|---|---|---|
| `gnu/getdtablesize.c` | `src/module_gnu_rlimit_08.rs` | Direct port of the current implementation |
| `getdtablesize` | `pub fn getdtablesize() -> i32` | Preserve C-style integer return shape if used internally in the port |
| `getdtablesize` | same Rust function | Duplicate listing in source analysis maps to the same migrated function |

If the crate already uses a module tree for clustered ports, the file may instead be placed at:

| C Source File | Rust Module/File | Notes |
|---|---|---|
| `gnu/getdtablesize.c` | `src/module_cluster/module_gnu_rlimit_08.rs` | Prefer this only if the existing crate layout already follows that convention |

## Data Model

This module has no named persistent data structures. The only relevant C data shape is the system `rlimit` structure used as a syscall boundary type.

| C Data Structure | Rust Mapping | Notes |
|---|---|---|
| anonymous / system `struct rlimit` usage | `libc::rlimit` | Use directly; do not wrap unless required by existing crate conventions |

### Type Mapping Notes

| C Type/Concept | Rust Type | Notes |
|---|---|---|
| `int` return value | `i32` | Matches C-facing semantics |
| `rlim_t` | `libc::rlim_t` | Avoid narrowing until final return conversion |
| resource limit constant | `libc` constant | Use platform-provided values |

### Memory Management

- Use stack allocation for `libc::rlimit`.
- No owned heap-backed data is needed.
- Any unsafe block should be limited to the syscall invocation and immediately checked.

### Error Handling

- Mirror the C implementation’s return behavior instead of introducing `Result` if the original function returns a plain integer.
- Handle syscall failure explicitly.
- Guard integer conversion from `rlim_t` to `i32` to avoid silent truncation; match the C logic for large values or sentinel values if present in the original source.

## Implementation Phases

### Phase 1: Source Analysis and Rust Module Skeleton

- Inspect `gnu/getdtablesize.c` and confirm:
  - exact syscall/API used
  - failure return value
  - fallback branches
  - conversion behavior for unlimited or oversized limits
- Create the Rust module file corresponding to the selected crate layout.
- Add the public function signature for `getdtablesize` with C-compatible return semantics.

**Deliverable**:
- Compiling Rust module skeleton with placeholder implementation or stub gated to the final function signature.

### Phase 2: Core Function Port

- Translate the C logic for `getdtablesize` directly into Rust.
- Use `libc::getrlimit` and `libc::rlimit` in a minimal unsafe block.
- Implement exact branch behavior for:
  - successful limit retrieval
  - syscall failure
  - return-value conversion to `i32`
- Keep logic local to the function; do not introduce helper layers unless required to express a direct C equivalent.

**Deliverable**:
- Functional Rust implementation of `getdtablesize` matching the original C behavior.

### Phase 3: Tests and Edge Verification

- Add unit tests for deterministic parts of behavior:
  - return value is nonnegative or matches documented failure path under controlled assumptions
  - conversion logic for bounded values, if factored into internal testable code
- Add platform-conditional tests only if needed for portability around `RLIMIT_NOFILE`.
- Verify the module builds and tests pass with `cargo test`.

**Deliverable**:
- Test-covered module port with validated compile/test execution.

### Phase 4: Integration and Final Review

- Wire the new module into the crate’s existing module declarations.
- Confirm no duplicate implementation exists elsewhere in the Rust branch.
- Review unsafe usage for minimal scope and documented assumptions.
- Check naming and file placement against existing project conventions without adding extra compatibility layers.

**Deliverable**:
- Integrated Rust port of `module_gnu_rlimit_08` ready on branch `014-module_gnu_rlimit_08-rust-port`.

## Acceptance Checklist

- `gnu/getdtablesize.c` is mapped to one Rust source file.
- `getdtablesize` is ported once, despite duplicate listing in analysis output.
- The implementation uses only required system bindings and standard Rust structure.
- No new capabilities or module abstractions are introduced.
- Unsafe code is limited to the syscall boundary.
- `cargo test` passes.