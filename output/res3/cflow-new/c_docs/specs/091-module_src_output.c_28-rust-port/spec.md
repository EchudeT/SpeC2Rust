# spec.md

## Title

Rust Functional Specification for `module_src_output.c_28`

## Document Metadata

- Project: `cflow-new`
- Source module: `src/output.c`
- Module category: `module_cluster`
- Target Rust branch: `091-module_src_output.c_28-rust-port`
- Generation date: `2026-06-17`

## Overview

This module is responsible for producing the module’s final symbol output through a selectable output driver model. It tracks which symbols are active for emission, organizes symbols through internal linked-list structures, and emits results through the configured output path when the main output routine is invoked.

The Rust rewrite must preserve the same functional role:

- maintain output-driver based formatting/emission behavior,
- mark symbols as active before emission,
- collect or traverse symbol entries in module-managed lists,
- produce output through the module’s main output operation.

This specification covers only behavior evidenced by `src/output.c`, including the main externally relevant output flow and the internal state transitions visible from the identified functions and types.

## Scope

Included in scope:

- symbol activation behavior tied to output preparation,
- output execution initiated by the module’s main output routine,
- management of output-related symbol/list entities needed to drive emission,
- support for driver-style output selection and invocation as represented by the module’s output driver structure.

Excluded from scope:

- features not evidenced in `src/output.c`,
- new public APIs beyond what is required to preserve the module’s current role,
- unrelated parsing, storage, or analysis logic owned by other modules.

## Feature Specification

### Summary

The module provides the project’s output stage for symbol-based results. It maintains output-specific symbol state, uses internal linked lists to organize data for emission, and delegates formatting/output behavior through an output-driver abstraction. The Rust version must reproduce this behavior so that invoking the module’s output flow yields the same class of output decisions and symbol inclusion behavior as the C module.

### Functional Behavior

1. **Output orchestration**
   - The module exposes a primary output operation (`output`) that performs the module’s output phase.
   - This operation must drive the output process using the module’s maintained symbol/list state and the configured driver behavior.

2. **Symbol activation**
   - The module includes symbol activation logic (`set_active`) that marks a symbol as active for output participation.
   - The Rust version must preserve the rule that activation is a distinct state transition applied to symbols before or during output preparation.

3. **Driver-based output selection**
   - The module defines an output driver entity used to represent output behavior.
   - The Rust version must preserve the functional boundary that emission behavior is routed through driver-defined capabilities rather than hard-coded as a single monolithic output mode.

4. **List-based symbol organization**
   - The module uses linked-list based entities to hold output-related entries.
   - The Rust version must preserve the ability to organize and traverse output items in module-managed sequence form sufficient to support output generation.

5. **Output-specific symbol representation**
   - The module defines a dedicated output symbol structure distinct from the generic symbol type.
   - The Rust version must preserve a corresponding internal representation for output-stage symbol handling when needed to maintain behavior.

## User Scenarios & Testing

### Scenario 1: Generate output for currently known symbols

A caller reaches the project stage where symbol relationships or symbol data are ready to be emitted. The module’s main output routine is invoked. The module walks its managed output state, applies active/inactive symbol decisions, and emits output using the configured output driver.

**Expected result**
- Output is produced without requiring the caller to manually iterate symbol entries.
- Symbols not made active according to module rules are not treated as active output participants.
- Driver-selected behavior governs how output is emitted.

**Test focus**
- Invoke the Rust `output` equivalent with prepared symbol state.
- Verify that output is generated and that active symbol handling affects what is emitted.

### Scenario 2: Mark a symbol active before output

During output preparation, a symbol must be marked active through the module’s activation logic. The symbol then participates in subsequent output processing.

**Expected result**
- After activation, the symbol is recognized by later output-stage processing as active.
- Re-applying activation does not produce a conflicting state transition.

**Test focus**
- Apply the Rust equivalent of `set_active` to a symbol used by the output stage.
- Verify that later output processing observes the symbol as active.

### Scenario 3: Emit through a selected output driver

The module is configured to use one of its output driver behaviors. The caller triggers output, and the module emits using the configured driver abstraction rather than bypassing it.

**Expected result**
- Output flow depends on the selected driver entity.
- Driver invocation is part of the output path.

**Test focus**
- Provide test doubles or equivalent internal verification for driver selection.
- Confirm that the chosen driver path is the one used during output.

### Scenario 4: Preserve ordered traversal of output entries

The module builds or maintains linked output entries and then emits them as part of output generation.

**Expected result**
- Output processing traverses the maintained sequence of entries.
- No entries present in the managed output list are skipped without module-defined reason.

**Test focus**
- Construct representative output-entry sequences.
- Verify traversal-based emission behavior over the maintained collection.

## Requirements

### Functional Requirements

- **FR-1: Main output execution**
  - The Rust module shall provide the functional equivalent of the C module’s `output` routine from `src/output.c:424-444`.
  - It shall perform the module’s output phase using module-managed state.

- **FR-2: Symbol activation support**
  - The Rust module shall provide the functional equivalent of `set_active` from `src/output.c:276-280`.
  - It shall be possible for output preparation logic to mark a `Symbol` as active.

- **FR-3: Active state influences output processing**
  - The Rust rewrite shall preserve the behavioral link between symbol activation and later output participation, as evidenced by the presence of dedicated activation logic and output-stage symbol structures in `src/output.c`.

- **FR-4: Driver-mediated output behavior**
  - The Rust module shall preserve the use of an output driver abstraction corresponding to `struct output_driver` (`src/output.c:60-70`).
  - Output behavior shall be routed through the configured driver role.

- **FR-5: Output-stage symbol handling**
  - The Rust module shall preserve an internal output-specific symbol representation corresponding to `struct output_symbol` (`src/output.c:159`), sufficient to support output processing.

- **FR-6: Linked collection management for output items**
  - The Rust module shall preserve the module’s ability to maintain and process linked output collections corresponding to `struct linked_list` (`src/output.c:209`) and `struct linked_list_entry` (`src/output.c:212` and referenced entry usages).
  - Output processing shall be able to traverse the maintained collection of entries.

- **FR-7: Integration of activation, collections, and output**
  - The Rust rewrite shall preserve the module behavior in which symbol activation, output-specific item management, and the final output routine participate in one coherent output flow within `src/output.c`.

### Key Entities

- **Symbol**
  - External or shared symbol entity referenced by `set_active`.
  - Relationship: serves as the base entity whose state is updated for output participation.

- **Output Driver**
  - Represented in C by `struct output_driver`.
  - Relationship: defines the module’s output behavior selection point and is used by the output flow.

- **Output Symbol**
  - Represented in C by `struct output_symbol`.
  - Relationship: output-stage representation associated with symbol emission logic.

- **Linked List**
  - Represented in C by `struct linked_list`.
  - Relationship: container for ordered output-related entries.

- **Linked List Entry**
  - Represented in C by `struct linked_list_entry`.
  - Relationship: node/entity within the output-managed linked collections; used repeatedly through output-related operations.

## Success Criteria

- **SC-1**
  - A Rust implementation of the module can execute the full output phase corresponding to `output` (`src/output.c:424-444`) without omitting the module’s driver-mediated output step.

- **SC-2**
  - A symbol marked through the Rust equivalent of `set_active` (`src/output.c:276-280`) is observed by later output-stage logic as active.

- **SC-3**
  - Output behavior in the Rust module remains structured around an output driver abstraction corresponding to `struct output_driver`, with tests demonstrating that the selected driver path is used during output.

- **SC-4**
  - The Rust module can maintain and traverse output-related entry collections corresponding to `struct linked_list` and `struct linked_list_entry`, and tests confirm traversal reaches all prepared entries in sequence.

- **SC-5**
  - The Rust rewrite includes an internal output-stage symbol handling model corresponding to `struct output_symbol`, and this model is used in output processing rather than collapsing away the module’s evidenced output-stage symbol role.

- **SC-6**
  - End-to-end tests covering activation plus output confirm that symbols intended for output are included according to active-state handling, while non-activated symbols are not incorrectly treated as active by the module.