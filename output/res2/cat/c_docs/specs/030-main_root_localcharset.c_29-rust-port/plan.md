# Implementation Plan

## Summary

Port `localcharset.c` into a Rust module that preserves the existing responsibility of determining the current locale character set through a single public function corresponding to `locale_charset`.

The Rust implementation should stay narrowly scoped to the existing C file and function set. The preferred approach is:

- create one Rust source module for this C file,
- expose one Rust function matching the role of `locale_charset`,
- use standard-library types for owned and borrowed string data,
- isolate any platform-specific charset lookup logic behind small internal helpers,
- replace C static tables and anonymous structures with private Rust constants and named structs or enums only where needed.

The migration should prioritize behavioral equivalence for the current call patterns over redesign. If the original C implementation relies on static lookup tables, aliases, or environment-driven locale parsing, those should be represented directly in Rust with immutable data and straightforward parsing logic.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only, unless the wider crate already mandates a locale-specific dependency. No third-party crate is required by the provided module analysis.
- **Testing**: `cargo test`
- **Performance Goals**:
  - keep charset detection effectively constant-time aside from bounded locale string parsing,
  - avoid unnecessary heap allocation where a borrowed `&'static str` or `Cow<'static, str>` is sufficient,
  - preserve low overhead comparable to the C implementation for repeated calls.

## Module Mapping

### C to Rust File Mapping

- `localcharset.c` → `src/localcharset.rs`

### Function Mapping

- `locale_charset` → `pub(crate) fn locale_charset(...) -> ...` or `pub fn locale_charset(...) -> ...`

Final visibility and signature should be chosen to match how the rest of the Rust port calls this function, but the port should not introduce additional public APIs beyond the existing C surface.

### Internal Organization

Keep implementation confined to `src/localcharset.rs`, using private helpers only if needed for:

- locale environment extraction,
- locale name normalization/parsing,
- charset alias/table lookup,
- fallback resolution.

Do not split this file into extra modules unless forced by existing project structure.

## Data Model

The analysis lists only anonymous C data structures. These should be translated minimally based on actual usage in `localcharset.c`.

### Expected Mapping Strategy

- anonymous C table entries used for locale/charset mappings
  → private named Rust `struct` with `&'static str` fields, for example:
  - locale key / pattern
  - charset result
- anonymous grouped constants or arrays
  → `const` or `static` slices of Rust structs
- anonymous flag-like or category-like values
  → private Rust `enum` only if the C logic clearly uses discrete tagged states; otherwise use simple local variables

### Memory Management Notes

- Replace C string pointers with Rust string slices where data is static.
- For computed results, prefer `Cow<'static, str>` if the logic may return either a static mapping or a parsed/normalized owned value.
- Avoid leaking or emulating C-style static mutable buffers.
- Keep all lookup tables immutable.

### Error Handling Notes

The C function likely returns a fallback charset rather than signaling rich errors. Mirror that behavior in Rust:

- use deterministic fallback values instead of introducing new error types,
- keep parsing failures internal and map them to the same fallback path the C implementation uses,
- reserve `Option` or `Result` for private helpers only if it simplifies logic.

## Implementation Phases

## Phase 1: Establish Module Skeleton and Port Static Data

- Create `src/localcharset.rs`.
- Add the Rust equivalent of `locale_charset` with a temporary internal flow matching the C control structure.
- Identify all static tables, aliases, and anonymous structures in `localcharset.c`.
- Convert those tables into private Rust `const`/`static` slices with named private structs.
- Replace C string constants and pointer-based table traversal with slice iteration.

### Deliverables

- Rust module file exists.
- All compile-time mapping data from `localcharset.c` is represented in Rust.
- Public function stub compiles, even if behavior is not yet fully wired.

## Phase 2: Port Locale Parsing and Charset Resolution Logic

- Port the exact decision flow of `locale_charset`.
- Translate environment and locale extraction logic using the Rust standard library.
- Recreate any normalization steps used by the C code, such as:
  - stripping modifiers,
  - handling language/territory/encoding segments,
  - applying alias lookup.
- Preserve fallback behavior for missing or malformed locale settings.
- Keep helper functions private and local to this module.

### Deliverables

- `locale_charset` returns the Rust equivalent of the C result for the main code paths.
- Parsing and lookup logic is implemented without unsafe code unless the surrounding crate architecture requires it.

## Phase 3: Integrate With Crate and Resolve Signature/Ownership Details

- Align the Rust function signature with the surrounding ported codebase.
- Decide whether the function returns:
  - `&'static str`,
  - `String`,
  - or `Cow<'static, str>`
  based strictly on the translated behavior and caller needs.
- Remove any temporary placeholders from Phase 1.
- Ensure no C-era assumptions remain about mutable global storage or null-terminated ownership.

### Deliverables

- Module is wired into the crate tree.
- Function signature is stable and idiomatic for the existing port.
- Ownership and lifetime choices are finalized.

## Phase 4: Validate Behavior With Focused Tests

- Add unit tests in the same module or crate test structure for:
  - recognized locale-to-charset mappings,
  - alias resolution,
  - normalization of locale strings with modifiers,
  - fallback behavior when locale-related input is absent or unsupported.
- Prefer table-driven tests derived from the original C cases where identifiable.
- Run `cargo test` and fix any divergences caused by string handling or parsing differences.

### Deliverables

- Regression tests cover the translated lookup behavior.
- The port is verified against representative locale input cases.
- `cargo test` passes.