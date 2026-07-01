# spec.md

## Title

Functional Specification: `module_src_linked_list_entry_01` Rust Port

## Document Control

- Project: `cflow-new`
- Module: `module_src_linked_list_entry_01`
- Category: `module_cluster`
- Target Rust Branch: `064-module_src_linked_list_entry_01-rust-port`
- Generation Date: `2026-06-17`

## Overview

This module provides two closely related areas of functionality used by the wider project:

1. A generic linked-list facility used to store, traverse, unlink, count, and destroy ordered collections of entries.
2. Output support that consumes linked-list-backed symbol/reference collections for textual and DOT-style rendering, including tree-oriented output decisions based on list position and printability.

The Rust rewrite must preserve the observed functional behavior of these responsibilities as used from:
- `src/linked-list.c`
- `src/dot.c`
- `src/output.c`
- `src/main.c`

The module also includes a small registration use case in `optfile_register` where list-backed identity tracking is used to avoid duplicate file registrations.

## In Scope

- Ordered insertion into a linked list at the front or back.
- Removal of a specific existing list entry from a list.
- Destruction of list structure storage.
- Iteration across list entries with callback-driven control over removal.
- Membership testing by stored data identity.
- List length calculation.
- Use of linked-list entries to support output formatting decisions.
- Printing of symbol/reference relationships from linked-list-backed collections.
- Duplicate registration prevention based on previously recorded file identity pairs.

## Out of Scope

The following are not required unless needed to preserve observed behavior from the source module:

- New container features not evidenced in the C sources.
- Thread-safety guarantees.
- Serialization or persistence.
- Recovery or transactional behavior.
- New public APIs beyond what is needed to match current module behavior.
- Ownership of arbitrary payload contents beyond the list node/container behavior evidenced by destruction logic.

## Feature Specification

### 1. Generic linked-list collection behavior

The module must provide a generic ordered collection abstraction equivalent in behavior to the C linked list used by the project.

Observed supported operations are:

- append an item to the end of a list
- prepend an item to the beginning of a list
- unlink a specific entry already present in a list
- iterate over entries while allowing callback-directed unlink/removal behavior
- test whether a given stored data pointer/value is already present in a list
- count entries in a list
- destroy the list structure and its entries

Behavioral boundaries evidenced by `src/linked-list.c`:

- The list may initially be absent/empty and must support creation through first insertion.
- The list preserves insertion order for append and prepend semantics.
- Membership testing is based on stored item identity/equality as used by the original list payload comparisons.
- Destruction removes list nodes/container structure; no evidence shows deep destruction of payload objects.

### 2. Output support over linked-list-backed symbol/reference data

The module must support output logic that consumes linked-list-managed collections of symbol or reference records.

Observed output responsibilities are:

- print symbol information for DOT output (`dot_print_symbol`)
- print references associated with a name (`print_refs`)
- determine whether a list entry should be considered printable (`is_printable`)
- determine whether a list entry is the last relevant entry for tree rendering (`is_last`)
- emit direct tree output (`direct_tree`)
- emit inverted tree output (`inverted_tree`)
- select and produce tree-oriented output (`tree_output`)

Behavioral boundaries evidenced by `src/dot.c` and `src/output.c`:

- Output decisions depend on list entry traversal and positional context.
- “Last” in tree formatting is not merely physical tail position; it is evaluated through helper logic tied to printable entries.
- Tree output must support at least two orientations/styles: direct and inverted.
- Reference printing consumes a linked-list-backed reference list associated with a symbol name.

### 3. Duplicate registration tracking for option file identities

The module must preserve the behavior used by `optfile_register` in `src/main.c`:

- accept a device/inode identity pair
- track previously registered identities
- detect and reject or report duplicate registrations according to current module behavior

This feature is functionally tied to list-backed identity storage and membership prevention.

## User Scenarios & Testing

### Scenario 1: Build a list of generic items in encounter order

A caller starts with no list, appends several items, and later queries list size.

Expected support:
- first append creates a usable list
- subsequent appends preserve encounter order
- reported size equals number of inserted items

Traceability:
- `linked_list_append`
- `linked_list_size`

### Scenario 2: Prioritize an item ahead of existing items

A caller has an existing list and prepends a new item that must appear before current entries.

Expected support:
- prepended item becomes the first entry
- existing entries remain after it in prior order

Traceability:
- `linked_list_prepend`

### Scenario 3: Remove a known entry without rebuilding the list

A caller holds a reference/handle to a list entry and unlinks it from the list.

Expected support:
- the target entry is removed from list traversal
- surrounding entries remain linked in order
- subsequent size or iteration reflects the removal

Traceability:
- `linked_list_unlink`
- `linked_list_size`
- `linked_list_iterate`

### Scenario 4: Iterate and selectively remove entries through callback logic

A caller traverses a list with a callback and uses callback results to drive removal of matching entries.

Expected support:
- every current entry is visited in traversal order unless removed by iteration behavior
- callback receives the stored entry payload and caller-supplied context data
- entries designated for removal are unlinked during iteration without corrupting traversal

Traceability:
- `linked_list_iterate`

### Scenario 5: Check whether a payload is already present

A caller wants to avoid duplicate storage and checks whether a data item is already in a list.

Expected support:
- positive result when the same stored item is present
- negative result when absent

Traceability:
- `data_in_list`

### Scenario 6: Destroy a list structure after use

A caller no longer needs a list and destroys it.

Expected support:
- all list nodes are released from the collection structure
- the caller no longer has a usable list structure through the passed list reference

Traceability:
- `linked_list_destroy`

### Scenario 7: Print references associated with a symbol name

A caller provides a name and a linked list of references for output.

Expected support:
- output is produced from the provided name and reference list
- traversal respects the list contents currently present

Traceability:
- `print_refs`

### Scenario 8: Render tree output with correct branch termination behavior

A caller triggers tree output for symbols that include linked-list-backed relations.

Expected support:
- printable status of entries affects which branches are shown
- last-entry detection affects branch/indent formatting
- both direct and inverted tree modes remain supported

Traceability:
- `is_printable`
- `is_last`
- `direct_tree`
- `inverted_tree`
- `tree_output`

### Scenario 9: Prevent duplicate option-file registration

The program registers a file identity by device and inode, then attempts to register the same identity again.

Expected support:
- first registration succeeds and records the identity
- second registration is recognized as duplicate according to current behavior

Traceability:
- `optfile_register`

## Requirements

### Functional Requirements

#### FR-1: Empty-list initialization through insertion
The module shall support insertion into an absent or empty list such that the first append or prepend produces a valid one-entry list.

Traceability:
- `linked_list_append`
- `linked_list_prepend`

#### FR-2: Ordered append
The module shall support adding a new item to the logical end of a list while preserving the order of existing entries.

Traceability:
- `linked_list_append`

#### FR-3: Ordered prepend
The module shall support adding a new item to the logical beginning of a list while preserving the relative order of existing entries after the inserted item.

Traceability:
- `linked_list_prepend`

#### FR-4: Entry unlink by entry identity
The module shall support removing a specific list entry from an existing list when the caller identifies the entry to remove.

Traceability:
- `linked_list_unlink`
- `linked_list_entry`

#### FR-5: Iteration with callback and caller context
The module shall support traversing list entries in list order, invoking a caller-supplied callback for each entry and passing caller-supplied context data.

Traceability:
- `linked_list_iterate`

#### FR-6: Iteration-driven removal support
The module shall support callback-directed removal/unlink behavior during iteration, with traversal continuing safely over the remaining entries.

Traceability:
- `linked_list_iterate`
- `linked_list_unlink`

#### FR-7: Membership testing
The module shall support determining whether a given stored data item is present in a list.

Traceability:
- `data_in_list`

#### FR-8: Size reporting
The module shall support reporting the current number of entries in a list.

Traceability:
- `linked_list_size`

#### FR-9: List destruction
The module shall support destroying a list structure and all of its entries.

Traceability:
- `linked_list_destroy`

#### FR-10: Non-deep payload destruction boundary
List destruction shall be limited to the list structure and entry storage; no requirement is evidenced to destroy or free arbitrary payload objects stored in the list.

Traceability:
- `linked_list_destroy`

#### FR-11: Reference-list output
The module shall support producing output for a named symbol together with a linked-list-backed reference list.

Traceability:
- `print_refs`

#### FR-12: DOT symbol output support
The module shall support rendering symbol information for DOT-oriented output using linked-list-backed symbol context where applicable.

Traceability:
- `dot_print_symbol`
- `output_symbol`

#### FR-13: Printable-entry filtering for tree rendering
The module shall support evaluating whether a linked-list entry is printable for purposes of tree-oriented output.

Traceability:
- `is_printable`

#### FR-14: Last-relevant-entry detection for tree rendering
The module shall support determining whether a linked-list entry is the last relevant entry for tree formatting decisions.

Traceability:
- `is_last`

#### FR-15: Direct tree output
The module shall support producing tree output in direct orientation.

Traceability:
- `direct_tree`

#### FR-16: Inverted tree output
The module shall support producing tree output in inverted orientation.

Traceability:
- `inverted_tree`

#### FR-17: Tree output orchestration
The module shall support selecting and executing tree-oriented output behavior over symbol relationships.

Traceability:
- `tree_output`

#### FR-18: Duplicate registration tracking by file identity
The module shall support recording file identities composed of device and inode values and detecting repeated registration of the same identity.

Traceability:
- `optfile_register`
- `optfileid`

### Key Entities

#### Linked List
A collection object that represents an ordered sequence of entries. It may be absent before first insertion, can be traversed, can have entries added at either end, and can be destroyed as a whole.

Traceability:
- `linked_list`

#### Linked List Entry
A node within a linked list that links one stored payload item into the collection and can be individually unlinked. Output helpers also inspect entry position and printability through entry-level traversal.

Traceability:
- `linked_list_entry`

#### Stored Payload / Data Item
An opaque item associated with a list entry. The list utilities treat it generically for insertion, membership testing, and callback delivery.

Traceability:
- `linked_list_append`
- `linked_list_prepend`
- `linked_list_iterate`
- `data_in_list`

#### Output Symbol
A symbol-oriented output record used by DOT printing logic and related output flows.

Traceability:
- `output_symbol`
- `dot_print_symbol`

#### Symbol
A symbol entity used by tree output functions to render direct or inverted symbol relationships.

Traceability:
- `direct_tree`
- `inverted_tree`
- `tree_output`

#### Option File Identity
A file identity record formed from device and inode values and used to detect duplicate registrations.

Traceability:
- `optfile_register`
- `optfileid`

## Success Criteria

### SC-1: Append/prepend correctness
Given an initially empty list, append and prepend operations shall produce list contents whose traversal order matches the C module’s semantics for end insertion and front insertion.

Traceability:
- `linked_list_append`
- `linked_list_prepend`

### SC-2: Size correctness
For any tested sequence of insertions and removals supported by the module, reported list size shall equal the number of entries reachable by traversal.

Traceability:
- `linked_list_size`
- `linked_list_unlink`
- `linked_list_iterate`

### SC-3: Membership correctness
Membership checks shall return positive for items currently present in the list and negative for items not present.

Traceability:
- `data_in_list`

### SC-4: Safe selective iteration behavior
When iteration callback logic marks some entries for removal, the final list contents shall match the expected retained set and traversal shall complete without skipping retained entries due to removal side effects.

Traceability:
- `linked_list_iterate`

### SC-5: Unlink correctness
After unlinking a specific entry, that entry shall no longer appear in subsequent traversal or count results, while non-target entries remain in order.

Traceability:
- `linked_list_unlink`
- `linked_list_size`

### SC-6: Destruction behavior
After list destruction, the list structure shall be considered no longer usable through the caller’s passed list reference, and no list entries shall remain reachable through it.

Traceability:
- `linked_list_destroy`

### SC-7: Reference output parity
For equivalent input name and reference-list contents, Rust output generated by the reference-printing path shall preserve the functional content of the C module’s printed references.

Traceability:
- `print_refs`

### SC-8: Tree rendering parity
For equivalent symbol/linkage inputs, Rust tree output shall preserve the direct/inverted mode behavior and branch termination decisions driven by printable-entry and last-entry logic.

Traceability:
- `is_printable`
- `is_last`
- `direct_tree`
- `inverted_tree`
- `tree_output`

### SC-9: DOT symbol output parity
For equivalent symbol output inputs, Rust DOT printing behavior shall preserve the functional content emitted by the C module’s symbol-printing path.

Traceability:
- `dot_print_symbol`

### SC-10: Duplicate registration prevention
When the same device/inode identity is registered more than once, the Rust implementation shall detect the repeated identity with the same accept/reject behavior as the C module.

Traceability:
- `optfile_register`