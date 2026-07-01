# Implementation Plan

## Summary

Port the `quotearg.c` portion used by `main_root_quotearg_style_14` into Rust by implementing the two exposed functions:

- `quotearg_style`
- `quotearg_style_mem`

The Rust work should stay narrowly aligned with the existing C surface and behavior needed by this module cluster. The implementation should translate the style-driven quoting logic into safe Rust string/byte handling while preserving the original call relationships and output semantics expected by the `pwd` project.

Technical approach:

- Migrate the quoting-style selection logic into a Rust module dedicated to `quotearg`.
- Represent quoting styles with Rust enums rather than anonymous C constants/records.
- Implement byte-aware processing for the `_mem` variant so the Rust code can handle non-UTF-8 input without assuming valid text.
- Use owned Rust return values (`String` or `Vec<u8>` internally, with a stable public wrapper chosen to match the project’s Rust-side API) instead of C-style static/shared buffers.
- Keep error handling explicit and minimal: avoid panics for normal input paths, and confine any UTF-8 conversion decisions to API boundaries.

## Technical Context

- **Language/Version**: Rust 1.75 or newer
- **Primary Dependencies**:
  - Rust standard library
  - No third-party crates recommended based on the provided module scope
- **Testing**:
  - `cargo test`
- **Performance Goals**:
  - Maintain linear-time processing with respect to input length
  - Avoid unnecessary intermediate allocations during quoting
  - Preserve byte-oriented behavior for `quotearg_style_mem`
  - Keep implementation cost comparable to the C version for short command-line-sized inputs

## Module Mapping

| C File | C Function | Rust Module | Rust Item |
|---|---|---|---|
| `quotearg.c` | `quotearg_style` | `src/quotearg.rs` | `pub fn quotearg_style(...) -> String` |
| `quotearg.c` | `quotearg_style_mem` | `src/quotearg.rs` | `pub fn quotearg_style_mem(...) -> String` or byte-preserving helper plus public wrapper |

Notes:

- Keep both migrated functions in a single Rust source file unless the existing Rust project layout already has a dedicated quoting module.
- If `main` or another migrated file already calls these functions, update only those call sites required for this branch.
- Do not introduce extra abstraction layers beyond a small internal helper for shared quoting logic.

## Data Model

The C analysis reports only anonymous data structures. For this migration, map them to minimal Rust types based on actual use in `quotearg_style` and `quotearg_style_mem`.

| C Construct | Rust Mapping | Notes |
|---|---|---|
| Anonymous style-related constants/records | `enum QuotingStyle` | Central representation for quote style selection |
| Anonymous option/config records used by quoting logic | `struct QuotingOptions` | Only if required by the migrated functions; otherwise keep style-only parameters |
| Anonymous static tables for quote characters / escape behavior | `const` arrays or `match` expressions | Prefer compile-time constants over runtime state |
| C strings / `char *` inputs | `&str` for text-only call paths, `&[u8]` internally for raw memory paths | Use byte slices for `_mem` behavior |
| C output buffers | `String` or internal `Vec<u8>` | Use `String` only when final output is guaranteed/constructed as valid UTF-8 |

Recommended Rust shapes:

```rust
pub enum QuotingStyle {
    // exact variants should match the styles actually referenced by this module
}

struct QuotingOptions {
    style: QuotingStyle,
}
```

Implementation note:

- If the original C logic operates on arbitrary bytes, implement the core routine over `&[u8]` and append ASCII quote/escape bytes into a `Vec<u8>`.
- Convert to `String` only at the outer boundary if the quoting output is guaranteed ASCII-compatible; this is likely acceptable because escaped output should be valid UTF-8 once non-printable bytes are rendered textually.

## Implementation Phases

### Phase 1: Establish Rust module skeleton and API mapping

- Create or update `src/quotearg.rs`.
- Add Rust definitions for the quoting style representation required by `quotearg_style` and `quotearg_style_mem`.
- Define function signatures that fit the existing Rust project usage:
  - one style-based string entry point
  - one style-plus-length/bytes entry point
- Identify the exact C dependencies these functions rely on inside `quotearg.c` and fold in only the minimum helper logic needed for this branch.
- Replace C global/static buffer assumptions with owned Rust return values.

Exit criteria:

- The Rust module compiles with placeholder logic.
- All required types and function entry points exist and are wired into the crate.

### Phase 2: Port quoting logic and byte handling

- Implement the shared internal quoting routine.
- Port style dispatch from the C implementation into `match`-based Rust logic.
- For `quotearg_style_mem`, process input as `&[u8]` and preserve behavior for embedded NUL and non-UTF-8 bytes.
- For `quotearg_style`, route string input through the same core logic to avoid divergence.
- Translate C escaping rules into explicit byte/character emission using `push`, `push_str`, or `Vec<u8>` extension.
- Remove any dependence on mutable static storage patterns from the C implementation.

Memory and error handling decisions:

- Use capacity-aware allocation where input length gives a reasonable lower bound.
- Do not use unsafe code unless a direct equivalence issue makes it unavoidable; none is expected here.
- If UTF-8 conversion is needed at the API boundary, ensure the generated quoted representation is always valid text rather than exposing fallible conversion to callers.

Exit criteria:

- Both functions behave according to the migrated C logic.
- No global mutable state or C-style lifetime assumptions remain.

### Phase 3: Integrate with callers and align behavior

- Update the Rust-side callers in the `main_cluster` path to use the migrated `quotearg` functions.
- Confirm that style values passed from the main flow map correctly onto the Rust enum.
- Ensure return types match caller expectations without adding compatibility layers beyond what this branch needs.
- Remove or avoid any duplicate temporary implementations if present elsewhere on the branch.

Exit criteria:

- The branch builds end-to-end with the Rust quoting module in use.
- Callers no longer depend on C-side quoting behavior for these functions.

### Phase 4: Add focused tests and finalize parity checks

- Add unit tests in `src/quotearg.rs` or `tests/` covering:
  - representative quoting style selection
  - empty input
  - ASCII input without escaping
  - inputs requiring escaping or quoting delimiters
  - byte-oriented input for `_mem`, including embedded NUL and non-UTF-8 bytes if applicable
- Add regression tests for any cases observed in current `pwd` behavior that depend on these functions.
- Run `cargo test` and fix any mismatches in output formatting or edge handling.

Exit criteria:

- Tests cover the migrated behavior of both functions.
- `cargo test` passes on the branch.