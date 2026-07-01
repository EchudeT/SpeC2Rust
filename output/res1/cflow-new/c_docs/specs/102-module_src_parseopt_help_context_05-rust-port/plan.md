# Implementation Plan: module_src_parseopt_help_context_05

## Summary

This module covers the Rust port of the help/option-display logic currently implemented in `src/parseopt/help.c`, specifically the migration of `sort_options` and `print_option_group`.

The Rust implementation should remain narrowly aligned with the existing C behavior: preserve current option ordering rules, group-print formatting behavior, and call sequencing, while replacing pointer-driven data access with borrowed slices and explicit ownership. The implementation should prefer plain Rust data types and iterator-based sorting/formatting over introducing new abstraction layers.

The technical approach is:

- migrate the logic in `src/parseopt/help.c` into a Rust module under the standard `src/` tree;
- represent option/group records with Rust structs or references to already-ported parse-option types;
- replace in-place C sorting over raw arrays with slice-based sorting using `sort_by`;
- replace C string/pointer formatting flow with `&str`, `String`, and `fmt::Write` or direct writer-based output;
- preserve observable formatting and ordering behavior through focused tests derived from current C behavior.

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Maintain asymptotic behavior equivalent to the C implementation for option sorting and group printing.
  - Avoid unnecessary heap allocation beyond output assembly already required by the original behavior.
  - Operate primarily on slices and borrowed string data where possible.
  - Keep formatting overhead predictable and suitable for command-line help generation paths.

## Technical Context Details

### Constraints

- The port is limited to functionality presently in `src/parseopt/help.c`.
- No new user-facing capabilities should be introduced.
- Rust structure should follow existing project layout and only add the minimum module surface required for this migration.
- Error handling should reflect actual failure points in printing/formatting only, not introduce speculative recovery paths.

### Recommended Standard-Library Usage

- `std::cmp::Ordering` for sort comparisons
- `std::fmt` and possibly `std::fmt::Write` for string-oriented formatting
- `std::io::Write` only if the surrounding Rust code already prints help text through writer APIs
- slices (`&[T]`, `&mut [T]`) instead of raw arrays/pointers
- `Option` instead of nullable pointers
- `enum` only where the C code clearly encodes a tagged variant or state

## Module Mapping

### C to Rust File Mapping

- **C source**: `src/parseopt/help.c`
- **Rust target**: `src/parseopt/help.rs`

If the Rust port already has a parseopt module root, expose this file through the existing module declaration only. Do not introduce extra helper submodules unless required by existing Rust project structure.

### Function Mapping

- `sort_options`
  -> `sort_options` in `src/parseopt/help.rs`
- `print_option_group`
  -> `print_option_group` in `src/parseopt/help.rs`

Function names may remain close to the C originals to keep migration traceable. Signature changes should be limited to idiomatic Rust borrowing and result handling.

### Expected Signature Direction

Because exact C signatures are not provided, the Rust signatures should be shaped by the surrounding already-ported parseopt types:

- `sort_options`
  - input should become a mutable slice of option/group entries or references
  - output should be in-place reordering
  - no ownership transfer unless required by surrounding module design

- `print_option_group`
  - input should borrow the group/options context
  - return type should be:
    - `()` if appending into a supplied buffer cannot fail, or
    - `fmt::Result` / `io::Result<()>` if writing to a formatter/writer
  - formatting state should be passed explicitly rather than hidden through globals

## Data Model

The analysis reports only anonymous C data structures, so the data-model plan must remain conservative and tied to actual fields found in `src/parseopt/help.c` during implementation.

### Mapping Strategy

Anonymous C structs used only within `help.c` should be converted as follows:

- **file-local temporary record with fixed fields**
  - -> private Rust `struct`
- **nullable pointer to optional related object/text**
  - -> `Option<&T>`, `Option<&str>`, or `Option<String>` depending on ownership
- **integer flags or category markers**
  - -> retain integer type initially if behavior depends on exact values
  - -> convert to `enum` only if the C code has a clear closed set used in branching
- **pointer + count pairs**
  - -> slices (`&[T]` / `&mut [T]`)
- **mutable output buffers**
  - -> `String`, `fmt::Write`, or borrowed writer depending on call site

### Planned Rust Data Representations

Since the C analysis lists only anonymous structures, use the following implementation rule:

| C construct in `help.c` | Rust representation |
|---|---|
| Anonymous local aggregation struct | Private `struct` in `src/parseopt/help.rs` |
| Raw pointer to option record | Borrowed reference `&OptionRecord` or mutable reference `&mut OptionRecord` |
| Raw pointer to group record | Borrowed reference `&OptionGroup` |
| Null string pointer | `Option<&str>` or `Option<String>` |
| C string used read-only | `&str` where UTF-8 is guaranteed by surrounding Rust port; otherwise defer conversion at module boundary |
| Array + length | Slice |
| Comparator over raw elements | `slice.sort_by(...)` |

### Ownership and Lifetime Notes

- Sorting should operate on borrowed mutable slices, avoiding cloning underlying option definitions.
- Printing should borrow option/group data and avoid copying strings except where formatted line assembly requires it.
- Temporary sort keys or display fragments should be stack-local where feasible.
- If current parseopt records are shared across modules, this module should hold references rather than owning converted duplicates.

### Error Handling and Safety

- Eliminate all null-pointer and bounds risks by converting arrays to slices and optional fields to `Option`.
- If writing to an output sink can fail, expose that failure using `Result` and propagate with `?`.
- Avoid `unsafe` unless forced by an unported API boundary; the target implementation for this module should be fully safe Rust.

## Implementation Phases

## Phase 1: Port Structure and Type Alignment

### Goals

Create the Rust module file and align the function/type surface with the existing parseopt Rust port.

### Tasks

- Add `src/parseopt/help.rs`.
- Identify the Rust equivalents of the option and group records referenced by `help.c`.
- Define any private helper structs needed to replace anonymous C local structures.
- Establish minimal public/internal visibility matching the existing call graph.
- Convert C function signatures into Rust signatures using:
  - borrowed references for input records,
  - mutable slices for sortable collections,
  - explicit return types for output/failure.

### Completion Criteria

- Module compiles with placeholder or partial bodies.
- All anonymous C helper structures used by `sort_options` and `print_option_group` have a concrete Rust representation or are mapped to existing ported types.
- No raw-pointer-based interfaces remain inside the module boundary.

## Phase 2: Sorting Logic Migration

### Goals

Implement `sort_options` with behavior equivalent to the C version.

### Tasks

- Translate the original comparison logic into a Rust comparator using `sort_by`.
- Preserve the original ordering semantics, including any grouping, priority, or lexical comparisons present in the C code.
- Avoid widening behavior: if the C code relies on existing unstable ordering characteristics, reproduce the effective behavior as closely as possible from the observed rules.
- Use borrowed field access and explicit `Option` handling instead of pointer checks.

### Validation

- Add unit tests covering:
  - empty input,
  - single option,
  - mixed ordering cases required by the C comparison rules,
  - ties or near-ties if the original logic distinguishes them.

### Completion Criteria

- `sort_options` passes tests and produces expected order for representative option sets.
- Implementation uses safe Rust only.
- No unnecessary cloning of option/group records is introduced.

## Phase 3: Group Printing Migration

### Goals

Implement `print_option_group` with output matching the C help text formatting.

### Tasks

- Translate the formatting flow from `help.c` into Rust with explicit writer/buffer handling.
- Preserve:
  - group heading emission rules,
  - indentation/padding behavior,
  - per-option line construction,
  - conditional text inclusion behavior.
- Keep formatting logic localized to this module rather than introducing broader presentation abstractions.
- Propagate output errors only if the selected writer API can fail.

### Validation

- Add unit tests for:
  - empty group behavior,
  - group with one option,
  - group with multiple options,
  - formatting cases involving optional text fields or alignment-sensitive output.

### Completion Criteria

- Output matches expected formatting captured from the C behavior.
- Printing path handles optional fields without panics.
- The implementation is limited to existing help/group output behavior.

## Phase 4: Integration and Behavioral Verification

### Goals

Connect the ported functions to the surrounding parseopt Rust module and confirm parity with the C module behavior.

### Tasks

- Wire `help.rs` into the existing Rust module tree.
- Replace remaining call sites targeting the C-side behavior with the Rust functions for this module scope.
- Reconcile any type mismatches between this module and previously ported parseopt records using the smallest possible changes.
- Finalize tests around end-to-end help generation paths that exercise both sorting and group printing together.

### Completion Criteria

- `cargo test` passes.
- The Rust module covers the behavior of `sort_options` and `print_option_group` from `src/parseopt/help.c`.
- Integration changes remain limited to what is necessary for this file/function migration.

## Acceptance Checklist

- [ ] `src/parseopt/help.c` mapped to `src/parseopt/help.rs`
- [ ] `sort_options` migrated
- [ ] `print_option_group` migrated
- [ ] Anonymous C helper structures replaced with private Rust structs or existing ported types
- [ ] Raw pointers replaced with references, slices, and `Option`
- [ ] Output/error handling made explicit and safe
- [ ] Unit tests added for ordering and formatting behavior
- [ ] `cargo test` passes