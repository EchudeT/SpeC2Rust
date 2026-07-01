# Implementation Plan

## Summary
Port the `quotearg_n_custom_mem` logic from `quotearg.c` into a focused Rust module within the `pwd` project, preserving current behavior and call patterns without expanding scope beyond this function’s existing responsibilities. The Rust implementation should mirror the C routine’s quoting/escaping decisions for a caller-provided byte slice and custom quoting parameters, while replacing manual buffer management with safe `Vec<u8>`/`String`-adjacent ownership patterns as appropriate.

The implementation approach should:
- migrate only the functionality needed for `quotearg_n_custom_mem`;
- keep the code close to the C control flow where practical for reviewability;
- represent input as byte-oriented data to preserve non-UTF-8 behavior;
- make allocation explicit and bounded through standard Rust containers;
- encode C failure paths as Rust `Result` only where the surrounding Rust code requires it, otherwise use infallible return types if the C logic is allocation-only and panic-free under normal Rust allocation semantics.

## Technical Context

### Language/Version
- Rust 1.75+
  Chosen to allow stable, current standard-library facilities without depending on nightly features.

### Primary Dependencies
- Rust standard library only:
  - `std::borrow` if borrowed/owned output views are needed during migration
  - `std::ffi` only if existing surrounding code already exposes C-like string boundaries
  - `std::vec::Vec`
  - `std::string::String`

No third-party crates are recommended from the available evidence, since the scope is a direct migration of one function from `quotearg.c`.

### Testing
- `cargo test`

Testing focus:
- direct unit tests for `quotearg_n_custom_mem` behavior;
- byte-preservation tests for non-UTF-8 inputs;
- edge-case tests for empty input, embedded NUL, and custom quoting delimiters/characters as represented by the C function.

### Performance Goals
- Remain linear in the size of the input buffer.
- Avoid repeated reallocations where output growth can be estimated.
- Preserve byte-oriented processing rather than introducing Unicode normalization or validation.
- Keep runtime characteristics close to the original C implementation for typical short path/string inputs used by `pwd`.

## Module Mapping

### C to Rust File Mapping
- `quotearg.c` -> `src/main_root_quoting_options_02.rs`

If the project already centralizes quoting helpers elsewhere, this function should still be migrated into the smallest existing Rust module that corresponds to this branch’s scope, without creating extra abstraction layers beyond what is needed to house the port.

### Function Mapping
- `quotearg_n_custom_mem` -> `pub(crate) fn quotearg_n_custom_mem(...)`

Recommended Rust-facing shape:
- accept raw bytes (`&[u8]`) for the source buffer;
- accept explicit custom quoting parameters matching the C semantics;
- return owned quoted bytes or a `String` only if the original usage is guaranteed text-safe.

Preferred default:
- `Vec<u8>` output internally and at the API boundary unless the surrounding call sites clearly require UTF-8 text.

## Data Model

The analysis only identifies repeated anonymous C data structures and does not provide field layouts. Because only `quotearg_n_custom_mem` is in scope, the Rust plan should avoid inventing broad replacements and instead introduce the minimum local representations required by this function.

### Data-Structure Mapping
- `anonymous` C structs used only as internal option/grouping carriers
  -> private Rust `struct` definitions with named fields only after field usage is confirmed during porting

- C string/buffer pairs
  -> `&[u8]` for borrowed input
  -> `Vec<u8>` for owned output

- C optional pointers for custom quote characters / delimiters
  -> `Option<u8>`, `Option<&[u8]>`, or small private Rust structs depending on actual call signature

- C integer sizes (`size_t`)
  -> `usize`

- C signed status/error-like integers
  -> `Result<_, _>` only if the translated logic has meaningful recoverable errors in Rust;
     otherwise plain return values with allocation handled by standard containers

### Rust Data Model Guidance
Introduce only the minimal private structures needed to make the translated function readable. For example, if the C function receives or constructs a local options bundle, use a private Rust struct such as:
- `CustomQuoting<'a> { ... }`

This should only be created if the C implementation actually groups related quoting parameters; otherwise, pass parameters directly to avoid unnecessary redesign.

## Implementation Phases

### Phase 1: Source Audit and Signature Freeze
- Inspect `quotearg_n_custom_mem` in `quotearg.c` and identify:
  - exact parameter list;
  - buffer ownership expectations;
  - any dependent constants/macros/local helper usage;
  - whether output is byte-oriented or assumed textual.
- Determine the narrowest Rust signature that preserves current behavior.
- Identify any anonymous C structs actually touched by this function and define only those Rust equivalents required for compilation.

Deliverables:
- Rust module file stub at `src/main_root_quoting_options_02.rs`
- frozen Rust function signature
- private type definitions for directly referenced data only

### Phase 2: Core Function Port
- Translate `quotearg_n_custom_mem` control flow into Rust, keeping branch structure close to the C source.
- Replace manual allocation and pointer arithmetic with:
  - indexed iteration over `&[u8]`;
  - `Vec<u8>` growth for output;
  - explicit push/extend operations for quoted fragments.
- Preserve handling for:
  - empty input;
  - embedded NUL bytes;
  - custom quote markers/escape rules;
  - length-aware input processing rather than sentinel-terminated assumptions.
- Keep helper extraction minimal: only factor out small private helpers if they directly correspond to repeated code from the C function.

Deliverables:
- compiling Rust implementation of `quotearg_n_custom_mem`
- no added APIs beyond what the port requires

### Phase 3: Behavioral Test Coverage
- Add unit tests derived from the C behavior of `quotearg_n_custom_mem`.
- Cover:
  - unchanged bytes that do not require quoting;
  - bytes requiring custom quoting/escaping;
  - empty and single-byte inputs;
  - non-UTF-8 and embedded-NUL cases;
  - delimiter edge cases defined by the original function.
- Verify output length and exact byte content, not only rendered text.

Deliverables:
- `cargo test` passing for all migrated cases
- tests located alongside the module or in the standard Rust test layout already used by the project

### Phase 4: Integration and Cleanup
- Connect the Rust function to the existing `pwd` call path for this branch scope.
- Remove or avoid duplicated temporary translation artifacts created during migration.
- Ensure the final implementation uses only the minimum necessary private types and imports.
- Confirm no extra module expansion, compatibility layers, or speculative helpers remain.

Deliverables:
- integrated Rust port on branch `002-main_root_quoting_options_02-rust-port`
- clean module-level documentation/comments describing only technical constraints and invariants relevant to the port