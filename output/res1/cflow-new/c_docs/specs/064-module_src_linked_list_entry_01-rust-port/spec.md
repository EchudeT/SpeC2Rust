# spec.md

## Title

Rust Port Functional Specification: `module_src_linked_list_entry_01`

## Document Metadata

- Project: `cflow-new`
- Module: `module_src_linked_list_entry_01`
- Category: `module_cluster`
- Target Rust branch: `064-module_src_linked_list_entry_01-rust-port`
- Source basis: `src/dot.c`, `src/linked-list.c`, `src/main.c`, `src/output.c`
- Generation date: `2026-06-11`

## Overview

This module provides two closely related areas of behavior:

1. A generic linked-list facility used to store opaque data pointers, support ordered insertion, removal, traversal, membership checks, and size calculation.
2. Output support that consumes linked-list content when rendering symbol-related information, including reference lists, tree-style output, and DOT symbol printing.

The Rust rewrite must preserve the observable behavior evidenced by the analyzed C module, including list mutation semantics, traversal semantics, membership and counting behavior, and the output decisions that depend on linked-list entry state.

## Feature Specification

### Linked-list management

The module must provide linked-list behavior for storing opaque per-entry data and managing a sequence of entries through the following supported operations:

- append a new data item to the end of a list
- prepend a new data item to the beginning of a list
- destroy a list and release all list nodes, while operating through the caller’s list handle
- unlink a specific entry from a list without requiring data-based lookup
- iterate through list entries using a callback and caller-supplied context
- test whether a given data pointer is present in a list
- compute the number of entries in a list

The C interface shows that list creation is implicit through append/prepend when the caller passes an addressable list handle. The Rust version must preserve that functional capability, even if represented idiomatically.

### Traversal with mutation-aware progression

The iteration behavior must support visiting list entries in sequence while invoking a caller callback for each entry’s stored data. Because the C implementation exposes both iteration and unlinking within the same module, the Rust rewrite must preserve safe and correct traversal semantics for the supported usage pattern where iteration progresses entry-by-entry and does not lose track of the remaining list.

No additional traversal modes are required beyond the evidenced callback-driven iteration.

### List-driven output behavior

The module participates in user-visible output generation in three evidenced ways:

- printing a named reference list
- deciding whether individual linked-list entries are printable and whether they are the last relevant printable item for tree formatting
- producing direct and inverted tree output that walks symbol relationships and formats them using list-derived child/reference ordering
- printing DOT-formatted symbol information for an output symbol at a given line number

The Rust version must preserve the output-relevant decisions made from linked-list content, especially those involving printable filtering and “last item” determination used by tree formatting.

### Duplicate-registration check support

The module cluster includes a registration helper that records device/inode pairs for option-file tracking. The analyzed evidence only supports the requirement that duplicate identity checks be handled by membership-oriented list use. The Rust rewrite must preserve the functional outcome: a device/inode identity can be checked against previously registered identities so repeat registration can be detected consistently.

## User Scenarios & Testing

### Scenario 1: Build a list of opaque items in order

A caller starts with no list, prepends or appends several items, and then queries the list size.

**Expected behavior**
- appending preserves insertion at the tail
- prepending preserves insertion at the head
- size reflects the number of inserted entries
- list operations work when starting from an initially absent/empty list handle

**Testing focus**
- append to empty list, then append again
- prepend to empty list, then prepend again
- mix prepend and append and verify resulting order through iteration
- verify size after each mutation

### Scenario 2: Remove a known entry from an existing list

A caller holds a reference to a particular list entry and requests unlinking that entry from the list.

**Expected behavior**
- the target entry is removed from the list sequence
- remaining entries preserve their relative order
- subsequent size calculation reflects the removal
- removed entry no longer appears during iteration or membership checks based on its stored data

**Testing focus**
- unlink first entry
- unlink middle entry
- unlink last entry
- verify list continuity after each case

### Scenario 3: Iterate over list data to drive higher-level behavior

A caller supplies a callback and context object to process each item in the list.

**Expected behavior**
- the callback is invoked once per visited entry in list order
- each invocation receives the stored data pointer/value and the caller context
- iteration completes over the whole list under normal callback behavior

**Testing focus**
- collect visited values into an external buffer
- verify visit order after append/prepend combinations
- verify iteration over empty list performs no callback invocations

### Scenario 4: Check whether a data object is already present

A caller needs to avoid duplicate processing and asks whether a specific stored data object is already in the list.

**Expected behavior**
- membership returns true when the same stored data object is present
- membership returns false when it is absent
- result changes appropriately after unlink or destroy

**Testing focus**
- present item
- absent item
- repeated equal-by-identity checks after list mutation

### Scenario 5: Destroy a list after use

A caller finishes using a list and destroys it through the caller-owned list handle.

**Expected behavior**
- all list entries are removed
- the caller-visible list handle becomes empty/reset
- later size checks report zero / empty behavior
- no stale entries remain available for iteration

**Testing focus**
- destroy populated list
- destroy empty list
- destroy after prior unlink operations

### Scenario 6: Print a named reference list

A caller has a symbol name and an associated list of references and asks the module to print them.

**Expected behavior**
- output includes the provided name
- output walks the reference list content in list order
- empty and non-empty reference lists are handled without corrupt formatting

**Testing focus**
- print with empty reference list
- print with one reference
- print with multiple references in known order

### Scenario 7: Produce tree output with correct branch termination

A caller requests tree-form output for symbol relationships.

**Expected behavior**
- entries considered printable participate in tree rendering
- non-printable entries are excluded from visible branch decisions
- “last item” logic reflects the last printable entry, not merely the physically last linked-list node
- both direct and inverted tree views preserve this behavior

**Testing focus**
- list where all entries are printable
- list where trailing entries are non-printable
- list where only one printable entry exists
- compare branch markers/termination placement in direct and inverted output modes

### Scenario 8: Print DOT symbol output for a symbol and line

A caller provides an output symbol and source line for DOT rendering.

**Expected behavior**
- the symbol is emitted in DOT-related output form
- linked-list-backed symbol/reference state used by the formatter is reflected consistently

**Testing focus**
- symbol with minimal associated state
- symbol with linked-list-backed relationships affecting output

### Scenario 9: Reject duplicate option-file identity registration

A caller attempts to register a device/inode identity more than once.

**Expected behavior**
- first registration is accepted into tracked state
- repeated registration of the same identity is detected consistently
- distinct identities remain independently registerable

**Testing focus**
- first unique identity
- duplicate identity
- multiple unique identities followed by duplicate of one prior entry

## Requirements

### Functional Requirements

#### FR-1: Generic list insertion
The module shall support inserting opaque data items at both the front and the back of a linked list.
**Traceability:** `linked_list_append`, `linked_list_prepend` in `src/linked-list.c`.

#### FR-2: Empty-list bootstrap through insertion
The module shall allow insertion operations to initialize list state when the caller begins with an empty or absent list handle.
**Traceability:** pointer-to-list usage in `linked_list_append`, `linked_list_prepend` in `src/linked-list.c`.

#### FR-3: List destruction
The module shall support destroying a list through the caller’s list handle so that list nodes are removed and the caller-observable handle becomes empty/reset.
**Traceability:** `linked_list_destroy` in `src/linked-list.c`.

#### FR-4: Entry unlinking by entry identity
The module shall support removing a specific linked-list entry from an existing list when that entry is directly identified.
**Traceability:** `linked_list_unlink` in `src/linked-list.c`; `linked_list_entry` type references.

#### FR-5: Ordered callback iteration
The module shall support iterating over list entries in sequence and invoking a caller-supplied callback with each entry’s stored data and caller context.
**Traceability:** `linked_list_iterate` in `src/linked-list.c`.

#### FR-6: Membership testing
The module shall support testing whether a given stored data object is present in a linked list.
**Traceability:** `data_in_list` in `src/linked-list.c`.

#### FR-7: Entry counting
The module shall support computing the number of entries currently present in a linked list.
**Traceability:** `linked_list_size` in `src/linked-list.c`.

#### FR-8: Reference-list printing
The module shall support printing a named set of references from linked-list-backed reference data.
**Traceability:** `print_refs` in `src/output.c`.

#### FR-9: Printable-entry filtering for tree output
The module shall support determining whether a linked-list entry should contribute to visible tree output.
**Traceability:** `is_printable` in `src/output.c`.

#### FR-10: Last-printable-entry determination
The module shall support determining whether a linked-list entry is the last relevant printable item for tree-format branch rendering.
**Traceability:** `is_last` in `src/output.c`.

#### FR-11: Direct tree rendering
The module shall support rendering direct tree output for a symbol hierarchy using linked-list-derived relationship traversal.
**Traceability:** `direct_tree`, `tree_output` in `src/output.c`; `Symbol` use in `src/output.c`.

#### FR-12: Inverted tree rendering
The module shall support rendering inverted tree output for a symbol hierarchy using linked-list-derived relationship traversal.
**Traceability:** `inverted_tree`, `tree_output` in `src/output.c`; `Symbol` use in `src/output.c`.

#### FR-13: DOT symbol emission
The module shall support printing DOT-oriented symbol output for a given output symbol and source line.
**Traceability:** `dot_print_symbol` in `src/dot.c`; `output_symbol` type references.

#### FR-14: Duplicate identity registration support
The module shall support registration-time detection of previously seen option-file identities based on device/inode identity.
**Traceability:** `optfile_register` in `src/main.c`; `optfileid` referenced type.

### Key Entities

#### Linked list
A mutable sequence container that owns an ordered chain of entries and serves as the primary storage abstraction for this module’s generic collection behavior and output traversal inputs.
**Traceability:** `struct linked_list` references throughout `src/linked-list.c`, plus use in `src/dot.c` and `src/output.c`.

#### Linked-list entry
A single node within a linked list that carries one stored opaque data object and participates in sequence order. Entry identity is sufficient for unlink operations, and entry state is examined by output helpers that determine printability and “last item” behavior.
**Traceability:** `struct linked_list_entry` references throughout `src/linked-list.c`, `src/dot.c`, and `src/output.c`.

#### Output symbol
A symbol-oriented output record consumed by DOT printing and associated with linked-list-backed related data.
**Traceability:** `struct output_symbol` reference in `src/dot.c`.

#### Symbol
A symbol/entity used by tree-output routines to traverse and render direct or inverted call/reference structure.
**Traceability:** `direct_tree`, `inverted_tree`, `tree_output` in `src/output.c`.

#### Option-file identity
A device/inode-based identity value used to record whether an option file has already been registered.
**Traceability:** `optfile_register` in `src/main.c`; `optfileid` referenced type.

## Success Criteria

1. The Rust module supports front and back insertion into an initially empty list and preserves observable order during later iteration.
   **Traceability:** `linked_list_append`, `linked_list_prepend`, `linked_list_iterate`.

2. Destroying a populated list leaves the caller-visible list state empty and prevents any further entries from being observed by size or iteration.
   **Traceability:** `linked_list_destroy`, `linked_list_size`, `linked_list_iterate`.

3. Unlinking a known entry removes exactly that entry while preserving the relative order of remaining entries.
   **Traceability:** `linked_list_unlink`.

4. Membership checks return positive only for data objects currently present in the list and negative after removal or destruction.
   **Traceability:** `data_in_list`, `linked_list_unlink`, `linked_list_destroy`.

5. Size reporting matches the number of currently linked entries after append, prepend, unlink, and destroy operations across empty, singleton, and multi-entry cases.
   **Traceability:** `linked_list_size`, all list mutation functions.

6. Callback iteration visits entries in list order and invokes the callback once per visited entry for non-empty lists and zero times for empty lists.
   **Traceability:** `linked_list_iterate`.

7. Reference-list output accepts a name plus linked-list-backed references and produces output without failing for empty or populated lists.
   **Traceability:** `print_refs`.

8. Tree output in both direct and inverted forms honors printable filtering and last-printable-item logic when rendering branch structure.
   **Traceability:** `is_printable`, `is_last`, `direct_tree`, `inverted_tree`, `tree_output`.

9. DOT symbol printing accepts an output symbol and line number and emits the corresponding DOT-oriented symbol representation.
   **Traceability:** `dot_print_symbol`.

10. Option-file registration preserves duplicate-detection behavior for repeated device/inode identities.
    **Traceability:** `optfile_register`.