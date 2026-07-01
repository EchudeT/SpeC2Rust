# Implementation Plan: module_src_parseopt_parseopt_03

## Summary

Port `src/parseopt/parseopt.c` into a Rust module that preserves the existing option-parsing behavior and public module responsibilities without adding new parsing features. The Rust implementation should translate the current parser state, option definition lookup, option grouping, parse lifecycle, and query helpers into idiomatic Rust structures while keeping control flow close to the C source for easier verification.

The implementation approach is:

- create a single Rust module corresponding to the C file
- represent parser state and option definitions with owned Rust structs/enums
- replace manual allocation/free patterns with RAII and `Drop`-free ownership where possible
- convert pointer-based lookup and mutation into slice/vector iteration and indexed access
- model parse outcomes with `Result` and explicit error types, while preserving the original success/failure boundaries
- migrate functions in dependency order so initialization, parse execution, lookup helpers, and state queries can be validated incrementally

## Technical Context

### Language/Version
- Rust stable, edition 2021
- Minimum recommended compiler: Rust 1.76 or newer

### Primary Dependencies
- Rust standard library only
- No third-party crates are recommended because the input only indicates a direct C module port and does not justify external parsing libraries

### Testing
- `cargo test`

### Performance Goals
- Maintain linear scan behavior where the C implementation uses sequential option-definition searches
- Avoid unnecessary string cloning during parse and lookup operations
- Keep parser state in contiguous Rust collections such as `Vec` and borrowed string views where ownership rules permit
- Match the C module’s expected runtime profile for command-line sized inputs rather than introducing heavier abstractions

## Module Mapping

### Source File Mapping
- C: `src/parseopt/parseopt.c`
- Rust: `src/parseopt/parseopt.rs`

### Function Mapping
- `set_version` -> `fn set_version(...)`
- `_parseopt_optgroup` -> `fn parseopt_optgroup(...)`
- `parseopt_init0` -> `fn parseopt_init0(...)`
- `parseopt_init` -> `fn parseopt_init(...)`
- `parseopt_free` -> removed as an explicit memory-management primitive; retained only as a compatibility-shaped reset/clear method if current call sites require it
- `parseopt_parse` -> `fn parseopt_parse(...) -> Result<..., ParseOptError>`
- `parseopt_getopt` -> `fn parseopt_getopt(...)`
- `parseopt_optdef_by_code` -> `fn parseopt_optdef_by_code(...)`
- `parseopt_optdef_by_name` -> `fn parseopt_optdef_by_name(...)`
- `parseopt_is_set` -> `fn parseopt_is_set(...) -> bool`

### Rust Module Placement
If the project already has a `parseopt` module tree:
- `src/parseopt/mod.rs` should expose `pub mod parseopt;`

If this C file maps directly into an existing flat module layout:
- place the translated implementation in the nearest equivalent Rust module without introducing extra submodules

## Data Model

Because the analysis only exposes anonymous C data structures, the Rust data model should be derived directly from usage in `parseopt.c` and named by role rather than by guessed original typedef names.

### Core Mapping Rules
- C raw struct with parser lifetime/state -> Rust `struct ParseOpt`
- C raw struct for option definitions -> Rust `struct OptDef`
- C raw struct for parsed option instances or values -> Rust `struct ParsedOpt` or `struct OptValue`
- C integer flags/bitfields -> Rust integer flag field or small enums, depending on actual usage density
- C `char *` / `const char *` -> Rust `String` for owned storage, `&str` for borrowed inputs
- C arrays with count fields -> Rust `Vec<T>`
- C nullable pointers -> Rust `Option<T>` / `Option<usize>` / `Option<String>`
- C status/error return codes -> Rust `Result<T, ParseOptError>`

### Expected Rust Structures
These names should be finalized from actual field usage during porting:

- `ParseOpt`
  - holds parser configuration, option definitions, parse results, argument cursor state, and version/program metadata if present
- `OptDef`
  - holds option code, long name, short name, argument requirement, grouping/category linkage, and flags
- `OptGroup`
  - if `_parseopt_optgroup` manipulates grouped definitions or sections, represent this as a dedicated struct or enum only if the C code has an actual persisted grouping object
- `ParsedOpt`
  - holds resolved option definition reference or index plus any parsed argument value and set-state
- `ParseOptError`
  - enum for initialization errors, unknown options, missing arguments, invalid state, and any direct C error-return cases

### Memory Management Notes
- Eliminate explicit heap lifetime management from `parseopt_free` by storing owned data in `Vec`/`String`
- Where the C code stores references into argument arrays, decide case-by-case between:
  - borrowing from input slices during parsing, if lifetimes remain local, or
  - normalizing into owned `String` values, if parser state must outlive the input call frame
- Avoid self-referential structures; prefer indices into `Vec<OptDef>` rather than internal references

### Error Handling Notes
- Preserve the original failure points from C instead of broadening the API surface
- Use `Result` for operations that can fail during initialization or parsing
- Keep lookup helpers returning `Option<&OptDef>` or `Option<usize>` where the C implementation uses null/not-found semantics
- If current callers depend on integer status codes, isolate code-to-error translation near the module boundary rather than leaking C-style codes internally

## Implementation Phases

### Phase 1: Establish Rust Module Skeleton and State Types
- Create `src/parseopt/parseopt.rs`
- Define Rust equivalents for the parser state and option-definition data used in `parseopt.c`
- Identify each anonymous C structure by usage and assign stable Rust names based on role
- Implement `parseopt_init0`, `parseopt_init`, and compatibility handling for `parseopt_free`
- Decide ownership of strings and option tables from actual call patterns
- Add unit tests for initialization/reset behavior and empty-state invariants

### Phase 2: Port Lookup and Query Helpers
- Port `set_version`
- Port `parseopt_optdef_by_code`
- Port `parseopt_optdef_by_name`
- Port `parseopt_is_set`
- Port `_parseopt_optgroup` as an internal helper with visibility restricted to the module
- Keep search behavior aligned with C ordering semantics, especially if duplicate or alias handling depends on first-match rules
- Add tests covering code lookup, name lookup, grouping behavior, and set-state queries

### Phase 3: Port Parse Execution Path
- Port `parseopt_getopt`
- Port `parseopt_parse`
- Translate argument scanning, state mutation, and option-value assignment in the same logical order as the C implementation
- Replace pointer arithmetic and null checks with index-based iteration and `Option` handling
- Preserve exact distinctions among recognized option, missing parameter, unknown option, and end-of-input paths
- Add focused tests for short options, long options, grouped options if supported by the C logic, option arguments, and parse termination behavior

### Phase 4: Integrate, Verify, and Remove C-Style Memory Assumptions
- Review all ported functions for unnecessary clones and convert to borrowed access where safe
- Ensure no explicit free-style cleanup remains necessary for normal use
- Align public signatures with surrounding Rust project conventions only as far as needed for compilation and existing caller migration
- Add regression tests covering full parser lifecycle: init, parse, lookup, query, reset/free-equivalent
- Confirm `cargo test` passes and the Rust module fully replaces the original C file behavior at the module boundary