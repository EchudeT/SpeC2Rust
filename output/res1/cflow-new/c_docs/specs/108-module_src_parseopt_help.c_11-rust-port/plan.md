# Implementation Plan

## Summary

Port `src/parseopt/help.c` into an equivalent Rust module with behavior preserved for the identified function surface, currently centered on `min`. The Rust implementation should stay narrowly aligned with the existing C file’s responsibilities and avoid introducing broader parsing or help-system redesign.

Technical approach:

- Create a Rust module corresponding directly to `src/parseopt/help.c`.
- Translate the C logic into safe Rust where possible, using plain functions and simple value types.
- Keep function boundaries close to the original C implementation so migration and verification remain straightforward.
- Replace C-style memory handling and unchecked integer operations with Rust ownership and explicit numeric types.
- Preserve output and control-flow semantics relevant to option-help processing in this file, while limiting scope to the existing module behavior.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Maintain effectively equivalent runtime characteristics for small utility-style helper logic.
  - Avoid unnecessary heap allocation beyond what is required by safe string and collection handling.
  - Preserve constant-time behavior for primitive helper functions such as `min`.
  - Keep the port simple and predictable rather than optimizing beyond the original C module’s needs.

## Module Mapping

C to Rust file mapping:

- `src/parseopt/help.c` → `src/parseopt/help.rs`

If the Rust crate already exposes parse-option modules through a parent module file, wire the migrated file through the existing Rust module tree only as needed:

- existing `src/parseopt/mod.rs` → add `pub(crate) mod help;` if not already present

Function mapping:

- `min` → `pub(crate)` or private Rust function `min(...)` with the narrowest visibility required by current callers

Migration constraints:

- Do not split this C file into additional Rust modules unless required by the existing crate layout.
- Do not introduce new helper subsystems; keep logic in `help.rs` unless a direct C-local static helper requires a small private Rust equivalent.

## Data Model

The analysis lists only anonymous C data structures and does not provide field definitions. The plan should therefore map structures conservatively during implementation based on the actual contents of `src/parseopt/help.c`.

Proposed mapping rules:

| C construct | Rust mapping |
|---|---|
| anonymous `struct` used only locally | private named `struct` in `src/parseopt/help.rs` |
| anonymous `union` or variant-style state | private `enum` if semantics are tagged; otherwise a small private `struct` with explicit fields |
| C string pointer (`char *`, `const char *`) | `String`, `&str`, or `Option<String>` / `Option<&str>` depending on ownership and nullability |
| integer fields for sizes/counts | `usize` where indexing/count semantics apply; fixed-width integers only if required by external behavior |
| flag fields / boolean state | `bool` |
| nullable pointers to owned data | `Option<T>` or `Option<Box<T>>` depending on representation need |
| borrowed references between local objects | Rust references with explicit lifetimes only if this reduces copying without complicating the port |

Data migration notes:

- Any anonymous C structure must be given a stable Rust name derived from its role in `help.c`.
- If a C structure exists only to bundle temporary values for formatting or iteration, prefer a small private Rust `struct`.
- If the C code relies on null pointers as sentinel values, model that explicitly with `Option`.
- If the file contains arrays embedded in structs, use `Vec<T>` for dynamic data and fixed-size arrays only where the C layout is truly static and behaviorally relevant.

## Implementation Phases

### Phase 1: Module Skeleton and Signature Port

- Create `src/parseopt/help.rs`.
- Establish the Rust module entry in the existing parseopt module tree.
- Port the `min` function first, selecting Rust numeric types that match the original usage sites.
- Identify all local anonymous C data structures in `help.c` and assign provisional Rust type names based on actual function-local roles.
- Compile the module with placeholders only where necessary to complete the migration incrementally.

Exit criteria:

- Rust module exists and builds within the crate structure.
- `min` is implemented and callable with the intended visibility.
- All required local types from `help.c` have a concrete Rust representation plan.

### Phase 2: Core Logic Translation

- Translate the remaining logic from `src/parseopt/help.c` directly into Rust within `src/parseopt/help.rs`.
- Preserve original control flow and formatting behavior rather than refactoring for abstraction.
- Replace pointer arithmetic and null checks with slices, indexing checks, and `Option`.
- Convert mutable temporary buffers into `String`, `Vec<u8>`, or `Vec<T>` only where the original code requires owned mutable storage.
- Keep helper functions private unless the original call graph requires broader visibility.

Memory and error-handling decisions:

- Eliminate manual allocation/free in favor of ownership-bound values.
- Represent impossible C error states explicitly where needed with `Option` or `Result`, but do not redesign the API beyond what integration requires.
- If the original code assumes infallible internal helpers, keep return types simple and use guarded conversions to avoid panics.

Exit criteria:

- All logic from `help.c` is represented in Rust.
- No raw-pointer-style ownership patterns remain unless strictly required by surrounding crate interfaces.
- The module compiles cleanly.

### Phase 3: Integration Alignment and Behavioral Checks

- Connect the Rust module to existing callers that previously depended on the C implementation.
- Verify that function signatures and output behavior match expected parse-option help behavior used by the project.
- Add focused unit tests for `min` and any translated formatting/helper logic that can be exercised in isolation.
- Add regression-style tests for edge cases observed in the C control flow, especially:
  - boundary numeric comparisons for `min`
  - empty or missing textual inputs
  - width/count calculations if present in `help.c`
  - null/sentinel-derived behavior now expressed as `Option`

Exit criteria:

- `cargo test` passes.
- Rust module behavior matches existing expectations for the migrated file.
- No unused placeholder types or dead migration scaffolding remain.

### Phase 4: Cleanup and C Replacement Completion

- Remove temporary compatibility code created during staged migration.
- Confirm the Rust file fully replaces the role of `src/parseopt/help.c` in this branch.
- Minimize visibility and keep only the types and functions required by actual module use.
- Perform final pass on integer conversions, borrowing, and string handling to ensure safe equivalents of C behavior without adding new abstractions.

Exit criteria:

- Migration for `module_src_parseopt_help.c_11` is complete on branch `108-module_src_parseopt_help.c_11-rust-port`.
- The implementation remains narrowly scoped to the original C module.
- Code is ready for review as a direct Rust port of the existing file.