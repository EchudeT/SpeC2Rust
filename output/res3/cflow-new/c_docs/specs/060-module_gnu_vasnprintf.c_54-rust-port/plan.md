# Implementation Plan: module_gnu_vasnprintf.c_54

## Summary

This module ports `gnu/vasnprintf.c` into Rust with a narrow migration scope centered on the `MAX_ROOM_NEEDED` logic and any directly related sizing behavior embedded in that source. The Rust implementation should preserve the original sizing semantics while replacing C-style manual buffer reasoning with checked arithmetic and owned buffers from the standard library.

The technical approach is to translate the relevant capacity-calculation logic into a Rust module that uses `usize`, `checked_*` operations, and explicit error propagation for overflow or invalid size growth. The migration should stay file-focused: move the behavior from the C source into a single Rust module without introducing broader formatting infrastructure beyond what is required to represent and validate the original calculation path.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve constant-time size/capacity calculations corresponding to the C macro behavior.
  - Avoid unnecessary heap allocations beyond the final required buffer growth path.
  - Keep overhead limited to Rust bounds/overflow checks, with no extra abstraction layers not required by the source migration.

## Module Mapping

### C to Rust File Mapping

- `gnu/vasnprintf.c` → `src/module_gnu_vasnprintf.rs`

### Responsibility Mapping

- C macro/functionality for `MAX_ROOM_NEEDED` → Rust `const fn` or small private helper function in `src/module_gnu_vasnprintf.rs`
- Any local size-growth or capacity-validation logic tied to this macro in `gnu/vasnprintf.c` → migrated into adjacent private Rust helpers in the same file
- Anonymous C-local data handling patterns → Rust local variables or small private structs only if required by translated control flow

### Public Surface

Keep the Rust module surface minimal. Expose only the items needed by the existing project integration for this port branch. If the original C behavior is internal-only, keep the Rust equivalents non-public except where crate-level access is necessary.

## Data Model

### C to Rust Data Mapping

- `anonymous` → no direct standalone type by default; map to:
  - local variables when the C anonymous structure usage is only temporary/stateful within a function, or
  - a private Rust `struct` if the translated logic requires grouped state across helper calls

### Scalar and Memory Mapping

- C size/count arithmetic (`size_t`, capacity math) → `usize`
- C integer growth/room calculations that may overflow → `usize` with `checked_add`, `checked_mul`, and explicit error returns
- C writable character buffer patterns → `Vec<u8>` or `String`, chosen according to whether the migrated path operates on raw bytes or UTF-8 text assumptions from surrounding Rust code
- C null/error sentinel returns for allocation failure/overflow → `Result<_, ModuleError>` or `Option<_>` internally, with preference for `Result` when the caller needs overflow distinction

### Error Model

Introduce only a minimal module-local error type if needed for overflow/capacity failure reporting, for example:

- overflow during room calculation
- requested capacity exceeds supported bounds

Do not add broader error frameworks or cross-module abstractions unless already required by the existing Rust project structure.

## Implementation Phases

### Phase 1: Source Analysis and Skeleton Port

- Create `src/module_gnu_vasnprintf.rs`.
- Inspect `gnu/vasnprintf.c` and isolate all direct uses and assumptions around `MAX_ROOM_NEEDED`.
- Translate the macro semantics into a Rust helper with matching arithmetic intent.
- Define the smallest necessary Rust types/signatures for the migrated logic.
- Keep all implementation in one module file to mirror the original source scope.

### Phase 2: Arithmetic and Buffer Management Translation

- Port the associated size-growth logic that depends on `MAX_ROOM_NEEDED`.
- Replace C manual capacity handling with Rust checked arithmetic before any allocation or resize.
- Use `Vec<u8>` or `String` according to the original data path, preferring the narrowest faithful representation.
- Convert sentinel-style failure handling into explicit Rust error propagation.
- Ensure no unchecked casts or wraparound behavior remain in the translated code.

### Phase 3: Integration and Behavioral Validation

- Wire the new Rust module into the branch’s existing crate structure with the minimum required visibility.
- Add unit tests covering:
  - normal room calculation cases
  - edge cases near `usize::MAX`
  - overflow rejection behavior
  - any preserved invariants from the original C sizing logic
- Confirm `cargo test` passes and that the module compiles without relying on unsupported C-era assumptions.

### Phase 4: Cleanup and Parity Review

- Review the Rust code against `gnu/vasnprintf.c` to confirm the migrated scope is complete and no unrelated functionality was introduced.
- Simplify helper boundaries only where it improves direct correspondence to the source file.
- Verify documentation/comments are limited to migration-relevant technical notes, especially around overflow and ownership behavior.
- Finalize the module as a constrained port of the original file logic.