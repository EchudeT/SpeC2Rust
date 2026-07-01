# spec.md

## Title

Rust Functional Specification for `module_src_linked_list_entry_02`

## Document Control

- **Project**: `cflow-new`
- **Module**: `module_src_linked_list_entry_02`
- **Category**: `module_cluster`
- **Source basis**: `src/symbol.c`
- **Rust branch**: `065-module_src_linked_list_entry_02-rust-port`
- **Generation date**: `2026-06-11`

## Overview

This module defines behavior around symbol-list traversal and filtering for function-oriented symbol entries. The analyzed functionality centers on:

- collecting function symbols into a returned array,
- moving parameter-linked symbols by level,
- iterating over a starter/target symbol list,
- marking caller symbols reachable from selected symbols, and
- eliminating symbols that are not retained as targets.

The Rust rewrite must preserve the observed functional behavior of these operations as they apply to the module’s symbol records and linked-list-backed relationships.

## Feature Specification

### Summary

The Rust version must implement a symbol-processing module that supports:

1. **Function symbol collection**
   - Gather the current set of function-related symbols from module-managed symbol storage.
   - Return both the count and access to the collected symbol references.

2. **Parameter relocation by scope/level**
   - Reassign or move parameter-associated entries according to a provided level value.
   - Preserve the module’s symbol relationships after the move.

3. **Starter list iteration**
   - Provide ordered traversal over a maintained linked-list-backed set of starter symbols.
   - Support obtaining the first starter and subsequent starters using iterator state.

4. **Caller marking**
   - Mark caller-related symbols starting from a provided symbol.
   - Apply marking through symbol-linked caller relationships used by target retention.

5. **Non-target elimination**
   - Remove or exclude symbols that are not retained as targets after caller marking and target selection have been established.
   - Preserve target-relevant symbols and their required linked entries.

### Functional Boundaries

The Rust rewrite must cover only the behavior evidenced by the analyzed functions in `src/symbol.c`:

- `collect_functions`
- `move_parms`
- `first_starter`
- `next_starter`
- `mark_callers`
- `eliminate_non_targets`

No additional externally visible capabilities are required beyond these behaviors.

## User Scenarios & Testing

### Scenario 1: Collecting all currently known function symbols

A caller needs a complete list of function symbols currently tracked by the symbol subsystem.

**Expected behavior**
- The module returns the number of collected function symbols.
- The caller receives access to the collected symbol references.
- The returned count matches the number of references made available.

**Testing focus**
- Verify zero-function, single-function, and multiple-function cases.
- Verify the returned count and returned symbol-reference list remain consistent.

### Scenario 2: Moving parameter symbols for a given level

A caller updates symbol organization after entering or processing a particular level and needs parameter symbols moved accordingly.

**Expected behavior**
- Calling the parameter-move operation with a level value updates the affected parameter-linked symbols.
- Symbols unrelated to the specified parameter move remain unaffected from the perspective of this module’s functional outputs.

**Testing focus**
- Verify behavior for a level with no matching parameter symbols.
- Verify behavior for a level with one or more parameter symbols.
- Verify subsequent symbol collection and starter iteration still operate on a coherent symbol set.

### Scenario 3: Iterating starter symbols

A caller needs to walk the maintained starter set in order using iterator state.

**Expected behavior**
- `first` returns the first available starter symbol for a supplied iterator state, or no symbol if the starter set is empty.
- `next` returns each subsequent starter symbol until iteration is exhausted.
- Repeated iteration over the same unchanged starter set yields the same traversal result.

**Testing focus**
- Empty starter set.
- One-entry starter set.
- Multi-entry starter set with full traversal.
- End-of-iteration behavior after the last starter.

### Scenario 4: Retaining target-relevant symbols by marking callers

A caller identifies a symbol that should drive retention and invokes caller marking so dependent caller relationships are preserved.

**Expected behavior**
- Caller-linked symbols reachable by the module’s caller-marking logic become marked for retention.
- The marking result is sufficient for later elimination logic to preserve target-relevant symbols.

**Testing focus**
- Symbol with no callers.
- Symbol with one caller.
- Symbol with multiple caller-linked entries.
- Repeated marking does not produce inconsistent retention state.

### Scenario 5: Eliminating non-target symbols

After targets and caller markings are established, a caller requests removal of non-target symbols.

**Expected behavior**
- Symbols not retained as targets are eliminated from the active symbol set considered by this module.
- Symbols retained through target or caller-marking logic remain available.
- Starter iteration and function collection reflect the post-elimination set.

**Testing focus**
- No targets retained.
- All symbols retained.
- Mixed retained and non-retained symbols.
- Verify function collection and starter iteration after elimination.

## Requirements

### Functional Requirements

#### FR-1: Function collection
The module shall provide behavior equivalent to `collect_functions` to gather function symbols from module-managed symbol data and report the number of collected symbols.

**Traceability**
- Function: `collect_functions`
- Types involved: `table_entry`, `collect_data`, `linked_list_entry`

#### FR-2: Returned collection consistency
When function collection returns symbol references, the module shall ensure the returned count matches the number of collected symbol references made available to the caller.

**Traceability**
- Function: `collect_functions`
- Types involved: `collect_data`, `table_entry`

#### FR-3: Parameter movement by level
The module shall provide behavior equivalent to `move_parms(int level)` that updates parameter-linked symbol placement or association based on the supplied level.

**Traceability**
- Function: `move_parms`
- Types involved: `linked_list_entry`, `table_entry`

#### FR-4: Starter iteration initialization
The module shall provide behavior equivalent to `first_starter(void *itr)` that initializes traversal over starter symbols and returns the first starter symbol when one exists.

**Traceability**
- Function: `first_starter`
- Types involved: `linked_list_entry`

#### FR-5: Starter iteration advancement
The module shall provide behavior equivalent to `next_starter(void *itr)` that advances traversal state and returns the next starter symbol until no more starter symbols remain.

**Traceability**
- Function: `next_starter`
- Types involved: `linked_list_entry`

#### FR-6: Caller marking
The module shall provide behavior equivalent to `mark_callers(Symbol *sym)` to mark caller-related symbols derived from a supplied symbol for later retention decisions.

**Traceability**
- Function: `mark_callers`
- Types involved: `linked_list_entry`

#### FR-7: Non-target elimination
The module shall provide behavior equivalent to `eliminate_non_targets(void)` that removes or excludes symbols not retained as targets after target-selection and caller-marking state are considered.

**Traceability**
- Function: `eliminate_non_targets`
- Types involved: `linked_list_entry`, `table_entry`

#### FR-8: Post-elimination coherence
After non-target elimination, the module shall maintain a coherent symbol state such that function collection and starter iteration operate on the remaining retained symbols only.

**Traceability**
- Functions: `collect_functions`, `first_starter`, `next_starter`, `eliminate_non_targets`
- Types involved: `table_entry`, `linked_list_entry`

### Key Entities

#### Symbol
The central entity manipulated by this module. Symbols represent entries subject to collection, iteration, caller marking, parameter movement, and elimination.

**Relationships**
- May be stored through table-entry structures.
- May appear in linked-list-backed collections.
- May be designated as function symbols, starter symbols, caller-related symbols, parameter-related symbols, or retained/non-retained targets depending on module state.

**Traceability**
- Functions: all analyzed functions use `Symbol *` directly or indirectly
- Types involved: `table_entry`

#### Linked list entry
A linked-list-backed entry type is used to organize symbol relationships and traversal sets, including starter traversal and caller-related lists.

**Relationships**
- Connects symbols into ordered or navigable collections.
- Supports traversal by iterator state for starter access.
- Supports caller and elimination processing.

**Traceability**
- Types: `linked_list_entry`, `linked_list`
- Functions: `collect_functions`, `move_parms`, `first_starter`, `next_starter`, `mark_callers`, `eliminate_non_targets`

#### Table entry
A table-backed entry type participates in symbol storage and lookup/collection behavior.

**Relationships**
- Holds or references symbols managed by the module.
- Participates in function collection and symbol retention operations.

**Traceability**
- Types: `table_entry`
- Functions: `collect_functions`, `move_parms`, `eliminate_non_targets`

#### Collect data
A temporary collection-state structure used during function-symbol gathering.

**Relationships**
- Accumulates state needed to build the function-symbol result set.
- Connects symbol-table traversal with returned collection output.

**Traceability**
- Type: `collect_data`
- Function: `collect_functions`

#### Iterator state
An opaque traversal state passed into starter iteration functions.

**Relationships**
- Carries traversal position across `first_starter` and `next_starter`.
- Must be sufficient to support repeatable traversal over the current starter collection.

**Traceability**
- Functions: `first_starter`, `next_starter`

## Success Criteria

### SC-1: Function collection correctness
Given a symbol state containing N function symbols, the Rust module returns a collection count of N and exposes exactly N collected symbol references.

**Traceability**
- `collect_functions`

### SC-2: Empty collection handling
Given a symbol state with no function symbols, function collection returns zero and an empty collected result without producing invalid traversal behavior.

**Traceability**
- `collect_functions`

### SC-3: Parameter move effectiveness
Given parameter-associated symbols for a specified level, invoking the parameter-move operation changes module state so subsequent symbol processing reflects the moved parameter associations.

**Traceability**
- `move_parms`

### SC-4: Starter iteration completeness
Given a starter set of size N, one call to starter initialization followed by repeated advancement yields each starter symbol exactly once and then reports no further starter symbols.

**Traceability**
- `first_starter`, `next_starter`

### SC-5: Empty starter iteration
Given an empty starter set, starter initialization returns no symbol and advancement does not yield any symbol.

**Traceability**
- `first_starter`, `next_starter`

### SC-6: Caller retention marking
Given a symbol with caller-linked symbols, caller marking results in those caller-linked symbols being retained by subsequent elimination behavior where applicable.

**Traceability**
- `mark_callers`, `eliminate_non_targets`

### SC-7: Non-target elimination correctness
Given a mixed symbol set containing retained targets and non-targets, elimination removes or excludes all non-targets while preserving retained target-relevant symbols.

**Traceability**
- `eliminate_non_targets`, `mark_callers`

### SC-8: Post-elimination observable consistency
After elimination, both function collection and starter iteration operate only on the retained symbol set and do not surface eliminated symbols.

**Traceability**
- `collect_functions`, `first_starter`, `next_starter`, `eliminate_non_targets`

## Out of Scope

The Rust rewrite specification does not require any behavior not evidenced by the analyzed source slice, including:

- new public APIs beyond the functional equivalents described here,
- thread-safety guarantees,
- serialization or persistence,
- FFI requirements,
- recovery mechanisms,
- performance or benchmark targets.

## Acceptance Notes

Conformance should be evaluated by tests that construct controlled symbol states and verify:

- collection counts and returned symbol references,
- parameter-move effects at specific levels,
- starter traversal order and completion behavior,
- caller-marking impact on retention state, and
- elimination results as observed through the module’s collection and iteration behaviors.