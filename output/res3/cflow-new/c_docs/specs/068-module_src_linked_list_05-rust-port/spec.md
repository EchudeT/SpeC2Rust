# spec.md

## Title

Rust Functional Specification for `module_src_linked_list_05`

## Metadata

- Project: `cflow-new`
- Module: `module_src_linked_list_05`
- Category: `module_cluster`
- Rust branch: `068-module_src_linked_list_05-rust-port`
- Source basis: `src/linked-list.c`, `src/symbol.c`
- Generation date: 2026-06-17

## 1. Feature Specification

### Summary

This module provides a mutable linked-list facility used by the project to hold pointer-like item data and to support symbol list accumulation. The Rust rewrite must preserve the observable behavior evidenced by the source module:

- creation of a linked list with an associated data-destruction callback,
- appending items to the list,
- internal normalization of a possibly indirect list reference before use,
- use of the list by symbol-handling code to collect `Symbol` entries.

The module boundary evidenced here is limited: it is a general linked-list utility plus a symbol-specific helper that appends `Symbol *` values into such a list. The Rust version must implement those same functional responsibilities and no broader capability set should be assumed.

### In-Scope Behavior

Based on the identified functions and types, the Rust port must support:

- creating an empty linked list;
- storing item references/owned values in insertion order through append behavior;
- associating cleanup behavior for stored item data at list creation time;
- using a helper equivalent to `append_symbol` to add symbol objects to a list used by symbol logic;
- safely handling the case where list operations receive a mutable reference to a list handle rather than a direct list object, corresponding to `deref_linked_list`.

### Out-of-Scope Behavior

The following are not evidenced by the provided module analysis and must not be added as required functionality in this spec:

- random-access indexing semantics,
- sorting, searching, filtering, or deduplication,
- thread-safe or lock-free behavior,
- persistence or serialization,
- iterator APIs beyond what is required to preserve current module behavior,
- cross-language FFI guarantees,
- performance targets beyond successful functional equivalence.

## 2. User Scenarios & Testing

### Scenario 1: Create an empty list for later population

A caller initializes a linked list and provides the module with the cleanup behavior that should be applied to each stored item when the list is later torn down.

**Expected behavior**
- List creation succeeds and returns an empty list handle.
- The list is ready to receive appended items.
- The configured item cleanup behavior is retained by the list.

**Testing implications**
- Verify creation of an empty list.
- Verify the list accepts a configured data-freeing strategy.
- Verify no items are present immediately after creation.

### Scenario 2: Append symbol entries during symbol processing

Symbol-related logic accumulates `Symbol` objects into a linked list through the module-local helper represented by `append_symbol`.

**Expected behavior**
- Each appended symbol becomes part of the list.
- Repeated appends preserve insertion sequence.
- Appending through the symbol helper operates on the same linked-list abstraction created by the list module.

**Testing implications**
- Append one symbol and verify presence.
- Append multiple symbols and verify count and order.
- Verify the helper can append into a list referenced indirectly through a mutable list handle.

### Scenario 3: Operate on a list through an indirect reference

Code passes a mutable reference to a list handle rather than a direct list object. Internal logic normalizes that input before append or related list mutation.

**Expected behavior**
- The operation resolves the effective target list before mutation.
- Mutating through the indirect handle updates the intended list instance.
- No duplicate list object is created as a side effect of dereferencing.

**Testing implications**
- Exercise append through a mutable list handle/reference.
- Verify the target list reflects the added item.
- Verify identity consistency of the list before and after the operation.

### Scenario 4: Dispose of list-held data using configured cleanup behavior

The list stores items together with a cleanup strategy configured at creation. When the list is destroyed or cleared by behavior represented in the linked-list module, item cleanup must follow that strategy.

**Expected behavior**
- Stored items are released using the configured cleanup function/strategy.
- The cleanup strategy is applied to each stored element exactly once during list disposal behavior.
- The list structure itself is released after element cleanup.

**Testing implications**
- Use a test cleanup hook that records invocations.
- Append multiple items, dispose of the list, and verify one cleanup per item.
- Verify cleanup order is consistent with the module’s disposal traversal if observable from source-preserved behavior.

## 3. Requirements

### Functional Requirements

#### FR-1: Linked list creation
The Rust module shall provide behavior equivalent to `linked_list_create` from `src/linked-list.c`, creating a new empty linked list and associating caller-supplied item cleanup behavior with that list.

**Traceability:** `linked_list_create` in `src/linked-list.c`

#### FR-2: Indirect list reference resolution
The Rust module shall preserve the behavior represented by `deref_linked_list`, allowing internal operations to resolve and mutate the effective linked-list object from a mutable list reference/handle input.

**Traceability:** `deref_linked_list` in `src/linked-list.c`

#### FR-3: Append support for symbol accumulation
The Rust module shall support appending `Symbol` items to a linked list in the manner used by `append_symbol` in `src/symbol.c`.

**Traceability:** `append_symbol` in `src/symbol.c`; `struct linked_list` references in `src/symbol.c`

#### FR-4: Preservation of insertion sequence
When multiple items are appended to the same list, the Rust module shall preserve append order within the list structure.

**Traceability:** evidenced by linked-list entry structures and append-oriented usage in `src/linked-list.c` and `append_symbol` in `src/symbol.c`

#### FR-5: Element-oriented list storage
The Rust module shall represent a list as a collection of linked entries holding item data and entry linkage, matching the functional role of `struct linked_list_entry` and its relationship to `struct linked_list`.

**Traceability:** `struct linked_list`, `struct linked_list_entry` references throughout `src/linked-list.c`

#### FR-6: Configured item cleanup on list disposal behavior
Where the original linked-list module disposes stored data via a configured free callback, the Rust rewrite shall preserve equivalent cleanup semantics for stored items.

**Traceability:** `linked_list_create(linked_list_free_data_fp fun)` in `src/linked-list.c`; `struct linked_list` references in `src/linked-list.c`

### Key Entities

#### Linked List
A mutable container that owns or manages a sequence of entries and retains the cleanup behavior for each stored item.

**Traceability:** `struct linked_list` references in `src/linked-list.c` and `src/symbol.c`

#### Linked List Entry
A single node within the list that links stored item data into the overall sequence.

**Traceability:** `struct linked_list_entry` references in `src/linked-list.c`

#### Item Cleanup Function
A caller-supplied data-release behavior associated with a list at creation time and applied during list disposal behavior.

**Traceability:** `linked_list_create(linked_list_free_data_fp fun)` in `src/linked-list.c`

#### Symbol
A symbol object stored in the linked list by symbol-handling logic through the append helper.

**Traceability:** `append_symbol(struct linked_list **plist, Symbol *sp)` in `src/symbol.c`

### Entity Relationships

- A **Linked List** contains zero or more **Linked List Entry** nodes.
- Each **Linked List Entry** carries one stored item value, including `Symbol` values when used by symbol logic.
- A **Linked List** is created with one **Item Cleanup Function** that governs stored item cleanup behavior for that list.
- **Symbol** objects are appended into a **Linked List** by the symbol helper.

## 4. Success Criteria

### Functional Equivalence Criteria

1. **Empty creation**
   - Creating a list through the Rust equivalent of `linked_list_create` produces an empty list with an attached cleanup strategy.
   - Traceability: `linked_list_create` in `src/linked-list.c`

2. **Append behavior**
   - Appending one item increases list contents from zero to one.
   - Appending multiple items results in all items being present in append order.
   - Traceability: `append_symbol` in `src/symbol.c`; linked-list entry/list structures in `src/linked-list.c`

3. **Indirect mutation**
   - An operation performed through an indirect mutable list reference updates the intended underlying list rather than a detached copy.
   - Traceability: `deref_linked_list` in `src/linked-list.c`

4. **Symbol integration**
   - The Rust port supports the symbol-module use case of appending `Symbol` items into a linked list without requiring a different container abstraction.
   - Traceability: `append_symbol` and `struct linked_list` references in `src/symbol.c`

5. **Cleanup correctness**
   - For a list populated with `N` items, disposal behavior invokes the configured item cleanup behavior `N` times, once per stored item.
   - Traceability: cleanup callback parameter to `linked_list_create` in `src/linked-list.c`

6. **Structural consistency**
   - The Rust representation preserves the functional relationship of one list containing linked entries that store item data.
   - Traceability: `struct linked_list` and `struct linked_list_entry` references in `src/linked-list.c`