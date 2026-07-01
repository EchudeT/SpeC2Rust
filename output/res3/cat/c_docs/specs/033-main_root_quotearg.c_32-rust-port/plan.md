# Implementation Plan: main_root_quotearg.c_32

## Summary

This module ports the `quotearg.c` functionality used by `cat` for producing quoted representations of strings and byte buffers. The Rust implementation should preserve the existing function-level behavior and migration boundaries, focusing on the current exported routines:

- `gettext_quote`
- `quotearg_buffer_restyled`
- `quotearg_free`
- `quotearg`
- `quotearg_mem`
- `quotearg_char`
- `quote_mem`
- `quote`

Technical approach:

- Migrate the logic from `quotearg.c` into a single Rust module that keeps the same conceptual responsibilities: quote style selection, buffer-based formatting, and convenience wrappers over owned or borrowed inputs.
- Prefer Rust standard library types for ownership and buffer construction:
  - `&[u8]` / `&str` for inputs
  - `Vec<u8>` / `String` for produced output
- Isolate any C-style global or reusable state behind module-private Rust structures rather than reproducing raw allocation patterns directly.
- Preserve byte-oriented behavior where the C code operates on arbitrary memory (`quotearg_mem`, `quote_mem`, `quotearg_buffer_restyled`), and only convert to `String` where the original call path is text-oriented and valid UTF-8 is required by surrounding Rust code.
- Replace manual freeing behavior with Rust ownership; `quotearg_free` should become a narrow compatibility layer for clearing module-managed cached allocations only if such cache/state is required by the migrated logic.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Match the C implementation’s asymptotic behavior for buffer scanning and quoting.
  - Avoid unnecessary intermediate allocations during buffer restyling.
  - Use pre-sized `Vec<u8>`/`String` buffers where output growth is predictable.
  - Preserve efficient handling of non-UTF-8 byte input without lossy conversion.

## Module Mapping

### C to Rust File Mapping

- `quotearg.c` → `src/quotearg.rs`

### Function Mapping

- `gettext_quote` → `pub(crate) fn gettext_quote(...) -> ...`
- `quotearg_buffer_restyled` → `pub(crate) fn quotearg_buffer_restyled(...) -> ...`
- `quotearg_free` → `pub(crate) fn quotearg_free()`
- `quotearg` → `pub(crate) fn quotearg(...) -> String` or `Vec<u8>` depending on caller needs
- `quotearg_mem` → `pub(crate) fn quotearg_mem(input: &[u8], ...) -> Vec<u8>` or `String` wrapper
- `quotearg_char` → `pub(crate) fn quotearg_char(...) -> ...`
- `quote_mem` → `pub(crate) fn quote_mem(input: &[u8]) -> Vec<u8>` or text wrapper
- `quote` → `pub(crate) fn quote(input: &str) -> String`

### Rust Module Scope

Keep all migrated logic in one Rust source file initially:

- `src/quotearg.rs`

If integration requires visibility from existing `cat` entry logic, expose only the minimal functions needed through the crate’s existing module declarations. Do not split helper modules unless required by compilation or existing project structure.

## Data Model

The C analysis only exposes anonymous data structures, so the Rust data model should be reconstructed from actual use sites in `quotearg.c` rather than inventing broader abstractions.

### Expected Mapping Strategy

| C construct | Rust mapping |
| --- | --- |
| anonymous option/config structs | named `struct` types with private fields |
| anonymous style/discriminant data | `enum QuoteStyle` or similar |
| raw character buffer + length | `&[u8]`, `Vec<u8>`, or `String` depending on function boundary |
| static/global reusable slot state | module-private `static`/`thread_local!` only if required by original semantics; otherwise explicit ownership |
| bitsets / flags for quoting behavior | integer flags or small wrapper struct, kept private |
| nullable pointers | `Option<T>` / `Option<&T>` / `Option<Box<T>>` |
| returned heap-allocated C strings | owned `String` or `Vec<u8>` |

### Planned Rust Structures

Because the source struct names are not available from the analysis, define only the minimum set of named Rust types needed to mirror the original behavior:

- `QuoteStyle`
  - Rust enum representing the quoting mode selected by the C logic.
- `QuotingOptions`
  - Rust struct holding the option set consumed by the buffer-restyling path.
- `Slot` or equivalent private cache record
  - Only if the original `quotearg` family depends on reusable allocation slots.
- `QuoteResult`
  - Not required unless needed to carry both produced length and output bytes cleanly during migration; otherwise return direct owned buffers plus lengths where needed.

### Memory Management Decisions

- Eliminate manual `malloc`/`free` ownership in favor of Rust-owned outputs.
- Preserve explicit output-length computation where the C API writes into caller-provided buffers.
- For `quotearg_free`, map C cleanup semantics to clearing any module-private cached buffers if such buffers are preserved during the port. If no cache survives the Rust design, implement it as a no-op compatibility function and keep it only if referenced by migrated callers.
- Avoid exposing borrowed references to temporary internal storage that would mimic unsafe C lifetime patterns.

### Error Handling Decisions

- Prefer infallible APIs where the C code assumes successful quoting into dynamically grown storage.
- For text-returning wrappers, avoid unchecked UTF-8 conversion if arbitrary bytes are possible; keep the core byte-oriented and add text wrappers only where input is guaranteed textual.
- If an internal helper must signal invalid assumptions, use `Result` internally and convert at the outer compatibility boundary only where needed by existing call sites.

## Implementation Phases

## Phase 1: Establish the Rust module skeleton and migrate core types

- Create `src/quotearg.rs`.
- Read `quotearg.c` and identify:
  - quote style representation
  - option/config structure layout
  - any persistent slot/cache state used by `quotearg` and `quotearg_free`
- Introduce minimal Rust equivalents:
  - `QuoteStyle`
  - `QuotingOptions`
  - private helpers for character classification and quote emission
- Wire the module into the crate with crate-private visibility matching current use.
- Decide function signatures based on actual caller requirements in the Rust branch:
  - byte-oriented signatures for memory-based functions
  - string-oriented wrappers only where callers require `String`

## Phase 2: Port the core buffer transformation path

- Port `gettext_quote` with the narrowest Rust representation needed for quote token selection.
- Port `quotearg_buffer_restyled` as the central implementation unit.
- Preserve:
  - byte-wise processing
  - escaping rules
  - quote insertion order
  - length accounting behavior
- Implement output generation using `Vec<u8>` and convert to `String` only in wrappers that operate on textual inputs.
- Add unit tests directly against the core path using representative inputs:
  - plain ASCII
  - embedded quote characters
  - control characters
  - empty input
  - non-UTF-8 byte sequences

## Phase 3: Port the public convenience wrappers and cleanup behavior

- Port:
  - `quotearg`
  - `quotearg_mem`
  - `quotearg_char`
  - `quote_mem`
  - `quote`
- Reconstruct wrapper relationships so each delegates to the Rust `quotearg_buffer_restyled` path rather than duplicating logic.
- Port `quotearg_free` last, after deciding whether any reusable slot/cache state remains necessary.
- If the C implementation uses rotating/static slots, model them privately with clear ownership rules and test repeated calls for stable behavior.
- Ensure no wrapper returns references to module-internal temporary buffers.

## Phase 4: Integration validation and behavioral alignment

- Update call sites in the `033-main_root_quotearg.c_32-rust-port` branch to use the migrated Rust module.
- Run `cargo test` and fix any mismatches caused by:
  - text vs byte return type assumptions
  - output length expectations
  - cleanup semantics around `quotearg_free`
- Add regression tests for wrapper equivalence:
  - `quote` vs text-form `quote_mem`
  - `quotearg` vs default-option `quotearg_mem`
  - repeated-call behavior when cache/state exists
- Keep the final implementation confined to the migrated `quotearg.c` responsibilities without introducing broader quoting frameworks or unrelated helpers.