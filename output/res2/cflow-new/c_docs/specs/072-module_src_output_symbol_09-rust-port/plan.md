# Implementation Plan: module_src_output_symbol_09

## Summary

This module cluster centers on the `print_symbol` functionality currently implemented across `src/gnu.c`, `src/output.c`, and `src/posix.c`. The Rust port should preserve the existing symbol-output behavior while consolidating implementation details into a small Rust module surface that mirrors the current source split only where it reflects behavior differences.

The technical approach is to migrate the existing C printing logic into Rust using standard-library I/O abstractions, explicit enums/structs for mode-specific behavior, and borrowed data where the C code currently passes pointers into shared state. The implementation should avoid adding new output capabilities and should focus on translating existing control flow, formatting rules, and mode-specific branching from the three C files into directly corresponding Rust functions.

Memory management should move from implicit pointer and buffer discipline in C to borrowed references and owned `String` values in Rust where formatting assembly is required. Error handling should use `std::io::Result` for writer operations and explicit return types rather than sentinel values.

## Technical Context

### Language / Version
- Rust 1.78 or newer

### Primary Dependencies
- Rust standard library only:
  - `std::io` for output
  - `std::fmt` for formatting support
  - `std::borrow` only if needed for borrowed/owned transitions

No third-party crates are recommended because the input only indicates straightforward output migration work.

### Testing
- `cargo test`

### Performance Goals
- Preserve the current asymptotic behavior of symbol printing paths.
- Avoid unnecessary heap allocation when writing directly to output streams is sufficient.
- Permit small temporary `String` assembly only where the C implementation effectively builds formatted output before emission.
- Maintain predictable per-symbol output cost comparable to the current C implementation.

## Module Mapping

### C to Rust File Mapping

| C File | Rust File | Migration Scope |
|---|---|---|
| `src/output.c` | `src/output.rs` | Core `print_symbol` implementation and shared output helpers |
| `src/gnu.c` | `src/gnu.rs` | GNU-specific symbol formatting branch migrated from C |
| `src/posix.c` | `src/posix.rs` | POSIX-specific symbol formatting branch migrated from C |

### Rust Module Placement

The Rust implementation should keep the module split restrained and aligned with the C source origins:

```text
src/
  output.rs
  gnu.rs
  posix.rs
```

If the surrounding crate already centralizes output code under another existing module, these functions should be migrated into that existing structure without introducing extra abstraction layers. The main goal is direct file/function migration, not architectural expansion.

### Function Mapping

Because all listed functions are named `print_symbol`, the Rust plan should map them by source file and output mode:

| C Function | Rust Function |
|---|---|
| `src/output.c::print_symbol` | `output::print_symbol(...)` |
| `src/gnu.c::print_symbol` | `gnu::print_symbol(...)` |
| `src/posix.c::print_symbol` | `posix::print_symbol(...)` |

Where the surrounding crate prefers a single dispatch point, `output::print_symbol(...)` may call `gnu::print_symbol(...)` or `posix::print_symbol(...)` based on the already-existing output mode state. This should only reflect current behavior and not introduce a new plugin-style interface.

## Data Model

The analysis only exposes multiple anonymous C data structures and does not provide field layouts. The Rust plan therefore should treat data-model migration conservatively:

### Data-Structure Mapping Strategy

| C Data Structure | Rust Mapping Plan |
|---|---|
| anonymous | Replace with a named `struct` if fields represent shared mutable state for symbol output |
| anonymous | Replace with a named `struct` if used as symbol metadata passed into `print_symbol` |
| anonymous | Replace with a named `enum` if it selects GNU vs POSIX vs generic output behavior |
| anonymous | Replace with a borrowed view struct if it only groups input references during formatting |
| anonymous | Replace with primitive fields in function arguments if the original struct is only a temporary grouping helper |
| anonymous | Replace with `Option<T>` where nullability exists in C |
| anonymous | Replace with slices or `&str` where C uses pointer-plus-string data |
| anonymous | Replace with `usize`/`u64`/`i64` according to the original integer semantics |
| anonymous | Replace with `bool` where C uses flags |
| anonymous | Replace with `enum` variants where C uses integer mode tags |
| anonymous | Replace with an internal helper struct for output context if multiple print paths share state |
| anonymous | Replace with an internal helper struct for formatting state only if present in existing C flow |

### Rust Type Decisions

The following rules should guide concrete type selection during implementation:

- `char *` string inputs:
  - Use `&str` when valid UTF-8 is guaranteed by existing project assumptions.
  - Use `&[u8]` or `&OsStr` only if the existing code path is byte-oriented; otherwise prefer `&str`.
- Nullable pointers:
  - Map to `Option<&T>` or `Option<T>` depending on ownership.
- Output destinations:
  - Prefer `&mut dyn std::io::Write` or a generic `W: Write` for direct translation of file-output behavior.
- Shared symbol/context state:
  - Prefer borrowed references into a broader execution context rather than cloning state.
- C flag fields:
  - Prefer explicit boolean fields or small enums over bitmask reproduction unless the bitmask is already deeply embedded in the surrounding Rust crate.

### Memory Management and Error Handling

- Eliminate manual buffer ownership concerns by using stack-local formatting and borrowed references.
- Do not replicate C-style raw pointer traversal unless required by existing neighboring Rust code.
- Convert write failures into `std::io::Result<()>`.
- Replace implicit success/failure conventions with explicit `Result` or `Option`.
- Validate all null-sensitive branches during migration so that absent symbol fields are represented safely.

## Implementation Phases

### Phase 1: Establish Rust module skeleton and shared signatures

- Create Rust files corresponding to:
  - `src/output.rs`
  - `src/gnu.rs`
  - `src/posix.rs`
- Define the initial Rust signatures for the three `print_symbol` functions based on the current call sites.
- Identify the minimal shared input/state types required by all three functions.
- Replace anonymous C state groupings with temporary named Rust structs only where needed to compile the migrated code.
- Decide the writer abstraction (`&mut dyn Write` or generic `W: Write`) based on the least invasive fit with surrounding code.

**Exit criteria**
- All target Rust files exist.
- Function signatures are fixed and compile in stub form.
- Required migrated data carriers are named and placed in their final files.

### Phase 2: Port core output logic from `src/output.c`

- Translate the common `print_symbol` flow from `src/output.c` into `src/output.rs`.
- Preserve existing branching order and formatting assembly rules.
- Convert buffer writes and direct stream emission into idiomatic Rust `Write` calls.
- Replace pointer/null checks with `Option` handling.
- Keep helper extraction minimal: only split local helpers if needed to preserve readability of the migrated function.

**Exit criteria**
- Core symbol output path is implemented in Rust.
- The Rust version preserves the same formatting decisions as the C source for shared behavior.
- Unit tests cover the common path with representative symbol inputs.

### Phase 3: Port GNU-specific and POSIX-specific symbol output paths

- Translate `src/gnu.c::print_symbol` into `src/gnu.rs::print_symbol`.
- Translate `src/posix.c::print_symbol` into `src/posix.rs::print_symbol`.
- Reuse only the shared state/types already introduced in earlier phases.
- Keep any mode-specific formatting local to each file rather than introducing a broader formatting framework.
- Add tests that compare GNU and POSIX output differences for the same logical symbol data.

**Exit criteria**
- GNU and POSIX print paths are implemented.
- Mode-specific tests confirm distinct formatting behavior where expected.
- No extra abstraction layer has been introduced beyond direct migration needs.

### Phase 4: Integration cleanup and behavior verification

- Wire call sites so the Rust output path dispatches to the correct `print_symbol` implementation matching existing mode selection.
- Remove remaining C-specific assumptions from the migrated interfaces, such as sentinel integers or mutable raw buffers.
- Add focused regression tests for:
  - missing optional symbol fields
  - edge-case names or values
  - output writer error propagation
- Verify the final Rust module boundaries still match the original C file responsibilities.

**Exit criteria**
- All migrated call paths use the Rust implementations.
- `cargo test` passes.
- Error handling and memory usage are fully expressed through safe Rust interfaces.

## Notes and Constraints

- Keep migration aligned to the existing three-file source split; do not introduce extra formatting, dispatch, or utility modules unless required by compilation.
- Prefer direct function and data migration over redesign.
- Keep all technical decisions local to symbol printing behavior already evidenced by the input.
- Do not add unrelated support systems or speculative data models.