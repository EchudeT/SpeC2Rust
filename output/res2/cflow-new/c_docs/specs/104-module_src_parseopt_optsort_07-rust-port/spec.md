# spec.md

## Title

Rust Functional Specification for `module_src_parseopt_optsort_07`

## Metadata

- Project: `cflow-new`
- Module category: `module_cluster`
- Source basis: `src/parseopt/help.c`
- Rust branch target: `104-module_src_parseopt_optsort_07-rust-port`
- Generation date: `2026-06-17`

## Overview

This module provides the option-ordering behavior used by parseopt help generation. Its evidenced scope is limited to preparing option definitions for display by sorting option names in a stable, help-oriented order.

The Rust rewrite must implement the same functional role within help processing:

- represent sortable option entries,
- compare and reorder option names for help presentation,
- preserve the association between each option definition and its sortable naming metadata,
- support the surrounding help-generation flow that depends on sorted option output.

This specification is derived from the functionality evidenced by:

- `optsort` in `src/parseopt/help.c:364-395`
- `sortnames` in `src/parseopt/help.c:623-640`
- `struct optsort` in `src/parseopt/help.c:330-337`
- related help/option context types in the same file.

## Feature Specification

### Feature: Help-option sorting support

The module must provide the behavior needed to sort option definitions before they are rendered in help text.

Observed functional boundary:

- A sortable record (`optsort`) ties an option definition to sorting metadata.
- `optsort(...)` performs ordering across a collection of sortable option entries.
- `sortnames(...)` reorders names within or across indexed sortable entries as required by the help formatting flow.

The Rust version must therefore support:

1. Construction or use of sortable option records that retain a link to the originating option definition.
2. Sorting of a bounded collection of option records for help display.
3. Name-level ordering operations used by help formatting after or during option sorting.
4. Deterministic ordering so that equivalent inputs produce equivalent help-option ordering.

### In-scope behavior

- Ordering option-related entries for presentation.
- Reordering names associated with options.
- Operating as an internal help-formatting utility rather than a standalone end-user feature.

### Out-of-scope behavior

Not evidenced for this module and therefore not required here:

- Parsing command-line arguments.
- Defining new option semantics.
- Rendering complete help output formatting beyond the ordering support this module contributes.
- External storage, serialization, concurrency guarantees, recovery logic, or FFI.

## User Scenarios & Testing

### Scenario 1: Sort a set of option definitions before help rendering

A help-generation flow has collected several option definitions. Before producing user-visible help text, it invokes this module to order those options.

The Rust version must support:

- receiving option-linked sortable entries,
- ordering them deterministically,
- returning or exposing the sorted result for downstream help rendering.

#### Test expectations

- Given multiple option entries in non-display order, the module produces a deterministic sorted order.
- Each sorted entry still refers to the same underlying option definition as before sorting.
- The number of entries is unchanged by sorting.

### Scenario 2: Reorder option names used in display

A help-generation flow needs the names associated with options to appear in the correct order for display. The module performs name reordering through the `sortnames` behavior.

The Rust version must support:

- selecting indexed sortable entries,
- reordering their names according to the same help-ordering rules,
- preserving valid option/name associations after reordering.

#### Test expectations

- Reordering two indexed entries changes only their name order, not unrelated entries.
- Reordering is deterministic for the same input indices and option-name state.
- After reordering, names remain attached to valid option records.

### Scenario 3: Integrate with help context data

Sorting occurs as part of a larger help-generation process that uses parseopt state, option definitions, and help context structures from the same source file.

The Rust version must support use within that flow by operating on entities corresponding to the original option/help records.

#### Test expectations

- Option entries created from help context data can be passed into sorting without data loss.
- Sorting does not require unrelated parser behavior to be implemented inside this module.
- Sorted results remain usable by the surrounding help-generation code path.

## Requirements

### Functional Requirements

#### FR-1: Sortable option record support
The module shall represent sortable option entries that maintain an association with an option definition, as evidenced by `struct optsort` and its use by `optsort`.

Traceability:
- `src/parseopt/help.c:330-337`
- `src/parseopt/help.c:364-395`

#### FR-2: Collection sorting for help order
The module shall sort a collection of sortable option entries for help presentation, as evidenced by `optsort(struct optsort *ops, int n)`.

Traceability:
- `src/parseopt/help.c:364-395`

#### FR-3: Name reordering support
The module shall support reordering option names across indexed sortable entries, as evidenced by `sortnames(struct optsort *ops, int i, int j)`.

Traceability:
- `src/parseopt/help.c:623-640`

#### FR-4: Deterministic behavior
For the same input option records and name state, the module shall produce the same sorted and reordered result each time. This is required by the sorting role evidenced in `optsort` and `sortnames`.

Traceability:
- `src/parseopt/help.c:364-395`
- `src/parseopt/help.c:623-640`

#### FR-5: Referential integrity of option definitions
Sorting and name reordering shall not sever the relationship between sortable entries and their originating option definitions.

Traceability:
- `struct optsort` references option-definition data in `src/parseopt/help.c:330-337`
- functions operating on `struct optsort` at `src/parseopt/help.c:364-395` and `623-640`

#### FR-6: Compatibility with help-generation flow
The module shall operate on data compatible with the surrounding help context and option-definition structures used in `help.c`, so that sorted results can be consumed by help-generation logic.

Traceability:
- help-related structures at `src/parseopt/help.c:397-405`, `408`, `471`, `499`
- option-definition structures referenced throughout `help.c`
- sorting functions at `364-395` and `623-640`

### Key Entities

#### `optsort`
A sortable option-entry record used as the unit of ordering. It binds sorting/name-order metadata to an option definition.

Relationship:
- Each `optsort` entry is associated with an option definition.
- A collection of `optsort` entries is the input to `optsort(...)`.
- Indexed `optsort` entries are manipulated by `sortnames(...)`.

Traceability:
- `src/parseopt/help.c:330-337`
- `src/parseopt/help.c:364-395`
- `src/parseopt/help.c:623-640`

#### Option definition (`optdef`)
Represents an individual option whose display order is managed by this module.

Relationship:
- Option definitions are referenced by sortable entries.
- Their names or display-relevant identifiers are the subject of sorting and reordering.

Traceability:
- `src/parseopt/help.c:157`, `169-170`, `181`, `265`, `268`, `328`, `332`, `346`, `409`, `412`, `472`, `479`

#### Help context
Represents the broader help-generation state in which option sorting is performed.

Relationship:
- Help context uses option definitions.
- This module contributes ordered option data for downstream help output.

Traceability:
- `src/parseopt/help.c:397-405`
- `src/parseopt/help.c:408`
- `src/parseopt/help.c:471`
- `src/parseopt/help.c:499`

#### Parseopt state
Represents parser-related configuration/state present in the same help-processing source file.

Relationship:
- Provides surrounding context for option/help handling, though this module’s evidenced responsibility is limited to sorting support.

Traceability:
- `src/parseopt/help.c:63`
- `src/parseopt/help.c:131`
- `src/parseopt/help.c:178`
- `src/parseopt/help.c:259`
- `src/parseopt/help.c:293`
- `src/parseopt/help.c:399`

## Success Criteria

### SC-1: Correct sortable-entry ordering
Given a collection of option-linked sortable entries corresponding to the original `optsort` role, the Rust module produces a deterministic ordering suitable for help generation.

Traceability:
- `optsort` at `src/parseopt/help.c:364-395`

### SC-2: Correct name reordering
Given indexed sortable entries, the Rust module reproduces the original module’s name-reordering behavior without affecting unrelated entries.

Traceability:
- `sortnames` at `src/parseopt/help.c:623-640`

### SC-3: Option-definition association preserved
After sorting and name reordering, every output entry remains associated with a valid originating option definition.

Traceability:
- `struct optsort` at `src/parseopt/help.c:330-337`
- sorting functions at `364-395` and `623-640`

### SC-4: Help-flow compatibility
The Rust module can be used by surrounding help-generation logic with entities corresponding to `help_context`, `optdef`, and parseopt-related state from `help.c`, without requiring expansion of scope beyond sorting support.

Traceability:
- help-context and related types in `src/parseopt/help.c`
- sorting functions in `src/parseopt/help.c:364-395`, `623-640`

### SC-5: No unsupported feature expansion
The Rust rewrite confines itself to the evidenced sorting and name-ordering responsibilities of this module and does not introduce unevidenced capabilities.

Traceability:
- module file scope: `src/parseopt/help.c`
- evidenced functions: `optsort`, `sortnames`

## Acceptance Notes

- Behavioral equivalence is judged on help-ordering outcomes, not on reproducing C implementation structure.
- Internal Rust design may differ, but it must preserve the functional boundaries defined above.
- Tests should use option collections and name-order cases that exercise both collection sorting and indexed name reordering.