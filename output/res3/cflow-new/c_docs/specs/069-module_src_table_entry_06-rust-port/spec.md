# spec.md

## Title
Rust Port Functional Specification for `module_src_table_entry_06`

## Status
Draft

## Scope
This specification defines the functional behavior to preserve when rewriting the symbol-table module from `src/symbol.c` into Rust for branch `069-module_src_table_entry_06-rust-port`.

The module is responsible for managing named symbol records stored in a hash-backed table, supporting symbol lookup, installation, removal from table membership, selective collection, and cleanup of parameter-related data associated with symbols.

## References
- Project: `cflow-new`
- Source module: `src/symbol.c`
- Main traced functions:
  - `hash_symbol_hasher`
  - `hash_symbol_compare`
  - `lookup`
  - `install`
  - `unlink_symbol`
  - `static_free`
  - `collect_processor`
  - `delete_parms_itr`

---

## 1. Feature Specification

### 1.1 Summary
The Rust module must provide the same observable symbol-table behavior as the C module:

- identify symbols by name,
- retrieve existing symbols by name,
- install symbols into table storage,
- avoid duplicate logical entries for the same symbol name,
- detach symbols from table membership,
- free or clear stored symbol-related state during cleanup,
- support iteration-time collection/filtering behavior used to gather symbols,
- support deletion of parameter-associated symbol state.

### 1.2 In-Scope Behavior
The Rust version must implement the following functional areas evidenced by the source module:

1. **Name-based symbol hashing and equality**
   - Symbols are table entries keyed by symbol name.
   - Table matching behavior is based on symbol-name equality.
   - Hashing behavior must be compatible with name-keyed storage semantics.

2. **Lookup of installed symbols**
   - Given a symbol name, the module returns the matching installed symbol if present.
   - If no matching symbol exists, lookup returns an absence result.

3. **Installation of symbols**
   - Given a name and flags, the module installs a symbol into the table.
   - Installation must produce a symbol record associated with the provided name.
   - When a symbol of the same name is already present, installation must preserve the module’s single-entry-by-name behavior rather than creating an unrelated duplicate visible to lookup.

4. **Unlinking/removal from table membership**
   - A symbol can be detached from the table structure so it is no longer found through normal lookup.
   - Removal behavior applies to table membership, not merely to a local reference.

5. **Static cleanup of stored symbol entries**
   - The module can process stored symbol entries for cleanup/freeing.
   - Cleanup must correctly handle the symbol entry container and associated symbol-owned state that this module is responsible for.

6. **Selective collection during traversal**
   - The module supports traversal-time processing in which candidate entries are examined and selected according to collector state.
   - The collector pathway must support accumulating qualifying symbol entries into a result set or list used by the caller.

7. **Deletion of parameter-related symbol data**
   - The module provides iteration-time logic for deleting parameter-associated data from symbol entries.
   - This behavior must be applicable across a set of entries, not only a single explicitly named symbol.

### 1.3 Out of Scope
The Rust rewrite must not add requirements not evidenced by the source analysis, including:
- thread-safety guarantees,
- persistence or serialization,
- FFI contracts,
- recovery/journaling behavior,
- new public APIs beyond what is needed to preserve module behavior,
- performance targets beyond preserving functional semantics.

---

## 2. User Scenarios & Testing

### Scenario 1: Install and retrieve a symbol
A caller installs a symbol using a name and flag value, then performs lookup using the same name.

**Expected result**
- Lookup returns the installed symbol.
- The returned symbol corresponds to the installed name.

**Traceability**
- `install`
- `lookup`

### Scenario 2: Repeated lookup for a missing symbol
A caller requests lookup for a name that has not been installed.

**Expected result**
- Lookup reports absence.
- No new symbol is created as a side effect of lookup alone.

**Traceability**
- `lookup`

### Scenario 3: Reinstall an existing symbol name
A caller installs a symbol name that is already present in the table.

**Expected result**
- The module maintains one logical lookup target for that name.
- Subsequent lookup by that name resolves to the existing installed symbol behavior rather than exposing multiple conflicting entries.

**Traceability**
- `install`
- `lookup`
- `hash_symbol_compare`

### Scenario 4: Remove a symbol from table visibility
A symbol has been installed, then is unlinked from the table structure.

**Expected result**
- After unlinking, normal lookup by that symbol name no longer finds the symbol through the table.

**Traceability**
- `install`
- `unlink_symbol`
- `lookup`

### Scenario 5: Cleanup of stored entries
The module processes installed entries through its cleanup path.

**Expected result**
- Entry-owned resources handled by this module are released or cleared.
- Cleanup completes without leaving the table entry in an active installed state.

**Traceability**
- `static_free`

### Scenario 6: Collect matching symbols during iteration
A caller invokes module logic that traverses table entries and uses collector state to gather relevant symbols.

**Expected result**
- Entries are examined one by one.
- Qualifying symbols are added to the caller’s collection state.
- Non-qualifying entries are skipped.

**Traceability**
- `collect_processor`
- `collect_data`

### Scenario 7: Delete parameter-associated data across entries
A caller applies the parameter-deletion iterator across symbol entries.

**Expected result**
- Parameter-related state associated with each processed symbol is deleted or cleared according to module rules.
- The iteration callback returns a status value suitable for continued traversal behavior expected by the caller.

**Traceability**
- `delete_parms_itr`

### Suggested Test Coverage
The Rust port should include tests for:

1. successful install followed by lookup,
2. lookup miss for unknown name,
3. repeated install of same name preserving single lookup identity,
4. unlink followed by lookup miss,
5. cleanup path handling installed entries,
6. collector path selecting some entries and rejecting others,
7. parameter-deletion callback modifying processed symbol state as expected.

---

## 3. Requirements

### 3.1 Functional Requirements

#### FR-1: Name-keyed symbol identity
The module shall treat symbol names as the key used for table hashing and equality comparison.

**Traceability**
- `hash_symbol_hasher`
- `hash_symbol_compare`
- `table_entry`

#### FR-2: Symbol lookup
The module shall provide a lookup operation that returns the symbol associated with a given name when that name exists in the table, and an absence result when it does not.

**Traceability**
- `lookup`

#### FR-3: Symbol installation
The module shall provide an installation operation that places a symbol identified by a provided name into the table and associates installation-time flags with the symbol state.

**Traceability**
- `install`

#### FR-4: No conflicting duplicate lookup entries
The module shall preserve consistent lookup behavior for a name already present in the table, so that lookup by name resolves to one logical symbol entry rather than multiple conflicting visible entries.

**Traceability**
- `install`
- `lookup`
- `hash_symbol_compare`

#### FR-5: Table unlinking
The module shall support removing a symbol from table membership such that subsequent table lookup no longer returns that symbol.

**Traceability**
- `unlink_symbol`
- `lookup`

#### FR-6: Entry cleanup
The module shall support cleanup/free processing for stored symbol table entries and the symbol-owned resources handled by this module.

**Traceability**
- `static_free`
- `table_entry`

#### FR-7: Iteration-based collection
The module shall support processing entries through a collector callback that evaluates each entry using caller-provided collection state and records qualifying entries into that state.

**Traceability**
- `collect_processor`
- `collect_data`
- `table_entry`

#### FR-8: Parameter-data deletion by iterator
The module shall support iteration-time deletion or clearing of parameter-related symbol data for processed entries.

**Traceability**
- `delete_parms_itr`
- `table_entry`

### 3.2 Key Entities

#### Symbol
The primary named record managed by the module. A `Symbol` is the object returned by lookup and installation operations and is the subject of unlinking, collection, cleanup, and parameter-data deletion.

**Relationships**
- Stored within or referenced by a table entry.
- Identified by a name key.
- Carries flags supplied during installation.

**Traceability**
- `lookup`
- `install`
- `unlink_symbol`

#### Table Entry
The hash-table resident wrapper or node that binds name-keyed symbol identity to table storage and traversal operations.

**Relationships**
- Contains or references a `Symbol`.
- Participates in hashing and equality comparison.
- Is processed by cleanup and collector callbacks.

**Traceability**
- `table_entry`
- `hash_symbol_hasher`
- `hash_symbol_compare`
- `static_free`
- `collect_processor`
- `delete_parms_itr`

#### Linked List / Linked List Entry
Internal list structures used by the module to organize associated symbols or collected results.

**Relationships**
- Used in support of collection and symbol-associated grouping.
- Provide traversal or membership structure for entry sets.

**Traceability**
- anonymous `struct linked_list`
- anonymous `struct linked_list_entry`
- `collect_processor`
- `delete_parms_itr`

#### Collection State
Caller-provided or traversal-provided state that guides selection of entries during collection and receives collected results.

**Relationships**
- Consumed by the collector callback.
- Updated as qualifying entries are processed.

**Traceability**
- `collect_data`
- `collect_processor`

---

## 4. Success Criteria

### SC-1: Lookup correctness
For any symbol installed under a name, lookup of that exact name returns that symbol; lookup of a name never installed returns absence.

**Traceability**
- `install`
- `lookup`

### SC-2: Stable name-based uniqueness
When the same name is installed more than once, the module preserves a single logical lookup result for that name and does not expose conflicting multiple matches through lookup.

**Traceability**
- `install`
- `lookup`
- `hash_symbol_compare`

### SC-3: Removal correctness
After a symbol is unlinked from the table, lookup by its former name no longer returns it.

**Traceability**
- `unlink_symbol`
- `lookup`

### SC-4: Cleanup coverage
Running the module’s cleanup path over stored entries releases or clears module-owned entry state without leaving cleaned entries discoverable as active table members.

**Traceability**
- `static_free`
- `table_entry`

### SC-5: Collection behavior
During traversal-based collection, qualifying entries are added to collector state and non-qualifying entries are not added.

**Traceability**
- `collect_processor`
- `collect_data`

### SC-6: Parameter-data deletion behavior
Applying the parameter-deletion iterator to applicable entries removes or clears parameter-related state for those entries.

**Traceability**
- `delete_parms_itr`

### SC-7: Behavior preservation
All scenarios in Section 2 are supported by the Rust port with outcomes matching the C module’s functional behavior.

**Traceability**
- `lookup`
- `install`
- `unlink_symbol`
- `static_free`
- `collect_processor`
- `delete_parms_itr`