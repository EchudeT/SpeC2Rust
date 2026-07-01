# spec.md

## Title

Functional Specification for `module_src_symbol.c_34` Rust Port

## Metadata

- Project: `cflow-new`
- Source module: `src/symbol.c`
- Module category: `module_cluster`
- Target Rust branch: `097-module_src_symbol.c_34-rust-port`
- Generation date: `2026-06-17`

## Overview

This module manages symbol records used by the project’s name-tracking workflow. Its responsibilities are centered on creating symbols, assigning and changing their storage class, organizing symbols into storage-dependent collections, and removing symbols from those collections when their lifetime ends.

The Rust port must preserve the observed module behavior from `src/symbol.c`:

- installation of identifier symbols and named special-purpose symbols,
- initialization and storage reassignment of identifiers,
- maintenance of membership in internal symbol collections,
- deletion of symbols according to storage category and scope level,
- management of starter symbols, including a default starter and clearing of starter state,
- installation of target symbols.

This specification covers only functionality evidenced by the analyzed module interface and referenced internal data groupings.

## Feature Specification

### 1. Symbol lifecycle management

The module creates and initializes symbol objects for identifiers and inserts them into the internal collection appropriate to their storage class. It must support both direct initialization of an existing symbol object and creation-plus-installation for a named identifier.

The Rust version must preserve the distinction between:

- initializing a symbol’s identifier storage state, and
- installing a named identifier into module-managed symbol collections.

Traceability:
- `init_ident`
- `install_ident`

### 2. Storage-class reassignment

The module can change the storage class of an existing identifier symbol after creation. Changing storage must update the symbol’s collection membership so the symbol is no longer tracked in the old storage-specific collection and is tracked in the new one.

Traceability:
- `ident_change_storage`
- `symbol_unlink_from_list`

### 3. Category-specific symbol deletion

The module supports deletion of symbols by category and scope/lifetime:

- deletion of static symbols,
- deletion of automatic symbols at a specified level,
- deletion of parameter symbols at a specified level.

Deletion must remove symbols from module-managed collections so they are no longer returned or retained as active members of those collections.

Traceability:
- `delete_symbol`
- `delete_statics`
- `delete_autos`
- `delete_parms`

### 4. Starter symbol management

The module supports a distinct class of named “starter” symbols. It must allow installation of a starter by name, assignment of a default starter state, and clearing of starter state.

The Rust version must preserve the externally visible effects implied by these operations: starter symbols can be added, a default starter can be established, and starter-related state can be reset.

Traceability:
- `install_starter`
- `set_default_starter`
- `clear_starters`

### 5. Target symbol installation

The module supports installation of named target symbols into a target-specific collection.

Traceability:
- `install_target`

## User Scenarios & Testing

### Scenario 1: Install a normal identifier

A caller needs a symbol record for a named identifier and specifies its storage class. The module creates or installs the symbol and places it into the correct storage-managed collection.

The Rust version must support tests that verify:

- a non-null symbol object is produced for a valid name,
- the symbol is initialized with the requested storage class,
- the symbol becomes a member of the appropriate internal category after installation.

Traceability:
- `install_ident`
- `init_ident`

### Scenario 2: Change an identifier’s storage class

A caller has an existing identifier symbol whose storage classification must be updated. The module changes the classification and updates collection membership accordingly.

The Rust version must support tests that verify:

- the symbol reflects the new storage class after reassignment,
- the symbol is removed from the previous collection,
- the symbol appears only in the collection corresponding to the new storage class.

Traceability:
- `ident_change_storage`
- `symbol_unlink_from_list`

### Scenario 3: Remove static symbols

At a cleanup phase, the caller requests deletion of static symbols. The module removes all symbols tracked as static from active management.

The Rust version must support tests that verify:

- static symbols present before cleanup are no longer retained afterward,
- non-static symbols are not deleted by this operation.

Traceability:
- `delete_statics`
- `delete_symbol`

### Scenario 4: Remove automatic symbols at a scope level

When leaving a scope, the caller requests deletion of automatic symbols for a specific level. The module removes only the automatic symbols associated with that level.

The Rust version must support tests that verify:

- automatic symbols at the requested level are deleted,
- automatic symbols at other levels remain,
- symbols of other categories remain.

Traceability:
- `delete_autos`
- `delete_symbol`

### Scenario 5: Remove parameter symbols at a scope level

When parameter lifetime ends for a given level, the caller requests deletion of parameter symbols for that level. The module removes only parameter symbols associated with that level.

The Rust version must support tests that verify:

- parameter symbols at the requested level are deleted,
- parameter symbols at other levels remain,
- non-parameter symbols remain.

Traceability:
- `delete_parms`
- `delete_symbol`

### Scenario 6: Manage starter symbols

A caller installs one or more starter symbols, optionally sets default starter state, and later clears starter-related state.

The Rust version must support tests that verify:

- starter installation creates a named starter symbol,
- invoking default-starter setup produces the same starter state as the C module for the same sequence,
- clearing starters removes or resets starter tracking so prior starter state is no longer active.

Traceability:
- `install_starter`
- `set_default_starter`
- `clear_starters`

### Scenario 7: Install a target symbol

A caller registers a named target symbol. The module installs it into target tracking.

The Rust version must support tests that verify:

- a non-null symbol object is produced for a valid target name,
- the target symbol is retained in the module’s target-specific tracking set.

Traceability:
- `install_target`

## Requirements

### Functional Requirements

#### FR-1: Identifier initialization
The module shall initialize a symbol object as an identifier with a specified storage class.

Traceability:
- `init_ident`

#### FR-2: Identifier installation
The module shall install a named identifier symbol and return the installed symbol object.

Traceability:
- `install_ident`

#### FR-3: Storage change for identifiers
The module shall support changing the storage class of an existing identifier symbol.

Traceability:
- `ident_change_storage`

#### FR-4: Collection membership update on storage change
When an identifier’s storage class changes, the module shall remove the symbol from its previous storage-managed collection and add it to the collection corresponding to the new storage class.

Traceability:
- `ident_change_storage`
- `symbol_unlink_from_list`

#### FR-5: Symbol unlinking from current collection
The module shall support removing a symbol from its current linked collection membership as part of symbol reclassification or deletion.

Traceability:
- `symbol_unlink_from_list`

#### FR-6: Symbol deletion primitive
The module shall support deleting an individual symbol from module-managed tracking.

Traceability:
- `delete_symbol`

#### FR-7: Static symbol cleanup
The module shall delete symbols tracked as static when static cleanup is requested.

Traceability:
- `delete_statics`

#### FR-8: Automatic symbol cleanup by level
The module shall delete automatic symbols associated with a specified level and shall not delete automatic symbols at other levels.

Traceability:
- `delete_autos`

#### FR-9: Parameter symbol cleanup by level
The module shall delete parameter symbols associated with a specified level and shall not delete parameter symbols at other levels.

Traceability:
- `delete_parms`

#### FR-10: Starter installation
The module shall install a named starter symbol and return the installed symbol object.

Traceability:
- `install_starter`

#### FR-11: Default starter setup
The module shall support setting default starter state.

Traceability:
- `set_default_starter`

#### FR-12: Starter state clearing
The module shall support clearing starter-related tracking or state.

Traceability:
- `clear_starters`

#### FR-13: Target installation
The module shall install a named target symbol and return the installed symbol object.

Traceability:
- `install_target`

### Key Entities

#### Symbol
The central entity is the symbol object (`Symbol *` in the C module), representing a named or managed symbol participating in one or more module workflows such as identifier tracking, starter tracking, and target tracking.

Observed relationships:
- a symbol has a storage classification relevant to identifier handling,
- a symbol can be a member of an internal linked collection,
- a symbol can be deleted from module management,
- some symbol groups are filtered by scope level for cleanup operations.

Traceability:
- `init_ident`
- `install_ident`
- `ident_change_storage`
- `delete_symbol`
- `delete_autos`
- `delete_parms`

#### Storage classification
The module uses `enum storage` to classify identifier symbols for initialization, installation, reassignment, and category-based cleanup behavior.

Observed relationships:
- determines which collection an identifier belongs to,
- changing it triggers collection reassignment,
- some cleanup functions operate on storage-related categories.

Traceability:
- `ident_change_storage`
- `init_ident`
- `install_ident`

#### Linked collection membership
The module maintains internal linked-list-based collections for symbol grouping. These collections are used to organize symbols by role or storage category and support reassignment and deletion.

Observed relationships:
- a symbol may be unlinked from a current collection,
- cleanup routines traverse category-specific collections,
- starter and target management also rely on collection tracking.

Traceability:
- `symbol_unlink_from_list`
- linked-list structures at `src/symbol.c:23-28`, `40`
- linked-list-entry structures referenced in later table/list operations

#### Scope level association
Some symbols are associated with an integer level used to control cleanup of automatic and parameter symbols.

Observed relationships:
- automatic cleanup is level-specific,
- parameter cleanup is level-specific.

Traceability:
- `delete_autos`
- `delete_parms`

#### Starter symbol set
The module maintains a distinct tracked set of starter symbols and related default/cleared state.

Traceability:
- `install_starter`
- `set_default_starter`
- `clear_starters`

#### Target symbol set
The module maintains a distinct tracked set of target symbols.

Traceability:
- `install_target`

## Success Criteria

### SC-1: Identifier creation parity
For a given valid name and storage class, installing an identifier in Rust shall produce a symbol record that matches the C module’s observable postconditions for successful installation.

Traceability:
- `install_ident`
- `init_ident`

### SC-2: Storage reassignment parity
For an existing identifier symbol, changing storage in Rust shall match the C module’s observable behavior by updating the symbol’s storage classification and collection membership.

Traceability:
- `ident_change_storage`
- `symbol_unlink_from_list`

### SC-3: Static cleanup correctness
After invoking static cleanup, Rust shall leave no symbols in the static-tracked collection that would still be present in the C module after the same operation sequence.

Traceability:
- `delete_statics`
- `delete_symbol`

### SC-4: Automatic cleanup level correctness
Given symbols across multiple levels, Rust shall delete exactly the automatic symbols at the requested level and preserve automatic symbols at all other levels.

Traceability:
- `delete_autos`
- `delete_symbol`

### SC-5: Parameter cleanup level correctness
Given parameter symbols across multiple levels, Rust shall delete exactly the parameter symbols at the requested level and preserve parameter symbols at all other levels.

Traceability:
- `delete_parms`
- `delete_symbol`

### SC-6: Starter management parity
For the same sequence of starter installation, default setup, and clearing operations, Rust shall produce the same observable starter tracking state as the C module.

Traceability:
- `install_starter`
- `set_default_starter`
- `clear_starters`

### SC-7: Target installation parity
For a given valid target name, Rust shall install and retain a target symbol with the same observable module state effect as the C module.

Traceability:
- `install_target`

### SC-8: No stale collection membership after deletion or reassignment
After any supported deletion or storage reassignment operation, Rust shall not retain the affected symbol in any obsolete internal collection.

Traceability:
- `symbol_unlink_from_list`
- `ident_change_storage`
- `delete_symbol`
- `delete_statics`
- `delete_autos`
- `delete_parms`