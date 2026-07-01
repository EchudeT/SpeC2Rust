# spec.md

## Title

Rust Functional Specification for `module_src_table_entry_06`

## Metadata

- Project: `cflow-new`
- Module: `module_src_table_entry_06`
- Category: `module_cluster`
- Source basis: `src/symbol.c`
- Target branch: `069-module_src_table_entry_06-rust-port`
- Generation date: 2026-06-11

## Overview

This module manages symbol-table entries keyed by symbol name and supports lifecycle operations on those entries. The analyzed functions show three core responsibilities:

1. locating an existing symbol by name,
2. installing a symbol into the table when needed,
3. removing and collecting symbols and symbol-associated parameter data.

The Rust rewrite must preserve this behavior as a symbol management module centered on table entries and linked-list membership. The specification is limited to functionality evidenced by `src/symbol.c`, especially `lookup`, `install`, `unlink_symbol`, `static_free`, `collect_processor`, and `delete_parms_itr`, plus the hash and comparison helpers used for name-based symbol table behavior.

## In Scope

The Rust version must implement:

- name-based symbol lookup,
- symbol installation with flags,
- symbol identity and equality by symbol name for table operations,
- unlinking a symbol from module-managed structures,
- freeing/removing symbol table entries through module-defined cleanup behavior,
- collection/filter processing over symbol table entries,
- deletion of parameter-related data during iteration where applicable.

## Out of Scope

The Rust version is not required by this specification to add:

- new public capabilities beyond the evidenced C behavior,
- thread-safety guarantees,
- persistence or serialization,
- FFI interfaces,
- recovery/rollback semantics,
- performance targets beyond preserving functional behavior.

## Feature Specification

### Feature 1: Name-based symbol table access

The module provides symbol table access keyed by a symbol name string.

Behavior evidenced by:
- `hash_symbol_hasher`
- `hash_symbol_compare`
- `lookup`

Required Rust behavior:
- Given a symbol name, the module can search the managed symbol table and return the corresponding symbol entry if it exists.
- Equality for symbol table matching must be based on symbol name content, as implied by the dedicated hash and compare helpers operating on table entries.
- Lookup must not create a new symbol entry when no matching name exists.

### Feature 2: Symbol installation

The module provides installation of a symbol into the managed table.

Behavior evidenced by:
- `install`

Required Rust behavior:
- The module accepts a symbol name and flags and returns the corresponding managed symbol.
- If a symbol of that name is already present, installation must resolve to the existing logical symbol entry rather than creating a duplicate name entry.
- If no symbol of that name exists, installation must create and register a new symbol entry that becomes discoverable by later lookup.
- Installed symbols must retain their associated flags as part of module-managed symbol state.

### Feature 3: Symbol unlinking and removal support

The module supports removing a symbol from linked/module-managed membership structures.

Behavior evidenced by:
- `unlink_symbol`

Required Rust behavior:
- A managed symbol can be detached from the module structures that track active membership.
- After unlinking/removal, the symbol must no longer appear through the table/list traversal paths that use those structures.
- Unlinking must be consistent with later cleanup operations so that removed entries are not retained as active members.

### Feature 4: Cleanup of static or collected symbol entries

The module defines cleanup behavior for stored symbol table data.

Behavior evidenced by:
- `static_free`
- `collect_processor`

Required Rust behavior:
- The module must support internal cleanup of symbol table entries through a module-defined free/remove path.
- It must support processing entries during collection/traversal using predicate or processor logic that can decide whether an entry is selected for collection-related action.
- Collected/selected entries must be handled in a way consistent with cleanup/removal responsibilities evidenced by the C module.

### Feature 5: Parameter-deletion iteration support

The module contains iteration-time deletion logic for parameter-associated data.

Behavior evidenced by:
- `delete_parms_itr`

Required Rust behavior:
- The module must support an iteration callback or equivalent internal traversal action that removes parameter-related state from applicable entries.
- The deletion behavior must be applicable during collection or traversal over module-managed entries.
- Parameter deletion must leave remaining symbol entries in a valid state for continued table/list management.

## User Scenarios & Testing

### Scenario 1: Look up an existing symbol

A caller needs to resolve a symbol name that may already be known to the module.

Expected behavior:
- When the symbol has already been installed, lookup returns that symbol entry.
- The returned symbol corresponds to the same logical name previously installed.
- No duplicate symbol is created during lookup.

Test coverage:
- Install a symbol with a given name.
- Perform lookup with the same name.
- Verify that lookup succeeds and refers to the existing logical entry.

### Scenario 2: Look up a missing symbol

A caller checks whether a symbol name is present before creating it.

Expected behavior:
- Lookup of a name not present in the table returns no symbol.
- The lookup operation does not mutate the table.

Test coverage:
- Ensure a name is absent.
- Call lookup.
- Verify absence is reported and entry count/membership is unchanged.

### Scenario 3: Install a new symbol

A caller introduces a new symbol name with associated flags.

Expected behavior:
- Installation creates a managed symbol entry when the name is new.
- The entry becomes available through later lookup.
- The installed symbol retains the provided flags.

Test coverage:
- Install a previously unseen name with flags.
- Verify lookup can find it afterward.
- Verify the stored symbol state reflects the provided flags.

### Scenario 4: Install an already-known symbol

A caller installs a symbol name that is already present.

Expected behavior:
- The module does not create a second active entry for the same name.
- Installation returns the existing logical symbol.

Test coverage:
- Install a name once.
- Install the same name again.
- Verify only one logical symbol exists for that name and subsequent lookup resolves to it.

### Scenario 5: Remove or unlink a symbol from active structures

A caller or internal module path removes a symbol from active membership structures.

Expected behavior:
- Once unlinked, the symbol no longer appears in normal active lookup/traversal paths driven by those structures.
- Cleanup can proceed without leaving the symbol as an active member.

Test coverage:
- Install a symbol.
- Unlink/remove it through the module’s removal path.
- Verify it is absent from active retrieval/traversal afterward.

### Scenario 6: Collect or process symbols with selection logic

The module traverses entries and applies collection logic to determine which entries should be selected for later action.

Expected behavior:
- Traversal visits managed entries.
- The processor receives each entry and collection context.
- Selection decisions are applied consistently to the relevant entries.

Test coverage:
- Populate the module with multiple entries.
- Run collection traversal with a controlled selection condition.
- Verify only matching entries are selected for follow-on cleanup or processing.

### Scenario 7: Delete parameter-related data during iteration

The module removes parameter-related data from entries as part of an iterative cleanup path.

Expected behavior:
- Parameter-related state on applicable entries is deleted.
- The deletion path does not invalidate the module’s ability to continue managing the remaining symbol entries.

Test coverage:
- Create entries with parameter-associated state.
- Execute the parameter-deletion iteration path.
- Verify parameter data is removed and remaining symbol management operations still function.

## Requirements

### Functional Requirements

#### FR-1: Symbol lookup by name
The module shall provide name-based lookup for symbols stored in the managed symbol table.

Traceability:
- `lookup`
- `hash_symbol_hasher`
- `hash_symbol_compare`

#### FR-2: Name equality for table matching
The module shall treat symbol table identity/matching according to symbol name content for purposes of lookup and installation.

Traceability:
- `hash_symbol_hasher`
- `hash_symbol_compare`
- `table_entry`

#### FR-3: Non-creating lookup
The module shall allow lookup of a missing symbol name without creating a new symbol entry.

Traceability:
- `lookup`

#### FR-4: Install new symbol entries
The module shall create and register a symbol entry when installation is requested for a name not already present.

Traceability:
- `install`
- `table_entry`

#### FR-5: Reuse existing symbol entries on duplicate installation
The module shall avoid creating duplicate active symbol entries for the same symbol name during installation.

Traceability:
- `install`
- `lookup`
- `table_entry`

#### FR-6: Preserve symbol flags on installation
The module shall associate installation flags with the managed symbol entry.

Traceability:
- `install`

#### FR-7: Support symbol unlinking from managed membership structures
The module shall support detaching a symbol from the linked/module-managed structures in which it participates.

Traceability:
- `unlink_symbol`
- linked-list-related anonymous structures

#### FR-8: Support cleanup/free behavior for symbol table entries
The module shall provide internal cleanup behavior for symbol table entries that are being removed or reclaimed.

Traceability:
- `static_free`
- `table_entry`

#### FR-9: Support collection traversal with processor logic
The module shall support traversal over managed entries where a processor receives entry data and collection context and returns a boolean result used by collection logic.

Traceability:
- `collect_processor`
- `collect_data`
- `table_entry`

#### FR-10: Support deletion of parameter-related state during iteration
The module shall support iterative deletion of parameter-related data associated with symbol entries.

Traceability:
- `delete_parms_itr`
- `table_entry`

#### FR-11: Maintain valid symbol management state after unlinking or parameter deletion
The module shall leave remaining managed entries in a valid state after unlinking, cleanup, or parameter-deletion operations.

Traceability:
- `unlink_symbol`
- `static_free`
- `delete_parms_itr`

### Key Entities

#### Symbol
The primary managed semantic object returned by lookup and installation. A symbol is keyed by name and carries module-managed state including installation flags.

Traceability:
- `lookup`
- `install`
- `unlink_symbol`

#### Table Entry
The storage unit used by the symbol table and helper functions for hashing, comparison, cleanup, collection, and iterative deletion. A table entry represents or contains a symbol and participates in table operations.

Traceability:
- `table_entry`
- `hash_symbol_hasher`
- `hash_symbol_compare`
- `static_free`
- `collect_processor`
- `delete_parms_itr`

#### Linked-list membership structures
The module uses linked-list-style structures to track symbol participation in one or more internal lists. These structures are relevant to unlinking and traversal-oriented behaviors.

Traceability:
- anonymous `struct linked_list`
- anonymous `struct linked_list_entry`
- `unlink_symbol`

#### Collection context
A collection-specific context object accompanies traversal/processing and carries data needed by collection logic.

Traceability:
- `collect_data`
- `collect_processor`

## Success Criteria

### SC-1: Correct existing-symbol lookup
Given a symbol previously installed with a specific name, lookup of that same name returns the existing logical symbol.

Traceability:
- `install`
- `lookup`

### SC-2: Correct missing-symbol lookup
Given a name not present in the symbol table, lookup reports absence and does not create a new entry.

Traceability:
- `lookup`

### SC-3: Correct installation of new symbols
Installing a previously unknown symbol name creates a managed entry that is subsequently discoverable by lookup.

Traceability:
- `install`
- `lookup`

### SC-4: No duplicate active entries for duplicate installation
Repeated installation requests for the same symbol name do not produce multiple active symbol entries for that name.

Traceability:
- `install`
- `hash_symbol_compare`

### SC-5: Installation flags are retained
A symbol installed with a flags value preserves that value in its managed state after installation.

Traceability:
- `install`

### SC-6: Unlinked symbols are removed from active membership
After a symbol is unlinked through the module’s removal path, it no longer appears through active module membership/traversal paths.

Traceability:
- `unlink_symbol`

### SC-7: Cleanup path handles stored entries without leaving active remnants
When the module cleanup/free path is applied to an entry, the entry is no longer treated as an active managed symbol-table member.

Traceability:
- `static_free`
- `table_entry`

### SC-8: Collection processor supports selective traversal behavior
When collection traversal is run with a defined selection condition, the processor’s boolean result is reflected in which entries are selected for subsequent action.

Traceability:
- `collect_processor`
- `collect_data`

### SC-9: Parameter-deletion iteration removes parameter-related state
After the iterative parameter-deletion path is executed on applicable entries, their parameter-related state is removed while the module remains usable for remaining entries.

Traceability:
- `delete_parms_itr`

## Acceptance Notes

- The Rust rewrite may change internal representations, but it must preserve the functional behavior described above.
- Public-facing behavior must remain centered on symbol-name lookup, installation, unlink/removal support, cleanup support, collection processing, and parameter-deletion iteration as evidenced by `src/symbol.c`.
- Any behavior not evidenced by the analyzed functions or referenced data structures is intentionally unspecified.