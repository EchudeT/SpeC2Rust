# spec.md

## Title

Functional Specification for `module_src_dot.c_24` Rust Port

## Document Metadata

- Project: `cflow-new`
- Module: `module_src_dot.c_24`
- Category: `module_cluster`
- Source file: `src/dot.c`
- Target branch: `087-module_src_dot.c_24-rust-port`
- Generation date: `2026-06-17`

## Overview

This module produces Graphviz DOT-formatted call graph output through an output-handler interface. Its responsibility is limited to formatting graph output, declaring graph nodes from symbol data, and reacting to output commands that drive the beginning of graph emission, per-line graph content emission, and related handler-directed output steps.

The Rust version must preserve the observed functional boundary:

- emit DOT graph prologue content,
- emit node declarations for symbols,
- emit graph output in response to output-handler commands,
- write formatted results to the provided output stream/context.

This specification covers only behavior evidenced by `src/dot.c` and its identified functions and internal data structures.

## Feature Specification

### Summary

The module acts as a DOT output backend for the larger cflow output pipeline. It receives output commands and writes Graphviz DOT syntax to an output target. It formats graph start content, represents symbols as DOT nodes, and coordinates output state needed for graph emission.

### In-Scope Functionality

The Rust port must implement the following functional behavior evidenced by the source module:

1. **DOT graph initialization**
   - Provide behavior equivalent to beginning a DOT graph output session.
   - Emit the DOT opening/prologue content to the output target when the corresponding handler flow requires it.

2. **Symbol-to-node declaration**
   - Accept symbol information and emit a DOT node declaration representing that symbol.
   - Use symbol-derived identity and display information consistently within the DOT output.

3. **Command-driven output handling**
   - Expose module behavior through the output-handler command model used by the surrounding system.
   - Interpret handler commands and perform the appropriate DOT output action for each supported command path present in the module.
   - Return an integer status result consistent with handler success/failure signaling.

4. **Per-output-session state usage**
   - Use internal state structures to track output symbols and list relationships as needed by the module’s command processing.
   - Ensure generated DOT content is coherent across multiple handler invocations within one graph output session.

### Out of Scope

The Rust port must not introduce unevidenced capabilities such as:

- alternate graph formats,
- public APIs beyond the handler-facing behavior required by this module,
- persistence, serialization, or import of graph state,
- concurrency guarantees,
- recovery logic beyond status signaling already implied by the handler interface.

## User Scenarios & Testing

### Scenario 1: Start DOT output for a call graph

**Context:** The surrounding cflow output system selects this module as the active output backend.

**Flow:**
1. The system invokes the module’s output handler with the command that begins DOT output.
2. The module writes the DOT graph opening content to the supplied output target.

**Expected result:**
- Output begins with valid DOT graph prologue text.
- The handler reports success through its integer return value.

**Testing focus:**
- Verify that the first command in a session causes graph-begin output.
- Verify that output is written to the provided output sink, not to global or implicit destinations.

### Scenario 2: Declare a symbol as a DOT node

**Context:** A symbol is available for graph output.

**Flow:**
1. The module is directed, through its handler flow, to emit content involving a symbol.
2. The node declaration behavior formats and writes a DOT node entry for that symbol.

**Expected result:**
- Exactly one DOT node declaration is emitted for the symbol occurrence required by the active command path.
- The output uses symbol-derived content suitable for DOT syntax.

**Testing focus:**
- Verify that a representative symbol produces a node declaration line or block.
- Verify that the declaration is syntactically suitable for inclusion in a DOT graph.

### Scenario 3: Process multiple handler commands in one graph session

**Context:** The output backend is used across several command callbacks while building one graph.

**Flow:**
1. A begin-related command is received.
2. One or more symbol- or line-related commands are received.
3. The handler continues to write graph content as directed.

**Expected result:**
- Output remains coherent across calls.
- Internal state needed for repeated symbol/output handling is preserved for the duration implied by the command sequence.

**Testing focus:**
- Verify correct ordered output across successive handler invocations.
- Verify that no command emits graph content inconsistent with earlier session output.

### Scenario 4: Handle line-associated output command data

**Context:** The handler is invoked with a line number and opaque data payload.

**Flow:**
1. The system calls the handler with a command, output target, line value, data pointer/context, and handler-local data.
2. The module uses only the command-relevant information to decide what DOT content to emit.

**Expected result:**
- The handler accepts the provided invocation shape and responds deterministically for supported command cases.
- The return status reflects whether handling completed normally.

**Testing focus:**
- Verify supported command cases return the expected status code.
- Verify unsupported or no-op command paths, if present in the source behavior, do not corrupt output.

## Requirements

### Functional Requirements

#### FR-1: DOT graph prologue emission
The module shall provide behavior to begin DOT graph output by writing the graph-opening/prologue content to the supplied output target.

**Traceability:** `dot_begin` in `src/dot.c`

#### FR-2: DOT node declaration from symbol data
The module shall format and emit a DOT node declaration for a provided symbol.

**Traceability:** `declare_node` in `src/dot.c`

#### FR-3: Output-handler command dispatch
The module shall implement command-driven output behavior through a handler entry point that receives:
- an output command,
- an output target,
- a line value,
- command-associated data,
- handler-associated data.

**Traceability:** `dot_output_handler` in `src/dot.c`

#### FR-4: Handler-directed output writing
The module shall write all DOT output generated by this module through the output target supplied to the handler or helper behavior for that invocation flow.

**Traceability:** `dot_begin`, `declare_node`, `dot_output_handler` in `src/dot.c`

#### FR-5: Session-coherent graph output
The module shall maintain whatever internal per-session output state is required so that repeated handler invocations can contribute to one coherent DOT graph output.

**Traceability:** internal `output_symbol`, `linked_list`, and `linked_list_entry` structures in `src/dot.c`, plus `dot_output_handler`

#### FR-6: Integer status signaling
The handler entry point shall return an integer status result indicating completion outcome consistent with the module’s command-processing contract.

**Traceability:** `dot_output_handler` in `src/dot.c`

### Key Entities

#### Symbol
An external symbol entity supplied to this module for graph output. It is the source object from which DOT node declarations are derived.

**Traceability:** parameter to `declare_node`

#### Output Symbol
An internal output-oriented representation used by the module while preparing or tracking DOT emission related to symbols.

**Traceability:** anonymous `struct output_symbol` in `src/dot.c`

#### Linked List
An internal collection structure used to maintain ordered or grouped state needed by the module during output handling.

**Traceability:** anonymous `struct linked_list` in `src/dot.c`

#### Linked List Entry
An internal element structure associated with the linked-list state maintained during output handling.

**Traceability:** anonymous `struct linked_list_entry` in `src/dot.c`

#### Output Command
An external command discriminator that tells the handler what stage or type of output action to perform.

**Traceability:** parameter type `cflow_output_command` to `dot_output_handler`

#### Output Session Context
The combination of:
- output target,
- handler data,
- command-associated data,
- any internal state retained across related handler calls.

This context defines one logical DOT emission flow.

**Traceability:** `dot_output_handler` parameters and internal state structures

## Success Criteria

### SC-1: Valid graph start emission
When invoked through the begin-related handler flow, the Rust module emits DOT graph-opening text to the provided output target.

**Traceability:** FR-1, `dot_begin`, `dot_output_handler`

### SC-2: Symbol node output present
Given a representative symbol input on a command path that requires node emission, the Rust module emits a DOT node declaration derived from that symbol.

**Traceability:** FR-2, `declare_node`

### SC-3: Command-driven behavior preserved
For each handler command case implemented by the source module, the Rust port performs the corresponding DOT output action and returns an integer status.

**Traceability:** FR-3, FR-6, `dot_output_handler`

### SC-4: Output sink fidelity
All module-generated DOT text appears in the caller-supplied output target for the active invocation flow.

**Traceability:** FR-4, all identified functions

### SC-5: Multi-call coherence
Across a sequence of related handler invocations forming one graph output session, the Rust module preserves sufficient internal state to keep emitted DOT output coherent.

**Traceability:** FR-5, internal state structures, `dot_output_handler`

### SC-6: No unevidenced feature expansion
The Rust port remains functionally limited to DOT output handling behavior evidenced by this module and does not require new public capabilities beyond that boundary.

**Traceability:** module scope defined by `src/dot.c`

## Acceptance Notes

- Conformance should be judged by observable output behavior and handler return behavior.
- Internal Rust design may differ from the C source, but it must preserve the functional boundaries and externally visible behavior described above.
- Only source-evidenced command handling and output behavior are required by this specification.