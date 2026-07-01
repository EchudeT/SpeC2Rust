# Implementation Plan: module_src_output_symbol_09

## Summary

This module ports the symbol-output path currently split across `src/gnu.c`, `src/output.c`, and `src/posix.c` into a Rust implementation on branch `072-module_src_output_symbol_09-rust-port`.

The C analysis indicates three `print_symbol` implementations distributed by source file, which strongly suggests format- or mode-specific symbol rendering rather than unrelated behaviors. The Rust port should preserve that separation at the file/module level and migrate the existing logic with minimal restructuring. The main technical approach is:

- keep one Rust module aligned to each contributing C file,
- move each existing `print_symbol` variant into the corresponding Rust function,
- consolidate shared call signatures only where required by Rust typing,
- represent C anonymous structures as named Rust structs only when they are directly touched by the migrated functions,
- replace implicit C ownership and null-based flow with borrowed references, `Option`, and `Result` only where failure is already meaningful in the C code path.

The implementation should favor direct translation of control flow and output behavior, avoiding new abstraction layers beyond what is necessary to express the current file-local responsibilities safely in Rust.

## Technical Context

- **Language/Version:** Rust 1.78 or newer
- **Primary Dependencies:** Rust standard library only
- **Testing:** `cargo test`
- **Performance Goals:**
  - Preserve current asymptotic behavior of symbol formatting/output paths.
  - Avoid unnecessary heap allocation during symbol rendering where C code writes directly to output streams.
  - Keep per-symbol formatting overhead close to the C implementation by using borrowed string/data views where possible.
  - Ensure no additional full-copy buffering is introduced unless required by Rust API boundaries.

## Module Mapping

### C to Rust File Mapping

| C File | Rust Module/File | Migration Notes |
|---|---|---|
| `src/output.c` | `src/output.rs` | Primary home for the generic/shared `print_symbol` logic currently owned by output handling. |
| `src/gnu.c` | `src/gnu.rs` | Port the GNU-specific `print_symbol` variant with behavior kept local to GNU formatting rules. |
| `src/posix.c` | `src/posix.rs` | Port the POSIX-specific `print_symbol` variant with behavior kept local to POSIX formatting rules. |

### Function Mapping

| C Function | Rust Function | Notes |
|---|---|---|
| `print_symbol` in `src/output.c` | `output::print_symbol(...)` | Keep signature close to call sites; use references instead of raw pointers where data must be present. |
| `print_symbol` in `src/gnu.c` | `gnu::print_symbol(...)` | Preserve GNU-specific formatting decisions; do not merge with POSIX logic unless already shared in C call flow. |
| `print_symbol` in `src/posix.c` | `posix::print_symbol(...)` | Preserve POSIX-specific formatting decisions; keep branch-specific output ordering unchanged. |

### Expected Rust Crate Structure

```text
src/
  lib.rs
  output.rs
  gnu.rs
  posix.rs
```

`lib.rs` should expose only the modules and functions needed by the existing project integration. No extra helper modules should be introduced unless required to break circular dependencies created during migration.

## Data Model

The analysis only reports anonymous data structures, so the Rust plan should derive concrete named types from actual usage in these three functions rather than introducing speculative global models.

### Mapping Strategy

| C Construct | Rust Mapping | Usage Rule |
|---|---|---|
| Anonymous struct used only within one source file | Private named `struct` in the corresponding Rust module | Create only if the migrated `print_symbol` path reads/writes fields directly. |
| Anonymous struct passed read-only through pointers | Borrowed Rust struct via `&T` | Replace non-null input pointers with references when call sites guarantee presence. |
| Anonymous struct optionally present via nullable pointer | `Option<&T>` or `Option<&mut T>` | Use only where the C path checks for null explicitly. |
| Anonymous mutable state block | Named `struct` with `&mut` borrowing | Preserve mutation order and field updates exactly. |
| C enum-like integer flags | `type` alias, `const`, or small Rust `enum` | Use `enum` only if the value set is closed and visible from the migrated code. |
| C string pointer | `&str`, `&[u8]`, or `Option<&str>` | Choose based on whether input is textual and valid UTF-8 in existing program assumptions; otherwise keep byte-oriented representation. |
| Output sink / `FILE *`-style target | `&mut dyn std::fmt::Write` or `&mut dyn std::io::Write` | Select based on whether current C logic is text formatting or byte emission; avoid introducing dual abstractions unless required by existing call sites. |

### Anonymous Structure Handling

Because all listed structures are anonymous, implement the following rule set during migration:

1. Inspect each `print_symbol` implementation and identify the minimal field set it touches.
2. Introduce a Rust struct named for role, not source anonymity, for example:
   - `Symbol`
   - `OutputState`
   - `FormatOptions`
   - `NameDisplay`
3. Keep these structs private to `output.rs`, `gnu.rs`, or `posix.rs` unless the same data is already shared by the C call path across those files.
4. If multiple C files operate on the same effective object layout, define one shared Rust struct in the lowest common existing module boundary, most likely `output.rs` or `lib.rs`, but only after confirming shared field usage.

### Memory Management and Error Handling Decisions

- Replace raw ownership with borrowed references wherever the C code does not transfer ownership.
- Convert nullable incoming pointers to `Option`.
- Eliminate manual lifetime management from C by tying output/state borrows to Rust function scopes.
- For output operations:
  - use `std::fmt::Result` if the path is formatting-oriented,
  - use `std::io::Result<()>` if the path writes bytes or interacts with I/O-like sinks.
- Do not add recovery-oriented error layers; propagate write/format failures upward in the narrowest form needed.
- Preserve sentinel-driven control flow only where required for behavior compatibility; otherwise model absence explicitly with `Option`.

## Implementation Phases

## Phase 1: Establish Rust Module Skeleton and Shared Signatures

### Goals
- Create the Rust files corresponding to the three C sources.
- Identify the current `print_symbol` call signatures and the minimum shared types they require.
- Settle output API choice (`fmt::Write` vs `io::Write`) based on existing symbol emission behavior.

### Tasks
- Add `src/output.rs`, `src/gnu.rs`, and `src/posix.rs`.
- Declare placeholder `print_symbol` functions in each module with temporary Rust signatures.
- Inspect all three C implementations and their direct callers to determine:
  - required input data,
  - whether pointers are always non-null,
  - whether output is textual formatting or byte-oriented writing.
- Introduce only the minimal shared named structs/enums needed to express the migrated arguments.
- Wire modules through `lib.rs` without adding extra facades.

### Deliverables
- Compiling Rust module skeleton.
- Initial type definitions for the symbol and output-related state touched by these functions.
- Documented signature decisions for the three Rust `print_symbol` functions.

## Phase 2: Port `output.c` Symbol Logic First

### Goals
- Migrate the generic or central symbol-printing behavior in `src/output.c`.
- Establish the base data mappings and output conventions used by the other two ports.

### Tasks
- Translate `output.c` `print_symbol` into `output::print_symbol`.
- Preserve branch order, formatting rules, and field access sequence from C.
- Replace:
  - raw pointers with references or `Option`,
  - mutable global-style state access with explicit borrowed state where already present in the C call path.
- Add unit tests covering:
  - normal symbol output,
  - null/optional input cases that were handled in C,
  - formatting edge cases visible from conditionals in the function.

### Deliverables
- Working Rust port of `src/output.c` symbol printing.
- Tests validating output equivalence for representative cases.
- Stable shared types reused by GNU/POSIX variants if applicable.

## Phase 3: Port `gnu.c` and `posix.c` Variants

### Goals
- Migrate the format-specific symbol printers while preserving file-local behavior differences.
- Reuse Phase 2 data types only where the C code already shares concepts.

### Tasks
- Translate `gnu.c` `print_symbol` into `gnu::print_symbol`.
- Translate `posix.c` `print_symbol` into `posix::print_symbol`.
- Keep GNU- and POSIX-specific conditional logic separate; do not collapse into one strategy layer.
- Align each function signature with the actual caller expectations in the Rust crate.
- Add focused tests for:
  - GNU-specific formatting/output details,
  - POSIX-specific formatting/output details,
  - behavioral differences between the two paths for the same logical symbol input.

### Deliverables
- Working Rust ports of `src/gnu.c` and `src/posix.c`.
- Variant-specific tests proving preserved divergence where present in C.
- Finalized cross-module imports with no unused abstraction layer.

## Phase 4: Integration Cleanup and Equivalence Verification

### Goals
- Replace remaining uses of the C implementations in this module’s path.
- Tighten safety and simplify any temporary migration scaffolding.

### Tasks
- Verify all three Rust `print_symbol` paths are used from the corresponding Rust-side call sites.
- Remove temporary compatibility code introduced during translation.
- Review all struct fields and function arguments; delete any that were added speculatively and are not required by the migrated logic.
- Normalize error propagation and return types across the three modules only as far as existing integration requires.
- Run `cargo test` and fix any output mismatches found during integration.

### Deliverables
- Completed Rust implementation for the module cluster.
- Reduced final type surface containing only data actually used by the migrated functions.
- Passing test suite for symbol output paths.