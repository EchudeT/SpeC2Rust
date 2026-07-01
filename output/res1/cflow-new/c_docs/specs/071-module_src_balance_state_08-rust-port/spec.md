# spec.md

## Title
Rust Functional Specification for `module_src_balance_state_08`

## Metadata
- Project: `cflow-new`
- Module: `module_src_balance_state_08`
- Category: `module_cluster`
- Source scope: `src/parser.c`
- Rust branch: `071-module_src_balance_state_08-rust-port`
- Generation date: 2026-06-11

## Overview
This module provides parser-local balance tracking used while scanning token streams with nested grouping. Its responsibilities are:

- maintain a stack of saved parser balance states;
- restore previously saved balance states in last-in, first-out order;
- release any remaining saved state;
- locate the matching closing parenthesis token for a given opening token while accounting for nesting depth.

The Rust rewrite must preserve this behavior as used by the parser in `src/parser.c`, including stack-like state management and balanced-parenthesis search driven by parser nesting level.

## Feature Specification

### Feature: Balance state stack management
The module supports temporary preservation of parser balance context by pushing `(idx, level)` pairs onto an internal linked stack of balance-state records.

The Rust version must implement:
- creation of a new saved state from the supplied index and nesting level;
- insertion of the new state at the top of the stack;
- last-in, first-out removal of the top saved state;
- extraction of the saved `idx` and `level` values when popping;
- full release of all saved states remaining on the stack.

This behavior is evidenced by:
- `push_balance_state`
- `pop_balance_state`
- `free_balance_stack`
- `struct balance_state`

### Feature: Matching closing parenthesis search
The module supports parser scanning that starts from an opening parenthesis token and determines the token position of its corresponding closing parenthesis.

The Rust version must implement:
- acceptance of an opening-token position and a parser nesting level;
- forward scanning from that opening position;
- tracking of nested balance changes via the balance-state mechanism;
- return of the token index corresponding to the matching closing parenthesis when found.

This behavior is evidenced by:
- `find_closing_paren`
- `struct balance_state`

## User Scenarios & Testing

### Scenario 1: Parser saves and restores nested balance context
A parser routine reaches a point where it must temporarily preserve the current balance position and nesting level before continuing analysis.

Expected support:
- the current `(idx, level)` can be pushed;
- later, the most recently pushed `(idx, level)` can be restored exactly;
- restoration follows LIFO order if multiple saves occurred.

Test cases:
- push one state and pop it, confirming returned values equal the originals;
- push multiple states and pop them in reverse order, confirming exact values;
- verify the stack top changes appropriately after each operation.

Traceability:
- `push_balance_state`
- `pop_balance_state`

### Scenario 2: Parser abandons pending saved states
A parser path ends with saved balance states still present and needs to discard them without restoring individual entries.

Expected support:
- all saved states can be released;
- after release, no stack entries remain.

Test cases:
- push several states, free the stack, and verify the stack is empty;
- free an already empty stack and confirm it remains empty.

Traceability:
- `free_balance_stack`

### Scenario 3: Parser finds the matching close for a simple parenthesized construct
A parser routine has the token index of an opening parenthesis in a non-nested construct and needs the corresponding closing parenthesis index.

Expected support:
- the matching closing token index is returned for a simple balanced pair.

Test cases:
- given a token sequence equivalent to `(` ... `)`, return the index of the closing `)` for the supplied opening token.

Traceability:
- `find_closing_paren`

### Scenario 4: Parser finds the matching close across nested parentheses
A parser routine encounters an opening parenthesis whose contents include nested parenthesized subexpressions.

Expected support:
- nested opens and closes are accounted for;
- the returned closing token corresponds to the original opening token, not an inner nested pair.

Test cases:
- given a token sequence equivalent to `( a ( b ) c )`, return the index of the outer closing `)`;
- given multiple nesting levels, confirm the result is the close that balances the original open.

Traceability:
- `find_closing_paren`
- `push_balance_state`
- `pop_balance_state`

## Requirements

### Functional Requirements

#### FR-1: Save balance state
The module shall allow the parser to save a balance state consisting of an index and a nesting level onto a stack of saved states.

Traceability:
- `push_balance_state`
- `struct balance_state`

#### FR-2: Restore most recent balance state
The module shall allow the parser to remove and retrieve the most recently saved balance state, returning both the saved index and the saved nesting level.

Traceability:
- `pop_balance_state`
- `struct balance_state`

#### FR-3: Release all saved balance states
The module shall allow the parser to discard all saved balance states remaining on the stack.

Traceability:
- `free_balance_stack`
- `struct balance_state`

#### FR-4: Use LIFO ordering for saved states
When multiple balance states have been saved, the module shall restore them in reverse order of saving.

Traceability:
- `push_balance_state`
- `pop_balance_state`
- `struct balance_state`

#### FR-5: Find matching closing parenthesis from an opening token
The module shall provide a parser operation that, given an opening parenthesis token position and a nesting level, returns the token position of the corresponding closing parenthesis.

Traceability:
- `find_closing_paren`

#### FR-6: Respect nested parenthesis structure during search
The matching-close search shall account for nested parenthesized regions so that inner pairs do not terminate the search for the original opening token.

Traceability:
- `find_closing_paren`
- `push_balance_state`
- `pop_balance_state`

#### FR-7: Clean up temporary balance tracking after search completion
The matching-close search shall not leave unreleased temporary saved balance states after it finishes.

Traceability:
- `find_closing_paren`
- `free_balance_stack`

### Key Entities

#### `balance_state`
A saved parser balance record containing:
- an `idx` value representing a parser/token position relevant to balance tracking;
- a `level` value representing nesting depth;
- a link to the next saved balance record, forming a stack.

Relationship:
- `balance_state` records form the internal stack used by balance push, pop, cleanup, and parenthesis matching operations.

Traceability:
- `struct balance_state`
- `push_balance_state`
- `pop_balance_state`
- `free_balance_stack`
- `find_closing_paren`

#### Balance state stack
A linked LIFO collection of `balance_state` records referenced through a top-of-stack pointer.

Relationship:
- receives new `balance_state` entries from save operations;
- provides entries to restore operations;
- is fully emptied by cleanup operations;
- may be used as temporary tracking storage during matching-close search.

Traceability:
- `push_balance_state`
- `pop_balance_state`
- `free_balance_stack`
- `find_closing_paren`

#### Opening token / closing token correspondence
A parser relationship between an opening parenthesis token and its matching closing parenthesis token under a given nesting context.

Relationship:
- determined by scanning logic in the matching-close operation;
- depends on correct handling of nested balance states.

Traceability:
- `find_closing_paren`

## Success Criteria

### SC-1: Correct push/pop value preservation
For any tested `(idx, level)` pair pushed onto an empty stack, one subsequent pop returns exactly the same `idx` and `level`.

Traceability:
- `push_balance_state`
- `pop_balance_state`

### SC-2: Correct LIFO restoration
For any tested sequence of at least three pushed balance states, repeated pops return them in reverse insertion order.

Traceability:
- `push_balance_state`
- `pop_balance_state`

### SC-3: Complete stack cleanup
After pushing one or more balance states and invoking stack cleanup, the stack is empty and no saved entries remain reachable.

Traceability:
- `free_balance_stack`
- `struct balance_state`

### SC-4: Simple matching close detection
Given parser token fixtures containing a single balanced parenthesis pair with no nested pair inside, the matching-close operation returns the index of the corresponding closing parenthesis for the supplied opening token.

Traceability:
- `find_closing_paren`

### SC-5: Nested matching close detection
Given parser token fixtures containing nested parenthesis pairs, the matching-close operation returns the closing parenthesis index that matches the original opening token rather than an inner nested close.

Traceability:
- `find_closing_paren`

### SC-6: No residual temporary balance state after matching search
After the matching-close operation completes on tested balanced inputs, temporary balance-state storage used by the operation has been fully released.

Traceability:
- `find_closing_paren`
- `free_balance_stack`

## Out of Scope
The Rust rewrite specification does not add or require:
- new parser features beyond balance-state stack handling and matching closing-parenthesis search;
- new public APIs beyond those needed to preserve the evidenced module behavior;
- thread-safety guarantees;
- serialization, persistence, or cross-language interfaces;
- recovery policies or extended diagnostics not evidenced by the source functions listed above.