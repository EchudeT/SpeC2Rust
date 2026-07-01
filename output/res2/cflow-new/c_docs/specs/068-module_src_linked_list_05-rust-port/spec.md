# spec.md

## Title

Rust Functional Specification for `module_src_linked_list_05`

## Metadata

- Project: `cflow-new`
- Module: `module_src_linked_list_05`
- Category: `module_cluster`
- Source files analyzed:
  - `src/linked-list.c`
  - `src/symbol.c`
- Rust branch target: `068-module_src_linked_list_05-rust-port`
- Generation date: 2026-06-17

## Overview

This module provides linked-list support used by the project and a symbol-specific append usage built on that list. The analyzed evidence shows:

- creation of a linked-list object with a stored data-destruction callback
- internal normalization of a possibly-null list handle into a usable list object
- appending `Symbol` items to a list through a symbol-layer helper

The Rust rewrite must preserve the functional behavior of this module as a reusable list facility for storing items and as the backing mechanism used by symbol-handling code to accumulate symbols.

## Feature Specification

### Summary

The Rust version must implement a linked-list facility that supports:

1. creating an empty list
2. retaining list-owned item cleanup behavior associated with the list
3. appending items to the list
4. allowing symbol code to append `Symbol` objects through a helper path that accepts a possibly-uninitialized list reference

### In-Scope Behavior

#### Linked-list creation

A caller can create a new linked-list object and associate it with an item-freeing function. The created list is empty and ready for later append operations.

Traceability:
- `linked_list_create` in `src/linked-list.c`

#### Null-safe list dereference/initialization path

The module includes behavior that accepts a mutable reference to a list handle and resolves it into a usable list object. If the caller has not yet provided a list instance, this path ensures a valid list object is obtained before subsequent operations continue.

Traceability:
- `deref_linked_list` in `src/linked-list.c`
- `append_symbol` in `src/symbol.c`

#### Append of symbol items

Symbol code can append a `Symbol` object into a linked list through a helper that takes a mutable list handle. This helper relies on the linked-list facility to ensure the destination list exists and then adds the symbol item.

Traceability:
- `append_symbol` in `src/symbol.c`

### Out of Scope

The analyzed input does not justify specifying:
- thread-safety guarantees
- serialization or persistence
- random access semantics
- sorting, searching, deduplication, or filtering
- public iterator contracts
- cross-language FFI
- performance guarantees beyond ordinary functional append behavior

## User Scenarios & Testing

### Scenario 1: Create an empty list for later use

A caller needs a list to hold module-managed items. The caller creates a linked list and associates item cleanup behavior with it. The returned list is valid and contains no items yet.

Required test coverage:
- Creating a list returns a valid list object.
- A newly created list is empty.
- The configured cleanup behavior is retained by the list for later ownership-based cleanup.

Traceability:
- `linked_list_create` in `src/linked-list.c`

### Scenario 2: Append a symbol through a nullable list handle

A symbol-handling path receives a mutable reference to a list handle that may not yet point to an allocated list. When a symbol is appended, the module ensures a usable list exists and stores the symbol.

Required test coverage:
- Appending to an uninitialized/absent list handle succeeds by first obtaining a valid list.
- After append, the list contains the appended symbol item.
- The list handle is updated so later operations use the same list.

Traceability:
- `deref_linked_list` in `src/linked-list.c`
- `append_symbol` in `src/symbol.c`

### Scenario 3: Append multiple symbols in order of use

A caller appends more than one `Symbol` through the symbol helper over time. The list continues to accumulate items rather than replacing earlier ones.

Required test coverage:
- Appending multiple symbols results in all appended items being present.
- Append operations preserve accumulation across repeated calls on the same list handle.

Traceability:
- `append_symbol` in `src/symbol.c`
- linked-list entry structures referenced throughout `src/linked-list.c`

## Requirements

### Functional Requirements

#### FR-1: The module shall provide linked-list creation

The Rust module shall provide behavior equivalent to `linked_list_create`, producing a new empty linked-list object and associating the caller-specified item cleanup function with that list.

Traceability:
- `linked_list_create` in `src/linked-list.c`

#### FR-2: The module shall support nullable list-handle resolution

The Rust module shall provide behavior equivalent to `deref_linked_list` for code paths that operate on a mutable list handle, yielding a usable linked-list object even when the incoming handle is not yet initialized.

Traceability:
- `deref_linked_list` in `src/linked-list.c`

#### FR-3: The module shall support append into a list-backed collection

The Rust module shall support adding an item to the linked list used by this module’s consumers, including the symbol-specific append path evidenced in the analyzed source.

Traceability:
- `append_symbol` in `src/symbol.c`
- linked-list entry structures referenced in `src/linked-list.c`

#### FR-4: The module shall support symbol append via the symbol-layer helper behavior

The Rust rewrite shall preserve the behavior where symbol code appends a `Symbol` item using a helper that accepts a mutable list reference and inserts the symbol into the linked-list-backed collection.

Traceability:
- `append_symbol` in `src/symbol.c`

#### FR-5: The module shall preserve list accumulation across repeated appends

Once a list exists, later append operations through the same logical list handle shall add additional items to that existing collection rather than discarding prior contents.

Traceability:
- `append_symbol` in `src/symbol.c`
- linked-list and linked-list entry structures referenced across `src/linked-list.c`

### Key Entities

#### LinkedList

A list object representing the collection managed by this module. It is created empty, can be resolved from a mutable handle, and stores ownership-related cleanup behavior for contained items.

Traceability:
- `struct linked_list` references in `src/linked-list.c`
- `linked_list_create`
- `deref_linked_list`

#### LinkedListEntry

An entry/node object that links stored items into the list structure. Its presence is evidenced by repeated `struct linked_list_entry` references in the linked-list source.

Traceability:
- `struct linked_list_entry` references in `src/linked-list.c`

#### Symbol

An item type that can be appended to the list through the symbol helper.

Traceability:
- `append_symbol` in `src/symbol.c`

#### Free-data callback

A function associated with a list at creation time and used to define how list-owned item data is cleaned up.

Traceability:
- parameter `linked_list_free_data_fp fun` in `linked_list_create` in `src/linked-list.c`

### Entity Relationships

- A `LinkedList` contains zero or more `LinkedListEntry` items.
- Each `LinkedListEntry` holds one stored item in the collection.
- A `LinkedList` is configured with one free-data callback governing cleanup of contained item data.
- `Symbol` values are one kind of item stored in a `LinkedList` through the symbol append helper.

## Success Criteria

### Functional Acceptance Criteria

1. **List creation works**
   - Given a request to create a list with a cleanup function, the Rust module returns a valid empty list object.
   - Traceability: `linked_list_create` in `src/linked-list.c`

2. **Nullable handle append works**
   - Given a mutable list handle with no existing list, appending a `Symbol` succeeds and leaves the handle referring to a valid list containing that symbol.
   - Traceability: `deref_linked_list` in `src/linked-list.c`, `append_symbol` in `src/symbol.c`

3. **Existing handle append accumulates**
   - Given a mutable list handle that already refers to a list, repeated symbol appends add further items to the same list without removing earlier appended items.
   - Traceability: `append_symbol` in `src/symbol.c`, linked-list entry types in `src/linked-list.c`

4. **Empty-to-nonempty transition is correct**
   - A newly created or newly resolved empty list becomes non-empty after the first successful append.
   - Traceability: `linked_list_create` in `src/linked-list.c`, `append_symbol` in `src/symbol.c`

5. **Configured cleanup behavior remains associated with the list**
   - The list object created by the Rust module retains the caller-provided cleanup behavior as part of its functional state.

### Test Completion Criteria

The Rust port is complete for this module when:
- all required scenarios in this document are implemented as tests
- each functional requirement is covered by at least one test
- the tested behavior matches the source-backed semantics described above without introducing unsupported new guarantees