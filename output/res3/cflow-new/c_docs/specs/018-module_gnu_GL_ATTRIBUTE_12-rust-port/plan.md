# Implementation Plan

## Summary

This module cluster covers migration of the GNU-originated attribute and utility usage currently appearing in `gnu/error.c` and `gnu/hash.c`, with specific attention to the attribute-style function annotations represented by `_GL_ATTRIBUTE_FORMAT_PRINTF_STANDARD` and `_GL_ATTRIBUTE_CONST`.

The Rust implementation should not reproduce C preprocessor attributes directly. Instead, it should migrate the affected functions into idiomatic Rust signatures and module structure, preserving behavior through:
- normal Rust function declarations,
- compile-time format checking via Rust formatting macros where applicable,
- side-effect-free function design for logic corresponding to `const`-annotated C functions,
- ownership- and borrow-based memory handling instead of manual allocation patterns.

Because the analyzed function list contains macro-like attribute entries rather than concrete behavioral functions, the implementation approach should focus on:
- porting the actual logic in `gnu/error.c` and `gnu/hash.c`,
- removing attribute-macro dependencies during translation,
- keeping the Rust module layout narrowly aligned to the source files being migrated.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve asymptotic behavior of hashing and error-path utility logic from the C sources.
  - Avoid unnecessary heap allocation during hash computation and message formatting.
  - Keep lookup and hash-related operations at comparable cost to the C implementation, subject to Rust safety guarantees.
  - Prefer zero-copy borrowing for string and byte inputs where the C code previously consumed pointers and lengths.

## Module Mapping

### Source-to-target mapping

| C Source File | Rust Target File | Rust Module | Migration Notes |
|---|---|---|---|
| `gnu/error.c` | `src/gnu/error.rs` | `gnu::error` | Port error-reporting helpers and related formatting logic; replace printf-style attributes with Rust formatting boundaries. |
| `gnu/hash.c` | `src/gnu/hash.rs` | `gnu::hash` | Port hash routines and any internal helpers; translate pointer-based input handling to slices/references. |

### Crate/module layout

| Rust File | Purpose |
|---|---|
| `src/lib.rs` | Expose the migrated `gnu` namespace only as needed by the existing project integration. |
| `src/gnu/mod.rs` | Declare `error` and `hash` submodules. |
| `src/gnu/error.rs` | Implementation migrated from `gnu/error.c`. |
| `src/gnu/hash.rs` | Implementation migrated from `gnu/hash.c`. |

### Attribute mapping decisions

| C Attribute/Macro | Rust Handling |
|---|---|
| `_GL_ATTRIBUTE_FORMAT_PRINTF_STANDARD` | Remove macro usage; use Rust formatting macros and typed arguments to obtain compile-time format validation where the call site is migrated into Rust. |
| `_GL_ATTRIBUTE_CONST` | Do not reproduce as an annotation; preserve by keeping functions pure where possible and avoiding hidden mutable global state. |

## Data Model

The analysis reports only anonymous data structures, which indicates that the primary migration work is function- and utility-oriented rather than centered on named public structs. The Rust plan should therefore minimize invented types and only introduce named Rust structures when required to represent persistent internal state from the C files.

### Data-structure mapping strategy

| C Data Shape | Rust Mapping | Notes |
|---|---|---|
| anonymous internal struct used only within one source file | private `struct` in the corresponding Rust module | Introduce only if the C logic requires grouped state. |
| anonymous constant table or static data | `const` or `static` item | Use fixed-size arrays or slices where shape is known. |
| pointer + length input pairs | `&[u8]`, `&str`, or generic borrowed input | Select according to whether the C logic is byte-oriented or text-oriented. |
| mutable output buffers | returned `String`, `Vec<u8>`, or caller-provided mutable reference | Prefer returned owned values unless exact in-place mutation is required by surrounding code. |
| integer status/error codes | `Result<T, E>` for recoverable errors; plain return values where no real error path exists | Keep error modeling minimal and aligned to current call behavior. |

### Expected Rust representations

Since no named C structs are available from the analysis output, the default expectation is:

- **Error-related state**: stateless functions, or at most private helper enums/structs for formatting context.
- **Hash-related state**: pure functions over borrowed input; private helper constants or accumulator variables rather than exported structs.
- **Anonymous structures**: evaluate each occurrence during source translation and convert only when necessary for correctness.

### Memory management decisions

- Replace raw ownership and manual allocation with stack values, `String`, `Vec`, slices, and references.
- Eliminate null-pointer checks by modeling optional inputs with `Option<&T>` only where the original C API truly permitted null.
- Avoid unsafe code unless the source logic cannot be expressed otherwise; any unsafe block must be localized and justified by a direct source construct.

### Error handling decisions

- For error-reporting routines from `gnu/error.c`, separate message construction from process-wide side effects where feasible within current project constraints.
- Use `Result` only for functions that genuinely propagate failure; do not wrap pure hash functions in `Result`.
- Preserve externally visible behavior of error paths without introducing new recovery mechanisms.

## Implementation Phases

### Phase 1: Source inventory and Rust module skeleton

- Create Rust module files matching the two C source files:
  - `src/gnu/error.rs`
  - `src/gnu/hash.rs`
- Add `src/gnu/mod.rs` and wire module exports through `src/lib.rs`.
- Inspect `gnu/error.c` and `gnu/hash.c` to enumerate:
  - actual callable functions,
  - internal helper functions,
  - anonymous structs/tables/constants,
  - dependencies between the two files.
- Identify every use of `_GL_ATTRIBUTE_FORMAT_PRINTF_STANDARD` and `_GL_ATTRIBUTE_CONST` and record the affected functions for signature translation.
- Define the narrowest Rust visibility needed for each migrated item.

### Phase 2: Port `gnu/hash.c`

- Translate hash logic first, because it is likely self-contained and side-effect-light.
- Convert C function signatures from raw pointers/integer lengths into:
  - `&[u8]` for byte hashing,
  - `&str` only if the original semantics are text-based rather than raw-byte-based.
- Preserve integer width semantics explicitly:
  - use `u32`, `u64`, `usize`, or signed types according to the original arithmetic requirements,
  - handle wrapping arithmetic with explicit `wrapping_*` operations where C overflow behavior was relied upon.
- Convert anonymous constants/tables into private Rust `const`/`static` items.
- Add focused unit tests for deterministic hash outputs and edge cases such as empty input, short input, and representative larger input.

### Phase 3: Port `gnu/error.c`

- Translate formatting and reporting helpers into Rust functions in `src/gnu/error.rs`.
- Replace printf-style formatting interfaces with Rust formatting patterns:
  - where call sites move fully into Rust, accept typed arguments or preformatted strings,
  - where a helper only builds messages, prefer `String` assembly using `format!` or `write!`.
- Remove dependence on `_GL_ATTRIBUTE_FORMAT_PRINTF_STANDARD`; rely on Rust compile-time formatting checks at migrated call sites.
- Preserve side-effect expectations such as writing to standard error or constructing error text, but do not add broader logging abstractions.
- If `error.c` depended on process-global C state, map only the minimum necessary state into private Rust module items.

### Phase 4: Integration validation and cleanup

- Reconcile shared types and helper usage between `gnu::error` and `gnu::hash`.
- Ensure all translated functions have stable, minimal signatures aligned with current project call sites.
- Run `cargo test` and add regression tests for:
  - expected error message construction paths,
  - non-panicking behavior on boundary inputs,
  - hash consistency across repeated calls.
- Remove residual C-style patterns that are no longer needed in Rust:
  - sentinel values standing in for `Option`,
  - manual buffer length bookkeeping where ownership already guarantees correctness,
  - attribute-macro placeholders.