# spec.md

## Title

Rust Functional Specification for `src/output.c` Port

## Summary

This module is responsible for output formatting and cross-reference emission for the `cflow-new` project. It manages a registry of named output drivers, selects the active driver, provides common output forwarding operations, classifies symbols for output purposes, and generates cross-reference output over the project symbol set.

The Rust rewrite must preserve the observable behavior evidenced by `src/output.c`, including:

- registration and selection of output drivers by name,
- dispatch of output lifecycle and text events to the selected driver,
- formatting support for nesting/level indicators,
- symbol classification used during output,
- cross-reference emission that traverses symbols and prints associated references.

## Scope

This specification covers the functionality evidenced by the following module file:

- `src/output.c`

It applies to the Rust rewrite on branch:

- `090-module_src_output.c_27-rust-port`

## Feature Specification

### 1. Output driver registry and selection

The module maintains a set of output drivers identified by name. Each driver provides a handler that receives output commands and a caller-supplied driver data pointer.

The Rust version must implement:

- registration of a named driver with its handler and associated driver-specific data,
- lookup and selection of the active driver by its registered name,
- initialization that establishes the module’s default output state.

Behavioral boundaries evidenced by the C module:

- registration returns a status code,
- selection returns a status code,
- output operations are routed through the currently selected driver.

### 2. Driver-command-based output dispatch

The module exposes common output operations that map to driver commands:

- initialize/start of an output section,
- end of an output section,
- emission of separators,
- emission of text,
- emission of line boundaries.

The Rust version must preserve the behavior that these operations are expressed through the active driver rather than hardcoded to one concrete format.

### 3. Level/indentation marker output

The module provides logic to print a visual representation of call depth or tree level, with different treatment for the last element versus non-last elements at a level.

The Rust version must preserve this formatting function’s role as a depth/branch visualizer used by output generation.

### 4. Symbol classification for output

The module classifies symbols to support output decisions. This includes:

- determining whether a symbol represents a variable,
- determining whether a symbol represents a function,
- clearing an "active" state associated with symbols,
- printing symbol type information for output.

The Rust version must preserve equivalent classification behavior as used by the module’s output paths.

### 5. Cross-reference output generation

The module generates cross-reference output by operating over symbols and their related references. Evidence from the helper types and functions shows that this includes:

- collecting symbol-related output records,
- sorting output records for deterministic processing,
- grouping and traversing linked reference data,
- printing symbol identity/type information and its reference lines.

The Rust version must implement cross-reference generation with the same functional responsibilities:
- process the symbol set,
- determine which symbols are eligible for cross-reference output,
- emit formatted output through the module’s output mechanisms,
- produce ordered output based on the module’s comparison logic.

## User Scenarios & Testing

### Scenario 1: Registering and selecting an output format

A caller configures available output formats during startup by registering one or more named output drivers. The caller then selects one registered driver by name before generating output.

The Rust version must support tests that verify:

- a driver can be registered under a name,
- selection succeeds for a registered name,
- selection fails for an unregistered name,
- after selection, subsequent output operations are delivered to the selected driver.

### Scenario 2: Initializing output before use

A caller invokes module initialization before any output is generated.

The Rust version must support tests that verify:

- initialization establishes a usable output state,
- output operations after initialization do not require direct access to driver internals,
- the module behaves consistently when initialization is performed before driver use.

### Scenario 3: Forwarding output lifecycle events

A caller or higher-level output routine begins an output section, emits text and separators, inserts line breaks, and ends the section.

The Rust version must support tests that verify:

- begin/end events are forwarded in order,
- separator and text events are forwarded correctly,
- newline behavior is preserved as a distinct output action.

### Scenario 4: Rendering tree level markers

A caller rendering hierarchical output invokes the level-printing function with a level value and a flag indicating whether the current node is the last at that level.

The Rust version must support tests that verify:

- different marker output is produced for "last" and "non-last" cases,
- output varies with nesting depth,
- the function remains suitable for tree/call-depth rendering.

### Scenario 5: Classifying symbols for output

During output generation, the module determines whether a symbol should be treated as a function or as a variable and may clear active state before reuse.

The Rust version must support tests that verify:

- symbols are classified consistently with the C module’s criteria,
- function classification remains externally available,
- active state clearing changes the symbol state used by subsequent output logic.

### Scenario 6: Producing cross-reference output

A caller requests cross-reference output for the current symbol data. The module traverses symbol entries, organizes output records, and emits symbol/type/reference information in sorted order.

The Rust version must support tests that verify:

- eligible symbols are included in cross-reference output,
- ordering follows the module’s comparison behavior,
- references attached to symbols are emitted in association with the correct symbol,
- symbol type information is included where the module prints it.

## Requirements

### Functional Requirements

#### FR-1: Named driver registration
The module shall allow registration of an output driver by name together with a handler callback and handler-associated data.

**Traceability:** `register_output`, `struct output_driver`

#### FR-2: Driver selection
The module shall allow selection of the active output driver by registered name and report success or failure via status return.

**Traceability:** `select_output_driver`, `struct output_driver`

#### FR-3: Output initialization
The module shall provide module-level initialization for output handling before normal output generation.

**Traceability:** `output_init`

#### FR-4: Driver-mediated line output
The module shall provide a newline operation that dispatches through the active output mechanism.

**Traceability:** `newline`

#### FR-5: Driver-mediated section lifecycle
The module shall support begin and end output events and dispatch them through the active driver.

**Traceability:** `begin`, `end`

#### FR-6: Driver-mediated separator and text emission
The module shall support emission of separators and text through the active driver.

**Traceability:** `separator`, `print_text`

#### FR-7: Level marker formatting
The module shall provide output for hierarchical level visualization based on a depth value and whether the current item is the last at that level.

**Traceability:** `print_level`

#### FR-8: Symbol comparison for ordered output
The module shall compare output symbol records to support deterministic ordering in cross-reference output.

**Traceability:** `compare`, `struct output_symbol`

#### FR-9: Variable classification
The module shall determine whether a symbol is treated as a variable for output purposes.

**Traceability:** `is_var`

#### FR-10: Function classification
The module shall determine whether a symbol is treated as a function for output purposes and expose this classification as module behavior.

**Traceability:** `symbol_is_function`

#### FR-11: Symbol active-state clearing
The module shall clear symbol active state as part of output-related symbol processing.

**Traceability:** `clear_active`

#### FR-12: Symbol type output
The module shall emit symbol type information for a symbol when generating applicable output.

**Traceability:** `print_type`

#### FR-13: Cross-reference generation
The module shall generate cross-reference output over symbols and associated references, using the module’s ordering and output formatting behavior.

**Traceability:** `xref_output`, `struct output_symbol`, `struct linked_list`, `struct linked_list_entry`

### Key Entities

#### Output Driver
A registered output format entry identified by name and associated with:

- a command handler,
- handler-specific data,
- participation in the module’s selectable driver set.

Relationship:
- one output driver may be the active driver at a time for dispatch of output commands.

**Traceability:** `struct output_driver`, `register_output`, `select_output_driver`

#### Output Symbol Record
An intermediate record used for cross-reference output processing and ordering.

Relationship:
- derived from project symbols,
- participates in sorting/comparison before output emission.

**Traceability:** `struct output_symbol`, `compare`, `xref_output`

#### Symbol
A project symbol that may represent a function, variable, or other program entity and may carry state relevant to output.

Relationship:
- classified by output logic,
- may have active state cleared,
- may have type information printed,
- may contribute to cross-reference output.

**Traceability:** `is_var`, `symbol_is_function`, `clear_active`, `print_type`, `xref_output`

#### Linked Reference List
A linked-list structure used to organize associated entries during cross-reference processing.

Relationship:
- contains entries associated with symbols or references,
- traversed during cross-reference emission.

**Traceability:** `struct linked_list`, `struct linked_list_entry`, `xref_output`

## Success Criteria

1. The Rust module can register at least one named output driver and successfully select it by name, while rejecting selection of an unknown name with failure status.
   - **Traceability:** `register_output`, `select_output_driver`

2. After driver selection, begin, end, separator, text, and newline operations are observable through the selected driver in the same call order issued by the module.
   - **Traceability:** `newline`, `begin`, `end`, `separator`, `print_text`

3. The Rust implementation provides hierarchical level-marker output that distinguishes last-child from non-last-child rendering and varies with depth.
   - **Traceability:** `print_level`

4. Symbol classification results used by output logic match the C module’s function/variable determination for equivalent symbol inputs.
   - **Traceability:** `is_var`, `symbol_is_function`

5. Clearing a symbol’s active state changes the symbol state as required for subsequent output processing.
   - **Traceability:** `clear_active`

6. Cross-reference output generated from a representative symbol/reference dataset is ordered according to the module’s comparison behavior and associates references with the correct symbol entries.
   - **Traceability:** `compare`, `xref_output`, `struct output_symbol`, `struct linked_list`, `struct linked_list_entry`

7. Cross-reference output includes symbol type information wherever the C module emits it.
   - **Traceability:** `print_type`, `xref_output`

8. Module initialization completes without preventing subsequent driver selection and output generation.
   - **Traceability:** `output_init`