# Implementation Plan: module_src_balance_state_08

## Summary

This module ports a small parser-support cluster from `src/parser.c` into Rust, focused on balance-state stack handling and closing-parenthesis search. The Rust implementation should preserve the existing control flow and parser-oriented behavior with minimal redesign.

The technical approach is to translate the C stack-oriented state management into owned Rust data structures using `Vec` as the backing store for push/pop/free behavior, and to express the parenthesis scan as a bounded string/byte traversal routine. The implementation should stay close to the original function boundaries:

- `push_balance_state`
- `pop_balance_state`
- `free_balance_stack`
- `find_closing_paren`

Memory cleanup that is explicit in C should become automatic through Rust ownership and `Drop`, while APIs should still reflect the original mutation order and failure conditions where relevant. The migration should prioritize keeping behavior stable rather than introducing new abstractions.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve linear-time scanning for closing-parenthesis search.
  - Preserve constant-amortized-time push/pop operations for balance-state storage.
  - Avoid unnecessary string copying; prefer borrowing slices or scanning byte views where possible.
  - Match the C module’s practical parser-support performance without adding allocation-heavy layers.

## Module Mapping

### Source Mapping

- **C source file**: `src/parser.c`
- **Rust target module**: `src/parser.rs`

If `src/parser.rs` already exists as part of the broader port, this module’s functions should be added into that file rather than creating additional helper modules unless the existing Rust layout already requires a private submodule for parser internals.

### Function Mapping

- `push_balance_state`
  - Port to a Rust function or private parser-state method that mutates a balance-state stack.
  - Preferred shape: `fn push_balance_state(...)` or `fn push_balance_state(&mut self, ...)`
  - Use `Vec` push semantics to replace manual node allocation or linked-stack manipulation.

- `pop_balance_state`
  - Port to a Rust function or private parser-state method that removes the most recent balance state.
  - Preferred shape: `fn pop_balance_state(...) -> Option<...>` or an equivalent result form that matches caller expectations.
  - Replace manual unlink/free behavior with `Vec::pop`.

- `free_balance_stack`
  - Port as a narrow cleanup/reset routine only if required by call sites.
  - Preferred shape: `fn free_balance_stack(...)` that becomes `clear()` or ownership-based drop.
  - If the Rust owner naturally drops the stack, keep this function only when needed to preserve structure and migration clarity.

- `find_closing_paren`
  - Port as a bounded scan over input text.
  - Preferred shape: `fn find_closing_paren(input: &str, start: usize) -> Option<usize>` or a signature aligned with surrounding parser code.
  - Keep the scanning logic close to the C routine’s balance tracking and stopping rules.

## Data Model

The C analysis lists only anonymous data structures, so the Rust plan should infer structure strictly from the migrated functions and existing parser context in `src/parser.c`. No speculative types should be introduced beyond what those functions require.

### Data-Structure Mapping

Because the original structs are anonymous in the analysis output, the mapping should be performed by role:

- **C anonymous balance-state record**
  - **Rust**: `struct BalanceState`
  - Purpose: store the per-level parser balance data required by push/pop operations.
  - Notes:
    - Fields should be copied only from the data actually read or written by the four target functions.
    - Use concrete integer and flag types (`usize`, `i32`, `bool`, `char`/`u8`) based on actual C usage.
    - Derive only what is needed, likely `Clone`, `Debug`, or `Copy` if the original state is trivially duplicated.

- **C anonymous stack container or linked nodes**
  - **Rust**: `Vec<BalanceState>` inside the owning parser structure, or a dedicated `BalanceStack` wrapper only if the C code already treats it as a distinct object.
    - Prefer `Vec<BalanceState>` over linked structures unless the original behavior strictly depends on node identity.
    - If C used a linked list solely for stack semantics, collapse to `Vec` directly.

- **C parser-owner structure holding stack state**
  - **Rust**: existing parser state struct in `src/parser.rs`
    - Add a field such as `balance_stack: Vec<BalanceState>` only if this state is persistent across calls.
    - Do not split into extra ownership layers.

- **C character pointer / buffer traversal state used by `find_closing_paren`**
  - **Rust**: borrowed `&str`, `&[u8]`, and index variables
    - Use byte indexing if the original parser works on ASCII syntax tokens and byte-level delimiters.
    - Return an index or slice position instead of raw pointers.
    - Validate that parser inputs are treated consistently with the wider Rust port.

### Memory Management Mapping

- Manual allocation in `push_balance_state` becomes `Vec::push`.
- Manual deallocation in `pop_balance_state` / `free_balance_stack` becomes `Vec::pop`, `Vec::clear`, or implicit drop.
- Raw-pointer traversal in `find_closing_paren` becomes indexed borrowing over `&[u8]` or `str` bytes.
- Null checks become `Option` where absence is meaningful.
- Error return codes should be converted only as far as needed to fit surrounding Rust parser APIs; do not redesign broader parser error types in this module plan.

## Implementation Phases

## Phase 1: Extract and map the C state model

- Inspect the implementations of:
  - `push_balance_state`
  - `pop_balance_state`
  - `free_balance_stack`
  - `find_closing_paren`
- Identify the exact anonymous struct fields touched by these functions.
- Identify whether balance state is:
  - embedded in the parser object,
  - represented as a linked stack,
  - or passed around as standalone state.
- Define the minimal Rust equivalents in `src/parser.rs`:
  - `BalanceState`
  - stack field or local stack owner
- Decide final Rust signatures based on the surrounding parser port rather than inventing new standalone interfaces.

**Exit criteria**:
- Every field used by the C functions has a concrete Rust type.
- The ownership location for the balance stack is fixed.

## Phase 2: Port stack operations with ownership-based cleanup

- Implement `push_balance_state` using `Vec<BalanceState>` mutation.
- Implement `pop_balance_state` using `Vec::pop`.
- Implement `free_balance_stack` as:
  - `clear()` if explicit reset is still used by callers, or
  - a no-op removal from public API if lifetime-driven cleanup fully replaces it and call sites can be updated safely.
- Preserve original empty-stack behavior:
  - If the C code tolerated empty pops, map to `Option`.
  - If the C code treated this as a parser-state error, return the narrowest matching error form already used nearby.
- Update call sites in `src/parser.rs` to use the Rust ownership model instead of manual free logic.

**Exit criteria**:
- No manual memory-management patterns remain for this stack logic.
- Push/pop/reset behavior matches the original call paths.

## Phase 3: Port `find_closing_paren` as a bounded scan

- Translate the search logic into Rust using a borrowed input view.
- Mirror the original nesting/balance handling exactly:
  - opening delimiter increments depth,
  - closing delimiter decrements depth,
  - matching close at the initial nesting level returns the target position.
- Preserve any original stopping conditions related to:
  - string termination,
  - nested delimiters,
  - escaped characters or quoted regions, but only if present in the C implementation.
- Use byte scanning if parser syntax is ASCII/token-oriented and the C code compares raw characters.
- Return `Option<usize>` or the parser’s established internal result style.

**Exit criteria**:
- The Rust function reproduces the same found/not-found behavior as the C implementation for representative nested cases.
- No unsafe code is introduced unless a surrounding parser API already requires it.

## Phase 4: Validate behavior with focused tests

- Add unit tests in the parser module’s existing test location covering:
  - push on empty stack
  - multiple push/pop ordering
  - pop on empty stack behavior
  - explicit stack clear/reset behavior if retained
  - simple matching parentheses
  - nested parentheses
  - missing closing parenthesis
  - starting scan near or at invalid positions, if such inputs are possible in current callers
- Keep tests tightly scoped to migrated behavior; do not add parser-wide scenario coverage beyond affected functions.
- Run `cargo test` and resolve any ownership/signature mismatches revealed by integration with existing parser code.

**Exit criteria**:
- Migrated functions compile cleanly in the Rust branch.
- Tests cover the translated stack lifecycle and delimiter scan behavior.