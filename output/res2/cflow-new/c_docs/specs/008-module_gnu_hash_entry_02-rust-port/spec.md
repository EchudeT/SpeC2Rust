# spec.md

## Title

Functional Specification: `module_gnu_hash_entry_02` Rust Port

## Document Metadata

- **Project**: `cflow-new`
- **Module**: `module_gnu_hash_entry_02`
- **Category**: `module_cluster`
- **Source file**: `gnu/hash.c`
- **Rust branch**: `008-module_gnu_hash_entry_02-rust-port`
- **Generation date**: `2026-06-17`

## 1. Feature Specification

### 1.1 Purpose

This module provides entry-level mutation and observation operations for an existing hash table abstraction:

- insert an entry only if no matching entry is already present,
- remove a matching entry if present,
- print the table contents for inspection.

The Rust rewrite must preserve the observable behavior of these operations as defined by the source module functions:

- `hash_insert_if_absent`
- `hash_remove`
- `hash_print`

### 1.2 In-Scope Functionality

The Rust version must implement the following behavior for a hash table instance that already exists and is configured with the table’s matching rules:

1. **Conditional insertion**
   - Accept a candidate entry.
   - Detect whether an equivalent entry is already stored in the table.
   - If no equivalent entry exists, insert the candidate entry and report success.
   - If an equivalent entry already exists, do not insert a duplicate and provide the already-present matching entry to the caller.

2. **Removal by entry match**
   - Accept an entry key or probe value used under the table’s existing matching semantics.
   - Remove the matching stored entry if one exists.
   - Return the removed stored entry to the caller.
   - If no match exists, report absence.

3. **Printable inspection output**
   - Produce a readable representation of the current table contents for diagnostics or debugging.
   - Operate on the current table state without mutating logical table contents.

### 1.3 Out of Scope

The following are not specified here unless required to support the listed functions:

- creation or destruction APIs,
- resizing policy as a public contract,
- thread-safety guarantees,
- serialization or persistence,
- foreign-function interfaces,
- recovery or transactional behavior,
- any public APIs beyond the three identified functions.

## 2. User Scenarios & Testing

### 2.1 Scenario: Insert a new unique entry

A caller has a hash table and an entry value not currently represented in the table.

**Expected behavior**
- Calling the conditional insert operation stores the entry.
- The operation reports insertion success.
- No preexisting match is returned.

**Testing focus**
- Start from a table without an equivalent entry.
- Invoke insertion.
- Verify table membership now includes the new entry.
- Verify the result indicates insertion rather than duplicate detection.

### 2.2 Scenario: Attempt to insert a duplicate entry

A caller provides an entry equivalent to one already stored.

**Expected behavior**
- The table remains with a single stored representative for that equivalence class.
- The operation does not insert a second copy.
- The caller can obtain the matched existing entry.

**Testing focus**
- Insert an initial entry.
- Attempt insertion of an equivalent entry.
- Verify table cardinality does not increase.
- Verify the returned matched entry corresponds to the original stored entry.

### 2.3 Scenario: Remove an existing entry

A caller wants to delete an entry known to be present.

**Expected behavior**
- The matching stored entry is removed.
- The removed entry is returned.
- A subsequent lookup-by-behavior through reinsertion semantics should behave as if the entry is absent.

**Testing focus**
- Populate the table with one or more entries.
- Remove a present entry.
- Verify the returned value is the stored entry.
- Verify a later conditional insert of an equivalent entry succeeds as a fresh insertion.

### 2.4 Scenario: Remove a non-existent entry

A caller requests removal of an entry for which no match exists.

**Expected behavior**
- No table contents are changed.
- The operation reports absence.

**Testing focus**
- Record table contents or count before removal.
- Attempt to remove an absent entry.
- Verify the result indicates no entry removed.
- Verify existing entries are unchanged.

### 2.5 Scenario: Print current table state

A caller wants diagnostic output for the current table.

**Expected behavior**
- The print operation emits a representation of the table’s current contents.
- The operation does not change membership.

**Testing focus**
- Populate the table with known entries.
- Capture output from the print operation.
- Verify output is produced and reflects current contents at a minimum sufficient for diagnostics.
- Verify table membership before and after printing is unchanged.

## 3. Requirements

### 3.1 Functional Requirements

#### FR-1: Conditional insertion
The module shall provide an operation equivalent to `hash_insert_if_absent` that attempts to insert a caller-provided entry into a hash table only when no matching stored entry already exists.

**Traceability**: `gnu/hash.c`, `hash_insert_if_absent`

#### FR-2: Duplicate suppression
When a matching entry is already present, the conditional insertion operation shall leave the table unchanged with respect to logical membership and shall not create a duplicate stored entry.

**Traceability**: `gnu/hash.c`, `hash_insert_if_absent`

#### FR-3: Matched-entry reporting on duplicate
When conditional insertion detects an existing matching entry, the module shall make that matched stored entry available to the caller.

**Traceability**: `gnu/hash.c`, `hash_insert_if_absent`

#### FR-4: Successful insertion reporting
When conditional insertion stores a previously absent entry, the module shall report success distinctly from the duplicate-detected case.

**Traceability**: `gnu/hash.c`, `hash_insert_if_absent`

#### FR-5: Removal by match
The module shall provide an operation equivalent to `hash_remove` that removes a stored entry matching the caller-provided entry according to the table’s existing matching semantics.

**Traceability**: `gnu/hash.c`, `hash_remove`

#### FR-6: Removed-entry return
When removal succeeds, the module shall return the removed stored entry.

**Traceability**: `gnu/hash.c`, `hash_remove`

#### FR-7: Absence reporting on removal
When no matching entry exists, the removal operation shall report absence and shall not alter logical table membership.

**Traceability**: `gnu/hash.c`, `hash_remove`

#### FR-8: Printable table inspection
The module shall provide an operation equivalent to `hash_print` that emits a representation of the table’s current contents suitable for inspection.

**Traceability**: `gnu/hash.c`, `hash_print`

#### FR-9: Non-mutating print behavior
The print operation shall not modify the table’s logical membership.

**Traceability**: `gnu/hash.c`, `hash_print`

### 3.2 Key Entities

#### Hash table
The central stateful collection that stores entries and defines the matching context used by insertion and removal.

**Traceability**: `struct hash_table` in `gnu/hash.c`

#### Hash entry
The stored table node/entity representing one member of the table’s contents.

**Traceability**: `struct hash_entry` references in `gnu/hash.c`

#### Caller-provided entry
An external entry value supplied to insertion or removal operations and compared against stored entries under the table’s rules.

**Traceability**: function parameters of `hash_insert_if_absent` and `hash_remove`

#### Matched stored entry
The existing stored entry identified during duplicate detection and returned or exposed to the caller when insertion is suppressed.

**Traceability**: `matched_ent` parameter in `hash_insert_if_absent`

#### Removed stored entry
The stored entry detached from the table and returned by successful removal.

**Traceability**: return value of `hash_remove`

### 3.3 Entity Relationships

- A **hash table** contains zero or more **hash entries**.
- A **caller-provided entry** is evaluated against entries already contained in the **hash table**.
- Conditional insertion either adds the **caller-provided entry** as a new member or identifies a **matched stored entry** already present.
- Removal locates a matching **hash entry**, detaches it from the **hash table**, and yields it as the **removed stored entry**.
- Printing observes the current set of **hash entries** held by the **hash table**.

## 4. Success Criteria

### 4.1 Behavioral Correctness

1. **Unique insertion succeeds**
   - Given a table with no matching entry, invoking the conditional insertion operation results in the entry being present afterward.
   - Traceability: `hash_insert_if_absent`

2. **Duplicate insertion is suppressed**
   - Given a table already containing a matching entry, invoking the conditional insertion operation does not increase logical membership count for that equivalence class.

3. **Duplicate insertion exposes existing entry**
   - In the duplicate case, the operation provides access to the already-stored matching entry.

4. **Present entry removal succeeds**
   - Given a table containing a matching entry, invoking removal returns a non-absent result and the entry is no longer present afterward.
   - Traceability: `hash_remove`

5. **Absent entry removal is side-effect free**
   - Given a table with no matching entry, invoking removal reports absence and leaves table membership unchanged.

6. **Print reflects current state without mutation**
   - For a table with known contents, invoking print produces output and does not change logical membership before versus after the call.
   - Traceability: `hash_print`

### 4.2 Port Completion Criteria

1. The Rust module exposes behavior covering all three source operations in this specification.
2. Tests exist for each user scenario in Section 2.
3. All functional requirements in Section 3.1 are covered by automated tests or direct behavioral assertions.
4. No extra public capability is claimed beyond the operations evidenced by `gnu/hash.c` for this module slice.