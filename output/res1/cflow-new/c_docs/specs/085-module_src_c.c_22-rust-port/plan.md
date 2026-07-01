# Implementation Plan: module_src_c.c_22

## Summary

This module appears to implement a small preprocessing/token-source layer centered on opening and closing input, reading tokens and numeric values, handling escaped input, updating source location state, and finalization/wrap-up behavior. The Rust port should preserve the existing control flow and state transitions from `src/c.c` with minimal redesign.

The implementation approach is a direct migration of the current C file into a single Rust module that keeps the same operational boundaries:

- input lifecycle: `pp_open`, `pp_close`, `pp_finalize`, `yywrap`
- token/input scanning: `get_token`, `source`, `getnum`, `backslash`
- location tracking: `update_loc`

The Rust version should replace implicit global/process state and raw pointer manipulation with explicit module-local state structs, borrowed slices/strings where possible, and `Result`-based error propagation for I/O and parse failures. Where C behavior depends on sentinel values or integer return codes, the Rust code should preserve externally visible behavior while using internal enums to make state transitions explicit.

## Technical Context

### Language/Version
- Rust 1.78 or newer

### Primary Dependencies
- Rust standard library only:
  - `std::fs`
  - `std::io`
  - `std::path`
  - `std::str`
  - `std::mem` as needed

No third-party crates are recommended based on the available module evidence.

### Testing
- `cargo test`

Tests should focus on behavior-preserving migration of:
- input open/close/finalize paths
- token extraction behavior
- numeric parsing behavior in `getnum`
- escape handling in `backslash`
- line/column or source position updates in `update_loc`
- end-of-input behavior matching `yywrap`

### Performance Goals
- Maintain performance comparable to the C implementation for sequential file/token scanning.
- Avoid unnecessary allocation during tokenization by reusing buffers where practical.
- Keep parsing and location updates linear in input size.
- Do not introduce additional abstraction layers that materially change the hot path.

## Module Mapping

### Source File Mapping
- `src/c.c` -> `src/module_src_c_c_22.rs`

If the crate already has a module tree, expose it with the minimal corresponding declaration from:
- `src/lib.rs` or `src/main.rs` -> `mod module_src_c_c_22;`

### Function Mapping
- `pp_finalize` -> `pub(crate) fn pp_finalize(state: &mut PreprocessorState) -> Result<(), ModuleError>`
- `pp_open` -> `pub(crate) fn pp_open(state: &mut PreprocessorState, path: &Path) -> Result<(), ModuleError>`
- `pp_close` -> `pub(crate) fn pp_close(state: &mut PreprocessorState) -> Result<(), ModuleError>`
- `yywrap` -> `pub(crate) fn yywrap(state: &mut PreprocessorState) -> bool`
- `get_token` -> `pub(crate) fn get_token(state: &mut PreprocessorState) -> Result<Token, ModuleError>` or sentinel-preserving equivalent
- `source` -> `pub(crate) fn source(state: &mut PreprocessorState) -> Result<Option<char>, ModuleError>`
- `getnum` -> `pub(crate) fn getnum(state: &mut PreprocessorState, first: char) -> Result<NumericToken, ModuleError>` or integrated token parse helper
- `backslash` -> `pub(crate) fn backslash(state: &mut PreprocessorState) -> Result<Option<char>, ModuleError>`
- `update_loc` -> `pub(crate) fn update_loc(state: &mut SourceLocation, ch: char)`

Notes:
- Visibility should remain crate-local unless this module is already externally consumed.
- If exact return contracts are required by neighboring code, preserve them with Rust enums or `Option`/`Result` wrappers plus thin compatibility conversion.

## Data Model

Because the input only identifies anonymous C structures, the Rust port should introduce a restrained set of named internal types based strictly on the observed function responsibilities.

### Core State Mapping
- C anonymous preprocessing/input state -> `struct PreprocessorState`
  - owns current input buffer/reader state
  - tracks whether input is open
  - stores current token scratch buffer
  - stores source location
  - stores end-of-input/finalized flags as needed

- C anonymous location state -> `struct SourceLocation`
  - likely fields:
    - `line: usize`
    - `column: usize`
    - `offset: usize`
    - optional source/file identifier if used by `pp_open`

- C anonymous token scratch/output record -> `enum Token`
  - variants only as required by the current parser contract, such as:
    - `Identifier(String)`
    - `Number(NumericToken)`
    - `Symbol(char)`
    - `End`
  - If the surrounding code expects integer token codes, use:
    - `enum TokenKind`
    - plus payload fields in a compact token struct

- C anonymous numeric parse storage -> `struct NumericToken`
  - preserve only fields required by current `getnum` behavior
  - likely:
    - raw lexeme or parsed value
    - numeric kind if the C code distinguishes bases/forms

### Error Mapping
- C integer error/sentinel returns -> `enum ModuleError`
  - `Io(std::io::Error)`
  - `InvalidNumber`
  - `UnexpectedEof`
  - `InvalidEscape`
  - `State(&'static str)`

Use `From<std::io::Error>` for propagation. Do not add broader error frameworks.

### Memory Management Decisions
- Replace raw buffers and manual lifetime management with owned `String`/`Vec<u8>` buffers.
- Use `Option<File>` or a buffered in-memory input representation instead of nullable file handles.
- Avoid self-referential structures.
- Keep token text owned if neighboring code needs it after the scanner advances; otherwise prefer temporary buffer reuse inside `PreprocessorState`.

### C-to-Rust Type Mapping
- `char*` input/token buffer -> `String` or `Vec<u8>`
- file handle pointer -> `std::fs::File` wrapped in `Option`
- integer status code -> `Result<T, ModuleError>` or `bool` where semantics are only wrap/end detection
- mutable global state -> explicit `&mut PreprocessorState`
- anonymous structs -> named `struct`/`enum` definitions in the module

## Implementation Phases

## Phase 1: Establish module shell and state translation

### Goals
- Create the Rust module file and minimal public surface.
- Translate the implicit C state into explicit Rust structs.
- Preserve current lifecycle boundaries without implementing extra features.

### Tasks
- Add `src/module_src_c_c_22.rs`.
- Define:
  - `PreprocessorState`
  - `SourceLocation`
  - `Token` and/or `TokenKind`
  - `NumericToken` if needed
  - `ModuleError`
- Add function signatures for all migrated C functions.
- Identify all file-scope/static C variables in `src/c.c` and move them into `PreprocessorState`.
- Decide on the narrowest viable input representation:
  - `File` + internal read buffer, or
  - full-file `String` if the original code already behaves as buffered text processing

### Deliverable
- Compilable Rust module with placeholders and complete type skeletons matching the C module’s responsibilities.

## Phase 2: Port input lifecycle and source location behavior

### Goals
- Make input open/close/finalize behavior work first.
- Port source character retrieval and location updates with behavior preserved.

### Tasks
- Implement `pp_open` to initialize state from a file path.
- Implement `pp_close` to release file/input resources by clearing owned state.
- Implement `pp_finalize` to perform any remaining cleanup and reset flags.
- Implement `yywrap` as the end-of-input indicator consistent with the original logic.
- Implement `source` as the single character/token-source primitive used by higher-level scanning.
- Implement `update_loc` with exact newline/column semantics from the C code.
- Add unit tests for:
  - opening valid and invalid files
  - closing/resetting state
  - EOF handling
  - line/column advancement across ordinary characters and newline sequences

### Deliverable
- Working input lifecycle and location tracking layer with tests.

## Phase 3: Port token and numeric scanning

### Goals
- Recreate token extraction logic without changing the parser-facing contract.
- Preserve numeric and escape semantics.

### Tasks
- Implement `backslash` according to the C escape-handling logic.
- Implement `getnum` with the same accepted forms and failure behavior as the C version.
- Implement `get_token` using the Rust source primitive and scratch buffers in `PreprocessorState`.
- Preserve any sentinel/end token conventions expected by callers.
- Ensure buffer reuse instead of repeated transient allocation where the C code used mutable buffers.
- Add unit tests from representative input cases:
  - plain tokens
  - numeric forms
  - escaped sequences
  - EOF immediately after partial token
  - malformed numeric/escape input as defined by current behavior

### Deliverable
- Feature-complete scanner/token-source behavior matching `src/c.c`.

## Phase 4: Behavior alignment and cleanup

### Goals
- Remove migration placeholders and verify parity with the original module’s edge cases.
- Keep the module narrow and ready for integration on the target branch.

### Tasks
- Compare each Rust function against the original C control flow and return behavior.
- Replace any temporary `panic!`/`todo!` with structured errors or preserved sentinel returns.
- Normalize function signatures if neighboring Rust code requires compatibility shims.
- Review ownership and borrowing to ensure no unnecessary cloning remains in the hot path.
- Run `cargo test` and fix any divergences in tokenization, EOF handling, or location accounting.

### Deliverable
- Completed Rust port of `module_src_c.c_22` suitable for branch `085-module_src_c.c_22-rust-port`.

## Notes and Constraints

- Keep the implementation confined to the migrated module and existing crate entrypoints.
- Do not introduce extra helper modules unless required by the existing Rust project structure.
- Prefer explicit state passing over recreated globals.
- Preserve original parsing behavior even where Rust could offer a more idiomatic redesign.
- Convert manual cleanup to normal Rust ownership, but keep explicit reset functions where the original module exposed them.
- Any unknown anonymous C structures should be named according to actual role discovered during migration, not generalized into reusable framework types.