# spec.md

## Title

Functional Specification for `module_src_symbol.c_34` Rust Port

## Metadata

- Project: `cflow-new`
- Source module: `src/symbol.c`
- Module category: `module_cluster`
- Rust branch: `097-module_src_symbol.c_34-rust-port`
- Generation date: `2026-06-17`

## Overview

This module manages symbol records grouped by storage class and lifecycle context. It supports creating identifiers, moving identifiers between storage classes, removing identifiers from the active symbol sets, and maintaining special symbol groups used by the broader program flow analysis: starters and targets.

The Rust rewrite must preserve the observable behavior of the C module in these areas:

- symbol installation by name and storage class
- symbol initialization and storage reassignment
- deletion of symbols by storage/lifetime category
- maintenance of starter symbols, including a default starter selection path and reset behavior
- maintenance of target symbols
- consistent removal of deleted or reclassified symbols from the lists or tables that currently own them

This specification is limited to functionality evidenced by `src/symbol.c` and the identified functions and data structures.

## Feature Specification

### 1. Symbol lifecycle management

The module owns symbol objects representing identifiers known to the program. A symbol can be initialized with a storage class, installed into module-managed collections, later have its storage class changed, and eventually be deleted through category-specific cleanup operations.

The Rust version must implement lifecycle behavior equivalent to the C module for:

- initializing a symbol's storage-related state
- installing a new identifier by name into the appropriate managed collection
- changing an existing identifier's storage class while keeping collection membership consistent
- deleting symbols when their storage duration or scope category is cleared

### 2. Storage-class-based grouping

The module distinguishes symbols by storage class and keeps them in separate logical groups. The provided entry points show explicit handling for at least:

- static symbols
- automatic symbols, with level-based deletion
- parameter symbols, with level-based deletion

The Rust version must preserve the module’s ability to place symbols into storage-dependent collections and to remove symbols from the correct collection when their storage changes or when a scoped cleanup operation occurs.

### 3. Scoped cleanup behavior

The module provides cleanup operations for symbol categories whose lifetime is bounded by program scope.

The Rust version must support:

- deleting all static symbols managed by this module
- deleting automatic symbols associated with a supplied level
- deleting parameter symbols associated with a supplied level

The cleanup behavior must remove affected symbols from active module-managed collections so that they are no longer returned or treated as present by subsequent module operations.

### 4. Starter symbol management

The module maintains a dedicated set of starter symbols distinct from general identifiers. It supports:

- installing a starter symbol by name
- selecting a default starter through a dedicated operation
- clearing the complete starter set

The Rust rewrite must preserve the existence of this dedicated starter management behavior and its reset path.

### 5. Target symbol management

The module maintains a dedicated set of target symbols. It supports installing a target symbol by name into that set.

The Rust rewrite must preserve separate target-symbol registration behavior.

### 6. Internal unlinking consistency

The module includes explicit symbol unlinking and deletion behavior, showing that symbols may participate in one or more linked collections and must be detached before or during deletion or reclassification.

The Rust version must preserve this ownership consistency at the behavioral level: after a symbol is removed or reclassified, it must no longer remain in the prior active collection.

## User Scenarios & Testing

### Scenario 1: Install a new identifier

A caller creates or requests installation of an identifier name with a chosen storage class.

Expected behavior:

- a symbol record is created or returned as the installed identifier
- the symbol is initialized for the supplied storage class
- the symbol becomes part of the module-managed collection corresponding to that storage class

Test focus:

- installing a symbol returns a valid symbol object
- storage class after installation matches the requested value
- the symbol is present in the expected category for subsequent module operations

Traceability: `install_ident`, `init_ident`

### Scenario 2: Reclassify an existing identifier

A caller changes the storage class of an existing identifier.

Expected behavior:

- the symbol leaves the old storage-group membership
- the symbol enters the new storage-group membership
- later cleanup of the old group does not incorrectly remove the reclassified symbol

Test focus:

- storage class changes to the requested new value
- old-group cleanup no longer affects the symbol
- new-group cleanup does affect the symbol when applicable

Traceability: `ident_change_storage`, `symbol_unlink_from_list`

### Scenario 3: Clear static symbols

The broader program reaches a phase where static symbols managed by the module must be deleted.

Expected behavior:

- all symbols classified as static are removed from active module-managed storage
- symbols in unrelated categories are not removed by this operation

Test focus:

- after static deletion, previously installed static symbols are absent
- automatic, parameter, starter, and target handling remain unaffected unless evidenced by shared membership rules

Traceability: `delete_statics`, `delete_symbol`

### Scenario 4: Exit a scope and clear automatic symbols

A caller exits a scope level and requests deletion of automatic symbols for that level.

Expected behavior:

- automatic symbols associated with the supplied level are removed
- automatic symbols at other levels remain
- non-automatic symbols remain unless independently deleted

Test focus:

- level-matching automatic symbols are deleted
- level-nonmatching automatic symbols remain present

Traceability: `delete_autos`, `delete_symbol`, `collect_data`

### Scenario 5: Exit a scope and clear parameter symbols

A caller exits a scope level and requests deletion of parameter symbols for that level.

Expected behavior:

- parameter symbols associated with the supplied level are removed
- parameter symbols at other levels remain

Test focus:

- only parameters for the requested level are deleted
- repeated cleanup does not reintroduce removed symbols

Traceability: `delete_parms`, `delete_symbol`, `collect_data`

### Scenario 6: Register starter symbols

A caller installs one or more starter names used by the wider program.

Expected behavior:

- each installed starter is tracked in the module’s starter set
- starter registration is independent of general identifier cleanup operations unless explicitly shared by module rules

Test focus:

- installing starters produces starter symbol records
- multiple starter installs are retained until cleared

Traceability: `install_starter`

### Scenario 7: Set and reset default starter state

A caller requests default starter selection and later clears all starters.

Expected behavior:

- the default-starter operation establishes starter state usable by the wider program
- clearing starters removes all current starter registrations

Test focus:

- after default starter setup, starter state is non-empty or otherwise initialized as defined by module behavior
- after clearing, no previously installed starter remains active

Traceability: `set_default_starter`, `clear_starters`

### Scenario 8: Register target symbols

A caller installs target names to a dedicated target set.

Expected behavior:

- target symbols are tracked separately from starters and general identifiers
- target registration persists until cleared or removed by the owning module behavior

Test focus:

- installed targets are represented in the target set
- starter operations do not implicitly remove targets

Traceability: `install_target`

## Requirements

### Functional Requirements

#### FR-1: Identifier installation
The module shall support installing an identifier by name together with a specified storage class and returning the corresponding symbol object.

Traceability: `install_ident`

#### FR-2: Symbol initialization
The module shall support initializing symbol state for a supplied storage class before the symbol participates in managed collections.

Traceability: `init_ident`

#### FR-3: Storage reassignment
The module shall support changing the storage class of an existing symbol and shall update collection membership so the symbol is no longer retained in the prior storage-class grouping.

Traceability: `ident_change_storage`, `symbol_unlink_from_list`

#### FR-4: Symbol deletion consistency
When a symbol is deleted by this module, it shall be removed from the module-managed collection or collections that currently contain it.

Traceability: `delete_symbol`, `symbol_unlink_from_list`

#### FR-5: Static-symbol cleanup
The module shall support deleting the currently managed set of static symbols.

Traceability: `delete_statics`

#### FR-6: Automatic-symbol cleanup by level
The module shall support deleting automatic symbols associated with a caller-supplied level.

Traceability: `delete_autos`, `collect_data`

#### FR-7: Parameter-symbol cleanup by level
The module shall support deleting parameter symbols associated with a caller-supplied level.

Traceability: `delete_parms`, `collect_data`

#### FR-8: Starter installation
The module shall support installing a starter symbol by name into a dedicated starter collection.

Traceability: `install_starter`

#### FR-9: Default starter setup
The module shall support a dedicated operation that establishes default starter state.

Traceability: `set_default_starter`

#### FR-10: Starter reset
The module shall support clearing the currently managed starter collection.

Traceability: `clear_starters`

#### FR-11: Target installation
The module shall support installing a target symbol by name into a dedicated target collection.

Traceability: `install_target`

### Key Entities

#### Symbol
The primary entity managed by the module. A symbol represents a named program element and carries at least storage-related classification needed for grouping, reassignment, and deletion.

Relationships:

- belongs to one current storage-based grouping for identifier management
- may also participate in dedicated starter or target collections depending on how it is installed
- may be removed from collections during deletion or storage reassignment

Traceability: all listed symbol-management functions

#### Storage classification
A classification value associated with a symbol that determines which managed group owns it for identifier lifecycle purposes.

Relationships:

- assigned during initialization
- changed by storage reassignment
- used to determine cleanup behavior for static, automatic, and parameter symbols

Traceability: `init_ident`, `ident_change_storage`, `install_ident`, `delete_statics`, `delete_autos`, `delete_parms`

#### Linked collection membership
The module uses linked collection structures to track groups of symbols and special symbol sets.

Relationships:

- a symbol can be linked into a collection
- unlinking is required before deletion or when moving between groups
- starter and target management each rely on dedicated linked collections

Traceability: `symbol_unlink_from_list`, linked list structures at `src/symbol.c:23-28, 40, 368, 417, 456, 472, 502, 518`

#### Table entry / indexed membership
The module also uses table-entry structures to organize symbols for lookup or grouped traversal.

Relationships:

- identifier installation and cleanup traverse or update table-managed membership
- level-based deletions rely on collected traversal state

Traceability: table entry structures at `src/symbol.c:49-51, 57, 67, 68, 76, 98, 194, 239, 315, 393`; `collect_data` at `src/symbol.c:305-310, 316, 333, 347`

## Success Criteria

1. Installing an identifier with a specified storage class yields a symbol that is tracked by the module and classified with that storage.
   - Traceability: `install_ident`, `init_ident`

2. Changing a symbol’s storage class removes it from its previous storage grouping and makes it subject only to the cleanup rules of its new storage grouping.
   - Traceability: `ident_change_storage`, `symbol_unlink_from_list`

3. Running static cleanup removes all static symbols managed by the module and does not remove symbols that are only automatic or parameter symbols.
   - Traceability: `delete_statics`

4. Running automatic-symbol cleanup for level `L` removes automatic symbols at level `L` and leaves automatic symbols at other levels intact.
   - Traceability: `delete_autos`, `collect_data`

5. Running parameter-symbol cleanup for level `L` removes parameter symbols at level `L` and leaves parameter symbols at other levels intact.
   - Traceability: `delete_parms`, `collect_data`

6. Installing starter symbols records them in a starter-specific managed set, and clearing starters removes all starter entries created earlier in the session.
   - Traceability: `install_starter`, `clear_starters`

7. Invoking the default-starter operation establishes starter state without requiring manual starter installation beforehand.
   - Traceability: `set_default_starter`

8. Installing target symbols records them in a target-specific managed set that is distinct from starter management.
   - Traceability: `install_target`

9. After any deletion path used by this module, deleted symbols are no longer retained in the collection from which they were deleted.
   - Traceability: `delete_symbol`, `symbol_unlink_from_list`