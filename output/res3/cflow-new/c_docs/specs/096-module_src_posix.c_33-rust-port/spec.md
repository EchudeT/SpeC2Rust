# spec.md

## Title

Functional Specification for Rust Port of `module_src_posix.c_33`

## Status

Draft

## Summary

This module provides a POSIX-style output handler for cflow output generation. It formats symbol entries into textual lines and reacts to output commands routed through a handler interface. The Rust rewrite must preserve the observed command-driven output behavior and emitted text structure evidenced by `src/posix.c`, including symbol type rendering and line-oriented output for symbol records.

## Scope

In scope:

- POSIX-style formatting of symbol information to an output stream.
- Command-based dispatch through the module output handler.
- Use of the module-local output record structure to receive per-symbol output data.

Out of scope:

- Symbol discovery, parsing, or graph construction.
- Ownership or lifecycle management of symbols beyond data received by this module.
- Any output modes not evidenced by this module.

## Source Basis

This specification is derived from:

- `src/posix.c`
- `print_symbol_type`
- `posix_output_handler`
- `struct output_symbol`

## Feature Specification

### Overview

The module acts as a formatting endpoint for cflow output generation in a POSIX-oriented textual form. It receives commands and associated data through a handler function. For symbol output commands, it emits a line describing a symbol using a fixed ordering of fields and a symbol type marker derived from the symbol.

### Required Rust Behavior

The Rust implementation must:

1. Accept handler-style commands equivalent in role to the C module's output command input.
2. For symbol-output commands, read the provided symbol-output record and write a POSIX-style textual representation to the target output sink.
3. Render the symbol type marker using the same symbol-driven distinction performed by the C helper that prints symbol type.
4. Preserve command-sensitive behavior of the handler, including returning a status result for processed commands.
5. Restrict behavior to what is evidenced in the module: command dispatch and symbol line formatting.

### Output Semantics

For symbol record output, the module must produce line-oriented text that includes:

- the symbol identity information supplied through the symbol output record,
- a symbol type representation derived from the symbol,
- formatting consistent with the POSIX output mode implemented by the module.

The Rust port must preserve externally observable formatting behavior of this module for supported commands.

## User Scenarios & Testing

### Scenario 1: Emit one symbol entry in POSIX format

A caller invokes the output handler with the command corresponding to symbol emission and provides:

- an output sink,
- a line value,
- a populated output-symbol record containing a symbol and related display data.

Expected result:

- one POSIX-style textual record is written to the output sink,
- the symbol type portion reflects the symbol classification used by the original module,
- the handler reports successful processing.

### Scenario 2: Emit multiple symbol entries through repeated handler calls

A caller repeatedly sends symbol-output commands with different symbol records.

Expected result:

- each call independently emits one correctly formatted textual record,
- output order matches call order,
- formatting remains consistent across entries.

### Scenario 3: Process non-symbol handler commands supported by the module

A caller sends a command handled by the module that does not require symbol line emission.

Expected result:

- the handler performs the command-specific behavior evidenced by the module,
- output changes only when the original module would produce output,
- the handler returns a status consistent with the C behavior.

### Scenario 4: Validate symbol type rendering

A caller provides symbol records whose symbols differ in type classification.

Expected result:

- the emitted symbol type marker changes accordingly,
- the distinction matches the behavior of the original `print_symbol_type` helper.

### Testing Guidance

The Rust port should be tested with golden-output comparisons against the C module behavior for:

- a representative symbol-output command,
- at least two distinct symbol classifications,
- repeated sequential symbol outputs,
- each additional command path evidenced in the handler.

## Requirements

### Functional Requirements

#### FR-1: Command-based output handling

The module shall expose behavior equivalent to the C output handler that accepts:

- an output command,
- an output target,
- a line value,
- command-associated data,
- handler-specific data.

Traceability: `posix_output_handler` in `src/posix.c`.

#### FR-2: Symbol record formatting

When invoked with the command path for symbol output, the module shall format and emit a textual record for the provided symbol output data.

Traceability: `posix_output_handler`, `struct output_symbol` in `src/posix.c`.

#### FR-3: Symbol type rendering

The module shall render a symbol type indicator based on the provided symbol, following the same classification behavior as the C helper.

Traceability: `print_symbol_type` in `src/posix.c`.

#### FR-4: Line-oriented emission

For symbol output operations, the module shall write output as discrete textual records suitable for sequential listing.

Traceability: `posix_output_handler` in `src/posix.c`.

#### FR-5: Status result reporting

The module shall report a success/failure status for each handled command in a way that preserves the observable outcome of the C handler's integer return value.

Traceability: `posix_output_handler` in `src/posix.c`.

### Key Entities

#### Output handler command

A command value directs which behavior the handler performs. It determines whether the module emits a symbol record or performs another supported command action.

Traceability: `posix_output_handler`.

#### Output symbol record

The module-local output record carries the data needed to print one symbol entry. It relates symbol data to the handler's formatting operation.

Traceability: `struct output_symbol`.

#### Symbol

A symbol is the primary subject being formatted. Its classification is used to determine the printed symbol type marker.

Traceability: `print_symbol_type`, `struct output_symbol`.

#### Output target

A writable output destination receives the formatted text records generated by the handler.

Traceability: `print_symbol_type`, `posix_output_handler`.

## Success Criteria

1. The Rust module can process the same command categories evidenced in `posix_output_handler` and produce matching observable behavior for each supported path.
2. For symbol-output commands, the Rust module emits text matching the C module's POSIX-style formatting for equivalent input records.
3. Symbol type markers emitted by the Rust module match those produced by the C module for the same symbol classifications.
4. Repeated symbol-output invocations preserve record boundaries and call order in the produced output.
5. The Rust module returns status results that map consistently to the original handler's success/failure outcomes for supported commands.

## Acceptance Evidence

Acceptance should be based on:

- comparison tests between C-module output and Rust-module output for representative symbol records,
- command-path tests covering each behavior evidenced in `posix_output_handler`,
- verification that symbol type rendering matches the C helper behavior.