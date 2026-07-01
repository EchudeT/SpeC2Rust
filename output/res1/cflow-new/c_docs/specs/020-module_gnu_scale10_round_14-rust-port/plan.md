# Implementation Plan

## Summary

Port the decimal scaling-and-rounding logic currently embedded in `gnu/vasnprintf.c` into a focused Rust module for `cflow-new`, preserving the existing behavior and call-level structure of:

- `scale10_round_decimal_decoded`
- `scale10_round_decimal_long_double`
- `scale10_round_decimal_double`

The Rust implementation should keep the migration narrow: translate the existing numeric processing into idiomatic Rust functions, retain the same conceptual layering between the decoded-decimal helper and the `double` / `long double` entry points, and avoid introducing new formatting or numeric abstraction layers beyond what is required for the port.

The technical approach is to:
- isolate the ported logic in one Rust source module corresponding to this C fragment,
- represent any anonymous C intermediate state with a private Rust struct,
- use standard-library floating-point types (`f64`, and a constrained mapping for `long double`),
- make mutation explicit through owned values and `&mut` references where the C code uses output parameters,
- preserve edge-case handling around decimal scaling, digit carry propagation, and rounding boundaries.

## Technical Context

- **Language/Version:** Rust 1.78+
- **Primary Dependencies:** Rust standard library only
- **Testing:** `cargo test`
- **Performance Goals:**
  - Match the original C logic closely enough to avoid avoidable regressions in decimal rounding paths.
  - Keep allocation behavior minimal and limited to what is already implied by digit-buffer handling.
  - Prefer direct numeric operations and in-place digit updates over layered abstractions.
  - Preserve predictable execution for typical formatting-sized inputs rather than optimizing for unsupported wider use cases.

## Module Mapping

### C to Rust File Mapping

- **C source:** `gnu/vasnprintf.c`
- **Rust target:** `src/module_gnu_scale10_round_14.rs`

This Rust file should contain only the ported logic for the identified functions and any strictly necessary private helpers or private data structures used by those functions.

### Function Mapping

- `scale10_round_decimal_decoded`
  - Port to a private or crate-visible Rust function with the same processing role.
  - Keep it as the core helper that operates on decoded decimal state and digit buffers.

- `scale10_round_decimal_long_double`
  - Port to a Rust function dedicated to the `long double` call path.
  - Because Rust has no native `long double`, map this path conservatively to the closest supported representation used by the current project constraints, most likely `f64`, unless surrounding code already defines an internal extended-precision representation.

- `scale10_round_decimal_double`
  - Port to a Rust function using `f64`.
  - Keep its responsibility limited to preparing numeric input and delegating to the decoded-decimal rounding helper.

### Visibility

- Default to `pub(crate)` only if the surrounding Rust crate needs cross-module access.
- Otherwise keep helpers private and expose only the minimum surface required by the translated call sites.

## Data Model

### Anonymous C Data Structure Mapping

The C analysis reports an anonymous structure. In Rust, represent it as a named private struct scoped to `src/module_gnu_scale10_round_14.rs`.

Example shape:

```rust
struct DecodedDecimal {
    // fields mapped directly from the anonymous C aggregate
}
```

### Mapping Rules

- **C anonymous struct**
  - **Rust:** private `struct DecodedDecimal`
  - Purpose: hold the decoded sign / exponent / digit span / temporary rounding state exactly as needed by the ported functions.

- **C character digit buffers**
  - **Rust:** `&mut [u8]`, `Vec<u8>`, or `&mut [char]` only if required by actual usage
  - Prefer `u8` digit storage when the C code operates on ASCII digit bytes.

- **C integer counters / indexes**
  - **Rust:** `usize` for indexes and lengths, `i32`/`isize` only where signed arithmetic is semantically required.

- **C floating-point inputs**
  - **Rust:** `f64` for `double`
  - **Rust for `long double`:** explicit project decision required during implementation; default to `f64` only if the original call sites do not depend on extended precision and no wider representation already exists in the Rust codebase.

- **C out-parameters**
  - **Rust:** return tuples or use `&mut` references where preserving update ordering improves fidelity to the original code.

### Memory Management

- Replace raw pointer-based digit manipulation with bounds-checked slices.
- Keep temporary storage local to functions.
- Avoid introducing heap allocation unless the original algorithm requires resizable digit storage.

### Error Handling

- If the C code assumes valid internal inputs, keep the Rust port similarly internal and use debug assertions for invariants.
- Use `Option` or `Result` only when a translated boundary actually needs to signal failure to callers.
- Do not invent new recoverable error cases for paths that are internal algorithm steps.

## Implementation Phases

## Phase 1: Extract and map the C logic boundary

- Inspect the relevant section of `gnu/vasnprintf.c` and isolate the exact code region for the three listed functions.
- Identify the anonymous data structure fields actually used by these functions.
- Determine whether `scale10_round_decimal_long_double` relies on precision beyond `double`, based on its local computations and immediate callers.
- Create `src/module_gnu_scale10_round_14.rs` with placeholder function signatures and the private decoded-decimal struct.
- Define Rust type mappings for:
  - digit buffers,
  - exponents / indexes,
  - rounding flags or state variables,
  - floating-point inputs.

### Deliverable
Compiling Rust module skeleton with function signatures, struct definitions, and documented type-mapping decisions.

## Phase 2: Port the core decoded-decimal rounding helper

- Translate `scale10_round_decimal_decoded` first, as the algorithmic center of the module.
- Preserve the original order of operations for:
  - decimal scaling,
  - digit inspection,
  - carry propagation,
  - boundary rounding decisions,
  - updates to exponent or digit count where applicable.
- Replace pointer arithmetic with explicit slice indexing.
- Keep helper logic local within the module; do not split into additional modules.
- Add focused unit tests derived from the C logic’s observable behavior, especially around:
  - exact midpoint cases,
  - carry into leading digit,
  - truncation boundaries,
  - zero-like and single-digit cases.

### Deliverable
Working Rust implementation of the decoded-decimal helper with narrow algorithm tests.

## Phase 3: Port the `double` and `long double` entry points

- Translate `scale10_round_decimal_double` onto `f64`, keeping its setup logic aligned with the C function.
- Translate `scale10_round_decimal_long_double` using the decided Rust representation from Phase 1.
- Reuse the decoded-decimal helper instead of reinterpreting the algorithm.
- Ensure the two entry points differ only where the C code truly differs in input preparation or precision assumptions.
- Add tests comparing the two call paths where behavior should coincide.

### Deliverable
Complete Rust port of all three functions with entry-point coverage and shared helper integration.

## Phase 4: Verification and cleanup

- Review all integer conversions and indexes for potential off-by-one or sign-conversion issues introduced during pointer-to-slice translation.
- Confirm that any in-place digit updates cannot panic under valid internal inputs.
- Remove placeholder comments and keep documentation limited to implementation-critical notes.
- Run `cargo test` and stabilize expected outputs for edge cases already represented by the C code.

### Deliverable
Finalized module ready on branch `020-module_gnu_scale10_round_14-rust-port` with passing tests and no extra architectural expansion.