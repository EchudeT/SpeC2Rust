# Implementation Plan

## Summary

Port `gnu/vasnprintf.c` into a single Rust module that preserves the existing internal helper behavior used by formatted numeric/string conversion logic, with particular focus on the localized length helpers, wide-character to multibyte conversion fallbacks, decimal point lookup, and decimal digit arithmetic helpers.

The Rust implementation should stay close to the original file structure and function boundaries rather than redesigning the subsystem. The technical approach is:

- migrate the C helper functions into one Rust source module with matching internal responsibilities;
- replace raw pointer and manual buffer traversal with slice- and iterator-based logic where possible;
- preserve C-like boundary behavior for `strnlen`/`wcsnlen` style helpers and arithmetic routines;
- represent fallible character conversion and arithmetic operations with `Result`/`Option` instead of sentinel return patterns where this does not break call-site equivalence;
- keep allocations minimal and localized, using standard library string and vector types only where the original code effectively builds byte sequences.

## Technical Context

- **Language/Version**: Rust 1.78 or newer
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Maintain linear behavior for bounded length scans.
  - Avoid unnecessary heap allocation in helper routines that can operate on slices or stack-local buffers.
  - Keep decimal arithmetic helpers efficient enough for formatting workloads by preserving digit-vector style processing rather than introducing heavyweight numeric abstractions.
  - Match original behavior before attempting any micro-optimization.

## Module Mapping

### Source File Mapping

- `gnu/vasnprintf.c` -> `src/module_gnu_vasnprintf.rs`

### Function Mapping

- `local_strnlen` -> `fn local_strnlen(input: &[u8], max_len: usize) -> usize`
- `local_wcslen` -> `fn local_wcslen(input: &[char]) -> usize` or `fn local_wcslen(input: &[u32]) -> usize` depending on upstream representation actually needed by callers
- `local_wcsnlen` -> `fn local_wcsnlen(input: &[char], max_len: usize) -> usize` or bounded `u32` slice variant
- `wctomb_fallback` -> `fn wctomb_fallback(ch: char) -> Result<Vec<u8>, ConversionError>`
- `local_wcrtomb` -> `fn local_wcrtomb(ch: char) -> Result<Vec<u8>, ConversionError>`
- `local_wctomb` -> `fn local_wctomb(ch: char) -> Result<Vec<u8>, ConversionError>`
- `decimal_point_char` -> `fn decimal_point_char() -> u8`
- `multiply` -> `fn multiply(digits: &mut Vec<u8>, factor: u32) -> Result<(), ArithmeticError>`
- `divide` -> `fn divide(digits: &mut Vec<u8>, divisor: u32) -> Result<u32, ArithmeticError>`
- `convert_to_decimal` -> `fn convert_to_decimal(...) -> Result<..., ArithmeticError>`

### Visibility Mapping

- Keep all migrated helpers private to the module unless existing Rust-side integration requires broader visibility.
- Expose only the minimum entry points required by the translated `vasnprintf` logic.

## Data Model

The analysis reports only an anonymous C data structure. The Rust port should avoid inventing new public data types unless needed to preserve local state across helper calls.

### Data Structure Mapping

- anonymous local C structure used for temporary arithmetic/conversion state
  - -> private Rust `struct` only if repeated mutable state exists across `multiply`, `divide`, and `convert_to_decimal`
  - otherwise -> local variables plus `Vec<u8>` for digit storage

### Recommended Internal Representations

- C `char *` with explicit bound
  - -> `&[u8]` for read-only scanning
- C wide-character buffers
  - -> `&[char]` if already decoded Unicode scalar values are the natural Rust-side representation
  - -> `&[u32]` if preserving original code-point semantics is required by translated callers
- C output byte buffer for multibyte conversion
  - -> fixed local `[u8; 4]` or `Vec<u8>` depending on call pattern
- C integer status/error sentinel
  - -> `Result<T, ConversionError>` / `Result<T, ArithmeticError>`

### Error Types

Define minimal private error enums only where the C code can fail in observable ways:

```rust
enum ConversionError {
    InvalidCodePoint,
    EncodingFailed,
}

enum ArithmeticError {
    DivisionByZero,
    Overflow,
    InvalidState,
}
```

These should remain private unless upstream callers require propagation across module boundaries.

## Implementation Phases

## Phase 1: Create Module Skeleton and Port Length Helpers

### Goals
- Establish the Rust file corresponding to `gnu/vasnprintf.c`.
- Port the bounded/unbounded string and wide-string length helpers first.

### Tasks
- Create `src/module_gnu_vasnprintf.rs`.
- Implement:
  - `local_strnlen`
  - `local_wcslen`
  - `local_wcsnlen`
- Decide the exact wide-character representation based on translated caller expectations:
  - prefer `char` when call sites already operate on Unicode scalar values;
  - otherwise use `u32` and validate only where conversion occurs.
- Preserve bounded-scan semantics exactly:
  - stop at NUL-equivalent element;
  - never read past provided bound/slice length.

### Notes
- These helpers should use safe slice iteration.
- If translated callers still model C NUL-terminated storage, represent it as slices containing a terminating zero value rather than raw pointers.

### Exit Criteria
- Unit tests cover empty, unterminated-within-bound, terminated-before-bound, and exact-bound cases.
- No `unsafe` required for these helpers.

## Phase 2: Port Wide-Character Conversion Helpers

### Goals
- Translate character-to-multibyte conversion helpers while keeping behavior narrow and local to the module.

### Tasks
- Implement:
  - `wctomb_fallback`
  - `local_wcrtomb`
  - `local_wctomb`
- Use Rust UTF-8 encoding as the default conversion path where it matches the original practical behavior.
- If the original helper distinguishes restartable/non-restartable conversion APIs, preserve separate function boundaries even if implementations share an internal helper.
- Normalize invalid code-point handling through a small private error enum.
- Keep temporary output on the stack when possible; return owned bytes only if that best matches surrounding translated code.

### Memory and Error Handling
- Avoid raw destination pointers; use local buffers or returned byte vectors/slices.
- Replace negative error returns with `Result`.
- If call sites need C-style lengths, return the produced byte count alongside the bytes or provide a small internal buffer wrapper.

### Exit Criteria
- Tests cover ASCII, multibyte Unicode, and invalid code-point scenarios.
- Function boundaries remain recognizable relative to the C source.

## Phase 3: Port Locale Decimal Point and Arithmetic Helpers

### Goals
- Migrate the numeric support routines used for decimal formatting without redesigning their algorithmic structure.

### Tasks
- Implement:
  - `decimal_point_char`
  - `multiply`
  - `divide`
- Default `decimal_point_char` to standard-library-accessible behavior; if locale-specific behavior is not directly available without extra dependencies, keep the implementation conservative and document the limitation in code comments rather than adding crates.
- Represent decimal working storage as a mutable digit vector closely aligned with the original algorithm.
- Preserve carry/borrow/remainder behavior exactly when porting `multiply` and `divide`.

### Memory and Error Handling
- Use `Vec<u8>` for mutable decimal digits.
- Detect division-by-zero and arithmetic overflow explicitly.
- Keep helper APIs private and shaped around existing call needs.

### Exit Criteria
- Tests validate arithmetic on small and carry-producing inputs.
- Decimal point lookup is implemented without introducing external locale crates.

## Phase 4: Port `convert_to_decimal` and Integrate Module Behavior

### Goals
- Complete the remaining conversion helper and verify the module works as a coherent translation unit.

### Tasks
- Implement `convert_to_decimal` using the already-ported digit helpers.
- Keep the original processing order and intermediate representation as much as possible.
- Adjust helper signatures only as needed to make ownership and mutation explicit in Rust.
- Wire all helpers together within the module in the same approximate dependency order as in the C file.
- Add targeted regression tests based on edge cases implied by the arithmetic and conversion helpers.

### Validation
- Review for:
  - bounds safety;
  - elimination of manual memory management;
  - equivalent handling of zero values, limits, and invalid input;
  - minimal and justified `unsafe` usage, ideally none.

### Exit Criteria
- `cargo test` passes for the migrated helper set.
- The module is self-contained and ready for integration on branch `058-module_gnu_vasnprintf.c_52-rust-port`.