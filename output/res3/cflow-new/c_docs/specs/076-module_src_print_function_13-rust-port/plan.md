# Implementation Plan: module_src_print_function_13

## Summary

This module ports the C printing logic centered on `print_function_name` and `print_function` from `src/gnu.c` and `src/output.c` into Rust, preserving existing behavior and call structure rather than redesigning output handling.

The Rust implementation should:
- translate the two function bodies and their immediate helper data access patterns into idiomatic but direct Rust,
- keep formatting behavior aligned with the C code’s current output semantics,
- replace raw pointer and nullable-state handling with explicit references, `Option`, and borrowed string/slice access,
- isolate mutable output state behind ordinary `&mut` parameters instead of global-style mutation where feasible within the existing project layout.

The technical approach is a file-by-file migration into a Rust module that mirrors the source responsibility split:
- one Rust source unit for logic originating in `gnu.c`,
- one Rust source unit for logic originating in `output.c`,
- shared type definitions for the anonymous C structures only to the extent required by these two functions.

The plan should avoid introducing new formatting subsystems or abstraction layers not required by the existing C behavior.

## Technical Context

- **Language/Version**: Rust 1.78+ edition 2021
- **Primary Dependencies**:
  - Rust standard library only
  - No third-party crates are recommended from the available evidence
- **Testing**:
  - `cargo test`
  - unit tests for formatting and name rendering behavior
  - regression-style tests comparing representative rendered strings to expected output
- **Performance Goals**:
  - maintain output-generation performance comparable to the C implementation for per-function rendering
  - avoid unnecessary string cloning; prefer `&str`, borrowed data, and `std::fmt::Write`/buffer append patterns
  - keep allocation behavior bounded to the same practical scope as current formatting operations
  - preserve linear traversal behavior over any lists or nested structures used by the two functions

## Module Mapping

### C to Rust File Mapping

| C File | C Functions | Rust Target | Notes |
|---|---|---|---|
| `src/gnu.c` | `print_function_name` | `src/gnu.rs` or equivalent module file | Keep function-focused port with direct mapping of naming/formatting rules. |
| `src/output.c` | `print_function` | `src/output.rs` or equivalent module file | Port main print orchestration and preserve call ordering. |

### Rust Module Placement

A restrained layout is recommended:

```text
src/
  gnu.rs
  output.rs
  lib.rs or main.rs
```

If the existing Rust project already has a different top-level organization, place the migrated functions into the nearest existing modules corresponding to GNU-format output and general output, without creating extra helper modules beyond what is needed to compile the port.

### Function Mapping

| C Function | Rust Function Shape | Migration Notes |
|---|---|---|
| `print_function_name` | `fn print_function_name(...) -> ...` | Preserve formatting rules exactly; convert pointer-based inputs into references or `Option<&T>`. |
| `print_function` | `fn print_function(...) -> ...` | Preserve sequencing, delegation to `print_function_name`, and output side effects via a mutable writer/buffer parameter. |

## Data Model

The analysis only identifies multiple anonymous C structures, so the Rust plan should introduce named internal structs/enums only for the fields actually touched by `print_function_name` and `print_function`.

### Mapping Principles

- Anonymous C structs used only as record carriers should become private Rust `struct`s with descriptive names derived from usage in these functions.
- Nullable pointers become `Option<T>`, `Option<&T>`, or `Option<Box<T>>` depending on ownership.
- C strings (`char *`) become:
  - `String` when owned and stored,
  - `&str` when borrowed for rendering,
  - `Option<String>` / `Option<&str>` when nullable.
- Linked relationships or nested records should be represented directly, but only to support the migrated logic.
- Integer flags should remain integer or become `bool`/small enums only when the branch meaning is clear from the C usage in these functions.

### Proposed Rust Type Strategy

Because the source analysis does not expose field names, define placeholder internal Rust types during migration and refine them from actual C field access:

| C Data Structure | Rust Mapping | Notes |
|---|---|---|
| anonymous struct used for function metadata | `struct FunctionInfo` | Expected to hold name and any printable qualifiers required by both functions. |
| anonymous struct used for output state | `struct OutputState` | Holds destination/buffer/config directly referenced by `print_function`. |
| anonymous struct used for formatting context | `struct PrintContext` | Use only if C code passes contextual flags or indentation state. |
| anonymous struct used for declaration/type information | `struct TypeInfo` | Needed only if `print_function_name` emits type-related text. |
| anonymous linked/list node(s) | `struct ...` with `Option<Box<_>>` or `Vec<_>` | Choose the simplest form matching observed traversal in the two functions. |
| anonymous flag groups | `struct ...` or `enum ...` | Prefer `bool` fields unless branch meanings justify enums. |

### Memory Management Decisions

- Eliminate manual lifetime and deallocation logic from C by using Rust ownership for stored data and references for read-only rendering.
- Avoid copying output fragments unless required by formatting APIs.
- If the C code writes to `FILE *` or a global stream, represent the sink as either:
  - `&mut String` for testable text generation, or
  - `&mut dyn std::fmt::Write` when formatting flexibility is needed without introducing I/O complexity.

### Error Handling Decisions

- If the C functions are effectively infallible string emitters, use direct return types such as `()`.
- If writing can fail due to the selected sink abstraction, return `std::fmt::Result`.
- Replace sentinel/null error signaling with `Option` or `Result` only where required by the original control flow.

## Implementation Phases

## Phase 1: Extract and Define Minimal Rust Types

- Inspect `print_function_name` and `print_function` field access to identify the exact anonymous C structures and fields they depend on.
- Introduce only the minimum named Rust structs/enums required to compile those accesses.
- Encode nullable relationships with `Option`.
- Decide the output sink signature for the ported functions based on actual C usage:
  - prefer `&mut String` if the C logic appends text,
  - use `&mut dyn std::fmt::Write` only if both functions naturally share a formatting interface.
- Add module declarations in the current Rust crate structure without creating unrelated support layers.

### Deliverables
- skeleton `gnu.rs`
- skeleton `output.rs`
- minimal shared type definitions placed in the existing crate location most directly used by these modules

## Phase 2: Port `print_function_name`

- Translate `print_function_name` from `src/gnu.c` into Rust with a close structural correspondence to the C control flow.
- Preserve branch ordering and text emission order.
- Convert:
  - null checks to `Option` matching,
  - C string handling to borrowed `&str` or owned `String`,
  - flag checks to booleans or integer comparisons depending on clarity.
- Keep helper extraction minimal; only split tiny repeated fragments if required for borrow-checker clarity.
- Add focused unit tests covering representative naming cases exposed by the control flow.

### Deliverables
- compiled Rust implementation of `print_function_name`
- tests for function-name formatting behavior

## Phase 3: Port `print_function`

- Translate `print_function` from `src/output.c`, preserving its sequencing and interaction with `print_function_name`.
- Migrate any direct output-side effects into the chosen Rust sink abstraction.
- Preserve conditional emission, separators, indentation/newline behavior, and any pre/post wrappers around the function-name output.
- Resolve ownership/borrowing so that shared state is passed explicitly rather than accessed through raw mutable globals, but do not redesign the broader architecture beyond what this function requires.
- Add regression tests for complete rendered output for representative function entries.

### Deliverables
- compiled Rust implementation of `print_function`
- tests for integrated output behavior

## Phase 4: Verification and Cleanup

- Compare generated output for selected representative inputs against expected strings derived from current C behavior.
- Remove any temporary placeholder fields or overly generic names introduced during initial extraction, renaming only where needed for maintainability.
- Confirm that all memory and nullability assumptions from the C code are represented safely in Rust.
- Keep the final module boundaries aligned with the original file responsibilities.

### Deliverables
- passing `cargo test`
- finalized module-local documentation/comments describing only migration-relevant invariants