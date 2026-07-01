# Implementation Plan: module_test

## Summary

Port the existing C test module from `test/c4.c` and `test/hello.c` into a Rust module set on branch `002-module_test-rust-port`, preserving the current control flow and function boundaries as closely as practical. The Rust implementation should focus on migrating the existing parsing/interpreter-oriented functions `next`, `expr`, `stmt`, and the two C entry-point-style `main` functions into Rust equivalents without introducing new subsystem layers or extra capabilities.

The technical approach is to:
- translate global-state-driven C logic into a small Rust module using explicit state structs where needed,
- keep parsing/evaluation routines close to the original function structure,
- use safe Rust ownership and borrowing in place of raw pointer manipulation,
- represent failures with `Result` where the C code relied on process termination, invalid state, or sentinel values,
- preserve file-level separation where it reflects the current module layout.

## Technical Context

- **Language/Version**: Rust 1.78+ stable
- **Primary Dependencies**:
  - Rust standard library only
- **Testing**:
  - `cargo test`
- **Performance Goals**:
  - Match the practical behavior of the C implementation for these test files without adding avoidable allocation or indirection.
  - Keep parser/interpreter state updates inexpensive and mostly in-place.
  - Favor slice/index-based scanning over unnecessary string copying.

## Module Mapping

### Source File Mapping

- `test/c4.c`
  - Migrate into a Rust source file such as `src/module_test/c4.rs` or `src/c4.rs`
  - Contains the Rust equivalents of:
    - `next`
    - `expr`
    - `stmt`
    - `main` corresponding to the `c4.c` test driver logic

- `test/hello.c`
  - Migrate into a Rust source file such as `src/module_test/hello.rs` or `src/hello.rs`
  - Contains the Rust equivalent of:
    - `main` corresponding to the `hello.c` sample/test logic

### Rust Module Layout

Use a restrained standard Rust layout:

```text
src/
  lib.rs
  module_test/
    mod.rs
    c4.rs
    hello.rs
```

If the surrounding project is binary-oriented rather than library-oriented, the same internal split can be retained under `src/`, but no additional abstraction layers should be added beyond what is necessary to host the migrated functions.

### Function Mapping

- `next` (C) -> `fn next(state: &mut ParserState) -> Result<(), ModuleError>`
- `expr` (C) -> `fn expr(state: &mut ParserState, level: i32) -> Result<Value, ModuleError>` or closest signature required by original behavior
- `stmt` (C) -> `fn stmt(state: &mut ParserState) -> Result<(), ModuleError>`
- `main` in `test/c4.c` -> Rust entry/test-facing function, likely `pub fn run_c4(...) -> Result<i32, ModuleError>`
- `main` in `test/hello.c` -> Rust entry/test-facing function, likely `pub fn run_hello(...) -> i32` or `Result<i32, ModuleError>` depending on original logic

Rust should not expose multiple true `main` functions in the same crate; instead, preserve both C entry points as named runner functions and cover them through tests.

## Data Model

No explicit C structs were identified in the analysis result, so the Rust data model should be introduced only to hold state currently implied by C globals, raw pointers, or shared mutable variables.

### C-to-Rust State Mapping

- **C global parser/interpreter state** -> `ParserState` Rust struct
  - Use fields for current token, source buffer position, symbol-related state, and evaluation-related state only if present in the migrated code.
  - Keep the field set minimal and derived directly from the existing C variables.

Example shape:

```rust
struct ParserState<'a> {
    source: &'a [u8],
    pos: usize,
    current_token: Token,
    // additional migrated fields only as required by c4.c
}
```

- **C integer token/type constants** -> Rust `enum` where practical, otherwise `const`/`type` aliases
  - Prefer `enum Token` for token categories if the original code uses distinguishable token kinds.
  - If the C code depends heavily on numeric precedence/token values, retain explicit discriminants or constant mappings to avoid semantic drift.

- **C raw character pointer traversal** -> Rust slice/index traversal
  - Replace pointer arithmetic with `&[u8]` plus `usize` index.
  - Avoid converting repeatedly between `String` and bytes.

- **C sentinel/error exits** -> `Result<T, ModuleError>`
  - Introduce a small module-local error enum for parse/runtime failures actually needed by the migrated functions.
  - Do not add generalized recovery machinery.

### Data-Structure Mapping Table

| C construct | Rust mapping | Notes |
|---|---|---|
| Implicit global parsing state | `ParserState` struct | Consolidates mutable state required by `next`, `expr`, and `stmt` |
| Integer token codes | `enum Token` or integer constants | Choose based on how tightly behavior depends on numeric ordering |
| `char *` / source cursor | `&[u8]` + `usize` | Safe replacement for pointer stepping |
| Return-code/error exit patterns | `Result<_, ModuleError>` | Keeps failure explicit without expanding behavior |
| C `main` functions | named Rust runner functions | Avoids multiple crate entry points |

## Implementation Phases

## Phase 1: Establish Rust module skeleton and migrate file boundaries

- Create the Rust module layout corresponding to the two C source files.
- Add `mod.rs` and file declarations with no extra helper modules beyond `c4` and `hello`.
- Introduce minimal shared types only if both migrated files require them.
- Convert each C `main` into a uniquely named Rust function rather than a crate entry point.
- Define placeholder signatures for:
  - `next`
  - `expr`
  - `stmt`
  - `run_c4`-style function for `test/c4.c`
  - `run_hello`-style function for `test/hello.c`

### Deliverables
- Compiling Rust module skeleton
- File mapping complete
- Function signatures fixed closely to migration needs

## Phase 2: Port parser/interpreter state and core functions from `test/c4.c`

- Identify all C globals and local shared variables used by `next`, `expr`, and `stmt`.
- Consolidate them into a minimal `ParserState` struct.
- Port `next` first, since token advancement typically drives the rest of the module.
- Port `expr` second, preserving precedence and evaluation order exactly as in C.
- Port `stmt` third, keeping control-flow handling aligned with the original implementation.
- Replace pointer arithmetic, unchecked indexing, and implicit integer casts with explicit Rust logic.
- Use `Result` for invalid states or parse failures where the C version would abort or print an error.

### Deliverables
- Working Rust translations of `next`, `expr`, and `stmt`
- Minimal state model with safe ownership
- Error type limited to actual migrated failure paths

## Phase 3: Port the two C entry flows and wire execution paths

- Port the `test/c4.c` `main` logic into the Rust runner function using the translated parser/interpreter functions.
- Port the `test/hello.c` `main` logic into its own Rust runner function, keeping it isolated from the `c4` path except for any strictly necessary shared definitions.
- Ensure argument handling, return codes, and output behavior remain as close as practical to the original C test behavior.
- Avoid introducing command frameworks or multi-binary setup unless the existing crate structure strictly requires a single binary entry.

### Deliverables
- Runnable Rust equivalents for both C file entry flows
- Stable public/internal function boundaries for testing

## Phase 4: Add regression tests and finalize behavioral parity

- Add `cargo test` coverage for the migrated functions at the module level.
- Create tests for:
  - token advancement behavior in `next`
  - representative expression parsing/evaluation paths in `expr`
  - representative statement handling in `stmt`
  - expected outcomes for the `c4` and `hello` runner functions
- Validate memory safety assumptions by removing any remaining C-style unchecked state handling that is unnecessary in Rust.
- Keep tests focused on current behavior only; do not add new feature-oriented cases.

### Deliverables
- Passing `cargo test`
- Regression coverage for migrated function behavior
- Final review for ownership, borrowing, and error propagation consistency