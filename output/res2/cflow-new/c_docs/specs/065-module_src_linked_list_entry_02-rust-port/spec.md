# spec.md

## Title

Functional Specification: `module_src_linked_list_entry_02` Rust Port

## Metadata

- Project: `cflow-new`
- Module: `module_src_linked_list_entry_02`
- Category: `module_cluster`
- Source basis: `src/symbol.c`
- Rust branch: `065-module_src_linked_list_entry_02-rust-port`
- Generation date: `2026-06-17`

## Overview

This module manages a symbol-linked-list based subset of call-graph and symbol-table behavior centered on:

- collecting function symbols into a returned sequence,
- reassigning parameter symbols by scope level,
- iterating over a subset of symbols designated as starters,
- propagating caller marking through linked symbol relationships,
- removing symbols that are not part of the selected target set.

The Rust rewrite must preserve the observable behavior of these operations as defined by the source module, including the relationships between symbol records, linked-list entries, and table entries used to traverse and filter symbols.

## Feature Specification

### In-Scope Functionality

The Rust version must implement the module behavior evidenced by these functions in `src/symbol.c`:

- `collect_functions`
- `move_parms`
- `first_starter`
- `next_starter`
- `mark_callers`
- `eliminate_non_targets`

### Functional Description

#### 1. Function symbol collection

The module provides a way to gather function-related symbols from the symbol store and return them as a contiguous result sequence together with a count.

Observed behavior boundary:

- the operation produces a count of collected symbols,
- it writes back a symbol pointer collection through an output parameter,
- the collection is derived from the module’s symbol/table contents rather than caller-provided symbol arrays.

The Rust port must preserve:

- collection over the module’s maintained symbol set,
- deterministic count/result consistency,
- correct handling of the empty-result case.

#### 2. Parameter relocation by level

The module supports moving parameter-associated symbols according to a supplied level value.

Observed behavior boundary:

- the operation takes a single integer level,
- it updates symbol organization based on that level,
- it acts on linked-list/table-managed symbol entries.

The Rust port must preserve:

- application of the move only within the symbol data managed by this module,
- level-based reassignment behavior,
- no creation of unrelated symbols as a side effect.

#### 3. Starter iteration

The module provides iteration over a subset of symbols identified as starters through two functions:

- one function returns the first starter for an iterator state,
- one function returns subsequent starters for that same iterator state.

Observed behavior boundary:

- iteration state is carried through an opaque iterator pointer,
- the result type is a symbol pointer,
- the first/next contract indicates ordered traversal over a starter subset.

The Rust port must preserve:

- a first-item retrieval operation,
- continued traversal from iterator state,
- termination by returning no further symbol when the starter subset is exhausted.

#### 4. Caller marking propagation

The module can mark caller-related symbols starting from a symbol input.

Observed behavior boundary:

- the operation is driven from one symbol,
- it traverses linked-list relationships associated with callers,
- it updates symbol state used by later elimination/filtering behavior.

The Rust port must preserve:

- traversal across caller links reachable from the input symbol,
- marking semantics sufficient for the elimination step to recognize target-connected symbols,
- idempotent behavior when already-marked symbols are revisited.

#### 5. Elimination of non-target symbols

The module removes or filters out symbols that are not targets.

Observed behavior boundary:

- the operation examines the module-managed symbol set,
- it depends on target/mark state represented in symbol/table/list relationships,
- it eliminates symbols outside the retained target-connected subset.

The Rust port must preserve:

- elimination/filtering based on established target criteria,
- retention of symbols that qualify through target or caller-mark propagation,
- consistent updates to the maintained symbol collection after elimination.

## User Scenarios & Testing

### Scenario 1: Collect all function symbols for downstream processing

A caller needs the current set of function symbols known to the symbol subsystem.

Expected module behavior:

- the caller invokes the collection operation,
- the module returns the number of collected function symbols,
- the returned sequence contains exactly the symbols selected by the module’s function-collection rules.

Test expectations:

- when no function symbols are present, count is zero and the result represents an empty collection,
- when function symbols are present, count matches the number of returned entries,
- returned entries are valid symbol references from the module-managed symbol set.

Traceability: `collect_functions`, `table_entry`, `collect_data`, `linked_list_entry`

### Scenario 2: Reassign parameter symbols after entering or leaving a scope level

The parser or symbol manager needs parameter-associated symbols to be moved according to a given level.

Expected module behavior:

- the caller supplies a level,
- symbols relevant to parameter placement are reassigned within the managed symbol structures,
- unrelated symbols remain unaffected by this specific operation.

Test expectations:

- invoking the operation with a level changes placement/state only for symbols governed by the parameter-move rule,
- repeated invocation with the same already-applied effective state does not corrupt symbol organization,
- symbol relationships remain traversable after the move.

Traceability: `move_parms`, `linked_list_entry`, `table_entry`

### Scenario 3: Iterate through starter symbols

A traversal component needs to enumerate starter symbols one by one.

Expected module behavior:

- the caller initializes or provides iterator state,
- the first operation returns the initial starter if one exists,
- repeated next calls yield subsequent starters until exhaustion,
- exhaustion is represented by no symbol result.

Test expectations:

- empty starter set returns no symbol from the first call,
- non-empty starter set yields all starters exactly once through first/next progression,
- iteration state progresses correctly across calls.

Traceability: `first_starter`, `next_starter`, `linked_list_entry`

### Scenario 4: Mark all callers connected to a selected symbol

A target-analysis flow identifies a symbol and must retain all caller-connected symbols.

Expected module behavior:

- starting from the selected symbol, caller relationships are traversed,
- reachable caller symbols are marked,
- revisiting already processed links does not cause incorrect duplication of state changes.

Test expectations:

- directly connected callers are marked,
- transitively connected callers are marked if traversal reaches them,
- pre-marked symbols remain valid and do not break traversal.

Traceability: `mark_callers`, `linked_list_entry`, `Symbol`

### Scenario 5: Eliminate symbols that are not in the target-connected subset

After marking targets and callers, the module must reduce the symbol set to relevant entries.

Expected module behavior:

- symbols lacking the required target/mark status are eliminated,
- symbols that are targets or preserved through marking remain,
- subsequent symbol operations observe the reduced set.

Test expectations:

- non-target, unmarked symbols are absent after elimination,
- target-connected symbols remain present,
- the resulting managed symbol set is internally consistent for further iteration or collection.

Traceability: `eliminate_non_targets`, `mark_callers`, `linked_list_entry`, `table_entry`

## Requirements

### Functional Requirements

#### FR-1: Function collection
The module shall provide an operation that scans the module-managed symbol/table contents and returns the set of collected function symbols together with the total number collected.

Traceability: `collect_functions`, `collect_data`, `table_entry`

#### FR-2: Empty collection support
The function-collection operation shall support the case where no eligible function symbols exist and shall report an empty result consistently.

Traceability: `collect_functions`

#### FR-3: Output/result consistency
The function-collection operation shall ensure the reported count matches the number of returned symbol references.

Traceability: `collect_functions`

#### FR-4: Parameter move by level
The module shall support a level-driven operation that reassigns or relocates parameter-associated symbols within the module-managed symbol structures.

Traceability: `move_parms`, `linked_list_entry`, `table_entry`

#### FR-5: Scoped effect of parameter move
The parameter-move operation shall affect only symbols governed by the parameter relocation behavior for the provided level.

Traceability: `move_parms`

#### FR-6: Starter subset iteration
The module shall provide ordered iteration over the starter subset of symbols through a first-item operation and a next-item operation using caller-supplied iterator state.

Traceability: `first_starter`, `next_starter`, `linked_list_entry`

#### FR-7: Starter iteration exhaustion
Starter iteration shall indicate exhaustion by returning no further symbol once all starter symbols have been traversed.

Traceability: `first_starter`, `next_starter`

#### FR-8: Caller marking from a seed symbol
The module shall support marking caller-related symbols reachable from a supplied symbol through maintained linked relationships.

Traceability: `mark_callers`, `linked_list_entry`, `Symbol`

#### FR-9: Idempotent caller marking
Caller marking shall tolerate revisiting symbols or links without producing inconsistent retained-state results.

Traceability: `mark_callers`

#### FR-10: Non-target elimination
The module shall provide an operation that removes or filters symbols not belonging to the retained target-related subset.

Traceability: `eliminate_non_targets`, `linked_list_entry`, `table_entry`

#### FR-11: Preservation of target-connected symbols
The elimination operation shall preserve symbols that qualify as targets or that must be retained due to caller-mark propagation.

Traceability: `eliminate_non_targets`, `mark_callers`

#### FR-12: Post-elimination consistency
After elimination, the remaining symbol structures shall remain valid for subsequent module operations such as collection or iteration.

Traceability: `eliminate_non_targets`, `collect_functions`, `first_starter`, `next_starter`

### Key Entities

#### Symbol
The primary semantic entity manipulated by this module. Symbols represent functions, parameters, starters, targets, callers, and retained or eliminated entries depending on context.

Relationships:

- referenced from table entries,
- linked through linked-list entries,
- returned by collection and starter iteration,
- used as the seed and result domain for caller marking.

Traceability: `Symbol`, `collect_functions`, `first_starter`, `next_starter`, `mark_callers`

#### Linked list entry
The linkage entity that connects symbols into traversable lists used by collection, parameter movement, starter iteration, caller traversal, and elimination.

Relationships:

- associates list membership with symbols,
- enables sequential traversal,
- carries the structure needed for subset iteration and relationship walking.

Traceability: `linked_list_entry`, `collect_functions`, `move_parms`, `first_starter`, `next_starter`, `mark_callers`, `eliminate_non_targets`

#### Table entry
The symbol-table-facing entity used to store or access symbol records participating in collection, movement, and elimination logic.

Relationships:

- provides access from table-managed storage to symbols,
- participates in scans and updates over the managed symbol set.

Traceability: `table_entry`, `collect_functions`, `move_parms`, `eliminate_non_targets`

#### Collection state
Temporary collection-oriented state used while assembling function symbol results.

Relationships:

- accumulates or tracks results during function collection,
- bridges internal traversal and final returned sequence.

Traceability: `collect_data`, `collect_functions`

#### Iterator state
Caller-held opaque state passed into starter iteration to maintain current traversal position.

Relationships:

- consumed by first/next starter operations,
- determines which starter symbol is returned next.

Traceability: `first_starter`, `next_starter`

## Success Criteria

### SC-1: Function collection correctness
Given a prepared symbol set containing known eligible function symbols, the Rust module returns a collection whose length exactly matches the reported count and whose members match the eligible function symbols from the managed set.

Traceability: `collect_functions`

### SC-2: Empty collection behavior
Given a symbol set with no eligible function symbols, the Rust module reports zero collected symbols and an empty result without failure.

Traceability: `collect_functions`

### SC-3: Parameter move integrity
After invoking parameter movement for a level, parameter-governed symbols reflect the intended level-based reassignment and the managed symbol structures remain traversable.

Traceability: `move_parms`

### SC-4: Starter iteration completeness
For any prepared starter subset, the Rust module’s first/next iteration returns every starter exactly once in traversal order and then reports exhaustion.

Traceability: `first_starter`, `next_starter`

### SC-5: Caller marking reachability
Starting from a selected symbol with caller relationships, the Rust module marks all caller-reachable symbols needed for target retention.

Traceability: `mark_callers`

### SC-6: Elimination correctness
After marking and elimination, all non-target, unretained symbols are absent from the managed set and all required target-connected symbols remain.

Traceability: `mark_callers`, `eliminate_non_targets`

### SC-7: Post-elimination operability
After elimination, subsequent supported operations such as function collection and starter iteration execute successfully over the remaining symbol set.

Traceability: `eliminate_non_targets`, `collect_functions`, `first_starter`, `next_starter`

## Out of Scope

The Rust rewrite specification does not require any behavior not evidenced by this module input, including:

- new public APIs beyond those needed to preserve the documented module behavior,
- thread-safety guarantees,
- serialization or persistence,
- FFI-specific interfaces,
- performance targets or benchmarking requirements,
- recovery or rollback features.