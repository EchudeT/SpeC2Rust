# Implementation Plan: module_gnu_GL_ATTRIBUTE_12

## Summary

This module cluster covers attribute-related functionality referenced from `gnu/error.c` and `gnu/hash.c`, specifically the C macro/function-like symbols:

- `_GL_ATTRIBUTE_FORMAT_PRINTF_STANDARD`
- `_GL_ATTRIBUTE_CONST`

For the Rust port, the implementation approach is to treat these GNU C attributes as portability/compiler-hint constructs rather than runtime features. The Rust migration should therefore focus on:

- preserving the behavior of the surrounding migrated functions in `gnu/error.c` and `gnu/hash.c`,
- replacing C attribute usage with Rust-native equivalents where applicable,
- omitting no-op compiler attributes when they have no direct semantic effect in Rust,
- keeping module boundaries aligned with the original files.

This is a narrow migration plan: port only the affected code paths and signatures in the existing file scope, without introducing additional abstraction layers or new capability beyond what is needed to preserve current behavior.

## Technical Context

- **Language/Version:** Rust 1.78+
- **Primary Dependencies:** Rust standard library only
- **Testing:** `cargo test`
- **Performance Goals:**
  - Maintain behavior equivalent to the C implementation with no avoidable heap allocation added by the migration.
  - Preserve constant-time property of pure/const-style helper functions where the original code relied on compiler attributes for optimization hints.
  - Keep hashing and error-path overhead comparable to direct standard-library implementations.
  - Prefer zero-copy borrowing (`&str`, `&[u8]`) over owned allocations where the C code used borrowed pointers.

## Module Mapping

| C File | Rust Module/File | Migration Notes |
|---|---|---|
| `gnu/error.c` | `src/gnu/error.rs` | Port only the functions/signatures impacted by GNU attribute macros in this file; replace attribute-driven contracts with Rust type/signature choices and standard formatting macros. |
| `gnu/hash.c` | `src/gnu/hash.rs` | Port only the affected helper functions and hashing-related logic; map const/pure-style attribute intent to idiomatic Rust functions. |

### Symbol-Level Mapping

| C Symbol | Rust Handling |
|---|---|
| `_GL_ATTRIBUTE_FORMAT_PRINTF_STANDARD` | Replace with Rust compile-time checked formatting at call sites (`format!`, `eprintln!`, `write!`, etc.) or ordinary function signatures that accept formatted data rather than varargs. No direct macro port. |
| `_GL_ATTRIBUTE_CONST` | Map to ordinary Rust functions with no interior mutation; mark `const fn` only if the migrated body is actually valid under Rust `const` rules and needed by the implementation. Otherwise preserve purity by API design and documentation in code comments. |
| `_GL_ATTRIBUTE_CONST` | Same handling as above; deduplicate during migration review if both occurrences refer to the same macro usage pattern. |

## Data Model

The analysis reports only anonymous data structures and does not identify named C structs in this module slice. The migration plan should therefore avoid inventing new model types unless required by the existing code being ported.

### Data-Structure Mapping

| C Data Structure | Rust Mapping | Notes |
|---|---|---|
| anonymous | Inline local variables / tuples / small private structs only if required during function port | Prefer direct translation into function-local state. |
| anonymous | Inline local variables / tuples / small private structs only if required during function port | Do not promote to public types. |
| anonymous | Inline local variables / tuples / small private structs only if required during function port | Keep scope minimal. |
| anonymous | Inline local variables / tuples / small private structs only if required during function port | |

### Memory Management and Error Handling Decisions

- Replace raw C pointers used only for borrowed string/data access with `&str`, `&[u8]`, or `Option<&T>` as appropriate.
- Use owned Rust types (`String`, `Vec<T>`) only where the original C implementation required mutable or dynamically sized storage.
- Eliminate manual lifetime and deallocation concerns by keeping ownership explicit at function boundaries.
- Replace C error-reporting conventions with:
  - `Result<T, E>` for fallible internal operations,
  - direct stderr output where the original `error.c` behavior is immediate reporting,
  - narrow error enums only if needed by already-existing function behavior in the migrated file.
- Avoid `unsafe` unless exact pointer-based behavior from the original file cannot be expressed safely; if needed, confine it to the smallest possible scope and document the invariants.

## Implementation Phases

## Phase 1: Inventory and File Scaffolding

- Create Rust file targets matching the original layout:
  - `src/gnu/error.rs`
  - `src/gnu/hash.rs`
- Identify every usage site of:
  - `_GL_ATTRIBUTE_FORMAT_PRINTF_STANDARD`
  - `_GL_ATTRIBUTE_CONST`
- Classify each usage as one of:
  - formatting contract only,
  - pure/const optimization hint only,
  - signature-affecting wrapper that needs a Rust API adjustment.
- Establish minimal module exports so existing migrated callers can reference the ported functions without adding new facade modules.

## Phase 2: Port `gnu/error.c`

- Migrate the affected `error.c` functions first because format-related GNU attributes usually indicate varargs or diagnostics semantics.
- Replace C printf-style contracts with Rust formatting mechanisms:
  - prefer concrete formatted output operations,
  - if helper functions are needed, accept already-formed strings or `std::fmt::Arguments<'_>` rather than recreating C varargs behavior.
- Preserve observable behavior of message emission order, destination, and return behavior.
- Remove attribute macros that have no Rust equivalent at runtime.
- Add unit tests covering:
  - expected message formatting shape,
  - fallible paths if present,
  - stable behavior for edge-case inputs migrated from the C implementation.

## Phase 3: Port `gnu/hash.c`

- Migrate the hashing-related functions in `hash.c` with signatures that reflect borrowed data rather than raw pointers.
- For `_GL_ATTRIBUTE_CONST`-annotated helpers:
  - implement as ordinary side-effect-free Rust functions,
  - use `const fn` only when the translated logic is valid in const context and this does not complicate the port.
- Preserve original hashing semantics, including integer width behavior, byte handling, and overflow behavior, by using explicit integer types and wrapping arithmetic where needed.
- Add unit tests for:
  - representative input/output stability,
  - empty input cases,
  - boundary-size or boundary-value cases present in the C logic.

## Phase 4: Integration Validation and Cleanup

- Run `cargo test` and resolve mismatches in signatures or behavior between the two migrated files.
- Remove any leftover placeholder mappings for GNU attribute macros once all call sites use Rust-native constructs.
- Confirm that the final Rust modules:
  - do not expose unnecessary public types,
  - do not allocate where borrowed data is sufficient,
  - do not retain dead compatibility shims for C attributes.
- Keep comments limited to migration-specific invariants, especially where C compiler attributes were dropped because Rust already enforces or renders them unnecessary.