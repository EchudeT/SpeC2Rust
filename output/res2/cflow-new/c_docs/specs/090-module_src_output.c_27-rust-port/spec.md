# spec.md

## Title

Functional Specification for `module_src_output.c_27` Rust Port

## Metadata

- Project: `cflow-new`
- Module: `module_src_output.c_27`
- Category: `module_cluster`
- Source file: `src/output.c`
- Target branch: `090-module_src_output.c_27-rust-port`
- Generation date: 2026-06-17

## Overview

This module is responsible for formatted output of cflow symbol information through a selectable output driver interface and for producing cross-reference style output over symbol data.

The Rust rewrite must preserve the module’s observable behavior in these areas:

- registration of named output drivers
- selection of the active output driver by name
- dispatch of output lifecycle and text events to the selected driver
- printing of call-tree indentation markers
- classification of symbols as functions versus variables
- generation of cross-reference output, including symbol ordering, type display, and traversal over related symbol/link information

The specification is limited to functionality evidenced by `src/output.c` and its listed functions and data structures.

## Scope

### In Scope

- Managing a registry of output drivers
- Selecting one active driver from the registry
- Emitting output commands corresponding to initialization, line breaks, section begin/end, separators, and text
- Producing visual indentation for levels in hierarchical output
- Determining whether a symbol should be treated as a variable or function for output purposes
- Producing cross-reference output from symbol/link data maintained elsewhere in the program

### Out of Scope

- Defining symbol parsing, symbol table construction, or file scanning
- Defining the external output driver command enum beyond what is necessary to dispatch supported commands
- Adding new output formats, new public APIs, or new behaviors not evidenced in `src/output.c`

## Feature Specification

### 1. Output Driver Registration and Selection

The module provides a named output-driver mechanism. A driver consists of:

- a driver name
- a handler callback
- driver-specific opaque data

The Rust version must support registering one or more drivers and selecting the active driver by its registered name.

Behavioral expectations evidenced by `register_output`, `select_output_driver`, and `output_init`:

- registration accepts a name, a handler, and handler-private data
- selection searches by name
- initialization prepares the module’s output state so later output operations are sent to the chosen driver
- if a named driver is not found, selection must report failure

### 2. Output Command Dispatch

The module abstracts output operations into command-style dispatches to the active driver.

The Rust version must preserve support for the command categories evidenced by these functions:

- `output_init`
- `newline`
- `begin`
- `end`
- `separator`
- `print_text`

These operations must map to driver callback invocations with the corresponding command identity and required contextual data.

Observable behavior to preserve:

- text output is sent through the active driver rather than written directly by this module
- structural events such as begin/end/separator/newline are likewise delegated through the active driver
- these operations are defined only in terms of driver dispatch behavior; no additional formatting side effects should be introduced beyond what is evidenced

### 3. Hierarchical Level Prefix Printing

The module provides `print_level(int lev, int last)` to render a prefix representing a nesting level and whether the current item is the last child.

The Rust version must preserve the function’s role as a formatter for hierarchical/tree output prefixes.

Observable requirements:

- output depends on both the level value and the `last` flag
- the function emits level-prefix formatting suitable for tree/list presentation
- the function must maintain the same external formatting semantics expected by callers in the project

This specification intentionally does not invent a different tree syntax; the Rust port must match the C module’s observed formatting.

### 4. Symbol Classification for Output

The module distinguishes function symbols from variable symbols for output purposes.

The Rust version must preserve the behavior evidenced by:

- `is_var`
- `symbol_is_function`
- `print_type`

Observable requirements:

- the module can determine whether a symbol should be treated as a variable
- the module exposes a function-oriented classification result through `symbol_is_function`
- type information printed for a symbol depends on the symbol’s classification and stored type-related state

### 5. Cross-Reference Output Generation

The module provides `xref_output()` to generate cross-reference output from the available symbol set and associated linked relationships.

The Rust version must preserve these evidenced behaviors:

- symbols are gathered into an intermediate output-oriented collection
- symbols are compared/sorted for output ordering using the module’s `compare`
- transient per-symbol activity state can be cleared via `clear_active`
- symbol type text is included through `print_type`
- linked-list style relationship data is traversed to emit reference/caller/callee related output
- the output is structured using the same driver-dispatched primitives used elsewhere in the module

The Rust rewrite must maintain the same functional result class: producing ordered cross-reference output over symbols and their relationships.

## User Scenarios & Testing

### Scenario 1: Register and select a named output driver

A caller initializes available output backends by registering a driver under a name, then requests that driver by name before emitting output.

Expected result:

- registration succeeds for a valid driver definition
- selecting the registered name succeeds
- selecting an unknown name fails
- after selection, output operations are routed to the chosen driver

Suggested tests:

- register one driver, select it, and verify subsequent command dispatch reaches that driver
- attempt selection of an unregistered name and verify failure is reported
- register multiple drivers and verify selection activates the requested one

### Scenario 2: Emit structural output through the active driver

A caller uses module output helpers to emit initialization, text, separators, and line breaks.

Expected result:

- `output_init` triggers the driver’s initialization-related command
- `print_text` forwards the provided text content unchanged
- `separator`, `newline`, `begin`, and `end` each trigger the matching driver command
- command ordering observed by the driver matches the caller’s invocation order

Suggested tests:

- use a recording driver and verify the exact sequence of dispatched commands
- verify text payload delivery for representative strings
- verify no command is emitted to a non-selected driver

### Scenario 3: Produce tree level prefixes

A caller generating hierarchical output requests indentation/prefix printing for several nesting depths and for both last-child and non-last-child cases.

Expected result:

- level prefix output varies by depth
- output differs between `last = 0` and `last != 0`
- formatting remains stable for repeated invocations with the same inputs

Suggested tests:

- snapshot output for depth 0, 1, and deeper nesting
- compare outputs for the same depth with differing `last` values
- verify deterministic formatting across repeated runs

### Scenario 4: Classify symbols for output

A caller passes symbols representing different declaration kinds to the module’s classification logic.

Expected result:

- symbols recognized by the C module as variables are still recognized as variables
- symbols recognized by the C module as functions are still recognized as functions
- type printing behavior remains consistent with classification

Suggested tests:

- construct representative function and variable symbols from project fixtures
- verify `symbol_is_function` returns the inverse class of `is_var` where applicable
- verify printed type text matches the symbol class expectations from the C behavior

### Scenario 5: Generate cross-reference output from symbol relationships

A caller requests cross-reference output after symbol and relationship data have been populated by the larger program.

Expected result:

- symbols are emitted in the same ordering class as the C implementation
- symbol type information is included where the C module includes it
- related references are traversed and emitted
- temporary active markers do not persist incorrectly across output generation

Suggested tests:

- run `xref_output` on a fixture symbol graph and compare the full emitted output against the C module
- verify ordering with symbols that differ only in fields used by `compare`
- verify repeated `xref_output` calls do not accumulate stale active-state effects

## Requirements

### Functional Requirements

#### FR-1: Named driver registration
The module shall allow registration of an output driver identified by name and associated with a handler callback and handler-specific data.

Traceability: `register_output`, `struct output_driver`

#### FR-2: Driver selection by name
The module shall allow selecting the active output driver by registered name and shall report failure when the name is not present.

Traceability: `select_output_driver`, `struct output_driver`

#### FR-3: Output initialization dispatch
The module shall provide an initialization operation that dispatches the corresponding command to the active driver.

Traceability: `output_init`

#### FR-4: Structural output dispatch
The module shall provide output operations for newline, begin, end, and separator, each dispatching its corresponding command to the active driver.

Traceability: `newline`, `begin`, `end`, `separator`

#### FR-5: Text output dispatch
The module shall provide an operation to send text content through the active driver.

Traceability: `print_text`

#### FR-6: Hierarchical level formatting
The module shall provide level-prefix formatting based on a nesting level and a last-item indicator.

Traceability: `print_level`

#### FR-7: Variable/function classification
The module shall determine whether a symbol is a variable and shall expose whether a symbol is a function for output logic.

Traceability: `is_var`, `symbol_is_function`

#### FR-8: Symbol type emission
The module shall emit type-related output for a symbol in the same cases and classification-dependent form as the C module.

Traceability: `print_type`

#### FR-9: Output ordering for cross-reference symbols
The module shall compare and order symbols for cross-reference output according to the same comparison behavior as the C module.

Traceability: `compare`, `struct output_symbol`, `xref_output`

#### FR-10: Temporary active-state clearing
The module shall support clearing active state on symbols as part of cross-reference generation.

Traceability: `clear_active`, `xref_output`

#### FR-11: Cross-reference generation
The module shall generate cross-reference output over available symbol data and linked relationships using the module’s output primitives.

Traceability: `xref_output`, `struct output_symbol`, `struct linked_list`, `struct linked_list_entry`

### Key Entities

#### Output Driver
A registered output backend containing:

- a name used for lookup
- a handler callback receiving output commands
- opaque handler-specific data

Relationship:
- one driver may be selected as the active driver for subsequent dispatches

Traceability: `struct output_driver`, `register_output`, `select_output_driver`

#### Output Symbol
An output-oriented symbol record used during cross-reference generation.

Relationship:
- wraps or references symbol data
- participates in sorting/comparison for output ordering
- is consumed by `xref_output`

Traceability: `struct output_symbol`, `compare`, `xref_output`

#### Symbol
A symbol entity originating from the larger program and examined here for output classification and type printing.

Relationship:
- classified by `is_var` and `symbol_is_function`
- has active state manipulated by `clear_active`
- has type-related information consumed by `print_type`
- may participate in cross-reference relationships

Traceability: `is_var`, `symbol_is_function`, `clear_active`, `print_type`, `xref_output`

#### Linked List and Linked List Entry
List structures used to traverse related symbol/reference data during cross-reference output.

Relationship:
- used by `xref_output` to iterate through associated entries
- entries connect symbols to their output relationships

Traceability: `struct linked_list`, `struct linked_list_entry`, `xref_output`

## Success Criteria

### Behavioral Equivalence Criteria

1. Registering and selecting drivers in the Rust module yields success/failure outcomes matching the C module for the same input names.
   - Traceability: `register_output`, `select_output_driver`

2. For a recording test driver, the sequence of dispatched commands produced by `output_init`, `begin`, `print_text`, `separator`, `newline`, and `end` matches the C module for the same call sequence.
   - Traceability: `output_init`, `begin`, `print_text`, `separator`, `newline`, `end`

3. `print_level` produces output equivalent to the C module for representative combinations of level depth and last-item status.
   - Traceability: `print_level`

4. Symbol classification results for representative fixture symbols match the C module’s distinction between variable and function symbols.
   - Traceability: `is_var`, `symbol_is_function`

5. Type-related output for representative symbols matches the C module output in the contexts where `print_type` is used.
   - Traceability: `print_type`

6. Cross-reference output generated from the same fixture symbol graph matches the C module in symbol ordering and emitted relationship content.
   - Traceability: `compare`, `xref_output`, `struct output_symbol`, `struct linked_list`, `struct linked_list_entry`

7. Repeated cross-reference generation over unchanged input does not introduce stale active-state effects not present in the C module.
   - Traceability: `clear_active`, `xref_output`

### Completion Criteria

The Rust port is complete for this module when:

- all functional requirements in this document are implemented
- all success criteria are satisfied by automated comparison tests or equivalent fixture-based validation against C behavior
- no additional unsupported behaviors or APIs are introduced beyond those required by this specification