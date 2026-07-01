# spec.md

## Title

Rust Port Functional Specification: `module_src_balance_state_08`

## Document Metadata

- Project: `cflow-new`
- Module: `module_src_balance_state_08`
- Category: `module_cluster`
- Source basis: `src/parser.c`
- Rust branch: `071-module_src_balance_state_08-rust-port`
- Generation date: `2026-06-17`

## Overview

This module provides parser-local balance tracking used while scanning token streams that contain nested paired delimiters. Its role is to maintain a stack of parser positions and nesting levels and to use that state to locate the matching closing parenthesis-like token for a previously observed opening token.

The Rust rewrite must preserve the same functional boundary:

- maintain a last-in, first-out balance-state stack,
- support pushing and popping `(token index, nesting level)` state pairs,
- support releasing all stored balance-state entries,
- scan parser tokens to find the closing delimiter that matches a given opening token at a specified nesting level.

This module is internal parser support logic. No broader parsing features, recovery behavior, or external interfaces are required beyond the behavior evidenced by the listed functions and the `balance_state` structure usage in `src/parser.c`.

## Feature Specification

### Feature: Balance-state stack management

The module must support temporary storage of parser balance state entries, where each entry records:

- a token index or parser position, and
- an associated nesting level.

The stack must behave in LIFO order:

- pushing a state makes it the current top,
- popping returns the most recently pushed state,
- freeing the stack removes all remaining entries.

This functionality exists to support nested delimiter tracking during parser scanning.

Traceability:
- `push_balance_state`
- `pop_balance_state`
- `free_balance_stack`
- `struct balance_state`

### Feature: Matching closing delimiter search

The module must support searching forward from an opening token to determine the corresponding closing parenthesis-like token for the same nesting context.

The search behavior must:

- accept an opening token identifier/index and a nesting level,
- inspect parser token state from that opening position onward,
- respect nesting changes so that inner balanced pairs do not terminate the outer search early,
- return the token index/identifier for the matching closing token when found,
- report failure through its return value when no matching closing token is found.

This search relies on balance-state logic and parser nesting context rather than simple linear first-close matching.

Traceability:
- `find_closing_paren`
- `struct balance_state`

## User Scenarios & Testing

### Scenario 1: Save and restore nested parser state

A parser routine enters a nested construct and needs to remember its current token position and nesting level before scanning deeper.

Expected support:

1. Push the current `(index, level)` state.
2. Perform nested scanning work.
3. Pop the state later.
4. Receive the exact last-pushed `(index, level)` pair.

Test focus:
- single push/pop round trip,
- multiple pushes followed by multiple pops in reverse order.

Traceability:
- `push_balance_state`
- `pop_balance_state`

### Scenario 2: Abandon outstanding saved states

A parser routine finishes or aborts a scan while there are still saved balance states.

Expected support:

1. A stack may contain zero or more saved entries.
2. Releasing the stack clears all stored entries.
3. After release, no prior saved state remains available through the module state.

Test focus:
- freeing an empty stack reference,
- freeing a non-empty stack,
- confirming stack state is reset after release.

Traceability:
- `free_balance_stack`

### Scenario 3: Find the matching close for a simple balanced pair

A parser routine encounters an opening parenthesis-like token in a non-nested context and needs the position of its matching close.

Expected support:

1. Provide the opening token position and current level.
2. Search forward.
3. Return the matching closing token position.

Test focus:
- one opening token followed by its direct matching close at the same level.

Traceability:
- `find_closing_paren`

### Scenario 4: Ignore inner balanced pairs while matching an outer pair

A parser routine encounters nested delimiters and must match the outer opening token, not the first closing token seen.

Expected support:

1. Search begins at the outer opening token.
2. Inner openings increase effective nesting during the scan.
3. Inner closings reduce that inner nesting.
4. The result is the closing token that matches the original outer opening token.

Test focus:
- outer pair containing one inner pair,
- outer pair containing multiple nested levels.

Traceability:
- `find_closing_paren`
- `struct balance_state`

### Scenario 5: Detect absence of a valid matching close

A parser routine scans from an opening token but the token stream ends or becomes unmatched before a proper close is found.

Expected support:

1. The search completes without identifying a valid matching close.
2. The function returns its failure indicator.

Test focus:
- unterminated opening token sequence,
- malformed nesting that prevents a valid same-context close.

Traceability:
- `find_closing_paren`

## Requirements

### Functional Requirements

#### FR-1: LIFO storage of balance states
The module shall store parser balance states as stack entries, where each entry contains an index and a nesting level.

Traceability:
- `push_balance_state`
- `pop_balance_state`
- `struct balance_state`

#### FR-2: Push operation
The module shall allow a caller to add a new balance-state entry to the top of the current stack using a provided index and nesting level.

Traceability:
- `push_balance_state`
- `struct balance_state`

#### FR-3: Pop operation
The module shall allow a caller to remove the current top balance-state entry and obtain the index and nesting level stored in that entry.

Traceability:
- `pop_balance_state`
- `struct balance_state`

#### FR-4: Stack release
The module shall allow a caller to release all balance-state entries currently retained by the stack.

Traceability:
- `free_balance_stack`
- `struct balance_state`

#### FR-5: Closing-delimiter search
The module shall support searching for the matching closing parenthesis-like token corresponding to a specified opening token and parser level.

Traceability:
- `find_closing_paren`

#### FR-6: Nested-balance-aware matching
The closing-delimiter search shall account for nested balanced constructs so that a closing token inside a nested inner construct is not returned as the match for the original opening token.

Traceability:
- `find_closing_paren`
- `struct balance_state`

#### FR-7: Failure reporting for unmatched opens
If no valid closing token is found for the specified opening token and level, the search shall return a failure result.

Traceability:
- `find_closing_paren`

### Key Entities

#### `balance_state`
A stack entry representing saved parser balance context.

Required content evidenced by usage:
- token index / parser position,
- nesting level,
- link to the next saved state entry in stack order.

Relationship:
- forms a singly linked LIFO stack used by parser support routines.

Traceability:
- `struct balance_state`
- `push_balance_state`
- `pop_balance_state`
- `free_balance_stack`

#### Parser token position
A numeric token identifier or index used to refer to a location in the parser's token stream.

Relationship:
- stored in `balance_state`,
- used as input to matching-close search,
- returned by the matching-close search on success.

Traceability:
- `push_balance_state`
- `pop_balance_state`
- `find_closing_paren`

#### Parser nesting level
A numeric nesting indicator representing the parser's current balance depth or related scan context.

Relationship:
- stored in `balance_state`,
- supplied to matching-close search to constrain matching behavior.

Traceability:
- `push_balance_state`
- `pop_balance_state`
- `find_closing_paren`

## Success Criteria

### SC-1: Stack round-trip correctness
Given a pushed `(index, level)` pair, a subsequent pop from the same stack returns the same values.

Traceability:
- `push_balance_state`
- `pop_balance_state`

### SC-2: Multi-entry LIFO correctness
Given multiple pushed entries, repeated pops return entries in reverse order of insertion.

Traceability:
- `push_balance_state`
- `pop_balance_state`
- `struct balance_state`

### SC-3: Full stack release
After releasing the balance-state stack, the module retains no remaining saved entries in that stack state.

Traceability:
- `free_balance_stack`
- `struct balance_state`

### SC-4: Simple match resolution
For a token sequence containing one balanced opening and closing pair at the specified level, the matching-close search returns the closing token's index.

Traceability:
- `find_closing_paren`

### SC-5: Nested match resolution
For a token sequence where an outer opening token contains one or more inner balanced pairs, the matching-close search returns the closing token for the original outer opening token, not an inner closing token.

Traceability:
- `find_closing_paren`
- `struct balance_state`

### SC-6: Unmatched-open detection
For a token sequence without a valid matching close for the specified opening token and level, the matching-close search returns its failure result.

Traceability:
- `find_closing_paren`

## Out of Scope

The Rust port specification does not require evidence-free additions such as:

- new public APIs beyond the ported module boundary,
- thread-safety guarantees,
- parser error recovery extensions,
- serialization or persistence,
- FFI surfaces,
- performance benchmark targets.

## Notes for Port Alignment

The Rust rewrite should preserve the module's observed role as internal parser support logic and should keep behavior aligned with the source-backed responsibilities in `src/parser.c`, especially around nested balance tracking and matching-close lookup.