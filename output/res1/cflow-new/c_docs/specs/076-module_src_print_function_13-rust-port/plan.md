# Implementation Plan: module_src_print_function_13

## Summary

This module port covers the C printing path centered on `print_function_name` and `print_function`, currently implemented across `src/gnu.c` and `src/output.c`. The Rust implementation should preserve the existing formatting and emission behavior while translating pointer-oriented C logic into explicit Rust borrowing and owned string handling.

The implementation approach is a direct migration of the existing routines into a narrowly scoped Rust module that:
- keeps the same call boundaries at the function level,
- maps C anonymous record layouts into named Rust structs only as needed by these two functions,
- uses standard-library formatting and output abstractions in place of C stream and buffer manipulation,
- makes nullability and optional fields explicit with `Option`,
- converts C status/error conventions into `Result` only where the surrounding Rust code requires propagation.

The plan intentionally limits scope to the code paths and data touched by `print_function_name` and `print_function`, avoiding unrelated refactors or new abstractions beyond what is necessary to preserve behavior safely.

## Technical Context

### Language / Version
- Rust 1.78 or newer
- Edition: 2021

### Primary Dependencies
- Rust standard library only
- No third-party crates are recommended based on the provided module evidence

### Testing
- `cargo test`

### Performance Goals
- Maintain behaviorally equivalent output with no avoidable extra passes over input data
- Prefer borrowed string slices and references over cloning where source lifetime allows
- Keep formatting operations linear in the size of the emitted function representation
- Avoid heap allocation beyond what is required for Rust string assembly or buffered output already implied by the existing logic

## Module Mapping

### C Source to Rust Module Mapping
- `src/gnu.c`
  - migrate the portions used by `print_function_name`
  - move formatting/name-selection logic into the Rust module responsible for GNU-style or function-name rendering, only if these routines are still semantically tied to GNU output behavior
- `src/output.c`
  - migrate the portions used by `print_function`
  - keep stream/output-facing behavior in the Rust output module that owns formatted emission

### Proposed Rust File Mapping
- `src/output.rs`
  - `print_function_name`
  - `print_function`
  - helper functions strictly required by these two functions

If the current Rust branch already separates GNU-specific and generic output concerns, preserve that split only to the extent required by the original call graph:
- `src/gnu.rs`
  - GNU-specific helper logic directly invoked by `print_function_name` or `print_function`
  - top-level rendering entry points

### Migration Boundaries
- Port only the logic exercised by:
  - `print_function_name`
  - `print_function`
- Migrate any directly referenced static helpers, constants, and local formatting rules that are necessary for these functions to compile and match behavior
- Do not broaden the module to cover unrelated output routines from `src/gnu.c` or `src/output.c`

## Data Model

The analysis lists only anonymous C data structures. For this port, introduce named Rust types only for records that are directly read or mutated by `print_function_name` and `print_function`.

### Mapping Principles
- C anonymous struct used as immutable function metadata -> Rust named `struct` with borrowed or owned fields as appropriate
- C anonymous struct used as mutable print/output context -> Rust named `struct` holding explicit mutable state
- C nullable pointers -> `Option<&T>`, `Option<&mut T>`, or `Option<T>`
- C string pointers (`char *`, `const char *`) -> `&str`, `String`, or `Option<String>` depending on ownership and nullability
- C flags/integers used as booleans -> `bool`
- C enums encoded as ints -> Rust `enum` if a closed set is evident from the used code paths; otherwise preserve as integer type locally until the full type surface is known

### Expected Rust Type Set
Because the source records are anonymous in the analysis output, define placeholder migration types based on role, then rename to match existing project conventions:

- anonymous record for function identity/name data
  - `FunctionNameData`
- anonymous record for function node / symbol descriptor
  - `FunctionRecord`
- anonymous record for output stream or emitter state
  - `OutputContext`
- anonymous record for formatting options / mode flags
  - `PrintOptions`
- anonymous record for source location or file/line attachment, if read by these functions
  - `SourceLocation`
- anonymous record for parameter or call graph related linked data, if traversed by these functions
  - `ParameterInfo` or `CallRelation`

### Linked and Optional Data
If the C implementation walks linked lists or sibling chains:
- represent traversal as references over existing container ownership where available
- if exact ownership is not yet established in the Rust branch, initially model links as indices or references managed by the parent owning collection rather than reproducing raw self-referential pointer structures

### Memory Management Decisions
- Replace raw pointer reads with shared or mutable borrows
- Make absent substructures explicit with `Option`
- Avoid interior mutability unless the original semantics require shared mutation across aliases; prefer passing `&mut OutputContext`
- Keep temporary formatted content in local `String` buffers only where direct `write!`-style streaming is not practical

### Error Handling Decisions
- If the C code writes directly to `FILE *` and reports failure implicitly, Rust functions should return:
  - `std::fmt::Result` for formatter-based output, or
  - `std::io::Result<()>` for writer-based output
- If the existing Rust code around this module already uses infallible string building, keep helper functions infallible and isolate fallibility at the outer write boundary
- Do not introduce custom error hierarchies unless already present in the branch

## Implementation Phases

## Phase 1: Inspect and Define Minimal Rust Data Shapes
- Identify the exact fields from `src/gnu.c` and `src/output.c` that `print_function_name` and `print_function` touch
- Extract the anonymous C records used by these paths and assign stable Rust names
- Define minimal Rust structs/enums for those records in the destination module or the nearest existing shared type file
- Convert null-sensitive fields and pointer relationships into `Option` and references
- Document unresolved ownership points inline with `TODO` comments only where needed for compilation sequencing

### Deliverables
- Rust type definitions for all data directly accessed by the two target functions
- Field mapping notes from C layout to Rust representation
- Compile-ready signatures for `print_function_name` and `print_function`

## Phase 2: Port `print_function_name`
- Translate the name-rendering logic from C into Rust with behavior-first fidelity
- Preserve ordering of conditional formatting decisions and fallback naming rules
- Replace manual C string concatenation and null checks with Rust match/if-let logic
- Keep helper extraction limited to repeated code required for readability or borrow-checker compliance
- Add unit tests for representative naming cases derived from the C branches:
  - normal named function
  - missing or optional name fields
  - mode/flag-dependent name formatting
  - edge cases around empty strings or absent metadata if present in the C logic

### Deliverables
- Working Rust implementation of `print_function_name`
- Focused unit tests covering branch behavior and output text

## Phase 3: Port `print_function`
- Translate the full function-printing routine into Rust using the Phase 1 data model and Phase 2 name printer
- Preserve exact emission order, separators, indentation, and conditional sections from the C implementation
- Convert stream writes to `write!`/`writeln!` or equivalent standard-library output operations
- Thread mutable output state through explicit `&mut` parameters rather than shared global mutation where the surrounding Rust branch allows
- Keep any required local helper functions in the same file unless already shared by existing Rust code

### Deliverables
- Working Rust implementation of `print_function`
- Integration of `print_function_name` into the higher-level rendering path
- Unit tests comparing complete formatted output for common and edge-case function records

## Phase 4: Stabilize Integration and Remove C-Specific Assumptions
- Reconcile any remaining C idioms still present after direct translation, such as sentinel values or implicit defaults
- Align signatures and module placement with the rest of the Rust branch without widening scope
- Run `cargo test` and fix borrow/lifetime issues by simplifying ownership or local buffering rather than redesigning the module
- Confirm that all migrated logic is isolated from raw-pointer assumptions and that output failures are propagated consistently

### Deliverables
- Cleanly integrated Rust module on branch `076-module_src_print_function_13-rust-port`
- Passing tests for the migrated code path
- Removal or retirement of the corresponding C-backed path for these functions if the branch migration strategy requires one-for-one replacement