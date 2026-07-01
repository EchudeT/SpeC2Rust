# Implementation Plan: module_src_parseopt_parseopt_03

## Summary

Port `src/parseopt/parseopt.c` into a Rust module that preserves the current option-parsing behavior, lookup flow, initialization lifecycle, and state-query APIs without adding new parsing features.

The Rust implementation should center on a single module that owns parser state explicitly and replaces C pointer/memory conventions with safe standard-library containers and borrowing. The migration should keep the original function boundaries recognizable so behavior can be validated incrementally against the C implementation. Error cases currently represented through return codes or null checks should be translated into narrow Rust result/option patterns while preserving externally visible outcomes.

## Technical Context

### Language/Version
- Rust 1.78+ stable

### Primary Dependencies
- Rust standard library only
- No third-party crates are recommended from the available evidence

### Testing
- `cargo test`

### Performance Goals
- Maintain linear-time option scan behavior consistent with the C implementation
- Avoid unnecessary string cloning during parse and lookup paths
- Keep parser state allocation bounded to the option-definition and parsed-option storage already required by the C design
- Preserve predictable cleanup by relying on Rust ownership instead of manual free logic

## Module Mapping

### C to Rust File Mapping
- `src/parseopt/parseopt.c` -> `src/parseopt.rs`

If the project already uses a nested module layout, the equivalent conventional placement is acceptable:
- `src/parseopt/parseopt.c` -> `src/parseopt/mod.rs`

### Function Mapping
Retain the current logical API shape as Rust methods/free functions where practical:

- `set_version`
  - Map to a small internal helper that updates version-related parser metadata/state
- `_parseopt_optgroup`
  - Map to a private helper function for option group classification/selection
- `parseopt_init0`
  - Map to a low-level constructor/helper that initializes parser state without full higher-level setup
- `parseopt_init`
  - Map to the main public constructor or initialization method
- `parseopt_free`
  - Eliminate explicit memory release logic; map to `Drop`/ownership semantics, with at most a compatibility `clear`/reset method only if needed by call sites
- `parseopt_parse`
  - Map to the main parse routine over argument slices
- `parseopt_getopt`
  - Map to a retrieval method for parsed option values/results
- `parseopt_optdef_by_code`
  - Map to a lookup helper by numeric/code identifier
- `parseopt_optdef_by_name`
  - Map to a lookup helper by string name
- `parseopt_is_set`
  - Map to a boolean query method over parser state

## Data Model

The source analysis exposes only anonymous C data structures, so the Rust mapping should be driven by usage in `parseopt.c` and remain minimal.

### Planned Rust Types
Define only the structures required to replace the C storage patterns present in this file:

- Parser state struct
  - Holds option definitions, parse results, argument cursor/indexes, version/program metadata, and any flags used during parse
- Option definition struct
  - Represents each option's code, long/short name forms, argument expectation, grouping/category fields, and help/version-related flags if present in the C file
- Parsed option/value struct
  - Stores whether an option was encountered and any associated value text
- Option group enum or integer-backed field
  - Replaces internal group-selection logic currently handled by `_parseopt_optgroup`
- Error enum
  - Represents initialization/parse/lookup failures now expressed through C return codes or sentinel values

### C-to-Rust Mapping Rules
Because the exact anonymous C structs are not named in the input, apply these direct translation rules during implementation:

- C aggregate owning arrays/lists -> `Vec<T>`
- C string pointers used as borrowed input -> `&str` or `&[String]` at API boundaries where lifetime permits
- C stored strings requiring ownership beyond the call -> `String`
- C optional pointer fields -> `Option<T>` or `Option<String>`
- C integer flags/booleans -> `bool` or small enums when values are closed and meaningful
- C index/count fields -> `usize`
- C code identifiers -> `i32` or `u32`, matching the original signedness used by the call sites
- C null-return lookups -> `Option<&T>` / `Option<&mut T>`
- C status returns -> `Result<T, ParseOptError>` where failure meaning must be preserved

### Memory Management
- Replace `parseopt_free` responsibilities with owned Rust fields and automatic drop
- Avoid self-referential storage; store owned strings or stable indexes instead of interior raw pointers
- Use borrowed string slices during parsing when results do not need ownership; otherwise normalize once into owned `String`
- Keep mutation localized to parser-state methods to avoid aliasing issues

### Error Handling
- Convert parse-time failures from ad hoc C return/error patterns into a compact `ParseOptError`
- Preserve “not found” as `Option` for lookup functions where absence is not exceptional
- Reserve `Result` for initialization and parse operations that can fail structurally or semantically

## Implementation Phases

### Phase 1: Translate Core State and Initialization
- Inspect `src/parseopt/parseopt.c` and identify each anonymous struct instance and its field usage
- Define the minimal Rust structs/enums needed to represent parser state, option definitions, and parsed results
- Implement `parseopt_init0`, `parseopt_init`, and version-state handling from `set_version`
- Replace manual zeroing/setup logic with explicit constructors and defaults
- Establish the Rust equivalents for any sentinel/flag fields used later by parse and lookup functions

### Phase 2: Port Option Definition Lookup and Internal Helpers
- Implement the private helper corresponding to `_parseopt_optgroup`
- Port `parseopt_optdef_by_code` and `parseopt_optdef_by_name`
- Preserve original search order and matching behavior
- Normalize string comparisons carefully to match the C implementation’s semantics
- Add focused unit tests for initialization plus code/name lookup behavior

### Phase 3: Port Parse and Query Logic
- Implement `parseopt_parse` over Rust argument slices while preserving the C control flow and state updates
- Port `parseopt_getopt` and `parseopt_is_set` against the new parser-state storage
- Represent option values and “set/not set” state without raw pointers or manual allocation tracking
- Convert parse failures into `Result` while keeping absence/query behavior non-exceptional
- Add unit tests covering successful parses, missing values, unknown options, repeated options if applicable, and state queries

### Phase 4: Remove C-Style Cleanup Semantics and Finalize Compatibility
- Replace `parseopt_free` with ownership-based teardown; add a compatibility reset/clear method only if existing Rust call sites require object reuse
- Review all translated functions for unnecessary cloning and tighten borrowing where possible
- Align public naming/signatures with the surrounding Rust crate conventions while keeping one-to-one traceability to the C module
- Complete regression-style tests that validate the full init -> parse -> query lifecycle
- Confirm the module builds cleanly under `cargo test`