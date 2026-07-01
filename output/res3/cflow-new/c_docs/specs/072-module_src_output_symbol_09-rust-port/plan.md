# Implementation Plan: module_src_output_symbol_09

## Summary

This module cluster centers on migrating the `print_symbol` logic currently implemented across `src/gnu.c`, `src/output.c`, and `src/posix.c` into Rust on branch `072-module_src_output_symbol_09-rust-port`. The Rust implementation should preserve the existing output behavior and variant-specific formatting paths while reducing reliance on raw mutable state and implicit C I/O conventions.

The implementation approach is to port the existing symbol-printing code paths into a small Rust module set that mirrors the current source split rather than redesigning the feature. Shared formatting and dispatch logic should be centralized only where the existing C files already overlap through behavior, with GNU- and POSIX-specific differences kept explicit. Output should be expressed through standard-library writers and string formatting, and C-style nullable pointers, sentinel values, and unchecked buffer access should be replaced with `Option`, `Result`, slices, and owned/borrowed string types as appropriate.

## Technical Context

### Language / Version
- Rust 1.78 or newer
- Edition: 2021

### Primary Dependencies
- Rust standard library only
- No third-party crates are recommended based on the available module evidence

### Testing
- `cargo test`

### Performance Goals
- Preserve the current asymptotic behavior of symbol output paths
- Avoid unnecessary heap allocation during per-symbol formatting where borrowing or direct writer output is sufficient
- Keep formatting dispatch lightweight and comparable to the existing C implementation
- Ensure no repeated string copying is introduced beyond what is required by Rust ownership and safe output APIs

## Module Mapping

### C to Rust File Mapping
- `src/output.c` -> `src/output.rs`
- `src/gnu.c` -> `src/gnu.rs`
- `src/posix.c` -> `src/posix.rs`

### Rust Module Roles
- `src/output.rs`
  - Holds the base `print_symbol` port if this file contains the shared or default implementation
  - Defines any shared helper functions required by both GNU and POSIX output paths, but only when directly needed by migrated code
  - Exposes the common writer-facing entry points used by the rest of the crate

- `src/gnu.rs`
  - Holds the GNU-specific `print_symbol` behavior
  - Keeps GNU formatting branches and symbol rendering rules isolated from POSIX behavior
  - Uses shared output helpers from `output.rs` only where the C code already shares semantics

- `src/posix.rs`
  - Holds the POSIX-specific `print_symbol` behavior
  - Preserves POSIX formatting and emission order separately from GNU behavior
  - Uses shared output helpers from `output.rs` only for truly common mechanics

### Function Mapping
Because all listed functions are named `print_symbol`, the Rust port should retain separate functions aligned to their current file ownership rather than collapsing them into a single generalized interface prematurely.

Suggested mapping:
- `src/output.c::print_symbol` -> `output::print_symbol(...)`
- `src/gnu.c::print_symbol` -> `gnu::print_symbol(...)`
- `src/posix.c::print_symbol` -> `posix::print_symbol(...)`

If the surrounding crate already uses an output-mode selector, the Rust side should keep that selection at the existing call boundary and dispatch into the corresponding module function.

## Data Model

The source analysis exposes only anonymous C data structures, so the Rust data model should be finalized during code migration by naming structures after their actual usage sites rather than introducing speculative abstractions.

### Mapping Rules
- Anonymous C structs used only within one file
  - Map to private Rust `struct`s in the corresponding module
- Anonymous C structs representing variant state or mode-dependent records
  - Map to Rust `enum`s if the C code uses tag-like branching
  - Otherwise map to plain `struct`s with explicit fields
- C pointers that may be null
  - Map to `Option<&T>`, `Option<&mut T>`, or `Option<NonNull<T>>` only if raw identity is required internally
- C strings (`char *`, `const char *`)
  - Map to `&str` when guaranteed valid text in the Rust-owned path
  - Otherwise map to `&CStr` or byte slices during transitional parsing, converting at formatting boundaries only when required
- Output buffers / FILE-based emission
  - Map to `&mut dyn std::io::Write` or to `fmt::Write` if the path is string-oriented
  - Choose one writer style based on the existing call patterns and keep it consistent across the three migrated functions
- Integer flags / mode fields
  - Map to explicit Rust enums or small newtype wrappers if the C code uses a closed set of values
  - Keep as primitive integers only when the values are pass-through and not interpreted locally

### Initial Rust Data Types to Introduce
These are migration placeholders and should be adjusted to the actual C field sets during porting:
- `SymbolRecord`
  - Rust representation of the symbol data consumed by `print_symbol`
- `OutputContext`
  - Rust representation of mutable output configuration or accumulated formatting state
- `OutputMode`
  - Enum distinguishing GNU vs POSIX behavior if the existing code already selects between them
- `PrintOptions`
  - Struct for boolean or integer formatting flags currently passed indirectly or through shared globals

### Memory Management Notes
- Replace shared mutable global access with borrowed context parameters where the current call chain permits it
- Prefer borrowing symbol data for formatting rather than cloning
- Use owned `String` only when formatting requires intermediate assembly before emission
- Eliminate manual buffer sizing and pointer arithmetic in favor of slices and formatted writes

### Error Handling Notes
- Convert output failures into `io::Result<()>` if writing to streams
- Use `Result` return values for parsing or formatting branches that can fail during migration
- Avoid panics in normal formatting flow
- Represent invalid or absent C state explicitly with `Option`/`Result` instead of sentinel integers where feasible without changing external behavior

## Implementation Phases

## Phase 1: Inventory and Rust Surface Definition
- Inspect the three existing `print_symbol` implementations and identify:
  - exact parameter lists
  - shared inputs and side effects
  - output sink type
  - any anonymous structs accessed directly
  - any global flags or mode selectors involved in symbol output
- Create `src/output.rs`, `src/gnu.rs`, and `src/posix.rs` matching the current C file boundaries
- Define minimal Rust data structures needed to compile the migrated signatures:
  - symbol record type(s)
  - output context type(s)
  - mode/flag enums only where directly evidenced by C branches
- Define the Rust return convention for printing functions, preferably `io::Result<()>` or a narrow crate-local result type if the surrounding code already has one

## Phase 2: Port Shared and Default Output Logic
- Migrate `src/output.c::print_symbol` into `src/output.rs`
- Port any directly related helper logic required to make this function work, but do not widen the migration beyond dependencies required by this symbol-output path
- Replace:
  - raw buffer writes with safe writer calls
  - null checks with `Option`
  - pointer-based field access with references and slices
- Preserve formatting order, separators, spacing, and conditional emission behavior
- Add focused unit tests for the base/shared symbol output behavior using representative symbol inputs and expected rendered strings

## Phase 3: Port GNU and POSIX Variants
- Migrate `src/gnu.c::print_symbol` into `src/gnu.rs`
- Migrate `src/posix.c::print_symbol` into `src/posix.rs`
- Keep format differences explicit; do not unify variant-specific branches unless the code is byte-for-byte equivalent in behavior
- Reuse only the shared mechanics already identified in Phase 2
- Add tests covering:
  - GNU-specific formatting differences
  - POSIX-specific formatting differences
  - edge cases around missing symbol attributes, optional fields, or conditional labels

## Phase 4: Integration and Behavioral Tightening
- Wire the Rust module entry points into the existing crate call path in the same places where the C implementations were previously selected
- Remove remaining transitional assumptions in data structures once all three ports compile together
- Verify that all anonymous C structures touched by these functions have concrete Rust names and ownership rules
- Run `cargo test` and fix any discrepancies in output formatting or error propagation
- Perform a final pass to:
  - minimize unnecessary allocation
  - narrow visibility of helper types/functions
  - ensure no unsafe code remains unless demanded by surrounding interfaces not covered in this module port