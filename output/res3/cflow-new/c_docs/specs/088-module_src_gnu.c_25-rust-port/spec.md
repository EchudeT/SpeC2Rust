# spec.md

## Title
Rust Functional Specification for `module_src_gnu.c_25`

## Document Metadata
- Project: `cflow-new`
- Module: `module_src_gnu.c_25`
- Category: `module_cluster`
- Source basis: `src/gnu.c`
- Rust branch: `088-module_src_gnu.c_25-rust-port`
- Generation date: `2026-06-17`

## Overview
This module provides a GNU-style output handler used by the project’s output dispatch system. Its functional role is to react to output commands and write formatted results to a provided output stream.

The Rust rewrite must preserve the module’s command-driven output behavior, including handling of lifecycle/output commands routed through the module’s single handler entry point and use of the module’s symbol-formatting data shape.

## Scope
In scope:
- Command-based output handling for GNU-style output.
- Writing module-produced output to a caller-provided destination.
- Use of module-local symbol output data required by the handler.

Out of scope:
- Defining new output formats.
- Changing the command protocol used by the surrounding system.
- Adding public APIs beyond the handler behavior evidenced by the source module.

## Feature Specification

### Feature: GNU-style output handling
The module accepts output commands through a handler function and performs the GNU output actions associated with those commands.

The Rust version must:
- Provide equivalent behavior for the command-handling entry point represented by `gnu_output_handler`.
- Accept the same categories of input context evidenced by the C interface: an output command, an output destination, a line value, opaque command data, and opaque handler state.
- Produce GNU-style output records to the supplied output target when the command requires output generation.
- Return an integer status result compatible with success/failure style handler dispatch.

### Feature: Stream-directed output
The module writes its results to a caller-supplied output stream rather than managing output destinations itself.

The Rust version must:
- Direct all generated output to the provided writer/stream abstraction supplied by the caller.
- Avoid requiring ownership of global output state not evidenced by this module.

### Feature: Symbol-oriented formatting support
The module defines and uses a local symbol-related data structure to support output formatting.

The Rust version must:
- Represent the functional content of the `output_symbol` structure sufficiently to preserve the handler’s observable behavior.
- Maintain the relationship between command input data and symbol-oriented output formatting.

## User Scenarios & Testing

### Scenario 1: Output dispatcher invokes GNU handler for a printable event
A higher-level output subsystem selects this module’s handler and passes a command corresponding to an output-producing event, along with an output destination and event data.

Expected behavior:
- The handler interprets the command.
- The handler emits GNU-style text to the provided output destination.
- The handler returns a status code indicating whether processing succeeded.

Test coverage:
- Dispatch a known output-producing command to the Rust handler.
- Verify that output is written to the provided sink.
- Verify that a success status is returned for valid input.

### Scenario 2: Output dispatcher invokes handler for a non-printing lifecycle/control event
A higher-level subsystem sends a command used for output lifecycle or control rather than symbol emission.

Expected behavior:
- The handler responds according to the command’s role.
- The handler does not emit unrelated output.
- The handler returns a status consistent with the command outcome.

Test coverage:
- Invoke the Rust handler with supported non-printing/control commands evidenced by the original behavior.
- Verify emitted output is absent or limited to what that command requires.
- Verify return status matches expected success/failure behavior.

### Scenario 3: Handler writes to caller-selected destination
The output subsystem supplies a specific file or writer to collect GNU-style output.

Expected behavior:
- The handler writes only to the supplied destination.
- Output content appears in the exact destination chosen by the caller.

Test coverage:
- Provide an isolated in-memory or temporary-file sink.
- Invoke the handler with an output-producing command.
- Verify content appears in that sink and not elsewhere.

### Scenario 4: Symbol-related command data is formatted through module-local symbol semantics
The output subsystem passes command data associated with symbol output.

Expected behavior:
- The handler uses the symbol-oriented data expected by the module.
- The resulting text reflects the same functional formatting role as in the C module.

Test coverage:
- Provide representative command data matching the original symbol-output path.
- Verify the Rust rewrite produces the same observable output structure as the C behavior for equivalent input.

## Requirements

### Functional Requirements

#### FR-1: Command-dispatch handling
The module shall expose behavior equivalent to `gnu_output_handler`, receiving a command plus associated context and producing a status result.

Traceability:
- `src/gnu.c`
- `gnu_output_handler`

#### FR-2: Caller-directed output writing
When a received command requires output, the module shall write the GNU-style result to the caller-provided output destination.

Traceability:
- `src/gnu.c`
- `gnu_output_handler`

#### FR-3: Command-sensitive behavior
The module shall vary its behavior based on the received output command and shall not treat all commands identically.

Traceability:
- `src/gnu.c`
- `gnu_output_handler`

#### FR-4: Line/context parameter acceptance
The module shall accept the line/context value passed to the handler and preserve any observable behavior tied to that parameter.

Traceability:
- `src/gnu.c`
- `gnu_output_handler`

#### FR-5: Opaque data consumption
The module shall accept command-associated data and handler-associated state passed through opaque pointers/interfaces and use them only as required to reproduce the original observable behavior.

Traceability:
- `src/gnu.c`
- `gnu_output_handler`

#### FR-6: Symbol-oriented output support
The module shall preserve the functional role of the symbol-related formatting data represented by `struct output_symbol` when producing output.

Traceability:
- `src/gnu.c`
- `struct output_symbol`
- `gnu_output_handler`

#### FR-7: Status reporting
The module shall return an integer-compatible status value indicating command handling outcome.

Traceability:
- `src/gnu.c`
- `gnu_output_handler`

### Key Entities

#### Output handler
The module’s primary entity is the GNU output handler entry point. It connects the project’s generic output command mechanism to this module’s GNU-style output behavior.

Relationships:
- Receives commands from the surrounding output system.
- Consumes command data and handler state.
- Writes results to the provided output destination.
- Uses symbol-related data when formatting output.

Traceability:
- `gnu_output_handler`

#### Output symbol
The module defines a local symbol-oriented data structure used to support output formatting.

Relationships:
- Supplies formatting-relevant symbol data to the handler’s output path.
- Exists as supporting data for GNU-style emitted records rather than as an independent feature.

Traceability:
- `struct output_symbol`

## Success Criteria

### SC-1: Behavioral equivalence for supported commands
For each command path implemented by `gnu_output_handler` in `src/gnu.c`, the Rust rewrite produces the same category of observable behavior: output emission versus non-emission, and success versus failure status.

Traceability:
- `gnu_output_handler`

### SC-2: Correct destination usage
In tests using caller-provided sinks, all module-generated output is written to the supplied destination.

Traceability:
- `gnu_output_handler`

### SC-3: Preserved symbol-output semantics
For representative symbol-output inputs corresponding to the original module behavior, the Rust rewrite preserves the observable GNU-style formatting role supported by `output_symbol`.

Traceability:
- `struct output_symbol`
- `gnu_output_handler`

### SC-4: No unsupported interface expansion
The Rust rewrite exposes only the functionality needed to preserve the behavior of the original GNU output handler module and does not require consumers to adopt new module-level capabilities not evidenced by `src/gnu.c`.

Traceability:
- `src/gnu.c`