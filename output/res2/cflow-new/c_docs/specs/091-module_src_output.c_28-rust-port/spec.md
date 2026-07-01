# spec.md

## Title
Rust Port Functional Specification: `module_src_output.c_28`

## Metadata
- Project: `cflow-new`
- Source module: `src/output.c`
- Module category: `module_cluster`
- Target branch: `091-module_src_output.c_28-rust-port`
- Generation date: `2026-06-17`

## Overview
This module is responsible for producing the program’s call-flow output from the symbol set collected by earlier analysis stages. It manages output through a selectable output driver abstraction, tracks per-symbol output state, and walks linked symbol relationships to emit results in the expected order.

The Rust rewrite must preserve the observed functional role of this module:
- determine which symbols are active for output,
- organize traversal through symbol/link relationships,
- invoke the configured output driver lifecycle and per-symbol emission behavior,
- produce complete module output when `output()` is called.

This specification is limited to behavior evidenced by `src/output.c` and the identified functions and data structures.

## Scope
In scope:
- module-level output orchestration
- active-symbol marking used by output processing
- driver-based output emission
- linked-list-backed traversal/state used during output generation

Out of scope:
- symbol discovery or parsing
- command-line processing
- unrelated formatting or reporting not evidenced in this module
- adding new output formats or public APIs beyond the existing module role

## Feature Specification

### Feature 1: Output orchestration
The module provides a top-level output operation that generates final output for the current analyzed symbol graph.

#### Behavior
- A call to `output()` must perform the module’s complete output pass.
- The pass must use the module’s configured driver abstraction to emit output.
- The pass must consider symbol activity state rather than blindly emitting every symbol.
- The pass must traverse symbol relationships through the linked structures represented in this module.

#### Traceability
- `output` in `src/output.c:424-444`
- `struct output_driver`
- `struct output_symbol`
- `struct linked_list`
- `struct linked_list_entry`

### Feature 2: Active symbol marking
The module supports marking a symbol as active for participation in output processing.

#### Behavior
- The module must be able to mark an individual symbol active.
- Active-state marking must affect later output behavior.
- Repeated activation of the same symbol must not require duplicate output-state creation if the original module avoids duplicate activation effects.

#### Traceability
- `set_active` in `src/output.c:276-280`
- `struct output_symbol`

### Feature 3: Driver-directed output
The module uses a driver abstraction to separate traversal/orchestration from output formatting or emission strategy.

#### Behavior
- The Rust version must preserve driver-mediated output behavior represented by the source module.
- Output generation must call into the driver at the same functional stages represented by the source module’s driver usage.
- The module must remain capable of supporting the output behavior represented by the existing driver structure without embedding all formatting directly into traversal logic.

#### Traceability
- `struct output_driver` in `src/output.c:60-70`
- `output` in `src/output.c:424-444`

### Feature 4: Relationship-driven traversal
The module uses linked-list entities to manage traversal or staging of symbols during output generation.

#### Behavior
- The Rust version must preserve the relationship-driven nature of output generation.
- Linked traversal order and grouping semantics used by the C module must be preserved where they affect emitted results.
- The module must maintain enough internal state to avoid losing pending traversal entries during output.

#### Traceability
- `struct linked_list` in `src/output.c:209`
- `struct linked_list_entry` in `src/output.c:212` and referenced local usages
- `output` in `src/output.c:424-444`

## User Scenarios & Testing

### Scenario 1: Generate output for an analyzed symbol set
A caller has already completed analysis and populated the symbol structures consumed by this module. The caller invokes the module’s top-level output operation.

Expected result:
- the module completes one output pass,
- eligible symbols are emitted through the configured driver,
- symbol relationship traversal is reflected in the produced output,
- inactive symbols are not treated as active solely by output invocation unless the source behavior does so explicitly.

Relevant evidence:
- `output()`
- output-driver and linked-list entities

### Scenario 2: Mark a symbol active before output
A caller or prior internal step marks a symbol active using the module’s active-state logic, then runs output.

Expected result:
- the marked symbol is included in output processing according to the module’s traversal and driver rules,
- the symbol’s active state influences selection during the output pass.

Relevant evidence:
- `set_active()`
- `output_symbol`

### Scenario 3: Output with interconnected symbols
The analyzed data includes symbols connected by call or dependency relationships represented through linked entries.

Expected result:
- output follows the relationship structure used by the module,
- traversal does not omit reachable entries that the original module would emit,
- output ordering and nesting/grouping remain consistent with the source module’s functional behavior.

Relevant evidence:
- linked-list entities
- `output()`

### Scenario 4: Output through the selected driver
The module is configured with one of its supported output drivers and then invoked.

Expected result:
- driver callbacks or equivalent driver behavior are used for emission,
- traversal/orchestration remains in the module,
- emitted output corresponds to the selected driver’s role.

Relevant evidence:
- `output_driver`
- `output()`

### Testing guidance
The Rust port should be tested with fixtures that vary:
- active vs inactive symbols,
- isolated vs linked symbols,
- repeated references to the same symbol,
- different driver selections represented by the original module.

Tests should verify:
- which symbols are emitted,
- whether relationship traversal is preserved,
- whether output is routed through the driver abstraction,
- whether one complete call to `output()` performs a complete output pass.

## Requirements

### Functional Requirements

#### FR-1: Top-level output pass
The module shall provide a top-level output operation that executes the complete output generation behavior for the current symbol data set.

Traceability:
- `output` in `src/output.c:424-444`

#### FR-2: Symbol activation
The module shall support marking a symbol as active for output-related processing.

Traceability:
- `set_active` in `src/output.c:276-280`

#### FR-3: Active-state influence
The module shall use symbol active state when determining output processing behavior.

Traceability:
- `set_active`
- `output_symbol`
- `output`

#### FR-4: Driver-based emission
The module shall perform output through the output-driver abstraction represented in the source module.

Traceability:
- `struct output_driver`
- `output`

#### FR-5: Relationship-aware traversal
The module shall traverse symbol/output relationships using internal linked structures equivalent in behavior to the source module’s linked-list entities.

Traceability:
- `struct linked_list`
- `struct linked_list_entry`
- `output`

#### FR-6: Preservation of output-relevant ordering/grouping
The module shall preserve any ordering or grouping semantics imposed by the linked traversal and driver coordination where such semantics affect emitted output.

Traceability:
- `struct linked_list`
- `struct linked_list_entry`
- `output`

#### FR-7: Per-symbol output state tracking
The module shall maintain per-symbol output state sufficient to support activation and output traversal behavior represented by the source module.

Traceability:
- `struct output_symbol`
- `set_active`
- `output`

### Key Entities

#### Output driver
Represents the selectable output behavior used by the module to emit results. Its role is to define output-stage actions while the module retains responsibility for orchestration.

Traceability:
- `struct output_driver` in `src/output.c:60-70`

#### Output symbol
Represents output-related state attached to or derived from a symbol, including state needed for activation and participation in the output pass.

Traceability:
- `struct output_symbol` in `src/output.c:159`

#### Linked list
Represents internal sequencing or staging of traversal items used during output generation.

Traceability:
- `struct linked_list` in `src/output.c:209`

#### Linked list entry
Represents an individual traversal or relationship entry within the module’s internal output-processing lists.

Traceability:
- `struct linked_list_entry` in `src/output.c:212` and later local references

#### Symbol relationship
A symbol participates in output through its associated output state and through linked traversal entries that determine how it is visited and emitted.

Traceability:
- `set_active`
- `output`
- `output_symbol`
- `linked_list_entry`

## Success Criteria

### SC-1: Complete output invocation
Given prepared symbol/input state equivalent to what `src/output.c` consumes, one invocation of the Rust module’s top-level output function completes one full output pass without requiring extra external traversal steps.

Traceability:
- `output`

### SC-2: Activation affects output
When a symbol is marked active through the Rust port’s equivalent of the source behavior, subsequent output processing reflects that active state in symbol selection or traversal.

Traceability:
- `set_active`
- `output`

### SC-3: Driver abstraction preserved
The Rust port routes output emission through an output-driver abstraction equivalent in role to `struct output_driver`, rather than collapsing all output behavior into a single hard-coded path.

Traceability:
- `struct output_driver`
- `output`

### SC-4: Relationship traversal preserved
For fixtures containing linked symbol relationships, the Rust port emits output that reflects the same relationship-driven traversal semantics as the C module.

Traceability:
- `struct linked_list`
- `struct linked_list_entry`
- `output`

### SC-5: Per-symbol output state preserved
The Rust port maintains per-symbol output state sufficient to support activation and output behavior without losing symbol participation across the output pass.

Traceability:
- `struct output_symbol`
- `set_active`
- `output`

### SC-6: No unsupported feature expansion
The Rust rewrite does not introduce new externally visible module capabilities beyond the evidenced behaviors of activation, traversal, driver-mediated emission, and top-level output orchestration.

Traceability:
- `set_active`
- `output`
- `output_driver`
- linked-list and output-symbol entities

## Non-Goals
The Rust port is not required by this specification to provide:
- new public configuration mechanisms,
- additional output formats beyond those represented by the existing driver model,
- concurrency guarantees,
- persistence or serialization of output state,
- error recovery features not evidenced in the source module.

## Notes for Porting
- Preserve functional behavior even if internal Rust representations differ from C structs.
- Prefer explicit Rust modeling of symbol output state, driver behavior, and traversal lists.
- Do not remove the distinction between orchestration and emission if that distinction is required to preserve source behavior.