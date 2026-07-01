# Implementation Plan: `main_root_quotearg.c_24`

## Summary

This module ports the quoting helpers currently implemented in `quotearg.c` into Rust, preserving the existing behavior and call patterns needed by the `pwd` project without adding new capabilities. The Rust implementation should focus on migrating the current entry points:

- `gettext_quote`
- `quotearg_buffer_restyled`
- `quotearg_free`
- `quotearg`
- `quotearg_mem`
- `quotearg_char`
- `quote_mem`
- `quote`

Technically, the port should translate the C string and buffer-oriented logic into safe Rust string/byte handling wherever possible, while keeping the implementation close to the existing control flow. The central work is to re-express the quoting algorithm over `&[u8]` and `String`/`Vec<u8>` buffers, with thin wrappers matching the current function family. Any C global or reusable slot-based temporary storage used by the original implementation should be migrated into a narrowly scoped Rust module state representation, minimizing `unsafe` and keeping ownership explicit.

The implementation should remain within a single Rust module corresponding to `quotearg.c`, with tests focused on output equivalence for representative quoting cases, buffer boundaries, and repeated calls that previously depended on C-managed memory.

## Technical Context

### Language / Version
- Rust stable, edition 2021
- Minimum recommended toolchain: `rustc 1.74+`

### Primary Dependencies
- Rust standard library only

No third-party crate is recommended from the provided inputs. The module’s needs—byte traversal, owned buffers, optional global cleanup/state replacement, and tests—can be handled with the standard library.

### Testing
- `cargo test`

Test coverage should include:
- direct quoting of UTF-8 and raw byte input where applicable
- empty input, single-character input, and embedded special characters
- wrapper functions delegating to the shared quoting core
- repeated invocations of allocation-returning helpers
- cleanup behavior for `quotearg_free` replacement semantics

### Performance Goals
- Maintain behaviorally equivalent linear-time quoting over input length
- Avoid unnecessary intermediate allocations where the C code writes into caller-provided buffers
- Use `String` for text output and `Vec<u8>` only where byte-preserving behavior is required
- Keep wrapper overhead negligible by routing all variants through one shared core implementation

## Module Mapping

### C to Rust File Mapping
- `quotearg.c` -> `src/quotearg.rs`

If the current Rust crate exposes module declarations from `src/lib.rs` or `src/main.rs`, add only the minimal corresponding:
- `mod quotearg;`

### Function Mapping
- `gettext_quote` -> `pub(crate) fn gettext_quote(...) -> ...`
- `quotearg_buffer_restyled` -> `pub(crate) fn quotearg_buffer_restyled(...) -> ...`
- `quotearg_free` -> `pub(crate) fn quotearg_free()`
- `quotearg` -> `pub(crate) fn quotearg(...) -> String` or borrowed/owned equivalent matching call sites
- `quotearg_mem` -> `pub(crate) fn quotearg_mem(input: &[u8]) -> String` or byte-preserving equivalent
- `quotearg_char` -> `pub(crate) fn quotearg_char(...) -> String`
- `quote_mem` -> `pub(crate) fn quote_mem(input: &[u8]) -> String`
- `quote` -> `pub(crate) fn quote(input: &str) -> String`

### Rust Module Scope
Keep all migrated logic in `src/quotearg.rs`. Do not split the algorithm, state, wrappers, and tests into additional production modules unless required by existing crate structure.

## Data Model

The source analysis lists only anonymous C data structures, so the Rust data model should be derived directly from actual usage in `quotearg.c` during implementation. The mapping below should be used as the migration guideline.

### Data-Structure Mapping

| C construct | Rust mapping | Notes |
|---|---|---|
| anonymous option/style struct(s) used to drive quoting behavior | `struct QuoteOptions` | Consolidate related flags, style selectors, and optional quote characters into one explicit Rust struct if the C code uses a shared option record. |
| anonymous enum-like style values | `enum QuotingStyle` | Use a Rust enum for style selection instead of integer constants. Preserve exact variants actually used by this file. |
| anonymous bitflag fields / booleans | `bool` fields in `QuoteOptions` | Prefer plain booleans over custom bitflag machinery unless the C logic depends on bitwise composition. |
| C string pointer inputs (`char *`, `const char *`) | `&str` when valid text is assumed; otherwise `&[u8]` | Use byte slices for the core quoting path if the C function accepts arbitrary memory. |
| C buffer output (`char *buf`, length arguments) | `&mut String` or `&mut Vec<u8>` | Choose based on whether the implementation must preserve non-UTF-8 bytes. |
| C heap-managed returned strings | `String` | Replace manual allocation/free with ownership-returning Rust values. |
| C static reusable slot arrays | `struct QuoteSlot { buf: String }` plus `Vec<QuoteSlot>` or module-local storage | Only if required by original API behavior. Keep representation minimal. |
| C global mutable state for quote slots | module-private state container | Prefer explicit state or `thread_local!` only if unavoidable for matching current call structure. Avoid broader infrastructure. |
| C null pointer / absent optional char | `Option<char>` or `Option<u8>` | Match whether the quoted content is handled as text or bytes. |
| C length counters (`size_t`) | `usize` | Direct mapping. |

### Memory Management Decisions
- Replace C-owned heap buffers with `String`/`Vec<u8>` ownership.
- Eliminate manual free logic internally; `quotearg_free` should only clear any retained module state if such state is still necessary after porting.
- Avoid raw pointers in public internal APIs unless required to bridge existing crate code.
- Keep allocation behavior predictable by reserving output capacity based on input length where practical.

### Error Handling Decisions
- The quoting operations should remain infallible under normal operation, returning owned output.
- If a direct C signature implies truncation/length reporting into caller buffers, model this with `usize` return values and safe buffer mutation.
- Do not introduce broad custom error enums unless an existing call site requires fallible behavior.

## Implementation Phases

## Phase 1: Translate Core Types and Quoting Engine
- Inspect `quotearg.c` and identify the actual anonymous structs, constants, and style selectors used by the listed functions.
- Create `src/quotearg.rs`.
- Define the minimal Rust equivalents for:
  - quoting style enum
  - options/state struct(s)
  - any retained slot storage required by wrappers
- Port `gettext_quote` and `quotearg_buffer_restyled` first, since they form the core selection and formatting logic.
- Implement the core algorithm over byte slices to preserve C behavior for memory-based inputs.
- Keep function shapes close to the C originals in internal helpers, then expose idiomatic Rust wrappers only where needed by the surrounding crate.

## Phase 2: Port Public Wrapper Functions and State Cleanup
- Port the wrapper family on top of the shared core:
  - `quotearg`
  - `quotearg_mem`
  - `quotearg_char`
  - `quote_mem`
  - `quote`
- Port `quotearg_free` last in this phase after deciding whether any reusable module state remains necessary.
- If the original C implementation used rotating/static quote slots, reproduce only the minimum equivalent behavior needed by current call sites.
- Remove any now-unnecessary manual memory lifecycle assumptions from the Rust side while preserving observable output behavior.

## Phase 3: Integrate with Existing Call Sites
- Replace references to the C-backed implementation with the Rust module in the `024-main_root_quotearg.c_24-rust-port` branch.
- Adjust signatures at call sites only as much as needed for Rust ownership and borrowing.
- Confirm that all migrated functions are used through the Rust module and that no placeholder stubs remain.
- Keep integration limited to the existing `pwd` project structure; do not introduce auxiliary abstraction layers.

## Phase 4: Verification and Cleanup
- Add focused unit tests in `src/quotearg.rs` or the crate’s existing test location for:
  - standard quoting output
  - memory-length-based quoting
  - character-triggered quoting behavior
  - empty and special-character inputs
  - repeated wrapper calls and cleanup via `quotearg_free`
- Run `cargo test` and resolve behavioral mismatches against the original C logic.
- Perform final cleanup to reduce `unsafe` to zero if possible, or isolate any unavoidable `unsafe` to the narrowest internal section with clear justification.