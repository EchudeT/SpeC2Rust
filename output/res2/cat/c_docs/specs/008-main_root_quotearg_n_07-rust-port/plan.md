# Implementation Plan

## Summary

This plan covers the Rust port of the `quotearg.c` portion used by `main_root_quotearg_n_07`, specifically the migration of:

- `quotearg_n`
- `quotearg_n_mem`
- `quotearg_n_custom`

The Rust implementation should preserve the existing behavior and call structure of the C code while replacing manual buffer ownership and slot management with Rust-owned string/byte storage. The work should stay narrowly scoped to the existing quoting entry points and the data they directly require from `quotearg.c`.

Technical approach:

- Port the required quoting state and option data from `quotearg.c` into a Rust module.
- Recreate the slot-based `quotearg_n*` behavior using Rust-managed per-slot storage rather than raw heap buffers.
- Represent input as `&[u8]` where the C code accepts pointer-plus-length, and expose string-returning or byte-backed results only as needed by the existing Rust port structure.
- Keep error behavior aligned with C expectations where practical, but model internal failures with infallible Rust allocation semantics unless the surrounding port requires explicit error propagation.
- Avoid adding broader quoting APIs or unrelated infrastructure beyond what is needed to migrate these three functions and their directly referenced helpers/state.

## Technical Context

- **Language/Version**: Rust 1.74 or newer
- **Primary Dependencies**:
  - Rust standard library
  - No third-party crates recommended from the provided evidence
- **Testing**:
  - `cargo test`
  - Unit tests focused on slot reuse, length-based input handling, and custom quoting behavior
- **Performance Goals**:
  - Maintain linear-time processing relative to input size
  - Avoid unnecessary intermediate allocations beyond the final quoted buffer
  - Preserve reusable slot storage semantics so repeated `quotearg_n*` calls do not degrade into avoidable repeated setup work
  - Keep memory ownership explicit and bounded to Rust containers (`Vec<u8>`, `String`, or equivalent)

## Module Mapping

### C to Rust File Mapping

- `quotearg.c` → `src/quotearg.rs`

### Function Mapping

- `quotearg_n` → `pub(crate) fn quotearg_n(...)`
- `quotearg_n_mem` → `pub(crate) fn quotearg_n_mem(...)`
- `quotearg_n_custom` → `pub(crate) fn quotearg_n_custom(...)`

### Internal Scope Mapping

Only migrate the functions and directly required supporting items from `quotearg.c`. Supporting helpers should remain private to `src/quotearg.rs` unless another already-ported module requires visibility. Do not split the port into additional modules unless the existing Rust project layout already mandates that.

## Data Model

The analysis lists only anonymous C data structures, so the Rust plan should map them by role as discovered during implementation rather than inventing new abstractions. The expected mappings are:

| C structure role | Rust mapping | Notes |
|---|---|---|
| Quoting options record | `struct QuotingOptions` | Holds the ported option fields needed by `quotearg_n`, `quotearg_n_mem`, and `quotearg_n_custom`. Keep fields close to C layout/meaning for migration clarity. |
| Slot entry for cached quoted result | `struct QuoteSlot` | Owns the current quoted buffer for a slot, likely as `Vec<u8>` or `String` depending on caller needs. |
| Global or module-local slot array/state | `struct QuoteSlotSet` or module-local `Vec<QuoteSlot>` guarded by existing single-threaded assumptions | Mirrors C’s reusable slot storage without raw pointers. |
| Style/discriminant values used by quoting options | `enum QuotingStyle` | Replace C integer constants/macros with a typed enum where directly applicable. |
| Character bitmap / flag arrays used by options | Fixed-size arrays such as `[bool; N]`, `[u32; N]`, or byte arrays | Choose exact type based on C field semantics discovered in `quotearg.c`. |
| Custom quoting delimiters | `struct CustomQuoting { left: Vec<u8>, right: Vec<u8> }` or borrowed slices in options | `quotearg_n_custom` needs explicit left/right delimiter representation. |
| Pointer-plus-length string inputs | `&[u8]` | Use for `_mem` behavior and for preserving non-UTF-8 compatibility. |
| NUL-terminated string inputs | `&CStr` at boundary, then `&[u8]` internally, or `&str` only if already guaranteed UTF-8 by surrounding Rust code | Prefer byte-oriented handling internally to match C semantics. |

### Memory Management Decisions

- Replace C-managed returned buffers with Rust-owned slot buffers.
- If a function must mimic “return stable pointer for slot `n`” semantics for surrounding code, centralize this in the module and ensure the backing storage outlives the returned borrow.
- Prefer `Vec<u8>` internally because quoting logic may operate on arbitrary bytes, not valid UTF-8.
- Convert to `String` only at interfaces that already require UTF-8 and only after validation.

### Error Handling Decisions

- C behaviors that assume allocation success should map to ordinary Rust allocation paths without extra recovery layers.
- Invalid custom delimiter configuration should be represented explicitly:
  - either by preserving C preconditions with `debug_assert!`/documented assumptions,
  - or by returning a narrow internal error type only if the surrounding Rust port already uses `Result`.
- Do not introduce broad new error hierarchies.

## Implementation Phases

## Phase 1: Port Required Types and State

- Create `src/quotearg.rs`.
- Identify the minimal set of constants, enums, option fields, and slot state from `quotearg.c` required by:
  - `quotearg_n`
  - `quotearg_n_mem`
  - `quotearg_n_custom`
- Port those items with names close to the C source to simplify verification.
- Replace anonymous C structs with explicitly named Rust structs based on their runtime role.
- Implement module-local storage for quote slots using Rust-owned containers.
- Decide and document the internal text representation (`Vec<u8>` preferred unless existing project code already dictates `String`).

### Exit Criteria

- All required data structures compile in Rust.
- Slot storage and quoting option state exist without unsafe raw allocation logic.
- No unrelated quoting entry points are added.

## Phase 2: Port Core Quoted-Argument Path

- Implement the internal quoting routine(s) needed by `quotearg_n_mem`.
- Port `quotearg_n_mem` first, since it is the length-aware base path and can serve as the core implementation.
- Preserve C behavior for:
  - explicit slot selection by index
  - reuse/overwrite of prior slot contents
  - byte-oriented input handling
  - option-driven quoting behavior needed by this module slice
- Keep helper functions private and limited to those directly invoked by the three target functions.

### Exit Criteria

- `quotearg_n_mem` is functional against representative byte inputs.
- Slot replacement behavior is stable and memory-safe.
- The implementation handles non-UTF-8 inputs without lossy conversion.

## Phase 3: Port Wrapper Entry Points

- Implement `quotearg_n` as the null-terminated or standard-string wrapper over `quotearg_n_mem`.
- Implement `quotearg_n_custom` by constructing the appropriate custom option state and delegating to the shared core path.
- Ensure the wrappers do not duplicate quoting logic.
- Keep ownership and borrowing boundaries explicit so returned data cannot outlive slot storage.

### Exit Criteria

- All three target functions compile and share a common implementation path.
- Custom delimiter handling is wired through the ported options model.
- No duplicate quoting code remains across the three entry points.

## Phase 4: Validation and Behavioral Parity Checks

- Add focused `cargo test` coverage for:
  - repeated calls with the same slot index
  - multiple slot indices
  - length-based inputs containing embedded NUL bytes for `quotearg_n_mem`
  - custom left/right delimiters for `quotearg_n_custom`
  - empty input
  - non-UTF-8 byte sequences
- Compare edge behavior against the C implementation during port review, especially around delimiter insertion and slot overwriting.
- Remove any temporary migration scaffolding not required by the final Rust module.

### Exit Criteria

- Tests pass under `cargo test`.
- The module remains narrowly scoped to the original C functions and their direct dependencies.
- Memory management is entirely Rust-owned, with no raw-pointer lifetime hazards exposed by the port.