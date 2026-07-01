# Implementation Plan: module_src_parseopt_optsort_07

## Summary

This module ports the option-sorting logic currently implemented in `src/parseopt/help.c` into Rust, covering the functions `optsort` and `sortnames`. The Rust implementation should preserve the existing ordering behavior and input/output expectations while replacing C-style pointer and array manipulation with safe slice- and vector-based operations.

The implementation approach is to migrate only the relevant sorting logic into a Rust module that operates on existing parsed option/help records, using standard-library sorting facilities where behavior can be matched directly. Any implicit C assumptions around null-terminated arrays, mutable shared buffers, or comparator callbacks should be translated into explicit Rust types and function signatures. The plan should avoid adding new formatting or parsing capabilities and should focus strictly on reproducing the existing sort behavior and its call pattern.

## Technical Context

- **Language/Version**: Rust 1.78+ edition 2021
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Maintain asymptotic behavior equivalent to the C implementation for option/name sorting.
  - Avoid unnecessary cloning during sorting; prefer in-place sorting over temporary reconstructed collections where ownership allows.
  - Keep allocation limited to what is required by existing Rust-side data ownership boundaries.
  - Preserve deterministic ordering semantics, including tie handling as required by the original implementation.

## Module Mapping

- **C source file**
  - `src/parseopt/help.c`

- **Rust target module**
  - `src/parseopt/help.rs`

- **Function mapping**
  - `optsort` → `optsort` in `src/parseopt/help.rs`
  - `sortnames` → `sortnames` in `src/parseopt/help.rs`

If the destination crate already centralizes parse-option logic differently, these functions should still be introduced in the closest existing `parseopt` Rust module rather than creating extra architectural layers.

## Data Model

The analysis output lists only anonymous C data structures, so the plan should treat this module as operating on pre-existing record types defined elsewhere in the port. The migration task is therefore to map the sorting-facing shape of those records, not to invent new domain models.

### Expected C-to-Rust mapping strategy

- **C anonymous struct used as sortable option/help entry**
  - **Rust mapping**: named `struct` already present in the Rust port, or a minimal new private struct in `src/parseopt/help.rs` only if no equivalent exists.
  - **Likely fields involved in sorting**:
    - option spelling / long-name text
    - short-name text or character
    - positional display name list or alias list
    - original declaration/index position if stable tie-breaking is required

- **C string pointers (`char *`, `const char *`)**
  - **Rust mapping**:
    - borrowed data: `&str`
    - owned data: `String`
  - Selection should follow the ownership model already established by surrounding parse/help code. Do not introduce `CString`/FFI types.

- **C arrays of names / aliases**
    - mutable sortable collection: `Vec<T>`
    - borrowed sortable region: `&mut [T]`

- **C sentinel-terminated lists**
    - explicit length-tracked slices/vectors
  - Null sentinels should be removed from the logic during migration.

- **C comparator callbacks used by `qsort`**
    - inline closures passed to `sort_by`
    - helper comparator function returning `std::cmp::Ordering` if reuse is needed

### Memory management and error handling decisions

- Use ownership and borrowing to eliminate manual lifetime management present in C pointer-based sorting code.
- Prefer `&mut [T]` for in-place sorting APIs when the caller owns the collection.
- If textual comparison in C assumes non-null pointers, represent that requirement in Rust through non-optional string fields whenever possible.
- If absent names are possible in the original data, model them explicitly as `Option<&str>` or `Option<String>` and define ordering to match the C behavior rather than panicking.
- Sorting functions should remain infallible unless the surrounding Rust port already expresses invalid data through `Result`. Do not add new error layers without evidence from the caller contract.

## Implementation Phases

### Phase 1: Inspect and pin down the sortable record shape

- Identify the exact data type(s) from `help.c` that are passed into `optsort` and `sortnames`.
- Determine:
  - whether sorting is over full option records, name arrays, or both
  - which fields participate in primary and secondary comparison
  - whether the original implementation depends on stable ordering for equal keys
  - whether sorting mutates shared arrays in place or produces reordered views
- Map each anonymous C structure usage to an existing Rust struct if already introduced elsewhere in the branch.
- Define the minimal Rust function signatures needed to match current call sites.

### Phase 2: Port `sortnames`

- Implement `sortnames` first as the lower-level name ordering routine.
- Replace any C string-pointer comparison with Rust string/slice comparison that preserves the original lexical rules.
- If the C code sorts arrays of aliases or display names, port this as in-place sorting over `&mut [..]` or `Vec<..>`.
- Preserve any special-case ordering behavior such as:
  - empty vs non-empty names
  - short names vs long names
  - fallback comparison when primary names match
- Add focused unit tests for:
  - ordinary lexical ordering
  - equal names / duplicate names
  - edge cases derived from nullable or empty entries if they exist in the source behavior

### Phase 3: Port `optsort`

- Implement `optsort` using the Rust record type and the comparison rules verified from the C source.
- Translate `qsort`-style logic into `sort_by` or `sort_unstable_by` based on required semantics:
  - use `sort_by` if equal-element stability matters or if the C behavior effectively preserved encounter order
  - use `sort_unstable_by` only if the source logic is clearly indifferent to equal-key order
- Ensure `optsort` delegates to `sortnames` where the C implementation does so, rather than duplicating name-sorting logic.
- Preserve in-place mutation semantics expected by callers.

### Phase 4: Integrate and validate against call sites

- Wire the Rust implementations into the existing parse/help path on branch `104-module_src_parseopt_optsort_07-rust-port`.
- Update or add tests at the module level to verify:
  - sorted output order for representative option sets
  - compatibility with existing help/parse data structures
  - no behavioral drift in tie-breaking or alias ordering
- Remove any temporary compatibility code introduced during migration and keep the final module surface limited to the migrated functions.