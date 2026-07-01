# spec.md

## Title
Rust Functional Specification for `module_src_parseopt_optsort_07`

## Summary
This module covers the option-name ordering behavior used by `src/parseopt/help.c` in `cflow-new`. Its identified functional scope is limited to sorting option definitions for help/usage presentation.

The Rust rewrite must preserve the observable behavior of:
- ordering a collection of option descriptors through `optsort`
- ordering names within the sorting workflow through `sortnames`

The module is part of the parseopt help-generation path and operates on option-definition data rather than parsing command-line input itself.

## Scope
Included in scope:
- sorting option-related help data prior to presentation
- comparing and reordering option entries through the `optsort` workflow
- internal name-order normalization used by `sortnames`
- preserving relationships between sorted entries and their originating option definitions

Out of scope:
- option parsing
- rendering of full help text outside the ordering behavior evidenced here
- adding new sort modes, configuration surfaces, or public APIs not evidenced by the source module

## Source Basis
Traceability for this specification is based on:
- File: `src/parseopt/help.c`
- Functions:
  - `optsort` (`static int optsort(struct optsort *ops, int n)`)
  - `sortnames` (`static void sortnames(struct optsort *ops, int i, int j)`)
- Data structures referenced in this module area:
  - `struct optsort`
  - `struct optdef`
  - `struct parseopt`
  - `struct help_context`

## Feature Specification

### Feature: Sort option help entries
The module shall provide the behavior needed to sort option-related entries used by help generation.

Observed boundaries from the source indicate:
- `optsort` is the controlling operation for sorting a bounded set of option-sort records.
- `sortnames` performs name-based reordering support on selected entries within that set.
- The sorted result is used in the help-generation context, so ordering must be deterministic for the same input set.

The Rust version must therefore:
1. accept a collection equivalent to the C module's `struct optsort` array plus item count
2. reorder that collection according to the same name-based help-order behavior as the C module
3. preserve association between each sortable entry and its underlying option definition (`optdef`)
4. support pairwise internal name-order adjustment between entries where the C module uses `sortnames`
5. return/produce completion status consistent with the C module's sorting step for valid inputs handled by this module

### Feature: Normalize comparable option names during sorting
This module includes a dedicated helper, `sortnames`, rather than only a single top-level sort call. That indicates name ordering is not merely external array permutation; it also includes controlled ordering of names inside the sortable representation.

The Rust version must preserve this behavior by ensuring:
- when two entries require name-order normalization in the sorting process, the internal name arrangement matches the C module's observable result
- any short-name/long-name or equivalent multi-name representation embedded in a sortable entry remains aligned with the final help-order semantics

### Feature: Support help-generation consumers
This sorting behavior exists within the help subsystem and is referenced alongside `help_context`. The Rust rewrite must therefore remain usable by the help-generation path by producing a sorted option-definition sequence suitable for downstream formatting.

## User Scenarios & Testing

### Scenario 1: Help output requires stable option ordering
A caller preparing command help gathers a set of option definitions and invokes the module's sorting behavior before formatting help text.

Expected result:
- the option entries are returned in the same relative ordering semantics as the C implementation
- each displayed help row still corresponds to the correct original option definition

Test focus:
- construct multiple option definitions with differing names
- run the Rust sorting logic
- verify the resulting order matches the C behavior for the same fixture

### Scenario 2: Entries contain multiple names that must be normalized
A caller has option entries whose sortable representation contains more than one name form and relies on the module to place names in the proper order used for help display.

Expected result:
- pairwise name normalization occurs where required by the sorting workflow
- the final sorted data uses the same internal name arrangement as the C module

Test focus:
- use fixtures where name ordering inside entries affects sort outcome or displayed form
- verify both entry order and internal name order against the C module

### Scenario 3: Mixed option set for help formatting
A help-generation path receives a nontrivial option set and sorts all entries before rendering grouped or sequential help output.

Expected result:
- all sortable entries are included exactly once after sorting
- no entry loses its attached descriptive/definition linkage
- the sorted collection can be consumed directly by downstream help formatting logic

Test focus:
- use a realistic array of option definitions mapped into sortable records
- verify entry count is preserved
- verify each sorted entry still references the expected option definition

### Scenario 4: Minimal sortable input
A caller invokes sorting with a minimal valid set, such as zero, one, or otherwise trivial numbers of entries supported by the original C behavior.

Expected result:
- the operation completes without introducing invalid reordering
- trivial inputs remain logically unchanged

Test focus:
- test empty and single-entry inputs if supported by the Rust function boundary chosen to mirror the C behavior
- verify no unintended mutation occurs beyond what the C code would do

## Requirements

### Functional Requirements

#### FR-1: Sort bounded option-entry collections
The module shall sort a finite collection of sortable option records corresponding to `struct optsort` entries, as evidenced by `optsort(struct optsort *ops, int n)` in `src/parseopt/help.c`.

#### FR-2: Use name-based ordering semantics from the source module
The module shall reproduce the source module's option-name ordering semantics used for help presentation, as evidenced by the combination of `optsort` and `sortnames` in `src/parseopt/help.c`.

#### FR-3: Support internal name reordering within sortable entries
The module shall support reordering of names within sortable entries when required by the source sorting logic, as evidenced by `sortnames(struct optsort *ops, int i, int j)`.

#### FR-4: Preserve linkage to option definitions
After sorting, each sortable record shall remain linked to the same underlying option definition represented by `struct optdef`, as evidenced by `struct optsort` and nearby `optdef` references in the same file region.

#### FR-5: Preserve collection cardinality
The sorting operation shall neither add nor remove sortable entries; it shall only reorder existing entries, as evidenced by the array-and-count interface of `optsort`.

#### FR-6: Produce deterministic results for identical input
For the same input collection state, the module shall produce the same sorted result each time, as required by its role in help generation and evidenced by its dedicated sorting helpers in `src/parseopt/help.c`.

#### FR-7: Remain suitable for help-generation consumption
The sorted output shall remain usable by the help-generation flow represented by `struct help_context` and related `optdef` usage in `src/parseopt/help.c`.

### Key Entities

#### `optsort`
A sortable option-entry record used as the working unit for help-order processing. It is the direct subject of both `optsort` and `sortnames`. It maintains the data needed to compare entries and to preserve their connection to option definitions.

Relationship:
- references or embeds data derived from `optdef`
- is stored in a collection processed by `optsort`
- is mutated by `sortnames` during ordering

#### `optdef`
An option definition entity representing one logical command-line option entry as used by the help system.

Relationship:
- serves as the source definition linked from sortable records
- remains associated with its sortable wrapper after reordering
- is consumed downstream by help-generation logic

#### `parseopt`
The broader parseopt/help configuration context in which option definitions exist.

Relationship:
- supplies or organizes option-definition data feeding the sort workflow
- is not itself the subject of sorting in this module slice

#### `help_context`
The surrounding help-generation context that consumes sorted option data.

Relationship:
- depends on sorted option-definition order for final help processing
- uses sorted records originating from the `optsort` workflow

## Success Criteria

### SC-1: Behavioral equivalence on ordering
For a shared fixture set derived from `src/parseopt/help.c`, the Rust module produces the same option-entry order as the C module for all tested valid inputs that exercise `optsort`.

### SC-2: Internal name-order equivalence
For fixtures that trigger the helper behavior represented by `sortnames`, the Rust module produces the same internal name arrangement as the C module.

### SC-3: Definition linkage preservation
For every tested sorted entry, the associated option definition remains the same before and after sorting.

### SC-4: Entry-count preservation
For every tested input collection, the number of entries after sorting equals the number before sorting.

### SC-5: Trivial-input correctness
For tested minimal valid inputs supported by the module boundary, the Rust implementation completes successfully and does not introduce incorrect reordering relative to the C behavior.

### SC-6: Help-path usability
The Rust-sorted result can be consumed by the rewritten help-generation path without requiring additional recovery or repair steps to restore option-definition associations or name ordering.