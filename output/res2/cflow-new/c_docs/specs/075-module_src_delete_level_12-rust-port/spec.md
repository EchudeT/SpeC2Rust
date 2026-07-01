# spec.md

## Title

Rust Functional Specification for `module_src_delete_level_12`

## Metadata

- Project: `cflow-new`
- Module: `module_src_delete_level_12`
- Category: `module_cluster`
- Source basis: `src/symbol.c`
- Rust branch target: `075-module_src_delete_level_12-rust-port`
- Generation date: 2026-06-17

## Overview

This module provides scope-level symbol cleanup behavior for entries stored in the symbol table maintained in `src/symbol.c`. Its evidenced responsibility is to identify entries associated with a specified nesting level and mark them for deletion according to storage class.

Two deletion predicates are present:

- `delete_level_autos`
- `delete_level_statics`

The Rust rewrite must preserve the observable behavior of these predicates when used during symbol-table traversal or collection workflows in the surrounding symbol-management logic.

## Feature Specification

### Summary

The module supplies filtering behavior used to remove symbol-table entries belonging to a particular level. It distinguishes between:

- automatic-scope entries at a target level
- static-scope entries at a target level

The Rust version must implement equivalent decision logic so that the surrounding symbol table can clean up the correct subset of entries when a scope level is exited or otherwise processed.

### In-Scope Functionality

1. Evaluate a symbol-table entry against a requested level.
2. Distinguish whether the deletion request applies to automatic entries or static entries.
3. Return a callback-style result compatible with traversal/filtering usage in the original module.
4. Operate on symbol-table entry data and callback context data supplied through generic pointers in the C source, while preserving equivalent semantic roles in Rust.

### Out of Scope

The following are not evidenced as responsibilities of this module and must not be added as new requirements:

- creation of symbol-table entries
- parsing of source code
- symbol lookup beyond deletion filtering
- persistence or serialization
- concurrency guarantees
- recovery or rollback behavior
- new public APIs beyond what is needed to preserve module behavior in Rust

## User Scenarios & Testing

### Scenario 1: Remove automatic symbols for a completed scope level

A caller is traversing symbol-table entries after leaving a scope level. For each entry, it applies the automatic-entry deletion predicate with a target level in the callback context.

Expected behavior:

- entries that are automatic and belong to the target level are selected for deletion
- entries from other levels are not selected
- non-automatic entries are not selected by this predicate

Testing implications:

- provide entries spanning multiple levels
- include both automatic and non-automatic storage classes
- verify only matching automatic entries at the requested level are flagged by the predicate result

### Scenario 2: Remove static symbols associated with a level

A caller performs a cleanup pass intended for static entries tied to a particular level and applies the static-entry deletion predicate during traversal.

Expected behavior:

- entries that are static and belong to the target level are selected for deletion
- entries at other levels are not selected
- non-static entries are not selected by this predicate

Testing implications:

- provide entries with static and non-static storage classes
- vary the level values
- verify only static entries at the requested level match

### Scenario 3: Mixed symbol table during traversal

A symbol table contains a mixture of entries, including automatic, static, and unrelated symbols. The caller runs traversal-based cleanup using one predicate at a time.

Expected behavior:

- each predicate selects only its intended subset
- predicates do not overlap on entries whose storage classification does not match the predicate
- traversal can process heterogeneous tables without misclassifying unrelated entries

Testing implications:

- build a mixed collection of symbol entries
- run the automatic predicate and static predicate independently
- verify the selected sets are correct and stable for each pass

### Scenario 4: No entries match the requested level

A cleanup pass is invoked with a target level for which no symbol-table entries qualify.

Expected behavior:

- no entries are selected by the predicate
- non-matching entries remain unaffected by the predicate outcome

Testing implications:

- use entries with different levels than the requested level
- confirm all callback evaluations produce the non-delete result

## Requirements

### Functional Requirements

#### FR-1: Level-based evaluation
The module shall evaluate whether a symbol-table entry is associated with the level provided in callback context data.

**Traceability:** `delete_level_autos`, `delete_level_statics`, `struct collect_data`

#### FR-2: Automatic-entry deletion predicate
The module shall provide behavior equivalent to `delete_level_autos`, selecting only entries that both:
- are classified as automatic entries, and
- belong to the requested level.

**Traceability:** `delete_level_autos`, `struct table_entry`, `struct collect_data`

#### FR-3: Static-entry deletion predicate
The module shall provide behavior equivalent to `delete_level_statics`, selecting only entries that both:
- are classified as static entries, and
- belong to the requested level.

**Traceability:** `delete_level_statics`, `struct table_entry`, `struct collect_data`

#### FR-4: Callback-compatible result
The module shall return a deterministic callback result for each evaluated entry that allows surrounding traversal logic to distinguish selected-for-deletion entries from non-selected entries.

**Traceability:** `delete_level_autos`, `delete_level_statics`

#### FR-5: Entry-local decision behavior
The module shall make deletion decisions from the evaluated symbol entry and the supplied level/context only, without requiring unrelated global side effects from the predicate itself.

**Traceability:** `delete_level_autos`, `delete_level_statics`

### Key Entities

#### Symbol Table Entry
Represents an item stored in the symbol table. The deletion predicates depend on this entity carrying at least:

- a level association
- a storage classification sufficient to distinguish automatic vs static handling

**Traceability:** `struct table_entry` occurrences in `src/symbol.c`, especially usage by `delete_level_autos` and `delete_level_statics`

#### Collection / Callback Context
Represents the auxiliary data passed to traversal callbacks. For this module, it provides the target level used during deletion selection.

**Traceability:** `struct collect_data` in `src/symbol.c:305-310` and related uses

#### Linked-List-Based Traversal Context
The wider symbol module uses linked-list structures to hold collections traversed by callbacks. This module participates in that traversal model by supplying callback predicates rather than owning storage itself.

**Traceability:** `struct linked_list`, `struct linked_list_entry`, callback signatures of `delete_level_autos` and `delete_level_statics`

#### Relationship of Entities
- traversal logic visits symbol-table entries held in linked-list-backed collections
- a callback context supplies the target level
- the deletion predicate evaluates one symbol-table entry at a time against that level and against its storage classification
- the traversal mechanism uses the predicate result to decide whether the entry is selected for deletion

## Success Criteria

### SC-1: Correct automatic-level matching
Given a mixed set of symbol-table entries, the Rust implementation selects exactly the automatic entries whose level equals the requested level, and selects no others.

**Traceability:** `delete_level_autos`

### SC-2: Correct static-level matching
Given a mixed set of symbol-table entries, the Rust implementation selects exactly the static entries whose level equals the requested level, and selects no others.

**Traceability:** `delete_level_statics`

### SC-3: Non-matching levels are rejected
For both deletion predicates, entries whose level differs from the requested level produce the non-selected result.

**Traceability:** `delete_level_autos`, `delete_level_statics`, `struct collect_data`

### SC-4: Storage-class separation is preserved
The automatic deletion behavior does not select static entries, and the static deletion behavior does not select automatic entries.

**Traceability:** `delete_level_autos`, `delete_level_statics`, `struct table_entry`

### SC-5: Traversal integration remains behaviorally compatible
When used in callback-driven traversal over symbol entries, the Rust predicates produce deterministic per-entry outcomes sufficient for the surrounding traversal to reproduce the original cleanup selection behavior.

**Traceability:** `delete_level_autos`, `delete_level_statics`, linked-list traversal-related types in `src/symbol.c`

## Acceptance Notes

- The Rust port may adapt C pointer-based callback inputs into Rust-safe types, but it must preserve the same functional decisions.
- The specification is limited to evidenced behavior from `src/symbol.c` for level-based deletion predicates only.
- Any broader symbol-table management behavior must be specified by other module documents, not inferred here.