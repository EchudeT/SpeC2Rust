# Implementation Plan: module_src_c.c_21

## Summary

This module is a direct Rust port of the lexer-support portion currently implemented in `src/c.c`. The scope is limited to migrating the existing file-level behavior and exported functions already present in the C source: lexer state accessors/mutators, allocator wrappers, lexer initialization helpers, identifier handling, and preprocessor option handling.

The Rust implementation should preserve the current control flow and state model rather than redesigning it. The recommended approach is to translate the C module into a single Rust module with:
- a concrete lexer-state struct replacing file-static/global C state,
- thin Rust equivalents for the current getter/setter functions,
- standard-library-backed allocation behavior replacing manual `yyalloc`/`yyrealloc`/`yyfree` patterns where possible,
- explicit enums/structs for currently anonymous C data structures,
- fallible parsing/configuration paths expressed with `Result` only where the original C code can fail materially.

The implementation should favor a close migration of existing functions and state transitions so that behavior can be validated against the current C module with minimal expansion of capabilities.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Maintain behavior and throughput comparable to the current C implementation for lexer setup and token initialization paths.
  - Avoid unnecessary heap allocations during repeated identifier and option processing.
  - Preserve constant-time accessor/setter behavior for lexer state functions.
  - Replace manual memory handling with Rust ownership while avoiding extra copying beyond what is needed for safety.

## Module Mapping

### Source File Mapping
- `src/c.c` -> `src/module_src_c_c_21.rs`

If the crate already exposes module declarations from `src/lib.rs` or `src/main.rs`, add only the minimal `mod` and visibility wiring needed for this migrated module.

### Function Mapping

| C Function | Rust Target | Notes |
|---|---|---|
| `yyget_text` | `pub fn yyget_text(state: &LexerState) -> &str` or `Option<&str>` | Return borrowed text from lexer state; use `Option` only if nullability exists in C behavior. |
| `yyset_lineno` | `pub fn yyset_lineno(state: &mut LexerState, line: usize)` | Replace raw integer mutation with checked unsigned line storage if semantics allow. |
| `yyset_in` | `pub fn yyset_in(state: &mut LexerState, input: InputSource)` | Represent current input stream/source as owned Rust state. |
| `yyset_out` | `pub fn yyset_out(state: &mut LexerState, output: OutputSink)` | Preserve current write target semantics without adding asynchronous abstractions. |
| `yyget_debug` | `pub fn yyget_debug(state: &LexerState) -> bool` or integer flag | Match original flag width only if required by callers. |
| `yyset_debug` | `pub fn yyset_debug(state: &mut LexerState, enabled: bool)` | Direct state mutation. |
| `yylex_destroy` | `pub fn yylex_destroy(state: LexerState)` or `reset` method | Rely on `Drop` where destruction is only cleanup. |
| `yyalloc` | internal helper or removed | Replace with standard containers/owned values; keep helper only if call-shape compatibility is needed inside module migration. |
| `yyrealloc` | internal helper or removed | Replace with `Vec`, `String`, or buffer growth logic. |
| `yyfree` | internal helper or removed | Usually unnecessary under ownership; only retain as no-op wrapper if migration staging needs it. |
| `init_tokens` | `pub fn init_tokens(state: &mut LexerState)` | Port token table/state initialization exactly. |
| `init_lex` | `pub fn init_lex(state: &mut LexerState)` | Establish initial lexer state and defaults. |
| `ident` | `pub fn ident(state: &mut LexerState, text: &str) -> Token` or equivalent | Preserve current identifier classification/output behavior. |
| `set_preprocessor` | `pub fn set_preprocessor(state: &mut LexerState, mode: PreprocessorMode)` | Replace raw flags/strings with explicit state fields. |
| `pp_option` | `pub fn pp_option(state: &mut LexerState, option: &str) -> Result<(), PpOptionError>` | Use `Result` only if the C path indicates invalid option handling. |

## Data Model

The source analysis reports only anonymous C data structures. In the Rust port, these should be named according to their actual role in `src/c.c`, with the minimum set of types required to preserve existing behavior.

### Data Structure Mapping

| C Data Structure | Rust Type | Migration Decision |
|---|---|---|
| anonymous lexer-global/state struct | `struct LexerState` | Centralize mutable lexer fields currently stored globally or indirectly. |
| anonymous token/config record | `struct TokenEntry` | Use named fields matching current token initialization data. |
| anonymous lexer buffer/input record | `struct InputSource` or enum | Use enum if C currently switches between file-like and in-memory input modes. |
| anonymous output record | `struct OutputSink` or enum | Keep only the variants evidenced by current setters. |
| anonymous preprocessor config record | `struct PreprocessorConfig` | Store preprocessor mode/options without adding unsupported flags. |
| anonymous option/flag grouping | `struct LexerFlags` or direct fields on `LexerState` | Prefer direct fields unless grouping clearly matches C layout. |
| anonymous token kind constants | `enum TokenKind` | Use only if token categories are explicit in C logic; otherwise retain integral constants in a confined form. |
| remaining anonymous helper structs | named private structs | Introduce one Rust struct per actual logical record found during translation; do not invent new abstraction layers. |

### Ownership and Memory Mapping

- Replace raw pointers to text buffers with `String` or `Vec<u8>` depending on whether the C code treats content as text or arbitrary bytes.
- Replace nullable pointers with `Option<T>`.
- Replace manual heap allocation patterns with owned Rust fields and standard collections.
- If temporary reallocation semantics are used for lexer buffers, map them to `Vec` capacity growth.
- Destruction previously handled by `yylex_destroy` should become normal Rust drop behavior, with an explicit `reset`/destroy function only if required by external call sites.

### Error Handling Mapping

- Functions that are pure setters/getters should remain infallible.
- Initialization functions should return `Result` only when the C code currently detects configuration/setup failure.
- Preprocessor option parsing may use a small dedicated error enum if invalid options are part of the current behavior.
- Avoid introducing broad custom error frameworks.

## Implementation Phases

## Phase 1: Establish module skeleton and state translation

### Goal
Create the Rust module and translate the C module's shared state into explicit Rust data types.

### Tasks
- Add `src/module_src_c_c_21.rs`.
- Inspect `src/c.c` and identify all file-scope variables and anonymous structs used by the listed functions.
- Define `LexerState` as the primary replacement for mutable lexer/global state.
- Define minimal named Rust structs/enums for each anonymous C data structure actually referenced by the migrated functions.
- Add field types using:
  - `String`/`Vec<u8>` for owned buffers,
  - `Option<T>` for nullable references,
  - `bool`/`usize` or exact-width integers only where C semantics require them.
- Wire the module into the crate with minimal public exposure needed by current callers.

### Deliverables
- Compiling Rust module with type definitions only.
- Clear one-to-one correspondence between C state holders and Rust state fields.

### Exit Criteria
- All state previously implicit in `src/c.c` is represented explicitly in Rust types.
- No placeholder types remain for structures needed by listed functions.

## Phase 2: Port initialization, accessors, and cleanup paths

### Goal
Migrate simple state-management functions first to stabilize the module interface.

### Tasks
- Port `init_lex`.
- Port `init_tokens`.
- Port `yyget_text`.
- Port `yyset_lineno`.
- Port `yyset_in`.
- Port `yyset_out`.
- Port `yyget_debug`.
- Port `yyset_debug`.
- Port `yylex_destroy`.
- Remove or internalize `yyalloc`, `yyrealloc`, and `yyfree` by replacing their usage with Rust ownership/container operations.
- If migration staging requires allocator-named functions, keep them private and narrowly typed rather than exposing pointer-style APIs.

### Testing
- Add unit tests covering:
  - default initialized lexer state,
  - line number mutation,
  - debug flag get/set behavior,
  - input/output target replacement,
  - cleanup/reset leaving no invalid state behind.

### Exit Criteria
- State access/update functions compile and pass tests.
- Manual allocation wrappers are no longer needed for normal module behavior, or are confined to private compatibility helpers.

## Phase 3: Port identifier and preprocessor logic

### Goal
Translate the remaining behavior-bearing functions while preserving current parsing/configuration semantics.

### Tasks
- Port `ident` using the Rust token/state types established earlier.
- Port `set_preprocessor`.
- Port `pp_option`.
- Preserve current branching and token/option classification logic from the C implementation.
- Introduce a small local error enum only if `pp_option` has failure cases in the C source.
- Ensure string handling matches C behavior for comparisons, normalization, and storage lifetimes.

### Testing
- Add focused unit tests for:
  - identifier recognition/classification,
  - preprocessor mode changes,
  - accepted/rejected preprocessor options based on current C behavior.

### Exit Criteria
- All listed functions are implemented in Rust.
- Identifier and preprocessor behaviors match the existing module logic without added features.

## Phase 4: Behavioral verification and C parity cleanup

### Goal
Finish the migration by checking parity, simplifying transitional code, and locking down the final Rust surface.

### Tasks
- Compare each listed C function against the Rust implementation for control-flow parity.
- Remove remaining translation-only compatibility code that is no longer used.
- Normalize signatures so the module exposes only the Rust interfaces actually required by the project.
- Verify that destruction, buffer ownership, and option handling do not rely on unsafe code unless direct memory-layout constraints are discovered in `src/c.c`.
- Run `cargo test` and fix any state-lifecycle mismatches.

### Deliverables
- Final Rust module replacing the targeted C functionality.
- Unit tests covering the migrated public behavior.

### Exit Criteria
- The Rust module fully covers the functionality of `module_src_c.c_21`.
- Memory management is fully owned by Rust types.
- No extra modules or unsupported abstractions were introduced during migration.