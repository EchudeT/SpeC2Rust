# plan.md

## Summary

This module ports the option-help sorting logic from `src/parseopt/help.c` into Rust for branch `104-module_src_parseopt_optsort_07-rust-port`. The scope is limited to migrating the behavior of `optsort` and `sortnames` and the data access patterns they depend on, without adding new features or restructuring unrelated parsing/help functionality.

The Rust implementation should keep the original operational model: collect or view existing option records, compare their display names using the same ordering rules as the C code, and produce a sorted sequence suitable for downstream help formatting. The preferred technical approach is to translate the existing comparison and sorting logic directly into idiomatic Rust using slices and `sort_by`, while preserving observable ordering behavior, nullability assumptions, and any implicit tie-breaking present in the source.

Where the C code sorts mutable arrays of pointers or structures in place, the Rust port should use borrowed views over existing option entries where possible, minimizing allocation and keeping ownership boundaries explicit. Error handling should remain simple and local; if the original logic assumes valid inputs, Rust code should express that through types and narrow internal assertions rather than introducing broader recovery behavior.

## Technical Context

- **Language/Version**: Rust 1.76+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve the asymptotic behavior of the C implementation, with standard-library sort performance acceptable for the module scope.
  - Avoid unnecessary string cloning during comparisons and sorting.
  - Keep allocations limited to any container required to hold sortable references or indices, if in-place mutation of the owning collection is not appropriate.
  - Maintain predictable ordering equivalent to the C implementation for the same input set.

## Module Mapping

### C to Rust File Mapping

- `src/parseopt/help.c`
  - Migrate the `optsort` and `sortnames` logic into a Rust module located under the existing Rust crate structure for parse-option/help behavior.
  - Recommended target file:
    - `src/parseopt/help.rs`
  - If the current Rust project already splits parse-option code across files, place these functions in the existing corresponding module rather than introducing a new abstraction layer.

### Function Mapping

- `optsort`
  - Rust function with module-private or crate-visible scope matching current call sites.
  - Responsible for arranging option entries or references into the required order.
  - Implement using `slice::sort_by` or sorting of an auxiliary `Vec<&OptionEntry>` / `Vec<usize>` if ownership constraints prevent direct in-place sorting.

- `sortnames`
  - Rust comparison helper used by `optsort`.
  - Implement as a dedicated comparator function operating on borrowed option-name data.
  - Return `std::cmp::Ordering` rather than integer comparison codes.

## Data Model

The analysis only identifies anonymous C data structures and does not provide named struct layouts. The plan should therefore avoid inventing new domain models and instead map onto the existing Rust representations already used by the surrounding parse-option/help code.

### Data-Structure Mapping Strategy

- **Anonymous C structs used by `help.c`**
  - Map to existing Rust structs/enums already representing:
    - option descriptors
    - short and long option names
    - help/display metadata
    - sortable collections of option entries

### Expected Rust Representations

Given typical C-to-Rust migration patterns for this area, use the following constrained mappings only where required by the existing codebase:

- `char *` / `const char *`
  - `&str` when UTF-8 text is already guaranteed by the Rust-side parser/help model
  - `&CStr` only if the surrounding port still retains C-compatible string storage internally
- arrays of option records or pointers
  - `&mut [T]`, `Vec<T>`, `Vec<&T>`, or `Vec<usize>` depending on ownership and mutation requirements
- nullable pointers to optional names/fields
  - `Option<&str>` or `Option<FieldType>`
- integer comparator return values
  - `std::cmp::Ordering`

### Memory Management Notes

- Replace pointer-based ownership with Rust borrowing wherever the sorted data is only viewed.
- If the C code sorts an array of pointers to externally owned records, prefer `Vec<usize>` or `Vec<&OptionRecord>` in Rust rather than cloning full records.
- Avoid heap allocation for temporary string normalization unless the original algorithm explicitly requires transformed comparison keys.
- Encode absence of short or long names with `Option` instead of sentinel null pointers.

### Error Handling Notes

- Keep function signatures narrow and deterministic.
- If invalid state is impossible after prior parsing/initialization, represent that in types rather than adding recoverable error flows.
- If a comparator depends on required fields, ensure those fields are present before sorting or use internal assertions in private code paths.

## Implementation Phases

## Phase 1: Inspect and Anchor the Existing Rust Data Types

- Locate the Rust module that already owns parse-option help/descriptor data.
- Identify the exact Rust types corresponding to the anonymous C structures referenced by `optsort` and `sortnames`.
- Determine whether the original C code sorts:
  - full option records,
  - pointers to option records,
  - or a separate list of display names.
- Define the minimum Rust function signatures needed to match existing call paths, using current project types instead of introducing new wrapper models.
- Record any nullable fields and ordering assumptions that must be preserved in the comparator.

**Deliverable**: concrete Rust targets for `optsort` and `sortnames`, with confirmed input/output types and no new module expansion.

## Phase 2: Port the Name Comparison Logic

- Translate `sortnames` first as an isolated Rust comparator.
- Preserve the exact comparison sequence from C, including:
  - precedence between short and long names if applicable,
  - handling of missing names,
  - lexical comparison behavior,
  - and any fallback/tie-break rules.
- Express comparisons using borrowed data and `Ordering`.
- Add focused unit tests covering:
  - entries with both names present,
  - entries with only one name present,
  - equal-prefix or tie cases,
  - and missing/optional field handling reflected in the original logic.

**Deliverable**: working Rust comparator with tests that lock down ordering behavior.

## Phase 3: Port the Sorting Procedure

- Translate `optsort` using the comparator from Phase 2.
- Choose the least invasive sorting representation based on Phase 1 findings:
  - `sort_by` on `&mut [T]` if direct mutation matches ownership,
  - otherwise sort a temporary vector of references or indices.
- Preserve the original function’s side effects and output shape expected by callers.
- Ensure no unnecessary cloning of option records or strings is introduced.
- Verify that lifetime and borrowing rules still permit downstream help-generation code to consume the sorted results cleanly.

**Deliverable**: Rust `optsort` integrated into the target module and replacing the C-path behavior for this functionality.

## Phase 4: Integration Cleanup and Regression Tests

- Remove or bypass the migrated C implementation for this module within the Rust port boundary.
- Add integration-level tests at the parseopt/help module level that validate sorted output ordering on representative option sets.
- Confirm behavior for edge inputs that matter to the original code path, such as:
  - empty option lists,
  - single-element lists,
  - mixed short/long option populations.
- Run `cargo test` and resolve any borrow, lifetime, or ordering mismatches without broadening scope.

**Deliverable**: completed Rust migration of `optsort` and `sortnames`, validated by unit and integration tests.