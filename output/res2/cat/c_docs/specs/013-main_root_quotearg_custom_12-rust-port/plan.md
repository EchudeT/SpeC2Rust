# Implementation Plan

## Summary

Port the `quotearg.c` portion represented by `main_root_quotearg_custom_12` into Rust on branch `013-main_root_quotearg_custom_12-rust-port`, limited to the existing behavior surface of:

- `quotearg_custom`
- `quotearg_custom_mem`

The Rust implementation should preserve the current quoting behavior entry points rather than redesigning the quoting subsystem. The technical approach is to migrate these functions into a Rust module that operates on byte slices and string-like inputs using standard-library-owned types, while keeping compatibility with the surrounding quoting option model already implied by the source module. The port should favor direct translation of control flow and data access patterns from C, replacing raw-pointer and length-pair handling with safe slice-based APIs internally and using explicit result/option handling where nullability or invalid state existed in C.

The implementation should remain tightly scoped to these functions and any immediately required supporting types already represented in the C source, without introducing additional abstraction layers beyond what is needed for a safe and maintainable Rust translation.

## Technical Context

### Language / Version

- Rust stable, edition 2021
- Minimum recommended compiler: `rustc 1.74` or newer

### Primary Dependencies

Use the Rust standard library by default.

Recommended dependencies:

- No third-party crates required for this module port

### Testing

- `cargo test`

Testing focus:

- Behavioral equivalence for custom quoting with explicit delimiters
- Correct handling of length-bounded input for `_mem` variant
- Empty input, embedded non-UTF-8 bytes, and delimiter edge cases
- Parity between string-oriented and memory-oriented call paths where inputs overlap

### Performance Goals

- Match the C implementation’s asymptotic behavior for quoting operations
- Avoid unnecessary intermediate allocations beyond the produced quoted output
- Preserve byte-oriented processing so non-UTF-8 inputs do not trigger conversion overhead
- Keep copy/scan passes restrained and predictable for small command-line utility workloads

## Module Mapping

### C to Rust File Mapping

- `quotearg.c` → `src/quotearg.rs`

If the current Rust port already has a crate root exposing quoting helpers, only add the migrated functions to the existing quoting module rather than creating extra modules.

### Function Mapping

- `quotearg_custom` → `pub(crate) fn quotearg_custom(...) -> ...`
- `quotearg_custom_mem` → `pub(crate) fn quotearg_custom_mem(...) -> ...`

Exact Rust signatures should be chosen to reflect existing call patterns in the Rust port, but the mapping should follow these rules:

- C pointer + implicit NUL string input → `&str` or `&[u8]` depending on whether non-UTF-8 must remain representable
- C pointer + explicit length input → `&[u8]`
- C output ownership returned through static/allocated quote helpers → owned `String` or `Vec<u8>` as required by surrounding module conventions

Preferred internal representation:

- `quotearg_custom`: accept custom left/right quote delimiters plus input as bytes or string-compatible input
- `quotearg_custom_mem`: accept custom delimiters plus `&[u8]`

Where the surrounding port already uses a shared quoting-options structure, these functions should construct or adjust that structure in the same module rather than bypassing it.

## Data Model

The analysis lists only anonymous C data structures, which indicates these functions likely interact with shared internal quoting configuration types defined in `quotearg.c`. The Rust plan should therefore map them conservatively into named Rust types only where required for compilation and behavioral parity.

### Data-Structure Mapping

- Anonymous C structs used as quoting option/state carriers
  - → Rust `struct QuotingOptions` or reuse existing ported quoting-options type if already present
- Anonymous C enums or integer mode fields
  - → Rust `enum` where variants are known and already used by adjacent migrated code
  - → otherwise preserve as narrow integer fields until the wider module is ported
- Anonymous pointer-based buffers
  - → Rust `Vec<u8>` for owned mutable output buffers
  - → Rust `&[u8]` for borrowed input memory
- Anonymous C string pointers for quote delimiters
  - → Rust `&str` if guaranteed UTF-8 by call sites
  - → otherwise `&[u8]` to preserve exact byte behavior

### Mapping Guidance

Because these two functions are entry points into an existing quoting implementation, avoid inventing new standalone data models. Instead:

1. Reuse any already ported Rust quoting state type.
2. If missing, introduce only the minimal Rust struct/enum definitions needed by these functions.
3. Keep field layout and semantics aligned with the C source so follow-on ports from `quotearg.c` can attach without churn.

### Memory Management and Error Handling

- Replace raw C buffer management with owned Rust output buffers
- Eliminate null-pointer state by requiring borrowed references at Rust boundaries where possible
- If C allowed invalid custom delimiter inputs, represent validation using:
  - `assert!` only if the original C treated misuse as programmer error, or
  - `Result<_, QuoteArgError>` if failure was observable and recoverable in existing behavior
- Preserve byte fidelity for memory-based quoting; do not assume UTF-8 for `_mem`

## Implementation Phases

## Phase 1: Establish Rust Module Surface

- Create or update `src/quotearg.rs` for this port unit
- Locate the existing Rust equivalents for shared quoting logic referenced by `quotearg_custom` and `quotearg_custom_mem`
- Define the Rust function signatures for the two target functions in a way that matches current crate usage
- Introduce only the minimal supporting type aliases/structs required for these functions to compile
- Add module-level tests scaffolding for custom-quote behavior

**Deliverable:** Compiling Rust module skeleton with function signatures and any required local type definitions.

## Phase 2: Port `quotearg_custom_mem`

- Translate the length-aware C implementation first, since it is the more direct byte-based primitive
- Map C pointer/length input to `&[u8]`
- Recreate delimiter handling and option setup exactly as in the C code path
- Route output through the existing Rust quoting machinery or a direct translated helper if this function depends on local C logic
- Add tests for:
  - empty slice
  - ordinary ASCII content
  - embedded special characters requiring quoting
  - non-UTF-8 byte content
  - distinct left/right custom delimiters

**Deliverable:** Working `quotearg_custom_mem` with passing unit tests.

## Phase 3: Port `quotearg_custom`

- Implement the string-oriented wrapper on top of the `_mem` variant where that matches the C structure
- Convert input to the appropriate borrowed representation without adding unnecessary allocation
- Preserve any C semantics tied to NUL-terminated input length detection
- Verify parity with the C wrapper behavior for standard string inputs and empty strings
- Add tests that compare `quotearg_custom` against `quotearg_custom_mem` on equivalent inputs

**Deliverable:** Working `quotearg_custom` with wrapper-level tests and shared behavior validated.

## Phase 4: Integration Cleanup and Equivalence Review

- Align naming, visibility, and return types with the rest of the Rust port
- Remove any temporary translation scaffolding that is no longer needed
- Review for C-to-Rust semantic mismatches:
  - delimiter lifetime assumptions
  - byte vs string conversions
  - ownership of produced quoted output
- Run full `cargo test`
- Keep implementation limited to the migrated functions and their immediate dependencies in `quotearg.rs`

**Deliverable:** Final integrated module port for `main_root_quotearg_custom_12`, ready for follow-on migration work.