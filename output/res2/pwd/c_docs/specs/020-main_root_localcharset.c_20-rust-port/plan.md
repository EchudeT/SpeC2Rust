# Implementation Plan: main_root_localcharset.c_20

## Summary

Port `localcharset.c` into a single Rust module that preserves the existing responsibility of resolving the active locale character set via `locale_charset`. The Rust implementation should keep the logic narrowly scoped to the current C file and function, using standard-library string and environment handling wherever possible and introducing only the minimum platform-specific branching needed to match current behavior.

The implementation approach is to migrate the existing lookup and normalization flow into safe Rust first, then isolate any unavoidable low-level or OS-specific access behind small internal helpers. Ownership should move from C-style borrowed/global pointers to Rust-owned `String` values or borrowed `&'static str` constants where applicable. Error handling should remain conservative: when exact locale resolution cannot be obtained, return the same effective fallback behavior as the C implementation rather than introducing new error surfaces.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only (`std::env`, `std::ffi` only if strictly needed during migration, `std::sync` only if caching behavior already exists in the C flow)
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve constant-time or near-constant-time behavior for common charset resolution paths
  - Avoid unnecessary heap allocation beyond the final returned charset string representation
  - Keep platform/environment probing limited to the same points used by the C implementation
  - Maintain no meaningful regression versus the C version for one-shot invocation from the main program path

## Module Mapping

| C File | C Function | Rust Module | Rust Item |
|---|---|---|---|
| `localcharset.c` | `locale_charset` | `src/localcharset.rs` | `pub fn locale_charset(...) -> ...` |

### Target Rust File Layout

| Rust File | Purpose |
|---|---|
| `src/localcharset.rs` | Direct port of `localcharset.c`, containing `locale_charset` and minimal private helpers needed to preserve current logic |
| `src/lib.rs` or `src/main.rs` integration point | Re-export or call `locale_charset` from the existing crate structure without creating extra abstraction layers |

## Data Model

The analysis reports only anonymous C data structures, which strongly suggests internal tables or transient compound values rather than stable external types. The Rust mapping should therefore stay minimal and local to `src/localcharset.rs`.

| C Data Shape | Expected Role in C | Rust Mapping |
|---|---|---|
| anonymous struct/table entry | Charset alias or locale-to-charset mapping row | Private `struct` with named fields, likely using `&'static str` members |
| anonymous static arrays | Lookup tables | Private `const` slices/arrays of table-entry structs |
| anonymous transient records | Local helper grouping | Replace with tuples or small private structs only if needed for readability during direct port |

### Rust Representation Guidance

- Convert C string literals and static tables to `&'static str`.
- Convert mutable C buffers to `String` only where normalization or concatenation is required.
- Avoid exposing any of the anonymous/internal data shapes publicly.
- If the C implementation returns a pointer to static storage, prefer:
  - `&'static str` if all outcomes can remain borrowed from constants, or
  - `String` if environment-derived normalized values must be returned.
- If compatibility with surrounding code requires borrowed output, centralize owned-to-borrowed conversion carefully and avoid unsafe global mutation unless the C design already depends on process-global cached state.

## Implementation Phases

### Phase 1: File Port Skeleton and Signature Mapping

- Create `src/localcharset.rs` and migrate `locale_charset` as the only public item for this module.
- Determine the exact Rust return type based on the C behavior and crate integration needs:
  - prefer `String` for safe ownership,
  - use `&'static str` only if the implementation is purely table/fallback based.
- Port the top-level control flow from `localcharset.c` without changing behavior.
- Add only the private helper functions required to mirror existing C sub-steps such as:
  - locale/environment extraction,
  - charset token parsing,
  - alias normalization,
  - fallback selection.

### Phase 2: Static Data and Parsing Logic Migration

- Translate anonymous C lookup tables into private Rust `const` arrays with explicit field names.
- Replace pointer arithmetic and sentinel-terminated iteration with slice iteration.
- Port charset normalization rules exactly, including case handling, separator trimming, and alias resolution only where present in the C logic.
- Replace C buffer management with safe Rust string slicing and owned `String` construction.
- Ensure memory behavior is explicit:
  - no borrowed references to temporary strings,
  - no raw-pointer-style lifetime assumptions,
  - no hidden mutable global state unless directly required by the original implementation.

### Phase 3: Platform Behavior and Fallback Equivalence

- Implement any platform-specific branches already implied by `localcharset.c` using `cfg` guards, keeping them in the same module rather than splitting into extra files unless the original file clearly separates behavior.
- Preserve existing fallback precedence for locale sources and default charset outcomes.
- Where the C implementation depends on environment variables or locale names, map them to `std::env` access with equivalent precedence and empty-value handling.
- If exact C behavior relies on non-UTF-8 environment content, use `std::env::var_os` and explicit conversion logic instead of assuming UTF-8.

### Phase 4: Validation and Cleanup

- Add focused unit tests for:
  - known locale name to charset mappings,
  - alias normalization,
  - empty/unset locale input behavior,
  - default fallback behavior.
- Add regression-style tests for edge cases visible in the C flow rather than inventing new API guarantees.
- Remove any temporary compatibility scaffolding introduced during the port.
- Verify the final module compiles cleanly with the existing crate entry points and does not introduce unnecessary public types or new feature surface.

## Memory Management and Error Handling Notes

- Replace all C-owned/static pointer assumptions with Rust ownership rules at the function boundary.
- Prefer immutable static lookup data and local owned strings over shared mutable storage.
- Do not expose recoverable errors unless the surrounding Rust crate already expects them; preserve C-style fallback behavior where lookup fails.
- Treat malformed or unavailable locale data as an input normalization case that resolves to the same fallback charset path as the C implementation.

## Acceptance Criteria

- `localcharset.c` is fully represented by `src/localcharset.rs`.
- `locale_charset` is migrated with behaviorally equivalent lookup, normalization, and fallback logic.
- Internal anonymous C data structures are replaced by private Rust constants and small local structs only where needed.
- The module builds and passes `cargo test` on the target branch without adding unrelated infrastructure or extra modules.