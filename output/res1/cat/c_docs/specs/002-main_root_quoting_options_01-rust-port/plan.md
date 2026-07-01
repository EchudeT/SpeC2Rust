# Implementation Plan: main_root_quoting_options_01

## Summary

This module ports the quoting option management and argument quoting entry points currently implemented in `quotearg.c` into Rust for the `cat` project branch `002-main_root_quoting_options_01-rust-port`.

The Rust implementation should preserve the existing module’s role: maintaining quoting configuration, producing derived quoting-option instances, and exposing the current set of quoting-oriented helper functions used by the main program cluster. The port should focus on migrating the existing file and listed functions only, without introducing new capabilities or reorganizing unrelated code.

Technically, the Rust port should:
- Represent quoting configuration with explicit Rust structs and enums rather than anonymous C structs.
- Translate C-style mutable option updates into owned/mutable Rust APIs.
- Replace manual allocation and buffer ownership rules with `String`, `Vec<u8>`, and slice-based interfaces as appropriate.
- Preserve behavior distinctions among style selection, flag mutation, custom quoting setup, and buffer/allocation-based quoting functions.
- Keep module boundaries narrow: one Rust module corresponding to `quotearg.c`, with tests covering migrated behavior.

## Technical Context

### Language / Version
- Rust stable, edition 2021
- Minimum recommended compiler: `rustc 1.75+`

### Primary Dependencies
- Rust standard library only

No third-party crates are recommended from the provided evidence. The listed functionality can be implemented with:
- `std::borrow`
- `std::fmt`
- `std::mem`
- `std::ops`
- `std::string::String`
- `std::vec::Vec`

### Testing
- `cargo test`

Testing should cover:
- style getter/setter behavior
- quoting flag updates
- per-character quoting configuration
- custom quoting delimiter validation and storage
- buffer-based and allocation-based quoting entry points
- equivalence among helper wrappers such as style-specific and colon-specific variants

### Performance Goals
- Match the C module’s practical runtime characteristics for short command-line argument quoting.
- Avoid unnecessary heap allocations in buffer-oriented paths.
- Keep cloning of quoting options explicit and limited to call sites that require ownership.
- Preserve predictable linear behavior with respect to input length.
- Avoid extra passes over the input beyond what is needed to compute quoted output.

## Module Mapping

### C to Rust File Mapping
- `quotearg.c` → `src/quotearg.rs`

If the existing Rust crate already exposes a central module list, add only the corresponding module declaration required to compile `src/quotearg.rs`. Do not create extra abstraction layers.

### Function Mapping
Each C function should be migrated into the Rust `quotearg` module with close naming and responsibility alignment.

- `clone_quoting_options`
  - Rust: `pub fn clone_quoting_options(opts: &QuotingOptions) -> QuotingOptions`
  - Notes: straightforward owned clone, replacing C heap/object duplication semantics.

- `get_quoting_style`
  - Rust: `pub fn get_quoting_style(opts: &QuotingOptions) -> QuotingStyle`

- `set_quoting_style`
  - Rust: `pub fn set_quoting_style(opts: &mut QuotingOptions, style: QuotingStyle)`

- `set_char_quoting`
  - Rust: `pub fn set_char_quoting(opts: &mut QuotingOptions, ch: u8, should_quote: bool) -> bool`
  - Notes: return previous setting if the C API did so; preserve that contract in Rust.

- `set_quoting_flags`
  - Rust: `pub fn set_quoting_flags(opts: &mut QuotingOptions, flags: QuotingFlags) -> QuotingFlags`
  - Notes: return prior flags if required by current behavior.

- `set_custom_quoting`
  - Rust: `pub fn set_custom_quoting(opts: &mut QuotingOptions, left: String, right: String)`
  - Notes: enforce any required non-empty delimiter assumptions during migration.

- `quoting_options_from_style`
  - Rust: `pub fn quoting_options_from_style(style: QuotingStyle) -> QuotingOptions`

- `quotearg_buffer`
  - Rust: `pub fn quotearg_buffer(dst: &mut [u8], arg: &[u8], opts: &QuotingOptions) -> usize`
  - Notes: return produced/required length according to current C semantics; keep exact contract explicit in code comments.

- `quotearg_alloc`
  - Rust: `pub fn quotearg_alloc(arg: &[u8], opts: &QuotingOptions) -> String`

- `quotearg_alloc_mem`
  - Rust: `pub fn quotearg_alloc_mem(arg: &[u8], opts: &QuotingOptions) -> Vec<u8>`
  - Notes: use byte-preserving output if the C function is length-aware rather than NUL-terminated string-oriented.

- `quotearg_n_options`
  - Rust: `pub fn quotearg_n_options(n: usize, arg: &[u8], opts: &QuotingOptions) -> String`
  - Notes: if the C implementation uses slot reuse, replicate only the observable behavior needed by current callers; keep any slot storage internal to this module if still required.

- `quotearg_n_style`
  - Rust: `pub fn quotearg_n_style(n: usize, style: QuotingStyle, arg: &[u8]) -> String`

- `quotearg_n_style_mem`
  - Rust: `pub fn quotearg_n_style_mem(n: usize, style: QuotingStyle, arg: &[u8]) -> Vec<u8>`

- `quotearg_char_mem`
  - Rust: `pub fn quotearg_char_mem(arg: &[u8], ch: u8) -> Vec<u8>`

- `quotearg_n_style_colon`
  - Rust: `pub fn quotearg_n_style_colon(n: usize, style: QuotingStyle, arg: &[u8]) -> String`
  - Notes: implement as a thin wrapper over style-based quoting plus `:` character quoting setup, mirroring the C layering.

## Data Model

The source analysis reports only anonymous C data structures. For the Rust port, introduce named types only where needed to represent the existing state clearly.

### Data-structure Mapping

| C construct | Rust mapping | Notes |
|---|---|---|
| anonymous quoting options struct | `pub struct QuotingOptions` | Central mutable configuration object for quoting behavior. |
| anonymous quoting style representation | `pub enum QuotingStyle` | Enumerates the styles used by the migrated functions. Include only variants evidenced by the existing C file during implementation. |
| anonymous quoting flags field/bitmask | `#[derive(Clone, Copy, PartialEq, Eq)] pub struct QuotingFlags(u32)` | Newtype over bitmask storage to preserve C flag behavior without exposing raw integers everywhere. |
| anonymous custom quoting storage | `pub struct CustomQuoting { left: String, right: String }` | Holds custom delimiters for the custom style/path. |
| anonymous per-character quoting table | `[bool; 256]` or compact bitset inside `QuotingOptions` | Prefer fixed-size array for direct C-table correspondence and simple indexing. |
| anonymous slot/cache storage used by `quotearg_n_*` family | private module-local slot container | Keep private; model only if required by call semantics. |

### Proposed Rust Types

```rust
pub enum QuotingStyle {
    // concrete variants added from quotearg.c during migration
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct QuotingFlags(u32);

#[derive(Clone, PartialEq, Eq)]
pub struct CustomQuoting {
    pub left: String,
    pub right: String,
}

#[derive(Clone)]
pub struct QuotingOptions {
    pub style: QuotingStyle,
    pub flags: QuotingFlags,
    pub quote_these_too: [bool; 256],
    pub custom: Option<CustomQuoting>,
}
```

### Memory Management Decisions
- Replace C allocation ownership with Rust-owned values.
- Use `Clone` for option duplication instead of manual memory copying.
- Use `String` only for text-oriented outputs that are guaranteed to be valid under the existing function contract.
- Use `Vec<u8>` for memory/length-oriented functions where embedded NUL or non-UTF-8 data must remain representable.
- For `quotearg_buffer`, accept a mutable destination slice and return the written or needed length per migrated semantics rather than exposing raw pointers.

### Error Handling Decisions
- Prefer infallible APIs where the C behavior is configuration-only and cannot fail under normal Rust invariants.
- For invalid custom quoting configuration discovered during migration, use narrow validation:
  - either `assert!` if the original C code treated this as programmer error,
  - or return a small local error type only if the current callers already depend on recoverable failure.
- Do not broaden the API surface with generalized error frameworks.

## Implementation Phases

### Phase 1: Port core option state and mutators
Scope:
- Create `src/quotearg.rs`
- Define `QuotingStyle`
- Define `QuotingFlags`
- Define `CustomQuoting`
- Define `QuotingOptions`
- Implement:
  - `clone_quoting_options`
  - `get_quoting_style`
  - `set_quoting_style`
  - `set_char_quoting`
  - `set_quoting_flags`
  - `set_custom_quoting`
  - `quoting_options_from_style`

Technical goals:
- Establish exact field mapping from the C state layout into Rust-owned types.
- Preserve previous-value return behavior for setters where applicable.
- Keep custom quoting state optional and style-dependent, without adding unrelated configuration features.

Validation:
- Unit tests for default option construction and all mutator functions.
- Tests confirming clone independence from the original mutable instance.

### Phase 2: Port the core quoting engine and direct allocation APIs
Scope:
- Implement the internal quoting routine used by all public entry points.
- Implement:
  - `quotearg_buffer`
  - `quotearg_alloc`
  - `quotearg_alloc_mem`

Technical goals:
- Centralize transformation logic in one internal routine to avoid divergence across wrappers.
- Support byte-oriented input directly from `&[u8]`.
- Keep buffer sizing and output length behavior aligned with the C implementation.

Validation:
- Tests for empty input, plain input, special-character quoting, and custom delimiter behavior.
- Tests comparing `quotearg_buffer` output with `quotearg_alloc` / `quotearg_alloc_mem` for the same options.

### Phase 3: Port indexed/style wrapper functions
Scope:
- Implement:
  - `quotearg_n_options`
  - `quotearg_n_style`
  - `quotearg_n_style_mem`
  - `quotearg_char_mem`
  - `quotearg_n_style_colon`

Technical goals:
- Reproduce the wrapper layering from `quotearg.c` with minimal additional structure.
- Introduce internal slot/index handling only if required by observable semantics of `quotearg_n_options`.
- Keep style-specialized helpers thin and derived from the core implementation.

Validation:
- Tests ensuring wrapper functions match the corresponding base functions.
- Tests for colon-specific quoting behavior.
- Tests for repeated calls with the same index if slot semantics exist in the original module.

### Phase 4: Integration cleanup and behavior parity review
Scope:
- Wire the module into the crate.
- Adjust calling sites in the main cluster only as needed to use the Rust APIs.
- Remove or isolate replaced C-side references for this module on the branch.

Technical goals:
- Confirm no extra modules or utility layers were introduced beyond the direct port.
- Check that ownership, mutability, and output-type choices fit existing call patterns.
- Review edge cases from the C implementation, especially byte handling and custom quoting.

Validation:
- `cargo test`
- Focused regression tests for any main-cluster call paths affected by the module port.