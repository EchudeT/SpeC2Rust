# spec.md

## Overview

This module is responsible for producing call-graph output in Graphviz DOT format for the `cflow-new` project. Its behavior is defined by `src/dot.c` and centers on handling output commands, emitting DOT graph framing, declaring graph nodes for symbols, and writing graph edges between symbols as traversal events are received.

The Rust rewrite must preserve the module’s observed functional role as an output handler for DOT generation:
- start a DOT graph output stream,
- emit symbol node declarations in DOT syntax,
- react to output commands that describe graph-generation events,
- write valid DOT-formatted output to the provided destination.

This specification covers only functionality evidenced by:
- `dot_begin`
- `declare_node`
- `dot_output_handler`
- the module-local output/linking records used to track symbol emission and relationships.

## Scope

In scope:
- DOT graph output generation for call-flow/call-graph data.
- Command-driven formatting through the module output handler.
- Per-symbol node declaration behavior.
- Emission of relationships between symbols as DOT edges.
- Use of module-local tracking structures needed to support correct DOT output behavior.

Out of scope:
- Symbol discovery or parsing of source code.
- Definition of the upstream command protocol beyond what this module consumes.
- Non-DOT output formats.
- Capabilities not evidenced in this module, including new public APIs or extended graph features.

## Feature Specification

### Feature: DOT graph output handler

The module provides an output handler that receives graph-generation commands and writes a DOT document to an output stream.

The Rust version must implement the same functional behavior:
1. Initialize DOT output with the required opening graph syntax.
2. Represent symbols as DOT nodes.
3. Represent caller/callee or parent/child relationships supplied through handler events as DOT edges.
4. Finish output as a syntactically complete DOT graph when the command sequence is complete.
5. Preserve command-driven behavior: output must be determined by the handler inputs, not by independent graph discovery.

### Feature: Node declaration for symbols

The module declares graph nodes for symbols before or when they are referenced in the graph output.

The Rust version must:
- accept symbol information passed to the module,
- emit a DOT node declaration corresponding to that symbol,
- ensure node declarations are consistent within a single graph output session.

### Feature: Output-session tracking

The module defines local tracking structures for output symbols and linked-list state. These structures evidence that the module keeps internal state to manage emitted symbols and/or pending relationships during DOT generation.

The Rust version must preserve equivalent behavior:
- maintain enough per-session state to generate correct DOT output,
- associate symbols with their emitted graph representation,
- support repeated handler invocations within one output session without losing graph consistency.

## User Scenarios & Testing

### Scenario 1: Start a new DOT graph output

A caller configures this module as the active output handler and begins a graph output session. The handler receives the start-related command(s), and the module writes the opening DOT graph syntax to the output stream.

Expected result:
- Output begins as a DOT graph document.
- The opening syntax is emitted exactly once per graph session.

Test coverage:
- Invoke the handler with the command sequence that begins output.
- Assert the output starts with valid DOT graph opening syntax.

### Scenario 2: Emit a node for a symbol

During graph generation, the module is asked to represent a symbol in the DOT output.

Expected result:
- A DOT node declaration is written for that symbol.
- The declaration is usable by subsequent edge output.

Test coverage:
- Provide a symbol via the relevant handler/data path.
- Assert that output contains one corresponding DOT node declaration in valid DOT syntax.

### Scenario 3: Emit a relationship between symbols

As traversal proceeds, the handler receives information indicating a relationship between two symbols.

Expected result:
- The module writes a DOT edge connecting the related symbols.
- The edge refers to graph nodes consistently with the node declarations used by the same session.

Test coverage:
- Provide two symbols and the handler event(s) that establish a relationship.
- Assert the output contains a DOT edge linking the expected node identifiers.

### Scenario 4: Process multiple handler events in one session

A single graph output session may involve multiple symbols and relationships delivered over several handler invocations.

Expected result:
- Output remains a single coherent DOT graph.
- Nodes and edges from later events are appended consistently.
- Session state remains valid across calls.

Test coverage:
- Send a realistic sequence of begin, symbol/relationship, and end commands.
- Assert the resulting document is one valid DOT graph containing all expected nodes and edges.

### Scenario 5: Complete the graph output

At the end of processing, the module receives the command that closes output.

Expected result:
- The DOT graph is properly terminated.
- No incomplete framing remains in the output.

Test coverage:
- Run a full session from begin to end.
- Assert the final output includes the closing graph syntax and is parseable as DOT framing.

## Requirements

### Functional Requirements

#### FR-1: DOT graph framing
The module shall emit the required opening DOT graph syntax when graph output begins and the required closing syntax when graph output ends.

Traceability:
- `dot_begin`
- `dot_output_handler`

#### FR-2: Command-driven output handling
The module shall accept output commands through the output-handler interface and perform the corresponding DOT output action for each supported command.

Traceability:
- `dot_output_handler`

#### FR-3: Symbol node declaration
The module shall convert a provided symbol into a DOT node declaration and write that declaration to the output stream.

Traceability:
- `declare_node`
- `dot_output_handler`

#### FR-4: Relationship edge emission
The module shall emit DOT edges representing relationships communicated through handler events during graph generation.

Traceability:
- `dot_output_handler`

#### FR-5: Session-consistent symbol handling
Within a single output session, the module shall maintain sufficient internal state to keep symbol references consistent across node and edge output.

Traceability:
- `dot_output_handler`
- `struct output_symbol`
- `struct linked_list`
- `struct linked_list_entry`

#### FR-6: Stream-directed output
The module shall write all generated DOT content to the output stream supplied by the caller.

Traceability:
- `dot_begin`
- `declare_node`
- `dot_output_handler`

### Key Entities

#### Output handler session
The central interaction unit is one invocation sequence of the DOT output handler against a caller-provided output stream. A session receives commands that collectively define one DOT graph document.

Traceability:
- `dot_output_handler`

#### Symbol
A symbol is the semantic node of the graph. The module consumes symbol data and emits a DOT node representing it; relationships between symbols become graph edges.

Traceability:
- `declare_node`
- `dot_output_handler`

#### Output symbol record
The module-local `output_symbol` structure represents per-symbol output tracking used during a DOT output session.

Traceability:
- `struct output_symbol`

#### Linked-list state
The module-local `linked_list` and `linked_list_entry` structures represent internal relationship or membership tracking needed to support graph output across multiple events.

Traceability:
- `struct linked_list`
- `struct linked_list_entry`

#### Relationships
A relationship connects two symbols in the generated graph and is emitted as a DOT edge when conveyed through the handler protocol.

Traceability:
- `dot_output_handler`

## Success Criteria

1. A complete begin-to-end output session produces one syntactically framed DOT graph document with both opening and closing graph syntax present.
   - Traceability: `dot_begin`, `dot_output_handler`

2. For each symbol supplied for output, the module emits a valid DOT node declaration corresponding to that symbol.
   - Traceability: `declare_node`, `dot_output_handler`

3. For each relationship event supplied to the handler, the module emits a DOT edge connecting the intended symbols.
   - Traceability: `dot_output_handler`

4. A session containing multiple symbols and multiple relationships yields one coherent DOT document in which later output remains consistent with earlier symbol handling.
   - Traceability: `dot_output_handler`, `struct output_symbol`, `struct linked_list`, `struct linked_list_entry`

5. All output produced by the module is written to the caller-supplied output destination rather than an internal or implicit destination.
   - Traceability: `dot_begin`, `declare_node`, `dot_output_handler`

6. The Rust rewrite matches the command-driven functional behavior of the C module for supported DOT output events, without requiring the caller to change the module’s role in the larger output pipeline.