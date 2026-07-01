# Implementation Plan: `main_root_quotearg.c_24`

## Summary

This module migration covers the quoting helpers currently implemented in `quotearg.c`, specifically the functions:

- `gettext_quote`
- `quotearg_buffer_restyled`
- `quotearg_free`
- `quotearg`
- `quotearg_mem`
- `quotearg_char`
- `quote_mem`
- `quote`

The Rust implementation should preserve the current module scope and behavior shape without introducing new quoting features or broader abstractions. The core approach is to port the existing quoting logic into a single Rust module that operates on byte slices and returns owned Rust strings or byte buffers where appropriate. Where the C code relies on reusable/static allocation patterns, the Rust version should replace them with explicit ownership and scoped memory management.

The migration should focus on:

- keeping the implementation concentrated in one Rust source file corresponding to `quotearg.c`
- representing input as `&[u8]` where C accepts pointer/length pairs
- using `String` when output is guaranteed UTF-8 text, and `Vec<u8>` or carefully validated string construction when byte-oriented escaping is needed
- removing manual free semantics from public usage while retaining a compatibility-shaped internal function path for `quotearg_free`
- preserving function-level behavior and migration order rather than redesigning the API surface

## Technical Context

### Language / Version

- Rust 1.78 or newer

### Primary Dependencies

Use the Rust standard library by default.

Recommended crates:

- None required for the initial port

Rationale:

- The input only identifies a single C source module and its functions.
- No explicit evidence suggests external localization, escape-processing, or memory-management crates are required.
- Quoting and byte/string handling can be implemented with `std`.

### Testing

- `cargo test`

Test coverage should include:

- direct function tests for each migrated function
- byte-oriented cases for embedded NUL and non-UTF-8 input where relevant
- edge cases for empty input, single-character quoting, and repeated calls that in C may have depended on static buffers
- equivalence-oriented tests derived from observed C behavior in this module

### Performance Goals

- Maintain linear-time processing with respect to input length
- Avoid repeated reallocations by reserving output capacity when practical
- Avoid unnecessary UTF-8 conversions for byte-oriented quoting paths
- Keep allocation count modest and predictable, while accepting ownership-based Rust returns in place of C static-buffer reuse

## Module Mapping

### C to Rust File Mapping

- `quotearg.c` -> `src/quotearg.rs`

If the project’s existing Rust crate root already exposes module declarations, add only the minimal corresponding declaration:

- `mod quotearg;` or `pub mod quotearg;`

### Function Mapping

| C Function | Rust Mapping |
|---|---|
| `gettext_quote` | `fn gettext_quote(...) -> &'static str` or `fn gettext_quote(...) -> &'static [u8]`, depending on observed return usage |
| `quotearg_buffer_restyled` | `fn quotearg_buffer_restyled(input: &[u8], ...) -> Vec<u8>` or internal writer-style helper returning produced length |
| `quotearg_free` | `fn quotearg_free()` as a no-op or internal state reset only if module-local cached storage is retained during migration |
| `quotearg` | `fn quotearg(input: &str) -> String` or byte-oriented wrapper over the core quoting routine |
| `quotearg_mem` | `fn quotearg_mem(input: &[u8]) -> String` or `Vec<u8>` with string wrapper if output contract requires text |
| `quotearg_char` | `fn quotearg_char(input: &str, ch: u8) -> String` |
| `quote_mem` | `fn quote_mem(input: &[u8]) -> String` |
| `quote` | `fn quote(input: &str) -> String` |

### Rust Module Scope

Keep all migrated logic inside the single `quotearg` module. Do not split into extra helper modules unless required by existing crate layout. Internal helpers may remain private functions in `src/quotearg.rs`.

## Data Model

The analysis reports only anonymous C data structures. Since no named structs are provided, the Rust plan should map them conservatively based on actual usage encountered during porting.

### Data-Structure Mapping Strategy

| C Representation | Rust Representation |
|---|---|
| anonymous option/config struct used only within `quotearg.c` | private `struct` with named fields inferred from field usage |
| anonymous enum-like style constants | private `enum` if the C code uses discrete modes; otherwise `const` values |
| pointer + length input pairs | `&[u8]` |
| mutable output buffer + size tracking | `Vec<u8>` or `String` with reserved capacity |
| static cached quoted strings/buffers | `Option<String>` / `Option<Vec<u8>>` in module-local state only if truly necessary; otherwise eliminate in favor of owned returns |
| translation quote strings | `&'static str` or `&'static [u8]` |

### Memory Management Decisions

- Replace C manual allocation/free patterns with Rust ownership.
- Prefer returning owned values instead of emulating static rotating buffers.
- If `quotearg_free` exists solely to release module-level cached allocations in C, reduce it to:
  - clearing minimal internal state if such state is still needed during migration, or
  - a no-op compatibility function if the Rust implementation has no retained heap state
- Avoid unsafe code unless a direct translation obstacle is encountered; if unavoidable, isolate it narrowly and document invariants.

### Error Handling Decisions

- Normal quoting operations should be infallible for valid inputs represented as bytes.
- If conversion to `String` is required from arbitrary bytes, ensure the quoting path emits ASCII-safe escaped output so the final result is valid UTF-8.
- Internal assumptions from C should be converted into:
  - bounds-safe indexing
  - explicit match handling for style/mode values
  - debug assertions only for impossible migrated states

## Implementation Phases

### Phase 1: Establish Rust module and core data mappings

- Create `src/quotearg.rs`.
- Identify all anonymous C structs/constants actually referenced by the listed functions.
- Define minimal private Rust structs/enums/constants to represent those items.
- Port `gettext_quote` first, along with any quote-style constants it depends on.
- Decide the core internal representation for input and output:
  - `&[u8]` for raw input
  - `Vec<u8>` or `String` for generated quoted output
- Remove dependence on C global/static buffer conventions unless the calling code strictly requires equivalent lifecycle.

**Exit criteria:**

- The module compiles with placeholder or partially implemented function bodies.
- Core private types and constants required by the listed functions are in place.

### Phase 2: Port the central quoting engine

- Port `quotearg_buffer_restyled` as the main internal implementation unit.
- Translate all byte-walking logic directly, preserving escaping and quoting decisions.
- Replace pointer arithmetic with indexed or iterator-based slice traversal.
- Preallocate output capacity conservatively based on input length.
- Ensure the produced output is valid Rust-owned data without external lifetime dependencies.
- Add focused unit tests for:
  - empty input
  - plain ASCII
  - characters requiring quoting/escaping
  - embedded NUL and non-printable bytes
  - behavior affected by selected quote characters or style inputs

**Exit criteria:**

- The core quoting function passes targeted tests.
- No unsafe memory behavior or manual frees are needed for the central logic.

### Phase 3: Port public wrappers and cleanup behavior

- Implement wrapper functions:
  - `quotearg`
  - `quotearg_mem`
  - `quotearg_char`
  - `quote_mem`
  - `quote`
- Ensure wrappers delegate to the single central quoting path rather than duplicating logic.
- Implement `quotearg_free` according to actual remaining state needs:
  - no-op if there is no retained module state
  - otherwise clear only module-local cached storage introduced during translation
- Align return types consistently across wrappers based on caller expectations in the Rust crate.

**Exit criteria:**

- All listed functions are implemented.
- Wrapper functions compile cleanly and share one internal quoting engine.
- Any retained state has explicit, minimal cleanup semantics.

### Phase 4: Verification and integration polishing

- Add regression-style tests covering repeated calls across wrappers to confirm no C-style static-buffer assumptions remain.
- Validate output consistency between string-based and byte-based wrapper paths.
- Review allocation behavior and remove unnecessary clones or intermediate buffers.
- Integrate the module into the existing crate root using only the necessary module declaration/export surface.
- Finalize documentation comments only where needed to clarify migrated invariants and ownership behavior.

**Exit criteria:**

- `cargo test` passes for the module and affected callers.
- The Rust port fully replaces the functionality from `quotearg.c` within the defined module scope.
- No extra facilities or modules were added beyond what the migration required.