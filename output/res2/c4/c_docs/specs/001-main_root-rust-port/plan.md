# Implementation Plan: main_root

## Summary

Port the `main_root` C module from `c4.c` and `hello.c` into a small Rust binary crate on branch `001-main_root-rust-port`. The implementation should preserve the existing module shape and function responsibilities rather than redesigning the program.

The Rust port should focus on migrating the existing top-level execution flow and parser/evaluator-related functions (`next`, `expr`, `stmt`, `main`) into Rust functions with equivalent control flow. Because the input analysis does not provide explicit C structs, the plan should keep state in Rust using module-local structs and enums only where needed to replace C globals, raw pointers, and integer-tagged state. The preferred technical approach is:

- keep a single binary entry point in `src/main.rs`,
- migrate C global state into an explicit Rust state holder passed by mutable reference,
- replace pointer arithmetic and unchecked indexing with slices, iterators, and index-based access,
- use `Result` for fallible startup and parsing/runtime error paths,
- use standard library facilities only unless later source inspection proves a crate is necessary.

## Technical Context

- **Language/Version**: Rust 1.78+ edition 2021
- **Primary Dependencies**:
  - Rust standard library only
- **Testing**:
  - `cargo test`
  - unit tests for token advancement and expression/statement handling where source behavior can be isolated
  - smoke tests for top-level program execution paths if `main` behavior can be made testable through extracted helper functions
- **Performance Goals**:
  - maintain broadly comparable single-threaded execution characteristics to the C implementation
  - avoid unnecessary heap allocation during token scanning/parsing
  - keep parsing/execution state in compact in-memory structures
  - prioritize semantic parity and safety over micro-optimizations

## Module Mapping

### Source File Mapping

- `c4.c` -> `src/main.rs`
- `hello.c` -> `src/main.rs` or a minimal internal helper section within `src/main.rs`

Given the limited module scope and the requirement not to expand extra capabilities, keep the Rust port in a single binary source file unless source inspection shows `hello.c` contains a clearly separate helper path that can be moved into a small internal submodule in the same file hierarchy.

### Function Mapping

- `next` -> `fn next(state: &mut MainRootState) -> Result<(), MainRootError>` or `fn next(state: &mut MainRootState)`
- `expr` -> `fn expr(state: &mut MainRootState, precedence: i32) -> Result<ExprValue, MainRootError>` if the C code returns a value-like result; otherwise mirror the original return shape as closely as possible
- `stmt` -> `fn stmt(state: &mut MainRootState) -> Result<(), MainRootError>`
- `main` from `c4.c` -> Rust binary entry `fn main()` with an internal `fn run() -> Result<(), MainRootError>`
- `main` from `hello.c` -> migrate into a named helper function if both C files are to coexist in one crate, since Rust can only expose one binary `main` per target

### Control-Flow Mapping Notes

- C global mutable variables should be consolidated into one `MainRootState` struct.
- C integer token kinds or opcode constants should become Rust enums or named constants, depending on how heavily they participate in arithmetic/comparison logic.
- C functions that terminate via `exit`, `printf` + `return`, or implicit undefined behavior should be normalized into explicit `Result` returns where feasible.

## Data Model

The analysis input does not list explicit C data structures, so the Rust data model should be derived from variable groups found during source migration rather than invented upfront.

### Planned Mapping Strategy

- **C globals used together for scanning/parsing/runtime state**
  - -> `struct MainRootState`
  - Holds source buffer, current position/index, current token, numeric value buffers, symbol-related arrays/maps only if present in the original code.
- **C token kind integer constants**
  - -> `enum TokenKind` if the values represent a closed set of lexical categories
  - -> `type TokenKind = i32` with associated `const` items only if exact numeric compatibility is required by the existing expression logic
- **C AST-less expression evaluation state**
  - -> direct Rust scalar fields and function return values, avoiding new AST layers unless the C source already materializes one
- **C char* / int* cursor variables**
  - -> `usize` indices into `Vec<u8>` / `String` / slices
- **C null-terminated strings**
  - -> `String` or `Vec<u8>` depending on whether byte-level parsing is required
- **C ad hoc error signaling**
  - -> `enum MainRootError`

### Initial Rust Structures

These are migration-oriented placeholders, to be finalized after inspecting the actual globals in `c4.c` and `hello.c`:

```rust
struct MainRootState {
    source: Vec<u8>,
    pos: usize,
    current_token: TokenKind,
    // additional migrated parser/runtime fields from C globals
}

enum TokenKind {
    // filled from C token constants if appropriate
}

enum MainRootError {
    Io(std::io::Error),
    Parse(String),
    Runtime(String),
}
```

### Memory Management Notes

- Replace all raw ownership assumptions from C with owned Rust containers.
- Avoid self-referential borrowing designs; store indices instead of references into mutable buffers.
- If the original code mutates a source/program buffer in place, represent it as `Vec<u8>` and constrain mutation through indexed access.
- Keep lifetimes simple by passing `&mut MainRootState` through `next`, `expr`, and `stmt`.

### Error Handling Notes

- The public entry path should be `main -> run() -> Result`.
- Parsing and statement/expression failures should return `MainRootError` instead of panicking.
- Use `std::process::exit` only at the outermost boundary if exact process exit behavior must be preserved after error reporting.

## Implementation Phases

## Phase 1: Establish crate and migrate entry flow

- Create a Rust binary crate layout for branch `001-main_root-rust-port`.
- Add `src/main.rs` and set up `fn main()` delegating to `run()`.
- Inspect `c4.c` and `hello.c` to determine which `main` is the effective entry point and which one must become an internal helper or separate logic path.
- Port command-line/input-loading logic from the active C `main` into Rust using `std::env` and `std::fs`.
- Define the initial `MainRootState` and `MainRootError` with only fields needed to compile the migrated entry path.
- Preserve existing output/error text structure where practical, without adding CLI features.

## Phase 2: Migrate tokenizer and parser state

- Port `next` first, because `expr` and `stmt` depend on token advancement.
- Convert C scanning logic from pointer-based traversal to byte-slice indexing.
- Map token-related globals/constants into Rust constants or `TokenKind`.
- Add tests for representative token advancement cases derived from the C source behavior.
- Expand `MainRootState` only with fields directly required by `next`.

## Phase 3: Migrate expression handling

- Port `expr` with the same precedence and evaluation/parsing strategy used in C.
- Preserve the original control flow and operator handling rather than introducing a new parser architecture.
- Replace unsafe intermediate state mutation with explicit mutable access through `MainRootState`.
- Use `Result` returns for malformed input and impossible states that were unchecked in C.
- Add focused tests for expression cases that reflect the original operator precedence and token consumption rules.

## Phase 4: Migrate statement handling and integrate execution

- Port `stmt` using the already migrated `next` and `expr`.
- Wire statement execution/parsing into `run()` so the Rust binary follows the C module’s original top-level flow.
- Reconcile any remaining globals into `MainRootState`.
- Migrate any necessary helper logic from `hello.c` only to the extent required for parity with the original build/runtime behavior.
- Add integration-style tests around statement sequences or top-level execution paths where feasible through extracted helpers.

## Phase 5: Stabilize parity and remove C-specific assumptions

- Review all remaining C idioms such as sentinel values, unchecked array access, and implicit conversions, and convert them to explicit Rust equivalents.
- Minimize panics by ensuring all expected failure paths return `MainRootError`.
- Confirm that duplicated C function names from separate files are represented cleanly in Rust without introducing extra modules.
- Run `cargo test` and fix semantic mismatches found during source comparison.
- Keep the final structure limited to the migrated module scope, without adding nonessential abstractions.