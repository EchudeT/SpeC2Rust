# Implementation Plan

## Summary

This module ports the quoting-options and argument-quoting logic currently implemented in `quotearg.c` into Rust for the `cat` project branch `002-main_root_quoting_options_01-rust-port`.

The Rust implementation should preserve the existing behavior shape and call patterns of the C code, especially around:

- cloning and mutating quoting option sets,
- selecting quoting styles,
- setting per-character quoting behavior,
- handling custom quoting delimiters,
- producing quoted output into caller-provided buffers or newly allocated strings,
- supporting the numbered-slot quoting entry points.

The technical approach is to translate the C module into a single Rust module with a small set of Rust-native data types representing quoting style, flags, custom delimiters, and option state. Functions that in C mutate or return heap-allocated buffers will be mapped to Rust APIs based on `String`, `Vec<u8>`, and borrowed slices where appropriate. Where the original C API implies global or persistent numbered buffers, the Rust port should keep the implementation narrowly scoped to the existing module behavior rather than introducing broader abstractions.

The migration should focus on preserving semantics from the listed functions and existing file boundaries, not on redesigning the quoting subsystem.

## Technical Context

### Language / Version

- Rust 1.78 or newer
- Edition: 2021

### Primary Dependencies

Use the Rust standard library by default.

Recommended dependencies:
- No third-party crates initially

Rationale:
- The listed functionality can be implemented with `std` types such as `String`, `Vec<u8>`, arrays, enums, and `Option`.
- There is no explicit evidence in the input requiring external parsing, Unicode libraries, or bitflag helper crates.

### Testing

- `cargo test`

Test scope should cover:
- option cloning and mutation behavior,
- style selection and style-derived option creation,
- custom quoting delimiter validation,
- quoting output for representative byte inputs,
- equivalence across buffer-based and allocation-based entry points,
- numbered-slot behavior for repeated calls.

### Performance Goals

- Preserve linear-time processing over the input buffer.
- Avoid unnecessary intermediate allocations in buffer-oriented APIs.
- Ensure allocation-based functions allocate proportionally to output size only.
- Keep per-call overhead low enough to match the C module’s role in command-line text formatting rather than introducing heavy abstraction layers.

## Module Mapping

### Source File Mapping

- C: `quotearg.c`
- Rust: `src/main_root_quoting_options_01.rs`

If the target crate already has a main-cluster layout, this file should be included from the existing module tree without creating extra architectural layers beyond what is needed to host the port.

### Function Mapping

Each C function should map closely to a Rust function in the same Rust module:

- `clone_quoting_options`
  - Rust: `fn clone_quoting_options(opts: &QuotingOptions) -> QuotingOptions`
- `get_quoting_style`
  - Rust: `fn get_quoting_style(opts: &QuotingOptions) -> QuotingStyle`
- `set_quoting_style`
  - Rust: `fn set_quoting_style(opts: &mut QuotingOptions, style: QuotingStyle)`
- `set_char_quoting`
  - Rust: `fn set_char_quoting(opts: &mut QuotingOptions, ch: u8, should_quote: bool) -> bool`
- `set_quoting_flags`
  - Rust: `fn set_quoting_flags(opts: &mut QuotingOptions, flags: QuotingFlags) -> QuotingFlags`
- `set_custom_quoting`
  - Rust: `fn set_custom_quoting(opts: &mut QuotingOptions, left: &[u8], right: &[u8])`
- `quoting_options_from_style`
  - Rust: `fn quoting_options_from_style(style: QuotingStyle) -> QuotingOptions`
- `quotearg_buffer`
  - Rust: `fn quotearg_buffer(dst: &mut Vec<u8>, src: &[u8], opts: &QuotingOptions) -> usize`
- `quotearg_alloc`
  - Rust: `fn quotearg_alloc(src: &[u8], opts: &QuotingOptions) -> Vec<u8>` or `String` if the original behavior is text-only after validation
- `quotearg_alloc_mem`
  - Rust: `fn quotearg_alloc_mem(src: &[u8], opts: &QuotingOptions) -> Vec<u8>`
- `quotearg_n_options`
  - Rust: `fn quotearg_n_options(slot: usize, src: &[u8], opts: &QuotingOptions) -> Vec<u8>` or borrowed result if existing crate architecture already supports stable slot storage
- `quotearg_n_style`
  - Rust: `fn quotearg_n_style(slot: usize, style: QuotingStyle, src: &[u8]) -> Vec<u8>`
- `quotearg_n_style_mem`
  - Rust: `fn quotearg_n_style_mem(slot: usize, style: QuotingStyle, src: &[u8]) -> Vec<u8>`
- `quotearg_char_mem`
  - Rust: `fn quotearg_char_mem(ch: u8, src: &[u8]) -> Vec<u8>`
- `quotearg_n_style_colon`
  - Rust: `fn quotearg_n_style_colon(slot: usize, style: QuotingStyle, src: &[u8]) -> Vec<u8>`

Notes:
- Final signatures may need minor adjustment to fit the surrounding crate’s established conventions.
- For functions historically returning NUL-terminated C strings, the Rust implementation should return owned byte/string buffers without embedded terminator management unless another migrated caller still depends on explicit terminators.

## Data Model

The C analysis lists anonymous structures only, so the Rust data model should be reconstructed from function behavior rather than by trying to preserve unnamed C layout literally.

### Core Rust Types

#### QuotingStyle

Map the C quoting-style discriminator to a Rust enum:

```rust
enum QuotingStyle {
    Literal,
    Shell,
    ShellAlways,
    C,
    Escape,
    Locale,
    Clocale,
    Custom,
}
```

Exact variants should match only the styles actually referenced by this module and its callers in the existing codebase. Do not add styles that are not present in the migrated source.

#### QuotingFlags

Map C bitwise flags to a compact integer wrapper:

```rust
#[derive(Clone, Copy, PartialEq, Eq)]
struct QuotingFlags(u32);
```

Implementation notes:
- Use associated constants for known flag bits.
- Keep bitwise manipulation explicit with helper methods if needed.
- Avoid adding a third-party bitflags crate unless another existing ported module already standardizes on it.

#### CustomQuoting

Represent custom left/right delimiters separately from the main option set’s style enum:

```rust
#[derive(Clone, PartialEq, Eq)]
struct CustomQuoting {
    left: Vec<u8>,
    right: Vec<u8>,
}
```

Use owned byte buffers because:
- the C version may store pointers whose lifetime is managed externally,
- the Rust port should avoid unsafe borrowed-lifetime coupling for mutable option objects,
- delimiters may not be valid UTF-8.

#### QuotingOptions

Primary state object:

```rust
#[derive(Clone, PartialEq, Eq)]
struct QuotingOptions {
    style: QuotingStyle,
    flags: QuotingFlags,
    quote_these_too: [bool; 256],
    custom: Option<CustomQuoting>,
}
```

Mapping intent:
- C per-character quoting table/bitset -> fixed `[bool; 256]` or a compact byte/bit representation.
- Prefer `[bool; 256]` first for clarity during migration.
- If the original C module uses a denser bitset and preserving that representation simplifies exact behavior, `[u32; N]` or `[u8; 32]` is also acceptable, but should remain internal.

#### Numbered Slot Storage

If `quotearg_n_*` requires persistent slot-based storage matching C semantics, use an internal module-local structure such as:

```rust
struct QuoteSlot {
    buf: Vec<u8>,
}
```

and keep a growable collection of slots:

```rust
Vec<QuoteSlot>
```

This storage should remain private to the module and should only exist if required to preserve caller-visible behavior from the original C API. Do not generalize it into a reusable cache layer.

### Data-Structure Mapping Summary

- C anonymous quoting options struct -> `QuotingOptions`
- C anonymous custom quote delimiter storage -> `CustomQuoting`
- C quoting style integer/enum -> `QuotingStyle`
- C quoting flags bitmask -> `QuotingFlags`
- C per-character quote map / bitset -> `[bool; 256]` inside `QuotingOptions`
- C static numbered argument slots -> private `Vec<QuoteSlot>` if needed

### Memory Management Decisions

- Replace C heap allocation and manual free patterns with owned Rust values.
- Buffer-producing functions should write into `Vec<u8>` or return owned buffers.
- Cloning options should perform deep copies of custom delimiters and quote maps.
- Avoid `unsafe` unless it becomes strictly necessary for compatibility with surrounding code; none is expected from the listed functions alone.

### Error Handling Decisions

- Preserve infallible setters where the C logic assumes valid state transitions.
- For `set_custom_quoting`, validate delimiter inputs according to the original C behavior:
  - if empty delimiters are invalid in C, represent that with `assert!`, `debug_assert!`, or a `Result` depending on existing crate conventions.
- Keep public behavior aligned with current callers; do not introduce broad error enums without evidence of caller need.
- If invalid style/custom combinations are impossible in the reconstructed Rust types, encode those constraints structurally instead of deferring to runtime errors.

## Implementation Phases

## Phase 1: Establish Rust data types and direct option operations

### Goals

Create the Rust module scaffold and port the option-state manipulation functions first so that quoting execution code can build on stable state objects.

### Tasks

- Create `src/main_root_quoting_options_01.rs`.
- Define:
  - `QuotingStyle`
  - `QuotingFlags`
  - `CustomQuoting`
  - `QuotingOptions`
- Implement:
  - `clone_quoting_options`
  - `get_quoting_style`
  - `set_quoting_style`
  - `set_char_quoting`
  - `set_quoting_flags`
  - `set_custom_quoting`
  - `quoting_options_from_style`
- Reconstruct any style defaults embedded in the C file directly in this module.
- Add unit tests for:
  - clone independence,
  - style round-trip behavior,
  - flag replacement behavior,
  - per-character toggle behavior,
  - custom delimiter persistence.

### Exit Criteria

- All option-related functions compile and pass `cargo test`.
- The Rust option model is sufficient to express all states required by the remaining quoting functions.

## Phase 2: Port core quoting engine and allocation/buffer entry points

### Goals

Implement the byte-processing logic once and route buffer-based and allocation-based APIs through it.

### Tasks

- Identify the central quoting algorithm in `quotearg.c`.
- Implement a single internal Rust routine that:
  - scans `src: &[u8]`,
  - applies style/flag/per-character/custom-delimiter rules,
  - appends output into a caller-supplied `Vec<u8>`,
  - returns produced length.
- Port:
  - `quotearg_buffer`
  - `quotearg_alloc`
  - `quotearg_alloc_mem`
- Ensure byte-oriented behavior is preserved for non-UTF-8 input.
- Keep NUL bytes and arbitrary byte sequences representable where the C APIs support them.
- Add tests covering:
  - empty input,
  - ASCII input,
  - inputs containing bytes that must be additionally quoted,
  - custom delimiter style,
  - consistency between `quotearg_buffer` and allocation variants.

### Exit Criteria

- Core quoting behavior exists in one internal implementation path.
- Buffer and allocation entry points produce matching output for the same options and input.

## Phase 3: Port numbered-slot wrappers and style convenience functions

### Goals

Complete the remaining public surface by migrating the convenience wrappers and any required slot retention behavior.

### Tasks

- Implement any required private slot storage for `quotearg_n_*` behavior.
- Port:
  - `quotearg_n_options`
  - `quotearg_n_style`
  - `quotearg_n_style_mem`
  - `quotearg_char_mem`
  - `quotearg_n_style_colon`
- Reuse `quoting_options_from_style` rather than duplicating style initialization logic.
- Ensure colon-specific behavior is expressed by mutating only the relevant quoted-character setting, matching the C call path.
- Add tests for:
  - repeated calls on the same slot,
  - multiple slots,
  - style wrapper equivalence to explicit options,
  - colon wrapper behavior,
  - single-character forced quoting behavior.

### Exit Criteria

- All listed functions are ported.
- Wrapper functions are thin and delegate to the core implementation without duplicated quoting logic.

## Phase 4: Integration cleanup and parity verification

### Goals

Finalize the migration by aligning signatures with surrounding crate expectations and confirming the Rust module can replace the C implementation within the branch.

### Tasks

- Adjust visibility and module exports to match current Rust crate usage.
- Replace any temporary placeholder representations with the minimal final forms required by callers.
- Verify all migrated functions are used through the Rust module instead of the C file in this branch.
- Add regression tests for behavior combinations most likely to diverge from C:
  - flag plus style interactions,
  - custom delimiter plus non-literal input,
  - numbered-slot wrappers with repeated mutation of options.
- Remove dead code and temporary compatibility helpers introduced during porting.

### Exit Criteria

- The module is fully represented in Rust in branch `002-main_root_quoting_options_01-rust-port`.
- `cargo test` passes with the migrated module wired into the project.
- No extra module layers or unevidenced support facilities have been introduced.