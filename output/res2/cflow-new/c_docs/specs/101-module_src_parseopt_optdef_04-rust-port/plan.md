# Implementation Plan

## Summary

This module ports the option-definition and help-formatting logic currently split across `src/parseopt/help.c` and `src/parseopt/parseopt.c` into Rust, preserving existing behavior and migration boundaries.

The Rust implementation should focus on the existing responsibilities only:

- locating short option names
- resolving option aliases
- comparing and sorting options/groups
- merging option-definition views
- formatting options and arguments for help output

The preferred technical approach is a direct, low-expansion translation into a small Rust module cluster that keeps parsing metadata in borrowed or owned Rust structs, uses slice-based sorting instead of pointer arithmetic, and replaces C string manipulation with `String`/`&str` formatting. Memory ownership should be explicit, with optional references represented by `Option<T>` and failure paths represented by `Result` only where the original code can actually fail; otherwise preserve current control flow with plain return values.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve current asymptotic behavior for option lookup, alias resolution, merge, and sorting paths.
  - Avoid unnecessary heap allocation in formatting and comparison helpers.
  - Keep help rendering linear in the number of options, aside from required sorting steps.
  - Use in-place slice sorting where the C code currently sorts arrays.

## Module Mapping

### Source File Mapping

| C File | Rust Target | Notes |
|---|---|---|
| `src/parseopt/help.c` | `src/parseopt/help.rs` | Migrate help/output formatting helpers only. |
| `src/parseopt/parseopt.c` | `src/parseopt/parseopt.rs` | Migrate option-definition transformation, aliasing, lookup, merge, and sorting helpers. |

### Function Mapping

| C Function | Rust Target | Migration Notes |
|---|---|---|
| `print_arg` | `parseopt::help::print_arg` | Convert C string output logic to formatter/string-appending helper. |
| `print_option_std` | `parseopt::help::print_option_std` | Preserve standard long/short option formatting layout. |
| `print_option_sdash` | `parseopt::help::print_option_sdash` | Preserve single-dash formatting rules. |
| `print_option` | `parseopt::help::print_option` | Dispatch helper over option style/shape. |
| `opt_unalias` | `parseopt::parseopt::opt_unalias` | Resolve aliases using explicit references/indexes instead of raw pointers. |
| `merge` | `parseopt::parseopt::merge` | Port array/list merge logic with slices or `Vec`. |
| `optcmp` | `parseopt::parseopt::optcmp` | Implement as comparison function returning `Ordering`. |
| `sethead` | `parseopt::parseopt::sethead` | Preserve header/group-linking behavior with explicit mutable fields. |
| `sort_group` | `parseopt::parseopt::sort_group` | Replace C sort routine usage with Rust slice sorting. |
| `find_short_name` | `parseopt::parseopt::find_short_name` | Implement direct scan over option definitions. |

### Rust Module Layout

```text
src/
  parseopt/
    mod.rs
    help.rs
    parseopt.rs
```

`mod.rs` should expose only the items already needed by surrounding code. Do not introduce extra abstraction layers beyond what is needed to host the migrated functions.

## Data Model

The analysis lists only anonymous C data structures, so the Rust data model should be derived from actual field usage in these two files during migration. The plan should keep the number of Rust types minimal and mirror the existing layout closely.

### Data-Structure Mapping Strategy

| C Pattern | Rust Mapping | Notes |
|---|---|---|
| Anonymous option-definition struct | Named `struct` in `parseopt` module | Create one concrete Rust struct once fields are confirmed from usage. |
| Anonymous group/header struct | Named `struct` in `parseopt` module | Use explicit fields instead of embedded pointer manipulation. |
| Anonymous temporary merge/sort node | Named `struct` only if required | Prefer tuples/local variables if the structure is purely local. |
| C string pointer fields (`char *`, `const char *`) | `String` or `&str` | Use borrowed `&str` where lifetime is naturally tied to input tables; use `String` only when mutation/ownership is required. |
| Optional linked object pointer | `Option<usize>` or `Option<&T>` / `Option<&mut T>` | Prefer index-based references for sortable collections to avoid borrow conflicts. |
| Flag/integer classification fields | `u32`, `usize`, or small enums | Introduce enums only where they directly replace stable tag-like values used by formatting or sorting logic. |
| C comparator result | `std::cmp::Ordering` | Convert to idiomatic Rust sorting/comparison. |

### Ownership and Lifetime Decisions

- Use `Vec<T>` for mutable option/group collections that are sorted or merged.
- Use indices instead of self-referential pointers when one item refers to another item in the same collection.
- Avoid interior mutability; ordinary mutable borrowing should be sufficient for this migration.
- Convert null-pointer checks into `Option`.
- Keep formatting functions mostly borrow-based to avoid cloning names/descriptions.

### Error Handling

- Functions that only inspect or reorder validated option data should return plain values.
- Use `Option` for not-found cases such as short-name lookup or alias target absence if the original code permits that state.
- Use `Result` only for operations that can genuinely fail in Rust, such as writing into a formatter if implemented over `fmt::Write`.

## Implementation Phases

## Phase 1: Establish Rust data structures and direct function skeletons

- Create `src/parseopt/help.rs` and `src/parseopt/parseopt.rs`.
- Identify all anonymous C structs used by these functions and replace them with the minimum named Rust structs/enums needed for compilation.
- Define Rust signatures for:
  - `print_arg`
  - `print_option_std`
  - `print_option_sdash`
  - `print_option`
  - `opt_unalias`
  - `merge`
  - `optcmp`
  - `sethead`
  - `sort_group`
  - `find_short_name`
- Decide collection ownership:
  - sortable/mergeable groups as `Vec<_>`
  - cross-references as indices or `Option<usize>`
- Keep signatures close to call sites in the existing module cluster to reduce broader edits.

## Phase 2: Port option-definition lookup, alias, merge, and sort behavior

- Implement `find_short_name` as a direct scan over the Rust option collection.
- Implement `opt_unalias` with explicit alias traversal/resolution, preserving original semantics for direct and already-resolved options.
- Implement `optcmp` using `Ordering`, matching the C comparison precedence exactly.
- Implement `sethead` by rewriting pointer-updating logic into field assignments on owned structs.
- Implement `merge` against slices/`Vec`, keeping original ordering and overwrite/selection behavior.
- Implement `sort_group` using Rust slice sorting and the `optcmp` comparator.

Focus testing in this phase on deterministic structural behavior:
- sorted order
- alias resolution outcome
- short-name lookup outcome
- merge result shape

## Phase 3: Port help-formatting functions

- Implement `print_arg` with equivalent argument-name rendering rules.
- Implement `print_option_std` and `print_option_sdash` as the two existing formatting paths without introducing new formatting modes.
- Implement `print_option` as the dispatcher that chooses the correct formatting helper and emits argument text as needed.
- Use `String` building or `fmt::Write` depending on surrounding call sites; avoid changing external output contracts more than necessary.

Focus testing in this phase on output equivalence:
- option with short and long names
- single-dash special form
- option with/without argument placeholder
- alias display behavior if applicable through existing call paths

## Phase 4: Integrate and stabilize against existing module behavior

- Wire the new Rust module into the project branch in place of the C implementation for this module boundary.
- Adjust only the immediate call sites required by ownership and signature changes.
- Add targeted unit tests reflecting migrated behavior from both source files.
- Verify there are no remaining raw C assumptions:
  - null sentinel logic replaced
  - array bounds explicit
  - string lifetime/ownership explicit
  - sorting and merge operations free of aliasing issues

## Notes and Constraints

- Keep the migration limited to the two listed source files and their directly required data representations.
- Do not introduce new public APIs beyond the Rust equivalents needed to replace these C functions.
- Prefer standard-library formatting, comparison, and sorting facilities over third-party crates.
- Preserve current behavior first; defer refactoring unless needed to express ownership safely in Rust.