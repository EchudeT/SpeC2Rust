# Implementation Plan

## Summary

Port `gnu/hash.c` into a focused Rust module that preserves the existing hash-string behavior without adding new capabilities. The implementation should migrate the current C string-hashing logic into a small, self-contained Rust source file, with the exported Rust function matching the original operational contract as closely as possible.

The technical approach is:

- translate the `hash_string` routine directly into safe Rust where possible;
- keep the algorithm layout close to the C source to reduce migration risk;
- model C string input explicitly, depending on how the surrounding Rust code will call it:
  - prefer `&[u8]` or `&str` for internal Rust-only usage;
  - if null-terminated byte semantics are required by the original code path, isolate that behavior in a small conversion layer rather than spreading C-style handling through the module;
- preserve integer width and overflow behavior intentionally, using fixed-width integer types where needed;
- keep the module limited to the hashing logic in `gnu/hash.c`, with unit tests covering parity-oriented examples and edge cases such as empty strings and byte sequences with high-bit values.

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - maintain linear-time hashing over input length;
  - avoid heap allocation during hashing;
  - preserve low-overhead byte iteration comparable to the C implementation;
  - keep function call boundaries minimal and data ownership simple.

## Module Mapping

### C to Rust File Mapping

- `gnu/hash.c` → `src/module_gnu_hash_string_17.rs`

If the project already groups migrated files under an existing module tree, the Rust file should be placed in that same tree, but no additional submodules should be introduced unless required by the current crate layout.

### Function Mapping

Because the input lists `hash_string` twice, treat this as a single migration target unless source inspection confirms distinct overload-like usages or duplicate declarations.

- `hash_string` → `pub(crate)` or private Rust function `hash_string(...) -> u32`/`u64`/`usize`
  - final return type must follow the original C integer type and overflow behavior from `gnu/hash.c`;
  - if there are two C declarations for the same implementation, expose only one Rust implementation and keep call sites consolidated.

## Data Model

The analysis reports only anonymous data structures and does not identify named structs required by this file. For this module, plan for a function-only port unless source inspection proves otherwise.

### Data-Structure Mapping

- Anonymous C data structures not used directly by `gnu/hash.c` hashing logic → no dedicated Rust type
- Any file-local constants, masks, or algorithm state in C → Rust `const` items or local variables
- C string inputs:
  - `const char *` → `&[u8]`, `&str`, or `&CStr` at the boundary chosen during implementation
  - selection should be based on actual caller expectations, with preference for the narrowest safe Rust representation that preserves original semantics

### Integer and Byte Semantics

- C unsigned integer used in hashing → matching Rust fixed-width unsigned type (`u32`, `u64`, or `usize` only if the C type is pointer-sized)
- `char` byte processing in C → `u8` processing in Rust
- overflow-sensitive arithmetic → explicit wrapping operations such as `wrapping_add`, `wrapping_mul`, or equivalent shifts/xors as required by the original algorithm

### Memory Management and Error Handling

- no manual memory management should be introduced;
- the hashing function should be allocation-free;
- if the original C function assumes valid non-null input, the Rust API should encode that as a non-optional borrowed input;
- if a null-terminated boundary is unavoidable, confine any `CStr` handling to a thin outer function and keep the core hash routine fully safe and slice-based;
- no new error types should be added unless source inspection reveals invalid-input handling already exists in the C code.

## Implementation Phases

### Phase 1: Source Audit and Signature Lock

- inspect `gnu/hash.c` to confirm:
  - the exact `hash_string` signature;
  - the precise return type and intermediate integer widths;
  - whether duplicate `hash_string` entries are duplicate declarations or separate call forms;
  - whether hashing stops at `'\0'` or uses explicit lengths;
  - any macro dependencies or file-local constants used by the function.
- choose the Rust function signature that most directly preserves existing semantics.
- create the target Rust file and wire it into the crate module tree with no extra architectural expansion.

### Phase 2: Direct Function Port

- translate `hash_string` into Rust with algorithm steps kept structurally close to the C implementation.
- replace C pointer traversal with:
  - slice iteration, if the input is length-based; or
  - `CStr`/byte scanning isolated at the boundary, if the input is null-terminated.
- preserve all arithmetic behavior explicitly using fixed-width integers and wrapping operations where needed.
- keep helper logic local unless the original C file already separates reusable pieces.

### Phase 3: Behavioral Verification

- add unit tests for:
  - empty input;
  - short ASCII strings;
  - longer strings;
  - bytes with values above ASCII range if the original C logic operates on raw bytes;
  - deterministic repeated calls producing identical output.
- where practical, derive expected values from the C implementation to confirm parity.
- verify there are no accidental allocations or ownership conversions in the hashing path.

### Phase 4: Integration Cleanup

- update any internal call sites to use the Rust module function signature selected in Phase 1.
- remove or avoid any redundant compatibility layers once call sites are confirmed.
- run `cargo test` and resolve any integer-cast, boundary-condition, or null-termination mismatches discovered during integration.