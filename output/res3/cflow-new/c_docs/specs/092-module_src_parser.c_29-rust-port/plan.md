# Implementation Plan: module_src_parser.c_29

## Summary

This module ports the parser token-stream support logic from `src/parser.c` into Rust, preserving the existing parsing behavior and mutation patterns rather than redesigning the parser. The migrated Rust code should cover token inspection, token stack manipulation, parser position save/restore, token pushback, and parser initialization.

The Rust implementation should stay close to the C control flow:

- represent parser state explicitly in Rust structs,
- replace raw pointer and manual lifetime handling with owned collections and indices,
- convert token-stack operations into safe `Vec`-based mutations,
- preserve debug/token-print helper behavior in narrow utility functions,
- translate file and parse errors into `Result`-based APIs where the caller currently depends on failure signaling.

The implementation should be limited to migrating the behavior now embodied in `src/parser.c` for the listed functions, without introducing new parser abstractions or expanding the module boundary.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Maintain parser throughput comparable to the C implementation for sequential token consumption.
  - Keep token push/pop/insert/delete operations amortized O(1) or O(n) only where the original logic requires positional mutation.
  - Avoid unnecessary token cloning by preferring ownership moves and indexed access.
  - Ensure no memory leaks or use-after-free risks in temporary token-stack and mark/restore state.

## Module Mapping

| C Source | Rust Target | Notes |
|---|---|---|
| `src/parser.c` | `src/parser.rs` | Direct migration target for token handling and parser state logic. |
| `print_token` | `parser::print_token` | Keep as a formatting/debug helper; may return `String` or write to a formatter depending on existing call sites. |
| `token_type_str` | `parser::token_type_str` | Map token type to `&'static str`, likely via `match`. |
| `dbgtok` | `parser::dbgtok` | Preserve as debug-oriented helper with minimal surface change. |
| `debugtoken` | `parser::debugtoken` | Keep near `dbgtok`; may delegate to shared formatting logic. |
| `file_error` | `parser::file_error` | Convert to structured error construction/reporting rather than unchecked side effects where possible. |
| `mark` | `parser::mark` | Save parser/token-stream position into explicit state snapshot. |
| `restore` | `parser::restore` | Restore parser/token-stream position from saved snapshot. |
| `tokdel` | `parser::tokdel` | Delete token(s) from internal buffer/stack using `Vec` operations. |
| `tokins` | `parser::tokins` | Insert token(s) into internal buffer/stack using `Vec::insert` or splice-like logic. |
| `tokpush` | `parser::tokpush` | Push token into parser-managed token storage. |
| `cleanup_stack` | `parser::cleanup_stack` | Release/reset temporary parser stack state by clearing owned collections. |
| `clearstack` | `parser::clearstack` | Preserve semantic distinction from `cleanup_stack` if present; likely reset active stack contents only. |
| `nexttoken` | `parser::nexttoken` | Primary token retrieval API; should consume from putback/inserted stream before upstream source. |
| `putback` | `parser::putback` | Return token to parser-managed lookahead/pushback storage. |
| `init_parse` | `parser::init_parse` | Initialize parser state and token buffers. |

## Data Model

Because the input only exposes anonymous C data structures, the Rust plan should infer a minimal set of named internal types from function behavior and keep them private unless already required by wider project APIs.

| C Construct | Rust Mapping | Purpose |
|---|---|---|
| anonymous parser state struct | `struct ParserState` | Owns token stream state, pushback buffer, marks, and debug-relevant parser context. |
| anonymous token struct | `struct Token` | Stores token kind plus associated text/value/location fields already used by parser logic. |
| anonymous token type constants | `enum TokenType` | Replaces integer/`#define` token kinds with typed discriminants. |
| anonymous mark/savepoint struct | `struct ParseMark` | Snapshot of token index, pushback depth, and any additional parser cursor state needed by `mark`/`restore`. |
| anonymous stack node / token buffer | `Vec<Token>` within `ParserState` | Replaces manually linked or array-managed token stack/buffer. |
| anonymous debug/file location record | `struct SourceLoc` or fields on `Token` | Tracks filename/line/column if used by `print_token` or `file_error`. |
| anonymous parser input handle | field on `ParserState` | Represent current source/input context with owned or borrowed standard-library types already used by the crate. |
| anonymous error signaling state | `enum ParseError` | Captures file/parse failure conditions emitted by `file_error` and related paths. |

### Ownership and Memory Management

- Replace all manual token allocation/free paths with owned `Token` values.
- Use `Vec<Token>` for parser-managed token storage; use indices instead of raw pointers for current position and restore points.
- If C logic keeps borrowed token text, convert to owned `String` unless an existing crate-wide token type already uses borrowed slices safely.
- `cleanup_stack` and `clearstack` should be implemented as collection clears/resets, making cleanup deterministic and safe.

### Error Handling

- Functions that can fail due to file/input state should return `Result<_, ParseError>` where the existing caller contract permits it.
- Pure debug/display helpers should remain infallible.
- If some migrated APIs must preserve side-effect-only behavior, centralize formatting/construction in `file_error` and let outer parser entry points decide whether to propagate or record the error.

## Implementation Phases

### Phase 1: Establish Rust parser state and type mappings

- Create `src/parser.rs` as the migration target for the logic currently in `src/parser.c`.
- Define minimal Rust equivalents for:
  - `Token`
  - `TokenType`
  - `ParserState`
  - `ParseMark`
  - `ParseError`
- Identify the exact fields required by the listed functions from the C implementation and migrate only those fields.
- Implement `init_parse`, `token_type_str`, and the basic debug/token formatting helpers in terms of the new typed structures.
- Keep visibility narrow (`pub(crate)` or private) unless other existing Rust modules require broader access.

### Phase 2: Port token-buffer mutation and parser position logic

- Implement `tokpush`, `tokdel`, `tokins`, `putback`, `mark`, `restore`, `clearstack`, and `cleanup_stack`.
- Model all token-stream edits with `Vec<Token>` plus explicit cursor indices.
- Preserve original ordering semantics for inserted, deleted, and put-back tokens.
- Validate that restore behavior resets both token position and any associated transient stack state expected by the C logic.
- Remove any need for manual cleanup beyond dropping owned collections.

### Phase 3: Port token retrieval and error paths

- Implement `nexttoken` against the Rust parser state, honoring pushback/inserted tokens before reading further input.
- Implement `file_error` using `ParseError` and existing source-location fields.
- Ensure end-of-input and malformed-input behavior follows the C control flow as closely as possible.
- Keep the API shape conservative so existing parser callers can be migrated with minimal change.

### Phase 4: Verification and behavior alignment

- Add focused unit tests for:
  - token type string mapping,
  - token pushback and re-read behavior,
  - insertion/deletion ordering,
  - mark/restore correctness,
  - stack clearing/cleanup behavior,
  - error generation with source location context.
- Compare Rust behavior against representative C execution paths from the listed functions, especially around cursor movement and stack mutation edge cases.
- Perform final cleanup to remove any leftover C-style assumptions that are no longer needed in safe Rust, without changing behavior or widening module scope.