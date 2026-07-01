# Implementation Plan

## Summary

This module ports the `quotearg.c` entry-point layer for indexed quoting calls into Rust, specifically the functions:

- `quotearg_n`
- `quotearg_n_mem`
- `quotearg_n_custom`

The Rust implementation should preserve the existing call patterns and output behavior of the C module while replacing C-managed buffers and implicit global-state style storage with explicit, safe Rust ownership. The work should stay narrowly scoped to the existing file and functions, without introducing broader quoting features beyond what these functions require.

The technical approach is:

- migrate the logic from `quotearg.c` into a single Rust module aligned with the current project layout;
- model the indexed quote-argument storage using Rust-managed containers such as `Vec` and `String`/`Vec<u8>` as needed by the original byte-oriented behavior;
- preserve byte-length-sensitive handling for `quotearg_n_mem`;
- represent custom quoting parameters for `quotearg_n_custom` with a small Rust struct or enum matching the original call requirements;
- convert C null-pointer/error-prone flows into explicit Rust return types while keeping the external module behavior close to the source implementation.

The implementation should focus on parity with the original module behavior and migration of the listed functions only.

## Technical Context

### Language / Version

- Rust 1.78 or newer
- Edition: 2021

### Primary Dependencies

Use the Rust standard library by default:

- `std::borrow`
- `std::ffi` only if required by surrounding project interfaces
- `std::string::String`
- `std::vec::Vec`
- `std::sync` only if already required by the current project structure for existing global mutable state migration

Third-party crates are not recommended for this module because the provided input does not show a need beyond standard-library facilities.

### Testing

- `cargo test`

Testing scope should include:

- indexed argument quoting behavior for repeated calls;
- byte-length-aware behavior for `quotearg_n_mem`;
- custom quoting path coverage for `quotearg_n_custom`;
- boundary cases such as empty input, embedded non-UTF-8 bytes if applicable to the surrounding quoting implementation, and larger index values.

### Performance Goals

- Maintain behavior comparable to the C implementation for typical command-line argument sizes.
- Avoid unnecessary reallocations when storing or reusing per-index quoted results.
- Preserve linear-time processing with respect to input length.
- Keep temporary allocations limited to what is needed to produce and retain the quoted output for the indexed slot.

## Module Mapping

### C to Rust File Mapping

- `quotearg.c` -> `src/quotearg.rs`

If the project already places quoting logic in another existing Rust file, these three functions should be added there instead of creating additional layers. The goal is direct migration of the current C file responsibilities, not architectural expansion.

### Function Mapping

- `quotearg_n` -> `pub(crate) fn quotearg_n(...) -> ...`
- `quotearg_n_mem` -> `pub(crate) fn quotearg_n_mem(...) -> ...`
- `quotearg_n_custom` -> `pub(crate) fn quotearg_n_custom(...) -> ...`

Implementation notes:

- `quotearg_n` should become the Rust entry point for quoting a null-terminated or string-like input using an indexed result slot.
- `quotearg_n_mem` should handle explicit byte slices and lengths, and should be treated as the core implementation path if that best matches the C structure.
- `quotearg_n_custom` should layer custom delimiter or quoting configuration onto the same indexed storage flow rather than duplicating quoting logic.

### Internal Mapping Strategy

The C implementation commonly relies on reusable buffers indexed by `n`. In Rust, map this to:

- an internal growable collection for per-index stored results;
- each slot owning its quoted buffer;
- helper routines that resize storage up to the requested index before writing the new result.

This keeps the original indexed semantics while replacing manual memory management.

## Data Model

The analysis lists only anonymous C structures. Since no stable named structs are provided, the Rust plan should reconstruct only the data actually required by these three functions.

### Data Structure Mapping

- anonymous quote option/config structure(s) -> private Rust `struct` holding only fields referenced by `quotearg_n`, `quotearg_n_mem`, and `quotearg_n_custom`
- anonymous custom quoting payload -> private Rust `struct CustomQuoting` or similar
- anonymous indexed slot/buffer state -> private Rust `Vec<QuotedSlot>`
- anonymous byte buffer record -> private Rust `struct QuotedSlot { buf: Vec<u8> }` or `String` if surrounding logic guarantees UTF-8
- anonymous flag/style constants -> private Rust `enum` or `const` values, matching only the styles used by this module path

### Recommended Rust Shapes

```rust
struct QuotedSlot {
    buf: Vec<u8>,
}

struct CustomQuoting {
    left: u8,
    right: u8,
}

struct QuoteOptions {
    // only fields actually needed by these functions
}
```

If the surrounding port already defines quote options and quoting style types, this module should reuse those existing Rust definitions instead of creating parallel ones.

### Memory Management Decisions

- Replace manual heap allocation and reallocation with `Vec<u8>` or `String`.
- Store per-index outputs in owned Rust containers so returned references or slices remain valid according to the chosen API shape.
- Resize indexed storage explicitly before assignment.
- Avoid exposing raw pointers internally unless required by compatibility with surrounding code.

### Error Handling Decisions

C code in this area often assumes infallible allocation or process-aborting behavior. In Rust:

- prefer internal infallible interfaces when matching existing project conventions;
- if the surrounding module already uses `Result`, propagate errors consistently;
- do not add new recovery paths not present in the source behavior;
- treat invalid custom quoting parameters as local precondition checks if the C implementation assumes valid input.

## Implementation Phases

## Phase 1: Establish Rust module skeleton and shared storage

- Create or update `src/quotearg.rs` as the Rust destination for `quotearg.c`.
- Identify any already-ported quote option/style types used by the broader quoting subsystem and reuse them directly.
- Introduce the minimal internal storage required to preserve indexed quoting results.
- Define Rust representations for any anonymous C data that these three functions directly depend on.
- Decide whether slot contents are stored as `Vec<u8>` or `String` based on the byte semantics needed by `quotearg_n_mem`.

### Deliverables

- Rust module file in place
- private storage and helper types defined
- compile-ready skeletons for:
  - `quotearg_n`
  - `quotearg_n_mem`
  - `quotearg_n_custom`

## Phase 2: Port core indexed quoting behavior

- Implement `quotearg_n_mem` first as the core path because it directly represents explicit input bytes and length.
- Port the slot-growth logic from C to Rust so a requested index always has backing storage before writing output.
- Migrate quoting invocation flow from the C implementation, reusing existing Rust quote logic if already available elsewhere in the port.
- Implement `quotearg_n` as a thin wrapper over `quotearg_n_mem`, preserving the original argument adaptation logic.
- Ensure ownership and returned data lifetimes match the module’s intended use without raw C buffer hazards.

### Deliverables

- working `quotearg_n_mem`
- working `quotearg_n`
- tests for:
  - empty input
  - repeated overwrite of the same index
  - nonzero index expansion
  - explicit-length handling

## Phase 3: Port custom quoting path

- Implement the Rust equivalent of `quotearg_n_custom` using the same indexed storage path as the other functions.
- Add the minimal custom quoting configuration structure needed by this function.
- Ensure the custom path does not fork buffer-management behavior from the shared implementation.
- Verify any delimiter or style assumptions taken from the original C logic and keep them local to this function or a small helper.

### Deliverables

- working `quotearg_n_custom`
- tests for:
  - custom delimiter application
  - repeated calls at the same and different indices
  - interaction with empty and ordinary inputs

## Phase 4: Conformance cleanup and integration validation

- Compare Rust behavior against the C source logic for all three functions, focusing on index handling, byte preservation, and custom quoting configuration.
- Remove any migration scaffolding not needed after the functions compile and tests pass.
- Align visibility and signatures with the rest of the Rust port so the module integrates without introducing extra abstraction layers.
- Run `cargo test` and fix any borrow/lifetime or ownership issues in retained slot access.

### Deliverables

- final integrated Rust module for `main_root_quotearg_n_08`
- passing unit tests for migrated function paths
- source kept narrowly aligned with the original file responsibilities