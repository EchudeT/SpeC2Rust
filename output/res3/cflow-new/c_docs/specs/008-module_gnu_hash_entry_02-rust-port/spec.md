# spec.md

## Title

Rust Functional Specification: `module_gnu_hash_entry_02`

## Document Metadata

- Project: `cflow-new`
- Module: `module_gnu_hash_entry_02`
- Category: `module_cluster`
- Source basis: `gnu/hash.c`
- Rust branch: `008-module_gnu_hash_entry_02-rust-port`
- Generation date: `2026-06-17`

## Overview

This module specifies the Rust rewrite of the hash-table entry operations represented by the source hash table implementation in `gnu/hash.c`, specifically the behaviors exposed through:

- `hash_insert_if_absent`
- `hash_remove`
- `hash_print`

The Rust version must preserve the functional behavior of these operations as a cohesive module that:

- inserts an entry only when no matching entry already exists,
- reports an existing matching entry without duplicating it,
- removes an entry when present and returns the removed stored element,
- prints the table contents for inspection.

The specification is limited to behaviors evidenced by the analyzed source and referenced data structures, chiefly `hash_table` and `hash_entry`. No additional capabilities are required beyond those behaviors.

## Feature Specification

### Summary

The module provides mutation and inspection operations over a hash table that stores entries and resolves lookup/insertion/removal in terms of entry matching within the table.

The Rust rewrite must implement the following functional boundaries:

1. **Conditional insertion**
   - Accept a candidate entry and determine whether an equivalent entry is already present in the table.
   - If no matching entry exists, insert the new entry into the table.
   - If a matching entry already exists, leave the table unchanged with respect to stored membership and make the matched stored entry available to the caller.

2. **Removal by entry match**
   - Accept an entry key/value reference used for table matching.
   - Locate a matching stored entry if one exists.
   - Remove and return the stored entry when found.
   - Return an absence result when no matching entry exists.

3. **Table content printing**
   - Traverse the table and print its stored contents in a diagnostic or inspection-oriented manner.
   - The Rust rewrite must preserve the existence of this externally visible inspection behavior, though exact formatting is only required insofar as it remains a table-content print operation consistent with the source role of `hash_print`.

### In-Scope Behavior

The Rust version must cover only the functional behavior directly evidenced by the identified functions and data structures:

- operation against a hash table state,
- interaction with stored entries through match-based table lookup,
- distinction between successful insertion and duplicate detection,
- distinction between successful removal and entry absence,
- read-only printing of table contents.

### Out of Scope

The following are not required because they are not evidenced by the provided module slice:

- defining new public operations beyond the identified behaviors,
- thread-safety guarantees,
- persistence or serialization,
- recovery or journaling behavior,
- performance guarantees beyond preserving module function,
- external interoperability layers.

## User Scenarios & Testing

### Scenario 1: Insert a new entry into a table with no matching entry

A caller has a valid hash table and a candidate entry. The caller requests insertion-if-absent.

Expected behavior:

- the module determines that no matching stored entry exists,
- the new entry becomes present in the table,
- the operation reports success as an insertion outcome,
- if the interface exposes a matched-entry output, it must not identify a different preexisting entry.

Test coverage:

- start from a table state where the candidate is absent,
- invoke conditional insertion,
- verify that a later lookup by the same match criteria would find the inserted entry,
- verify that table membership count or equivalent observable state reflects one new stored entry.

### Scenario 2: Attempt to insert an entry that already matches an existing stored entry

A caller provides a candidate entry equivalent to one already stored in the table.

Expected behavior:

- the module detects the existing matching entry,
- no duplicate stored membership is created,
- the stored matching entry is reported back to the caller when the operation provides matched-entry reporting,
- the operation result distinguishes this case from successful insertion.

Test coverage:

- populate the table with one entry,
- invoke conditional insertion with an equivalent candidate,
- verify the table still contains only the original stored membership for that match class,
- verify the matched stored entry reported is the existing one.

### Scenario 3: Remove an existing entry

A caller requests removal using an entry that matches a stored element.

Expected behavior:

- the module finds the matching stored entry,
- removes it from the table,
- returns the removed stored entry,
- the table no longer reports that entry as present afterward.

Test coverage:

- populate the table with one or more entries including the target,
- invoke removal with a matching entry,
- verify a non-absence result is returned,
- verify subsequent removal or membership check for the same entry reflects absence.

### Scenario 4: Remove a non-existent entry

A caller requests removal of an entry not stored in the table.

Expected behavior:

- the table remains unchanged,
- the operation returns an absence result.

Test coverage:

- start from a table state lacking the target,
- invoke removal,
- verify absence is returned,
- verify existing unrelated entries remain present.

### Scenario 5: Print table contents for inspection

A caller requests a printed representation of the table.

Expected behavior:

- the module traverses the current table contents,
- produces output representing stored entries,
- does not mutate table membership as a side effect of printing.

Test coverage:

- populate the table with known entries,
- invoke print,
- verify output is produced,
- verify table contents before and after printing are unchanged.

## Requirements

### Functional Requirements

#### FR-1: Conditional insert-if-absent
Traceability: `gnu/hash.c`, `hash_insert_if_absent`, `hash_table`, `hash_entry`

The module shall provide an operation that accepts a table, a candidate entry, and a way to report a matched stored entry.

#### FR-2: Duplicate detection during insertion
Traceability: `gnu/hash.c`, `hash_insert_if_absent`

When the candidate entry matches an existing stored entry, the module shall not add a duplicate stored entry for that same match.

#### FR-3: Existing-entry reporting
Traceability: `gnu/hash.c`, `hash_insert_if_absent`

When insertion is skipped because a matching stored entry already exists, the module shall report that stored matching entry to the caller through the operation’s matched-entry result path.

#### FR-4: Successful insertion outcome
Traceability: `gnu/hash.c`, `hash_insert_if_absent`

When no matching stored entry exists, the module shall insert the candidate entry into the table and report an insertion-success outcome distinguishable from duplicate detection or failure.

#### FR-5: Removal by match
Traceability: `gnu/hash.c`, `hash_remove`, `hash_table`, `hash_entry`

The module shall provide an operation that accepts a table and an entry used for matching, locates a matching stored entry, and removes it if present.

#### FR-6: Removed-entry return
Traceability: `gnu/hash.c`, `hash_remove`

When removal succeeds, the module shall return the stored entry that was removed.

#### FR-7: Absence-preserving removal
Traceability: `gnu/hash.c`, `hash_remove`

When no matching entry exists, the removal operation shall report absence and leave table membership unchanged.

#### FR-8: Printable table inspection
Traceability: `gnu/hash.c`, `hash_print`, `hash_table`

The module shall provide a table-print operation that emits a representation of the current table contents.

#### FR-9: Non-mutating print behavior
Traceability: `gnu/hash.c`, `hash_print`

Invoking the print operation shall not change which entries are stored in the table.

### Key Entities

#### `HashTable`
Traceability: `gnu/hash.c`, `struct hash_table`

Represents the hash table instance that owns or tracks stored membership and supports insertion, removal, and traversal for printing.

Relationship to other entities:

- contains or organizes multiple `HashEntry` instances,
- is the primary state object passed to all three identified module operations.

#### `HashEntry`
Traceability: `gnu/hash.c`, `struct hash_entry`

Represents an individual stored table entry or node participating in table membership and match-based operations.

Relationship to other entities:

- belongs to a `HashTable`,
- is the unit returned when a match is found during duplicate detection or removal,
- is the unit traversed or represented during printing.

#### Candidate entry / match probe
Traceability: `gnu/hash.c`, `hash_insert_if_absent`, `hash_remove`

Represents caller-provided entry data used either for attempted insertion or for locating a stored matching entry for removal.

Relationship to other entities:

- compared against stored `HashEntry` instances within a `HashTable`,
- may become a stored entry on successful insertion,
- may identify an existing stored entry without becoming newly stored.

## Success Criteria

### SC-1: Insert-if-absent correctness
Traceability: `hash_insert_if_absent`

For a table state in which no matching entry exists, invoking the insert-if-absent operation results in exactly one newly present matching entry.

### SC-2: Duplicate suppression correctness
Traceability: `hash_insert_if_absent`

For a table state in which a matching entry already exists, invoking the insert-if-absent operation leaves the number of stored entries for that match class unchanged.

### SC-3: Matched-entry reporting correctness
Traceability: `hash_insert_if_absent`

In duplicate-detection cases, the operation returns or exposes the preexisting stored matching entry rather than reporting insertion of a new one.

### SC-4: Removal correctness
Traceability: `hash_remove`

For a table state containing a matching entry, invoking removal returns a non-absence result and the same entry is no longer present afterward.

### SC-5: Removal absence behavior
Traceability: `hash_remove`

For a table state not containing a matching entry, invoking removal returns absence and does not alter the presence of unrelated entries.

### SC-6: Print behavior correctness
Traceability: `hash_print`

Invoking print on a non-empty table produces output representing stored contents.

### SC-7: Print non-mutation
Traceability: `hash_print`

For any tested table state, the set of stored entries before and after printing is identical.

## Acceptance Notes

- The Rust rewrite may adapt C pointer-style result signaling into idiomatic Rust result forms, provided all specified behaviors remain observable.
- The rewrite must not broaden module scope beyond the evidenced responsibilities of conditional insertion, removal, and printing for the hash table entry layer.