# spec.md

## Title

Functional Specification: `module_gnu_hash_entry_02` Rust Port

## Metadata

- Project: `cflow-new`
- Module: `module_gnu_hash_entry_02`
- Category: `module_cluster`
- Source file: `gnu/hash.c`
- Source functions in scope:
  - `hash_insert_if_absent`
  - `hash_remove`
  - `hash_print`
- Rust branch: `008-module_gnu_hash_entry_02-rust-port`
- Generation date: `2026-06-11`

## Overview

This module provides entry-level mutation and inspection operations for the project's hash table abstraction.

Within the analyzed scope, the module supports:

- inserting an entry only when no equal entry is already present,
- reporting the already-matched entry when insertion is skipped,
- removing an entry that matches a lookup key,
- printing the current table contents for inspection.

The Rust rewrite must preserve the observable behavior of these operations as defined by the existing hash table abstraction in `gnu/hash.c`, including success/failure signaling, duplicate-detection behavior, returned removed entries, and non-mutating print behavior.

## Feature Specification

### Feature: Conditional insertion into a hash table

The module shall support inserting a candidate entry into an existing hash table only if the table does not already contain an equal entry.

Observed behavior in scope:

- The caller provides a table, an entry to insert, and a location for reporting a matched existing entry.
- If no equal entry is present, the candidate entry is added to the table.
- If an equal entry is already present, the table remains unchanged with respect to membership for that key, and the existing entry is reported through the match output parameter.
- The operation reports status as an integer result.

The Rust version must preserve this conditional-insert semantics, including the distinction between:
- successful insertion of a previously absent entry,
- detection of an already-present equal entry,
- inability to complete the operation when the C behavior reports failure.

### Feature: Removal of a matching entry

The module shall support removing a stored entry that matches a supplied lookup key.

Observed behavior in scope:

- The caller provides a table and an entry-like key.
- If a matching stored entry exists, that stored entry is removed from the table and returned.
- If no matching entry exists, the operation reports absence by returning no entry.

The Rust version must preserve the remove-by-match behavior and must return the removed stored entry, not merely a boolean indicator.

### Feature: Table content printing

The module shall support printing the contents of a hash table for inspection.

Observed behavior in scope:

- The caller provides a table reference.
- The operation traverses and prints the table contents.
- The operation is observational and does not modify table membership.

The Rust version must provide equivalent inspection output behavior suitable for the same debugging/introspection role. Exact byte-for-byte formatting is only required if the surrounding project relies on it; otherwise, the printed output must still reflect the current table contents and remain non-mutating.

## User Scenarios & Testing

### Scenario 1: Insert a new entry into a table that does not yet contain it

A caller has an initialized hash table and submits a candidate entry with no equal entry currently stored.

Expected outcome:

- the operation reports insertion success,
- the entry becomes present in the table,
- the matched-entry output indicates that no prior equal entry blocked insertion.

Test coverage expectations:

- verify status indicates success for absent-entry insertion,
- verify subsequent lookup/removal confirms the entry is now stored,
- verify no unrelated entries are affected.

Traceability: `hash_insert_if_absent`, `hash_table`, `hash_entry`.

### Scenario 2: Attempt to insert an entry that matches an existing stored entry

A caller submits a candidate entry whose equality relation matches an entry already in the table.

Expected outcome:

- the table does not gain a duplicate logical member for that key,
- the operation reports the existing matched entry through the output parameter,
- the result distinguishes this path from successful fresh insertion.

Test coverage expectations:

- pre-populate the table with one entry,
- attempt insertion of an equal entry,
- verify stored membership count for that logical key is unchanged,
- verify the reported matched entry is the already-stored one.

Traceability: `hash_insert_if_absent`, `hash_table`, `hash_entry`.

### Scenario 3: Remove an entry that exists

A caller requests removal using a key that matches a currently stored entry.

Expected outcome:

- the matching stored entry is returned,
- the entry is no longer present afterward.

Test coverage expectations:

- insert or pre-populate a removable entry,
- remove it,
- verify returned value corresponds to the stored entry,
- verify a second removal attempt reports absence.

Traceability: `hash_remove`, `hash_table`, `hash_entry`.

### Scenario 4: Remove an entry that does not exist

A caller requests removal using a key for which the table contains no matching stored entry.

Expected outcome:

- the operation returns no entry,
- the table remains otherwise unchanged.

Test coverage expectations:

- create a table with known contents,
- remove using a non-matching key,
- verify null/none result,
- verify pre-existing entries remain available.

Traceability: `hash_remove`, `hash_table`, `hash_entry`.

### Scenario 5: Print current table contents without mutating the table

A caller invokes table printing for diagnostics.

Expected outcome:

- output is produced that reflects current contents,
- the operation does not insert, remove, or alter stored membership.

Test coverage expectations:

- populate a table with known entries,
- capture print output,
- verify output is non-empty when entries exist,
- verify before/after membership is identical.

Traceability: `hash_print`, `hash_table`, `hash_entry`.

## Requirements

### Functional Requirements

#### FR-1: Conditional insert behavior

The module shall provide an operation equivalent to `hash_insert_if_absent` that accepts a hash table, a candidate entry, and a matched-entry reporting location.

Traceability: `gnu/hash.c`, `hash_insert_if_absent`.

#### FR-2: Insert only when no equal entry exists

When no equal stored entry is present, the conditional insert operation shall add the candidate entry to the table.

Traceability: `hash_insert_if_absent`, `hash_table`, `hash_entry`.

#### FR-3: Duplicate detection and reporting

When an equal stored entry already exists, the conditional insert operation shall not create a second logical member for that key and shall report the existing entry through the matched-entry output.

Traceability: `hash_insert_if_absent`, `hash_table`, `hash_entry`.

#### FR-4: Distinct operation result signaling

The conditional insert operation shall expose result signaling sufficient to distinguish successful insertion, duplicate-match handling, and failure when the underlying operation cannot be completed.

Traceability: `hash_insert_if_absent`.

#### FR-5: Remove by matching key

The module shall provide an operation equivalent to `hash_remove` that removes a stored entry matching the supplied key from the table.

Traceability: `gnu/hash.c`, `hash_remove`.

#### FR-6: Return removed stored entry

When removal succeeds, the remove operation shall return the stored entry that was removed.

Traceability: `hash_remove`, `hash_entry`.

#### FR-7: Absence reporting on remove

When no matching entry exists, the remove operation shall report absence by returning no entry.

Traceability: `hash_remove`.

#### FR-8: Print current table contents

The module shall provide an operation equivalent to `hash_print` that outputs the current contents of the table for inspection.

Traceability: `gnu/hash.c`, `hash_print`.

#### FR-9: Print is non-mutating

The print operation shall not change table membership.

Traceability: `hash_print`, `hash_table`.

### Key Entities

#### `hash_table`

The central container entity that owns or tracks the collection of stored hash entries and is the target of insert, remove, and print operations.

Relationships:

- contains or links multiple `hash_entry` instances,
- is mutated by conditional insertion and removal,
- is read by print.

Traceability: `struct hash_table` in `gnu/hash.c`; functions `hash_insert_if_absent`, `hash_remove`, `hash_print`.

#### `hash_entry`

The entry entity representing one stored member within the hash table's internal organization.

Relationships:

- associated with a parent `hash_table`,
- compared through the table's existing matching semantics during insertion and removal,
- returned by remove when a stored match is deleted,
- traversed/read during printing.

Traceability: `struct hash_entry` references in `gnu/hash.c`; functions `hash_insert_if_absent`, `hash_remove`, `hash_print`.

#### Matched entry output

An output reference supplied to conditional insertion for reporting an already-existing entry that matched the candidate.

Relationships:

- written by conditional insertion when duplicate detection occurs,
- refers to an existing `hash_entry` payload already represented in the `hash_table`.

Traceability: `hash_insert_if_absent`.

## Success Criteria

### SC-1: Absent-entry insertion works

Given a table that does not contain an equal entry, invoking the Rust equivalent of `hash_insert_if_absent` results in the entry being present afterward and reports the insertion path rather than the duplicate path.

Traceability: `hash_insert_if_absent`.

### SC-2: Duplicate insert does not add a second logical member

Given a table that already contains an equal entry, invoking the Rust equivalent of `hash_insert_if_absent` leaves the table without an added duplicate logical member for that key and reports the matched existing entry.

Traceability: `hash_insert_if_absent`, `hash_entry`.

### SC-3: Removal returns stored entry and deletes membership

Given a table containing a removable matching entry, invoking the Rust equivalent of `hash_remove` returns that stored entry and makes a subsequent removal of the same key report absence.

Traceability: `hash_remove`.

### SC-4: Removal of absent key reports no entry and preserves other contents

Given a table that lacks a matching entry, invoking the Rust equivalent of `hash_remove` returns no entry and does not alter unrelated stored entries.

Traceability: `hash_remove`, `hash_table`.

### SC-5: Print reflects contents without mutation

Given a table with known contents, invoking the Rust equivalent of `hash_print` produces inspection output and leaves the set of stored entries unchanged before and after printing.

Traceability: `hash_print`, `hash_table`.

### SC-6: Scope fidelity

The Rust port implements the in-scope behaviors for conditional insert, remove, and print, and does not require additional public capabilities beyond those evidenced by `gnu/hash.c` for this module slice.

Traceability: `gnu/hash.c`; functions in scope.