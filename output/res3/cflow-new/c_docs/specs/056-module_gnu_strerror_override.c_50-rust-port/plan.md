# Implementation Plan

## Summary

Port `gnu/strerror-override.c` into a focused Rust module that preserves the existing behavior of `strerror_override` without adding new API surface or auxiliary facilities. The Rust implementation should mirror the C logic as closely as practical, using standard-library string types and explicit matching over known error codes or cases handled by the original function.

The implementation approach is to migrate the single C source file into a single Rust module with one public function corresponding to `strerror_override`. Because the original module is a narrow override layer around error-string resolution, the Rust version should emphasize:
- direct translation of condition checks and return cases,
- borrowed string output where possible,
- explicit handling of platform-sensitive integer error codes,
- no dynamic memory management beyond standard Rust string references unless required by the translated logic.

## Technical Context

- **Language/Version**: Rust 1.78 or current stable compatible with the repository toolchain
- **Primary Dependencies**:
  - Rust standard library
  - No third-party crates recommended based on the provided module scope
- **Testing**:
  - `cargo test`
- **Performance Goals**:
  - Constant-time or equivalent branch-based lookup consistent with the original C function
  - No unnecessary heap allocation for fixed override messages
  - Behaviorally equivalent error-text selection for supported inputs
  - Minimal overhead relative to the C implementation

## Module Mapping

### C to Rust File Mapping

- `gnu/strerror-override.c` → `src/module_gnu_strerror_override.rs`

If the repository already uses a module tree, keep placement aligned with the existing crate layout and expose only the migrated function needed by current callers.

### Function Mapping

- `strerror_override` → `pub(crate) fn strerror_override(...) -> Option<&'static str>` or equivalent Rust signature based on the original C contract

### Signature Migration Notes

Because the exact C signature is not included in the input, finalize the Rust signature by preserving the existing call pattern from the source:
- If the C function returns `const char *` and uses `NULL` for "no override", map to `Option<&'static str>`.
- If surrounding code requires a raw string-like return form, contain that adaptation at the call boundary and keep the internal function idiomatic.
- If the input is an integer error code, use a Rust integer type matching the C source usage, typically `i32` or a type alias if the crate already defines one.

## Data Model

This module appears to define no standalone structs or persistent state.

### Data Structure Mapping

- No C structs/enums identified → no Rust struct/enum required

### Value Mapping

- C error code integers → Rust integer primitive (`i32` unless source usage requires another exact width)
- C string literals (`const char *`) → `&'static str`
- C null-return sentinel → `Option::None`

### Memory Management Notes

- Fixed override messages should be represented as string literals, avoiding allocation.
- No manual memory ownership model is needed.
- Avoid constructing `String` unless required by an existing Rust-facing API outside this module.

### Error Handling Notes

- This function should not introduce a `Result` unless the original behavior includes fallible operations beyond simple lookup.
- Preserve the original "override exists / override does not exist" distinction through `Option` or an exact equivalent adapted to caller expectations.

## Implementation Phases

## Phase 1: Source Analysis and API Lock-In

- Inspect `gnu/strerror-override.c` and identify:
  - exact `strerror_override` signature,
  - all handled error-code cases,
  - return behavior for unmatched inputs,
  - any conditional compilation or platform-specific branches.
- Inspect current call sites to determine the narrowest compatible Rust signature.
- Define the Rust module file and function visibility to match existing project usage.
- Record any integer type requirements needed for exact behavior preservation.

**Deliverable**:
- Finalized Rust function signature and module placement ready for direct implementation.

## Phase 2: Direct Function Port

- Create `src/module_gnu_strerror_override.rs`.
- Implement `strerror_override` as a direct translation of the C decision logic.
- Replace C string-literal returns with `&'static str`.
- Replace `NULL`/sentinel behavior with `Option`.
- Preserve compile-time conditional behavior only if present in the original file and required for correctness on supported targets.
- Keep logic local to the module; do not introduce helper abstractions unless needed to express an exact C branch cleanly.

**Deliverable**:
- Compiling Rust module containing the migrated function with behavior matching the original file.

## Phase 3: Caller Integration and Type Adaptation

- Update internal callers to use the Rust module and the Rust return type.
- If callers expect C-like nullability, adapt at the immediate boundary rather than weakening the Rust function design.
- Confirm that unmatched error codes flow through exactly as before.
- Remove or isolate any temporary compatibility code created during migration.

**Deliverable**:
- Integrated Rust path replacing the C module behavior for all existing call sites.

## Phase 4: Targeted Tests and Behavioral Verification

- Add unit tests covering:
  - each explicitly overridden error code/message pair,
  - unmatched input returning no override,
  - boundary or platform-conditioned cases present in the original source.
- Validate that no heap allocation is required for fixed outputs.
- Run `cargo test` and confirm behavior consistency with the translated C logic.

**Deliverable**:
- Passing tests demonstrating parity for the migrated function.