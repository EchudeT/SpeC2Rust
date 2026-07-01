# Implementation Plan

## Summary

Port the C module `localcharset.c` into a single Rust module that preserves the existing responsibility of resolving the locale character set through process environment and platform-facing locale information. The Rust implementation should keep the surface area narrow and centered on the existing function `locale_charset`, avoiding expansion into broader locale utilities.

The implementation approach is to translate the current lookup and normalization flow into safe Rust using `std` APIs first, with small, explicit handling for platform-specific branches where the original C code relied on libc or static tables. Any static alias or lookup tables present in the C source should become Rust `const`/`static` data with borrowed string slices where possible. The returned charset name should be represented in Rust with owned or borrowed string data as appropriate to match the original function’s effective behavior while keeping memory management automatic.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**:
  - Rust standard library only by default
  - No third-party crates are recommended from the provided evidence
- **Testing**: `cargo test`
- **Performance Goals**:
  - Match the C module’s practical runtime characteristics for single charset resolution
  - Keep lookup overhead low for alias/table matching
  - Avoid unnecessary heap allocation except where a normalized charset string must be returned as owned data
  - Preserve straightforward execution with minimal branching beyond the original logic

## Module Mapping

- **C source file**: `localcharset.c`
- **Rust module/file**: `src/main_root_localcharset.rs` or `src/localcharset.rs` depending on the crate’s existing layout
- **Primary function mapping**:
  - `locale_charset` -> `pub(crate) fn locale_charset(...) -> ...`

Implementation should remain limited to the logic currently housed in `localcharset.c`. If the crate already has a root module organization for the `main_cluster`, this file should be placed into that existing structure rather than introducing a new subsystem.

## Data Model

The analysis only identifies multiple anonymous C data structures, which in this kind of module are typically static table rows or internal helper layouts rather than externally visible domain types. The Rust port should therefore prefer minimal private representations and only introduce named Rust types when required to express existing table-driven logic.

### C to Rust structure mapping

- **anonymous static table entry structs** -> private Rust structs with named fields
  - Example shape:
    - C anonymous table row containing charset/alias mapping
    - Rust: `struct CharsetAlias { from: &'static str, to: &'static str }`
- **anonymous locale mapping records** -> private Rust structs or tuples
  - Rust should use:
    - `struct LocaleMap { locale: &'static str, charset: &'static str }`
    - or `(&'static str, &'static str)` if the C structure is trivial and only used internally
- **C string pointers (`char *`, `const char *`)** -> `&'static str`, `&str`, or `String`
  - Use `&'static str` for compiled-in tables
  - Use `&str` for borrowed intermediate parsing results
  - Use `String` only for normalized or computed output
- **C sentinel-terminated arrays** -> Rust slices
  - Replace null-terminated table scans with `&'static [T]`
- **C nullability conventions** -> `Option<T>`
  - Any internal “not found” path should use `Option<&'static str>` or `Option<String>`
- **C error signaling by fallback strings** -> explicit fallback branches in Rust
  - Preserve behavior by returning the same effective fallback charset name, but represent the control flow explicitly

### Memory management and ownership

- Eliminate manual lifetime handling from the C implementation by storing static mapping data in Rust static slices.
- Prefer returning borrowed static strings when the selected charset comes directly from a built-in table.
- Return `String` only if normalization requires allocation from environment-derived input.
- Avoid unsafe code unless a platform-specific locale query cannot be expressed otherwise; if unavoidable, confine it to a tiny helper with documented assumptions.

## Implementation Phases

### Phase 1: Establish file/module skeleton and function boundary

- Create the Rust module corresponding to `localcharset.c`.
- Define the Rust signature for `locale_charset` based on how the surrounding project consumes it.
- Identify the exact C control flow:
  - environment and locale sources consulted
  - alias normalization steps
  - fallback behavior
- Translate compile-time constants and internal tables into Rust `const`/`static` data structures.
- Keep all helper items private to the module unless required by existing call sites.

### Phase 2: Port charset resolution and normalization logic

- Implement the main locale charset lookup path in Rust, preserving the original precedence order.
- Port internal parsing of locale-related strings using `std` string operations rather than pointer arithmetic.
- Convert C table scans into iterator or indexed slice traversal without changing lookup behavior.
- Preserve existing fallback semantics exactly, especially for empty, missing, or unrecognized locale values.
- Resolve ownership carefully:
  - borrowed static result for table matches
  - owned string for computed normalization when needed

### Phase 3: Handle platform-specific branches and edge behavior

- Translate any OS-conditional logic from `localcharset.c` into Rust `cfg` blocks.
- Keep each branch limited to functionality already present in the C module.
- Where the C code depended on libc locale state, first check whether equivalent behavior can be achieved with environment inspection through `std`.
- If a direct platform call is unavoidable, isolate it in a minimal helper and keep the unsafe boundary as small as possible.
- Verify behavior for:
  - unset locale variables
  - locale names with codeset suffixes
  - alias remapping
  - default/fallback charset selection

### Phase 4: Testing and migration validation

- Add focused unit tests for the lookup and normalization rules that can be validated deterministically.
- Prefer tests around pure helper logic and table-driven cases rather than broad process-global locale mutation.
- Use `cargo test` to validate:
  - alias resolution
  - parsing of locale strings
  - fallback behavior for missing or malformed input
  - platform-conditional compilation where applicable
- Compare Rust outputs against expected results inferred from the current C implementation for representative cases before considering the port complete.