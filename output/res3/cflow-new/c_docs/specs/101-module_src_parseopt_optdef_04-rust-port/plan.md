# Implementation Plan

## Summary

This module ports the option-definition and help-formatting logic currently split across `src/parseopt/help.c` and `src/parseopt/parseopt.c` into a focused Rust implementation on branch `101-module_src_parseopt_optdef_04-rust-port`.

The Rust work should preserve the existing behavior and migration boundaries of the listed functions only:

- `print_arg`
- `opt_unalias`
- `merge`
- `print_option_std`
- `print_option_sdash`
- `print_option`
- `optcmp`
- `sethead`
- `sort_group`
- `find_short_name`

The implementation approach is to translate the existing option metadata handling, sorting, alias resolution, and formatted output into Rust using standard-library collections and string handling. The plan should keep the code narrowly scoped to the current module responsibilities: representing option definitions, locating short names, merging or normalizing option entries, ordering option groups, and emitting help text. Any C patterns based on raw pointers, mutable shared buffers, or in-place list rewiring should be converted into explicit ownership with `Vec`, slices, `Option`, and borrowed string views where possible.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Maintain linear or near-linear traversal behavior for option scans already present in the C code.
  - Preserve sort behavior with standard-library sorting without adding extra allocation-heavy passes beyond what is needed for safe ownership.
  - Avoid unnecessary string cloning during formatting and lookup; prefer `&str`, slices, and borrowed access where possible.
  - Keep help rendering suitable for command-line use, with no material regression relative to the C implementation.

## Module Mapping

### Source File Mapping

| C File | Rust Target | Notes |
|---|---|---|
| `src/parseopt/help.c` | `src/parseopt/help.rs` | Move help-printing and option display functions into a direct Rust module. |
| `src/parseopt/parseopt.c` | `src/parseopt/parseopt.rs` | Move option-definition manipulation, alias normalization, grouping, sorting, and lookup functions into a direct Rust module. |

### Function Mapping

| C Function | Rust Location | Rust Form |
|---|---|---|
| `print_arg` | `src/parseopt/help.rs` | Private helper function for argument-name formatting. |
| `print_option_std` | `src/parseopt/help.rs` | Private helper for standard option rendering. |
| `print_option_sdash` | `src/parseopt/help.rs` | Private helper for single-dash rendering path. |
| `print_option` | `src/parseopt/help.rs` | Module-visible function coordinating option formatting. |
| `opt_unalias` | `src/parseopt/parseopt.rs` | Internal normalization function returning canonical option references or indexes. |
| `merge` | `src/parseopt/parseopt.rs` | Internal helper combining option/group state during migration of current logic. |
| `optcmp` | `src/parseopt/parseopt.rs` | Comparator logic expressed as `Ord`-like helper or closure used by sorting. |
| `sethead` | `src/parseopt/parseopt.rs` | Internal helper for assigning or updating group/header association. |
| `sort_group` | `src/parseopt/parseopt.rs` | Function performing in-place sort of grouped option entries. |
| `find_short_name` | `src/parseopt/parseopt.rs` | Lookup helper returning matching short-name entry/index. |

### Rust Module Structure

Keep the Rust structure minimal and aligned to the original files:

```text
src/
  parseopt/
    mod.rs
    help.rs
    parseopt.rs
```

`mod.rs` should only expose the items already required by the wider parseopt subsystem. Do not introduce extra facade modules.

## Data Model

The analysis only identifies anonymous C data structures, so the Rust plan should derive concrete named types from actual usage in these two source files rather than inventing broader abstractions.

### Mapping Strategy

| C Pattern | Rust Mapping | Notes |
|---|---|---|
| Anonymous struct used as option descriptor | `struct OptionDef` | Central named struct capturing fields actually referenced by the listed functions. |
| Anonymous struct used as group/header record | `struct OptionGroup` or `struct GroupHead` | Use one or two structs only if the C code clearly separates concepts. |
| Linked or array-based option collections | `Vec<OptionDef>` / slices | Prefer index-based access if the original code relies on stable relative ordering. |
| Nullable pointer to related option/group | `Option<usize>` or `Option<&T>` | Use indexes for relationships that survive sorting or mutation. |
| C string pointer fields | `Option<String>` or `String` | Use `Option` where null has semantic meaning; otherwise owned `String`. |
| Short option character | `Option<char>` or `u8` | Prefer `char` if the code treats it textually; use `u8` only if raw byte semantics are needed. |
| Flags/bitfields | `u32` or small newtype wrapper | Keep raw integer flags initially if the C code uses bit masks extensively. |
| Output accumulator / FILE writes | `fmt::Write` target or `String` buffer | Choose the narrowest interface already needed by calling code. |

### Proposed Core Rust Types

These names are implementation placeholders and should be finalized after field extraction from the C files:

```rust
struct OptionDef {
    long_name: Option<String>,
    short_name: Option<char>,
    arg_name: Option<String>,
    help_text: Option<String>,
    flags: u32,
    alias_of: Option<usize>,
    group: Option<usize>,
    // additional migrated fields only if referenced by listed functions
}

struct OptionGroup {
    header: Option<String>,
    option_indexes: Vec<usize>,
}
```

If the C code stores group membership directly on option records and only needs transient sorting, avoid adding `option_indexes` and instead sort `Vec<OptionDef>` or a parallel index vector in place. The goal is to model only what the current functions require.

### Ownership and Memory Management

- Replace C pointer ownership with Rust-owned `Vec` and `String`.
- Convert null-pointer checks into `Option`.
- Avoid self-referential structures; represent cross-links such as aliases or group heads with indexes into a stable vector.
- If sorting would invalidate references, sort index vectors or perform all cross-link resolution before reordering.
- Keep formatting helpers borrowing `&OptionDef` and `&str` instead of cloning.

### Error Handling

The C functions likely encode absence or failure through null returns, sentinel values, or silent no-op behavior. In Rust:

- Use `Option<T>` for “not found” cases such as `find_short_name` and alias absence.
- Use `Result<T, E>` only where the original logic can genuinely fail in a way the caller must handle, such as malformed internal state discovered during merge/unalias operations.
- For help rendering, prefer infallible string formatting where possible; if writing into a generic output sink, return `fmt::Result`.

## Implementation Phases

## Phase 1: Extract and Define Rust Data Structures

- Inspect `help.c` and `parseopt.c` to enumerate the actual anonymous struct fields touched by the listed functions.
- Introduce the minimal named Rust structs in `src/parseopt/parseopt.rs` or a nearby shared location inside the same module subtree.
- Map nullable relationships to `Option`, and pointer-based cross-references to indexes.
- Preserve existing flag representations as integer masks unless the C code clearly benefits from a tiny enum for local readability.
- Add basic unit tests for data-model invariants that matter to later logic:
  - alias reference presence/absence
  - optional short and long names
  - group/header attachment presence

## Phase 2: Port Lookup, Alias, Merge, and Ordering Logic

- Port `find_short_name`, `opt_unalias`, `merge`, `sethead`, `optcmp`, and `sort_group` into `src/parseopt/parseopt.rs`.
- Translate pointer iteration into slice or `Vec` traversal.
- Implement comparator behavior exactly enough to preserve current ordering semantics.
- If original sorting depends on custom comparison involving null fields, encode that explicitly rather than relying on derived ordering.
- Keep mutation localized:
  - use `&mut [OptionDef]` or `&mut Vec<OptionDef>` for in-place normalization
  - use index-based helper returns where alias/group relations must survive sorting
- Add focused unit tests around:
  - short-name lookup success/failure
  - alias resolution to canonical options
  - merge behavior for overlapping or grouped definitions
  - stable expected sort order for mixed option inputs

## Phase 3: Port Help and Option Formatting Functions

- Port `print_arg`, `print_option_std`, `print_option_sdash`, and `print_option` into `src/parseopt/help.rs`.
- Replace C output operations with standard Rust formatting:
  - either build into `String`
  - or write through `core::fmt::Write` / `std::fmt::Write`, depending on surrounding code expectations
- Preserve current presentation decisions:
  - short vs long option spelling
  - argument placeholder formatting
  - alias/canonical rendering distinctions if present
  - spacing and header behavior required by existing output
- Keep helper boundaries similar to the C code so behavior stays traceable during review.
- Add unit tests with exact or near-exact rendered output for representative options:
  - short-only
  - long-only
  - short+long
  - option with argument
  - single-dash special case

## Phase 4: Integrate and Validate Module-Level Behavior

- Wire `help.rs` and `parseopt.rs` into the existing Rust parseopt module tree without broadening public API exposure.
- Update call sites on this branch to use the new Rust functions and types, staying within the scope of the migrated files.
- Run `cargo test` and add integration-style tests only for behavior already exercised across these two source files.
- Verify final memory-safety conditions:
  - no dangling cross-references after sorting
  - no unnecessary clones in hot formatting/lookup paths
  - all absent-name and absent-group cases represented explicitly with `Option`
- Keep any remaining compatibility shims minimal and local to the module.