# Functional Specification: module_src_linked_list_entry_01

## Document Control

- **Project**: `cflow-new`
- **Module**: `module_src_linked_list_entry_01`
- **Category**: `module_cluster`
- **Rust branch**: `064-module_src_linked_list_entry_01-rust-port`
- **Generation date**: `2026-06-17`

## Overview

This module provides two closely related areas of functionality used by the program:

1. A generic linked-list facility for storing opaque item pointers, adding items at either end, iterating items, unlinking entries, destroying the container, testing membership by pointer identity, and counting elements.
2. Linked-list-driven output behavior used by the program’s reporting paths, including:
   - printing reference lists,
   - filtering list entries for tree rendering,
   - determining whether a tree node is the last printable sibling,
   - rendering direct and inverted tree views,
   - printing symbol references in DOT output,
   - registering file identities to avoid duplicate optfile registration.

The Rust rewrite must preserve the observable behavior of these responsibilities as evidenced by the analyzed functions in `src/linked-list.c`, `src/output.c`, `src/dot.c`, and `src/main.c`.

## Feature Specification

### 1. Generic linked-list operations

The module must provide list behavior for collections of opaque data items.

Supported behaviors:

- Create or extend a list by appending a data item to the tail.
- Create or extend a list by prepending a data item to the head.
- Destroy a list container and its entries.
- Unlink a specific entry from an existing list.
- Iterate over entries while allowing the iteration callback to control continued traversal.
- Test whether a given data pointer is present in a list.
- Count the number of entries in a list.

Observed behavior indicates that the list stores item references without taking responsibility for destroying the pointed-to payload objects.

### 2. Iteration-driven filtering and traversal support

The module must support traversal patterns used by output code:

- list entry inspection to determine whether an entry should be considered printable,
- checking whether an entry is the last printable element among siblings,
- visiting list contents in order for output generation.

The Rust rewrite must preserve output-facing traversal semantics relied on by tree rendering and reference printing.

### 3. Reference-list output

The module must support printing a named reference list from linked-list data.

Observed usage shows that a textual name and a linked list of references are supplied, and output is generated from the list contents in list order.

### 4. Tree output support

The module must support hierarchical output over symbol relationships using linked-list-backed child/reference collections.

Required behaviors evidenced by the source set:

- render a direct tree view,
- render an inverted tree view,
- perform top-level tree output dispatch/workflow,
- determine printable siblings so tree formatting reflects whether an item is the last visible branch.

The Rust rewrite must preserve the same visible tree-structure decisions for printable versus non-printable entries.

### 5. DOT symbol output support

The module must support outputting a symbol record to a DOT-format stream, using symbol data and a line context.

The Rust rewrite must preserve the formatting role of this helper as part of graph output generation.

### 6. File-identity registration for option-file handling

The module must support registering file identities by device/inode pair and reporting whether a file identity has already been seen.

The Rust rewrite must preserve the deduplication behavior implied by repeated registration checks.

## User Scenarios & Testing

### Scenario 1: Build a list of items in encountered order

A caller starts with no list and appends several opaque items as they are discovered.

**Expected support**:
- the list is created on first append,
- later appends preserve encounter order,
- size reporting matches the number of appended items,
- membership tests succeed for pointers that were appended and fail for pointers that were not.

**Relevant evidence**:
- `linked_list_append`
- `linked_list_size`
- `data_in_list`

### Scenario 2: Build a list where newest items must appear first

A caller prepends items to form a front-loaded list.

**Expected support**:
- the list is created on first prepend,
- prepended items appear before prior items during traversal,
- counting reflects all inserted entries.

**Relevant evidence**:
- `linked_list_prepend`
- `linked_list_iterate`
- `linked_list_size`

### Scenario 3: Remove a known entry from a list without destroying payload data

A caller has a list and a specific entry handle already belonging to that list and unlinks it.

**Expected support**:
- the entry is removed from list traversal,
- remaining entries stay linked and traversable,
- payload objects referenced by list data are not implied to be destroyed by unlinking.

**Relevant evidence**:
- `linked_list_unlink`
- linked-list entry and list types in `src/linked-list.c`

### Scenario 4: Traverse a list to drive output

A caller iterates a list with a callback that processes each stored item.

**Expected support**:
- traversal proceeds in list order,
- callback invocation receives both the stored item and caller-supplied context data,
- callback return value influences whether iteration continues, consistent with the original module’s traversal behavior.

**Relevant evidence**:
- `linked_list_iterate`

### Scenario 5: Destroy a list container after use

A caller finishes using a list and destroys it.

**Expected support**:
- list entries are released,
- the list container is no longer usable afterward,
- payload objects are not required by this module to be freed as part of list destruction.

**Relevant evidence**:
- `linked_list_destroy`

### Scenario 6: Print a reference list for a symbol or name

A reporting path supplies a name and a linked list of references.

**Expected support**:
- output is produced from the provided list,
- traversal uses the linked-list contents,
- the routine tolerates the reference-list structure expected by the rest of the program.

**Relevant evidence**:
- `print_refs`

### Scenario 7: Render tree output while skipping non-printable entries

The reporting path renders hierarchical output from symbol relationships.

**Expected support**:
- printable status is derived per linked-list entry,
- “last child” formatting depends on the last printable sibling, not merely the physically last entry,
- both direct and inverted tree rendering modes are supported,
- top-level tree output coordinates the rendering flow.

**Relevant evidence**:
- `is_printable`
- `is_last`
- `direct_tree`
- `inverted_tree`
- `tree_output`

### Scenario 8: Emit DOT output for symbols

A graph-output path writes symbol information to a DOT stream.

**Expected support**:
- a helper emits symbol-related output using the provided stream, line context, and symbol data,
- output remains suitable for DOT-generation flow.

**Relevant evidence**:
- `dot_print_symbol`

### Scenario 9: Avoid duplicate option-file registration

The program encounters files identified by device and inode values and registers them.

**Expected support**:
- first registration of a unique identity is accepted,
- repeated registration of the same identity is recognized as already present,
- deduplication is based on file identity rather than textual path.

**Relevant evidence**:
- `optfile_register`

## Requirements

### Functional Requirements

#### FR-1: Opaque item list management
The Rust module shall support linked-list storage of opaque item references, including insertion at both tail and head positions.

**Traceability**:
- `linked_list_append`
- `linked_list_prepend`

#### FR-2: List destruction
The Rust module shall support destroying a list container and its entries without requiring destruction of the stored payload objects by this module.

**Traceability**:
- `linked_list_destroy`

#### FR-3: Entry unlinking
The Rust module shall support removing a specified existing entry from a list while preserving the integrity of the remaining list.

**Traceability**:
- `linked_list_unlink`

#### FR-4: Ordered iteration with callback control
The Rust module shall support iterating list contents in stored order, invoking a caller-provided callback with each stored item and caller context, with callback result affecting continuation behavior as in the source module.

**Traceability**:
- `linked_list_iterate`

#### FR-5: Membership testing by stored pointer identity
The Rust module shall support checking whether a given stored item reference is present in a list.

**Traceability**:
- `data_in_list`

#### FR-6: Size reporting
The Rust module shall support returning the number of entries currently present in a list.

**Traceability**:
- `linked_list_size`

#### FR-7: Reference-list output
The Rust module shall support generating textual output for a named list of references using linked-list-backed input.

**Traceability**:
- `print_refs`

#### FR-8: Printable-entry filtering for tree formatting
The Rust module shall support determining whether a list entry should contribute to visible tree output and whether an entry is the last printable sibling for formatting purposes.

**Traceability**:
- `is_printable`
- `is_last`

#### FR-9: Direct and inverted tree rendering
The Rust module shall support rendering symbol hierarchies in both direct and inverted tree forms.

**Traceability**:
- `direct_tree`
- `inverted_tree`

#### FR-10: Tree output orchestration
The Rust module shall support the top-level workflow that produces tree output from program symbol data.

**Traceability**:
- `tree_output`

#### FR-11: DOT symbol printing support
The Rust module shall support emitting DOT-related symbol output using stream-like output, line context, and symbol information.

**Traceability**:
- `dot_print_symbol`

#### FR-12: File-identity deduplication
The Rust module shall support registration of file identities keyed by device/inode values and shall distinguish first-seen identities from duplicates.

**Traceability**:
- `optfile_register`

### Key Entities

#### Linked list
A container representing an ordered collection of entries. It is used across list management and output-related traversal.

**Relationships**:
- owns zero or more linked-list entries,
- is consumed by generic list operations,
- is used as input to reference printing and tree-related traversal.

**Traceability**:
- `struct linked_list` references in `src/linked-list.c`, `src/dot.c`, `src/output.c`

#### Linked-list entry
A node within a linked list that links one stored opaque data reference into the ordered collection.

**Relationships**:
- belongs to a linked list,
- is the unit inspected by printable/last-sibling logic,
- may be unlinked from its parent list.

**Traceability**:
- `struct linked_list_entry` references in `src/linked-list.c`, `src/dot.c`, `src/output.c`

#### Output symbol
A symbol-oriented output record used by DOT-printing support.

**Relationships**:
- supplied to DOT output helper,
- represents the symbol information needed for graph emission.

**Traceability**:
- `struct output_symbol` in `src/dot.c`

#### Symbol
A program symbol entity used by tree-rendering routines.

**Relationships**:
- rendered by direct and inverted tree output,
- associated with linked-list-backed hierarchical/reference relationships used during output.

**Traceability**:
- `direct_tree`
- `inverted_tree`
- `tree_output`

#### Option-file identity
A file identity represented by device and inode values for duplicate detection.

**Relationships**:
- registered through the option-file registration function,
- compared against prior registrations to detect repeats.

**Traceability**:
- `optfile_register`
- referenced `optfileid` type in analysis

## Success Criteria

1. **Append/prepend behavior parity**
   Given an empty list, appending and prepending items in controlled sequences shall yield traversal orders consistent with tail insertion and head insertion respectively.

   **Traceability**:
   - `linked_list_append`
   - `linked_list_prepend`
   - `linked_list_iterate`

2. **Accurate size and membership results**
   For lists built from known item references, reported size shall equal the number of current entries, and membership checks shall correctly distinguish present versus absent references.

   **Traceability**:
   - `linked_list_size`
   - `data_in_list`

3. **Correct unlink behavior**
   After unlinking a known entry from a multi-entry list, that entry shall no longer appear in traversal, and the remaining entries shall still appear in valid order.

   **Traceability**:
   - `linked_list_unlink`

4. **Destruction without payload-free requirement**
   Destroying a list shall remove list structure ownership without requiring the module to free the opaque payload objects stored in entries.

   **Traceability**:
   - `linked_list_destroy`

5. **Iteration callback semantics preserved**
   Iteration tests using callbacks that request continued traversal versus early stop shall match the original module’s continuation behavior.

   **Traceability**:

6. **Reference-list output preserved**
   For representative linked-list reference input, the Rust rewrite shall produce reference output equivalent in content and ordering to the source module.

   **Traceability**:
   - `print_refs`

7. **Tree printable/last-sibling behavior preserved**
   In test hierarchies containing both printable and non-printable entries, branch formatting shall reflect the last printable sibling rather than the physically last raw list entry.

   **Traceability**:
   - `is_printable`
   - `is_last`
   - `direct_tree`
   - `inverted_tree`

8. **Direct and inverted tree output available**
   The Rust rewrite shall generate both direct and inverted tree views for representative symbol hierarchies.

   **Traceability**:
   - `tree_output`

9. **DOT symbol output role preserved**
   For representative symbol input, DOT-related output shall still include symbol emission performed from symbol data plus line context through the corresponding helper path.

   **Traceability**:
   - `dot_print_symbol`

10. **Duplicate file registration detection preserved**
    Registering the same device/inode pair more than once shall distinguish the first registration from subsequent duplicate registrations.

    **Traceability**:
    - `optfile_register`