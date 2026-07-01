# Implementation Plan

## Summary

Port the option-definition and help-formatting logic currently split across `src/parseopt/help.c` and `src/parseopt/parseopt.c` into a focused Rust module that preserves existing behavior and call structure as closely as practical. The migration scope is limited to the listed functions:

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

The Rust implementation should keep the original responsibilities intact:

- formatting option/help text
- resolving aliases to canonical options
- comparing and sorting options/groups
- locating short option names
- preparing heading/group presentation state

Technical approach:

- Translate the C logic into one Rust module cluster under the existing parseopt area, keeping file boundaries aligned with the source where feasible.
- Replace pointer-heavy C data access with borrowed references and slices.
- Represent optional pointers and nullable links with `Option<T>` / `Option<&T>` / indices as appropriate.
- Keep formatting output based on `std::fmt` / `String` / `Write` rather than introducing extra abstraction.
- Preserve sort and merge semantics using standard library slice sorting and explicit merge routines where the original ordering behavior matters.
- Convert implicit C error cases into explicit return values only where required by the existing function behavior; otherwise keep functions infallible and deterministic.

## Technical Context

### Language / Version

- Rust 1.78+ (edition 2021)

### Primary Dependencies

Use the Rust standard library by default.

Recommended crates:

- none required for this module port

### Testing

- `cargo test`

Testing scope should include:

- direct unit tests for option comparison and sorting behavior
- formatting tests for standard and single-dash option rendering
- alias resolution tests for canonical-option lookup
- short-name lookup tests
- regression-style tests that validate stable output for representative option sets

### Performance Goals

- Maintain behavior with performance at least comparable to the C implementation for typical command-line option set sizes.
- Avoid unnecessary allocation during formatting and merge/sort operations.
- Keep sorting and lookup logic within standard expected complexity:
  - sort operations: `O(n log n)`
  - linear scans where the original code uses linear traversal
- Prefer borrowing over cloning for option metadata.

## Module Mapping

Map the C sources into Rust with minimal expansion of structure:

| C File | Rust File | Notes |
|---|---|---|
| `src/parseopt/help.c` | `src/parseopt/help.rs` | Port help/printing-related functions and local formatting helpers. |
| `src/parseopt/parseopt.c` | `src/parseopt/parseopt.rs` | Port option-definition helpers, alias resolution, grouping, comparison, and search logic. |

Function-level mapping:

| C Function | Rust Location | Rust Shape |
|---|---|---|
| `print_arg` | `src/parseopt/help.rs` | private helper function operating on borrowed option/argument metadata |
| `print_option_std` | `src/parseopt/help.rs` | private formatting helper |
| `print_option_sdash` | `src/parseopt/help.rs` | private formatting helper |
| `print_option` | `src/parseopt/help.rs` | module-visible function orchestrating option rendering |
| `opt_unalias` | `src/parseopt/parseopt.rs` | helper returning canonical option reference/index |
| `merge` | `src/parseopt/parseopt.rs` | helper preserving original merge behavior for grouped/sorted options |
| `optcmp` | `src/parseopt/parseopt.rs` | comparator function |
| `sethead` | `src/parseopt/parseopt.rs` | helper for heading/group metadata initialization |
| `sort_group` | `src/parseopt/parseopt.rs` | group-local sorting routine |
| `find_short_name` | `src/parseopt/parseopt.rs` | lookup helper for short option resolution |

If the current Rust crate already exposes a `parseopt` module tree, these files should be added there without introducing extra top-level modules.

## Data Model

The C analysis lists only anonymous structures, so the Rust plan should derive named internal structs/enums from actual field usage during migration rather than inventing new abstractions beyond necessity.

### Data-structure Mapping Strategy

Because the original C structs are anonymous in the analysis output, use the following conservative mapping rules when porting each accessed record:

| C Pattern | Rust Mapping | Notes |
|---|---|---|
| anonymous struct holding option metadata | named `struct` in `parseopt.rs` | Use explicit field names taken from the C source during migration. |
| anonymous struct used only for help formatting state | named `struct` in `help.rs` or shared parseopt module | Keep scope private unless cross-file access is required. |
| nullable pointer to option/group/head | `Option<usize>` for owned collection indexing, or `Option<&T>` for borrowed traversal | Prefer indices if elements are stored in vectors and need stable referencing. |
| C string pointer (`char *`, `const char *`) | `String` for owned text, `&str` for borrowed text | Choose borrowed text where data lifetime is external to the struct. |
| mutable output buffer / stream-style writes | `String` or generic `impl std::fmt::Write` | Use `String` if the original writes into assembled help text. |
| arrays with count fields | `Vec<T>` / slices | Preserve order. |
| integer flags / mode fields | `u32`, `usize`, or small `enum`/`bitflags-like constants` implemented manually | Prefer plain integer constants unless the C source clearly defines a closed set. |
| comparator return convention | `std::cmp::Ordering` internally | Convert to integer-style comparison only if external compatibility requires it. |

### Expected Rust Structures

During implementation, define only the structures necessary to replace the anonymous C records actually touched by the listed functions. Likely categories are:

- option definition record
- argument/help display record
- group/header record
- alias linkage or canonical-reference field

Guidelines for these structures:

1. **Ownership**
   - Store long-lived parsed option definitions in `Vec<...>`.
   - Use references or indices during sorting, merging, and rendering.
   - Avoid self-referential structures.

2. **Nullability**
   - Replace all nullable C pointers with `Option`.
   - Distinguish “not found” from valid zero-like values explicitly.

3. **String Data**
   - Use UTF-8 `String`/`&str`.
   - If the original logic treats strings as opaque identifiers, preserve textual equality and lexical comparison behavior as implemented in C.

4. **Ordering**
   - Use `Ordering` for internal comparisons in `optcmp`.
   - Preserve any original tie-break sequence exactly when re-implementing sort logic.

5. **Mutation**
   - Localized mutable access only for:
     - merge output assembly
     - setting header/group state
     - in-place sorting of group collections

## Implementation Phases

### Phase 1: Establish Rust module skeleton and migrate shared option/group data access

Scope:

- Create `src/parseopt/help.rs` and `src/parseopt/parseopt.rs`.
- Inspect the anonymous C structs used by the target functions and introduce the minimum named Rust structs/enums needed.
- Port low-level non-formatting helpers first:
  - `opt_unalias`
  - `find_short_name`
  - `sethead`

Technical decisions:

- Use `Vec<T>` plus indices where C code relies on stable element identity across sorting/merging.
- Replace pointer chasing with explicit borrowed parameters or collection indices.
- Keep helper visibility restricted to the module unless already required externally.

Completion criteria:

- Rust types compile.
- Canonical-option resolution, short-name lookup, and header-state setup are implemented with unit tests covering nullable/alias edge cases.

### Phase 2: Port comparison, merge, and group ordering behavior

Scope:

- Port:
  - `optcmp`
  - `merge`
  - `sort_group`

Technical decisions:

- Reproduce C ordering rules exactly, including tie-breaks and special handling for aliases, short names, long names, or group headers as observed in source.
- Use standard slice sorting APIs where they preserve required semantics; if the original merge logic encodes behavior beyond simple sorting, keep an explicit merge function rather than replacing it with a broader redesign.
- Avoid cloning option records during sorting; sort indices or lightweight references if needed.

Memory and correctness concerns:

- Ensure no invalidation of references occurs when vectors are reordered.
- Prefer sorting index vectors if merged/grouped elements are also referenced elsewhere.

Completion criteria:

- Ordering behavior is covered by deterministic unit tests.
- Merge/group outputs match expected ordering for representative mixed option sets.

### Phase 3: Port help and option rendering functions

Scope:

- Port:
  - `print_arg`
  - `print_option_std`
  - `print_option_sdash`
  - `print_option`

Technical decisions:

- Express formatting through `String` building or `fmt::Write`.
- Preserve spacing, punctuation, and dash-style conventions from the C implementation.
- Keep helper decomposition aligned to existing C functions rather than collapsing them into a new formatter abstraction.

Error handling:

- Formatting functions should be infallible unless they write into a generic formatter interface; if generic writing is used, propagate `fmt::Result`.
- Avoid lossy conversions or hidden fallback behavior.

Completion criteria:

- Formatting unit tests validate exact output for standard and single-dash cases.
- `print_option` composes the same text arrangement as the C logic for representative inputs.

### Phase 4: Integrate, verify, and replace C-path usage for this module scope

Scope:

- Wire the new Rust functions into the existing Rust crate module tree.
- Add regression-oriented tests that exercise the end-to-end behavior across sorting, alias resolution, and rendering.
- Remove or isolate any temporary compatibility scaffolding introduced during migration.

Technical decisions:

- Keep integration limited to the existing parseopt call paths that require these functions.
- Do not add unrelated refactors, shared utility crates, or extra module layers.

Completion criteria:

- `cargo test` passes.
- The Rust branch contains a complete replacement for the targeted C functionality in:
  - `src/parseopt/help.c`
  - `src/parseopt/parseopt.c`
- Public/internal interfaces are minimized to what current callers need.