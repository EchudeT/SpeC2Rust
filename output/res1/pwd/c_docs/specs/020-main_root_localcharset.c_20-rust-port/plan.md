# Implementation Plan

## Summary

Port `localcharset.c` into a Rust module that preserves the existing `locale_charset` behavior and lookup flow without adding new capabilities. The Rust implementation should focus on reproducing the current charset-detection logic using standard-library facilities for environment access, string handling, and table-driven matching.

The implementation should remain narrowly scoped to the existing module surface: one Rust module containing the migrated `locale_charset` function and its internal lookup data. Static C tables and anonymous helper records should become private Rust constants and small private structs or tuples, with ownership handled by Rust strings and string slices instead of manual memory management.

## Technical Context

- **Language/Version**: Rust 1.77+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve constant-time or near-constant lookup characteristics of the original table-driven logic.
  - Avoid unnecessary heap allocation where borrowed `&str` or static string data is sufficient.
  - Keep startup and per-call overhead minimal, since locale charset resolution is expected to be lightweight and frequently callable.

## Module Mapping

- **C source**: `localcharset.c`
- **Rust target**: `src/main_root_localcharset.rs` or `src/localcharset.rs` depending on the existing crate layout for the main cluster
- **Primary function mapping**:
  - `locale_charset` -> `pub(crate)` or `pub` `fn locale_charset(...) -> ...`
    The exact visibility and return type should be chosen to match the surrounding Rust port interface, while preserving the original module role.
- **Internal tables/helpers**:
  - C static lookup tables -> private Rust `const` or `static` arrays
  - C helper logic embedded in `localcharset.c` -> private Rust functions within the same module

## Data Model

The C analysis reports multiple anonymous data structures. These should not be expanded beyond what is required by the migrated file.

- **C anonymous table records** -> private Rust struct(s) with named fields only if field grouping is needed for readability and direct translation.
  - Likely shape: locale/alias key to charset value mapping
  - Rust form:
    ```rust
    struct CharsetMapEntry {
        key: &'static str,
        value: &'static str,
    }
    ```
  - If the C data is only paired values, use:
    ```rust
    type CharsetMapEntry = (&'static str, &'static str);
    ```
- **C string pointers / static char arrays** -> `&'static str`
- **C optional lookup results / null pointer cases** -> `Option<&'static str>` internally, converted to the required public return representation
- **C mutable temporary buffers** -> `String` only where normalization or environment-derived values require owned data

### Memory Management Notes

- Replace manual pointer and buffer handling with Rust borrowing where source data is static.
- Use `String` for environment-derived locale values only when normalization requires ownership.
- Eliminate null-pointer state internally by using `Option`.

### Error Handling Notes

- If the original function falls back to a default charset on missing or malformed locale data, preserve that behavior directly rather than introducing `Result`.
- Reserve `Result` only if the surrounding Rust API already requires it; otherwise keep the function behavior-oriented and fallback-based.

## Implementation Phases

## Phase 1: Module Skeleton and API Port

- Create the Rust module file corresponding to `localcharset.c`.
- Define the Rust signature for `locale_charset` to match the ported crate’s calling conventions.
- Identify the C function’s observable return behavior:
  - static return values
  - fallback charset behavior
  - handling of missing locale-related inputs
- Add minimal unit tests covering the public API shape and default/fallback behavior.

## Phase 2: Lookup Data and Internal Logic Migration

- Translate C static tables and anonymous records into private Rust constants.
- Port any locale normalization and alias-matching logic into small private helpers within the same module.
- Replace pointer traversal and sentinel-based iteration with idiomatic slice iteration.
- Ensure comparisons preserve original matching semantics, including case handling and delimiter parsing if present in the C code.
- Keep all migrated logic local to this module; do not split into additional support modules.

## Phase 3: Environment and Locale Resolution Behavior

- Port the environment-based locale resolution flow using `std::env`.
- Reproduce the original precedence among locale-related variables and locale name parsing as implemented in `localcharset.c`.
- Handle absent, empty, or malformed values by preserving C fallback behavior.
- Ensure no borrowed data outlives owned environment strings; normalize into owned `String` only when required.

## Phase 4: Validation and Cleanup

- Add table-driven tests for representative locale names, aliases, and fallback cases derived from the C module behavior.
- Verify that the Rust implementation returns the same charset names expected from the original lookup tables.
- Remove any remaining C-style artifacts such as sentinel assumptions or manual buffer patterns.
- Confirm the final module is limited to the migrated file scope and does not introduce extra facilities beyond the original implementation.