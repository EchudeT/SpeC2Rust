# Implementation Plan

## Summary

Port the `quotearg.c` subset for `quotearg_colon` and `quotearg_colon_mem` into a Rust module that preserves current quoting behavior and call patterns required by the `cat` main-cluster work. The implementation should stay narrowly scoped to these two functions and only migrate the logic and local data dependencies they require from the existing C source.

The Rust approach should:
- translate the quoting rules used by the colon-specialized entry points,
- represent input as `&str` or `&[u8]` depending on whether the original C path is string-length or explicit-memory based,
- return owned Rust string/byte output without manual buffer ownership,
- keep internal helper logic private to the module unless a direct caller requires exposure.

Memory safety will come from replacing C pointer arithmetic and shared static buffer patterns with slice-based processing and owned output types. Error handling should remain minimal and explicit: inputs that are not valid UTF-8 should be handled through byte-oriented paths rather than lossy conversion.

## Technical Context

- **Language/Version**: Rust 1.75 or newer
- **Primary Dependencies**:
  - Rust standard library
  - No third-party crates recommended based on current evidence
- **Testing**: `cargo test`
- **Performance Goals**:
  - Maintain linear-time processing relative to input length
  - Avoid unnecessary intermediate allocations where practical
  - Preserve a single-pass or near-single-pass transformation strategy
  - Keep behavior suitable for command-line text handling without introducing heavyweight abstractions

## Module Mapping

- **C source file**: `quotearg.c`
- **Rust target module**: `src/quotearg.rs`

Function mapping:
- `quotearg_colon` -> `pub fn quotearg_colon(input: &str) -> String`
- `quotearg_colon_mem` -> `pub fn quotearg_colon_mem(input: &[u8]) -> Vec<u8>` or `String` if all call sites and behavior are confirmed UTF-8-safe

Mapping decision notes:
- Prefer `&str -> String` for the null-terminated C string variant.
- Prefer `&[u8]` for the explicit-length variant because the C API accepts arbitrary memory.
- If project call sites require textual output only and source behavior is UTF-8 constrained, the byte-returning function may be narrowed later, but initial migration should preserve byte-level correctness.

## Data Model

The analysis identifies only anonymous C data structures, with no named exported types tied directly to the requested functions. The Rust port should avoid inventing broad new types unless needed to mirror local option/state used by these two functions.

Proposed mapping:

- **Anonymous local configuration/state structs in `quotearg.c`**
  - **Rust mapping**: private module-level `struct` or `enum` only if required to encode the quoting style/options used internally by `quotearg_colon` and `quotearg_colon_mem`
  - If the colon functions only delegate to existing option constants in C, model that in Rust as:
    - a small private `enum QuotingStyle`
    - a small private `struct QuotingOptions`
  - Do not migrate unrelated fields or styles not needed by these two functions

- **C string / memory buffer parameters**
  - `char const *` -> `&str` for null-terminated string entry points where UTF-8 is guaranteed by caller context
  - `char const *` + `size_t` -> `&[u8]` for explicit-memory entry points

- **C output buffers / static return storage**
  - `char *` returned from managed/static buffers -> `String` or `Vec<u8>`
  - This removes shared mutable storage and implicit lifetime hazards

- **C integral flags / character classification state**
  - `int`, `bool-like flags` -> `bool`, `u8`, or small Rust enums as appropriate
  - Character tests should use byte comparisons where byte-preserving behavior matters

## Implementation Phases

### Phase 1: Isolate and map the required quoting path

- Inspect `quotearg.c` and identify the exact internal helpers, constants, and option data touched by:
  - `quotearg_colon`
  - `quotearg_colon_mem`
- Determine whether these functions are thin wrappers over a generic quoting engine or contain inline logic.
- Create `src/quotearg.rs`.
- Port only the minimum private constants, helper functions, and option representations needed for the colon-specific behavior.
- Define the initial Rust signatures to preserve C semantics as closely as practical:
  - `quotearg_colon(&str) -> String`
  - `quotearg_colon_mem(&[u8]) -> Vec<u8>` as the conservative first step

### Phase 2: Implement byte-safe quoting behavior

- Translate the core escaping/quoting logic used by the colon variant into slice-based Rust code.
- Replace pointer arithmetic with indexed or iterator-based traversal over input bytes.
- Use owned output buffers:
  - `String` when constructing validated text
  - `Vec<u8>` when preserving arbitrary byte content
- Keep helper functions private and narrowly scoped to this module.
- Ensure special handling for `:` matches the original C behavior exactly.
- Avoid implicit UTF-8 assumptions in the `_mem` path.

### Phase 3: Integrate with existing callers

- Update the Rust main-cluster code on branch `012-main_root_quotearg_colon_11-rust-port` to call the new module functions instead of any placeholder or untranslated path.
- If the surrounding code expects textual output from both functions, add a thin conversion only at the call boundary where validity is known, rather than weakening the core byte-safe API.
- Confirm no extra public API is exposed beyond what current migrated callers need.

### Phase 4: Verify correctness and clean up

- Add focused unit tests in the module covering:
  - empty input
  - input containing `:`
  - inputs with characters that require quoting/escaping under the inherited C logic
  - byte-oriented `_mem` cases, including non-UTF-8 bytes if applicable
- Add comparison-style tests against expected outputs derived from the C implementation behavior for representative cases.
- Remove leftover C-style assumptions such as static temporary storage or sentinel-based buffer handling.
- Run `cargo test` and finalize small refactors only where they reduce risk or improve direct fidelity to the source logic.