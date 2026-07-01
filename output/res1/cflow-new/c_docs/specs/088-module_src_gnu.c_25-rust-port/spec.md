# spec.md

## Title

Rust Port Functional Specification for `module_src_gnu.c_25`

## Document Metadata

- **Project**: `cflow-new`
- **Module**: `module_src_gnu.c_25`
- **Category**: `module_cluster`
- **Source file**: `src/gnu.c`
- **Primary function covered**: `gnu_output_handler`
- **Primary data structure covered**: `struct output_symbol`
- **Target Rust branch**: `088-module_src_gnu.c_25-rust-port`
- **Generation date**: `2026-06-11`

## Overview

This module provides a GNU-style output handler used by the larger cflow output pipeline. Its functional role is to receive output commands from the surrounding traversal/output framework and emit command-dependent text to an output stream in the GNU output format expected by the original module behavior.

The Rust rewrite must preserve the module’s externally observable behavior as an output handler: it must accept the same class of handler inputs, react to supported command values, and write the corresponding formatted output to a provided destination without introducing new features or changing the module’s role within the output system.

## Feature Specification

### Summary

The Rust version must implement the GNU-format output handling behavior currently provided by this module. The module is not a standalone formatter; it operates as a command-driven handler invoked by the broader output subsystem.

### In-Scope Functional Behavior

- Accept an output command and associated context from the caller.
- Interpret command-specific handler data relevant to GNU-format emission.
- Write GNU-style textual output to the provided output destination.
- Return an integer status result consistent with success/failure signaling expected from the handler role.
- Preserve behavior associated with the module’s symbol-oriented formatting state represented by `struct output_symbol`.

### Out-of-Scope

The Rust port must not assume or add any functionality not evidenced by the source module interface, including:

- new output formats
- new public configuration APIs
- persistence or serialization
- concurrency guarantees
- recovery workflows beyond the existing handler success/failure result
- standalone parsing or traversal logic unrelated to output handling

## User Scenarios & Testing

### Scenario 1: Handler is invoked during normal GNU-format output generation

A caller in the cflow output pipeline invokes the GNU output handler with a command representing a formatting step and supplies a writable output target plus command-specific data.

**Expected behavior**
- The handler processes the command.
- It emits the corresponding GNU-format text to the output target.
- It returns an integer status indicating successful handling when the operation completes normally.

**Test focus**
- Verify text is written for supported commands.
- Verify return value signals success for valid inputs and writable output.

### Scenario 2: Handler receives symbol-related data for output formatting

The caller provides symbol-oriented data used by the GNU output handler to render output entries.

**Expected behavior**
- The handler uses the symbol data required by the GNU-format output behavior.
- The emitted text reflects the provided symbol information in the same observable way as the C module.

**Test focus**
- Compare emitted output for representative symbol data against the C module behavior.
- Confirm symbol-associated formatting is preserved.

### Scenario 3: Handler is called repeatedly as part of a larger output sequence

The output subsystem invokes the handler multiple times with successive commands to build a complete GNU-style output stream.

**Expected behavior**
- Each call behaves correctly in sequence.
- Output remains correctly ordered according to invocation order.
- The handler does not require standalone orchestration beyond the command-driven protocol it already participates in.

**Test focus**
- Run multi-call sequences and compare aggregate output with the original module.
- Verify no missing, duplicated, or reordered text relative to command order.

### Scenario 4: Handler encounters an unsupported or unsuccessfully processed command path

The caller invokes the handler with a command or context that does not lead to successful formatting.

**Expected behavior**
- The handler returns a non-success status consistent with original behavior.
- It does not claim success when processing did not complete as required.

**Test focus**
- Exercise non-success paths evidenced by the C behavior.
- Verify status code behavior matches the original module for those paths.

## Requirements

### Functional Requirements

#### FR-1: Command-driven output handling
The module shall implement a GNU output handler that accepts an output command, an output destination, a line-related input value, command data, and handler-specific data, and shall produce behavior based on the command received.

**Traceability**: `src/gnu.c`, `gnu_output_handler`

#### FR-2: GNU-format text emission
The module shall emit GNU-style textual output to the provided output destination for the commands supported by the original handler.

**Traceability**: `src/gnu.c`, `gnu_output_handler`

#### FR-3: Symbol-oriented formatting support
The module shall preserve formatting behavior that depends on the symbol representation modeled by `struct output_symbol`.

**Traceability**: `src/gnu.c`, `struct output_symbol`, `gnu_output_handler`

#### FR-4: Output destination usage
The module shall direct emitted output to the caller-provided output stream target rather than to any implicit global destination.

**Traceability**: `src/gnu.c`, `gnu_output_handler`

#### FR-5: Status return behavior
The module shall return an integer status result representing whether handling completed successfully, matching the success/failure semantics of the original module.

**Traceability**: `src/gnu.c`, `gnu_output_handler`

#### FR-6: Sequential invocation compatibility
The module shall support repeated invocation as part of the surrounding output pipeline, with each invocation contributing output according to the command-driven protocol.

**Traceability**: `src/gnu.c`, `gnu_output_handler`

### Key Entities

#### `gnu_output_handler`
The module’s central functional entity. It serves as the integration point between this module and the broader cflow output subsystem. It receives command and context inputs, decides what GNU-format output action to perform, writes output, and reports status.

#### `struct output_symbol`
The module’s key symbol-related data entity. It represents the symbol information used by the handler’s output behavior. Its relationship to the handler is functional: the handler consumes or interprets symbol-related data in order to emit GNU-style output.

## Success Criteria

### SC-1: Observable output equivalence
For the set of supported handler commands exercised against representative inputs, the Rust module produces the same observable GNU-format output as the C module.

**Traceability**: `src/gnu.c`, `gnu_output_handler`

### SC-2: Correct destination behavior
When provided a writable output target, the Rust module writes output to that target and does not require an alternate implicit destination.

**Traceability**: `src/gnu.c`, `gnu_output_handler`

### SC-3: Status compatibility
For successful and non-successful handling paths evidenced by the original module, the Rust module returns status results compatible with the original handler behavior.

**Traceability**: `src/gnu.c`, `gnu_output_handler`

### SC-4: Symbol-formatting preservation
For inputs involving symbol-oriented output data, the Rust module preserves the same externally visible formatting behavior as the C module.

**Traceability**: `src/gnu.c`, `struct output_symbol`, `gnu_output_handler`

### SC-5: Multi-call sequence correctness
In test sequences where the handler is invoked multiple times to build a complete output stream, the Rust module preserves output ordering and cumulative output behavior relative to the original C module.

**Traceability**: `src/gnu.c`, `gnu_output_handler`