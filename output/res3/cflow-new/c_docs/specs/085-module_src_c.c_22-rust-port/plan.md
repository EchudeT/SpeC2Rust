# Implementation Plan

## Summary

This module is a focused port of `src/c.c` into Rust for the `085-module_src_c.c_22-rust-port` branch. The C file appears to combine source-input management, token extraction, numeric parsing, escape handling, location tracking, and lifecycle hooks around a preprocessing or lexical-input stage.

The Rust implementation should preserve the existing behavior and call flow of the C module without adding new capabilities. The preferred technical approach is to migrate the current file into a single Rust module with direct function-level correspondence for:

- module lifecycle: `pp_open`, `pp_close`, `pp_finalize`
- scanner hooks: `yywrap`, `get_token`
- source and character handling: `source`, `backslash`
- parsing helpers: `getnum`
- position tracking: `update_loc`

The implementation should replace C global/static state and anonymous structs with explicit Rust structs held within a module-local state object. Ownership should be made explicit, input/resource cleanup should be represented by `Drop` only where it directly matches existing C cleanup behavior, and fallible operations should return `Result` instead of relying on sentinel values where practical. Where the original interface depends on integer/status returns, Rust code should preserve that behavior internally and adapt it with narrow wrappers as needed.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Maintain behavior comparable to the C implementation for sequential source scanning.
  - Avoid unnecessary allocations during tokenization and source reading.
  - Preserve streaming-style processing where the C code reads and advances through input incrementally.
  - Keep location tracking and character classification O(1) per consumed character.

## Module Mapping

### Source File Mapping

- `src/c.c` -> `src/module_src_c_c_22.rs`

If the crate already exposes per-module files through `lib.rs` or `mod.rs`, add only the minimal declaration needed to compile this port.

### Function Mapping

| C Function | Rust Target | Notes |
|---|---|---|
| `pp_finalize` | `fn pp_finalize(state: &mut PreprocessorState)` | Final cleanup of module-owned state/resources. |
| `pp_open` | `fn pp_open(...) -> Result<PreprocessorState, ModuleError>` or `fn pp_open(state: &mut PreprocessorState, ...) -> Result<(), ModuleError>` | Final signature should match how the surrounding project constructs module state. |
| `pp_close` | `fn pp_close(state: &mut PreprocessorState)` | Close current source/resource handles without dropping unrelated state. |
| `yywrap` | `fn yywrap(state: &mut PreprocessorState) -> bool` | Preserve end-of-input semantics expected by scanner flow. |
| `get_token` | `fn get_token(state: &mut PreprocessorState) -> Result<TokenKind, ModuleError>` or integer-compatible equivalent | Keep return behavior aligned with current parser/scanner integration. |
| `source` | `fn source(state: &mut PreprocessorState) -> Option<u8>` or `Result<Option<u8>, ModuleError>` | Represents the next input unit from current source. |
| `getnum` | `fn getnum(state: &mut PreprocessorState, first: u8) -> ParsedNumber` or equivalent | Port exact numeric scanning rules from C. |
| `backslash` | `fn backslash(state: &mut PreprocessorState, ch: u8) -> u8` or fallible equivalent | Preserve line-continuation / escape semantics exactly. |
| `update_loc` | `fn update_loc(state: &mut PreprocessorState, ch: u8)` | Centralize line/column/file position updates. |

### Internal Organization

Keep the Rust implementation in one module unless compilation constraints require a second file for shared types. Do not split lexer/state/helpers into extra modules unless existing Rust project layout already mandates it.

Suggested internal sections within `src/module_src_c_c_22.rs`:

1. state/type definitions
2. lifecycle functions
3. source-reading helpers
4. token/parsing helpers
5. tests

## Data Model

The C analysis reports only anonymous data structures. The Rust plan should therefore introduce named internal types based strictly on observed function responsibilities, not inferred extra features.

### State Mapping

| C Construct | Rust Mapping | Purpose |
|---|---|---|
| file-scope globals/statics in `src/c.c` | `PreprocessorState` struct | Consolidates mutable module state into one owned value. |
| anonymous source/input record | `SourceState` struct | Tracks current input source, cursor/buffer, and EOF state. |
| anonymous location record | `SourceLocation` struct | Tracks line, column, and possibly source identifier. |
| anonymous token/result record | `TokenKind` enum and/or `Token` struct | Represents scanner output if the original C code uses token categories. |
| anonymous numeric parse record | `ParsedNumber` struct or primitive tuple | Holds numeric value/type details only if required by current behavior. |
| integer status/error conventions | `ModuleError` enum | Maps open/read/parse failures into explicit Rust errors. |

### Recommended Rust Types

#### `PreprocessorState`
A single mutable state object for this module.

Possible fields, to be finalized during code migration from `src/c.c`:

- current source handle/buffer
- pending character or pushback slot if the C code uses one
- current token assembly buffer
- current source location
- flags previously stored as integers/bitfields
- any counters needed by `yywrap`, `source`, or `update_loc`

#### `SourceState`
Represents the current input stream.

Preferred standard-library representations:

- `std::fs::File` if the C code reads from files directly
- `std::io::BufReader<File>` if buffered reads are already implied by C stdio usage
- `Vec<u8>` / slice cursor only if the original code is already memory-buffer based

Do not introduce abstraction layers over readers unless the surrounding Rust crate already requires them.

#### `SourceLocation`
Likely fields:

- `line: usize`
- `column: usize`
- `offset: usize`
- optional source name/path field if the C module carries one

Use `usize` for indexes/counters unless the original behavior depends on fixed-width wrapping.

#### `TokenKind`
If `get_token` returns token categories from integer constants/macros, convert them to a Rust enum and keep numeric compatibility only at module boundaries if necessary.

Example shape:

```rust
enum TokenKind {
    End,
    Number,
    Identifier,
    Symbol(u8),
}
```

This should remain minimal and only reflect categories actually present in `src/c.c`.

#### `ParsedNumber`
Use a small struct only if `getnum` needs to return both value and scan metadata. Otherwise return a primitive integer type plus any consumed-state updates through `PreprocessorState`.

### Memory Management

- Replace manual open/close ownership with RAII-managed Rust fields.
- Keep explicit `pp_close`/`pp_finalize` functions because they are part of the migrated API, even if Rust would otherwise drop state automatically.
- Avoid unsafe code unless the surrounding project interface forces raw-pointer interoperability.
- Replace mutable shared buffers with owned `Vec<u8>` or `String` buffers scoped inside `PreprocessorState`.

### Error Handling

- Convert file open/read failures to `Result`.
- Preserve EOF as `Option` or a dedicated token/status instead of treating it as a hard error.
- Keep parsing behavior strict to the original C rules; do not broaden accepted numeric or escape formats.
- Where the rest of the port expects C-like status integers, isolate that compatibility at the function boundary and keep internal logic typed.

## Implementation Phases

## Phase 1: Establish module shell and migrate state definitions

### Goals
- Create the Rust module file for this C unit.
- Identify all file-scope mutable state from `src/c.c`.
- Replace anonymous structs and globals with named Rust types.

### Tasks
- Add `src/module_src_c_c_22.rs`.
- Declare minimal module exposure in the crate root if required.
- Define:
  - `PreprocessorState`
  - `SourceState`
  - `SourceLocation`
  - minimal `ModuleError`
- Transcribe C constants/macros used by the listed functions into Rust `const` items or enums.
- Record any sentinel values currently used by `source`, `get_token`, and `yywrap`.

### Completion Criteria
- Rust module compiles with placeholder function bodies.
- All state previously implicit in C has an explicit Rust home.
- No extra helper modules or infrastructure added.

## Phase 2: Port lifecycle and source-management functions

### Goals
- Migrate resource setup/teardown and sequential input handling first.
- Make location updates deterministic before token parsing is ported.

### Tasks
- Port `pp_open`.
  - Map file/resource acquisition to standard library I/O.
  - Initialize state fields to match C defaults.
- Port `pp_close`.
  - Clear/close current source state explicitly.
- Port `pp_finalize`.
  - Reset or release any remaining module-owned resources.
- Port `source`.
  - Implement byte/character retrieval in the same order and buffering style as C.
  - Preserve EOF behavior exactly.
- Port `update_loc`.
  - Match line/column advancement rules, including newline handling.
- Port `yywrap`.
  - Preserve end-of-input transition behavior.

### Completion Criteria
- Input can be opened, consumed, closed, and finalized through the Rust API.
- Location tracking matches C behavior on representative multiline inputs.
- EOF handling is stable and testable.

## Phase 3: Port parsing helpers and token extraction

### Goals
- Migrate the lexical logic with behavior fidelity to the original file.
- Keep helper sequencing close to the C implementation to simplify review.

### Tasks
- Port `backslash`.
  - Preserve escape and/or line-continuation handling exactly.
- Port `getnum`.
  - Match the original numeric base detection, accumulation, and termination rules.
  - Keep overflow behavior aligned with the C code’s effective semantics as closely as practical.
- Port `get_token`.
  - Preserve control flow and token boundaries from the C implementation.
  - Reuse `source`, `backslash`, `getnum`, and `update_loc` rather than redesigning the scanner.
- Convert any C switch/case token dispatch into Rust `match` expressions with the same branch ordering.

### Completion Criteria
- Token extraction works end-to-end over the same source path as the C module.
- Numeric and escape handling are covered by unit tests derived from observed C behavior.
- No new token model or parser layer has been introduced beyond what is necessary for the port.

## Phase 4: Behavioral verification and cleanup

### Goals
- Lock down parity-oriented tests.
- Remove temporary migration scaffolding while preserving the existing API shape.

### Tasks
- Add unit tests for:
  - open/close/finalize lifecycle
  - EOF and `yywrap`
  - line/column updates
  - numeric scanning via `getnum`
  - escape or continuation behavior via `backslash`
  - token boundaries via `get_token`
- Compare edge-case outputs against the C implementation for a small set of fixture inputs.
- Simplify signatures only where doing so does not alter integration expectations.
- Ensure all resource ownership paths are explicit and leak-free.

### Completion Criteria
- `cargo test` passes.
- Migrated functions are implemented without placeholders.
- The module remains narrowly scoped to the original `src/c.c` responsibilities.