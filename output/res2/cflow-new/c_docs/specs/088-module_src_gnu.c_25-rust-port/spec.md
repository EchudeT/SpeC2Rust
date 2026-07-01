# spec.md

## Document Control

- **Project**: `cflow-new`
- **Module**: `module_src_gnu.c_25`
- **Category**: `module_cluster`
- **Source files**: `src/gnu.c`
- **Rust branch**: `088-module_src_gnu.c_25-rust-port`
- **Generation date**: `2026-06-17`

## 1. Feature Specification

### 1.1 Purpose

This module provides a GNU-style output handler for the project’s output dispatch mechanism. Its functional role is to receive output commands from the wider cflow output pipeline and emit the corresponding text representation to a provided output stream.

The Rust rewrite must preserve this role: it must act as a command-driven formatter/output handler that writes GNU-format output for the same command inputs accepted by the current module.

### 1.2 In-Scope Functionality

Traceable source: `gnu_output_handler` in `src/gnu.c`.

The Rust version must implement:

- A handler entry point equivalent in behavior to `gnu_output_handler`.
- Command-based processing using the provided output command value.
- Output emission to a caller-supplied destination corresponding to the current `FILE *outfile` role.
- Use of the provided per-call context data needed to format output entries.
- Return of an integer status/result consistent with success/failure signaling expected from the current handler contract.

### 1.3 Output Behavior

Based on the available evidence, this module is responsible for formatting and emitting GNU-style output records rather than for computing graph/call relationships itself.

The Rust version must therefore:

- Consume already-prepared data passed through the handler interface.
- Produce textual output in GNU format for supported commands.
- Respect the sequencing implied by repeated handler invocations from the surrounding output framework.

### 1.4 Explicit Non-Goals

Not evidenced in this module and therefore out of scope for the Rust port specification:

- Building or analyzing call graphs.
- Defining new output commands.
- Persisting output state beyond what is required by the handler contract.
- New public APIs beyond the existing handler role.
- Concurrency guarantees, recovery features, serialization, or FFI extensions.

## 2. User Scenarios & Testing

### 2.1 Scenario: Output framework invokes GNU handler for a valid output event

A caller in the project’s output subsystem selects the GNU output mode and invokes the module’s handler with:

- a valid output command,
- a writable output destination,
- a line/depth integer,
- command-specific data,
- handler-specific context.

**Expected behavior**:
- The handler accepts the event.
- It emits the GNU-format text associated with that command to the supplied destination.
- It returns a status code indicating successful handling.

**Testing focus**:
- Verify that valid commands produce output.
- Verify that output is written to the provided sink, not to an internal or global destination.
- Verify that the returned status indicates success.

### 2.2 Scenario: Output framework delivers multiple commands in sequence

The output subsystem invokes the GNU handler repeatedly as part of generating a complete output document or stream.

**Expected behavior**:
- Each invocation handles only the current command and its supplied data.
- Output across calls is ordered by invocation order.
- The handler remains compatible with the surrounding dispatcher’s multi-call workflow.

**Testing focus**:
- Feed a representative sequence of commands.
- Confirm that the resulting output stream preserves call order.
- Confirm that each command contributes only its expected text.

### 2.3 Scenario: Handler receives command/data combinations outside normal generation flow

The dispatcher or a test harness invokes the handler with unsupported, empty, or otherwise non-productive input combinations.

**Expected behavior**:
- The handler returns an integer result according to the existing contract.
- It does not invent output for unsupported cases.
- It does not require functionality beyond the passed-in handler interface.

**Testing focus**:
- Exercise unrecognized or no-op command paths if present in legacy behavior.
- Confirm the absence of unrelated side effects.
- Confirm that the status/result remains usable by the caller.

## 3. Requirements

### 3.1 Functional Requirements

FR-1. The module shall provide a GNU output handler interface corresponding to `gnu_output_handler` in `src/gnu.c`.
**Traceability**: `gnu_output_handler`

FR-2. The handler shall accept a command-driven invocation and determine behavior from the supplied output command parameter.
**Traceability**: `gnu_output_handler`

FR-3. The handler shall write generated output to the caller-provided output destination corresponding to the `outfile` parameter.
**Traceability**: `gnu_output_handler`

FR-4. The handler shall use invocation-supplied contextual data from the `data` and `handler_data` parameters as required to produce its output behavior.
**Traceability**: `gnu_output_handler`

FR-5. The handler shall support the output sequencing model in which it may be called repeatedly by external code to build a complete GNU-format output stream.
**Traceability**: `gnu_output_handler`

FR-6. The handler shall return an integer status/result for each invocation so callers can determine handling outcome according to the existing module contract.
**Traceability**: `gnu_output_handler`

FR-7. The module shall preserve the module-specific GNU output behavior associated with this handler and shall not shift responsibility for formatting to unrelated module boundaries.
**Traceability**: `src/gnu.c`, `gnu_output_handler`

### 3.2 Key Entities

#### GNU Output Handler

- **Entity**: `gnu_output_handler`
- **Role**: Primary functional entry point of the module.
- **Relationship**:
  - Receives an output command from the broader output subsystem.
  - Consumes caller-supplied output destination and context pointers.
  - Produces GNU-format text output and a status result.

#### Output Symbol Structure

- **Entity**: `struct output_symbol` (anonymous structure identified at `src/gnu.c:42`)
- **Role**: Module-local data shape associated with output symbol handling.
- **Relationship**:
  - Supports the handler’s formatting/output responsibilities.
  - Represents symbol-related output information used within the module’s GNU output behavior.

## 4. Success Criteria

SC-1. A Rust implementation exposes a handler with behavior equivalent to the current GNU output handler role and can be invoked by the project’s output dispatch flow.
**Traceability**: `gnu_output_handler`

SC-2. For supported command inputs used by the current module, the Rust version emits GNU-format text to the supplied output sink rather than to any implicit destination.
**Traceability**: `gnu_output_handler`

SC-3. Under repeated invocation, the Rust version preserves invocation order in the produced output stream and remains usable as a dispatcher-managed handler.
**Traceability**: `gnu_output_handler`

SC-4. The Rust version returns per-invocation status/results compatible with the caller expectations of the current handler contract.
**Traceability**: `gnu_output_handler`

SC-5. The Rust port does not require capabilities not evidenced by `src/gnu.c`, and implementation scope remains limited to the GNU output handling behavior of this module.
**Traceability**: `src/gnu.c`, `gnu_output_handler`