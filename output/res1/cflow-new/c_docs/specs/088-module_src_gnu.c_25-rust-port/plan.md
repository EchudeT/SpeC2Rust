# Implementation Plan: module_src_gnu.c_25

## Summary

Port `src/gnu.c` into an idiomatic Rust module while preserving the existing module boundary and behavior centered on `gnu_output_handler`. The Rust implementation should translate the current C control flow and output logic directly, avoiding feature expansion and keeping the scope limited to the existing file and function set.

The implementation approach is a narrow migration:

- create a single Rust module corresponding to `src/gnu.c`
- convert the handler logic into a Rust function with explicit input/output types
- replace C-style implicit ownership and nullable state with Rust references, options, and scoped allocation
- model the unnamed C data shape only as far as required by `gnu_output_handler`
- keep output generation and error propagation explicit through standard library I/O traits and `Result`

The plan should prefer direct structural correspondence over redesign so behavior remains easy to compare against the C source during validation.

## Technical Context

### Language/Version

- Rust stable, edition 2021
- Minimum practical compiler target: Rust 1.76+

### Primary Dependencies

Use the Rust standard library by default.

Recommended standard components:

- `std::io` for writer-based output and error propagation
- `std::fmt` for formatting support where needed
- `std::borrow` only if borrowed/owned text handling becomes necessary during translation

No third-party crates are recommended from the provided evidence.

### Testing

- `cargo test`

Testing focus:

- unit tests for `gnu_output_handler`
- output-equivalence checks for representative inputs
- edge-case tests for null/empty/optional state translated from C semantics
- error-path tests for writer failures if the function writes through `std::io::Write`

### Performance Goals

- Preserve the asymptotic behavior of the C implementation
- Avoid unnecessary heap allocation beyond what is required to represent C string/data handling safely in Rust
- Keep output generation single-pass where the C implementation is single-pass
- Minimize cloning; prefer borrowed data and direct writes to an output sink

## Module Mapping

### C to Rust File Mapping

- `src/gnu.c` -> `src/gnu.rs`

If the project already centralizes modules through `src/lib.rs` or `src/main.rs`, expose only the direct equivalent Rust module entry required for the migrated function. Do not introduce additional helper modules unless they are strictly needed to complete the translation of `gnu_output_handler`.

### Function Mapping

- `gnu_output_handler` -> `pub(crate)` or private Rust function `gnu_output_handler(...) -> Result<..., ...>` depending on existing crate visibility needs

Mapping guidance:

- Convert output-side effects to a function that accepts an explicit mutable writer when the C code writes to a stream or file-like target
- Convert integer status returns into `Result` if the C function signals failure through return codes
- Preserve call ordering and formatting behavior from the C implementation

## Data Model

### Data-structure Mapping

The analysis identifies one `anonymous` C data structure. Because the original type is unnamed, the Rust plan should introduce a narrowly scoped named type only if required by the translated function signature or internal state.

- `anonymous` -> Rust `struct` with a module-private name reflecting its role in `gnu_output_handler`

Guidelines for the mapping:

- Represent C pointers to required state as:
  - `&T` for non-null borrowed input
  - `&mut T` for mutable non-null state
  - `Option<&T>` / `Option<&mut T>` for nullable borrowed state
- Represent C strings as:
  - `&str` when valid UTF-8 is guaranteed by surrounding project assumptions
  - otherwise `&[u8]` or `&CStr` at the boundary, converting only where necessary
- Represent C flag/integer mode fields with:
  - `bool` for binary flags
  - integer primitives (`i32`, `u32`, `usize`) only where numeric semantics matter
- Represent C tagged behavior with a Rust `enum` only if the C logic clearly branches on discrete variants already present in the source; do not invent new abstraction layers

### Memory Management

- Remove manual lifetime management, malloc/free pairs, and temporary buffer ownership by using stack values, `String`, or `Vec<u8>` only where the C code requires mutable dynamic storage
- Replace nullable ownership patterns with `Option`
- Keep borrowed data lifetimes local to the handler where possible to reduce structural changes

### Error Handling

- Replace C status-code checks with `Result`
- Use `std::io::Result` if the main failure mode is output writing
- If the function has non-I/O validation failures, use a small module-local error enum only if needed; otherwise keep errors mapped to existing caller expectations without broadening scope

## Implementation Phases

## Phase 1: Establish Module Skeleton and Signature Mapping

Goals:

- Create the Rust file corresponding to `src/gnu.c`
- Identify the exact Rust signature for `gnu_output_handler`
- Define the minimal Rust representation for the anonymous C data used by the function

Tasks:

- Add `src/gnu.rs`
- Wire the module into the crate using the existing project entry structure
- Translate the C function signature into Rust:
  - map pointer parameters to references or `Option`
  - map stream/output parameters to `&mut impl std::io::Write` or a concrete writer type if dictated by surrounding code
  - map return codes to `Result` or the narrowest compatible Rust type
- Introduce a module-private struct for the anonymous data only if the function cannot be expressed without it

Exit criteria:

- The crate compiles with a stub or partial body for `gnu_output_handler`
- All required types are defined without adding unrelated modules or infrastructure

## Phase 2: Port Core Handler Logic

Goals:

- Translate the body of `gnu_output_handler` faithfully from C to Rust
- Preserve output formatting and control flow
- Eliminate unsafe memory handling unless a boundary requirement makes a small unsafe block unavoidable

Tasks:

- Port conditionals, loops, and formatting operations in the original order
- Replace C buffer operations with safe Rust string/byte handling
- Write directly to the output sink rather than assembling unnecessary intermediate buffers
- Translate null checks into `Option` handling
- Translate any sentinel-based logic into explicit Rust branching
- Keep helper logic local within `src/gnu.rs`; avoid extracting utility modules unless required for compilation clarity

Exit criteria:

- `gnu_output_handler` is fully implemented in Rust
- No manual allocation/free behavior remains
- Error returns are explicit and propagated consistently

## Phase 3: Validate Behavioral Equivalence

Goals:

- Confirm the Rust handler matches the C module’s observable behavior for expected inputs
- Lock down formatting and edge-case semantics

Tasks:

- Add unit tests covering:
  - normal output generation paths
  - empty or absent optional inputs corresponding to nullable C pointers
  - boundary cases in formatting/ordering
  - failure propagation from the writer
- Use fixed expected output strings or byte buffers to verify exact output where practical
- Review integer and string conversions for parity with the C behavior

Exit criteria:

- `cargo test` passes
- Output and status behavior for tested cases are stable and aligned with the original implementation intent

## Phase 4: Cleanup and Integration Review

Goals:

- Finalize the migration with minimal surface area
- Ensure the Rust module integrates cleanly into the branch without extra architectural changes

Tasks:

- Remove any temporary translation scaffolding left from the port
- Narrow visibility on types and functions to the smallest required scope
- Confirm no unused abstractions or extra dependencies were introduced
- Run formatting and standard lint checks already used by the project

Exit criteria:

- The Rust replacement for `src/gnu.c` is complete, minimal, and compile-clean
- The branch contains only the migration artifacts needed for this module