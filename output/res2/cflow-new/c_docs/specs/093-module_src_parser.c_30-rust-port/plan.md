# Implementation Plan: module_src_parser.c_30

## Summary

This module migrates `src/parser.c` into an idiomatic Rust parser module that preserves the existing control flow and parsing behavior of the C implementation without introducing new parsing capabilities. The Rust implementation should keep the original single-module scope, translate parser state and save/undo stack handling into explicit owned data structures, and convert pointer-driven token traversal into borrow-checked access over slices or iterators.

The technical approach is to port the existing parser routines largely function-for-function into one Rust source module, keeping the original parsing order and internal helper boundaries:

- token save/restore behavior becomes an explicit stack type
- declaration/function detection remains procedural rather than redesigned
- balanced skipping and declaration skipping remain utility routines over the token stream
- parser-global mutable state in C becomes a parser state struct in Rust
- C error/signaling conventions become `Result`, `Option`, and small internal status enums where needed

This plan avoids broad AST redesign and focuses on migrating the current parser logic into safe Rust with minimal structural expansion.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**:
  - Rust standard library only
- **Testing**:
  - `cargo test`
- **Performance Goals**:
  - Preserve linear token-stream parsing behavior
  - Avoid unnecessary token cloning where borrowing or indexed access is sufficient
  - Keep save/undo stack operations amortized O(1) using `Vec`
  - Maintain comparable behavior to the C implementation for declaration scanning, function detection, and balanced-token skipping

## Module Mapping

### C to Rust File Mapping

- `src/parser.c` → `src/parser.rs`

### Function Mapping

The Rust module should retain close correspondence with the original C routines to reduce migration risk:

- `save_token` → `save_token`
- `undo_save_stack` → `undo_save_stack`
- `finish_save_stack` → `finish_save_stack`
- `skip_to` → `skip_to`
- `skip_balanced` → `skip_balanced`
- `yyparse` → `yyparse`
- `is_function` → `is_function`
- `parse_declaration` → `parse_declaration`
- `skip_declaration` → `skip_declaration`
- `expression` → `expression`
- `parse_function_declaration` → `parse_function_declaration`
- `fake_struct` → `fake_struct`
- `parse_variable_declaration` → `parse_variable_declaration`
- `initializer_list` → `initializer_list`
- `parse_knr_dcl` → `parse_knr_dcl`

### Rust Module Shape

Keep the implementation in a single module file to match the current source layout:

- `src/parser.rs`
  - parser state struct
  - token save-stack types
  - migrated parser functions
  - internal enums/helpers only where required to model C control flow safely

If the crate currently exposes parser entry points elsewhere, re-export only the existing externally used entry point(s) without creating extra layers.

## Data Model

Because the source analysis only reports anonymous C data structures, the Rust mapping should be driven by usage during migration rather than speculative redesign. Anonymous C structs/unions used as parser state or stack nodes should be given narrow, purpose-based Rust names.

### Data-Structure Mapping Strategy

- **C anonymous parser-global state** → `ParserState`
  - Holds mutable parser position, saved-token stack, and any status flags previously stored in globals or file-local statics.
- **C anonymous saved-token record** → `SavedToken`
  - Represents one saved token and any metadata needed for undo/finish semantics.
- **C anonymous save-stack container** → `SaveStack`
  - Backed by `Vec<SavedToken>`.
- **C anonymous token cursor / temporary parser snapshot** → `ParserCheckpoint`
  - Used only if the original code snapshots parse position for lookahead or rollback.
- **C anonymous declaration parsing temporary record** → `DeclarationState`
  - Only introduce if a cluster of temporary fields in C is easier to preserve as one struct.
- **C anonymous expression / initializer traversal temporary state** → small local structs or enums only when necessary
- **Other anonymous structs** → named Rust structs/enums based on concrete usage discovered in `src/parser.c`

### Rust Ownership and Representation

- Replace raw pointer traversal with:
  - `usize` indices into a token slice, or
  - mutable references to a parser state struct carrying the current index
- Replace linked save stacks or manual allocation with `Vec`
- Replace nullable pointers with `Option<T>` / `Option<usize>` / `Option<&T>`
- Replace integer success/failure returns with:
  - `bool` for simple predicate behavior
  - `Option<T>` for optional parse products
  - `Result<T, ParseError>` where the C code can fail in a way that should stop parsing

### Error Handling Mapping

Use restrained error modeling:

- `yyparse` and major parse entry points should return `Result<..., ParseError>` if the C routine has explicit failure paths
- Internal scanning helpers that only advance or detect shape should stay as `bool` or `Option`
- `ParseError` should be a compact enum containing only actual parser failure modes observed in the C code during migration

### Memory Management Notes

- No manual allocation/free translation should remain in Rust
- Saved tokens and temporary parser state should be owned by `ParserState`
- Any C logic relying on object lifetime through static/global storage should be moved into struct fields with explicit lifetimes or owned copies as appropriate
- Prefer borrowing existing token data over duplicating it unless save/undo semantics require ownership

## Implementation Phases

## Phase 1: Establish Rust parser skeleton and state model

### Goals
- Create `src/parser.rs`
- Define the parser state container and minimal supporting types
- Port save/restore mechanics before higher-level parse routines

### Tasks
- Introduce `ParserState` holding:
  - token source reference or owned token sequence, matching the surrounding crate design
  - current cursor/index
  - saved-token stack
  - parser flags formerly represented by C globals/statics
- Define `SavedToken`, `SaveStack`, and `ParserCheckpoint` only if directly needed by migrated code
- Port:
  - `save_token`
  - `undo_save_stack`
  - `finish_save_stack`
- Preserve original stack semantics exactly, especially rollback behavior and token ownership expectations
- Add unit tests for:
  - save then undo restoring the expected parser position/state
  - save then finish committing the consumed state
  - nested save-stack behavior if present in C

### Exit Criteria
- Save-stack logic compiles and behaves deterministically under `cargo test`
- No raw-pointer-style state mutation remains in this portion of the code

## Phase 2: Port token skipping and parser-shape detection helpers

### Goals
- Migrate low-level traversal helpers that higher parse routines depend on
- Keep lookahead and balanced-delimiter logic close to the C implementation

### Tasks
- Port:
  - `skip_to`
  - `skip_balanced`
  - `is_function`
  - `skip_declaration`
- Implement delimiter tracking with explicit counters / small enums rather than recursive unsafe traversal
- Preserve C behavior around nested parentheses/braces/brackets and early termination conditions
- Use `usize` cursor movement and bounds-checked token access
- Add focused tests for:
  - skipping to target tokens
  - balanced skipping across nested delimiters
  - declaration-vs-function detection on representative token sequences
  - skip behavior at end-of-input and malformed nesting, based on observed C behavior

### Exit Criteria
- Core scanning helpers are available and validated
- Function/declaration discrimination can be used by higher-level parse routines without placeholder logic

## Phase 3: Port declaration and expression parsing routines

### Goals
- Migrate the main parser routines that operate on declarations, initializers, and old-style parameter declarations
- Keep internal control flow aligned with the C source

### Tasks
- Port:
  - `parse_declaration`
  - `expression`
  - `parse_function_declaration`
  - `fake_struct`
  - `parse_variable_declaration`
  - `initializer_list`
  - `parse_knr_dcl`
- Where C uses anonymous temporaries across several branches, introduce minimal Rust structs/enums only to preserve state clearly
- Convert mutation-heavy branches into methods on `ParserState` when that reduces borrow conflicts, but do not split into additional modules
- Preserve parser side effects and ordering assumptions, especially around declaration parsing and initializer consumption
- Add tests covering:
  - variable declaration parsing
  - function declaration parsing
  - initializer list traversal
  - K&R declaration handling if still exercised by the existing project inputs
  - synthetic struct handling via `fake_struct`

### Exit Criteria
- Declaration-related routines compile and execute through representative parse scenarios
- No placeholder `todo!()` remains in migrated declaration/expression paths

## Phase 4: Port top-level parse entry point and finalize integration

### Goals
- Complete the migration with the parser entry point and connect all helper routines
- Ensure behavior remains aligned with the original module boundaries

### Tasks
- Port `yyparse` as the Rust top-level parser entry
- Wire existing helper routines into the same call order used by the C implementation
- Replace remaining C-style status propagation with restrained `Result`/`Option` handling
- Confirm all parser state is encapsulated in Rust-owned structures rather than file-global mutable state
- Add integration-style tests that exercise end-to-end parsing paths through `yyparse`
- Remove or fold any migration-only scaffolding that is no longer needed once all functions are connected

### Exit Criteria
- `src/parser.rs` fully replaces the `src/parser.c` logic for this module
- `cargo test` passes for unit and integration coverage added during migration
- The implementation remains single-module in scope and does not introduce non-required facilities