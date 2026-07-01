# Implementation Plan

## Summary

Port `propername-lite.c` into a Rust module that preserves the current module boundary and behavior of `proper_name_lite` without adding new capabilities. The Rust implementation should follow a direct translation strategy: map the existing string-processing logic into safe Rust using standard library string types and borrowing where possible, while keeping output semantics aligned with the C function.

The implementation should favor:
- a single Rust module corresponding to the source C file,
- minimal API surface centered on the migrated function,
- explicit handling of string ownership and lifetimes,
- test coverage derived from observed behavior and edge cases in the migrated function.

Because this module appears to be a small main-cluster utility with no listed custom data structures, the port should remain lightweight and avoid introducing extra abstraction layers.

## Technical Context

- **Language/Version**: Rust 1.78+ edition 2021
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve linear-time behavior for string scanning/manipulation.
  - Avoid unnecessary allocations beyond what is required to produce the function result.
  - Prefer borrowed string slices internally where practical, converting to owned `String` only for returned values if needed.
  - Match C behavior closely enough that no material regression is introduced in startup-path or utility-path execution.

## Module Mapping

| C Source File | Rust Module/File | Notes |
|---|---|---|
| `propername-lite.c` | `src/propername_lite.rs` | Direct migration target for the module logic. |
| `proper_name_lite` | `proper_name_lite` | Keep the function name in snake_case-compatible Rust form only if required by crate conventions; otherwise preserve recognizable naming with a thin wrapper if needed. |

If this module is called from the crate entry path, expose only the migrated function through the existing main-module wiring in the smallest possible way, such as a `mod propername_lite;` declaration and a direct call site update.

## Data Model

No explicit C structs were identified for this module.

### Data-structure Mapping

| C Representation | Rust Representation | Notes |
|---|---|---|
| `char *` / `const char *` inputs | `&str` | Preferred for read-only string inputs when UTF-8 assumptions are valid in the surrounding port. |
| Returned string data | `String` or `&str` | Choose based on actual C behavior: use `String` if the function constructs or normalizes output; use `&str` only if the result is always a direct input slice. |
| Nullability from C string pointers | `Option<&str>` only if required by call sites | Introduce `Option` only when the C API genuinely allows null. Otherwise require valid `&str`. |

### Memory Management Decisions

- Replace manual C memory handling with Rust ownership and borrowing.
- Keep temporary string processing local to the function scope.
- If the C function conditionally returns either original text or transformed text, prefer a clear Rust implementation that computes the final owned result explicitly rather than emulating pointer aliasing.

### Error Handling Decisions

- Avoid introducing generalized error types unless the C function has clear failure states.
- For invariant-preserving internal logic, use straightforward control flow.
- If invalid input states are possible due to former null-pointer usage, encode them explicitly with `Option` at the boundary rather than unchecked assumptions.

## Implementation Phases

## Phase 1: Establish Module Skeleton and Interface

- Create `src/propername_lite.rs`.
- Add the migrated public function corresponding to `proper_name_lite`.
- Wire the module into the existing crate structure with the minimum required `mod` and `use` changes.
- Define the Rust function signature based on actual call patterns:
  - prefer `&str` inputs,
  - return `String` if transformation/allocation is part of the original behavior,
  - use `Option` only if nullability exists in the C usage.

**Exit criteria**:
- The crate builds with the new module file present.
- The function signature is fixed and integrated at compile time.

## Phase 2: Port Core Logic from C to Safe Rust

- Translate the body of `proper_name_lite` directly from C into idiomatic but behavior-preserving Rust.
- Replace pointer arithmetic and manual buffer handling with:
  - iteration over `chars()` or byte slices as appropriate,
  - slice-based comparisons,
  - bounded string construction via `String`.
- Preserve branch structure and output rules from the C implementation instead of redesigning the algorithm.
- Review any C assumptions around ASCII vs UTF-8 and keep behavior conservative:
  - if logic is byte-oriented in C, consider byte-based processing in Rust to avoid semantic drift,
  - only use Unicode-aware operations if the original behavior clearly supports them.

**Exit criteria**:
- The Rust function fully replaces the C logic in behavior.
- No unsafe Rust is introduced unless a specific C behavior cannot otherwise be represented, which is unlikely for this module.

## Phase 3: Validate Behavior with Focused Tests

- Add unit tests in `src/propername_lite.rs` or `tests/` for the migrated function.
- Cover:
  - typical valid input cases,
  - boundary cases such as empty strings,
  - formatting/name-handling edge cases visible in the C control flow,
  - any nullability-equivalent cases if represented as `Option`.
- Where expected outputs are known from the C behavior, encode them directly as assertions.
- Confirm the function integrates correctly with the main-cluster call path.

**Exit criteria**:
- `cargo test` passes.
- Tests exercise all meaningful branches in the migrated function.

## Phase 4: Cleanup and Final Conformance Review

- Remove any temporary translation scaffolding left from the initial port.
- Confirm that the Rust module does not expose unnecessary helper APIs.
- Recheck ownership decisions to eliminate avoidable clones or allocations.
- Ensure naming, file placement, and visibility match the existing crate conventions without creating extra module layers.
- Perform a final review for parity with the original C behavior, especially around:
  - string slicing safety,
  - allocation boundaries,
  - treatment of edge-case inputs.

**Exit criteria**:
- Final Rust module is minimal, compile-clean, and test-clean.
- The migrated implementation is limited to the existing module/function scope with no speculative extensions.