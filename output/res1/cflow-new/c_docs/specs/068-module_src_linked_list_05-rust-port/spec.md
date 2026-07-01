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
- Rust target branch: `068-module_src_linked_list_05-rust-port`
- Generation date: `2026-06-11`

## Overview

This module provides linked-list functionality used by the project to collect and manage ordered items, including `Symbol` objects. The analyzed evidence shows two roles:

1. A general linked-list facility centered on `struct linked_list` and `struct linked_list_entry`.
2. Use of that facility from symbol-handling code to append symbols into a list.

The Rust rewrite must preserve the module’s observable behavior as a list container that can be created, dereferenced safely from an optional list handle pattern, and used by symbol code to append `Symbol` items in insertion order.

## Feature Specification

### Summary

The Rust version must implement a linked-list module that supports:

- creating a new linked list with an optional element cleanup behavior associated with the list,
- handling list references passed indirectly through a pointer-to-list pattern as seen in the source helper,
- appending `Symbol` items through symbol-related code,
- maintaining item order as items are added,
- supporting list entry traversal and cleanup behavior consistent with a list container used across the analyzed C module.

### Functional Scope

Within the analyzed source, the linked-list module is responsible for representing a mutable sequence of entries. Each entry belongs to one list, and list operations act on that shared container state.

The symbol-related usage demonstrates that the list is not only abstract infrastructure but is actively used to accumulate `Symbol` instances. Therefore, the Rust rewrite must support symbol insertion into a linked list without changing the effective semantics of accumulation.

### Included Behavior

The Rust module must preserve these evidenced behaviors:

- A list can be instantiated through a constructor equivalent to `linked_list_create`.
- Code may hold a mutable reference to an optional or indirect list handle and resolve it to a concrete list, matching the purpose of `deref_linked_list`.
- Symbol code can append a `Symbol` to a target list, matching the behavior of `append_symbol`.
- The list stores entries in append order.
- The list supports entry ownership and cleanup semantics tied to the list-level free callback evidenced by `linked_list_create(linked_list_free_data_fp fun)`.

### Excluded Behavior

The Rust rewrite must not assume or specify capabilities that are not evidenced in the analyzed module, including:

- thread-safe concurrent access,
- persistence or serialization,
- sorting, searching, deduplication, or indexing beyond list-style ordered storage,
- public APIs unrelated to list creation and append-style use,
- FFI guarantees,
- performance guarantees beyond correctness of list behavior.

## User Scenarios & Testing

### Scenario 1: Create an empty list for later population

A caller needs a list container before adding items. The caller creates a new linked list and receives an empty list handle ready for use.

**Expected behavior**
- Creation succeeds and returns an empty list object.
- The list is valid for subsequent append operations.
- The list retains any configured element cleanup behavior associated with creation.

**Testing focus**
- Construct a new list.
- Verify the list starts empty.
- Verify no entries are present before any append.

### Scenario 2: Resolve a list from an indirect handle

Internal module code receives a mutable indirect reference to a list, following the source pattern `struct linked_list **plist`. It must safely obtain the list instance to operate on it.

**Expected behavior**
- If a list is already present through the indirect handle, the operation resolves to that list.
- Resolution does not change existing list contents.
- The returned or resolved list is suitable for subsequent list operations.

**Testing focus**
- Start with an already-created list wrapped in an indirect mutable handle.
- Resolve the handle through the Rust equivalent of the helper behavior.
- Verify that appends performed afterward affect the same underlying list.

### Scenario 3: Append symbols in insertion order

Symbol-handling code adds multiple `Symbol` objects to a list as symbols are encountered.

**Expected behavior**
- Each append adds one new list entry.
- Symbols remain in the same order they were appended.
- Existing items remain intact after subsequent appends.

**Testing focus**
- Append several distinct symbols.
- Iterate or inspect the list in order.
- Verify the stored order matches append order exactly.

### Scenario 4: Clean up a populated list

A list may own data requiring release when the list is destroyed or cleared, as indicated by the constructor’s free-function argument.

**Expected behavior**
- List cleanup releases all list entries.
- If element cleanup behavior was configured, it is applied consistently to contained items.
- Cleanup of an empty list is also valid.

**Testing focus**
- Create a list with test cleanup tracking.
- Append items.
- Destroy or clear the list.
- Verify entry cleanup occurred for all inserted elements.

## Requirements

### Functional Requirements

#### FR-1: List creation
The module shall provide the ability to create a new linked list corresponding to the behavior evidenced by `linked_list_create` in `src/linked-list.c`.

**Traceability**
- `src/linked-list.c`
- `linked_list_create`

#### FR-2: Indirect list resolution
The module shall support resolving and operating on a list through an indirect mutable list handle pattern corresponding to `deref_linked_list(struct linked_list **plist)`.

**Traceability**
- `src/linked-list.c`
- `deref_linked_list`

#### FR-3: Ordered append behavior
The module shall support appending an item to the tail of a list so that insertion order is preserved for later traversal or processing.

**Traceability**
- `src/linked-list.c`
- `struct linked_list`
- `struct linked_list_entry`
- `src/symbol.c`
- `append_symbol`

#### FR-4: Symbol list integration
The Rust rewrite shall support the symbol-module use case in which a `Symbol` is appended to a linked list through symbol-handling code.

**Traceability**
- `src/symbol.c`
- `append_symbol`
- `struct linked_list`

#### FR-5: Entry-based containment
The module shall model list contents as entries associated with a parent list, reflecting the presence of both `struct linked_list` and `struct linked_list_entry` in the analyzed source.

**Traceability**
- `src/linked-list.c`
- `struct linked_list`
- `struct linked_list_entry`

#### FR-6: Element cleanup association
The module shall preserve list-level association with element cleanup behavior as evidenced by the creation function accepting `linked_list_free_data_fp fun`.

**Traceability**
- `src/linked-list.c`
- `linked_list_create`

### Key Entities

#### Linked List
A mutable container that owns or manages an ordered sequence of entries. It is the central state object used by linked-list operations and by symbol code that appends symbols.

**Traceability**
- `src/linked-list.c`
- `struct linked_list`
- `src/symbol.c`

#### Linked List Entry
A node-like record representing one contained item within a linked list. Entries link list contents to the parent container and provide the per-item structure needed for ordered append behavior.

**Traceability**
- `src/linked-list.c`
- `struct linked_list_entry`

#### Symbol
A symbol-domain object that may be inserted into a linked list through symbol-module logic.

**Traceability**
- `src/symbol.c`
- `append_symbol`

#### Element Cleanup Function
A list-associated cleanup behavior supplied when the list is created and used when releasing stored item data.

**Traceability**
- `src/linked-list.c`
- `linked_list_create(linked_list_free_data_fp fun)`

## Success Criteria

### SC-1: Creation correctness
Given a request to create a list, the Rust module creates an empty linked-list instance that can accept later appends.

**Traceability**
- `linked_list_create`
- `struct linked_list`

### SC-2: Indirect handle behavior
Given an indirect mutable list handle, the Rust module can resolve and use the underlying list without losing existing contents.

**Traceability**
- `deref_linked_list`

### SC-3: Append order preservation
Given multiple append operations, the Rust module retains items in the same order they were appended.

**Traceability**
- `append_symbol`
- `struct linked_list_entry`

### SC-4: Symbol append compatibility
Given symbol-handling code that appends `Symbol` objects to a list, the Rust module supports that workflow without requiring changed list semantics.

**Traceability**
- `src/symbol.c`
- `append_symbol`

### SC-5: Cleanup behavior preservation
Given a list created with element cleanup behavior, list teardown or equivalent cleanup processing applies that behavior to all contained elements.

**Traceability**
- `linked_list_create(linked_list_free_data_fp fun)`

### SC-6: Empty-list validity
Operations covered by this module that are valid on a newly created list remain valid when the list contains no entries.

**Traceability**
- `linked_list_create`
- `struct linked_list`

## Out of Scope

The Rust rewrite specification does not require any behavior not evidenced in the analyzed module, including:

- new collection features,
- random-access interfaces,
- thread synchronization,
- serialization formats,
- cross-module API redesign beyond what is needed to preserve the evidenced list and symbol-append behavior.