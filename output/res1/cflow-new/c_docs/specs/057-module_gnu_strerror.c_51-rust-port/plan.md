# Implementation Plan

## Summary

Port the behavior currently concentrated in `gnu/strerror.c` into an idiomatic Rust module that preserves the existing responsibility: producing the textual representation for an error number. The Rust implementation should stay narrowly aligned with the current C module scope and avoid introducing broader error-framework abstractions.

The technical approach is to migrate the single exported function into a small Rust module that:
- accepts an integer error code compatible with C error-number usage,
- returns a stable string view or owned string as required by the surrounding Rust crate design,
- uses Rust standard-library facilities where they match platform behavior,
- preserves fallback behavior for unknown error codes through explicit formatting logic when no standard message is available.

Because the analyzed module contains one function and no custom data structures, the port should remain compact and focus on API shape, platform-compatible message lookup, and safe string handling without manual memory management.

## Technical Context

- **Language/Version**: Rust 1.76 or newer
- **Primary Dependencies**:
  - Rust standard library
  - No third-party crates recommended based on current module evidence
- **Testing**: `cargo test`
- **Performance Goals**:
  - Constant-time or near-constant-time path for common error-code translation
  - No unnecessary heap allocation when a borrowed/static message can be returned by the chosen API shape
  - Minimal formatting overhead for unknown error codes
  - Behaviorally compatible replacement prioritized over optimization work

## Module Mapping

| C Source File | Rust Module/File | Notes |
|---|---|---|
| `gnu/strerror.c` | `src/module_gnu_strerror.rs` | Direct migration target for the `strerror` logic |
| `strerror` | `module_gnu_strerror::strerror` | Preserve single-responsibility mapping; final Rust signature should fit crate conventions while keeping C-compatible semantics |

If the crate already organizes ports under a different existing file layout, place the implementation in the closest equivalent existing module rather than creating extra wrapper layers.

## Data Model

This module has no identified custom C structs to port.

### Function-Level Type Mapping

| C Type/Concept | Rust Type | Notes |
|---|---|---|
| `int errnum` | `i32` | Direct mapping for POSIX-style error numbers |
| `char *` / `const char *` result semantics | `String` or `Cow<'static, str>` | Prefer the narrowest safe Rust representation that matches surrounding crate interfaces |
| static C string storage | `&'static str` | Use when message text is known and immutable |
| unknown error fallback text | formatted `String` | Used only when no message mapping exists |

### Memory Management Notes

The C implementation likely relies on static storage or implementation-defined string ownership. In Rust, ownership must be explicit:
- prefer `&'static str` for fixed messages,
- use `String` only for synthesized fallback text,
- avoid raw pointers and manual allocation,
- keep lifetimes internal to the module API so callers receive safe Rust string types.

### Error Handling Notes

This port should not introduce a new `Result`-based interface unless the existing Rust crate conventions already require it. The source function’s responsibility is message retrieval, not recoverable control flow. Unknown error codes should therefore map to deterministic fallback text rather than Rust errors.

## Implementation Phases

### Phase 1: Define Rust API and file placement

- Create the target Rust file for the ported module using the project’s existing `src/` layout.
- Add the module declaration in the nearest existing `mod` tree, without creating additional architectural layers.
- Define the Rust function signature for `strerror` based on expected crate usage:
  - prefer `fn strerror(errnum: i32) -> Cow<'static, str>` if mixed borrowed/owned output is needed,
  - otherwise use `fn strerror(errnum: i32) -> String` if the crate already standardizes on owned strings.
- Document the intended compatibility constraints in code comments:
  - integer error-code input,
  - stable human-readable output,
  - fallback for unmapped codes.

### Phase 2: Port message resolution logic

- Translate the existing C function body into Rust control flow.
- Use standard-library or platform-neutral message lookup only if it can satisfy the observed module behavior without adding dependencies.
- Preserve the current module’s fallback behavior for unknown error numbers through explicit formatting logic.
- Ensure all string construction is safe and ownership-aware:
  - borrowed string for known fixed messages where possible,
  - owned formatted string for fallback cases.
- Keep implementation local to this module; do not expand into shared registries or generalized error subsystems.

### Phase 3: Validate edge cases and compatibility behavior

- Add unit tests covering:
  - representative known error numbers,
  - zero or implementation-relevant neutral codes if applicable,
  - negative inputs if the C logic permits them,
  - unknown large values producing fallback text.
- Verify that output is deterministic for the selected Rust implementation path.
- Confirm absence of unsafe memory handling and eliminate any need for mutable static storage present in the C idiom.

### Phase 4: Integrate and clean up

- Wire the new Rust module into the branch’s build so it replaces the C module responsibility for this unit.
- Remove any temporary scaffolding introduced during migration.
- Run `cargo test` and address signature or ownership mismatches at call sites strictly as required by this module port.
- Keep final scope limited to the migrated `strerror` behavior only.