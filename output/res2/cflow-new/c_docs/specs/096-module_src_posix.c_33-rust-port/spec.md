# spec.md

## Title

Rust Port Functional Specification: `module_src_posix.c_33`

## Summary

This module provides POSIX-style output formatting for symbol records produced by the wider `cflow-new` system. Its behavior is centered on handling output commands and rendering one symbol entry to a text stream in a POSIX-oriented format.

The Rust rewrite on branch `096-module_src_posix.c_33-rust-port` must preserve the observable formatting behavior and command handling supported by the C module in `src/posix.c`.

## Scope

### In Scope
- Formatting and writing POSIX-style symbol output to a provided output stream.
- Handling the module’s supported output command dispatch behavior.
- Mapping symbol metadata to the textual type marker used in output.
- Using the per-call symbol payload associated with output handling.

### Out of Scope
- Symbol discovery, parsing, or semantic analysis.
- Ownership or lifecycle management of the wider symbol table beyond what is needed to read symbol data passed into this module.
- Defining new output modes, public APIs, or command types not evidenced by `src/posix.c`.

## Source Basis

This specification is derived from:
- `src/posix.c`
- Functions:
  - `print_symbol_type`
  - `posix_output_handler`
- Data structure:
  - `struct output_symbol`

## Feature Specification

### Feature: POSIX-style symbol output

The module acts as an output handler that receives commands from the surrounding output framework and writes formatted text to a supplied output stream.

The Rust version must implement the same functional role:
- accept output-handler command invocations,
- interpret the symbol-bearing invocation data needed for output,
- write the expected POSIX-style symbol line content,
- include a symbol type marker derived from symbol metadata.

### Feature: Symbol type marker selection

The module determines a textual type representation for a symbol before writing the symbol entry. The Rust version must preserve the same category-to-output mapping behavior evidenced by `print_symbol_type`.

This feature is limited to rendering the symbol type indicator used by this module’s output. It does not require introducing a new symbol taxonomy.

### Feature: Output record composition from handler data

The module uses a dedicated output record structure (`struct output_symbol`) as the data carrier associated with output operations. The Rust rewrite must preserve the ability to consume the equivalent record fields required to format output for one symbol entry.

## User Scenarios & Testing

### Scenario 1: Emit one symbol entry through the output handler

A caller in the main cflow output pipeline invokes the POSIX output handler with the command used for symbol emission, a valid output stream, and symbol-related data.

Expected behavior:
- The handler recognizes the command as one that requires output.
- It reads the provided symbol-bearing data record.
- It writes one correctly formatted POSIX-style symbol entry to the output stream.
- The output includes the symbol type marker derived from the symbol metadata.

Testing guidance:
- Provide a symbol with known type/category metadata.
- Invoke the handler with the symbol-output command.
- Verify that exactly the expected formatted text is written.

### Scenario 2: Emit multiple symbol entries sequentially

A caller invokes the handler repeatedly for multiple symbols using the same output stream.

Expected behavior:
- Each invocation produces one correctly formatted output record.
- Output order matches invocation order.
- No invocation corrupts the formatting of subsequent records.

Testing guidance:
- Invoke the handler for at least three distinct symbols.
- Verify line-by-line output order and formatting.

### Scenario 3: Handle a non-output or unsupported command

A caller invokes the handler with a command that does not correspond to symbol-line emission.

Expected behavior:
- The handler responds according to the command-dispatch behavior implemented in `posix_output_handler`.
- It does not emit a symbol line unless the command requires it.

Testing guidance:
- Exercise each command path evidenced by the C function.
- Verify emitted output and return status for each path.

### Scenario 4: Distinguish symbol type markers

A caller emits symbols whose metadata causes different type markers to be selected.

Expected behavior:
- The marker written for each symbol matches the same mapping decisions as the C implementation’s `print_symbol_type`.
- Symbols from different supported categories produce distinct output where the C module does so.

Testing guidance:
- Build test cases that cover each symbol category branch evidenced in `print_symbol_type`.
- Compare Rust output against C output for the same inputs.

## Requirements

### Functional Requirements

#### FR-1: Output handler command processing
The module shall provide an output-handler function that accepts:
- an output command,
- an output stream,
- a line-related integer parameter,
- generic per-call data,
- generic handler data,

and shall perform the command-specific behavior implemented by `posix_output_handler`.

Traceability:
- `src/posix.c`
- `posix_output_handler`

#### FR-2: Symbol-line emission
When invoked with the command path that emits symbol output, the module shall write one POSIX-style textual record for the provided symbol data to the supplied output stream.

Traceability:
- `src/posix.c`
- `posix_output_handler`
- `struct output_symbol`

#### FR-3: Symbol type rendering
The module shall derive and emit the symbol type indicator using the same symbol classification behavior implemented by `print_symbol_type`.

Traceability:
- `src/posix.c`
- `print_symbol_type`

#### FR-4: Use of output symbol payload
The module shall consume the symbol-related payload represented by `struct output_symbol` to obtain the data needed for record formatting.

Traceability:
- `src/posix.c`
- `struct output_symbol`
- `posix_output_handler`

#### FR-5: Stream-directed output
The module shall direct all formatted output to the caller-provided output stream and shall not require an internally owned destination.

Traceability:
- `src/posix.c`
- `print_symbol_type`
- `posix_output_handler`

#### FR-6: Return behavior preservation
The Rust rewrite shall preserve the observable success/failure return behavior of `posix_output_handler` for the command paths evidenced in the C module.

Traceability:
- `src/posix.c`
- `posix_output_handler`

### Key Entities

#### Entity: Symbol
A symbol is the semantic item being rendered. The module reads symbol metadata to determine the textual type marker used in output.

Relationship:
- Consumed by the type-rendering logic.
- Referenced by the output payload used by the handler.

Traceability:
- `print_symbol_type`
- `struct output_symbol`

#### Entity: Output Symbol Record
`struct output_symbol` is the per-output record used to carry the symbol-related data needed by the handler to format one output entry.

Relationship:
- Passed into the output handler through generic call data.
- Contains or references the symbol rendered by the module.

Traceability:
- `src/posix.c`
- `struct output_symbol`
- `posix_output_handler`

#### Entity: Output Command
The output command identifies which handler behavior to execute.

Relationship:
- Input to the handler dispatcher.
- Determines whether symbol output is emitted.

Traceability:
- `posix_output_handler`

#### Entity: Output Stream
The output stream is the destination receiving formatted POSIX-style text.

Relationship:
- Provided by the caller.
- Used by both command handling and symbol type rendering.

Traceability:
- `print_symbol_type`
- `posix_output_handler`

## Success Criteria

### SC-1: Output equivalence for supported command paths
For every command path implemented in `posix_output_handler`, the Rust module produces the same observable output text and return result as the C module for equivalent inputs.

Traceability:
- `src/posix.c`
- `posix_output_handler`

### SC-2: Type marker equivalence
For every symbol classification branch evidenced in `print_symbol_type`, the Rust module emits the same type marker text as the C module.

Traceability:
- `src/posix.c`
- `print_symbol_type`

### SC-3: One-record-per-emission invocation
For each symbol-emission invocation, the Rust module writes exactly one correctly formatted symbol record using the provided output payload and stream.

Traceability:
- `src/posix.c`
- `posix_output_handler`
- `struct output_symbol`

### SC-4: Sequential stability
Across repeated invocations on the same stream, the Rust module preserves invocation order and does not introduce extra or missing records relative to the C behavior.

Traceability:
- `src/posix.c`
- `posix_output_handler`

### SC-5: No unsupported feature expansion
The Rust rewrite limits behavior to the command handling and POSIX-style formatting evidenced by `src/posix.c`, without adding new output modes or new externally visible capabilities.

Traceability:
- `src/posix.c`

## Acceptance Notes

- Conformance should be verified primarily through output-comparison tests against the C module using identical symbol inputs and command sequences.
- Where the exact formatting is concerned, the Rust port should be judged by byte-for-byte output equivalence for supported scenarios.
- Any command-specific behavior not evidenced in `src/posix.c` is outside this specification.