# Implementation Plan: module_src_set_level_15

## Summary

This module migration covers the level-setting logic currently implemented across `src/main.c` and `src/output.c`, specifically the functions `set_level_indent` and `set_level_mark`. The Rust implementation should preserve the existing control flow and observable behavior while minimizing structural expansion beyond what is required to host these functions safely.

The technical approach is a direct C-to-Rust port:
- move the logic of the two functions into Rust functions with closely matching responsibilities,
- migrate any shared state they read or update into explicit Rust data structures,
- replace implicit C memory and pointer handling with borrowed references, owned `String` data, slices, and `Option` where nullability exists,
- keep module boundaries aligned with the source files being ported rather than introducing additional abstraction layers.

Because the analysis only identifies anonymous C data structures, the Rust design should begin by reconstructing the concrete fields actually touched by `set_level_indent` and `set_level_mark`, then define the smallest corresponding Rust structs needed to support those functions.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Match the C implementation’s asymptotic behavior for level update and output-mark/indent handling.
  - Avoid unnecessary heap allocations beyond what is needed to represent strings safely in Rust.
  - Preserve low-overhead state mutation by using mutable references instead of cloning shared state.
  - Keep formatting and level-setting operations linear in the size of the affected text/state, with no additional passes introduced by the port.

## Module Mapping

### C to Rust file mapping

| C File | Rust File | Migration Scope |
|---|---|---|
| `src/main.c` | `src/main.rs` or `src/lib.rs` + `src/main.rs` | Port call sites and any level-related shared state setup used by `set_level_indent` and `set_level_mark`. |
| `src/output.c` | `src/output.rs` | Port the implementations of `set_level_indent` and `set_level_mark` and keep output-related helpers colocated if they are directly required. |

### Function mapping

| C Function | Rust Function | Notes |
|---|---|---|
| `set_level_indent` | `fn set_level_indent(...)` | Keep signature close to actual usage; convert raw pointer inputs to references/slices and null cases to `Option`. |
| `set_level_mark` | `fn set_level_mark(...)` | Same migration rule; preserve update order and formatting semantics. |

### Rust module layout

Use the smallest conventional structure that supports the migrated files:

```text
src/
  main.rs
  output.rs
```

If these functions need to be tested independently and the crate already supports library-style testing, use:

```text
src/
  lib.rs
  main.rs
  output.rs
```

No extra modules should be introduced unless required by existing crate organization.

## Data Model

The C analysis reports only anonymous structures, so the Rust data model should be derived from actual field access in the two target functions and their immediate callers.

### Mapping strategy

| C Pattern | Rust Mapping | Notes |
|---|---|---|
| Anonymous struct used for shared mutable state | Named `struct` with only accessed fields | Create minimal named structs based on real field usage from the migrated functions. |
| `char *` string data | `String` or `Option<String>` | Use `String` for owned mutable text; `Option<String>` when the C code permits null. |
| Borrowed read-only C strings | `&str` | Use when the Rust function does not need ownership. |
| Mutable character buffers | `String` or `Vec<u8>` | Prefer `String` if the content is text; use `Vec<u8>` only if byte-level mutation is required by the original logic. |
| Integer flags / levels | `i32`, `u32`, `usize`, or `bool` | Choose based on actual arithmetic and indexing behavior seen in C. |
| Nullable pointers to records | `Option<&T>` / `Option<&mut T>` / `Option<Box<T>>` | Select ownership model from actual lifetime semantics in the call graph. |
| Arrays with explicit length | Slices `&[T]` / `&mut [T]` or `Vec<T>` | Prefer slices for borrowed storage, `Vec<T>` for owned resizable storage. |

### Expected Rust structures

Since the source analysis does not expose concrete field names, define only the structures needed by migration, for example:

- a context/state struct for output-level configuration,
- a struct holding indent-related text or counters,
- a struct holding mark-related text or flags.

The exact number of Rust structs should remain minimal and follow these rules:
1. create one named Rust struct per distinct anonymous C record actually touched by the target functions,
2. do not model anonymous C records that are irrelevant to `set_level_indent` and `set_level_mark`,
3. flatten nested pointer-heavy C layouts only when this simplifies safe borrowing without changing behavior.

### Memory management and error handling

- Replace null checks with `Option` and pattern matching.
- Replace manual buffer ownership with Rust-owned strings or vectors.
- Prefer in-place mutation through `&mut` references to reflect the C code’s update semantics.
- If the original functions cannot fail except through invalid pointers, model them as infallible Rust functions after validation is encoded in the type signature.
- If invalid runtime state must still be detected during migration, return a narrow internal error type such as `Result<(), LevelSetError>` only where needed by existing call paths.

## Implementation Phases

## Phase 1: Inventory and signature reconstruction

- Inspect `src/main.c` and `src/output.c` to recover the exact signatures, parameter types, and shared state dependencies of `set_level_indent` and `set_level_mark`.
- Identify every anonymous struct instance and field accessed directly or indirectly by these functions.
- Define the minimal Rust file layout matching the C source split, centered on `src/output.rs` and the existing entry-point file.
- Translate C types to provisional Rust types, focusing on:
  - string ownership,
  - nullable inputs,
  - mutable shared state,
  - integer width used for levels and counters.
- Establish compileable Rust function stubs and struct definitions before implementing logic.

## Phase 2: Data model port and function migration

- Implement named Rust structs for the anonymous C records actually required by the two functions.
- Port `set_level_indent` into `src/output.rs`, preserving:
  - input normalization,
  - level/state mutation order,
  - string or buffer updates,
  - any conditional formatting behavior.
- Port `set_level_mark` with the same directness, avoiding refactors beyond what is needed for borrow checking and safe ownership.
- Update call sites in the Rust equivalent of `src/main.c` so they pass typed references instead of raw pointers.
- Resolve any C global or file-static state used by these functions by moving it into explicit module-local state or function parameters, whichever best matches existing usage without widening scope.

## Phase 3: Behavioral alignment and tests

- Add unit tests around the Rust versions of `set_level_indent` and `set_level_mark` using representative inputs derived from the C behavior.
- Cover:
  - normal level updates,
  - empty or absent mark/indent values,
  - boundary level values actually permitted by the original code,
  - repeated calls that verify state mutation remains consistent.
- Add integration-level checks through the public call path if `main`/top-level output behavior depends on these functions.
- Validate that no unnecessary allocations or copies were introduced in the hot path.

## Phase 4: Cleanup and final parity pass

- Remove temporary compatibility code and unused provisional fields from the Rust structs.
- Tighten function signatures from broad placeholder types to the exact borrowed or owned forms proven by the implementation.
- Review all mutation paths for borrow clarity and ensure there is no remaining C-style sentinel handling that should be expressed as `Option` or `bool`.
- Run `cargo test` and fix any parity issues found during comparison with the C behavior.