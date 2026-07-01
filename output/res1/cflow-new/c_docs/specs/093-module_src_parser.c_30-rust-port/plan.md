# Implementation Plan: module_src_parser.c_30

## Summary

This module ports the parser-oriented portion of `src/parser.c` to Rust, preserving the existing control flow and parsing behavior while replacing C-style mutable global state, manual stack management, and pointer-based token handling with explicit Rust-owned state.

The Rust implementation should remain a direct migration of the current file rather than a redesign. The main technical approach is:

- move parser state into a dedicated Rust module-local state struct,
- convert save/undo/finish token stack logic into `Vec`-backed state,
- preserve the current parsing entry points and function boundaries as closely as practical,
- represent parser outcomes with `Result` where failures are currently signaled through return codes or implicit state,
- keep parsing as a single-threaded, in-memory operation using the standard library.

The migration target should be a single Rust source module corresponding to the current C file, with helper types defined only as needed to model the existing C data and parser state.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Maintain parsing throughput in the same practical range as the C implementation for equivalent input sizes.
  - Avoid unnecessary token cloning; prefer borrowing or compact owned token values where required by save-stack behavior.
  - Keep stack-based parser bookkeeping amortized `O(1)` for push/pop operations through `Vec`.
  - Preserve linear scanning behavior for declaration and balanced-token skipping routines.

## Module Mapping

### C to Rust File Mapping

- `src/parser.c` → `src/parser.rs`

### Function Mapping

The Rust module should preserve the existing function decomposition, either as free functions scoped to the module or as methods on a parser state type when state access is pervasive.

| C Function | Rust Mapping | Notes |
|---|---|---|
| `save_token` | `fn save_token(...)` or `ParserState::save_token(&mut self, ...)` | Implement with `Vec`-backed saved-token storage. |
| `undo_save_stack` | `fn undo_save_stack(...)` or method | Restore saved parser/token state by truncation or replay based on current C semantics. |
| `finish_save_stack` | `fn finish_save_stack(...)` or method | Commit save-stack state without C manual memory cleanup. |
| `skip_to` | `fn skip_to(...)` | Direct scan routine over token stream. |
| `skip_balanced` | `fn skip_balanced(...)` | Use explicit nesting counters; no recursion unless C logic requires it. |
| `yyparse` | `fn yyparse(...) -> Result<..., ParseError>` | Main entry point; preserve external behavior. |
| `is_function` | `fn is_function(...) -> bool` | Keep lookahead behavior close to C implementation. |
| `parse_declaration` | `fn parse_declaration(...) -> Result<..., ParseError>` | Central declaration parser. |
| `skip_declaration` | `fn skip_declaration(...)` | Skipping path for unsupported or ignorable declaration forms. |
| `expression` | `fn expression(...) -> Result<..., ParseError>` | Preserve token consumption semantics. |
| `parse_function_declaration` | `fn parse_function_declaration(...) -> Result<..., ParseError>` | Keep declaration parsing split unchanged. |
| `fake_struct` | `fn fake_struct(...)` | Model current placeholder/normalization behavior only. |
| `parse_variable_declaration` | `fn parse_variable_declaration(...) -> Result<..., ParseError>` | Preserve declaration-side effects and emitted state. |
| `initializer_list` | `fn initializer_list(...) -> Result<..., ParseError>` | Port brace/balanced parsing exactly. |
| `parse_knr_dcl` | `fn parse_knr_dcl(...) -> Result<..., ParseError>` | Keep legacy K&R handling intact if present in current file. |

### Recommended Rust Organization

Keep the implementation restrained:

- `src/parser.rs`
  - parser state struct(s)
  - token save-stack support
  - direct ports of the listed parsing functions
  - local enums/structs required to replace anonymous C structures

If the crate currently uses `mod` declarations from `lib.rs` or `main.rs`, add only the single corresponding `parser` module entry.

## Data Model

The C analysis reports multiple anonymous structures. These should not be reproduced as anonymous tuple-like placeholders in Rust; each distinct role in `parser.c` should be assigned a narrowly named Rust type derived from its actual usage during porting.

### Data-structure Mapping Strategy

| C Data Shape | Rust Mapping | Notes |
|---|---|---|
| anonymous parser-global state struct | `struct ParserState` | Centralizes mutable state previously held globally or file-locally. |
| anonymous saved-token stack entry | `struct SavedToken` | Stores token plus any location/auxiliary fields needed by undo/finish operations. |
| anonymous token/lookahead record | `struct TokenRecord` or reuse existing token type | Reuse existing project token type if already defined elsewhere. |
| anonymous declaration parsing scratch struct | `struct DeclarationState` | For temporary declaration flags/specifiers if present. |
| anonymous function declaration scratch struct | `struct FunctionDeclState` | Only if distinct state is needed by `parse_function_declaration`. |
| anonymous variable declaration scratch struct | `struct VariableDeclState` | Only if current C code tracks separate variable parsing fields. |
| anonymous initializer parsing state | `struct InitializerState` | Only if `initializer_list` currently depends on dedicated bookkeeping. |
| anonymous K&R declaration state | `struct KnrDeclState` | Preserve legacy parsing state without widening scope. |
| anonymous expression parsing scratch data | small local `struct` or locals | Prefer locals unless data is shared across functions. |
| anonymous enum-like integer tags | `enum` with explicit variants | Replace magic integers where the current file uses symbolic categories. |
| null-terminated strings / token text pointers | `String` / `Box<str>` / borrowed `&str` | Choose ownership based on whether saved tokens outlive the lexer buffer. |

### Core Rust Types

The implementation should prefer a small set of explicit types:

```rust
struct ParserState {
    save_stack: Vec<SavedToken>,
    // migrated parser-local state fields from parser.c
}

struct SavedToken {
    // token identity and any associated parser position/state
}

enum ParseError {
    UnexpectedToken,
    EndOfInput,
    Message(String),
}
```

### Memory Management Decisions

- Replace manual allocation/free for saved tokens and temporary parser records with ownership through `Vec`, `String`, and stack locals.
- Avoid shared mutable aliasing; pass `&mut ParserState` explicitly.
- If token text in C relies on borrowed lexer buffers that become invalid after lookahead/save operations, convert those saved values to owned Rust strings during `save_token`.
- Use `Option<T>` instead of nullable pointers.
- Use slices or indices instead of raw pointer arithmetic when traversing buffered tokens.

### Error Handling Decisions

- Convert parse failure paths to `Result<T, ParseError>` internally.
- Where the external API requires C-like integer success/failure behavior, keep a thin compatibility wrapper at the Rust module boundary.
- Distinguish syntax termination (`EndOfInput`) from malformed input (`UnexpectedToken`) only where the current logic depends on that distinction.
- Do not add recovery behavior beyond what current skip routines already implement.

## Implementation Phases

## Phase 1: Establish Rust module skeleton and parser state

- Create `src/parser.rs` as the Rust counterpart to `src/parser.c`.
- Identify all file-scope mutable state in the C module and move it into `ParserState`.
- Introduce Rust placeholders for all anonymous C data structures, naming them by role discovered in the file.
- Define the minimal token and parse error abstractions required by the listed functions.
- Port low-level save-stack operations first:
  - `save_token`
  - `undo_save_stack`
  - `finish_save_stack`
- Add unit tests for save-stack semantics, especially:
  - nested save/undo sequences,
  - commit-after-save behavior,
  - ownership of saved token text where cloning is required.

## Phase 2: Port token-skipping and lookahead logic

- Port the non-emitting parser helpers that control token traversal:
  - `skip_to`
  - `skip_balanced`
  - `is_function`
  - `skip_declaration`
- Keep control flow close to the C implementation, using loops and counters rather than introducing new parsing abstractions.
- Replace pointer comparisons and sentinel checks with enum/value comparisons.
- Validate balanced-delimiter handling on nested inputs and malformed closing-token cases.
- Add focused tests for:
  - delimiter nesting,
  - skip-to target detection,
  - function-vs-variable lookahead decisions.

## Phase 3: Port declaration and expression parsing functions

- Port the main parser routines in dependency order:
  - `expression`
  - `parse_knr_dcl`
  - `initializer_list`
  - `fake_struct`
  - `parse_variable_declaration`
  - `parse_function_declaration`
  - `parse_declaration`
- Preserve the current split between function declarations, variable declarations, fake-struct handling, and K&R declaration parsing.
- Keep temporary parsing state local unless the C code clearly shares it across functions.
- Replace implicit C error propagation with `Result` returns, while preserving branch behavior and token consumption order.
- Add tests around:
  - declaration form discrimination,
  - initializer nesting,
  - K&R declaration handling if still supported by the original file,
  - fake-struct paths and skipped declarations.

## Phase 4: Port parser entry point and complete behavioral alignment

- Port `yyparse` last, after all helper routines compile and behave consistently.
- Connect `yyparse` to the migrated helper functions without restructuring the public parse flow.
- Align return behavior with the surrounding crate’s expected API shape.
- Remove any leftover C-style assumptions:
  - manual cleanup paths now covered by ownership,
  - nullable parser state references,
  - integer-coded internal booleans.
- Add integration-style tests using representative source snippets that exercise:
  - top-level declarations,
  - function declarations and definitions as currently recognized,
  - rollback through save-stack operations during lookahead,
  - malformed input reaching skip paths without panicking.