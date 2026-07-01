# Implementation Plan

## Summary

This module cluster covers migration of GNU-style attribute-related behavior currently appearing through `gnu/error.c` and `gnu/hash.c`, specifically the compile-time intent expressed by `_GL_ATTRIBUTE_FORMAT_PRINTF_STANDARD` and `_GL_ATTRIBUTE_CONST`. In C, these are macro-based annotations rather than runtime logic, so the Rust port should treat them as semantic constraints on function signatures and implementation style, not as features to re-create directly.

The Rust implementation approach is therefore conservative:

- migrate the affected code from `gnu/error.c` and `gnu/hash.c` into Rust modules with equivalent file-level responsibility,
- replace attribute-macro intent with idiomatic Rust function signatures and validation patterns,
- preserve observable behavior of the migrated functions while omitting C-specific compiler annotation machinery,
- keep ownership and lifetime rules explicit so that memory safety is obtained through native Rust types rather than manual discipline.

Because the analyzed items are attribute macros and anonymous C data forms rather than named exported types, the plan should focus on file migration, function migration order, and how to encode const-/format-like expectations in Rust.

## Technical Context

- **Language/Version**: Rust 1.78 or newer
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Maintain behavior with no meaningful regression versus the original C paths for hash and error-related operations.
  - Avoid unnecessary heap allocation during migrated hot-path hash operations.
  - Preserve constant-time/side-effect-free semantics where C used `_GL_ATTRIBUTE_CONST`.
  - Keep formatted error construction efficient by using standard formatting facilities directly.

## Module Mapping

The Rust port should mirror the original source responsibility without introducing extra architectural layers.

| C File | Rust Target | Notes |
|---|---|---|
| `gnu/error.c` | `src/gnu/error.rs` | Migrate error-related functions and any formatting helpers referenced by this module. Replace printf-format annotations with Rust formatting APIs. |
| `gnu/hash.c` | `src/gnu/hash.rs` | Migrate hash-related functions. Functions marked with const-like attributes should become pure Rust functions taking immutable inputs and returning deterministic outputs. |

Recommended crate module layout:

```text
src/
  lib.rs
  gnu/
    mod.rs
    error.rs
    hash.rs
```

Recommended `mod` wiring:

- `src/lib.rs`: expose `pub mod gnu;`
- `src/gnu/mod.rs`: expose `pub mod error;` and `pub mod hash;`

This keeps the migration traceable back to the original C files and avoids creating extra modules not supported by the input.

## Data Model

The analysis only reports `anonymous` data structures and attribute-like functions/macros, so no stable named C struct inventory is available. The data-model plan should therefore remain minimal and driven by actual usage discovered during porting.

| C Data Form | Rust Mapping | Migration Rule |
|---|---|---|
| anonymous struct/union used only within `gnu/error.c` | private `struct` in `src/gnu/error.rs` or elimination into local variables | Introduce a Rust type only if state must persist across helper calls; otherwise inline into function scope. |
| anonymous struct/union used only within `gnu/hash.c` | private `struct` in `src/gnu/hash.rs` or elimination into local variables | Prefer plain values, slices, and tuples where the C anonymous form only grouped temporary fields. |
| `_GL_ATTRIBUTE_FORMAT_PRINTF_STANDARD` | no direct type mapping | Replace with Rust formatting entry points such as `format!`, `eprintln!`, or typed formatting helpers. Enforce correctness through compile-time format checking. |
| `_GL_ATTRIBUTE_CONST` | pure Rust function | Use `fn` with immutable inputs, no global mutation, and deterministic return values. Mark as `const fn` only if the migrated body actually satisfies Rust const restrictions and this does not force redesign. |

### Data Ownership and Memory Rules

- Convert raw pointer/string usage into `&str`, `&[u8]`, or `Option<&T>` where the original call patterns permit it.
- Use owned `String` or `Vec<u8>` only when the C code actually constructs or stores dynamic content.
- Eliminate manual lifetime management from anonymous C aggregates by using stack-local values and borrow checking.
- If nullability exists in the C call surface, model it explicitly with `Option`.

### Error Handling Rules

- For formatting and reporting logic migrated from `gnu/error.c`, use Rust return types appropriate to the existing behavior:
  - `Result<T, E>` when failure is part of the function contract,
  - direct side-effecting reporting functions when the C code only emits diagnostics.
- Do not reproduce C varargs interfaces directly; replace them with fixed Rust signatures or macro-based formatting at call sites inside the Rust port.
- Avoid panics for normal error paths unless the C code represented programmer misuse rather than runtime failure.

## Implementation Phases

## Phase 1: Establish Rust Module Skeleton and Inventory Actual Call Surfaces

Goals:

- Create the Rust file structure corresponding to `gnu/error.c` and `gnu/hash.c`.
- Inventory the concrete functions in these files that were relying on `_GL_ATTRIBUTE_FORMAT_PRINTF_STANDARD` and `_GL_ATTRIBUTE_CONST`.
- Identify any anonymous structs/unions that must survive as Rust private types.

Tasks:

- Add:
  - `src/lib.rs`
  - `src/gnu/mod.rs`
  - `src/gnu/error.rs`
  - `src/gnu/hash.rs`
- For each migrated C function:
  - record original signature,
  - classify pointer inputs as borrowed, mutable borrowed, owned, or optional,
  - note whether C attribute usage implies pure computation or format-checked output behavior.
- Establish initial Rust signatures without implementing extra behavior.

Deliverables:

- Compiling module skeleton with placeholder items.
- Migration notes embedded as comments or tracked in the porting checklist.
- Clear list of which anonymous C aggregates become Rust private structs and which are eliminated.

## Phase 2: Port `gnu/hash.c` as Pure Computation First

Goals:

- Migrate hash-related logic before error formatting, because `_GL_ATTRIBUTE_CONST`-style functions are typically easier to port and validate.
- Preserve deterministic behavior and input/output equivalence.

Tasks:

- Implement hash functions in `src/gnu/hash.rs`.
- Replace C integer and pointer arithmetic with explicit Rust integer types and slice indexing.
- Where C relied on null-terminated byte traversal, convert to slice- or `&str`-based traversal as appropriate to the original semantics.
- Keep helper functions private unless externally required.
- Ensure const-like functions remain free of hidden state and side effects.

Memory and safety considerations:

- Use checked indexing or iterator-based traversal.
- Avoid `unsafe` unless the original algorithm cannot be expressed otherwise; if unavoidable, isolate it to the smallest possible block and document invariants.

Validation:

- Add unit tests covering known hash inputs, empty input, boundary-sized input, and determinism across repeated calls.

## Phase 3: Port `gnu/error.c` with Typed Formatting Semantics

Goals:

- Migrate error-reporting logic while replacing printf-style attribute machinery with Rust-native formatting.
- Preserve message construction and visible reporting behavior.

Tasks:

- Implement error helpers in `src/gnu/error.rs`.
- Replace varargs-oriented C patterns with:
  - fixed typed helper functions, and/or
  - internal use of Rust formatting macros.
- Map C string parameters to `&str` when valid UTF-8 is guaranteed by usage; otherwise use byte-oriented handling until a safe textual boundary is known.
- Preserve any status-code propagation or side effects from the original functions.

Memory and safety considerations:

- Prefer borrowed text inputs to avoid unnecessary allocation.
- Allocate `String` only for composed diagnostic messages that must be stored or returned.
- Use `Result` only where the original semantics include reportable failure in the function contract.

Validation:

- Add tests for message formatting shape, edge cases around empty inputs, and any status/error code behavior exposed by the migrated functions.

## Phase 4: Integration Cleanup and Behavior Verification

Goals:

- Finish parity checks between the C sources and Rust modules.
- Remove leftover C-specific annotation concepts from the implementation while preserving their semantics.

Tasks:

- Review all migrated functions for:
  - unnecessary mutability,
  - hidden allocation,
  - accidental side effects in const-like helpers,
  - panic paths that should be ordinary error returns.
- Confirm the Rust module exports match the intended visibility of the original C module usage.
- Run `cargo test` and resolve mismatches found during comparison with the original code behavior.

Acceptance criteria:

- `src/gnu/error.rs` and `src/gnu/hash.rs` compile cleanly.
- Tests cover the migrated behavior relevant to formatting and hash determinism.
- No direct recreation of C compiler-attribute macros remains; their intent is captured through Rust type signatures and implementation constraints.
- The port stays limited to the original module responsibilities without adding unrelated facilities.