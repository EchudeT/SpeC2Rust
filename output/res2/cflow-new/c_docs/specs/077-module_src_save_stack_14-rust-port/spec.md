# spec.md

## Title

Functional Specification: `module_src_save_stack_14`

## Document Control

- Project: `cflow-new`
- Module: `module_src_save_stack_14`
- Category: `module_cluster`
- Source basis: `src/parser.c`
- Rust target branch: `077-module_src_save_stack_14-rust-port`
- Generation date: `2026-06-17`

## Overview

This module provides parser save-stack state management within `src/parser.c`. Its evidenced responsibilities are limited to:

- saving the current parser stack state via `save_stack`
- reporting whether the save-stack currently contains any saved state via `save_stack_is_empty`

The Rust rewrite must preserve this functional boundary: it must support saving parser stack state and checking save-stack emptiness in a way that is behaviorally compatible with the original module’s role in the parser.

## Scope

### In Scope

- Save-stack state capture behavior represented by `save_stack`
- Save-stack emptiness query behavior represented by `save_stack_is_empty`
- The relationships between save-stack operations and parser state storage used by the parser module

### Out of Scope

The following are not evidenced by the provided module analysis and must not be added to the specification:

- any public API beyond the save operation and emptiness query
- persistence, serialization, or external state export
- thread-safety guarantees
- recovery, replay, or restore behavior unless separately evidenced elsewhere
- performance targets or benchmark obligations
- FFI-specific behavior

## Feature Specification

### Feature: Save parser stack state

The module must support recording the current parser stack state into an internal save-stack.

This feature exists so parser logic can preserve parser state for later parser-controlled use. The specification only requires the observed behavior that a save operation adds saved state to the module-managed save-stack.

#### Required behavior

- When `save_stack` is invoked, the module records one saved parser stack state into its managed save-stack.
- A successful save operation causes the save-stack to become non-empty if it was previously empty.
- Multiple save operations must be representable as multiple saved states in the save-stack, because the module is named and structured around stack-saving behavior and exposes an emptiness query for that stack.

### Feature: Query whether the save-stack is empty

The module must support checking whether any saved parser stack state is currently held.

#### Required behavior

- `save_stack_is_empty` returns an integer-style truth value indicating whether the save-stack contains no saved state.
- Before any save has occurred, the save-stack must be reportable as empty.
- After at least one successful save, the save-stack must be reportable as non-empty unless another parser-owned operation outside this module has removed all saved states.

## User Scenarios & Testing

### Scenario 1: Initial parser state has no saved stack entries

A parser component starts with no saved parser stack state.

#### Expected behavior

- Calling the emptiness query reports that the save-stack is empty.

#### Test approach

- Initialize the Rust port’s parser/save-stack state to its baseline state.
- Invoke the emptiness check.
- Verify that it reports empty.

### Scenario 2: Saving parser state makes the save-stack non-empty

A parser component decides to preserve the current parser stack state.

#### Expected behavior

- After one call to the save operation, the emptiness query reports non-empty.

#### Test approach

- Start from an empty save-stack state.
- Invoke the save operation once.
- Invoke the emptiness query.
- Verify that it reports non-empty.

### Scenario 3: Multiple saves remain represented as saved state presence

A parser component saves parser state more than once during parsing.

#### Expected behavior

- Repeated save operations continue to leave the save-stack in a non-empty state.
- The module must not regress to reporting empty solely because multiple saves occurred.

#### Test approach

- Start from an empty save-stack state.
- Invoke the save operation multiple times.
- After each call, or at minimum after the final call, invoke the emptiness query.
- Verify that it reports non-empty.

### Scenario 4: Emptiness query is stable when no state changes occur

A parser component polls the save-stack without modifying it.

#### Expected behavior

- Repeated calls to the emptiness query, without intervening save-stack mutation, return the same result.

#### Test approach

- Measure the query result in an initial state.
- Repeat the query without calling save or any other save-stack mutator.
- Verify the result remains unchanged.

## Requirements

### Functional Requirements

#### FR-1: Save-stack state capture

The Rust module shall provide behavior equivalent to `save_stack` from `src/parser.c:411-416`, capturing the current parser stack state into module-managed saved-state storage.

**Traceability:** `save_stack` (`src/parser.c:411-416`)

#### FR-2: Empty-state query

The Rust module shall provide behavior equivalent to `save_stack_is_empty` from `src/parser.c:424-428`, allowing parser code to determine whether the saved-state stack currently contains any entries.

**Traceability:** `save_stack_is_empty` (`src/parser.c:424-428`)

#### FR-3: Save operation changes observable emptiness when applicable

If the save-stack is empty before a save, then after a successful `save_stack` operation the emptiness query shall indicate non-empty.

**Traceability:** `save_stack` (`src/parser.c:411-416`), `save_stack_is_empty` (`src/parser.c:424-428`)

#### FR-4: Multiple saved states are supported as stack content

The Rust module shall preserve the behavioral model of a save-stack that can hold saved parser state entries across multiple save operations, such that the observable state remains non-empty after one or more saves until cleared by parser behavior outside the evidenced scope.

**Traceability:** `save_stack` (`src/parser.c:411-416`), `save_stack_is_empty` (`src/parser.c:424-428`)

### Key Entities

#### Save-stack state

The central entity is the internal storage that represents saved parser stack states. Its exact implementation is not specified here, but it must support:

- holding zero or more saved parser state entries
- changing from empty to non-empty when a save occurs
- being observable through the emptiness query

**Traceability:** `save_stack`, `save_stack_is_empty`

#### Parser stack snapshot entry

A saved entry represents the parser stack state captured at the time of `save_stack`. The internal contents of each entry are not specified by the provided evidence, but the Rust rewrite must preserve the concept of a saved parser-state item being added to the save-stack.

**Traceability:** `save_stack`

#### Related parser-local structures

The source analysis shows parser-local structures in `src/parser.c`, including anonymous structs and `struct balance_state`. These structures establish that save-stack behavior exists within broader parser state management. However, no direct externally visible requirements are evidenced here beyond save-stack storage and empty/non-empty observation.

**Traceability:** `src/parser.c` type set including anonymous parser structs and `struct balance_state`

## Success Criteria

### SC-1: Empty at baseline

In a test environment representing the parser’s initial save-stack state, the Rust implementation reports the save-stack as empty before any save operation occurs.

**Traceability:** `save_stack_is_empty` (`src/parser.c:424-428`)

### SC-2: Non-empty after one save

From an initially empty save-stack, one successful save operation causes the Rust implementation to report the save-stack as non-empty.

**Traceability:** `save_stack` (`src/parser.c:411-416`), `save_stack_is_empty` (`src/parser.c:424-428`)

### SC-3: Non-empty after repeated saves

From an initially empty save-stack, two or more save operations still result in the save-stack being reported as non-empty.

**Traceability:** `save_stack` (`src/parser.c:411-416`), `save_stack_is_empty` (`src/parser.c:424-428`)

### SC-4: Query consistency without mutation

Repeated emptiness queries with no intervening save-stack mutation yield the same result.

**Traceability:** `save_stack_is_empty` (`src/parser.c:424-428`)

### SC-5: Functional boundary preservation

The Rust rewrite implements the evidenced module behavior for save-stack saving and emptiness checking without requiring or exposing unsupported capabilities not evidenced in the source analysis.

**Traceability:** `save_stack` (`src/parser.c:411-416`), `save_stack_is_empty` (`src/parser.c:424-428`), module file `src/parser.c`

## Acceptance Notes

- Conformance is defined by behavioral equivalence for save-stack save and empty/non-empty observation.
- This specification intentionally does not define restore/pop semantics, memory layout, or parser-wide architecture beyond what is evidenced for this module.
- Any Rust design chosen to satisfy this specification must remain within these functional boundaries.