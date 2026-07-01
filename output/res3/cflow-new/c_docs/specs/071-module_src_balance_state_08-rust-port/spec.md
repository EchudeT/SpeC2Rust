# spec.md

## Title

Functional Specification: `module_src_balance_state_08` Rust Port

## Document Information

- Project: `cflow-new`
- Module: `module_src_balance_state_08`
- Category: `module_cluster`
- Source basis: `src/parser.c`
- Rust branch: `071-module_src_balance_state_08-rust-port`
- Generation date: `2026-06-17`

## Overview

This module provides parser-local balance tracking used while scanning token streams for matching closing parenthesis tokens across nested syntactic structure. Its behavior is centered on:

- storing and restoring parser balance checkpoints as nested state,
- releasing all stored checkpoints when balance tracking ends,
- locating the token index of the closing parenthesis that matches a given opening parenthesis token, subject to a specified nesting level.

The Rust rewrite must preserve the observable behavior of this balance-state handling and matching logic as used by the parser in `src/parser.c`.

## Scope

In scope:

- balance state stack management for nested parser traversal,
- push/pop semantics for stored `(idx, level)` parser state pairs,
- cleanup of accumulated balance state,
- matching of an opening parenthesis to its corresponding closing parenthesis while respecting nesting level.

Out of scope:

- unrelated parser features,
- tokenization behavior not required by closing-parenthesis matching,
- any new public API not evidenced by the source module.

## Feature Specification

### Feature: Nested balance-state tracking

The module must support temporary storage of parser balance state during nested traversal. Each stored state consists of:

- a token position or index (`idx`),
- a nesting level (`level`).

The Rust version must support adding a new state to the top of the stack, removing the most recently added state, and discarding the full stack when no longer needed.

Behaviorally, stack usage is LIFO. A state popped after multiple pushes must return the most recently pushed `(idx, level)` pair.

Traceability:

- `push_balance_state`
- `pop_balance_state`
- `free_balance_stack`
- `struct balance_state`

### Feature: Matching closing parenthesis discovery

The module must support finding the closing parenthesis corresponding to a given opening parenthesis token, using the parser’s notion of nesting level.

The Rust version must preserve the behavior that:

- matching begins from a specified opening token reference,
- nesting level is part of the matching decision,
- nested balanced regions are handled correctly,
- the result is an integer token position identifying the matching closing parenthesis when found.

This feature depends on temporary nested balance tracking during traversal.

Traceability:

- `find_closing_paren`
- `push_balance_state`
- `pop_balance_state`
- `free_balance_stack`
- `struct balance_state`

## User Scenarios & Testing

### Scenario 1: Save and restore nested parser position

A parser routine enters a nested syntactic region and must remember its current token index and nesting level before continuing. Later, it restores the saved state to resume from the prior parsing context.

The Rust version must support:

- storing a state,
- storing another nested state,
- restoring the second state first,
- restoring the first state second.

Suggested tests:

- Push one state and pop it; verify returned `idx` and `level` equal the pushed values.
- Push two or more states and pop them in sequence; verify LIFO ordering.
- After cleanup, no saved states remain.

Traceability:

- `push_balance_state`
- `pop_balance_state`
- `free_balance_stack`

### Scenario 2: Release all temporary balance tracking

A parser routine finishes or aborts a search and must discard all temporary balance checkpoints without individually popping each one.

The Rust version must support full cleanup of all stored balance states.

Suggested tests:

- Push multiple states, invoke full cleanup, and verify stack storage is empty.
- Invoke cleanup on an already empty stack and verify no residual state is created.

Traceability:

- `free_balance_stack`

### Scenario 3: Find the matching closing parenthesis in a simple balanced region

A parser routine has the token position of an opening parenthesis and a current level. It needs the token position of the corresponding closing parenthesis in a region with no additional nested parentheses.

The Rust version must return the correct closing token position for that pair.

Suggested tests:

- Provide a token sequence equivalent to `( ... )` at the specified level and verify the returned index is the closing parenthesis for the original opening token.

Traceability:

- `find_closing_paren`

### Scenario 4: Find the matching closing parenthesis through nested parentheses

A parser routine must find the correct closing parenthesis for an opening parenthesis when additional balanced parentheses appear inside the region.

The Rust version must ignore inner closing parentheses that match inner opens and return the closing parenthesis that matches the original opening token.

Suggested tests:

- Provide a token sequence equivalent to `( ... ( ... ) ... )` and verify that the result is the outer matching closing parenthesis.
- Use multiple nested levels and verify correct matching for both inner and outer opens.

Traceability:

- `find_closing_paren`
- `push_balance_state`
- `pop_balance_state`

### Scenario 5: Matching constrained by parser level

A parser routine searches for a matching close using a supplied nesting level. Tokens outside the relevant level must not be treated as the match for the target opening token.

The Rust version must preserve level-aware matching behavior.

Suggested tests:

- Construct token layouts where the same parenthesis token kinds appear at different levels and verify the result corresponds to the requested level.
- Verify that nested transitions do not cause an incorrect close to be selected for the original opening token.

Traceability:

- `find_closing_paren`
- `struct balance_state`

## Requirements

### Functional Requirements

#### FR-1: Store balance checkpoints

The module shall store a balance checkpoint containing an integer token index and an integer nesting level as a new top-of-stack entry.

Traceability:

- `push_balance_state`
- `struct balance_state`

#### FR-2: Restore the most recent balance checkpoint

The module shall remove and return the most recently stored balance checkpoint, yielding both the saved token index and saved nesting level.

Traceability:

- `pop_balance_state`
- `struct balance_state`

#### FR-3: Preserve LIFO stack semantics

When multiple balance checkpoints are stored, the module shall restore them in reverse order of insertion.

Traceability:

- `push_balance_state`
- `pop_balance_state`
- `struct balance_state`

#### FR-4: Release all stored balance checkpoints

The module shall support releasing the entire balance checkpoint stack so that no stored checkpoint remains active afterward.

Traceability:

- `free_balance_stack`
- `struct balance_state`

#### FR-5: Find a matching closing parenthesis for an opening token

The module shall determine the token index of the closing parenthesis corresponding to a specified opening parenthesis token.

Traceability:

- `find_closing_paren`

#### FR-6: Respect nested balanced structure during matching

While locating a closing parenthesis, the module shall account for nested parenthesis pairs so that inner pairs are matched before the original opening token is considered closed.

Traceability:

- `find_closing_paren`
- `push_balance_state`
- `pop_balance_state`

#### FR-7: Respect supplied nesting level during matching

The module shall use the supplied nesting level as part of closing-parenthesis matching behavior.

Traceability:

- `find_closing_paren`
- `struct balance_state`

#### FR-8: Support cleanup after matching traversal

The module shall support full release of temporary balance checkpoints used during closing-parenthesis discovery.

Traceability:

- `find_closing_paren`
- `free_balance_stack`

### Key Entities

#### `balance_state`

A balance checkpoint record representing parser state needed for nested matching traversal. Based on usage, it contains:

- an `idx` field representing a token position or token index,
- a `level` field representing parser nesting level,
- linkage to another `balance_state` entry to form a stack.

Relationship:

- multiple `balance_state` records form a LIFO stack used by balance-state operations,
- `find_closing_paren` uses this stack to manage nested traversal state.

Traceability:

- `struct balance_state`
- `push_balance_state`
- `pop_balance_state`
- `free_balance_stack`
- `find_closing_paren`

## Success Criteria

### Behavioral correctness

1. Pushing a checkpoint and immediately popping it returns the same `idx` and `level` values that were pushed.
   - Traceability: `push_balance_state`, `pop_balance_state`

2. After pushing `N` checkpoints with distinct values, popping `N` times returns them in exact reverse insertion order.

3. After full cleanup, the balance-state stack contains no remaining checkpoints.
   - Traceability: `free_balance_stack`

4. Given a token sequence with one balanced parenthesis pair and a valid opening token reference, matching returns the index of that pair’s closing parenthesis.
   - Traceability: `find_closing_paren`

5. Given nested balanced parenthesis pairs, matching the outer opening token returns the outer closing token rather than an inner closing token.

6. Given nested balanced parenthesis pairs, matching an inner opening token returns that inner pair’s closing token.

7. Matching behavior remains consistent with the supplied nesting level and does not incorrectly select a closing parenthesis from an unrelated level.

8. Temporary balance-state storage used during closing-parenthesis discovery is fully releasable at the end of the operation.
   - Traceability: `find_closing_paren`, `free_balance_stack`

## Constraints and Notes

- The Rust rewrite must preserve the functional boundaries evidenced by `src/parser.c`.
- This specification does not require exposing these operations as public APIs unless needed by the Rust module design.
- This specification does not add capabilities beyond parser balance-state management and closing-parenthesis matching behavior evidenced in the source.