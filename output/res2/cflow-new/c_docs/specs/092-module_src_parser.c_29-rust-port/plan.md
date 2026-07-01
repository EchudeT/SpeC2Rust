# Implementation Plan: module_src_parser.c_29

## Summary

Port the parser token-management portion of `src/parser.c` into a Rust module that preserves the existing control flow and parsing behavior while replacing manual memory and stack handling with ownership-based Rust structures.

The Rust implementation should focus on migrating the existing token inspection, token stack manipulation, lookahead/putback flow, parser state save/restore, and parser initialization routines represented by:

- `print_token`
- `token_type_str`
- `dbgtok`
- `debugtoken`
- `file_error`
- `mark`
- `restore`
- `tokdel`
- `tokins`
- `tokpush`
- `cleanup_stack`
- `clearstack`
- `nexttoken`
- `putback`
- `init_parse`

The technical approach is to:
- move parser-local state into a dedicated Rust `Parser` struct,
- model token kinds and token records with Rust enums/structs,
- replace raw token linked structures or manual stack allocation with `Vec` or `VecDeque` depending on insertion/removal semantics observed during porting,
- convert implicit global/error-state behavior into explicit `Result` returns where it does not alter caller-visible behavior,
- keep function boundaries and execution order close to the C implementation to reduce migration risk.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Maintain parser throughput comparable to the C implementation for token iteration and pushback-heavy paths.
  - Avoid unnecessary token cloning; prefer move semantics and borrowing.
  - Preserve O(1) or amortized O(1) push/pop behavior for token stack operations.
  - Keep initialization and cleanup costs minimal and proportional to current parser state size only.

## Module Mapping

### C to Rust File Mapping

- `src/parser.c` -> `src/parser.rs`

If the existing Rust crate already separates parser code differently, this port should still land in the parser-focused module already corresponding to `parser.c`, without introducing extra abstraction layers beyond what is needed to hold migrated state.

### Function Mapping

| C Function | Rust Target | Notes |
|---|---|---|
| `print_token` | `fn print_token(...)` or `impl Parser { fn print_token(...) }` | Keep as debug-oriented formatting helper. |
| `token_type_str` | `fn token_type_str(...) -> &'static str` | Prefer enum-based mapping. |
| `dbgtok` | `fn dbgtok(...)` | Preserve current debug output behavior. |
| `debugtoken` | `fn debug_token(...)` | Rust naming may be snake_case while keeping behavior aligned. |
| `file_error` | `fn file_error(...) -> ParseError` or `Result` helper | Convert ad hoc error emission into typed parser error flow where feasible. |
| `mark` | `impl Parser { fn mark(&self) -> ParserMark }` | Snapshot parser position/state. |
| `restore` | `impl Parser { fn restore(&mut self, mark: ParserMark) }` | Rewind parser state to saved mark. |
| `tokdel` | `impl Parser { fn tokdel(...) }` | Remove token from parser-managed storage/stack. |
| `tokins` | `impl Parser { fn tokins(...) }` | Insert token in parser-managed storage/stack. |
| `tokpush` | `impl Parser { fn tokpush(...) }` | Push token onto pending/putback stack. |
| `cleanup_stack` | `impl Parser { fn cleanup_stack(&mut self) }` | Explicitly clear temporary token storage if still needed. |
| `clearstack` | `impl Parser { fn clear_stack(&mut self) }` | Likely thin wrapper around stack clearing. |
| `nexttoken` | `impl Parser { fn next_token(&mut self) -> Result<Token, ParseError> }` | Central token fetch path. |
| `putback` | `impl Parser { fn put_back(&mut self, token: Token) }` | Return token to lookahead buffer. |
| `init_parse` | `impl Parser { fn init_parse(...) -> Self }` or builder/init method | Establish parser state and token buffers. |

## Data Model

Because the analysis only exposes anonymous C data structures, the Rust port should derive concrete names from actual use in `src/parser.c` during migration and keep them local to the parser module unless already shared elsewhere.

### Expected Data Structure Mapping

| C Shape | Rust Shape | Migration Decision |
|---|---|---|
| anonymous token record | `struct Token` | Owns token type, lexeme/text if present, source location if present. |
| anonymous token type constants | `enum TokenType` | Replaces integer/tag-based token categories. |
| anonymous parser state | `struct Parser` | Holds current input state, token pushback stack, markable cursor state, and parse diagnostics context. |
| anonymous mark/snapshot record | `struct ParserMark` | Contains only the minimal parser state needed by `mark`/`restore`. |
| anonymous stack node / token list node | `Vec<Token>` or `VecDeque<Token>` field on `Parser` | Replace manual node allocation; final choice depends on whether C inserts/removes at front or arbitrary positions. |
| anonymous file/source position record | `struct SourcePos` | Use if the C code tracks line/file/column for `file_error` and token debugging. |
| anonymous error payload | `enum ParseError` | Covers file/input/tokenization/parser-state failures used by migrated functions. |

### Rust Ownership and Memory Management

- Replace manual allocation/free in token stack helpers with container-managed ownership.
- Prefer owned `Token` values in parser buffers to avoid aliasing hazards common in the C implementation.
- Use `Option<Token>` only if the C logic relies on sentinel/null token states.
- Use `Clone` sparingly; marks should snapshot indices and lightweight state rather than duplicating full token buffers unless the C logic requires structural rollback of inserted tokens.
- Any cleanup routines from C should become deterministic container clears; no custom destructor logic should be added unless the source data model requires it.

### Error Handling Strategy

- Convert `file_error` into a typed error constructor or emitter used by token-fetching/init paths.
- Use `Result<T, ParseError>` for operations that can fail due to file/input/parser-state issues.
- Preserve non-failing debug helpers as infallible functions.
- Avoid panics for normal parse errors; reserve `debug_assert!` for invariants equivalent to C internal assumptions.

## Implementation Phases

## Phase 1: Establish Rust Parser State and Type Mappings

- Create or extend `src/parser.rs` with Rust equivalents for:
  - token type representation,
  - token record,
  - parser state,
  - parser mark/snapshot,
  - parse error type.
- Inspect `src/parser.c` and assign stable Rust names to each anonymous structure based strictly on current parser usage.
- Implement `token_type_str` first, driven by the Rust `TokenType` enum rather than integer switches where possible.
- Add `init_parse` as the constructor or initialization method for `Parser`.
- Add minimal unit tests covering:
  - parser initialization,
  - token-type string mapping,
  - default empty stack/buffer state.

## Phase 2: Port Token Stack and Save/Restore Mechanics

- Port `tokpush`, `tokins`, `tokdel`, `cleanup_stack`, and `clearstack` using standard containers.
- Port `mark` and `restore` after the stack layout is finalized, ensuring marks capture exactly the mutable parser state used by the C code.
- Keep operation order and side effects close to the C implementation; do not refactor parsing semantics during this phase.
- Add tests covering:
  - push and putback ordering,
  - insert/remove behavior,
  - stack clearing,
  - mark/restore round trips,
  - restoration after intermediate token stack mutations.

## Phase 3: Port Token Flow and Error Paths

- Port `nexttoken` and `putback` using the finalized parser state and token buffer design.
- Port `file_error` into the shared parser error path used by `next_token` and initialization logic.
- Ensure EOF/end-of-input behavior matches the C implementation, including any sentinel token handling.
- Add tests covering:
  - next-token retrieval from normal input flow,
  - putback followed by reread,
  - error propagation with source/file context,
  - edge cases around empty input and repeated pushback.

## Phase 4: Port Debug/Inspection Helpers and Final Parity Pass

- Port `print_token`, `dbgtok`, and `debugtoken` after token formatting and source location fields are stable.
- Align debug text output with C behavior closely enough for existing diagnostics and parser troubleshooting.
- Perform a final pass over `src/parser.c` to verify:
  - all listed functions are migrated,
  - no manual cleanup assumptions remain,
  - no hidden mutable globals from this function group were omitted from `Parser`.
- Add focused tests for debug formatting if output is stable and deterministic; otherwise keep coverage on structural behavior only.

## Notes and Constraints

- Keep the migration scoped to the functionality embodied by `src/parser.c` for this function cluster only.
- Do not introduce new parser layers, trait hierarchies, or generic token frameworks.
- Keep names idiomatic in Rust where harmless, but prioritize direct traceability to the original C functions during review.
- If some functions rely on wider parser globals outside this cluster, fold only the required fields into `Parser` and defer unrelated parser logic to later ports.