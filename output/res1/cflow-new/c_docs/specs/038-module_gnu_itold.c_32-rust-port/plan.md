# Implementation Plan: module_gnu_itold.c_32

## Summary

This module ports `gnu/itold.c` into Rust with a narrow scope: preserve the behavior of `_Qp_itoq` and migrate its numeric conversion logic into an idiomatic Rust implementation that stays close to the original control flow and representation requirements.

The Rust implementation should focus on:
- translating the integer-to-extended-floating conversion routine without adding new capabilities,
- using explicit, fixed-width integer types from the standard library,
- keeping the implementation self-contained in a single Rust module aligned to the original file,
- making ownership and initialization explicit so that the Rust version avoids the undefined behavior risks common in C bit-level numeric code.

Because the source module exposes one conversion function and no named C data structures were identified, the plan should center on faithfully mapping the function boundary, internal bit manipulation, and result representation used by the surrounding Rust port.

## Technical Context

- **Language/Version**: Rust 1.78+ stable
- **Primary Dependencies**:
  - Rust standard library only
  - No third-party crates are recommended based on the provided module scope
- **Testing**:
  - `cargo test`
- **Performance Goals**:
  - Match the original C routine’s constant-time, allocation-free behavior for scalar conversion
  - Avoid heap allocation
  - Keep conversions and bit operations on primitive integers only
  - Preserve directness of the original implementation rather than introducing abstraction layers that may obscure generated code

## Module Mapping

| C Source File | Rust Module/File | Notes |
|---|---|---|
| `gnu/itold.c` | `src/module_gnu_itold.rs` | Direct port of the module’s single function and any required internal helpers kept private to the same file |

| C Function | Rust Item | Mapping Notes |
|---|---|---|
| `_Qp_itoq` | `pub(crate)` function `qp_itoq` or `_qp_itoq` | Keep the function focused on integer-to-target-float conversion semantics; naming may preserve the original symbol shape if required by the existing Rust port conventions |

## Data Model

No explicit C structs or enums were listed for this module. The data model work should therefore be limited to primitive representation mapping required by `_Qp_itoq`.

| C Representation | Rust Representation | Notes |
|---|---|---|
| integral input to `_Qp_itoq` | `i32`, `i64`, `u32`, or `u64` as determined by the original signature | Use exact-width integer types once the C signature is confirmed from `gnu/itold.c` |
| extended/quad-style floating result storage | project-local Rust representation already used by the surrounding port, or a private Rust struct if this module currently owns the representation | Do not invent a new public numeric type unless the existing port requires one |
| C bitfields / manual word assembly | primitive integer fields and masks | Implement with explicit shifts, masks, and endianness-independent logic where possible |

If the original function writes through pointers rather than returning by value, map this carefully:
- input references should become plain values where possible,
- output pointers should become `&mut` only if the surrounding API requires in-place mutation,
- nullability assumptions from C should be eliminated at the Rust boundary.

## Implementation Phases

### Phase 1: Source Signature Confirmation and Module Skeleton

- Inspect `gnu/itold.c` to confirm the exact signature, integer width, signedness, and result representation used by `_Qp_itoq`.
- Create the Rust module file at the mapped location following the crate’s existing module layout.
- Define the Rust function signature to mirror the C behavior as closely as possible without using raw pointers unless the surrounding port already requires them.
- Identify any constants, masks, exponent values, or word-layout assumptions embedded in the C implementation and transcribe them as private Rust constants.

**Exit criteria**:
- Rust module exists with a compiling function skeleton.
- All primitive type mappings are fixed and documented in code comments where layout-sensitive.

### Phase 2: Core Conversion Port

- Port the body of `_Qp_itoq` into Rust using primitive arithmetic and bit operations only.
- Preserve the original conversion order:
  - zero handling,
  - sign extraction if applicable,
  - magnitude normalization,
  - exponent/significand assembly,
  - final result placement into the target representation.
- Replace C implicit casts with explicit Rust casts at every width transition.
- Eliminate C-style uninitialized temporaries by fully initializing local variables before use.
- Where the C code depends on shifting edge cases, guard them explicitly so Rust shift semantics remain correct and panic-free in debug builds.

Memory and safety focus:
- no heap allocation,
- no aliasing through multiple mutable references,
- avoid `unsafe` unless the broader port’s representation makes it strictly necessary.

**Exit criteria**:
- Function logic is fully ported.
- Implementation compiles without placeholder branches.
- No unnecessary `unsafe` is introduced.

### Phase 3: Representation Validation and Edge-Case Tests

- Add unit tests for `_Qp_itoq` covering:
  - zero input,
  - smallest and largest relevant magnitude values,
  - negative input if signed conversion is involved,
  - powers of two and near-boundary integers that stress normalization,
  - sign and exponent assembly edge cases implied by the C logic.
- If an existing Rust representation for the target floating format is used, validate the produced internal words/fields rather than relying only on decimal display behavior.
- Compare expected outputs against known bit patterns derived from the C implementation or manually reasoned cases.

**Exit criteria**:
- `cargo test` passes.
- Tests cover the major conversion branches and boundary values.

### Phase 4: Cleanup and Integration Alignment

- Ensure the final Rust item names and visibility match the rest of the port branch conventions.
- Remove any temporary debug scaffolding used during translation.
- Review for exact-width correctness, unnecessary casts, and any remaining C-idiom artifacts that can be simplified without changing behavior.
- Confirm that the final module remains constrained to the original file’s responsibility and does not introduce unrelated helpers elsewhere.

**Exit criteria**:
- Module is ready for branch integration.
- Implementation is minimal, self-contained, and behavior-focused.

## Notes on Memory Management and Error Handling

- This module should be allocation-free.
- Numeric conversion should be total for the input domain defined by the original C function; prefer direct deterministic computation over fallible APIs.
- If the C function assumes valid output storage via pointer parameters, encode that guarantee in Rust through references rather than runtime checks.
- Do not add recovery paths or new error enums unless the original interface already exposes failure behavior.