# Implementation Plan

## Summary

Port `gnu/strerror-override.c` to a focused Rust module that preserves the existing override behavior of `strerror_override` without introducing new facilities. The Rust implementation should model the C function as a small, table-driven lookup over known error numbers and return an optional static message representation. The migration should stay close to the original control flow and file scope, using Rust standard library types and pattern matching to replace C conditionals and pointer-based string handling.

The implementation should prioritize:
- direct migration of the existing function behavior,
- static string data instead of mutable C string storage,
- explicit handling of “no override available” cases,
- minimal surface area consistent with the original module.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Constant-time or near-constant-time lookup for overridden error codes.
  - No heap allocation during normal lookup.
  - Preserve lightweight behavior suitable for frequent error-message access.

## Module Mapping

| C File | Rust Module/File | Notes |
|---|---|---|
| `gnu/strerror-override.c` | `src/gnu/strerror_override.rs` | Direct port of the single-function module. |
| `gnu/strerror-override.c` | `src/gnu/mod.rs` | Re-export or declare the migrated module if the project keeps a `gnu` namespace. |

### Function Mapping

| C Function | Rust Function | Return Strategy |
|---|---|---|
| `strerror_override` | `pub(crate) fn strerror_override(errnum: i32) -> Option<&'static str>` | `None` replaces null-pointer style “no override” result. |

If surrounding project constraints require a C-like signature, keep the internal implementation in safe Rust and isolate any signature adaptation at the call boundary. Do not broaden the API beyond what is needed by the current module users.

## Data Model

This module has no declared C structs or persistent state. The migration is primarily a function and constant-string conversion.

### Data Mapping

| C Representation | Rust Representation | Notes |
|---|---|---|
| integer error code parameter | `i32` | Matches conventional C `int` usage for errno values. |
| `const char *` return | `Option<&'static str>` | Models either a known override string or no override. |
| string literals | `&'static str` | Stored as immutable static string data. |

### Memory Management Notes

- C string literal pointers become Rust string slices with static lifetime.
- No manual allocation or deallocation is needed.
- If downstream code requires C-compatible strings, conversion should be deferred to the boundary that needs it rather than embedded into this module.

### Error Handling Notes

- The original C behavior likely signals “not handled here” via `NULL`; in Rust this should be represented as `None`.
- The function itself should not produce `Result` unless required by a surrounding interface, since lookup failure is part of normal control flow rather than an exceptional condition.

## Implementation Phases

## Phase 1: Establish Module Skeleton and Signature

- Create `src/gnu/strerror_override.rs`.
- Add the Rust equivalent of `strerror_override` with the narrowest useful visibility based on existing call sites.
- Declare the module from `src/gnu/mod.rs` only if the project already uses that layout.
- Choose the core signature as:
  - `fn strerror_override(errnum: i32) -> Option<&'static str>`
- Preserve the original module scope and avoid introducing helper layers unless they directly simplify the single-function migration.

### Deliverables
- Rust module file added.
- Function signature defined.
- Build wiring in place for the module namespace.

## Phase 2: Port Lookup Logic and Static Messages

- Translate the C decision logic directly into Rust `match` or equivalent branch logic.
- Convert each overridden error message from C string literal form into `&'static str`.
- Preserve exact mapping between error numbers and messages from the source module.
- Keep the implementation allocation-free and free of global mutable state.
- Return `None` for all error codes not explicitly overridden.

### Deliverables
- Complete Rust logic for all existing override cases.
- Static message mapping encoded in source.
- No behavioral expansion beyond the C source.

## Phase 3: Integrate Call Sites and Boundary Semantics

- Update existing internal users of `strerror_override` to consume `Option<&'static str>` instead of null-pointer semantics.
- Where old code expected a raw pointer check, replace with idiomatic `if let` / `match`.
- Keep any necessary conversion at the use site minimal and local.
- Confirm that fallback behavior remains outside this module, just as in the C design, if that separation exists today.

### Deliverables
- Internal call sites compiled against the Rust function.
- Null-style checks migrated to option handling.
- Fallback responsibility preserved in the same architectural layer as before.

## Phase 4: Validate Behavior with Focused Tests

- Add unit tests covering:
  - each known overridden error code,
  - at least one unknown error code returning `None`,
  - exact message string equality for the override cases.
- Use straightforward `cargo test` unit tests in the same module or adjacent test module.
- Keep tests scoped to current behavior only; do not add speculative compatibility matrices.

### Deliverables
- Deterministic unit tests for override coverage.
- Passing `cargo test` for the migrated module.

## Acceptance Criteria

- `gnu/strerror-override.c` functionality is fully represented in Rust by a single corresponding module.
- `strerror_override` behavior matches the original mapping and fallback signaling.
- The Rust implementation uses static string data and does not allocate during lookup.
- No additional module capabilities are introduced.
- Tests verify known mappings and unknown-code behavior.