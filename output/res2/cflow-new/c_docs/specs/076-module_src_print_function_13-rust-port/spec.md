# spec.md

## Title

Rust Port Functional Specification: `module_src_print_function_13`

## Metadata

- Project: `cflow-new`
- Module category: `module_cluster`
- Source basis: `src/gnu.c`, `src/output.c`
- Rust branch: `076-module_src_print_function_13-rust-port`
- Generation date: `2026-06-17`

## Overview

This module is responsible for emitting function-name-oriented output for symbols during cflow output generation. The analyzed scope shows two relevant behaviors:

1. Formatting and printing a function name in the GNU-oriented output path.
2. Printing a function entry in the generic output path by delegating to the active output formatting logic.

The Rust rewrite must preserve the observable behavior of these printing responsibilities within the surrounding output system. The ported module is expected to participate in symbol output, not to redefine the project’s symbol model or output architecture.

## In Scope

- Printing a symbol’s function name for output.
- Handling the presence or absence of a subtree indicator when printing function names where supported by the source behavior.
- Printing a function-oriented symbol entry through the output subsystem.
- Interacting with the module’s output-related symbol and driver entities as needed to preserve existing output behavior.

## Out of Scope

- Parsing source code.
- Creating or resolving symbols.
- Designing new output formats or extending the output protocol beyond the evidenced behavior.
- Adding new public APIs unrelated to the current printing responsibilities.
- Non-evidenced guarantees such as concurrency behavior, serialization support, or error recovery features.

## Feature Specification

### Feature: Function-oriented symbol output

The module shall provide the behavior needed to emit a symbol as a function entry in the project’s output flow.

This includes two distinct but related responsibilities evidenced by the source module:

- A GNU-specific function-name printing behavior that receives a symbol and a flag indicating whether the symbol has a subtree.
- A generic function-printing behavior in the output layer that prints a function symbol through the module’s output machinery.

### Functional Behavior

#### 1. GNU-style function name emission

The Rust version must implement behavior equivalent to `print_function_name(Symbol *sym, int has_subtree)`.

Observed responsibilities evidenced by this function signature and placement:

- Accept a symbol to be printed.
- Accept a boolean-like indicator describing whether the symbol has a subtree.
- Emit the function’s printed name according to the GNU output path.
- Preserve any output distinction that depends on the subtree flag.

The Rust port must preserve output differences caused by `has_subtree`; it must not collapse subtree and non-subtree cases into identical behavior unless the source behavior is proven identical.

#### 2. Output-layer function entry emission

The Rust version must implement behavior equivalent to `print_function(Symbol *symp)` in the output layer.

Observed responsibilities evidenced by this function and nearby output entities:

- Accept a function symbol for printing.
- Emit the symbol as a function entry within the module’s output workflow.
- Use the active output formatting behavior represented by the output subsystem rather than inventing independent formatting rules.

The Rust rewrite must preserve the role of this operation as part of the output pipeline for function symbols.

#### 3. Integration with output subsystem entities

The source module contains output-related entities for symbols, output drivers, and linked-list-based output organization. The Rust version must preserve the functional relationships required for function printing:

- Function printing must operate on or through output symbol representations where the module does so.
- Function printing must remain compatible with the active output driver concept used by the output layer.
- If symbol emission depends on linked ordering in the current module flow, the Rust version must preserve the same externally observable print ordering for function entries handled by this module.

No additional output orchestration capabilities may be introduced beyond those needed to preserve the existing module behavior.

## User Scenarios & Testing

### Scenario 1: Print a function name in GNU output mode

A caller has a symbol representing a function and needs the GNU output path to print that function’s name.

Expected behavior:

- The module accepts the symbol.
- The module emits the function name in the GNU-oriented format used by this module.
- The output is appropriate for the supplied subtree indicator.

Tests should verify:

- A function symbol is accepted and printed.
- The emitted output differs appropriately when the subtree indicator changes, if the source behavior distinguishes those cases.
- The output matches the C module’s observable output for the same symbol and subtree condition.

### Scenario 2: Print a function entry through the output layer

A caller in the output subsystem has a symbol representing a function entry to be emitted as part of cflow output.

Expected behavior:

- The module accepts the symbol.
- The module emits the function entry through the output-layer behavior.
- The output remains consistent with the active output formatting arrangement.

Tests should verify:

- A function symbol is printed through the output-layer path.
- The Rust output matches the C output for representative function symbols.
- The printing path remains compatible with the output driver arrangement used by the module.

### Scenario 3: Function output within ordered symbol emission

The output subsystem processes symbols in an established internal order and reaches function symbols that must be emitted.

Expected behavior:

- Function entries are printed when encountered in the existing output flow.
- Their relative order is preserved as observed from the original module behavior.
- No additional entries are inserted and no eligible function entries are skipped.

Tests should verify:

- A sequence containing multiple function symbols produces output in the same order as the C implementation.
- Interactions with linked output organization do not alter externally visible function-entry ordering.

### Scenario 4: Function symbol with and without descendants

A function symbol may or may not have a subtree in the call graph or output tree.

Expected behavior:

- The GNU printing path reflects the subtree state in the same way as the original module.
- The function remains printable in either case.

Tests should verify:

- Both subtree states are accepted.
- Output parity with the original C behavior is maintained for both states.

## Requirements

### Functional Requirements

#### FR-1: Print GNU function names from symbols
The module shall accept a symbol representing a function and emit its function name in the GNU output path.

Traceability:
- `src/gnu.c`
- `print_function_name(Symbol *sym, int has_subtree)`

#### FR-2: Respect subtree-sensitive output behavior
The module shall preserve any output behavior in GNU function-name printing that depends on whether the symbol has a subtree.

Traceability:
- `src/gnu.c`
- `print_function_name(Symbol *sym, int has_subtree)`

#### FR-3: Print function entries in the output layer
The module shall accept a function symbol in the output layer and emit it as a function entry in the module’s output flow.

Traceability:
- `src/output.c`
- `print_function(Symbol *symp)`

#### FR-4: Remain compatible with the active output driver model
Function printing in the output layer shall operate consistently with the module’s output driver concept and shall not bypass the output subsystem’s formatting role.

Traceability:
- `src/output.c`
- `struct output_driver`
- `print_function(Symbol *symp)`

#### FR-5: Preserve function-output ordering as observed through module-managed output structures
Where function printing participates in linked or ordered output traversal in this module, the Rust version shall preserve the same externally observable ordering for function entries.

Traceability:
- `src/output.c`
- `struct linked_list`
- `struct linked_list_entry`
- `print_function(Symbol *symp)`

#### FR-6: Use the module’s symbol-output representations as required for function emission
The Rust version shall preserve the functional role of output-symbol representations used by this module when emitting function-related output.

Traceability:
- `src/gnu.c`
- `src/output.c`
- `struct output_symbol`

### Key Entities

#### Symbol
A symbol is the input entity passed to both identified printing functions. Within this module’s scope, it is the source object from which function-oriented output is produced.

Relationships:
- Consumed by GNU function-name printing.
- Consumed by output-layer function printing.
- May be associated with subtree state for GNU output behavior.

Traceability:
- `print_function_name(Symbol *sym, int has_subtree)`
- `print_function(Symbol *symp)`

#### Output Symbol
An output symbol is a module-local representation involved in symbol emission.

Relationships:
- Supports printed output generation from symbols.
- Exists in both `gnu.c` and `output.c`, indicating output formatting participation in both paths.

Traceability:
- `src/gnu.c`: `struct output_symbol`
- `src/output.c`: `struct output_symbol`

#### Output Driver
An output driver represents the active output formatting behavior in the output layer.

Relationships:
- Governs or participates in how function entries are emitted.
- Serves as the output-layer formatting context with which function printing must remain compatible.

Traceability:
- `src/output.c`: `struct output_driver`
- `print_function(Symbol *symp)`

#### Linked List and Linked List Entry
These entities represent ordered output-related organization within the module.

Relationships:
- Support ordered traversal or storage associated with output generation.
- Constrain the externally visible ordering that function printing must preserve where applicable.

Traceability:
- `src/output.c`: `struct linked_list`
- `src/output.c`: `struct linked_list_entry`

## Success Criteria

### SC-1: GNU function-name output parity
For representative function symbols, the Rust implementation produces the same observable GNU function-name output as the C implementation.

Measured by:
- Golden-output comparison against the C module for identical symbol inputs.

Traceability:
- `src/gnu.c`
- `print_function_name(Symbol *sym, int has_subtree)`

### SC-2: Subtree-state output parity
For representative symbols printed with subtree present and subtree absent, the Rust implementation matches the C implementation’s observable output in both cases.

Measured by:
- Side-by-side output comparison for both subtree states.

Traceability:
- `src/gnu.c`
- `print_function_name(Symbol *sym, int has_subtree)`

### SC-3: Output-layer function entry parity
For representative function symbols emitted through the output layer, the Rust implementation matches the C implementation’s observable function-entry output.

Measured by:
- Golden-output comparison of function-entry emission cases.

Traceability:
- `src/output.c`
- `print_function(Symbol *symp)`

### SC-4: Output-driver-compatible behavior
Function entry emission in Rust remains compatible with the module’s output driver model and does not produce divergent formatting when run in the same output mode as the C implementation.

Measured by:
- Output comparison under the same selected output driver behavior used by the original module.

Traceability:
- `src/output.c`
- `struct output_driver`
- `print_function(Symbol *symp)`

### SC-5: Preserved function-entry ordering
For test inputs that cause multiple function entries to be emitted through this module, the Rust implementation preserves the same observable output order as the C implementation.

Measured by:
- Ordered output comparison across multi-symbol test cases.

Traceability:
- `src/output.c`
- `struct linked_list`
- `struct linked_list_entry`
- `print_function(Symbol *symp)`

### SC-6: No unsupported capability expansion
The Rust port implements the evidenced printing responsibilities of this module without introducing unrelated externally observable features.

Measured by:
- Review of the Rust module interface and behavior against this specification and the traced source scope.

Traceability:
- `src/gnu.c`
- `src/output.c`

## Acceptance Notes

- Acceptance should be based on behavioral parity of emitted output for the scoped function-printing responsibilities.
- Where exact formatting details are determined by broader project context, the Rust port must follow the same context-sensitive behavior as the C module rather than introducing local reinterpretation.
- Any ambiguity discovered during porting should be resolved in favor of matching the original module’s observable output behavior.