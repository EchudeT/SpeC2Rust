# Implementation Plan

## Summary

Port `src/c.c` into a single Rust module that preserves the existing lexer-facing API surface and initialization flow without adding new behavior. The Rust implementation should focus on migrating the current scanner state accessors, memory helpers, token/lexer initialization routines, identifier handling, and preprocessor option handling into idiomatic Rust while keeping execution order and state transitions close to the C source.

The technical approach is to replace C global/scanner state manipulation with an explicit Rust state structure owned within the module, map raw allocation helpers to safe owned buffers where possible, and isolate any unavoidable low-level behavior behind small internal functions. The implementation should keep the port constrained to the existing file and function set, with direct function-for-function migration order and tests centered on observable state updates and lifecycle behavior.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Maintain behavior comparable to the current C implementation for scanner state access and token initialization.
  - Avoid unnecessary heap allocations during repeated lexer state updates.
  - Preserve linear-time processing for identifier and preprocessor option handling.
  - Keep memory ownership explicit so cleanup work done by `yylex_destroy` remains predictable and cheap.

## Module Mapping

### Source File Mapping

- `src/c.c` -> `src/c.rs`

### Function Mapping

- `yyget_text` -> `pub(crate) fn yyget_text(...) -> ...`
- `yyset_lineno` -> `pub(crate) fn yyset_lineno(...)`
- `yyset_in` -> `pub(crate) fn yyset_in(...)`
- `yyset_out` -> `pub(crate) fn yyset_out(...)`
- `yyget_debug` -> `pub(crate) fn yyget_debug(...) -> ...`
- `yyset_debug` -> `pub(crate) fn yyset_debug(...)`
- `yylex_destroy` -> `pub(crate) fn yylex_destroy(...) -> ...`
- `yyalloc` -> internal allocation helper in Rust
- `yyrealloc` -> internal reallocation helper in Rust
- `yyfree` -> internal deallocation helper in Rust
- `init_tokens` -> `pub(crate) fn init_tokens(...)`
- `init_lex` -> `pub(crate) fn init_lex(...)`
- `ident` -> `pub(crate) fn ident(...) -> ...`
- `set_preprocessor` -> `pub(crate) fn set_preprocessor(...)`
- `pp_option` -> `pub(crate) fn pp_option(...) -> ...`

### Rust Module Scope

Keep all migrated items in one Rust source file matching the original module scope. Only introduce private helper types/functions required to represent scanner state and memory ownership that were implicit in the C implementation.

## Data Model

Because the input only exposes anonymous C structures, the port should first identify each concrete struct/typedef usage in `src/c.c` and then assign stable Rust names based on role rather than preserving anonymity. The mapping should stay local to this module.

### Data Structure Mapping Strategy

- `anonymous` scanner/lexer state struct -> `struct LexerState`
  - Holds text buffer/view, line number, debug flag, input handle state, output handle state, and any token tables initialized by this file.
- `anonymous` token metadata struct -> `struct TokenEntry`
  - Represents token definitions populated by `init_tokens`.
- `anonymous` preprocessor configuration struct -> `struct PreprocessorConfig`
  - Stores flags/options manipulated by `set_preprocessor` and `pp_option`.
- `anonymous` identifier-related record -> `struct IdentifierState`
  - Holds temporary or persistent state needed by `ident`.
- Remaining `anonymous` structs -> dedicated private Rust structs named by actual usage in the C file during implementation
  - Example pattern: `LexBuffer`, `IoState`, `DebugState`, `TokenTable`, only if each corresponds to a distinct C aggregate already present.

### Primitive and Pointer Mapping

- `char *` text buffer -> `String`, `Vec<u8>`, or `&str` depending on mutability and ownership in the original code
- nullable text pointer returned by scanner -> `Option<String>` or borrowed `Option<&str>` if lifetime-safe
- `int` line/debug fields -> `i32` or `usize`/`bool` where semantics are clear from usage
- file pointers used by `yyset_in` / `yyset_out` -> internal handles represented without exposing C `FILE *`; prefer stored reader/writer state abstractions local to the module
- raw allocated memory from `yyalloc`/`yyrealloc`/`yyfree` -> owned buffers (`Vec<u8>`, `Box<[u8]>`) where call sites allow direct replacement

### Memory Management Decisions

- Eliminate manual free paths where Rust ownership can model the same lifecycle directly.
- Retain dedicated helper functions named after `yyalloc`, `yyrealloc`, and `yyfree` only if existing call structure depends on them; internally they should translate to safe buffer creation/resizing and no-op deallocation-by-drop patterns where possible.
- `yylex_destroy` should become explicit state teardown/reset, not unsafe memory reclamation, except for isolated internal buffers if required by the translated design.

### Error Handling Decisions

- Preserve simple C-style success/failure returns where the original functions encode status that callers expect.
- Use `Result` internally for fallible allocation or input/output state setup, then convert at the module boundary only if needed to match existing calling conventions.
- Avoid panic-based control flow for malformed options or missing lexer state.

## Implementation Phases

## Phase 1: Establish Rust Module Skeleton and State Types

- Create `src/c.rs` as the direct port target for `src/c.c`.
- Read `src/c.c` and enumerate each anonymous struct by usage site before coding; assign one Rust type per actual distinct aggregate.
- Define the core `LexerState` and related private structs needed to represent:
  - current text
  - line number
  - debug flag
  - input/output attachment state
  - token/preprocessor state
- Add function stubs for all listed C functions with signatures shaped around the discovered call sites.
- Decide per function whether the Rust boundary should accept mutable state references, owned buffers, or simple scalar values, keeping signatures minimal and local to the current module migration.

## Phase 2: Port Scanner State Accessors and Memory Lifecycle

- Implement:
  - `yyget_text`
  - `yyset_lineno`
  - `yyset_in`
  - `yyset_out`
  - `yyget_debug`
  - `yyset_debug`
  - `yylex_destroy`
- Translate global or hidden scanner state access from C into explicit field access on `LexerState`.
- Implement internal allocation helpers corresponding to:
  - `yyalloc`
  - `yyrealloc`
  - `yyfree`
- Replace raw allocation patterns at call sites with Rust-owned buffers wherever the original logic does not require byte-for-byte allocator semantics.
- Add unit tests for:
  - line number updates
  - debug flag round-trip
  - text retrieval behavior
  - input/output state replacement
  - destroy/reset behavior

## Phase 3: Port Initialization Logic and Token Setup

- Implement `init_tokens` using Rust collections sized to the original token table usage.
- Implement `init_lex` so initialization order matches the C module and leaves the lexer state ready for identifier/preprocessor handling.
- Preserve any one-time initialization semantics present in the C code, but model them with explicit state fields instead of implicit static mutation when possible.
- Verify that initialization and teardown can run repeatedly without stale state leakage.
- Add tests covering:
  - initial token table contents expected by downstream logic
  - repeated initialization
  - initialization after destroy

## Phase 4: Port Identifier and Preprocessor Handling

- Implement `ident` with close attention to original character handling, token lookup/update behavior, and any interaction with lexer state.
- Implement `set_preprocessor` and `pp_option` using a dedicated configuration struct or fields within `LexerState`, matching the original option mutation flow.
- Preserve current return-value behavior for accepted/rejected options and identifier processing outcomes.
- Add tests covering:
  - representative identifier inputs
  - preprocessor selection changes
  - option parsing/state mutation
  - interaction between initialization and preprocessor configuration

## Phase 5: Conformance Cleanup and Final Validation

- Reconcile any remaining C idioms that survived the first-pass port, especially:
  - nullable state handling
  - integer-to-boolean conversions
  - buffer resizing assumptions
- Minimize `unsafe`; if any is still required after translation, confine it to the smallest internal helper and document the invariant it relies on.
- Ensure the Rust file remains a direct replacement for the current module rather than a redesigned subsystem.
- Run `cargo test` and finalize any signature adjustments needed to integrate the ported module with the rest of the branch.
- Perform a final review that every function from `src/c.c` is migrated or intentionally folded into an equivalent Rust helper with no added capabilities.