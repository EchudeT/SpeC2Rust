# spec.md

## Title

Rust Functional Specification for `module_src_delete_level_12`

## Document Control

- **Project**: `cflow-new`
- **Module**: `module_src_delete_level_12`
- **Category**: `module_cluster`
- **Source scope**: `src/symbol.c`
- **Rust branch**: `075-module_src_delete_level_12-rust-port`
- **Generation date**: `2026-06-17`

## Overview

This module provides level-based symbol cleanup callbacks used during symbol-table traversal. Its evidenced behavior is limited to deciding whether a symbol-table entry should be deleted when compared against a target level supplied by the caller.

The Rust rewrite must preserve the observed callback behavior of:

- `delete_level_autos`
- `delete_level_statics`

These callbacks operate on symbol-table entries and call-supplied level context. They return an integer deletion decision suitable for use by surrounding collection or traversal logic in `src/symbol.c`.

## Feature Specification

### Feature: Level-matched deletion decisions for automatic-scope entries

The module must support a callback that evaluates a symbol-table entry against a requested level and reports whether the entry is eligible for deletion as an automatic-scope symbol.

#### Expected behavior

- Accept a symbol-table entry through the callback data parameter.
- Accept a caller-provided level through the callback context parameter.
- Inspect the entry’s stored level information.
- Return a deletion decision indicating whether the entry matches the requested level for automatic-scope cleanup.

### Feature: Level-matched deletion decisions for static-scope entries

The module must support a callback that evaluates a symbol-table entry against a requested level and reports whether the entry is eligible for deletion as a static-scope symbol.

#### Expected behavior

- Accept a symbol-table entry through the callback data parameter.
- Accept a caller-provided level through the callback context parameter.
- Inspect the entry’s stored level information.
- Return a deletion decision indicating whether the entry matches the requested level for static-scope cleanup.

### Feature boundaries

The Rust version must preserve only the behavior evidenced by this module slice:

- callback-style level comparison for deletion decisions
- use with symbol-table entries
- distinction between automatic and static deletion callbacks

The Rust version must not assume or introduce unsupported capabilities such as:

- independent ownership of the whole symbol table
- new public querying interfaces
- persistence, serialization, or recovery features
- concurrency guarantees
- extra deletion policies beyond the two evidenced callbacks

## User Scenarios & Testing

### Scenario 1: Remove automatic symbols for a completed scope level

A caller traverses a symbol collection after exiting a scope. For each symbol-table entry, it invokes the automatic-scope deletion callback with the target scope level.

**Expected result**

- Entries representing automatic symbols at the target level are reported as deletable.
- Entries at other levels are reported as non-deletable by this callback.

### Scenario 2: Remove static symbols for a specific level

A caller performs cleanup for static entries associated with a target level and invokes the static-scope deletion callback during traversal.

**Expected result**

- Entries representing static symbols at the target level are reported as deletable.
- Entries at other levels are reported as non-deletable by this callback.

### Scenario 3: Mixed entries in one traversal

A caller processes a set of symbol-table entries that includes multiple levels and both automatic and static categories.

**Expected result**

- The automatic callback only marks entries appropriate to automatic cleanup at the requested level.
- The static callback only marks entries appropriate to static cleanup at the requested level.
- Non-matching entries remain unselected by the respective callback.

### Scenario 4: Exact-level matching

A caller supplies a target level adjacent to, but not equal to, an entry’s stored level.

**Expected result**

- The callback returns a non-delete decision.
- No broader range-based or partial matching is applied.

### Testing guidance

The Rust port must be tested with callback inputs representing:

- an entry matching the requested level
- an entry with a different level
- automatic and static entry cases evaluated separately
- mixed collections where only a subset should be selected by each callback

Tests should verify integer or boolean-equivalent deletion decisions as required by the Rust traversal integration, while preserving the C behavior of match-based selection.

## Requirements

### Functional Requirements

#### FR-1: Automatic-level deletion callback
The module shall provide behavior equivalent to `delete_level_autos` from `src/symbol.c:274-284`, producing a deletion decision for a symbol-table entry using caller-supplied level context.

#### FR-2: Static-level deletion callback
The module shall provide behavior equivalent to `delete_level_statics` from `src/symbol.c:286-296`, producing a deletion decision for a symbol-table entry using caller-supplied level context.

#### FR-3: Symbol-table entry evaluation
The deletion decision logic shall evaluate data as a symbol-table entry consistent with the `table_entry` structures evidenced in `src/symbol.c`.

#### FR-4: Caller-provided level comparison
The deletion decision logic shall compare an entry’s stored level against the level provided through callback context, and shall base its decision on that comparison.

#### FR-5: Separate handling paths for automatic and static cleanup
The module shall preserve distinct callback behaviors for automatic-scope cleanup and static-scope cleanup, rather than collapsing both into one undocumented policy.

### Key Entities

#### `table_entry`
A symbol-table entry entity used as the subject of deletion checks. The callbacks inspect this entity to determine whether it belongs to the requested level and cleanup category.

#### linked-list-based collections
The module exists within a symbol-management area that uses linked-list collection structures in `src/symbol.c`. The deletion callbacks are intended to be usable during traversal or collection over such structures, though this module slice only evidences callback decision behavior.

#### callback context level
A caller-supplied level value passed through callback context. This value is the reference used to determine whether a symbol-table entry is selected for deletion.

#### relationship summary
- A traversal mechanism supplies one `table_entry` at a time.
- The caller also supplies a target level as callback context.
- The callback returns a deletion decision based on the relationship between the entry and that target level.
- Two callback variants exist, one for automatic cleanup and one for static cleanup.

## Success Criteria

### SC-1: Behavioral equivalence for automatic cleanup
For representative symbol-table entries and target levels, the Rust implementation of the automatic cleanup callback shall select the same entries for deletion as `delete_level_autos` in `src/symbol.c:274-284`.

### SC-2: Behavioral equivalence for static cleanup
For representative symbol-table entries and target levels, the Rust implementation of the static cleanup callback shall select the same entries for deletion as `delete_level_statics` in `src/symbol.c:286-296`.

### SC-3: Exact level discrimination
Tests shall demonstrate that entries with non-matching levels are not reported as deletable by either callback when evaluated against a different target level.

### SC-4: Distinct callback roles preserved
Tests shall demonstrate that the Rust module exposes or preserves two separate deletion-decision paths corresponding to automatic and static cleanup, matching the functional split evidenced in `src/symbol.c`.

### SC-5: Traversal-compatible callback use
The Rust implementation shall be usable from collection or traversal logic that evaluates entries one at a time with caller-provided context, consistent with the callback-style behavior evidenced by the C function signatures.