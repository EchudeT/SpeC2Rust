# Implementation Plan

## Summary

This module ports the option-sorting logic currently located in `src/parseopt/help.c`, specifically the functions `optsort` and `sortnames`, into Rust on branch `104-module_src_parseopt_optsort_07-rust-port`.

The Rust implementation should preserve the current behavior and ordering semantics of the C code while reducing risks related to pointer aliasing, manual memory management, and in-place string handling. The implementation should remain narrowly scoped to the existing responsibilities of this file: sorting option-related name data used by parse/help generation paths. No new option-processing features or broader parse framework changes should be introduced.

The technical approach is to:
- migrate the relevant logic into a Rust module under the existing project structure,
- replace C pointer-based array manipulation with slice- and vector-based operations,
- model nullable/string-pointer inputs with borrowed string references or owned `String` values only where required by the surrounding call paths,
- preserve comparison and ordering behavior exactly, including any special-case name ordering implemented by `sortnames`,
- express failures through explicit Rust result types only if the original logic can actually fail; otherwise keep pure sorting helpers as infallible functions.

## Technical Context

- **Language/Version:** Rust 1.75+
- **Primary Dependencies:** Rust standard library only (`std`); no third-party crates are required by the available module evidence.
- **Testing:** `cargo test`
- **Performance Goals:**
  - Preserve asymptotic behavior equivalent to the C implementation.
  - Avoid unnecessary string cloning during sort comparisons.
  - Keep sorting operations in-memory and based on slices/vectors.
  - Maintain behavior suitable for existing help/parse option counts without introducing extra allocation-heavy abstractions.

## Module Mapping

### C to Rust File Mapping

- `src/parseopt/help.c`
  - migrate `optsort`
  - migrate `sortnames`

### Proposed Rust Module Placement

Because the source file belongs to the parse option/help area, keep the Rust placement aligned with the current structure rather than creating unrelated abstractions:

- `src/parseopt/help.rs`
  - Rust implementation of the migrated sorting helpers:
    - `optsort`
    - `sortnames`

If the project already uses a different Rust layout for `parseopt`, place these functions in the directly corresponding module file and avoid creating extra helper modules unless required by existing compilation structure.

### Function Mapping

- `optsort` (C)
  - becomes a Rust function operating on mutable slices or vectors of the corresponding option/help entry type.
  - responsibility remains limited to ordering entries.

- `sortnames` (C)
  - becomes a Rust comparison/helper function encapsulating the original name ordering rules.
  - should be used by `optsort` rather than duplicating comparison logic.

## Data Model

The analysis only identifies anonymous C data structures, so the plan should avoid inventing new broad domain types. The Rust side should map only the data actually touched by `optsort` and `sortnames`.

### Data-Structure Mapping Strategy

- **Anonymous C structs used by `help.c`**
  - map to existing named Rust structs if those have already been introduced by adjacent ports.
  - otherwise create minimal Rust structs representing only the fields needed for sorting:
    - option short name if present,
    - option long name if present,
    - any display name text or key used by comparison,
    - any ordering-relevant flags already present in the C records.

### C-to-Rust Type Guidance

- `char *` / `const char *`
  - map to `&str` when data is borrowed from stable input.
  - map to `String` only if ownership is required by the surrounding storage.
  - map nullable string pointers to `Option<&str>` or `Option<String>` depending on ownership.

- C arrays manipulated for sorting
  - map to `&mut [T]` when sorting existing contiguous storage.
  - use `Vec<T>` only if the surrounding module already owns dynamic collections.

- integer flags / category markers
  - preserve as integer types only if exact value compatibility is necessary.
  - otherwise prefer small Rust enums for local comparison clarity, but only where this does not expand the scope of the port.

### Sorting Representation

The central requirement is to represent sortable option/help entries safely:

```rust
struct HelpOptionEntry {
    short_name: Option<String>,
    long_name: Option<String>,
    // additional existing fields only if required by current C sorting behavior
}
```

This is illustrative only. If nearby module ports already define the equivalent entry type, reuse that type instead of introducing a duplicate.

### Memory Management

- Eliminate manual allocation and pointer swapping from the C implementation.
- Use Rust slice sorting (`sort_by` / `sort_unstable_by`) based on the original ordering requirements.
- Prefer borrowed field access during comparison to avoid cloning.
- Keep ownership boundaries explicit at module edges; internal sorting should mutate collections in place.

### Error Handling

- `sortnames` should return `std::cmp::Ordering` if it is purely a comparator.
- `optsort` should be infallible if the C code only performs deterministic ordering.
- If invalid or missing name data must be handled because the original code tolerates null pointers, encode that with `Option` and define ordering for `None` to match the C behavior.

## Implementation Phases

## Phase 1: Inspect and Model Existing Sort Inputs

- Identify the exact C record type(s) passed into `optsort` and read by `sortnames` inside `src/parseopt/help.c`.
- Determine which anonymous struct fields participate in comparison:
  - short option name,
  - long option name,
  - display token,
  - flags or mode fields affecting order.
- Locate the corresponding Rust type in the current branch; if absent, introduce the smallest possible struct or field additions in the parseopt help module only.
- Document nullability and string lifetime expectations so the Rust signatures use `Option` and borrowed data where appropriate.

### Deliverable
- Compiling Rust type definitions or reuse decisions for the sortable entries.
- Draft Rust signatures for `optsort` and `sortnames`.

## Phase 2: Port `sortnames` Comparison Logic

- Translate the C comparison logic into a dedicated Rust comparator.
- Preserve exact precedence rules from the C implementation, including:
  - handling of missing names,
  - ordering between short and long names if distinguished,
  - lexical comparison behavior,
  - any punctuation/prefix normalization already encoded in the C logic.
- Express comparisons with `Ordering` and avoid allocating temporary normalized strings unless the C behavior requires transformed views that cannot be compared directly.
- Add focused unit tests covering the comparator behavior from representative C cases and edge cases around absent names.

### Deliverable
- Rust `sortnames` implementation with unit tests validating comparison semantics.

## Phase 3: Port `optsort` In-Place Ordering

- Translate `optsort` to operate over the Rust collection type used by the parseopt help path.
- Replace C sorting calls or manual swapping with slice sorting using the `sortnames` comparator.
- Preserve stable/unstable ordering behavior according to the C implementation:
  - if equal-key relative order matters in current outputs, use stable sorting;
  - otherwise use the simpler in-place standard sort matching observed behavior.
- Ensure no unnecessary cloning of entry data occurs during sorting.
- Validate that the sorted output matches existing expected help/option ordering.

### Deliverable
- Rust `optsort` implementation integrated into the parseopt help module.

## Phase 4: Integration Cleanup and Regression Tests

- Connect the migrated Rust functions into the existing call path replacing the C implementation for this module scope.
- Remove or gate any now-obsolete intermediate compatibility code only where necessary for compilation on this branch.
- Add regression tests at the module level to confirm:
  - complete list ordering,
  - mixed short/long option ordering,
  - behavior with missing names or partial entries,
  - deterministic output across repeated runs.
- Run `cargo test` and fix any mismatches between comparator semantics and existing expected output.

### Deliverable
- Completed Rust module migration for `optsort` and `sortnames` with passing tests.