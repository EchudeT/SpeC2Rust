# Implementation Plan: main_root_quotearg_n_08

## Summary

This module ports the `quotearg.c` entry-point subset centered on indexed quoting helpers:

- `quotearg_n`
- `quotearg_n_mem`
- `quotearg_n_custom`

The Rust implementation should preserve the existing module boundary and migrate only the functionality needed for these functions. The technical approach is to translate the C routines into a single Rust source module that keeps the same operational model: accept an argument index plus input data and return a quoted representation while handling ownership safely through Rust `String`/`Vec<u8>` and borrowing via `&str`/`&[u8]` where appropriate.

Because this module originates from a shared quoting implementation, the Rust port should avoid introducing new abstractions beyond what is needed to represent the existing option/state data used by these three functions. Any static or per-slot storage behavior present in the C code should be mapped directly to Rust structures with explicit ownership and bounded mutation, favoring simple internal vectors over C-style manual allocation. Error handling should follow Rust conventions for invalid UTF-8 or allocation-sensitive paths, while preserving behavior required by callers by keeping public function signatures as close as practical to the project’s existing Rust side.

## Technical Context

### Language / Version

- Rust 1.75 or newer

### Primary Dependencies

- Rust standard library only:
  - `std::borrow`
  - `std::string::String`
  - `std::vec::Vec`
  - `std::sync` only if required to mirror global slot storage already present in the C logic

No third-party crates are recommended because the input does not provide evidence of external dependency requirements, and the module can be implemented with the standard library.

### Testing

- `cargo test`

Testing should cover:
- stable output for each migrated function
- indexed slot reuse behavior, if present in the C implementation
- byte-slice handling for `_mem` variants
- custom quoting delimiter behavior for `_custom`
- boundary conditions such as empty inputs and larger slot indices

### Performance Goals

- Preserve near-linear processing cost with respect to input length
- Avoid unnecessary intermediate allocations during quoting
- Reuse per-index storage where the C implementation reuses buffers
- Keep byte-oriented handling efficient for `quotearg_n_mem`, especially when input is not valid UTF-8

## Module Mapping

### C to Rust File Mapping

- `quotearg.c` -> `src/quotearg.rs`

If the current Rust crate already places equivalent logic elsewhere, migrate these three functions into the existing quoting module rather than introducing a second module. The goal is one Rust module corresponding to the original C implementation unit.

### Function Mapping

- `quotearg_n` -> `pub(crate)` or `pub` Rust function in `src/quotearg.rs`
- `quotearg_n_mem` -> `pub(crate)` or `pub` Rust function in `src/quotearg.rs`
- `quotearg_n_custom` -> `pub(crate)` or `pub` Rust function in `src/quotearg.rs`

Recommended migration order:

1. `quotearg_n_mem`
2. `quotearg_n`
3. `quotearg_n_custom`

This order keeps the byte-length-aware primitive first, then layers string-oriented and custom-delimiter entry points on top of it where possible.

## Data Model

The analysis only reports anonymous C data structures, which is typical for internal option/state structs and tables embedded in `quotearg.c`. The Rust plan should therefore map by role rather than by exact original tag name.

### Data-Structure Mapping

- anonymous state/options struct(s) -> named Rust `struct` types in `src/quotearg.rs`
- anonymous flag/config groups -> Rust `struct` or compact `enum` types
- anonymous static slot array/buffer records -> Rust `Vec<Slot>` or `Vec<Option<Slot>>`
- anonymous quoting style constants -> Rust `enum`
- anonymous delimiter/custom quote holders -> Rust `struct` with borrowed or owned byte/string fields as needed

### Recommended Rust Representations

#### Quoting option/state record
Map any C struct carrying quoting parameters into a named Rust struct, for example:

- `QuotingOptions`
  - style field as enum
  - optional left/right quote delimiters
  - flags as integer or small booleans, depending on what the C fields actually contain

Use explicit field names derived from the source during implementation. Do not generalize beyond fields touched by the three target functions.

#### Slot storage
If the C code uses `quotearg_n` slot-based persistent buffers, represent them as:

- `struct QuotedSlot { buf: Vec<u8> }`
- container: `Vec<QuotedSlot>`

If returned data is always UTF-8 in practice at this layer, `String` can be used; otherwise prefer `Vec<u8>` internally and convert at the public boundary only where required.

#### Custom delimiter data
Map custom quote pairs to a small struct:

- `struct CustomQuoting { left: Vec<u8>, right: Vec<u8> }`

If delimiters are always single-byte or valid UTF-8 strings, use `String` or `char` only when directly justified by the C parameter types.

### Memory Management Notes

- Replace manual allocation/reallocation with `Vec` growth and `String` construction.
- Eliminate raw pointer ownership transfer.
- Use borrowing for input parameters (`&str`, `&[u8]`) and owned buffers for stored slot results.
- If global slot state exists in the original C code, encapsulate it in a minimal Rust static with interior mutability only as required by the original API shape.

### Error Handling Notes

- Avoid panic-based control flow.
- If the existing Rust interface allows returning owned strings directly, do so.
- For byte-oriented quoting where UTF-8 conversion may fail, either:
  - keep internal bytes and expose a byte-returning helper, or
  - use lossless escaping so the final output is valid UTF-8 before constructing `String`

The implementation should preserve caller-visible behavior rather than introducing broad `Result` propagation unless the surrounding Rust project already expects it.

## Implementation Phases

### Phase 1: Establish Rust module skeleton and internal state mapping

- Create or update `src/quotearg.rs` as the Rust home for this C module subset.
- Identify the exact anonymous C structs used by `quotearg_n`, `quotearg_n_mem`, and `quotearg_n_custom`.
- Rename those anonymous structures into minimal Rust `struct`/`enum` definitions based only on fields required by the three target functions.
- Map any static slot-storage mechanism from C into a Rust-owned container.
- Define internal helper signatures needed by the three functions without expanding into unrelated quoting features.

**Exit criteria**
- Rust module compiles with placeholder bodies.
- All required state/config structures are represented in Rust.
- No extra modules or support systems are introduced.

### Phase 2: Port core byte-length-aware quoting path

- Implement `quotearg_n_mem` first as the central migration unit.
- Translate the C logic for:
  - slot lookup/allocation by index
  - input byte-slice processing
  - quoted output buffer generation
  - buffer replacement/reuse semantics
- Keep the implementation byte-oriented internally to match C memory semantics.
- Confirm no raw-pointer lifetime assumptions remain.

**Exit criteria**
- `quotearg_n_mem` behavior matches the C logic for normal, empty, and non-UTF-8 inputs.
- Slot growth and replacement are safe and deterministic.
- Unit tests cover representative byte-level cases.

### Phase 3: Port string and custom-entry wrappers

- Implement `quotearg_n` as the string-oriented wrapper over the length-aware path, reusing the same storage and quoting logic.
- Implement `quotearg_n_custom` by mapping custom left/right quote arguments into the existing option/state representation.
- Ensure custom quoting does not duplicate the entire core logic; it should route through the same internal quoting path with adjusted options.

**Exit criteria**
- `quotearg_n` and `quotearg_n_custom` compile and reuse common internals.
- Tests confirm wrapper equivalence with the core path.
- Custom delimiter handling matches C behavior for empty and non-empty inputs.

### Phase 4: Validation and cleanup

- Add focused `cargo test` coverage for:
  - multiple slot indices
  - repeated calls reusing the same index
  - custom delimiters
  - embedded NUL and arbitrary byte inputs via `_mem`
- Review function signatures and visibility so they match current crate usage.
- Remove temporary scaffolding and ensure comments document only migration-relevant behavior.

**Exit criteria**
- All migrated functions pass tests.
- The module uses standard Rust ownership and borrowing without unsafe code unless directly required by a verified C behavior dependency.
- The final layout remains constrained to the existing module scope.