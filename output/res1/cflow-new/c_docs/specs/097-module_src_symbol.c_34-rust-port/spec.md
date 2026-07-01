# spec.md

## Title

Rust Functional Specification for `module_src_symbol.c_34`

## Metadata

- Project: `cflow-new`
- Source module: `src/symbol.c`
- Module category: `module_cluster`
- Target branch: `097-module_src_symbol.c_34-rust-port`
- Generation date: `2026-06-11`

## Overview

This module manages symbol records used by the program during name installation, storage-class assignment, scope-based cleanup, and maintenance of special symbol groups. The Rust rewrite must preserve the observed functional behavior of symbol lifecycle management within `src/symbol.c`.

The module’s evidenced responsibilities are:

- creating and initializing identifier symbols,
- changing a symbol’s storage classification after creation,
- inserting symbols into module-maintained collections,
- deleting symbols directly or by category and scope,
- maintaining special starter and target symbol sets,
- clearing or resetting special symbol collections.

The Rust version must implement the same functional boundaries and must not introduce new externally visible capabilities beyond those evidenced here.

## Feature Specification

### Symbol lifecycle management

The module provides facilities to create a symbol for an identifier name, initialize it for a requested storage class, and register it into the module’s managed symbol collections.

Supported lifecycle operations include:

- installation of a new identifier symbol from a name and storage class,
- initialization of an existing symbol object to a requested storage class,
- reassignment of storage class for an existing symbol,
- removal of a symbol from the list structure that currently links it,
- destruction of an individual symbol record.

The Rust rewrite must preserve the behavior that symbols participate in category-specific collections and that changes in storage classification update that membership consistently.

### Storage-class organization

The module organizes symbols according to storage-related categories and supports cleanup by category. Evidence for this behavior is provided by initialization, storage reassignment, and deletion entry points for static, automatic, and parameter-related symbols.

The Rust version must support:

- categorizing installed identifiers by storage class,
- moving an existing symbol between storage-class categories when storage changes,
- deleting static symbols,
- deleting automatic symbols for a specified scope level,
- deleting parameter symbols for a specified scope level.

### Scope-based cleanup

The module supports deletion of symbols associated with a given nesting or scope level for at least automatic and parameter storage groups.

The Rust rewrite must preserve:

- acceptance of an integer level for automatic-symbol cleanup,
- acceptance of an integer level for parameter-symbol cleanup,
- deletion behavior limited to symbols matching the requested cleanup group and level.

### Special starter symbol management

The module maintains a distinct starter-symbol collection separate from ordinary identifier handling. It supports:

- installing a starter by name,
- selecting the default starter state,
- clearing all starter symbols.

The Rust rewrite must preserve the functional distinction between starter symbols and ordinary identifier symbols.

### Special target symbol management

The module maintains a distinct target-symbol collection. It supports installing a target by name.

The Rust rewrite must preserve the existence of this separate target registration path and its effect on the module-managed symbol collections.

## User Scenarios & Testing

### Scenario 1: Install and initialize an identifier

A caller needs a symbol for a named identifier and requests installation with a specific storage class.

Expected behavior:

- a symbol object is created or prepared for that identifier name,
- the symbol is initialized for the requested storage class,
- the symbol becomes part of the appropriate managed collection for later lookup or cleanup.

Test focus:

- installing an identifier returns a usable symbol reference,
- initialization effects are observable through subsequent storage-based operations,
- symbols installed under different storage classes are distinguishable by later cleanup routines.

### Scenario 2: Change storage classification after creation

A caller has an existing symbol and changes its storage class.

Expected behavior:

- the symbol remains valid after the change,
- its category membership reflects the new storage class rather than the old one,
- later category-based deletion acts according to the new storage class.

Test focus:

- changing storage from one class to another updates later cleanup behavior,
- the symbol is not left linked in an obsolete category list,
- repeated valid storage changes preserve consistent membership.

### Scenario 3: Delete automatic symbols for a scope level

A caller exits a scope and requests deletion of automatic symbols at that level.

Expected behavior:

- automatic symbols associated with the specified level are deleted,
- automatic symbols at other levels are retained,
- symbols in other groups are not removed by this operation.

Test focus:

- create or register automatic symbols across multiple levels,
- call automatic cleanup for one level,
- verify only matching automatic symbols are removed.

### Scenario 4: Delete parameter symbols for a scope level

A caller completes processing of a function scope and removes parameter symbols for that level.

Expected behavior:

- parameter symbols at the specified level are deleted,
- parameters at other levels, if present, are retained,
- non-parameter symbols are unaffected.

Test focus:

- parameter cleanup is level-sensitive,
- parameter cleanup is category-specific.

### Scenario 5: Delete all static symbols

A caller requests cleanup of static symbols.

Expected behavior:

- symbols categorized as static are deleted,
- symbols in non-static categories remain available.

Test focus:

- static cleanup removes static entries only,
- repeated cleanup after removal does not require remaining static symbols to exist.

### Scenario 6: Manage starter symbols

A caller registers starter names, optionally resets to the default starter state, and clears starter registrations.

Expected behavior:

- starter installation records starter symbols in the special starter collection,
- setting the default starter establishes the module’s default starter state,
- clearing starters removes current starter registrations.

Test focus:

- installed starters are tracked separately from ordinary identifiers,
- clearing starters empties that special collection,
- default-starter setup is compatible with subsequent starter operations.

### Scenario 7: Manage target symbols

A caller registers target names.

Expected behavior:

- each target is recorded in the target-specific collection,
- target registration does not substitute for ordinary identifier installation unless separately performed by the caller.

Test focus:

- target installation succeeds independently of starter handling,
- target registration remains distinct from starter registration.

## Requirements

### Functional Requirements

#### FR-1: Identifier installation

The module shall provide identifier installation by name and storage class, producing a symbol associated with the requested name and registered in module-managed symbol collections.

Traceability:

- `install_ident`
- `init_ident`

#### FR-2: Symbol initialization

The module shall initialize a symbol instance for a specified storage class so that subsequent module operations treat the symbol according to that classification.

Traceability:

- `init_ident`

#### FR-3: Storage change handling

The module shall support changing the storage class of an existing symbol and shall update the symbol’s managed-collection membership accordingly.

Traceability:

- `ident_change_storage`
- `symbol_unlink_from_list`

#### FR-4: List unlinking during membership changes

When a symbol is removed from its current managed collection as part of deletion or reclassification, the module shall unlink it from the corresponding list structure so it is no longer considered a member of that collection.

Traceability:

- `symbol_unlink_from_list`
- `ident_change_storage`
- `delete_symbol`

#### FR-5: Individual symbol deletion

The module shall support deletion of an individual symbol, including removal from module-managed collections.

Traceability:

- `delete_symbol`

#### FR-6: Static symbol cleanup

The module shall support deletion of symbols maintained in the static-symbol group.

Traceability:

- `delete_statics`

#### FR-7: Automatic-symbol cleanup by level

The module shall support deletion of automatic symbols associated with a specified integer level.

Traceability:

- `delete_autos`

#### FR-8: Parameter-symbol cleanup by level

The module shall support deletion of parameter symbols associated with a specified integer level.

Traceability:

- `delete_parms`

#### FR-9: Starter symbol installation

The module shall support installing a starter symbol by name into a starter-specific collection.

Traceability:

- `install_starter`

#### FR-10: Default starter setup

The module shall support setting the default starter state used by the starter-symbol facility.

Traceability:

- `set_default_starter`

#### FR-11: Starter collection clearing

The module shall support clearing the starter-symbol collection.

Traceability:

- `clear_starters`

#### FR-12: Target symbol installation

The module shall support installing a target symbol by name into a target-specific collection.

Traceability:

- `install_target`

### Key Entities

#### Symbol

A symbol is the central entity managed by this module. It represents a named program object or a special named entry and participates in one or more module-maintained collections.

Observed relationships:

- a symbol can be initialized with a storage class,
- a symbol can change storage class,
- a symbol can be linked into and unlinked from list-based collections,
- a symbol can be deleted individually or as part of group cleanup,
- some symbols belong to special starter or target collections.

Traceability:

- all main functions use `Symbol *`

#### Storage classification

Storage classification determines how an identifier symbol is organized and which cleanup operations apply to it.

Observed relationships:

- identifier installation requires a storage class,
- initialization requires a storage class,
- storage change reclassifies an existing symbol,
- static, automatic, and parameter deletion behaviors depend on this classification and, for some classes, scope level.

Traceability:

- `ident_change_storage`
- `init_ident`
- `install_ident`
- `delete_statics`
- `delete_autos`
- `delete_parms`

#### Linked-list collections

The module maintains multiple linked-list-based collections to organize symbols into operational groups. These lists are used for membership management, deletion, and special symbol grouping.

Observed relationships:

- symbols are linked into group-specific collections,
- symbols can be unlinked from a current collection,
- starter and target operations use separate collection paths,
- deletion routines operate over maintained collections.

Traceability:

- `symbol_unlink_from_list`
- anonymous `struct linked_list`
- anonymous `struct linked_list_entry`

#### Table-driven collection traversal

The module uses table-entry-based grouping/traversal structures to process symbol collections for installation, collection, and deletion tasks.

Observed relationships:

- multiple table-entry structures exist in the module,
- cleanup and collection-oriented functions operate over grouped symbol sets.

Traceability:

- anonymous `struct table_entry`
- `delete_statics`
- `delete_autos`
- `delete_parms`
- starter and target installation functions

#### Scope level

Scope level is an integer attribute used by cleanup operations for automatic and parameter symbols.

Observed relationships:

- automatic-symbol deletion is filtered by level,
- parameter-symbol deletion is filtered by level.

Traceability:

- `delete_autos(int level)`
- `delete_parms(int level)`

## Success Criteria

1. Installing an identifier by name and storage class produces a symbol that can subsequently participate in storage-based cleanup consistent with that storage class.
   - Traceability: `install_ident`, `init_ident`

2. Changing the storage class of an existing symbol causes later cleanup behavior to follow the new classification rather than the old one.
   - Traceability: `ident_change_storage`, `symbol_unlink_from_list`

3. Deleting static symbols removes symbols in the static group and does not require removal of symbols outside that group.
   - Traceability: `delete_statics`

4. Deleting automatic symbols for a given level removes only automatic symbols associated with that level.
   - Traceability: `delete_autos`

5. Deleting parameter symbols for a given level removes only parameter symbols associated with that level.
   - Traceability: `delete_parms`

6. Clearing starter symbols leaves the starter collection empty for subsequent module use.
   - Traceability: `clear_starters`

7. Setting the default starter state completes without preventing later starter installation or clearing.
   - Traceability: `set_default_starter`, `install_starter`, `clear_starters`

8. Installing starter symbols and installing target symbols operate through distinct functional paths and remain separately manageable.
   - Traceability: `install_starter`, `install_target`, `clear_starters`

9. Individual symbol deletion removes the symbol from module-managed collections so that later group cleanup does not continue treating it as an active member.
   - Traceability: `delete_symbol`, `symbol_unlink_from_list`

10. The Rust rewrite exposes no less functionality than the evidenced module responsibilities in `src/symbol.c` and preserves the documented behaviors for identifier, starter, and target symbol management.
   - Traceability: all listed main functions