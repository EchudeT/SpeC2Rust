# Implementation Plan

## Summary

Port the `main_root` C module from `c4.c` and `hello.c` into a Rust binary crate on branch `001-main_root-rust-port`, preserving the existing execution flow and function boundaries as closely as practical. The migration should focus on the current parser/interpreter entry path represented by `next`, `expr`, `stmt`, and `main`, with Rust replacements that keep the original control flow and state transitions understandable during verification.

The Rust implementation should prefer a direct translation approach over redesign. Global or file-scoped C state should be consolidated into a small explicit Rust state holder passed by mutable reference where needed. Pointer-based scanning and integer-coded state should be mapped to slice/string iteration and typed enums or aliases only where the mapping is straightforward and does not alter behavior. Memory safety should come from owned buffers and borrowed views from the standard library, and failure cases that were implicit in C should become explicit `Result` returns only at function boundaries where I/O or process setup occurs.

## Technical Context

- **Language/Version**: Rust 1.78+ stable
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Keep parsing/scanning and statement/expression handling within the same asymptotic behavior as the C implementation.
  - Avoid unnecessary heap allocation during token scanning and expression parsing.
  - Preserve single-process command-line execution characteristics appropriate for a small compiler/interpreter-style binary.
  - Favor direct data access and sequential parsing over abstraction layers that would change runtime characteristics.

## Module Mapping

### Source File Mapping

- `c4.c` -> `src/main.rs`
- `hello.c` -> `src/bin/hello.rs` if it is a separate executable entry, otherwise fold into `src/main.rs` only if it is strictly auxiliary to the same binary behavior

### Function Mapping

Because the input lists duplicated symbols, the Rust plan should normalize them by source role rather than reproduce duplicates mechanically:

- `next` -> `fn next(state: &mut ParserState) -> Token` or `fn next(state: &mut ParserState)` depending on how much token state is stored globally in C
- `expr` -> `fn expr(state: &mut ParserState, precedence: i32) -> ExprValue` or nearest direct translation signature from the C control flow
- `stmt` -> `fn stmt(state: &mut ParserState) -> ExecStatus` or direct void-style translation if side effects are the original behavior
- `main` from `c4.c` -> `fn main() -> std::process::ExitCode`
- `main` from `hello.c` -> separate Rust binary entry only if `hello.c` was independently buildable in C; otherwise migrate only the relevant callable logic and avoid introducing extra binaries without evidence

### Rust Project Layout

Use the minimum standard Rust layout needed for the migrated files:

```text
src/
  main.rs
  bin/
    hello.rs   # only if hello.c is an independent program
```

Do not introduce extra library crates or helper modules unless needed to break a compile cycle during migration.

## Data Model

No explicit C structs were identified in the input. The plan should therefore derive Rust data structures only from the migrated file-scope state and function signatures found during implementation.

### Expected C-to-Rust Mappings

- C file-scope mutable parsing/execution state -> `struct ParserState`
- C integer token codes / operator codes -> `type Token = i32` initially, then `enum Token` only if the full constant set is known and conversion does not risk semantic drift
- C raw source pointer (`char *`) -> `&[u8]` with an index cursor, or `String`/`Vec<u8>` plus cursor for owned source text
- C symbol/value integers -> `i32`, `isize`, or `usize` according to original arithmetic and indexing usage
- C null-terminated strings -> `String` or borrowed `&str` where UTF-8 assumptions are valid; otherwise `Vec<u8>` / `&[u8]` for byte-accurate scanning
- C side-effect-only functions -> Rust functions returning `()` unless an explicit status/error return is needed for safe process-level handling

### Initial Rust State Holder

A restrained initial shape is appropriate:

```rust
struct ParserState {
    source: Vec<u8>,
    cursor: usize,
    token: i32,
    // additional migrated fields from C globals, added only as encountered
}
```

This should be expanded only with fields directly corresponding to existing C globals or static locals used by `next`, `expr`, and `stmt`.

### Memory Management and Error Handling

- Replace C buffer/pointer lifetime assumptions with owned input buffers and indexed access.
- Avoid unsafe Rust unless a direct C memory model cannot be represented otherwise; the default plan is safe Rust only.
- Convert command-line and file-loading failures in `main` into `Result` handling with explicit exit codes.
- Keep parser/runtime internal behavior close to C; if the original code aborts on invalid input, preserve that behavior with concise Rust error paths rather than adding recovery machinery.

## Implementation Phases

## Phase 1: Establish crate and migrate entry structure

- Create the Rust binary crate layout on branch `001-main_root-rust-port`.
- Migrate `c4.c` into `src/main.rs` as the primary target.
- Inspect `hello.c` to determine whether it is:
  - a separate executable, in which case migrate it to `src/bin/hello.rs`, or
  - a trivial alternate demo, in which case defer unless it is part of required module scope.
- Translate top-level constants, file-scope variables, and command-line/file-loading setup into Rust.
- Introduce a minimal `ParserState` (or similarly named) holder for C global state that must persist across `next`, `expr`, and `stmt`.
- Make `main` compile with placeholder bodies for parser functions if necessary.

**Exit criteria**:
- `cargo build` succeeds.
- Entry-point wiring and input loading are represented in Rust.
- Global C state required by parser functions has a defined Rust home.

## Phase 2: Port lexical scanning and shared parser state

- Migrate `next` first, since `expr` and `stmt` depend on token advancement.
- Replace pointer arithmetic with index-based traversal over a byte buffer.
- Port token/category constants exactly before considering enum refactoring.
- Preserve whitespace/comment/identifier/number scanning logic in original order.
- Add focused unit tests around token advancement using representative input fragments extracted from the C behavior.

**Exit criteria**:
- `next` behavior is implemented in Rust against `ParserState`.
- Token cursor advancement is testable and stable.
- No unsafe memory access patterns remain from the C scanner logic.

## Phase 3: Port expression and statement execution/parsing

- Migrate `expr` directly from C, preserving precedence handling, evaluation order, and state mutation.
- Migrate `stmt` next, preserving statement dispatch and block/control-flow semantics from the original implementation.
- Keep function signatures close to original behavior; introduce typed return values only when they clarify already-existing status/value flows.
- Resolve remaining shared global variables by moving them into `ParserState` rather than introducing broader module abstractions.
- Add tests that exercise `expr` and `stmt` through small source snippets and compare expected outcomes with the C implementation.

**Exit criteria**:
- `expr` and `stmt` compile and run through the same parser state as `next`.
- Core parsing/execution path is available from `main`.
- Tests cover representative expression and statement cases.

## Phase 4: Complete parity pass and finalize auxiliary file migration

- Reconcile any remaining differences between `c4.c`/`hello.c` and the Rust translation.
- If `hello.c` is an actual second program, complete its direct migration with minimal duplication.
- Replace any temporary placeholder integer aliases with enums/constants only where confirmed by the migrated code.
- Remove dead translation scaffolding introduced during bring-up.
- Add end-to-end `cargo test` cases for command-line-relevant behavior that can be checked without adding new infrastructure.

**Exit criteria**:
- All required functions from the listed module files are migrated.
- Rust entry points match the original file roles.
- The implementation remains minimal, direct, and aligned with the original C structure.