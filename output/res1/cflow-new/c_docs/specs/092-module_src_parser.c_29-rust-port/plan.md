# Implementation Plan: module_src_parser.c_29

## Summary

This module migration covers the parser token-handling and parser-state utilities currently implemented in `src/parser.c`. The Rust port should preserve the existing parsing flow and function boundaries as closely as practical, with a narrow focus on translating token inspection, token stack manipulation, parser checkpoint/restore behavior, error reporting, and parser initialization.

The Rust implementation should remain in the existing parser area of the project and convert C-style mutable global or shared parser state into explicit Rust-owned state structures passed by mutable reference where needed. The primary technical approach is:

- represent parser state, token buffers, and saved marks with Rust structs;
- replace manual memory management with owned containers such as `Vec` and `Option`;
- map token kinds to enums and debug/token-print helpers to methods or free functions;
- translate error paths into `Result` where the surrounding call chain allows it, while keeping externally visible behavior aligned with the C code;
- preserve token pushback/insert/delete semantics without introducing new parsing abstractions.

The implementation should be a direct port of the existing file logic, not a redesign of the parser.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - preserve parser throughput close to the C implementation for sequential token processing;
  - avoid unnecessary token cloning by moving or borrowing token data where practical;
  - keep stack and pushback operations amortized O(1) where the original behavior permits;
  - ensure no unbounded temporary allocations are introduced during token debug printing or parser initialization.

## Module Mapping

### Source File Mapping
- `src/parser.c` → `src/parser.rs`

### Function Mapping
The Rust port should keep migration scope aligned to the existing functions in this module.

| C Function | Rust Target | Notes |
|---|---|---|
| `print_token` | `fn print_token(...)` or parser-local helper | Keep as formatting/debug helper; prefer writing to a `fmt::Write` or returning `String` only if required by current usage. |
| `token_type_str` | `fn token_type_str(kind: TokenType) -> &'static str` | Prefer enum-based token kind mapping. |
| `dbgtok` | `fn dbgtok(...)` | Preserve debug-only token display behavior. |
| `debugtoken` | `fn debugtoken(...)` | Keep separate only if current call sites rely on both helpers distinctly. |
| `file_error` | `fn file_error(...) -> ParseError` or reporting helper | Convert C-side file/parser diagnostics into structured Rust error handling where feasible. |
| `mark` | `fn mark(&ParserState) -> ParserMark` | Snapshot parser position/state without raw pointer preservation. |
| `restore` | `fn restore(&mut ParserState, mark: ParserMark)` | Reapply saved parser/token position state. |
| `tokdel` | `fn tokdel(&mut ParserState, ...)` | Delete token from current stack/buffer representation. |
| `tokins` | `fn tokins(&mut ParserState, ...)` | Insert token into token sequence/stack using `Vec`. |
| `tokpush` | `fn tokpush(&mut ParserState, ...)` | Push token onto parser-managed stack. |
| `cleanup_stack` | `fn cleanup_stack(&mut ParserState)` | Explicit cleanup remains useful for deterministic state reset. |
| `clearstack` | `fn clearstack(&mut ParserState)` | Clear active stack contents; may delegate to `Vec::clear`. |
| `nexttoken` | `fn nexttoken(&mut ParserState) -> Result<Token, ParseError>` or equivalent | Main sequential token acquisition path; preserve putback interaction. |
| `putback` | `fn putback(&mut ParserState, token: Token)` | Restore one token or push back into a buffer according to current C semantics. |
| `init_parse` | `fn init_parse(...) -> ParserState` or `fn init_parse(&mut ParserState, ...)` | Initialize parser-owned state and buffers. |

## Data Model

Because the analysis only exposes anonymous C data structures, the Rust data model should be derived from actual field usage in `src/parser.c` and named according to role rather than anonymous source shape.

### C-to-Rust Structure Mapping

| C Data Structure | Rust Mapping | Migration Decision |
|---|---|---|
| anonymous token record | `struct Token` | Collect token type, lexeme/value payload, location, and flags into a single owned type. |
| anonymous token type/constants | `enum TokenType` | Replace numeric token identifiers/macros with a Rust enum; preserve exact discriminant mapping only if required by external logic. |
| anonymous parser stack node/list | `Vec<Token>` inside `ParserState` | Prefer contiguous owned storage over linked allocation unless the C code depends on stable node addresses. |
| anonymous parser mark/checkpoint | `struct ParserMark` | Store stack length, current index, and any other restorable parser offsets. |
| anonymous parser/global state | `struct ParserState` | Central mutable state for token stream, putback buffer, stack, current token, source position, and debug flags. |
| anonymous source/file context | `struct SourceContext` or fields within `ParserState` | Keep local to parser if not shared elsewhere. |
| anonymous error/location record | `struct ParseError` | Include file/path context, line/column if present, and message text. |
| remaining anonymous helper structs | inline fields in `ParserState` or small dedicated structs only where repeated logically | Do not split into extra modules; create named structs only when the C layout clearly represents a persistent concept. |

### Ownership and Memory Management

- Replace heap-managed token nodes and manual cleanup with `Vec<Token>`, `Option<Token>`, and owned `String`/byte storage as required by the original token payload.
- If the C implementation stores borrowed pointers into source buffers, convert these carefully:
  - use owned `String` if token text must outlive the current input buffer mutation;
  - use indices/ranges into an owned source buffer if the original code relies on zero-copy slices and the backing storage lifetime is parser-owned.
- Represent optional current token / pushed-back token with `Option<Token>`.
- Use explicit parser-state methods instead of implicit pointer mutation across globals.

### Error Handling Mapping

- Replace `file_error` side effects and sentinel returns with a `ParseError` type and `Result` in internal functions where practical.
- If some call paths are fundamentally side-effecting and non-fallible in existing architecture, keep a reporting helper that constructs formatted diagnostics and then mirrors the current behavior.
- Avoid `panic!` for parse errors; reserve panics for invariant violations only.

## Implementation Phases

### Phase 1: State and Type Extraction

- Inspect `src/parser.c` and identify all anonymous structures, token-related constants, and mutable parser-state variables used by:
  - `init_parse`
  - `mark`
  - `restore`
  - `nexttoken`
  - `putback`
- Create `src/parser.rs` with Rust equivalents for:
  - `TokenType`
  - `Token`
  - `ParserMark`
  - `ParserState`
  - `ParseError`
- Translate token type string/debug mappings first:
  - `token_type_str`
  - `print_token`
  - `dbgtok`
  - `debugtoken`
- Preserve naming close to the C file to simplify side-by-side verification.

**Exit criteria**:
- All parser state fields required by the listed functions are represented in Rust.
- Debug/token formatting helpers compile and match current token-kind naming.

### Phase 2: Token Buffer and Stack Operations

- Port the token mutation helpers using Rust-owned containers:
  - `tokpush`
  - `tokins`
  - `tokdel`
  - `cleanup_stack`
  - `clearstack`
- Implement stack cleanup semantics explicitly even if `Vec` would drop automatically, to preserve parser reset behavior and call ordering.
- Validate insertion/deletion semantics against C code, especially:
  - index movement after delete;
  - whether inserted tokens become current, next, or pending;
  - whether cleanup must also reset associated parser marks or current token pointers.
- Introduce focused unit tests for stack and pushback behavior from observed C semantics.

**Exit criteria**:
- Token stack operations are complete and pass unit tests covering push, insert, delete, and clear/reset behavior.
- No manual memory lifecycle from C remains in these paths.

### Phase 3: Sequential Parsing Control Flow

- Port parser checkpoint and token retrieval flow:
  - `mark`
  - `restore`
  - `putback`
  - `nexttoken`
- Replace pointer-based save/restore with index/length-based snapshots in `ParserMark`.
- Ensure `nexttoken` respects:
  - pushed-back tokens;
  - current stack contents;
  - source input progression;
  - error propagation through `ParseError`.
- Confirm that restore behavior correctly rewinds all state that the C implementation considers part of the parse checkpoint.

**Exit criteria**:
- Mark/restore round-trips are verified by tests.
- Token retrieval order matches C behavior for normal, pushed-back, and restored states.

### Phase 4: Initialization and Integration Finalization

- Port `init_parse` and any `file_error` integration required by parser setup and early tokenization state.
- Wire the Rust module into the project branch using the existing module location and standard crate layout only.
- Add module-local tests covering:
  - initialization of empty/default parser state;
  - error creation/reporting paths;
  - end-to-end token flow through init → nexttoken → putback → restore-related operations.
- Remove or isolate remaining C assumptions such as null checks, sentinel token pointers, and manual deallocation branches.

**Exit criteria**:
- `cargo test` passes.
- The Rust parser module covers all listed functions from `src/parser.c`.
- The migrated implementation preserves existing behavior without adding new parser facilities.

## Notes and Constraints

- Keep all migrated logic in the parser module area; do not introduce extra architectural layers.
- Prefer method implementations on `ParserState` only when they directly replace C functions operating on parser state.
- Avoid speculative refactoring of surrounding parser or lexer behavior beyond what is required for these listed functions.
- Preserve externally observable token ordering, debug output intent, and parser reset/checkpoint semantics.