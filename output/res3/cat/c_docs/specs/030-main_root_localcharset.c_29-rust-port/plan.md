# Implementation Plan

## Summary

Port `localcharset.c` into a Rust module that preserves the existing `locale_charset` behavior and keeps the implementation narrowly scoped to charset detection logic used by the current `cat` codebase. The Rust version should translate the C control flow and lookup data into idiomatic but direct Rust, avoiding feature expansion.

The implementation approach is:

- create a single Rust module corresponding to `localcharset.c`
- port `locale_charset` as the primary public function
- translate the C anonymous static mapping tables into private Rust constants or small private structs
- use standard-library environment and platform facilities first
- represent charset results as borrowed static strings where possible to match the C module’s table-driven behavior and avoid unnecessary allocation
- keep error handling conservative: when locale inspection is incomplete or unsupported, fall back to the same default outcome as the C logic rather than introducing richer error types

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - keep charset resolution effectively constant-time aside from environment lookup
  - avoid heap allocation unless unavoidable for platform string conversion
  - preserve low overhead suitable for startup-path execution
  - keep static lookup tables in read-only constants

## Module Mapping

| C File | Rust File | Notes |
|---|---|---|
| `localcharset.c` | `src/localcharset.rs` | Direct port of locale/charset detection logic |
| `localcharset.c` (`locale_charset`) | `src/localcharset.rs` -> `pub fn locale_charset(...)` or `pub fn locale_charset() -> &'static str` | Final signature should match how the surrounding Rust port consumes the result; prefer no arguments if the C function is global-state based |

If the branch already has a crate root layout, expose the module with a minimal declaration from `src/lib.rs` or the existing crate root only as needed by current callers.

## Data Model

The C analysis reports only anonymous data structures. These should be mapped to minimal private Rust representations based on actual usage in `localcharset.c`.

| C Data Structure | Rust Mapping | Usage |
|---|---|---|
| anonymous static mapping entries | `struct CharsetMapEntry { locale: &'static str, charset: &'static str }` | For locale-to-charset table rows if the C file contains paired string tables |
| anonymous alias entries | `struct AliasEntry { from: &'static str, to: &'static str }` | For canonicalization tables if present |
| anonymous grouped tables | `const &[CharsetMapEntry]` / `const &[AliasEntry]` | Replaces C static arrays |
| anonymous platform-specific records | private `struct` with only required fields | Used only if the C file contains OS-conditional metadata |
| C string pointers | `&'static str` | For compile-time known names and return values |
| transient parsed locale slices | `&str` | For parsing locale environment values without allocation |
| optional lookup result | `Option<&'static str>` | Replaces null pointer checks |
| unreachable/unsupported branches | `match` / `if let` with fallback return | Replaces sentinel-based C flow |

### Memory Management

- C static string storage becomes Rust `const`/`static` string data.
- Avoid manual ownership translation; prefer borrowed string slices throughout.
- For any OS string acquisition requiring lossy conversion, keep the conversion local and return only canonical static charset names.
- Eliminate null pointer handling by using `Option`.

### Error Handling

- Do not introduce a public error type unless the surrounding port already requires one.
- Internal lookup failures should resolve through explicit fallback paths mirroring the C behavior.
- Platform parsing edge cases should be treated as “unknown locale” and use the module’s default charset result.

## Implementation Phases

### Phase 1: Module Skeleton and Function Port

- create `src/localcharset.rs`
- port the `locale_charset` function signature and main decision flow
- map environment and locale source inspection from C into `std::env` and string parsing
- define the minimal public visibility needed by current callers
- add crate-root module declaration only where required

### Phase 2: Static Table and Parsing Migration

- identify each anonymous C table and convert it into private Rust constants
- introduce small private structs only where table rows need named fields
- port locale normalization, suffix extraction, and alias/canonicalization logic exactly as needed by the C function
- ensure all table lookups return static charset names without allocation where possible

### Phase 3: Platform/Fallback Behavior Alignment

- port any conditional branches in `localcharset.c` that depend on target OS or libc behavior using Rust `cfg` gates
- implement the same fallback ordering used by the C version for missing or malformed locale values
- verify that unsupported paths still return the expected default charset rather than panicking

### Phase 4: Tests and Integration Verification

- add focused unit tests for:
  - locale string parsing
  - known locale-to-charset mappings present in the C tables
  - alias resolution if present
  - fallback behavior for empty, invalid, or unset locale inputs
- add target-conditional tests only where the original C logic is platform-specific
- run `cargo test` and adjust the implementation to preserve behavior parity with the original module