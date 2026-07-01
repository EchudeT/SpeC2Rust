# Functional Specification: `module_src_posix.c_33`

## Document Control
- **Project**: `cflow-new`
- **Module**: `module_src_posix.c_33`
- **Category**: `module_cluster`
- **Source Basis**: `src/posix.c`
- **Rust Target Branch**: `096-module_src_posix.c_33-rust-port`
- **Generation Date**: 2026-06-11

## Overview
This module provides a POSIX-style output handler for symbol reporting. It receives output commands from the surrounding cflow output pipeline and emits formatted symbol information to a provided output stream.

The module has two observable responsibilities:
1. Determine the textual symbol type marker for a symbol.
2. Handle output commands by formatting and writing a POSIX-style line for symbol data when requested.

The Rust rewrite must preserve the same externally visible behavior: command-driven output handling, use of symbol metadata to derive a type marker, and emission of formatted output to the caller-supplied destination.

## Scope
Included in scope:
- POSIX-style formatting of symbol output.
- Mapping from symbol classification to printed type marker.
- Command-based dispatch for output behavior.
- Use of line number and symbol-related data when producing output.

Out of scope:
- Symbol creation, parsing, or semantic analysis.
- Ownership of the output destination.
- Non-POSIX output formats.
- Capabilities not evidenced by this module, including new APIs or extended formatting modes.

## Feature Specification

### Feature: POSIX-style symbol output handling
The module acts as an output backend that is invoked through a command-oriented handler interface. When called with symbol output data, it writes a line in the module’s POSIX-oriented format to the provided output stream.

The Rust version must:
- Accept the same class of handler inputs: a command, an output destination, a line number, opaque input data for the command, and handler-specific data.
- Recognize and process the command cases supported by this module.
- For the symbol-output case, format output using symbol information and a symbol type marker.
- Return an integer status compatible with the handler contract used by the surrounding system.

### Feature: Symbol type marker emission
The module derives a textual type indication for a symbol and writes it as part of the POSIX-formatted output.

The Rust version must:
- Preserve the behavior of symbol type selection as represented by the source module.
- Emit the same symbol type marker for the same symbol classification inputs.
- Use that marker within the generated output line.

## User Scenarios & Testing

### Scenario 1: Output pipeline emits a symbol record
A caller in the cflow output pipeline invokes the POSIX output handler with a command indicating that a symbol record should be written, along with:
- an output destination,
- a source line number,
- symbol-related data.

Expected behavior:
- The handler formats a single POSIX-style record.
- The record includes a symbol type marker derived from the symbol.
- The record is written to the supplied output destination.
- The handler returns its defined status code.

Testing focus:
- Verify that a valid symbol input produces output.
- Verify that line-associated formatting is preserved.
- Verify that the type marker in the output matches the input symbol category.

### Scenario 2: Symbol kinds differ
The caller provides symbol records whose symbol classifications differ.

Expected behavior:
- The handler emits different type markers when the source symbols belong to different supported classifications.
- Output remains structurally consistent aside from the symbol-specific fields.

Testing focus:
- Provide representative symbol instances spanning the classifications handled by `print_symbol_type`.
- Verify exact marker selection for each case.

### Scenario 3: Handler receives a command that does not require symbol line output
The caller invokes the handler with another command value supported by the module contract.

Expected behavior:
- The handler responds according to the source module’s defined behavior for that command.
- It does not emit unintended symbol lines for commands that are not symbol-output requests.

Testing focus:
- Exercise all command cases evidenced by the source function behavior.
- Verify return status and output side effects for each supported command.

### Scenario 4: Output is directed to a caller-provided destination
The caller provides a writable output destination distinct from standard output.

Expected behavior:
- The module writes only to the supplied destination.
- Produced text is suitable for capture and comparison by the caller.

Testing focus:
- Use an in-memory or temporary output sink.
- Verify that emitted output appears in the provided sink and not elsewhere.

## Requirements

### Functional Requirements
- **FR-1**: The module shall provide a POSIX output handler entry point that accepts a command, an output destination, a line number, command data, and handler data.
  **Traceability**: `posix_output_handler` in `src/posix.c`.

- **FR-2**: The module shall interpret handler commands according to the behavior defined in the source module and return an integer status result for each invocation.
  **Traceability**: `posix_output_handler` in `src/posix.c`.

- **FR-3**: For symbol-output handling, the module shall generate output using symbol-related input data and write the formatted result to the provided output destination.
  **Traceability**: `posix_output_handler` and `struct output_symbol` in `src/posix.c`.

- **FR-4**: The module shall determine a symbol type marker from a symbol instance and emit that marker in the produced output.
  **Traceability**: `print_symbol_type` in `src/posix.c`.

- **FR-5**: The module shall preserve the POSIX-style formatting behavior represented by the source module for the supported command path(s).
  **Traceability**: `posix_output_handler`, `print_symbol_type`, and `struct output_symbol` in `src/posix.c`.

### Key Entities
- **Symbol**
  - Represents the symbol whose information is being output.
  - Supplies the classification consumed by symbol type marker generation.
  - **Traceability**: Used by `print_symbol_type` and symbol-output handling in `src/posix.c`.

- **Output Symbol Record (`struct output_symbol`)**
  - Represents the grouped data consumed by the output handler for symbol line generation.
  - Relates symbol information to line-oriented output formatting.
  - **Traceability**: `struct output_symbol` in `src/posix.c`.

- **Output Command**
  - Selects the handler behavior to perform on each invocation.
  - Governs whether symbol output formatting is executed.
  - **Traceability**: `posix_output_handler` in `src/posix.c`.

- **Output Destination**
  - Caller-supplied writable stream receiving formatted POSIX output.
  - **Traceability**: `FILE *outfile` parameters in `print_symbol_type` and `posix_output_handler`.

## Success Criteria
- **SC-1**: For every command case supported by the source module, the Rust implementation returns behaviorally equivalent status results and output side effects when given equivalent inputs.
  **Traceability**: `posix_output_handler` in `src/posix.c`.

- **SC-2**: For representative symbols covering each classification handled by the source module, the Rust implementation emits the same symbol type marker text as the source behavior.
  **Traceability**: `print_symbol_type` in `src/posix.c`.

- **SC-3**: For symbol-output invocations, the Rust implementation writes POSIX-style formatted records to the caller-provided output destination using the provided symbol-related data and line context.
  **Traceability**: `posix_output_handler` and `struct output_symbol` in `src/posix.c`.

- **SC-4**: Automated tests comparing captured output from the Rust implementation against expected POSIX-style output strings pass for supported command scenarios and symbol classification variants.
  **Traceability**: `src/posix.c`.