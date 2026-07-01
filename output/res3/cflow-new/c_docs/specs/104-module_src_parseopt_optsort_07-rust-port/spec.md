# spec.md

## Title

Rust Functional Specification for `module_src_parseopt_optsort_07`

## Metadata

- Project: `cflow-new`
- Module: `module_src_parseopt_optsort_07`
- Category: `module_cluster`
- Source basis: `src/parseopt/help.c`
- Rust branch target: `104-module_src_parseopt_optsort_07-rust-port`
- Generation date: `2026-06-17`

## Overview

This module is the option-ordering portion of the parseopt help system. Its evidenced responsibility is to sort option definitions for help/usage presentation, using an internal sortable representation that refers back to option definitions and their names.

The Rust rewrite must preserve the observable behavior of this module within the help-generation flow:

- create and manipulate sortable option entries,
- order those entries deterministically for display,
- reorder name-related fields within an entry when needed for canonical presentation.

This specification is limited to functionality evidenced by `optsort`, `sortnames`, and the related structures in `src/parseopt/help.c`. It does not define new public APIs or capabilities beyond this scope.

## Feature Specification

### Feature: Help-option ordering for display

The module shall support ordering a collection of help option entries before they are rendered by the surrounding parseopt help logic.

Observed scope from source:

- `optsort` performs sorting over an array of `optsort` entries.
- `sortnames` adjusts the internal name ordering of a single sortable entry across indexed name positions.
- The sortable entries are tied to option definitions (`optdef`) and are used in the help-context flow.

The Rust version must implement equivalent behavior at the functional level:

1. Accept a set of sortable option records derived from option definitions.
2. Normalize or reorder the names inside each sortable record when required by the original behavior.
3. Sort the full set into the same display order expected by the C module.
4. Preserve the mapping from sorted entries back to their originating option definitions so downstream help formatting can use the reordered results.

### Functional boundary

Included:

- sorting of option-display entries,
- name-position reordering within a sortable entry,
- operation in support of parseopt help/usage generation.

Excluded unless directly required to support the above:

- full command-line parsing,
- output formatting unrelated to the sorted order itself,
- non-help option semantics,
- any new interfaces not evidenced in the source module.

## User Scenarios & Testing

### Scenario 1: Sorting options for help output

A help-generation path has collected multiple option definitions and converted them into sortable entries. Before rendering help text, it invokes this module to order them.

Expected result:

- all entries are returned in deterministic help-display order,
- each sorted entry still refers to the correct originating option definition.

Test coverage:

- provide a mixed set of option entries with distinct names,
- verify that the Rust output order matches the C module behavior for the same input set.

### Scenario 2: Canonicalizing names within one option entry

An option definition may contribute more than one displayed name form inside a sortable record. The help path needs those names arranged in the module’s expected internal order before or during sorting.

Expected result:

- the chosen name positions are swapped/reordered exactly as the original module does,
- no unrelated fields in the sortable record are modified.

Test coverage:

- construct a sortable entry with two name positions in non-canonical order,
- apply the Rust equivalent of `sortnames`,
- verify that only the expected name-related fields have changed.

### Scenario 3: Sorting a subset or small collection

The help system may sort a very small number of entries, including zero, one, or a partial collection.

Expected result:

- empty and single-entry collections remain valid and unchanged,
- the routine reports or yields a result consistent with the source behavior for such counts,
- no invalid reordering occurs.

Test coverage:

- zero-entry input,
- one-entry input,
- two-entry input already ordered,
- two-entry input requiring reordering.

### Scenario 4: Duplicate or closely related display names

Different option definitions may produce entries whose names compare closely or equally under the module’s ordering rules.

Expected result:

- ordering remains deterministic and consistent with the C behavior,
- option-definition identity is preserved even when names are similar.

Test coverage:

- entries with identical primary names but different originating option definitions,
- entries with short/long name combinations that compare similarly,
- verification against the original module’s ordering outcome.

## Requirements

### Functional Requirements

#### FR-1: Sort help option entries
The module shall sort a collection of sortable option records used by parseopt help generation.

Traceability:

- Function: `optsort` in `src/parseopt/help.c:364-395`
- Type: `struct optsort` in `src/parseopt/help.c:330-337`

#### FR-2: Operate on an explicit item count
The sorting behavior shall apply to a provided collection length and shall only consider the specified number of sortable entries.

Traceability:

- Function signature: `optsort(struct optsort *ops, int n)`

#### FR-3: Reorder names within a sortable entry
The module shall support reordering indexed names within a single sortable option record.

Traceability:

- Function: `sortnames` in `src/parseopt/help.c:623-640`
- Type: `struct optsort` in `src/parseopt/help.c:330-337`

#### FR-4: Preserve association to option definitions
After name reordering and collection sorting, each sortable entry shall remain associated with its original option definition so that later help-generation stages can render the correct option metadata.

Traceability:

- Types: `struct optsort`, `struct optdef`
- Related usage context: `struct help_context` in `src/parseopt/help.c:397-405`

#### FR-5: Support integration with help-generation flow
The module shall produce sorted/normalized option-entry data suitable for consumption by the surrounding help-context logic in the same source file.

Traceability:

- Types: `struct help_context`, `struct parseopt`, `struct optdef`
- Functions in scope: `optsort`, `sortnames`

#### FR-6: Handle trivial collection sizes correctly
The module shall correctly handle empty, single-entry, and small collections without producing invalid state.

Traceability:

- Function: `optsort(struct optsort *ops, int n)`

### Key Entities

#### `optsort`
Internal sortable representation for help options.

Role evidenced by source:

- stores the fields needed to compare and reorder options for display,
- refers to an `optdef`,
- contains name-related state that can be reordered by index.

Relationships:

- derived from or linked to `optdef`,
- consumed by `optsort`,
- mutated at name-field level by `sortnames`,
- participates in help-generation processing alongside `help_context`.

Traceability:

- `struct optsort` in `src/parseopt/help.c:330-337`

#### `optdef`
Option definition record used by the parseopt help system.

Role evidenced by source:

- source option metadata for entries that will be sorted for display.

Relationships:

- each sortable entry corresponds to an `optdef`,
- sorted results must preserve this linkage for downstream help rendering.

Traceability:

- `struct optdef` declarations referenced throughout `src/parseopt/help.c`

#### `help_context`
Context object for help generation.

Role evidenced by source:

- holds the surrounding state in which option definitions are prepared and rendered,
- consumes sorted option-definition-related data.

Relationships:

- refers to parseopt state and option definitions,
- depends on sorted option entry ordering produced by this module.

Traceability:

- `struct help_context` in `src/parseopt/help.c:397-405`

#### `parseopt`
Higher-level parseopt/help configuration state.

Role evidenced by source:

- upstream context from which help processing and option definitions are derived.

Relationships:

- referenced by `help_context`,
- provides the larger operational context in which this module is used.

Traceability:

- `struct parseopt` declarations referenced throughout `src/parseopt/help.c`

## Success Criteria

### SC-1: Behavioral equivalence of ordering
For the same constructed `optsort` input data and item count, the Rust implementation shall produce the same option-entry order as the C module.

Traceability:

- `optsort`
- `struct optsort`

Measurement:

- compare ordered results against a reference run or source-derived expected outputs for representative datasets.

### SC-2: Behavioral equivalence of name reordering
For the same sortable entry and the same pair of indices, the Rust implementation shall produce the same internal name ordering as the C module’s `sortnames`.

Traceability:

- `sortnames`
- `struct optsort`

Measurement:

- field-by-field comparison of the affected sortable entry before and after operation.

### SC-3: Identity preservation
After sorting, every output entry shall still map to the same originating `optdef` as before sorting.

Traceability:

- `struct optsort`
- `struct optdef`

Measurement:

- verify pointer/identifier correspondence for each entry across input and output orderings.

### SC-4: Correct handling of boundary counts
The Rust implementation shall correctly process `n = 0`, `n = 1`, and small positive counts without panic, corruption, or unintended mutation outside the addressed range.

Traceability:

- `optsort(struct optsort *ops, int n)`

Measurement:

- targeted unit tests for boundary sizes and partial-range checks.

### SC-5: Fit for help-generation integration
The Rust module shall provide sorted and name-normalized option-entry data that can be consumed by the surrounding help-generation logic without loss of needed option-definition linkage.

Traceability:

- `struct help_context`
- `struct optdef`
- `struct optsort`

Measurement:

- integration tests that run the help preparation path using the Rust-sorted data and confirm expected option association and display order.

## Non-Goals

The Rust rewrite specification does not require:

- introducing a new public command-line parsing API,
- changing help text formatting policy beyond preserving sort-related behavior,
- adding persistence, serialization, FFI, concurrency guarantees, or recovery features,
- supporting inputs or workflows not evidenced by `src/parseopt/help.c`.

## Notes for Validation

Validation should be based on source-equivalent behavior, not on reproducing C implementation structure. The decisive outcomes are:

- same sorted option ordering,
- same intra-entry name ordering behavior,
- same preservation of option-definition associations within the parseopt help flow.