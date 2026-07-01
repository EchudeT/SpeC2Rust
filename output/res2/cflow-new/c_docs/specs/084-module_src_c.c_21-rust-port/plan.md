# Implementation Plan

## Summary

Port `src/c.c` into a single Rust module that preserves the existing lexer-support and preprocessing-related behavior exposed by the current C functions. The Rust implementation should stay narrowly aligned with the current file and function surface: scanner state accessors/mutators, allocator-style helpers, lexer initialization, identifier handling, and preprocessor option/state control.

The technical approach is to replace the implicit C global/stateful scanner model with an explicit Rust-owned module state structure, while keeping the migrated function boundaries recognizable. Memory-management helpers from C should be converted into safe Rust allocation patterns internally, with small compatibility-style functions only where needed by the migrated code. Error-prone nullable pointers, file handles, and mutable global state should be represented with `Option`, owned buffers, and explicit mutable state fields.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Maintain behavior comparable to the C implementation for token/text access and lexer state updates.
  - Avoid unnecessary string copying in scanner text access where borrowing is sufficient.
  - Keep allocation behavior simple and bounded to migrated logic; no additional abstraction layers beyond what is needed for safety.
  - Preserve linear-time behavior for token/identifier preprocessing paths.

## Module Mapping

### C to Rust File Mapping

- `src/c.c` → `src/module_src_c_c_21.rs`

### Rust Module Scope

Implement the migrated logic in a single Rust source file to match the source-module boundary and avoid inventing extra architecture.

### Function Mapping

| C Function | Rust Target | Notes |
|---|---|---|
| `yyget_text` | `pub(crate) fn yyget_text(state: &LexerState) -> &str` | Return borrowed current token text from scanner state. |
| `yyset_lineno` | `pub(crate) fn yyset_lineno(state: &mut LexerState, line: usize)` | Store line number in explicit state. |
| `yyset_in` | `pub(crate) fn yyset_in(state: &mut LexerState, input: InputSource)` | Replace C file-pointer style input with owned input source handle/state. |
| `yyset_out` | `pub(crate) fn yyset_out(state: &mut LexerState, output: OutputSink)` | Replace mutable output target with explicit state field. |
| `yyget_debug` | `pub(crate) fn yyget_debug(state: &LexerState) -> bool` | Map integer debug flag to `bool` unless exact numeric semantics are required by migrated code. |
| `yyset_debug` | `pub(crate) fn yyset_debug(state: &mut LexerState, enabled: bool)` | Mutate debug flag in state. |
| `yylex_destroy` | `pub(crate) fn yylex_destroy(state: &mut LexerState)` | Clear owned buffers/state; rely on Rust drop semantics. |
| `yyalloc` | internal helper or removed | Prefer direct `Vec`/`String` allocation; keep helper only if needed to preserve local migration shape. |
| `yyrealloc` | internal helper or removed | Replace with `Vec::resize`, capacity growth, or buffer replacement. |
| `yyfree` | internal helper or removed | Usually unnecessary under ownership; keep only as a no-op wrapper if it simplifies staged migration. |
| `init_tokens` | `pub(crate) fn init_tokens(state: &mut LexerState)` | Initialize token tables/sets in Rust-owned collections. |
| `init_lex` | `pub(crate) fn init_lex(state: &mut LexerState)` | Initialize scanner fields and preprocessing-related defaults. |
| `ident` | `pub(crate) fn ident(state: &mut LexerState, text: &str) -> TokenKind` | Represent identifier classification/handling directly in Rust. |
| `set_preprocessor` | `pub(crate) fn set_preprocessor(state: &mut LexerState, mode: PreprocessorMode)` | Replace C flags/string state with explicit enum/fields. |
| `pp_option` | `pub(crate) fn pp_option(state: &mut LexerState, option: &str) -> Result<(), PpOptionError>` | Parse/apply preprocessing option with explicit error result. |

## Data Model

Because the source analysis only exposes anonymous C data structures, the Rust plan should reconstruct only the minimum named state needed to migrate the listed functions.

### Data Structure Mapping

| C Structure | Rust Type | Mapping Decision |
|---|---|---|
| anonymous scanner state | `struct LexerState` | Consolidate scanner text, line number, debug flag, token state, input/output handles, and preprocessing fields. |
| anonymous current token/text buffer | `String` or `Vec<u8>` field in `LexerState` | Use `String` if the C code treats token text as textual; use `Vec<u8>` only if byte-preserving behavior is required by the file contents. |
| anonymous token initialization tables | `Vec<TokenEntry>` or static slice | Keep representation minimal and local to `init_tokens`. |
| anonymous identifier/token metadata | `enum TokenKind` plus optional `TokenEntry` | Use enums for finite token categories rather than integer constants where possible. |
| anonymous preprocessor configuration | `struct PreprocessorState` | Track mode/options explicitly instead of scattered C flags. |
| anonymous input handle | `enum InputSource` | Represent absence/presence and source ownership explicitly. |
| anonymous output handle | `enum OutputSink` | Mirror current usage without introducing generalized I/O frameworks. |
| anonymous debug flag | `bool` | Prefer boolean unless exact integer storage is required by migrated logic. |
| anonymous allocator-managed buffers | owned Rust containers | Replace raw allocation bookkeeping with ownership. |
| remaining anonymous structs | fold into `LexerState` fields unless clearly separable | Do not split into extra modules; name only what is needed to complete the file migration. |

### Proposed Rust Types

```rust
pub(crate) struct LexerState {
    line_no: usize,
    debug: bool,
    current_text: String,
    input: InputSource,
    output: OutputSink,
    tokens: Vec<TokenEntry>,
    preprocessor: PreprocessorState,
}

pub(crate) struct TokenEntry {
    name: String,
    kind: TokenKind,
}

pub(crate) enum TokenKind {
    Identifier,
    Keyword,
    Other,
}

pub(crate) struct PreprocessorState {
    mode: PreprocessorMode,
    options: Vec<String>,
}

pub(crate) enum PreprocessorMode {
    Disabled,
    External,
    Internal,
}

pub(crate) enum InputSource {
    None,
    // concrete variant chosen after inspecting actual C file usage
}

pub(crate) enum OutputSink {
    None,
    // concrete variant chosen after inspecting actual C file usage
}
```

### Memory Management Decisions

- Replace `yyalloc`, `yyrealloc`, and `yyfree` with standard owned Rust containers wherever possible.
- Do not preserve manual free/realloc semantics unless a direct staged translation requires local helper functions.
- Eliminate raw ownership transfer patterns; represent optional resources with `Option<T>` or dedicated enums.
- Let `yylex_destroy` become state reset/cleanup logic rather than explicit deallocation.

### Error Handling Decisions

- Convert invalid preprocessor option handling in `pp_option` into `Result`.
- Keep simple setter functions infallible unless the original logic clearly depends on failure signaling.
- Avoid `unwrap`; propagate parse/application failures through narrow local error enums.
- If `yyset_in` or `yyset_out` can reject invalid handles in the C code, model that with `Result` only when inspection of `src/c.c` confirms it.

## Implementation Phases

## Phase 1: Establish the Rust Module Skeleton and Core State

- Create `src/module_src_c_c_21.rs`.
- Define `LexerState` with fields covering:
  - current text buffer
  - line number
  - debug flag
  - token storage
  - preprocessor state
  - input/output placeholders
- Add initial Rust equivalents for:
  - `yyget_text`
  - `yyset_lineno`
  - `yyget_debug`
  - `yyset_debug`
  - `yylex_destroy`
- Decide concrete representations for input/output based only on actual `src/c.c` usage.
- Add basic unit tests for state getters/setters and destroy/reset behavior.

## Phase 2: Migrate Initialization and Token/Identifier Logic

- Port `init_tokens` into Rust-owned token table initialization.
- Port `init_lex` to initialize all scanner and preprocessing defaults in `LexerState`.
- Port `ident` using Rust string matching and token classification.
- Replace any integer/token macro usage in the migrated code with `enum` or constant-based Rust forms, keeping naming close to the original logic.
- Add tests covering:
  - initialization defaults
  - token table population
  - identifier classification behavior

## Phase 3: Migrate Preprocessor State and Option Handling

- Port `set_preprocessor` into explicit `PreprocessorState` mutation.
- Port `pp_option` with a minimal local error enum for invalid or unsupported option strings, matching the existing C behavior as closely as possible.
- Keep option parsing local to the module; do not introduce command-line parsing crates or broader configuration systems.
- Add tests for:
  - switching preprocessor modes
  - accepted option paths
  - rejected/invalid options

## Phase 4: Eliminate Manual Allocation Patterns and Complete I/O-State Setters

- Port `yyset_in` and `yyset_out` using concrete Rust state representations derived from actual C usage.
- Inline or remove `yyalloc`, `yyrealloc`, and `yyfree` by converting all dependent code to standard Rust containers.
- If transitional helper functions remain necessary, keep them private and narrowly scoped to the migrated module.
- Review all translated code for:
  - removal of raw-pointer assumptions
  - lifetime-safe text access
  - explicit ownership at state boundaries
- Add final regression-style tests covering integrated initialization, state mutation, text access, and teardown.