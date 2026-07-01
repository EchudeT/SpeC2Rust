# Implementation Plan

## Summary

Port `fpurge.c` into a Rust module that preserves the existing module boundary and function intent with minimal surface-area change. The Rust implementation should focus on translating the current file-oriented purge behavior into safe Rust code where possible, while keeping compatibility with the surrounding `cat` project structure and avoiding any expansion beyond the existing `fpurge` functionality.

The implementation approach should be:
- migrate the single C source file into a single Rust module;
- represent the `fpurge` operation through a narrow Rust function with explicit error propagation;
- prefer the Rust standard library for file and buffering interactions;
- isolate any platform- or handle-specific behavior behind a small internal function boundary if direct std-only translation is insufficient;
- preserve observable behavior relevant to callers, especially success/failure signaling and buffer state handling expectations.

## Technical Context

- **Language/Version**: Rust 1.77+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Maintain behavior with no unnecessary heap allocation in the purge path.
  - Keep overhead near the original C implementation for normal file-handle usage.
  - Avoid copying buffered contents solely to clear/discard them.
  - Ensure error paths are explicit and cheap.

## Module Mapping

| C File | Rust Module/File | Notes |
|---|---|---|
| `fpurge.c` | `src/fpurge.rs` | Direct migration target for the `fpurge` functionality. |
| function `fpurge` | `pub(crate)` or internal function in `src/fpurge.rs` | Visibility should match actual use within the Rust port of `cat`; do not widen API visibility unless required by existing call sites. |

If the project already centralizes module declarations in `src/lib.rs` or `src/main.rs`, add only the corresponding `mod fpurge;` declaration there.

## Data Model

This module analysis shows no dedicated C structs or custom data structures. The migration should therefore remain function-centered.

| C Concept | Rust Mapping | Notes |
|---|---|---|
| `FILE *` or equivalent stream handle usage inside `fpurge` | Borrowed Rust handle abstraction, chosen to match actual caller context | Use the narrowest practical Rust type. If caller interactions are stream-like but not representable with std-only safe APIs, isolate this in a small internal boundary. |
| integer success/failure return | `Result<(), std::io::Error>` internally, with caller-facing adaptation if needed | Prefer idiomatic Rust error handling; convert to legacy-style status only where integration requires it. |
| libc/global error signaling | `std::io::Error` | Avoid implicit global error state in the Rust-facing implementation. |

## Implementation Phases

### Phase 1: Module Scaffold and Interface Mapping

- Create `src/fpurge.rs`.
- Identify all existing or planned Rust call sites for `fpurge`.
- Define the Rust function signature based on actual caller needs, preferring:
  - borrowed parameters over owned ones;
  - `Result<(), std::io::Error>` for internal implementation;
  - a thin adaptation layer only if the surrounding port expects C-like status codes.
- Add the minimal module declaration in the crate root (`lib.rs` or `main.rs`) without introducing extra helper modules.

**Exit criteria**:
- Rust module exists and compiles as a stub.
- Function signature is fixed and aligned with current project usage.

### Phase 2: Core Translation of Purge Logic

- Port the logic from `fpurge.c` into `src/fpurge.rs`.
- Replace manual C memory/state handling with Rust ownership and borrowing.
- Use standard-library operations where they can express the same buffer-discard behavior.
- If direct translation requires low-level handle interaction not covered cleanly by `std`, keep that interaction confined to the smallest possible internal section and document the safety assumptions.
- Preserve the original success/failure behavior and avoid introducing broader abstractions.

**Key technical points**:
- Do not allocate temporary buffers just to emulate purging.
- Ensure mutable access rules prevent aliasing bugs that would be possible in C.
- Map all failure cases to explicit `io::Error` returns.

**Exit criteria**:
- `fpurge` behavior is implemented.
- No placeholder logic remains.
- Unsafe code, if any, is minimal and justified inline.

### Phase 3: Error Semantics and Integration Cleanup

- Verify that the Rust return path matches how the surrounding `cat` code consumes the result.
- Add any necessary conversion from idiomatic `Result` to project-local status handling, but keep the conversion at the boundary rather than inside the core logic.
- Confirm that file/buffer state after purge matches intended behavior from the C implementation.
- Remove any translation artifacts that are no longer needed after integration.

**Exit criteria**:
- Function integrates cleanly with existing Rust port code.
- Error handling is consistent and explicit.
- Module API is no broader than necessary.

### Phase 4: Tests

- Add focused unit tests in `src/fpurge.rs` or the project’s standard test location.
- Cover:
  - successful purge/discard behavior on supported handle types;
  - repeated calls where relevant;
  - failure propagation for invalid or unsupported states;
  - post-purge state expectations visible to callers.
- Use `cargo test` as the validation path.
- Keep tests limited to the migrated module behavior; do not introduce unrelated infrastructure.

**Exit criteria**:
- Tests pass under `cargo test`.
- Happy-path and error-path coverage exists for the migrated function.