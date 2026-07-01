# spec.md

## Title

Rust Port Functional Specification: `module_src_output.c_27`

## Overview

This module is responsible for formatted output selection and cross-reference style symbol reporting for the `cflow-new` project.

The Rust rewrite must preserve two functional areas evidenced in `src/output.c`:

1. **Output driver management**
   - Register named output handlers.
   - Select the active handler by name.
   - Initialize output state.
   - Route basic output commands such as begin, end, separator, newline, and text emission through the selected handler.

2. **Cross-reference symbol output**
   - Classify symbols for reporting.
   - Sort symbols for deterministic output.
   - Print symbol type/category information.
   - Traverse symbol relationships and references to emit cross-reference output.

The Rust version must match the observable behavior of this module, including handler selection behavior, command routing, and generation of cross-reference output from the project symbol data made available to this module.

## Scope

### In Scope
- Registration of output drivers identified by name.
- Selection of one registered driver as the active output backend.
- Initialization of output driver state.
- Emission of output control commands and text through the active driver.
- Symbol classification needed by this module’s reporting logic.
- Cross-reference oriented symbol listing and formatting.

### Out of Scope
- Defining the project-wide symbol table model beyond what this module consumes.
- Creating new output formats or extending the output command set beyond what is evidenced.
- Adding concurrency, persistence, recovery, or compatibility layers not present in the source module.

## Source Traceability

Primary traceability for this specification comes from:
- `src/output.c`
- Functions:
  - `print_level`
  - `register_output`
  - `select_output_driver`
  - `output_init`
  - `newline`
  - `begin`
  - `end`
  - `separator`
  - `print_text`
  - `compare`
  - `is_var`
  - `symbol_is_function`
  - `clear_active`
  - `print_type`
  - `xref_output`
- Core data structures:
  - output driver record
  - output symbol record
  - linked list and linked list entry records used during report generation

## Feature Specification

### Feature 1: Named Output Driver Registration and Selection

The module maintains a set of output drivers, each identified by a name and associated with a handler callback plus handler-specific data.

The Rust version must:
- Allow registration of a driver under a textual name.
- Associate each registered driver with:
  - a handler capable of receiving output commands,
  - handler-specific opaque state/data.
- Support selecting the active driver by name.
- Return a status code for registration and selection outcomes.
- Preserve the requirement that subsequent output operations use the currently selected driver.

This feature is evidenced by `register_output`, `select_output_driver`, and the output driver structure.

### Feature 2: Output Lifecycle and Command Dispatch

The module exposes output lifecycle and formatting operations that map to commands sent to the currently selected output driver.

The Rust version must support:
- initialization of output subsystem state,
- beginning an output sequence,
- ending an output sequence,
- emitting separators,
- emitting newlines,
- emitting text payloads.

These behaviors are evidenced by `output_init`, `begin`, `end`, `separator`, `newline`, and `print_text`.

### Feature 3: Indentation/Tree-Level Formatting Support

The module provides a function to print a structural level marker based on a nesting level and whether the current item is the last sibling.

The Rust version must preserve this formatting role so that callers relying on level-based visual structure receive equivalent output behavior.

This feature is evidenced by `print_level`.

### Feature 4: Symbol Classification for Reporting

The module distinguishes symbol kinds needed for output decisions, specifically whether a symbol should be treated as a variable and whether it should be treated as a function.

The Rust version must:
- expose the same function-level behavior for function classification,
- preserve variable classification logic used internally by cross-reference output,
- use these classifications consistently during report generation.

This feature is evidenced by `is_var` and `symbol_is_function`.

### Feature 5: Deterministic Cross-Reference Output

The module produces cross-reference output over symbols using internal intermediate records and linked list traversal.

The Rust version must:
- collect symbols relevant to cross-reference reporting,
- sort them deterministically using the same comparison role as the original module,
- clear temporary active/reporting state as needed between symbols or passes,
- print type/category annotations for symbols,
- emit a full cross-reference report over the available symbols and their relationships.

This feature is evidenced by `compare`, `clear_active`, `print_type`, `xref_output`, and the module-local output and linked-list records.

## User Scenarios & Testing

### Scenario 1: Register and select a known output driver
A caller initializes the output subsystem, registers one or more named drivers, and selects one by its registered name.

Expected behavior:
- registration succeeds for a valid driver definition,
- selecting an existing driver succeeds,
- subsequent output commands are directed to that driver.

Test coverage:
- register a stub driver,
- select it by exact name,
- trigger output operations,
- verify the stub receives the expected command sequence.

### Scenario 2: Attempt to select an unknown driver
A caller requests a driver name that has not been registered.

Expected behavior:
- selection fails with the module’s status indication for failure,
- previously established valid behavior is not replaced by a nonexistent driver.

Test coverage:
- register one driver,
- attempt to select a different unknown name,
- verify failure result,
- verify no output is dispatched to an unknown target.

### Scenario 3: Emit output lifecycle commands
A caller uses the module to begin output, emit text and separators/newlines, and end output.

Expected behavior:
- each operation is translated into the corresponding driver command,
- text payload is passed through intact,
- command ordering is preserved.

Test coverage:
- select a recording driver,
- invoke begin/text/separator/newline/end path,
- assert received command order and associated text content.

### Scenario 4: Produce level-based structural formatting
A caller invokes level printing for nested items, including both non-terminal and terminal siblings.

Expected behavior:
- output reflects the supplied nesting depth,
- output differs appropriately when the item is marked as last versus not last.

Test coverage:
- call the level-printing function with multiple `(level, last)` combinations,
- compare produced output against expected formatting patterns from the original behavior.

### Scenario 5: Classify symbols by report role
A caller or dependent module checks whether a symbol is treated as a function, while cross-reference generation internally distinguishes variables from other symbols.

Expected behavior:
- function-classification results match original module behavior for representative symbols,
- cross-reference generation uses the same classification decisions.

Test coverage:
- construct representative symbol instances for function and non-function cases,
- verify `symbol_is_function`,
- verify report output changes appropriately when symbol kinds differ.

### Scenario 6: Generate cross-reference output for a symbol set
The project provides symbols and reference relationships, and the module emits cross-reference output.

Expected behavior:
- symbols are included in deterministic order,
- type/category information is printed,
- related entries are traversed and emitted,
- temporary active state does not leak across items.

Test coverage:
- build a small symbol graph with multiple symbols and references,
- run cross-reference generation twice,
- verify identical output order and content across runs,
- verify no duplicate or stale active-state effects appear.

## Requirements

### Functional Requirements

#### FR-1: Driver registration
The module shall support registering an output driver under a caller-provided name together with a handler and handler-specific data.
Traceability: `register_output`, output driver record.

#### FR-2: Driver selection
The module shall support selecting the active output driver by registered name and shall report success or failure through its return value.
Traceability: `select_output_driver`, output driver record.

#### FR-3: Output subsystem initialization
The module shall provide initialization that prepares output handling for use before reporting operations occur.
Traceability: `output_init`.

#### FR-4: Output command dispatch
The module shall route output lifecycle and formatting operations to the active output driver, including begin, end, separator, newline, and text emission.
Traceability: `begin`, `end`, `separator`, `newline`, `print_text`.

#### FR-5: Level-format output
The module shall provide level-based formatting output driven by a nesting level and a last-item indicator.
Traceability: `print_level`.

#### FR-6: Function classification
The module shall provide symbol classification behavior that identifies whether a symbol is considered a function.
Traceability: `symbol_is_function`.

#### FR-7: Variable classification for reporting
The module shall determine whether a symbol is treated as a variable for report-generation purposes.
Traceability: `is_var`, `xref_output`.

#### FR-8: Deterministic symbol ordering
The module shall order symbols for cross-reference output using a deterministic comparison rule.
Traceability: `compare`, `xref_output`, output symbol record.

#### FR-9: Temporary active-state clearing
The module shall clear symbol active/report state as required by cross-reference generation so that one symbol’s traversal state does not incorrectly affect another’s output.
Traceability: `clear_active`, `xref_output`.

#### FR-10: Symbol type/category emission
The module shall emit type or category information for symbols during cross-reference reporting.
Traceability: `print_type`, `xref_output`.

#### FR-11: Cross-reference report generation
The module shall generate a cross-reference report over available symbols and their associated relationships/references.
Traceability: `xref_output`, output symbol record, linked list records.

### Key Entities

#### Output Driver
A named output backend definition consisting of:
- driver name,
- handler callback,
- handler-specific data.

Relationship:
- One driver may be selected as the active target for command dispatch.
- Output lifecycle and text operations are routed through the active driver.

Traceability: output driver record, `register_output`, `select_output_driver`.

#### Output Symbol
An internal reporting record used to prepare symbol data for output.

Relationship:
- Participates in sorting and report generation.
- Connects symbol information to cross-reference emission order and formatting.

Traceability: output symbol record, `compare`, `xref_output`.

#### Linked List / Linked List Entry
Internal collection structures used during report generation to organize traversed or grouped items.

Relationship:
- Support assembly and traversal of report content associated with symbols and references.

Traceability: linked list records, `xref_output`.

#### Symbol
A project symbol consumed by this module for classification and reporting.

Relationship:
- May be classified as function or variable.
- May carry type/category information for printed output.
- May participate in cross-reference relationships traversed by the module.

Traceability: `is_var`, `symbol_is_function`, `print_type`, `clear_active`, `xref_output`.

## Success Criteria

### SC-1: Driver registration and selection parity
For a test set of named drivers, the Rust module can register drivers and successfully select an existing one by name, while returning failure for an unregistered name.
Traceability: `register_output`, `select_output_driver`.

### SC-2: Command dispatch fidelity
When a recording driver is active, invoking begin, text, separator, newline, and end operations produces the same command order and text payloads as the C module for the same call sequence.
Traceability: `begin`, `end`, `separator`, `newline`, `print_text`.

### SC-3: Level formatting equivalence
For representative combinations of nesting levels and last-item flags, the Rust module produces the same visible level-format output as the C module.
Traceability: `print_level`.

### SC-4: Classification equivalence
For representative symbol fixtures, the Rust module returns the same function-classification results as the C module and preserves the same variable/non-variable distinctions used in report output.
Traceability: `is_var`, `symbol_is_function`.

### SC-5: Deterministic cross-reference ordering
Given the same symbol input set and relationships, repeated runs of the Rust module produce identical cross-reference ordering and content.
Traceability: `compare`, `xref_output`.

### SC-6: Type/category reporting preservation
Cross-reference output from the Rust module includes the same symbol type/category annotations as the C module for the same symbols.
Traceability: `print_type`, `xref_output`.

### SC-7: No stale active-state effects
In tests with multiple symbols and references, the Rust module does not carry temporary active/reporting state from one symbol’s processing into unrelated later output.
Traceability: `clear_active`, `xref_output`.

### SC-8: End-to-end cross-reference output parity
For a representative project symbol graph, the Rust module’s complete cross-reference output matches the C module’s observable output in content structure and ordering.
Traceability: `xref_output`, output symbol record, linked list records.