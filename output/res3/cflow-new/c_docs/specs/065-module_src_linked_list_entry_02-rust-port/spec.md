# spec.md

## Title

Rust Functional Specification for `module_src_linked_list_entry_02`

## Metadata

- Project: `cflow-new`
- Module: `module_src_linked_list_entry_02`
- Category: `module_cluster`
- Source basis: `src/symbol.c`
- Rust branch target: `065-module_src_linked_list_entry_02-rust-port`
- Generation date: `2026-06-17`

## Overview

This module defines symbol-list behaviors used to work with function symbols and their call relationships inside the symbol subsystem.

The Rust rewrite must preserve the observed behavior of the analyzed C module in these areas:

- collecting function symbols into a returned sequence,
- moving parameter symbols between scope levels,
- iterating over a subset of symbols designated as starters,
- marking caller relationships reachable from a selected symbol,
- eliminating symbols that are not target-reachable according to the module’s caller-marking logic.

The specification is limited to functionality evidenced by the analyzed functions and related data structures in `src/symbol.c`.

## Scope

### In Scope

The Rust version must implement behavior corresponding to the following module functions in `src/symbol.c`:

- `collect_functions`
- `move_parms`
- `first_starter`
- `next_starter`
- `mark_callers`
- `eliminate_non_targets`

It must also preserve the functional role of the module’s linked-list-based symbol traversal and table-entry relationships as used by those functions.

### Out of Scope

The following are not specified here unless directly required by the functions above:

- unrelated symbol-table operations outside the observed call paths,
- new public APIs not evidenced by the C module,
- concurrency behavior,
- persistence or serialization,
- FFI guarantees,
- performance promises beyond successful completion of the required behaviors.

## Feature Specification

### 1. Function Symbol Collection

The module provides a way to collect symbol entries representing functions and return them as an indexed sequence together with the total count.

The Rust version must:

- produce a complete collection of function symbols recognized by the module’s symbol store,
- return the number of collected function symbols,
- make the collected sequence available to the caller in the same conceptual role as the C output parameter.

This behavior is evidenced by `collect_functions` and its use of collection-specific data and linked-list entries.

### 2. Parameter Relocation by Level

The module supports moving parameter-related symbols according to a supplied scope or nesting level.

The Rust version must:

- accept a level value,
- apply the parameter-moving operation to symbols associated with that level,
- preserve symbol availability consistent with the C module’s scope-adjustment behavior.

This behavior is evidenced by `move_parms`.

### 3. Starter Iteration

The module exposes iteration over symbols designated as starters.

The Rust version must:

- provide a way to obtain the first starter symbol for an iteration state,
- provide a way to advance to the next starter symbol using the same iteration state,
- return symbols in a stable traversal order consistent with the underlying linked-list ordering used by the C module,
- signal exhaustion when no further starter symbol exists.

This behavior is evidenced by `first_starter` and `next_starter`, both of which operate through an iterator handle and linked-list entry traversal.

### 4. Caller Mark Propagation

The module supports marking caller relationships starting from a given symbol.

The Rust version must:

- begin from a provided symbol,
- traverse caller-linked symbol relationships as represented in the module’s symbol/list structures,
- mark callers according to the same reachability semantics used by the C implementation.

This behavior is evidenced by `mark_callers`.

### 5. Elimination of Non-Target Symbols

The module supports removal or suppression of symbols that are not considered targets under the module’s caller-marking logic.

The Rust version must:

- evaluate symbol target status using the same relationship basis as the C implementation,
- retain symbols that qualify as targets,
- eliminate or exclude symbols that do not qualify,
- perform this operation over the relevant symbol collection used by the module.

This behavior is evidenced by `eliminate_non_targets`, which traverses linked-list-backed entries and relies on caller marking.

## User Scenarios & Testing

### Scenario 1: Collecting all function symbols for later processing

A caller needs a complete list of function symbols currently known to the symbol subsystem.

Expected behavior:

- invoking the collection operation returns a count,
- the returned sequence contains exactly the function symbols gathered by the module,
- the count matches the number of returned symbols.

Test guidance:

- prepare a symbol set containing function and non-function entries,
- run the collection operation,
- verify only function symbols are included,
- verify count and returned sequence length match.

Traceability:

- `collect_functions`
- collection-related table and linked-list entries in `src/symbol.c`

### Scenario 2: Reassigning parameter symbols after a scope-level change

A caller updates symbol organization after parsing or symbol-scope changes and needs parameters moved for a specific level.

Expected behavior:

- invoking the operation with a level updates parameter placement for that level,
- symbols not associated with the specified level are not incorrectly moved by this operation.

Test guidance:

- prepare symbols spanning multiple levels with parameter-designated entries,
- invoke the operation for one level,
- verify only the intended level’s parameter associations change consistently with the C behavior.

Traceability:

- `move_parms`
- linked-list entry usage around the parameter-moving logic

### Scenario 3: Iterating through starter symbols

A caller needs to enumerate the starter subset of symbols without directly traversing internal storage.

Expected behavior:

- the first-iteration call returns the first available starter or exhaustion,
- repeated next-iteration calls return subsequent starters in order,
- iteration eventually exhausts with no duplicates introduced by the iterator itself.

Test guidance:

- prepare a symbol set with zero, one, and multiple starter symbols,
- verify first/next behavior in each case,
- verify traversal order remains consistent across repeated runs over unchanged data.

Traceability:

- `first_starter`
- `next_starter`
- linked-list entry traversal associated with starters

### Scenario 4: Marking all caller-reachable symbols from a target

A caller needs to identify caller-side reachability for a chosen symbol.

Expected behavior:

- invoking caller marking on a symbol marks its callers,
- transitive caller relationships are marked according to the module’s traversal semantics,
- unrelated symbols remain unmarked.

Test guidance:

- prepare a call graph with direct callers, transitive callers, and unrelated symbols,
- invoke caller marking on a selected callee,
- verify reachable caller set is marked and unrelated symbols are not.

Traceability:

- `mark_callers`
- caller-linked list traversal in `src/symbol.c`

### Scenario 5: Eliminating symbols outside the target set

A caller wants the symbol set reduced to target-relevant entries after reachability analysis.

Expected behavior:

- target-relevant symbols are preserved,
- non-target symbols are eliminated or omitted from the retained set,
- the retained set matches the marking rules used by the module.

Test guidance:

- prepare a symbol graph containing target-reachable and non-reachable entries,
- execute elimination,
- verify only target-qualified symbols remain represented for subsequent module use.

Traceability:

- `eliminate_non_targets`
- `mark_callers`

## Requirements

### Functional Requirements

#### FR-1: Collect function symbols
The module shall provide a function-symbol collection operation that gathers the function symbols present in the module’s symbol storage and returns both the collection and its count.

Traceability:

- `collect_functions`
- `struct collect_data`
- linked-list entry use at `src/symbol.c:368`

#### FR-2: Restrict collection to function-class symbols
The collection operation shall include function symbols and exclude symbols outside that class.

Traceability:

- `collect_functions`
- related symbol table entry usage in `src/symbol.c`

#### FR-3: Support level-based parameter movement
The module shall provide an operation that moves parameter symbols for a specified level.

Traceability:

- `move_parms`
- linked-list entry use at `src/symbol.c:417`

#### FR-4: Provide starter iteration entry point
The module shall provide an operation that returns the first starter symbol for a supplied iteration state.

Traceability:

- `first_starter`
- linked-list entry use at `src/symbol.c:456`

#### FR-5: Provide starter iteration advancement
The module shall provide an operation that advances an iteration state and returns the next starter symbol until exhaustion.

Traceability:

- `next_starter`
- linked-list entry use at `src/symbol.c:472`

#### FR-6: Preserve starter traversal semantics
Starter iteration shall traverse only starter-designated symbols and shall eventually terminate when no further starter symbols remain.

Traceability:

- `first_starter`
- `next_starter`

#### FR-7: Mark caller relationships from a symbol
The module shall provide caller-marking behavior beginning from a specified symbol and applying marks to caller-related symbols.

Traceability:

- `mark_callers`
- linked-list entry use at `src/symbol.c:502`

#### FR-8: Support caller-mark propagation over caller links
Caller marking shall follow the caller-link relationships represented by the module’s symbol/list structures.

Traceability:

- `mark_callers`
- linked-list structures in `src/symbol.c`

#### FR-9: Eliminate non-target symbols
The module shall provide an operation that eliminates or excludes symbols not qualifying as targets under the module’s marking rules.

Traceability:

- `eliminate_non_targets`
- linked-list entry use at `src/symbol.c:518`

#### FR-10: Preserve target-relevant symbols during elimination
The elimination operation shall retain symbols that qualify as targets according to the same caller-mark-based logic used by the C module.

Traceability:

- `eliminate_non_targets`
- `mark_callers`

### Key Entities

#### Symbol
The central entity is the symbol record (`Symbol`), representing items stored in the symbol subsystem. The analyzed functions treat symbols as:

- function candidates for collection,
- parameter-related entries subject to level-based movement,
- starter candidates for iteration,
- nodes in caller relationships,
- entries subject to target elimination.

Traceability:

- all listed main functions

#### Linked List Entry
The module relies on linked-list entry structures to connect symbols into traversable sequences used for:

- function collection traversal,
- parameter movement traversal,
- starter iteration,
- caller traversal,
- elimination traversal.

Traceability:

- `struct linked_list_entry` references at `src/symbol.c:368, 417, 456, 472, 502, 518`
- anonymous `struct linked_list` definitions

#### Table Entry
Table-entry structures represent symbol-table organization used by the module to access and manipulate symbols and related lists.

Traceability:

- anonymous `struct table_entry` references throughout `src/symbol.c`

#### Collect Data
Collection-specific state is represented by `struct collect_data`, which supports accumulation of symbols during function collection.

Traceability:

- `struct collect_data` at `src/symbol.c:305-310, 316, 333, 347`

#### Iterator State
Starter iteration uses an opaque iteration handle (`void *itr` in the C interface) that carries traversal position across first/next calls.

Traceability:

- `first_starter`
- `next_starter`

## Success Criteria

### SC-1: Correct function collection
Given a mixed symbol set containing function and non-function symbols, the Rust module returns a collection whose size equals the reported count and whose members correspond exactly to the function symbols recognized by the source behavior.

Traceability:

- `collect_functions`

### SC-2: Correct level-based parameter movement
Given symbols associated with multiple levels, invoking parameter movement for one level changes parameter placement only as required for that level and does not misapply the operation to unrelated levels.

Traceability:

- `move_parms`

### SC-3: Correct starter iteration behavior
Given any symbol set, starter iteration returns each starter symbol in traversal order, returns no non-starter symbol, and reaches exhaustion cleanly.

Traceability:

- `first_starter`
- `next_starter`

### SC-4: Correct caller marking reachability
Given a caller graph with direct, transitive, and unrelated symbols, caller marking from a selected symbol marks exactly the caller-reachable set required by the source behavior.

Traceability:

- `mark_callers`

### SC-5: Correct non-target elimination
After elimination is run on a symbol set with both target-relevant and non-target symbols, only the symbols that satisfy the module’s target logic remain available to subsequent module traversal or processing.

Traceability:

- `eliminate_non_targets`
- `mark_callers`

### SC-6: No loss of required linked-list-driven traversal behavior
For collection, starter iteration, caller traversal, and elimination, the Rust rewrite preserves the observable traversal outcomes of the C module over equivalent symbol/link data.

Traceability:

- `collect_functions`
- `first_starter`
- `next_starter`
- `mark_callers`
- `eliminate_non_targets`
- linked-list entry references in `src/symbol.c`

## Acceptance Notes

- The Rust rewrite may change internal representation, but it must preserve the functional behavior specified above.
- Any safe Rust abstraction used in place of the C linked-list and table-entry structures must support the same observable outcomes for collection, iteration, caller marking, and elimination.
- The specification intentionally does not require capabilities not evidenced by `src/symbol.c`.