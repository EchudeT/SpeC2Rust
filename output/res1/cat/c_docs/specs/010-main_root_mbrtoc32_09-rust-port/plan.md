# Implementation Plan: main_root_mbrtoc32_09

## Summary

This module ports the C source file `mbrtoc32.c` into an idiomatic Rust implementation that preserves the original conversion behavior and stateful semantics of `mbrtoc32`. The scope is limited to migrating the existing function logic into the Rust codebase for the `cat` project branch `010-main_root_mbrtoc32_09-rust-port`.

The Rust implementation should focus on:

- reproducing the byte-sequence to UTF-32 scalar conversion flow represented by `mbrtoc32`
- preserving incremental/stateful decoding behavior across calls
- mapping C-style return signaling and state mutation into explicit Rust types and controlled mutable state
- keeping the implementation small and localized, without introducing extra abstraction layers beyond what is needed to represent the original function and its conversion state

The preferred technical approach is to implement the port using the Rust standard library only, with a dedicated module corresponding directly to the original C file. Memory management should rely on Rust ownership and borrowing, while error and partial-sequence handling should be represented explicitly rather than through unchecked pointer operations.

## Technical Context

### Language/Version

- Rust stable, edition 2021
- Minimum recommended compiler: Rust 1.74 or newer

### Primary Dependencies

- Rust standard library only

Recommended standard facilities:

- `core::char` / `std::char` for Unicode scalar validation where needed
- `core::result::Result` for explicit success/error handling
- `Option` for nullable C pointer/state equivalents
- fixed-width integer types from `core::ffi` / `std::ffi` compatible primitives as needed for C-like semantics

No third-party crates are recommended because the input provides no evidence requiring external Unicode, locale, or FFI support.

### Testing

- `cargo test`

Testing should cover:

- valid single-byte and multibyte decoding cases
- incomplete multibyte sequences across repeated calls
- invalid byte sequence handling
- state reset/initial-state behavior
- end-of-input and null-destination style cases if the original function semantics require them

### Performance Goals

- match the original C module’s asymptotic behavior
- avoid heap allocation in the decoding path
- process input using direct byte-slice indexing and compact state updates
- keep per-call overhead minimal and suitable for repeated decoding in streaming scenarios

## Module Mapping

### C to Rust File Mapping

- `mbrtoc32.c` -> `src/main_root_mbrtoc32_09.rs`

If the project already groups main-cluster ports under an existing module tree, use the nearest existing standard Rust location and keep the file-to-module mapping one-to-one.

### Function Mapping

Because the C analysis lists `mbrtoc32` twice, treat this as a duplicate symbol entry rather than distinct functions.

- C `mbrtoc32(...)` -> Rust `pub(crate) fn mbrtoc32(...)`

Implementation guidance:

- preserve a single exported Rust function for this module
- if the C logic uses internal helpers, introduce only private helper functions inside the same Rust module
- do not split behavior into additional public modules or utility crates

### API Shape Notes

C patterns likely involved:

- destination output pointer
- input pointer plus byte count
- mutable conversion state (`mbstate_t`)
- integer/status return value with sentinel meanings

Rust translation should use:

- `Option<&mut ...>` for nullable output/state arguments where needed
- `&[u8]` for input buffer plus explicit length handling
- a small dedicated state struct instead of raw mutable memory
- a return type that preserves the original status space clearly, likely via a compact enum or result-like type

## Data Model

No explicit C structs were identified in the analysis output. For this module, the main data-model work is mapping implicit C state and pointer-based outputs into Rust types.

### Data-Structure Mapping

- C implicit `char32_t` output -> Rust `u32` or `char`
- C implicit `mbstate_t` conversion state -> Rust `struct MbState` (module-local if not shared elsewhere)
- C nullable output pointer -> Rust `Option<&mut u32>` or `Option<&mut char>`
- C byte pointer and length -> Rust `&[u8]`
- C `size_t`/status return codes -> Rust enum representing:
  - completed conversion with consumed byte count
  - incomplete sequence
  - invalid sequence
  - special reset/no-output cases if present in the original behavior

### Proposed Rust Types

```rust
pub(crate) struct MbState {
    pending: [u8; 4],
    pending_len: u8,
    expected_len: u8,
}
```

This should remain minimal and only store the information required by the original function’s incremental decoding logic. If the C logic shows a different state representation during implementation, the Rust struct should be adjusted to match it more closely rather than generalized.

A possible result representation:

```rust
pub(crate) enum MbrToC32Result {
    Complete { value: u32, consumed: usize },
    Incomplete,
    Invalid,
    Reset,
}
```

If integration constraints require a C-like numeric return convention instead, keep the external signature numeric and use a private enum internally to structure the port.

### Memory Management Notes

- replace raw pointer writes with bounded mutable references
- keep state owned by the caller or passed as `&mut MbState`
- avoid unsafe code unless exact compatibility requirements make it unavoidable
- if unsafe is required for signature compatibility, isolate it at the boundary and keep decoding logic safe

## Implementation Phases

## Phase 1: Establish module skeleton and state representation

- Create the Rust module corresponding to `mbrtoc32.c`.
- Define the Rust-facing `mbrtoc32` function signature based on how the surrounding project expects to call it.
- Introduce the minimal conversion-state type needed to replace C `mbstate_t` usage for this module.
- Define internal result/status representations that can express the original C outcomes without ambiguity.
- Add placeholder unit tests for the expected categories of behavior.

### Deliverables

- `src/main_root_mbrtoc32_09.rs`
- `mbrtoc32` signature finalized
- minimal `MbState` and internal status types compiled

## Phase 2: Port decoding logic from C to Rust

- Translate the byte-consumption and codepoint assembly logic from `mbrtoc32.c` directly into Rust.
- Preserve partial-sequence handling across calls using mutable state.
- Implement invalid-sequence detection and state transitions carefully.
- Map output writing from C pointer semantics to safe Rust mutation.
- Keep helper functions private and local to the module.

### Deliverables

- complete Rust port of `mbrtoc32`
- no unchecked buffer access
- explicit handling for complete, incomplete, and invalid input paths

## Phase 3: Align return semantics and edge-case behavior

- Verify that the Rust return behavior matches the original C function’s sentinel meanings and edge cases.
- Implement any required reset behavior for null or empty-input cases according to the original file logic.
- Confirm whether output should be `u32` or validated `char`, and preserve exact original semantics if non-scalar values or sentinel values are possible.
- Review state mutation on both success and failure paths to ensure parity with C behavior.

### Deliverables

- edge-case-compatible return mapping
- state behavior aligned with the C implementation
- final signature and internal semantics stabilized

## Phase 4: Complete tests and integration verification

- Add unit tests derived from the ported logic and known conversion categories.
- Test repeated-call scenarios to verify stateful decoding.
- Test invalid and truncated sequences to confirm correct error signaling and state handling.
- Run `cargo test` and resolve any integration mismatches with the surrounding `cat` Rust project layout.

### Deliverables

- passing unit tests for valid, partial, and invalid paths
- module integrated into the target branch structure
- implementation ready for review

## Notes and Constraints

- Keep the migration limited to the behavior contained in `mbrtoc32.c`.
- Do not add broader Unicode conversion facilities or shared framework code unless required by existing project structure.
- Prefer safe Rust; if exact external compatibility requires low-level handling, confine unsafe operations to thin boundaries.
- Preserve the original control flow and state semantics closely to reduce behavioral drift during the port.