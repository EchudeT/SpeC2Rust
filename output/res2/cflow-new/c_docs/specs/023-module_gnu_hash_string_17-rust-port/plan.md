# Implementation Plan

## Summary

This module is a focused port of `gnu/hash.c` into Rust, preserving the existing hashing behavior of `hash_string` without adding new capabilities or restructuring the surrounding project beyond what is required for the migration.

The Rust implementation should translate the C hashing routine into a small, self-contained Rust module that:
- keeps the algorithm behavior byte-for-byte compatible with the C implementation,
- uses safe Rust where possible,
- handles string and byte traversal explicitly to match C semantics,
- exposes a direct Rust equivalent of the existing function shape used by the project.

Because the source analysis shows only `hash_string` from a single file, the implementation scope should remain narrow: migrate the function logic, confirm integer-width behavior, and add compatibility tests around representative inputs and edge cases.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Match the C implementation’s asymptotic cost and low-overhead execution profile.
  - Avoid heap allocation in the hashing path.
  - Operate directly on string/byte slices.
  - Preserve integer overflow/bit-manipulation semantics explicitly so optimized builds remain correct and predictable.

## Module Mapping

| C File | C Function | Rust Target | Notes |
|---|---|---|---|
| `gnu/hash.c` | `hash_string` | `src/gnu/hash.rs::hash_string` | Direct port of the hashing routine. |
| `gnu/hash.c` | `hash_string` | `src/gnu/hash.rs::tests::*` | Validation coverage for compatibility and edge cases. |

Recommended Rust file layout:

```text
src/
  gnu/
    mod.rs
    hash.rs
```

If the project already has a different internal layout, place the Rust port into the closest existing equivalent and avoid introducing extra abstraction layers.

## Data Model

The analysis lists only anonymous data structures and no named structures tied to this module’s visible API. Since the module scope appears to be a standalone hashing function, the expected Rust migration is minimal.

| C Representation | Rust Representation | Notes |
|---|---|---|
| `char *` / `const char *` input string | `&str` or `&[u8]` | Prefer `&str` when callers provide valid text; use `&[u8]` internally or as the primary implementation target if exact byte semantics are required. |
| NUL-terminated string traversal | byte-slice iteration | Rust should not rely on sentinel termination; callers should pass the intended slice explicitly. |
| `unsigned int` / `unsigned long` hash accumulator | `u32` or `u64` | Choose the exact width by inspecting the C implementation and preserving overflow behavior. |
| anonymous local temporaries | local variables | Keep these as function-local bindings; do not introduce structs unless the C code requires stateful grouping. |

### Data-structure handling decisions

- No dedicated Rust struct is required unless the original file contains hidden state not shown in the analysis.
- Bitwise and arithmetic behavior must be encoded with explicit wrapping operations such as `wrapping_add`, `wrapping_mul`, `wrapping_shl`, or equivalent masking, depending on the original C algorithm.
- If the C function accepts raw pointers, the public Rust API should convert that behavior into slice-based or string-based input while keeping the internal algorithm unchanged.

## Implementation Phases

## Phase 1: Inspect and define exact function contract

- Review `gnu/hash.c` and determine the exact signature and integer types used by `hash_string`.
- Confirm:
  - input type semantics,
  - termination rules,
  - character signedness assumptions,
  - accumulator width,
  - overflow expectations,
  - return type width.
- Decide the Rust signature that most directly preserves behavior, for example:
  - `pub fn hash_string(input: &str) -> u32`, or
  - `pub fn hash_string(input: &[u8]) -> u32`
- Record any required conversion rules for callers if the C implementation depended on NUL-terminated input.

### Deliverable
- Finalized Rust function signature and type mapping for the module.

## Phase 2: Port `hash_string` into Rust

- Create `src/gnu/hash.rs`.
- Translate the C function body directly, keeping operation order close to the source.
- Preserve all arithmetic semantics explicitly using wrapping operations where needed.
- Avoid heap allocation and avoid introducing helper types unless required by repeated low-level logic.
- Keep the implementation local and flat rather than abstracting the algorithm into multiple layers.

### Memory management and safety
- Replace pointer walking with slice or byte iteration.
- Eliminate undefined behavior risks from signed overflow, invalid pointer access, and implicit NUL scanning.
- If exact C behavior depends on reading until NUL, model that explicitly over a provided byte slice and stop at the first zero byte only if that is part of the original contract.

### Error handling
- Prefer an infallible API if the original function is infallible.
- Do not introduce `Result` unless the Rust boundary requires validating input not representable in the C contract.
- If both `&str` and raw byte compatibility are needed, keep one internal function over `&[u8]` and expose only the minimal public wrapper actually needed by the project.

### Deliverable
- Completed Rust implementation of `hash_string` in `src/gnu/hash.rs`.

## Phase 3: Add compatibility and edge-case tests

- Add unit tests in `src/gnu/hash.rs` or adjacent test modules.
- Cover:
  - empty input,
  - short ASCII strings,
  - longer strings,
  - strings containing high-bit bytes if byte-oriented behavior matters,
  - embedded NUL byte behavior if relevant to the C contract,
  - deterministic repeated calls.
- Where feasible, derive expected outputs from the original C implementation and encode them as fixed vectors.
- Validate that overflow-sensitive cases produce stable outputs in debug and release builds.

### Deliverable
- `cargo test` passing with compatibility-focused coverage.

## Phase 4: Integrate and finalize module replacement

- Export the Rust module through the existing crate module tree with the smallest necessary change.
- Replace any direct use of the C implementation within the Rust project branch with the new Rust function.
- Verify naming alignment so the migrated function remains easy to trace back to `gnu/hash.c`.
- Confirm no extra facilities were added beyond the port itself.

### Deliverable
- Rust module wired into the project and ready to serve as the replacement for the C implementation.