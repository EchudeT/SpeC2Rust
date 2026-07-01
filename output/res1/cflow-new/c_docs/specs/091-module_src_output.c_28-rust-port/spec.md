# Specification: `module_src_output.c_28`

- **Project**: `cflow-new`
- **Module**: `src/output.c`
- **Category**: `module_cluster`
- **Target Rust branch**: `091-module_src_output.c_28-rust-port`
- **Generation date**: 2026-06-11

## 1. Feature Specification

This module is responsible for producing the module’s final output from the project’s symbol/call-flow state. The analyzed entrypoint is `output()`, and the module also includes internal state management centered on `set_active(Symbol *sym)` and module-local output-related structures.

The Rust rewrite must implement the same functional role:

- maintain output-related per-symbol state used during emission,
- track and traverse output work items using linked-list style collections,
- select and activate symbols for output processing,
- dispatch formatting/output behavior through an output-driver abstraction,
- generate the final output when the module entrypoint is invoked.

The evidence for this scope is limited to:

- `output()` as the public module action,
- `set_active(Symbol *sym)` as internal symbol activation,
- module-local structures for:
  - output drivers,
  - output symbols,
  - linked lists and linked-list entries.

No additional capabilities are specified beyond what is supported by these functions and structures.

## 2. User Scenarios & Testing

### Scenario 1: Produce final output for the current analyzed symbol graph
A caller invokes the module’s main output routine after the broader program has prepared symbol data.

Expected behavior:
- the module walks the relevant symbol set,
- marks symbols as active as needed for output processing,
- uses the configured output driver behavior,
- emits the final output representation for the current program state.

Tests should verify:
- calling `output()` completes successfully for a prepared symbol set,
- output is generated through the active driver path,
- symbols that must participate in emission are processed by the module.

### Scenario 2: Activate a symbol before or during output traversal
During output generation, a symbol is selected for participation in the emitted result.

Expected behavior:
- the module updates that symbol’s output-related active state through `set_active`,
- subsequent output processing treats the symbol as active.

Tests should verify:
- activating a symbol changes only its output participation state relevant to this module,
- an activated symbol is included in later output traversal/emission where applicable,
- repeated activation does not cause incorrect duplicate state transitions.

### Scenario 3: Traverse pending output items in linked-list order
The module manages output work using linked-list structures and entries.

Expected behavior:
- output-related items can be collected in linked-list form,
- traversal consumes or iterates them in a stable list-based sequence,
- list membership is sufficient for the module to find symbols/items to emit.

Tests should verify:
- linked-list-backed work sets can be traversed without losing entries,
- empty and non-empty lists are handled correctly,
- output behavior is consistent with the list contents provided to the module.

### Scenario 4: Support multiple output driver behaviors through a common abstraction
The module contains an output-driver structure, indicating output is performed through interchangeable driver-defined behavior.

Expected behavior:
- the output path uses the driver abstraction rather than hard-coded formatting only,
- the selected driver controls the emitted output style/behavior expected by the surrounding program.

Tests should verify:
- driver selection affects which output behavior is executed,
- the module can invoke the configured driver for a valid output run,
- unsupported or absent driver state is handled according to existing module behavior in the C source during port validation.

## 3. Requirements

### 3.1 Functional Requirements

#### FR-1: Final output entrypoint
The Rust module shall provide the functional equivalent of `output()` from `src/output.c:424-444`, serving as the module’s main operation for generating output from the current symbol/call-flow state.

**Traceability**: `output` in `src/output.c:424-444`

#### FR-2: Symbol activation for output participation
The Rust module shall implement the functional equivalent of `set_active(Symbol *sym)` from `src/output.c:276-280` to mark a symbol as active for this module’s output processing.

**Traceability**: `set_active` in `src/output.c:276-280`

#### FR-3: Driver-based output behavior
The Rust module shall preserve the output-driver role represented by the module’s `struct output_driver`, so output generation is performed according to driver-defined behavior used by this module.

**Traceability**: `struct output_driver` in `src/output.c:60-66`, `src/output.c:70`; `output` in `src/output.c:424-444`

#### FR-4: Per-symbol output state
The Rust module shall preserve the role of `struct output_symbol` as module-specific symbol-associated state used during output generation and/or symbol activation.

**Traceability**: `struct output_symbol` in `src/output.c:159`; `set_active` in `src/output.c:276-280`; `output` in `src/output.c:424-444`

#### FR-5: Linked-list-based output work tracking
The Rust module shall preserve the functional use of `struct linked_list` and `struct linked_list_entry` for maintaining and traversing collections of output-related items required by this module’s processing.

**Traceability**: `struct linked_list` in `src/output.c:209`; `struct linked_list_entry` in `src/output.c:212, 283, 289, 302, 327, 362`; `output` in `src/output.c:424-444`

#### FR-6: Consistent processing of active symbols during output
The Rust rewrite shall ensure symbols activated for output participation are processed consistently by the main output routine.

**Traceability**: `set_active` in `src/output.c:276-280`; `output` in `src/output.c:424-444`; `struct output_symbol` in `src/output.c:159`

### 3.2 Key Entities

#### Output Driver
A module-local abstraction representing the behavior used to generate output. The presence of `struct output_driver` indicates that output formatting/emission is not monolithic and is selected or dispatched through driver-defined behavior.

**Traceability**: `struct output_driver` in `src/output.c:60-66`, `src/output.c:70`

#### Output Symbol
A module-local representation of symbol-related state used during output processing. It relates the broader program symbol model to this module’s output-specific needs, including activation/participation tracking.

**Traceability**: `struct output_symbol` in `src/output.c:159`; `set_active` in `src/output.c:276-280`

#### Linked List
A module-local collection used to organize output processing items. It provides the container role for traversal by the output logic.

**Traceability**: `struct linked_list` in `src/output.c:209`

#### Linked List Entry
An item node associated with the linked-list collection and used repeatedly across output-processing paths in this module.

**Traceability**: `struct linked_list_entry` in `src/output.c:212, 283, 289, 302, 327, 362`

#### Symbol
An external project entity accepted by `set_active(Symbol *sym)` and used by this module as the subject of output participation.

**Traceability**: `set_active` in `src/output.c:276-280`

### Relationships

- An **Output Driver** defines how the module emits output.
- An **Output Symbol** stores output-related state associated with a **Symbol**.
- **Linked List** and **Linked List Entry** organize collections of output-related items, including symbol-associated work, for traversal by `output()`.
- `set_active` updates symbol/output-symbol state that affects later handling by `output()`.

## 4. Success Criteria

### SC-1: Functional equivalence of main output behavior
For program states that reach `src/output.c` in the C implementation, the Rust module’s main output operation produces the same category of final output behavior as the C `output()` routine.

**Traceability**: `output` in `src/output.c:424-444`

### SC-2: Symbol activation affects output participation
When a symbol is activated through the Rust equivalent of `set_active`, subsequent output processing reflects that activation in the same way as the C module.

**Traceability**: `set_active` in `src/output.c:276-280`; `output` in `src/output.c:424-444`

### SC-3: Driver-based dispatch is preserved
The Rust rewrite preserves the effective behavior boundary of `struct output_driver`, and output execution follows the configured driver path used by the module.

**Traceability**: `struct output_driver` in `src/output.c:60-66`, `src/output.c:70`; `output` in `src/output.c:424-444`

### SC-4: Linked-list-managed items remain traversable and correctly processed
Collections represented in C by `linked_list` and `linked_list_entry` can be traversed and used for output processing in Rust without dropping, duplicating, or skipping valid items relative to the original module behavior.

**Traceability**: `struct linked_list` in `src/output.c:209`; `struct linked_list_entry` in `src/output.c:212, 283, 289, 302, 327, 362`; `output` in `src/output.c:424-444`

### SC-5: Output-specific per-symbol state is preserved
The Rust rewrite preserves the observable role of `output_symbol` state in output generation and activation handling.

**Traceability**: `struct output_symbol` in `src/output.c:159`; `set_active` in `src/output.c:276-280`; `output` in `src/output.c:424-444`