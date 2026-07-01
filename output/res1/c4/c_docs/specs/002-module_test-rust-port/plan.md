# Implementation Plan: module_test

## Summary

Port the C test module files `test/c4.c` and `test/hello.c` into a Rust module layout that preserves the current execution flow and function boundaries as closely as practical. The implementation should focus on migrating the existing functions `next`, `expr`, `stmt`, and the two file-local `main` entrypoints into Rust equivalents without adding new behavior or reorganizing the logic beyond what Rust ownership and error handling require.

The technical approach is a direct translation of procedural parsing/execution logic into Rust functions operating on explicit state rather than implicit global mutable state where necessary. C string/buffer handling should be replaced with safe Rust slices and owned strings only where the source files require textual input. Any C integer-based token/state handling should remain simple and close to the original representation unless Rust enums provide a clearly safer one-to-one replacement. Memory safety will come from Rust ownership and borrowing, and fallible operations should return `Result` where the C code relied on process termination or unchecked assumptions.

## Technical Context

### Language / Version
- Rust stable, edition 2021
- Minimum practical toolchain target: Rust 1.75+

### Primary Dependencies
- Rust standard library only

No third-party crates are recommended from the available evidence. The source scope is limited to porting existing C test files and their functions.

### Testing
- `cargo test`

Tests should cover:
- direct behavior of migrated `next`
- direct behavior of migrated `expr`
- direct behavior of migrated `stmt`
- smoke tests for the translated `main` behaviors from `test/c4.c` and `test/hello.c` where practical

### Performance Goals
- Preserve the original module’s single-threaded procedural performance characteristics
- Avoid unnecessary heap allocation during token scanning/parsing
- Keep parsing/execution logic linear with respect to input size, matching the C implementation’s intent
- Prefer borrowed views into input over repeated string copying

## Module Mapping

### C to Rust File Mapping
- `test/c4.c` → `src/module_test/c4.rs`
- `test/hello.c` → `src/module_test/hello.rs`

### Rust Module Layout
- `src/module_test/mod.rs`
  - re-exports or exposes the migrated submodules only as needed for tests
- `src/module_test/c4.rs`
  - migrated implementations of:
    - `next`
    - `expr`
    - `stmt`
    - file-specific `main` logic from `test/c4.c`, renamed to avoid collision
- `src/module_test/hello.rs`
  - migrated file-specific `main` logic from `test/hello.c`, renamed to avoid collision

### Function Mapping
Because Rust can only have one crate entrypoint `main`, the C file-local `main` functions should be mapped as ordinary functions:
- `test/c4.c::main` → `pub(crate) fn run_c4(...) -> Result<i32, ModuleError>` or `fn run_c4(...) -> i32`
- `test/hello.c::main` → `pub(crate) fn run_hello(...) -> Result<i32, ModuleError>` or `fn run_hello(...) -> i32`

The exact signature should follow the original dependency on arguments:
- if no real argument parsing exists, use a no-argument function
- if argument handling exists in C, translate to `&[String]` or iterator-based input at the outer boundary only

## Data Model

No explicit C structs were identified in the analysis input. The port should therefore introduce only the minimum Rust state structures required to replace C global variables or shared parser state.

### Data-Structure Mapping
- C implicit global parser/interpreter state → Rust `struct ParserState` or similarly named internal state holder
- C token integers / symbolic constants → Rust `type Token = i32` initially, or `enum Token` only if the source constants are clearly bounded and directly translatable
- C raw character pointer traversal → Rust indices over `&str` / `&[u8]`
- C mutable global source/program cursor → Rust fields on `ParserState`
- C return-code style error signaling → Rust `Result<T, ModuleError>` at function boundaries where failure is possible

### Recommended Minimal Rust Structures
```rust
pub(crate) struct ParserState<'a> {
    input: &'a [u8],
    pos: usize,
    // token/current-value fields added only if present in C logic
}
```

```rust
pub(crate) enum ModuleError {
    ParseError,
    InvalidState,
    Io(std::io::Error),
}
```

Notes:
- Keep the error enum minimal and driven only by actual migrated failure cases.
- If the original C code relies on sentinel values rather than recoverable errors, internal helpers may still use simple return values, with conversion to `Result` only at externally visible boundaries.
- Use `&[u8]` instead of `String` internally when the C code is pointer/arithmetic heavy and byte-oriented.

## Implementation Phases

## Phase 1: Establish Rust module skeleton and entrypoint mapping

### Goals
- Create the Rust module structure mirroring the two C files
- Define the minimum shared state and error types
- Resolve naming collisions for the two C `main` functions

### Tasks
- Add `src/module_test/mod.rs`
- Add `src/module_test/c4.rs`
- Add `src/module_test/hello.rs`
- Introduce `run_c4` and `run_hello` as Rust replacements for the two C `main` functions
- Add minimal internal types for parser/execution state if `test/c4.c` uses shared mutable globals
- Keep all visibility restricted to `pub(crate)` unless tests need broader access

### Output
- Compiling skeleton with stubbed function signatures for:
  - `next`
  - `expr`
  - `stmt`
  - `run_c4`
  - `run_hello`

## Phase 2: Port core parsing/execution functions from `test/c4.c`

### Goals
- Translate `next`, `expr`, and `stmt` with behavior preserved as closely as possible
- Replace C pointer mutation and unchecked memory access with explicit indexed state updates

### Tasks
- Port `next` first, since token advancement likely drives the other functions
- Port `expr` against the same state representation
- Port `stmt` after `expr`, preserving call ordering and control flow
- Move C globals into a single mutable Rust state object passed by `&mut`
- Convert raw C character/buffer traversal into:
  - `&[u8]` plus `pos`, or
  - `Chars` plus explicit lookahead only if the original logic does not depend on byte indexing
- Preserve integer widths intentionally:
  - use `i32`/`isize` where the C logic assumes arithmetic semantics
  - use `usize` only for indexing
- Replace implicit fatal paths with `Result` only where the port would otherwise panic or read invalid memory

### Output
- Working Rust equivalents of `next`, `expr`, and `stmt`
- Unit tests focused on stable observable behavior from migrated functions

## Phase 3: Port file-specific main logic from `test/c4.c` and `test/hello.c`

### Goals
- Complete the procedural flow of each original test file
- Keep the two translated entry functions independent

### Tasks
- Translate `test/c4.c::main` into `run_c4`
- Translate `test/hello.c::main` into `run_hello`
- Preserve original return-code behavior using `i32` results, optionally wrapped in `Result`
- If the original code reads command-line arguments or fixed input, keep that handling at the outer function boundary
- Avoid introducing a combined dispatcher unless the existing project structure explicitly requires one

### Output
- Executable logic for both translated file entrypoints
- Smoke tests validating the translated `main`-equivalent flows

## Phase 4: Tighten safety, error handling, and migration verification

### Goals
- Ensure the direct port is memory-safe and behaviorally consistent
- Finalize tests and remove temporary translation scaffolding

### Tasks
- Review all translated index and slice operations for bounds safety
- Remove any placeholder panics used during the initial port
- Narrow error types to only actual failure modes present in the migrated code
- Confirm no unnecessary heap allocations were introduced in hot parsing paths
- Add regression-style tests for the most C-like edge cases encountered during translation:
  - end-of-input handling in `next`
  - malformed expression handling in `expr`
  - control-flow termination in `stmt`
- Keep the final module layout limited to the mapped C files and their minimal shared definitions

### Output
- Finalized Rust port for `module_test`
- Passing `cargo test`