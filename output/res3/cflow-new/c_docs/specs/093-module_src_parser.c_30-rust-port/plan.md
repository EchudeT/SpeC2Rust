# Implementation Plan: module_src_parser.c_30

## Summary

Port `src/parser.c` into a Rust parser module that preserves the current parsing flow, token buffering behavior, declaration/function discrimination, and balanced-skipping logic without adding new parsing capabilities. The Rust implementation should migrate the existing functions directly into a focused module, keeping the control flow close to the C source so behavioral parity is easier to validate.

The technical approach is to:
- translate parser state held implicitly in C globals or stack-managed buffers into explicit Rust structs,
- represent save/undo token buffering with owned collections from the standard library,
- convert C error/status signaling into `Result`, `Option`, and small internal enums where needed,
- preserve sequential parsing behavior for:
  - token saving and rollback,
  - declaration parsing/skipping,
  - balanced token skipping,
  - function and variable declaration detection,
  - K&R declaration parsing support.

The implementation should remain narrowly scoped to the existing `parser.c` responsibilities and migrate functions in dependency order.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Maintain single-pass parsing behavior equivalent to the C implementation.
  - Avoid unnecessary token cloning beyond what is required for save/undo behavior.
  - Use `Vec`-backed stacks and buffers for predictable linear performance.
  - Keep balanced-skip and declaration-skip operations iterative to avoid avoidable allocation or recursion overhead.

## Module Mapping

### Source File Mapping
- `src/parser.c` → `src/parser.rs`

### Function Mapping
- `save_token` → `parser::Parser::save_token`
- `undo_save_stack` → `parser::Parser::undo_save_stack`
- `finish_save_stack` → `parser::Parser::finish_save_stack`
- `skip_to` → `parser::Parser::skip_to`
- `skip_balanced` → `parser::Parser::skip_balanced`
- `yyparse` → `parser::Parser::parse` or `parser::yyparse` wrapper calling `Parser::parse`
- `is_function` → `parser::Parser::is_function`
- `parse_declaration` → `parser::Parser::parse_declaration`
- `skip_declaration` → `parser::Parser::skip_declaration`
- `expression` → `parser::Parser::expression`
- `parse_function_declaration` → `parser::Parser::parse_function_declaration`
- `fake_struct` → `parser::Parser::fake_struct`
- `parse_variable_declaration` → `parser::Parser::parse_variable_declaration`
- `initializer_list` → `parser::Parser::initializer_list`
- `parse_knr_dcl` → `parser::Parser::parse_knr_dcl`

### Rust Module Shape
Keep the port in a single Rust module file to match the original scope:
- `src/parser.rs`
  - parser state struct
  - internal token/save-stack data types
  - migrated parsing methods
  - minimal public entry point matching current project integration needs

If the crate currently exposes modules through `src/lib.rs` or `src/main.rs`, only add:
- `mod parser;`
- minimal `pub(crate)` visibility required by current call sites

## Data Model

Because the analysis only exposes anonymous C data structures, the Rust plan should introduce narrowly scoped named types based on observed parser roles rather than inventing new abstractions.

### Core State Mapping

| C construct | Rust mapping | Notes |
|---|---|---|
| implicit parser state / file-scope mutable state | `struct Parser` | Centralizes parser cursor, token stream access, save stack, and parse context. |
| anonymous token-save stack records | `struct SavedToken` | Owns the token data needed for rollback/replay. |
| anonymous nested save stack / save frame | `struct SaveFrame` | Tracks boundaries for `save_token`, `undo_save_stack`, and `finish_save_stack`. |
| anonymous parse-status flags | `enum ParseStatus` or booleans in `Parser` | Use enum only where multiple states are already present in C logic. |
| anonymous delimiter/balance tracking records | plain counters or `struct BalanceState` | Prefer counters in `skip_balanced` unless the C logic clearly requires a stored composite state. |
| anonymous declaration classification records | `enum DeclarationKind` | For internal discrimination between function, variable, and skipped declaration paths. |
| anonymous expression/declarator temporary records | local structs only if required | Prefer local variables first; only lift to named structs if repeated across migrated functions. |

### Token Representation

If the project already has an existing token type, reuse it directly.
If `parser.c` currently relies on lexer-provided integer/token-code values, map them as:

- C token code integers → Rust `enum TokenKind` if already available in the crate
- otherwise preserve compatibility with `type TokenKind = i32` during the first migration step

Recommended approach:
- keep the parser aligned with the existing lexer interface,
- avoid redesigning token representation during this port,
- store saved tokens in owned form so rollback does not depend on invalid references.

### Parser State Skeleton

A likely Rust shape for the migration is:

```rust
pub(crate) struct Parser {
    save_stack: Vec<SavedToken>,
    save_frames: Vec<SaveFrame>,
    // lexer/token cursor fields reused from current project design
    // parser flags carried over from parser.c
}
```

With supporting types such as:

```rust
struct SavedToken {
    // token kind and any semantic payload required by parser.c
}

struct SaveFrame {
    start: usize,
}
```

### Memory Management Decisions

- Replace C manual stack/list memory handling with `Vec`.
- Use owned token snapshots for rollback-sensitive paths.
- Avoid borrowing lexer output across parser mutations unless the existing lexer API already guarantees stable storage.
- Keep temporary parse data local where possible to reduce lifetime complexity.
- Do not mirror C pointer ownership patterns directly; convert them into explicit Rust ownership at module boundaries.

### Error Handling Decisions

- Map parser success/failure returns to `Result<_, ParseError>` where the caller needs explicit failure information.
- Use `Option` for lookups or classification checks that naturally may not yield a value.
- If the original `yyparse` returns an integer status, keep a thin compatibility layer that translates:
  - success → `0`
  - failure → non-zero
- Preserve existing control-flow distinctions between recoverable skipping and hard parse failure, but do not add generalized recovery beyond the C logic.

## Implementation Phases

## Phase 1: Establish Rust Parser Module and State Containers

### Goals
Create the Rust file and core state needed to host direct function ports from `parser.c`.

### Tasks
- Add `src/parser.rs`.
- Define `Parser` with:
  - token access/cursor fields needed by the existing parser flow,
  - save stack storage,
  - frame tracking for nested save/undo operations,
  - parser flags migrated from C statics/globals.
- Introduce minimal internal supporting types:
  - `SavedToken`
  - `SaveFrame`
  - internal status/classification enums only where required
- Add a narrow parse entry point corresponding to `yyparse`.
- Preserve existing project-facing signatures with minimal adaptation wrappers if needed.

### Completion Criteria
- The module builds with placeholder or partial method bodies.
- Parser state formerly spread across C globals is explicitly represented in Rust.
- No extra modules or infrastructure are introduced beyond the direct port target.

## Phase 2: Port Token Save/Skip and Balance Control Flow

### Goals
Migrate the parser mechanics that other parsing routines depend on.

### Tasks
- Port:
  - `save_token`
  - `undo_save_stack`
  - `finish_save_stack`
  - `skip_to`
  - `skip_balanced`
- Implement save-frame semantics using `Vec` indices rather than manual memory bookkeeping.
- Keep token replay order identical to the C implementation.
- Ensure delimiter balancing logic remains iterative and matches original stop conditions.
- Add unit tests for:
  - nested save/undo sequences,
  - finishing save frames without rollback,
  - skipping to a target token,
  - skipping balanced pairs across nested delimiters.

### Completion Criteria
- Core token buffering and rollback behavior is functional.
- Balanced skipping behavior is verified through focused tests.
- No parser-specific declaration logic has been rewritten yet beyond what these functions require.

## Phase 3: Port Declaration and Expression Parsing Routines

### Goals
Translate the main declaration-analysis logic in dependency order while preserving current branch behavior.

### Tasks
- Port:
  - `expression`
  - `is_function`
  - `parse_declaration`
  - `skip_declaration`
  - `parse_function_declaration`
  - `fake_struct`
  - `parse_variable_declaration`
  - `initializer_list`
  - `parse_knr_dcl`
- Migrate in bottom-up order where helpers are completed before callers.
- Preserve the existing distinction between:
  - function declaration parsing,
  - variable declaration parsing,
  - declaration skipping,
  - special-case K&R declarator handling.
- Reuse the save/undo mechanisms from Phase 2 instead of redesigning parse lookahead.
- Keep expression parsing limited to the scope used by this module; do not generalize it beyond current call patterns.

### Completion Criteria
- All parser helper routines from `parser.c` exist in Rust.
- Function-vs-variable detection behavior is represented in Rust with equivalent control flow.
- The module compiles with the full declaration parsing path connected.

## Phase 4: Integrate `yyparse` Flow and Validate Behavioral Parity

### Goals
Complete the top-level parse routine and verify the port against expected parser behaviors.

### Tasks
- Finish the Rust implementation of `yyparse` as the top-level orchestration method.
- Wire all migrated helper methods into the same call sequence used by `parser.c`.
- Add targeted tests covering:
  - function declaration input paths,
  - variable declaration input paths,
  - skipped/unsupported declaration paths as currently handled,
  - balanced initializer/declarator cases,
  - K&R declaration cases if they are part of current parser expectations.
- Add compatibility checks for returned parse status codes if external callers still expect C-style status values.
- Review ownership/cloning in saved-token paths and remove unnecessary copies where behavior allows.

### Completion Criteria
- The Rust parser module passes `cargo test`.
- The top-level parse path is fully migrated from `parser.c`.
- Behavior matches current module expectations without adding new parser features.