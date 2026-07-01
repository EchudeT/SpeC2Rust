# Implementation Plan

## Summary

Port the `quotearg.c` portion represented by `main_root_quotearg_style_13` into Rust on branch `014-main_root_quotearg_style_13-rust-port`, limited to the behavior surface exercised by:

- `quotearg_style`
- `quotearg_style_mem`

The Rust implementation should preserve the existing module boundary and migrate these functions into a focused Rust module that handles quoting-style-driven argument formatting without introducing new features or broader API redesign. The technical approach is to translate the relevant C control flow and style-selection logic into safe Rust using standard library string/byte handling, keeping ownership explicit and minimizing allocation churn where practical. Since one function accepts explicit memory/length input, the Rust API should operate on byte slices where needed and only convert to `String` when the quoting result is textual and valid under the original behavior assumptions.

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**:
  - Rust standard library
  - No third-party crates recommended based on the provided input
- **Testing**:
  - `cargo test`
- **Performance Goals**:
  - Preserve linear-time processing over input length
  - Avoid unnecessary intermediate allocations during quoting
  - Match C behavior closely for byte-slice input handling
  - Keep per-call overhead small and predictable for short command-line arguments

## Module Mapping

### C to Rust File Mapping

- `quotearg.c` → `src/quotearg.rs`

### Function Mapping

- `quotearg_style` → `pub(crate) fn quotearg_style(...) -> String`
- `quotearg_style_mem` → `pub(crate) fn quotearg_style_mem(...) -> String`

If the surrounding port already has a crate-level module organization for `cat`, expose these through the existing main-cluster path rather than creating additional abstraction layers.

## Data Model

The analysis lists only anonymous C data structures and does not identify named structs required directly by this slice. The migration should therefore keep the Rust data model minimal and tied only to what these two functions actually consume.

### Data-structure Mapping

- C anonymous quoting/style-related constants or tag sets → Rust `enum`
- C anonymous option/config aggregates used only for internal quoting decisions → Rust `struct` with private fields
- C string pointer plus explicit length patterns → Rust `&[u8]`
- C NUL-terminated string input patterns → Rust `&str` or `&[u8]`, depending on the exact call site being ported
- C mutable output buffers/internal temporaries → Rust `String` or `Vec<u8>`

### Recommended Rust Types

#### Quoting style selector
Use a Rust enum for the style argument if the original C code relies on a style constant set:

```rust
pub(crate) enum QuotingStyle {
    // variants added only as required by the migrated functions
}
```

This avoids integer-style dispatch errors and makes match-based translation direct.

#### Internal quoting options
If the C implementation depends on an internal options bundle even for these two functions, represent only the fields actually referenced by `quotearg_style` and `quotearg_style_mem`:

```rust
struct QuotingOptions {
    style: QuotingStyle,
    // only migrated fields that affect these paths
}
```

Do not recreate unrelated fields from `quotearg.c` unless required for compilation or behavior preservation.

#### Input and output representation
- For `quotearg_style_mem`: prefer `&[u8]` input to reflect C pointer-plus-length semantics exactly.
- For `quotearg_style`: prefer `&str` if the original path is strictly text input; otherwise route through `&[u8]`.
- For return values: use `String` if the resulting quoted form is textual by contract; if byte-preserving behavior is necessary internally, build in `Vec<u8>` and convert once at the boundary.

## Implementation Phases

## Phase 1: Establish module skeleton and type mapping

- Create `src/quotearg.rs`.
- Add the Rust equivalents for the quoting style selector needed by these functions.
- Identify from `quotearg.c` the smallest internal option/state representation necessary to support:
  - `quotearg_style`
  - `quotearg_style_mem`
- Define function signatures in Rust that reflect the original C usage patterns:
  - style-driven quoting entry point
  - style-driven quoting with explicit byte length
- Keep visibility restricted to the current crate/module needs (`pub(crate)` unless a broader existing interface requires otherwise).

### Deliverables
- Compiling module skeleton
- Enum/struct definitions for required style and options data
- Placeholder tests for module wiring

## Phase 2: Port `quotearg_style_mem` core logic

- Translate the explicit-memory variant first, since it is the lower-level function and best matches C semantics.
- Implement byte-slice traversal directly from the C logic:
  - preserve style-based branching
  - preserve escaping/quoting decisions
  - preserve handling of embedded non-NUL bytes according to length-driven input
- Use safe Rust iteration over `&[u8]`.
- Build output with `String` or `Vec<u8>` depending on whether any intermediate non-UTF-8 handling is needed.
- Resolve memory-management differences by replacing any C-managed temporary buffers with owned Rust output values.

### Error-handling approach
- Do not mirror C-style null-pointer handling unless the surrounding Rust call boundary still requires it.
- Represent invalid assumptions with narrow internal assertions only if they reflect impossible states after type conversion.
- Avoid introducing `Result` unless the translated behavior genuinely needs fallible output construction visible to callers.

### Deliverables
- Working Rust implementation of `quotearg_style_mem`
- Unit tests covering:
  - empty input
  - short plain input
  - input requiring quoting/escaping
  - explicit-length handling with embedded special bytes

## Phase 3: Port `quotearg_style` as the higher-level wrapper

- Implement `quotearg_style` on top of `quotearg_style_mem` where the C relationship permits.
- Map C string input semantics to the Rust boundary cleanly:
  - `&str` if input is guaranteed textual
  - otherwise convert from the existing byte-oriented representation used elsewhere in the port
- Ensure style dispatch remains identical to the C code path and does not add normalization or policy changes.
- Remove duplication by reusing the lower-level quoting path rather than re-implementing quoting rules.

### Deliverables
- Working Rust implementation of `quotearg_style`
- Tests proving wrapper parity with `quotearg_style_mem` on equivalent input

## Phase 4: Behavioral verification and integration cleanup

- Compare the Rust results against the C behavior for the migrated function set using focused test vectors derived from the original logic.
- Align any remaining edge behavior:
  - delimiter placement
  - escaping details
  - handling of zero-length and special-character inputs
- Integrate the module into the existing Rust port branch with only the required `mod`/`use` changes.
- Remove any temporary scaffolding not needed after migration.

### Deliverables
- Finalized `src/quotearg.rs`
- `cargo test` passing
- Minimal integration updates in the existing crate structure

## Notes on Memory Management and Error Handling

- Replace C buffer ownership and lifetime concerns with owned Rust return values.
- Prefer immutable input borrowing (`&[u8]`, `&str]`) and local output buffers.
- Avoid global mutable state unless it is already unavoidable in the current port architecture and directly required by these functions.
- Keep panic paths out of normal control flow; use typed inputs to eliminate C-era null and length mismatch issues where possible.
- Do not add compatibility layers beyond what is necessary to preserve the behavior of the two specified functions.