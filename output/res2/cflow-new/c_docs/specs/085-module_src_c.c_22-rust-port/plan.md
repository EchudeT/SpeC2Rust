# Implementation Plan: module_src_c.c_22

## Summary

This module will port `src/c.c` into a single Rust module that preserves the current preprocessing and token-source flow centered around:

- preprocessor lifecycle control: `pp_open`, `pp_close`, `pp_finalize`
- token and source extraction: `get_token`, `source`, `getnum`
- character and line handling helpers: `backslash`, `update_loc`
- compatibility end hook: `yywrap`

The Rust implementation should keep the existing execution model and migrate the current file-local state into explicit Rust-owned state structures. The main technical approach is:

- convert implicit C global/static mutable state into a dedicated Rust context struct
- replace raw pointers and manual buffer handling with `String`, `Vec<u8>`, and slices
- preserve function boundaries where practical so migration can be verified incrementally against the C behavior
- use `Result` for fallible open/read/parse operations and simple return values for token/status helpers that are not inherently exceptional

The port should remain narrow in scope: one Rust module corresponding to `src/c.c`, with only the minimum internal types required to represent the existing anonymous C data shapes and parser/preprocessor state.

## Technical Context

- **Language/Version:** Rust 1.78+
- **Primary Dependencies:** Rust standard library only (`std::fs`, `std::io`, `std::path`, `std::mem`, `std::str`)
- **Testing:** `cargo test`
- **Performance Goals:**
  - match the current module’s single-threaded file-processing characteristics without avoidable extra allocations
  - keep token scanning and source updates linear in input size
  - avoid repeated string copying where borrowed slices or reusable buffers are sufficient
  - preserve predictable memory usage by making parser/preprocessor state explicit and owned

## Module Mapping

### Source File Mapping

- `src/c.c` → `src/module_src_c_c_22.rs`

If the Rust crate already uses a different naming convention for ports, the implementation should still remain a single Rust source file for this module and avoid splitting logic into additional submodules unless required by the existing project layout.

### Function Mapping

- `pp_finalize` → `pub(crate) fn pp_finalize(ctx: &mut PreprocessorContext) -> Result<(), ModuleError>`
- `pp_open` → `pub(crate) fn pp_open(ctx: &mut PreprocessorContext, path: &Path) -> Result<(), ModuleError>`
- `pp_close` → `pub(crate) fn pp_close(ctx: &mut PreprocessorContext) -> Result<(), ModuleError>`
- `yywrap` → `pub(crate) fn yywrap(ctx: &mut PreprocessorContext) -> bool`
- `get_token` → `pub(crate) fn get_token(ctx: &mut PreprocessorContext) -> Result<TokenKind, ModuleError>`
- `source` → `pub(crate) fn source(ctx: &mut PreprocessorContext) -> Result<Option<char>, ModuleError>` or equivalent byte-oriented form, depending on original behavior
- `getnum` → `pub(crate) fn getnum(ctx: &mut PreprocessorContext, first: char) -> Result<TokenKind, ModuleError>`
- `backslash` → `pub(crate) fn backslash(ctx: &mut PreprocessorContext) -> Result<bool, ModuleError>`
- `update_loc` → `pub(crate) fn update_loc(ctx: &mut PreprocessorContext, ch: char)`

### State Mapping

Any static storage or file-scope mutable variables in `src/c.c` should be moved into:

- `PreprocessorContext` for scanning/preprocessor state
- `SourceLocation` for line/column/file tracking
- small internal enums for token categories and scanner mode where needed

This keeps ownership explicit and removes unsafe shared mutable state.

## Data Model

Because the C analysis exposes only anonymous data structures, the Rust plan should introduce named internal types according to operational role rather than attempting one-to-one anonymous replication.

### C Anonymous Structs/Unions → Rust Types

- anonymous structure used for file/preprocessor session state
  → `struct PreprocessorContext`
- anonymous structure used for current source position
  → `struct SourceLocation`
- anonymous structure used for token assembly/buffer state
  → `struct TokenBuffer`
- anonymous structure used for input source/file stack, if present
  → `struct SourceFrame`
- anonymous structure used for numeric token parsing intermediates, if present
  → fold into `TokenKind::Number(...)` or a dedicated `NumberToken`
- anonymous flag/grouping records
  → plain Rust fields on `PreprocessorContext`
- anonymous constant categories
  → `enum TokenKind`
- anonymous mode/state selectors
  → `enum ScanState`

### Recommended Rust Structures

```rust
pub(crate) struct PreprocessorContext {
    input: Option<std::fs::File>,
    path: Option<std::path::PathBuf>,
    buffer: String,
    cursor: usize,
    token_buffer: TokenBuffer,
    loc: SourceLocation,
    source_stack: Vec<SourceFrame>,
    reached_eof: bool,
}
```

```rust
pub(crate) struct SourceLocation {
    line: usize,
    column: usize,
    byte_offset: usize,
}
```

```rust
pub(crate) struct TokenBuffer {
    text: String,
}
```

```rust
pub(crate) struct SourceFrame {
    path: std::path::PathBuf,
    buffer: String,
    cursor: usize,
    loc: SourceLocation,
}
```

```rust
pub(crate) enum TokenKind {
    Number(String),
    Symbol(String),
    Char(char),
    EndOfInput,
}
```

```rust
pub(crate) enum ModuleError {
    Io(std::io::Error),
    InvalidState(&'static str),
    InvalidToken,
    NumericParse,
}
```

### Memory Management Decisions

- replace C-owned character arrays with owned `String` buffers
- replace pointer arithmetic with cursor indices into buffers
- use `Option<File>` and `Vec<SourceFrame>` instead of nullable pointers and manually managed stacks
- avoid `unsafe` unless a specific C behavior cannot be expressed otherwise; the default plan assumes safe Rust is sufficient
- ensure close/finalize routines clear owned buffers and stack state deterministically, even though Rust already guarantees drop behavior

### Error Handling Decisions

- `pp_open`, `pp_close`, `pp_finalize`, `source`, `get_token`, `getnum`, and `backslash` should return `Result` where I/O or malformed state can occur
- `yywrap` should return `bool` if it is only an EOF-style predicate
- `update_loc` should be infallible and mutate location state directly
- map `std::io::Error` into `ModuleError::Io`
- represent impossible C-state assumptions as `InvalidState` rather than panicking in normal execution paths

## Implementation Phases

## Phase 1: Establish module skeleton and state translation

- create `src/module_src_c_c_22.rs`
- define `PreprocessorContext`, `SourceLocation`, `TokenBuffer`, and any minimal enums needed to represent scanner state
- identify all file-scope variables and anonymous data uses from `src/c.c` and move them into `PreprocessorContext` fields
- define `ModuleError` and the common `Result` type alias if the crate already uses one; otherwise use explicit `Result<_, ModuleError>`
- implement `update_loc` first, since later scanning functions depend on consistent source tracking

**Exit criteria:**
- module compiles with placeholder implementations
- all previously implicit mutable state has an explicit Rust owner
- location updates are covered by basic unit tests

## Phase 2: Port lifecycle and input-source management

- implement `pp_open` using `std::fs::File` and input buffering compatible with the original processing model
- implement `pp_close` to release current source state and reset cursors/buffers consistently
- implement `pp_finalize` to clear all remaining stacked input/session data
- implement `yywrap` based on the original EOF/final-input semantics
- if the C code maintains nested source/include frames, port that behavior directly into `source_stack` without adding new abstractions

**Exit criteria:**
- open/close/finalize flow works deterministically across repeated calls
- EOF behavior matches expected lifecycle transitions
- tests cover open → consume → close and finalize-after-open sequences

## Phase 3: Port character sourcing and lexical helpers

- implement `source` as the central character/byte fetch routine
- implement `backslash` with the same continuation/escape handling logic used by the C module
- ensure `source` and `backslash` both call `update_loc` consistently whenever consumed input advances source position
- preserve C ordering semantics for newline handling, escaped newlines, and EOF transitions

**Exit criteria:**
- character consumption behavior is stable for plain input, newline boundaries, and backslash-related cases
- unit tests cover representative edge cases from the C logic

## Phase 4: Port token extraction and numeric scanning

- implement `getnum` to parse numeric sequences using the same accepted character classes and stop conditions as the C function
- implement `get_token` on top of `source`, `backslash`, and `getnum`
- keep token text assembly in `TokenBuffer` to minimize temporary allocations and to mirror the original staged scanning behavior
- preserve return-shape compatibility as closely as possible within Rust enums and results, without widening functionality

**Exit criteria:**
- tokenization path compiles and exercises all migrated helper functions
- tests validate numeric token extraction, ordinary token boundaries, and end-of-input behavior
- no remaining logic from the listed C functions is unported