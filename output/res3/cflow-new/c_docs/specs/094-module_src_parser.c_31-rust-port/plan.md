# Implementation Plan: module_src_parser.c_31

## Summary

This module migrates `src/parser.c` into a Rust parser module that preserves the existing declaration-parsing and symbol-reference behavior without adding new parser features. The Rust implementation should keep the current procedural parsing flow and function boundaries as close as practical to the C source so that migration risk stays low and behavior remains comparable.

The implementation approach is to:
- port the existing parser routines into a single Rust source module first,
- translate C stateful parsing logic into explicit mutable Rust context/state,
- replace implicit pointer-based ownership with borrowed slices, indices, and owned `String` values only where needed,
- model C parse outcomes and symbol/declaration state with Rust structs/enums,
- use `Result` for parsing failures and internal invariants instead of sentinel/error-prone C return patterns.

The plan should prioritize direct migration of:
- declaration parsing flow,
- identifier and parameter parsing,
- function-body related handling,
- symbol lookup and reference registration.

## Technical Context

### Language / Version
- Rust stable, edition 2021
- Minimum recommended compiler: Rust 1.76+

### Primary Dependencies
- Rust standard library only

No third-party crates are recommended based on the available module evidence. The parser logic appears self-contained and should be ported using:
- `String`
- `&str`
- `Vec<T>`
- `Option<T>`
- `Result<T, E>`
- `HashMap` only if the original symbol lookup behavior already implies table-based access and the surrounding Rust codebase benefits from it; otherwise preserve existing storage shape as closely as possible.

### Testing
- `cargo test`

Testing should focus on:
- function-level unit tests for declaration parsing helpers,
- regression-style tests for typedef, declarator, parameter list, and function-body transitions,
- symbol lookup/reference tests using representative parser inputs.

### Performance Goals
- Maintain behavior and parsing throughput comparable to the C implementation for single-threaded source parsing.
- Avoid unnecessary string cloning during token/declaration handling.
- Use index-based traversal or borrowed token views where possible.
- Keep allocation patterns bounded to parsed identifiers, declaration records, and symbol/reference storage already implied by the original logic.

## Module Mapping

### Source Mapping
- C file: `src/parser.c`
- Rust file: `src/parser.rs`

If the Rust project already uses a nearby parser entry file, place this migration into that existing parser module rather than splitting into additional modules. The goal is a minimal structural change.

### Function Mapping
Map each C function to a Rust function with closely matching responsibility:

- `skip_struct` -> `fn skip_struct(...) -> Result<..., ...>`
- `parse_typedef` -> `fn parse_typedef(...) -> Result<..., ...>`
- `parse_dcl` -> `fn parse_dcl(...) -> Result<..., ...>`
- `dcl` -> `fn dcl(...) -> Result<..., ...>`
- `getident` -> `fn getident(...) -> Option<String>` or `Result<String, ...>`
- `dirdcl` -> `fn dirdcl(...) -> Result<..., ...>`
- `parmdcl` -> `fn parmdcl(...) -> Result<..., ...>`
- `maybe_parm_list` -> `fn maybe_parm_list(...) -> Result<..., ...>`
- `func_body` -> `fn func_body(...) -> Result<..., ...>`
- `get_knr_args` -> `fn get_knr_args(...) -> Result<..., ...>`
- `declare` -> `fn declare(...) -> Result<..., ...>`
- `declare_type` -> `fn declare_type(...) -> Result<..., ...>`
- `get_symbol` -> `fn get_symbol(...) -> Option<...>` or mutable lookup helper
- `add_reference` -> `fn add_reference(...)`
- `call` -> `fn call(...) -> Result<..., ...>` or side-effecting helper

### Integration Mapping
The Rust module should expose only the same parser-facing entry points needed by the current codebase. Helper routines that were file-local in C should become private Rust functions by default.

## Data Model

Because the analysis lists only anonymous C data structures, the Rust data model should be derived from actual field usage during porting rather than invented up front. The migration should preserve each structure’s role and lifecycle with narrow Rust types.

### C Anonymous Structs -> Rust Named Types
Create named Rust types only for structures directly used by these functions, for example:

- anonymous declaration record -> `Declaration`
- anonymous declarator state -> `Declarator`
- anonymous parameter record -> `Parameter`
- anonymous symbol entry -> `Symbol`
- anonymous reference/call site record -> `Reference`
- anonymous parser state/context -> `ParserState`
- anonymous type descriptor -> `TypeSpec`
- anonymous function context -> `FunctionContext`

Final naming should follow the actual responsibilities discovered in `src/parser.c`; do not create extra abstraction layers.

### Recommended Rust Representations

#### Parser state
Any C global or file-static parser state used by these functions should be consolidated into:
```rust
struct ParserState {
    // token stream access, current position, symbol storage, context flags
}
```
Use:
- indices instead of raw cursor pointers where possible,
- `Vec` for owned collections,
- mutable references to pass state through helper functions.

#### Declarator/type information
C declarator/type flags and nested declaration information should become:
```rust
struct Declarator {
    // identifier, pointer depth, arrays/functions modifiers, etc.
}

enum TypeSpec {
    // concrete variants only if the C code clearly distinguishes them
}
```
If the C implementation uses bit flags extensively, start with integer flags in Rust for fidelity, then convert to enums only where this does not alter behavior.

#### Symbols and references
For symbol table and call/reference tracking:
```rust
struct Symbol {
    // name, classification, declaration info, references
}

struct Reference {
    // location and relationship kind
}
```
Use `String` for owned symbol names unless the surrounding lexer/token model already provides stable borrowed lifetimes.

### Memory Management Decisions
- Replace C manual allocation/free with Rust ownership of parser records.
- Prefer borrowing input text/tokens and owning only persisted names/records.
- Avoid self-referential structures.
- Use `Option` for nullable pointers.
- Use `Vec` for expandable parameter/reference lists.

### Error Handling Decisions
- Convert parser failure paths into `Result<T, ParseError>`.
- Use `Option` only for “not found” or “not present” states such as symbol lookup or optional identifier presence.
- Keep error types local to the parser module unless the project already has a shared error type.
- Preserve original control flow distinctions between recoverable optional parses and actual syntax/state errors.

## Implementation Phases

## Phase 1: Establish Rust parser skeleton and state mapping

### Goals
- Create `src/parser.rs`.
- Define the minimal Rust state and core data structures required by the migrated functions.
- Port the lowest-level declarator/token helpers first.

### Tasks
- Inspect `src/parser.c` and identify all file-local state touched by the listed functions.
- Introduce `ParserState` to hold the mutable parser context currently spread across C globals/statics/arguments.
- Define placeholder named Rust structs/enums for the anonymous C structures actually used by this function set.
- Port:
  - `getident`
  - `skip_struct`
  - `get_symbol`
  - `add_reference`
- Keep these helpers private unless externally required.

### Notes
- During this phase, favor direct translation over refactoring.
- If C helper functions depend on integer status codes, translate them first into simple `Result` aliases without redesigning semantics.

## Phase 2: Port declaration and declarator parsing path

### Goals
- Recreate the core declaration parsing pipeline in Rust with behavior close to the C implementation.

### Tasks
- Port:
  - `dcl`
  - `dirdcl`
  - `parse_dcl`
  - `parse_typedef`
  - `parmdcl`
  - `maybe_parm_list`
  - `declare_type`
  - `declare`
- Introduce Rust representations for declaration/type state only as needed by these functions.
- Preserve ordering dependencies and side effects on parser/symbol state.
- Add focused unit tests for:
  - identifier declarators,
  - pointer/function/array declarator combinations as supported by the C code,
  - typedef parsing,
  - parameter list handling including optional-list branches.

### Notes
- Keep the parsing model procedural; do not replace it with parser combinators or a new grammar engine.
- Where the C code mutates shared declaration buffers, use explicit mutable structs passed between functions.

## Phase 3: Port function parsing and call/reference handling

### Goals
- Complete the migration of function-specific parsing and symbol usage recording.

### Tasks
- Port:
  - `get_knr_args`
  - `func_body`
  - `call`
- Integrate function-body parsing with declaration/symbol handling from earlier phases.
- Ensure K&R argument handling follows the original branch behavior if still present in the C source.
- Add tests covering:
  - function declaration vs. function definition transitions,
  - call-site reference registration,
  - old-style argument parsing where applicable,
  - interaction between local declarations and symbol lookup.

### Notes
- Preserve original symbol table update timing, especially around entering/exiting function scope if present in the module.

## Phase 4: Validation, cleanup, and C-to-Rust parity review

### Goals
- Finish migration with behavior checks and minimal cleanup required for maintainable Rust integration.

### Tasks
- Compare each migrated function against `src/parser.c` for branch parity and side effects.
- Remove any temporary translation scaffolding no longer needed.
- Normalize visibility so only necessary parser entry points are public.
- Add regression tests for representative end-to-end parser inputs that exercise:
  - typedef parsing,
  - nested declarators,
  - parameter parsing,
  - function body parsing,
  - symbol reference/call registration.
- Run `cargo test` and resolve borrow/ownership simplifications without changing behavior.

### Exit Criteria
- All listed functions are migrated into Rust.
- The Rust parser module replaces the C module’s covered responsibilities for this file.
- Tests validate declaration parsing and symbol/reference handling behavior at the module level.