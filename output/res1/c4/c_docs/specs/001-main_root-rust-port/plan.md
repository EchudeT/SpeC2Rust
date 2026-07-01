# Implementation Plan

## Summary

Port the `main_root` C module into Rust by migrating the existing entry-point and parser/interpreter-oriented functions from `c4.c` and `hello.c` into a small Rust binary crate layout centered on `src/main.rs`. The implementation should preserve the current file/function responsibilities rather than redesigning the program structure.

Technical approach:

- Translate the C global-state-driven flow into Rust with explicit owned state collected in a single module-local context struct where needed.
- Migrate the duplicated function set (`next`, `expr`, `stmt`, `main`) by aligning each C source file to a Rust source responsibility:
  - `c4.c` becomes the primary Rust implementation of token advancement, expression parsing, statement parsing, and program entry.
  - `hello.c` is migrated only to the extent needed to preserve its existing `main` behavior, likely as a minimal alternate executable behavior folded into the same binary flow or represented as a simple helper if both C `main` functions cannot coexist directly.
- Use standard-library types (`String`, slices, `Vec`, enums, `Result`) to replace raw pointers, integer tags, and implicit error signaling.
- Preserve execution order and parser behavior first; only introduce Rust enums/structs where required to make state and token kinds explicit and memory-safe.

## Technical Context

- **Language/Version:** Rust 1.78+ stable
- **Primary Dependencies:** Rust standard library only
- **Testing:** `cargo test`
- **Performance Goals:**
  - Maintain comparable single-threaded execution characteristics to the C implementation for token scanning and parsing.
  - Avoid unnecessary string cloning during token traversal where borrowing or index-based access is sufficient.
  - Keep parser/interpreter state in contiguous standard collections (`Vec`, `String`) and avoid heap allocation in tight parse loops except where behavior requires owned values.

## Module Mapping

### Source File Mapping

- `c4.c` -> `src/main.rs`
- `hello.c` -> `src/hello.rs` or a narrowly scoped helper section in `src/main.rs` if its logic is trivial and only supports an alternate `main` flow

### Function Mapping

Because the C analysis shows duplicated names across files, preserve behavior by mapping according to original file ownership rather than merging semantics prematurely.

- `c4.c::next` -> `next` Rust function, likely operating on `&mut ParserState`
- `c4.c::expr` -> `expr` Rust function, likely operating on `&mut ParserState`
- `c4.c::stmt` -> `stmt` Rust function, likely operating on `&mut ParserState`
- `c4.c::main` -> crate entry `fn main()`
- `hello.c::main` -> helper function invoked conditionally, or migrated into a separate internal function such as `fn hello_main(...) -> Result<(), String>`

If `hello.c` is a standalone sample program with its own entrypoint semantics, keep its code isolated in a dedicated Rust module and expose only one actual Rust crate `main`.

## Data Model

No explicit C structs were identified in the analysis results, so the Rust data model should be introduced only as needed to replace implicit C global state and primitive encodings.

### Likely State Mappings

- C global parser/scanner variables -> Rust `struct ParserState`
- C token integer constants / char-class codes -> Rust `enum TokenKind`
- C source buffer pointers (`char *`, cursor arithmetic) -> Rust `String` plus byte index or `Vec<u8>` plus cursor index
- C expression/statement return codes (`int`) -> Rust integer types only if semantically meaningful; otherwise `Result<T, String>` for fallible operations
- C null-terminated strings -> Rust `String` / `&str`
- C arrays used as dynamic buffers -> Rust `Vec<T>`

### Proposed Minimal Rust Types

```rust
struct ParserState {
    source: String,
    cursor: usize,
    current_token: TokenKind,
}

enum TokenKind {
    Unknown,
    Eof,
    // Additional variants introduced only as required by migrated logic
}
```

Notes:

- Prefer index-based traversal over direct `&str` mutation to mirror pointer advancement safely.
- If the original C parser stores numeric token/value side channels, keep them as fields on `ParserState` rather than scattering mutable globals.
- Use `Result<_, String>` for parse/runtime errors unless the original behavior is strict process termination, in which case `main` may print diagnostics and exit.

## Implementation Phases

## Phase 1: Establish crate layout and migrate entry behavior

- Create a Rust binary crate on branch `001-main_root-rust-port`.
- Add `src/main.rs` as the primary destination for `c4.c` logic.
- Inspect both C `main` functions and determine which one represents the actual root executable path.
- Migrate the primary `main` flow first with equivalent argument intake using `std::env::args`.
- Move `hello.c` logic into either:
  - a private `src/hello.rs` module if it contains distinct behavior, or
  - a small helper function inside `src/main.rs` if it is trivial.
- Replace direct C process termination patterns with Rust return-driven control where practical, while keeping observable behavior aligned.

## Phase 2: Migrate scanner and parser state

- Introduce a minimal `ParserState` struct to gather all mutable parser/scanner state previously held in C globals or local static variables.
- Port `next` first, converting pointer arithmetic into index advancement over source bytes or chars.
- Define a minimal `TokenKind` enum only for token categories actually used by migrated `expr` and `stmt`.
- Ensure end-of-input handling is explicit and safe.
- Convert C sentinel/null checks into Rust boundary checks.
- Add focused unit tests for token advancement on representative input snippets.

## Phase 3: Migrate expression and statement functions

- Port `expr` and `stmt` directly from `c4.c`, preserving control flow and precedence handling rather than refactoring algorithm shape.
- Thread mutable parser state through function arguments instead of relying on globals.
- Replace implicit integer truth/error conventions with:
  - concrete return values where parsing succeeds, and
  - `Result` for invalid input or unsupported states.
- Keep temporary values stack-allocated where possible.
- Add unit tests covering:
  - basic expression parsing,
  - statement parsing,
  - invalid input paths that should surface errors instead of memory-unsafe behavior.

## Phase 4: Integrate, validate, and remove C assumptions

- Connect `main`, `next`, `expr`, and `stmt` into a single executable flow matching the original module behavior.
- Audit all translated code for C-specific assumptions:
  - unchecked indexing,
  - pointer aliasing,
  - mutable global state,
  - implicit integer narrowing/sign behavior.
- Normalize error reporting at the crate boundary so parse/runtime failures become clear Rust diagnostics.
- Run `cargo test` and fix behavior mismatches against the original C implementation intent.
- Keep the final layout minimal and limited to migrated file/function coverage only.