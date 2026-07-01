# Implementation Plan

## Summary

This module ports the `quotearg.c` functionality needed by `main_root_quotearg_custom_13`, specifically the existing `quotearg_custom` and `quotearg_custom_mem` entry points, into Rust with no added surface area beyond what the current call path requires.

The Rust implementation should preserve the current behavior of custom quoting by:
- translating the C string/memory-based quoting logic into safe Rust string/byte processing,
- keeping the distinction between NUL-terminated string input and explicit-length memory input,
- representing custom quoting parameters with compact Rust data types,
- avoiding global mutable buffers and C-style ownership patterns.

The technical approach is to migrate the needed logic from `quotearg.c` into a focused Rust module that operates primarily on `&str` / `&[u8]` and returns owned `String` values. Where the original C behavior depends on byte-oriented input, the Rust port should implement the core transformation over bytes first and only convert to `String` once the quoted output is assembled. This keeps behavior close to the source while avoiding unsafe memory handling except where unavoidable for boundary adaptation.

## Technical Context

- **Language/Version**: Rust 1.74+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve linear-time processing relative to input length.
  - Avoid unnecessary intermediate allocations by using `String::with_capacity` or `Vec<u8>` growth based on input size.
  - Keep parity with the C implementation’s practical throughput for single-pass quoting of short and medium command-line strings.
  - Do not introduce repeated UTF-8 validation during byte-wise quoting.

## Module Mapping

| C File | C Function | Rust Target | Notes |
|---|---|---|---|
| `quotearg.c` | `quotearg_custom` | `src/quotearg.rs::quotearg_custom` | Port as the string-oriented wrapper over the memory-length variant. |
| `quotearg.c` | `quotearg_custom_mem` | `src/quotearg.rs::quotearg_custom_mem` | Port as the core implementation for explicit-length input. |

Recommended project-local file mapping:

| Rust File | Responsibility |
|---|---|
| `src/quotearg.rs` | Direct port of the required quoting logic and local helper types/functions. |
| `src/main.rs` or existing caller module | Replace current C-backed use sites with calls into the Rust `quotearg` module. |

The migration should keep all logic in a single Rust module unless an existing repository layout already contains the destination module for quoting helpers.

## Data Model

The analysis only exposes anonymous C data structures, so the Rust plan should avoid inventing broad abstractions and instead introduce the minimum local representations required by these two functions.

| C Data Shape | Rust Mapping | Usage |
|---|---|---|
| anonymous struct holding custom quote delimiters | `struct CustomQuotes<'a> { left: &'a [u8], right: &'a [u8] }` | Encodes the custom opening and closing quote sequences needed by both functions. |
| anonymous raw string pointer input | `&str` or `&CStr` at boundary, normalized to `&[u8]` | Used for the NUL-terminated wrapper form. |
| anonymous raw memory pointer plus length | `&[u8]` | Used for explicit-length quoting. |
| anonymous result buffer / heap allocation state | `String` or `Vec<u8>` | Owns the produced quoted output. |
| anonymous option/config carrier if present in shared `quotearg.c` logic | narrow local struct or function parameters only | Only introduce if required to preserve internal branching already used by these two functions. |

### Memory Management

The C implementation likely relies on internal allocation and pointer-returning conventions. In Rust:
- prefer returning owned `String`,
- use borrowed byte slices for input,
- avoid exposing interior mutable buffers,
- keep allocation scoped to each call.

If the surrounding port requires compatibility with an existing signature, contain any conversion glue at the caller boundary rather than reproducing C allocation semantics throughout the module.

### Error Handling

The quoted output generation itself should be infallible under normal Rust allocation semantics. Therefore:
- core functions should return `String` where possible,
- only use `Result` if integration boundaries require validation that can fail, such as non-UTF-8 assumptions at a string-only API edge,
- prefer byte-oriented processing to avoid accidental UTF-8 decoding failures for arbitrary memory input.

## Implementation Phases

## Phase 1: Establish the Rust module skeleton

- Create `src/quotearg.rs` as the direct destination for the ported logic from `quotearg.c`.
- Add the public Rust equivalents of:
  - `quotearg_custom`
  - `quotearg_custom_mem`
- Define the minimal local data structure for custom left/right quote delimiters.
- Decide and document final Rust signatures based on current caller needs:
  - string-oriented wrapper for NUL-terminated input,
  - byte-slice-oriented core function for explicit-length input.
- Wire the module into the existing crate without adding unrelated helper modules.

## Phase 2: Port the core quoting logic

- Implement `quotearg_custom_mem` first as the primary logic path.
- Translate the C byte iteration and custom delimiter insertion into Rust byte-slice processing.
- Preserve the original escaping and delimiter placement behavior exactly for the covered paths.
- Build the output using a single owned buffer with predictable append operations.
- Keep helper routines private to `src/quotearg.rs` and limited to behavior already used by these functions.

Special attention in this phase:
- maintain the distinction between input bytes and output text,
- avoid indexing bugs when scanning memory ranges,
- ensure no reads past the provided slice length,
- avoid assuming UTF-8 for the source memory buffer.

## Phase 3: Add the wrapper function and integrate callers

- Implement `quotearg_custom` as a thin wrapper that forwards to `quotearg_custom_mem`.
- Convert caller sites in the `main_cluster` path to use the Rust functions from `src/quotearg.rs`.
- Remove or bypass the original dependency on the C implementation for this module’s call path.
- Keep the migration narrowly scoped to the functions listed for this module.

## Phase 4: Verify behavioral parity with targeted tests

- Add unit tests in the Rust module covering:
  - empty input,
  - ordinary ASCII input,
  - input containing characters that require escaping under the existing custom quoting rules,
  - asymmetric custom left/right delimiters,
  - explicit-length memory input including embedded non-printable or non-UTF-8 bytes where applicable.
- Add comparison-style tests derived from known current behavior in the existing project call path.
- Run `cargo test` and fix any output mismatches before completing the module migration.