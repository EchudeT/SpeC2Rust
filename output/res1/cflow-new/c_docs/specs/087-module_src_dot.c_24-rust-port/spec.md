# spec.md

## Title
Rust Port Functional Specification: `module_src_dot.c_24`

## Metadata
- Project: `cflow-new`
- Source module: `src/dot.c`
- Module category: `module_cluster`
- Target Rust branch: `087-module_src_dot.c_24-rust-port`
- Generation date: 2026-06-11

## Overview
This module formats call-flow output as Graphviz DOT text.

The source evidence shows three responsibilities:
1. emit the DOT graph header,
2. declare graph nodes for symbols,
3. handle output commands and write the corresponding DOT-formatted content to an output stream.

The Rust rewrite must preserve this functional role: given output commands and symbol-related data supplied by the surrounding system, it must produce DOT graph text with equivalent behavior and command handling boundaries as the C module.

## Scope
In scope for this module:
- DOT graph output generation.
- Node declaration output for symbols.
- Command-driven output handling through the module’s output handler entry point.

Out of scope for this module:
- Symbol discovery or parsing.
- Ownership of the broader call graph model outside the data passed into the handler.
- Non-DOT output formats.
- New public APIs or capabilities not evidenced by `src/dot.c`.

## Source Evidence
Primary evidence for this specification:
- `dot_begin` in `src/dot.c:18-23`
- `declare_node` in `src/dot.c:25-35`
- `dot_output_handler` in `src/dot.c:60-78`
- internal structures at `src/dot.c:39-42` for output-symbol tracking and linked-list relationships

---

## Feature Specification

### Feature: Emit DOT graph output
The module must generate textual output in Graphviz DOT form to a provided output destination.

Behavior evidenced by `dot_begin` and `dot_output_handler`:
- The module begins DOT output with the graph-opening content required for a DOT graph.
- The module emits node-related content based on symbol input.
- The module responds to output commands through a single handler that writes DOT text to the provided output stream.

### Feature: Declare nodes for symbols
The module must be able to emit a DOT node declaration for a provided symbol.

Behavior evidenced by `declare_node`:
- A symbol is converted into DOT node output.
- The declaration is written to the active output stream.
- The declaration logic is symbol-centered rather than graph-global.

### Feature: Command-oriented output processing
The module must support the output-command dispatch behavior represented by `dot_output_handler`.

Behavior evidenced by the handler signature and module role:
- Accept an output command value and associated context.
- Write the DOT output corresponding to that command.
- Use provided auxiliary data and handler-specific state as needed to complete command processing.
- Return an integer status result consistent with command handling success/failure signaling used by the module interface.

### Feature: Internal tracking of emitted or related symbols
The module contains internal symbol-output tracking structures (`output_symbol`, `linked_list`, `linked_list_entry`), indicating that command handling may maintain relationships among symbols while generating DOT output.

The Rust rewrite must preserve the functional need implied by these structures:
- maintain any internal state required to correctly produce DOT output across handler invocations,
- preserve relationships between tracked symbols where needed for correct node/edge-oriented output,
- keep this tracking internal to the module boundary.

---

## User Scenarios & Testing

### Scenario 1: Start DOT output for a graph
A caller initializes DOT-formatted output by invoking the module’s output handler in the command mode that starts output generation.

Expected result:
- The output stream receives valid DOT graph-opening text.
- The module returns a status indicating successful handling.

Traceability:
- `dot_begin`
- `dot_output_handler`

### Scenario 2: Emit a node for a symbol
A caller provides a symbol to the DOT output path so that it appears as a node in the generated graph.

Expected result:
- The output stream contains a DOT node declaration representing that symbol.
- The declaration is formatted through the module’s symbol declaration behavior.

Traceability:
- `declare_node`
- `dot_output_handler`

### Scenario 3: Process multiple output commands in sequence
A caller uses the module across multiple handler calls during one logical graph emission process.

Expected result:
- The generated DOT text remains coherent across calls.
- Internal tracking preserves whatever per-output relationships are required for correct output generation.
- Status codes reflect each command’s handling result.

Traceability:
- `dot_output_handler`
- internal tracking structures at `src/dot.c:39-42`

### Scenario 4: Use handler-provided state during output
A caller passes command data and handler-specific state into the output handler during generation.

Expected result:
- The handler uses the supplied context to determine what DOT content to emit.
- Output behavior remains driven by the command and associated data rather than hidden global expansion of functionality.

Traceability:
- `dot_output_handler`

### Testing Guidance
The Rust version must be tested with command sequences that cover:
- graph start emission,
- symbol node declaration,
- repeated handler invocations within one output session,
- status return behavior for handled commands.

Tests should compare generated text and command results against behavior derived from the C module’s observed output semantics.

---

## Requirements

### Functional Requirements

#### FR-1: DOT header emission
The module shall emit DOT graph-opening text when the corresponding output-start behavior is invoked.

Traceability:
- `dot_begin` in `src/dot.c:18-23`
- `dot_output_handler` in `src/dot.c:60-78`

#### FR-2: Symbol node declaration
The module shall emit a DOT node declaration for a provided symbol.

Traceability:
- `declare_node` in `src/dot.c:25-35`

#### FR-3: Command-dispatch output handling
The module shall expose equivalent command-driven behavior to process output commands, write DOT text to the supplied output stream, and return an integer status result.

Traceability:
- `dot_output_handler` in `src/dot.c:60-78`

#### FR-4: Output stream targeting
The module shall direct all generated DOT text to the output destination supplied to the handler or helper behavior.

Traceability:
- `dot_begin(FILE *fp)`
- `declare_node(FILE *fp, Symbol *sym)`
- `dot_output_handler(..., FILE *outfile, ...)`

#### FR-5: Command-context usage
The module shall use command-specific input data and handler state passed to the handler to determine output behavior.

Traceability:
- `dot_output_handler(cflow_output_command cmd, FILE *outfile, int line, void *data, void *handler_data)`

#### FR-6: Internal symbol relationship/state tracking
The module shall preserve the internal capability needed to track symbol-related output state across command handling where required for correct DOT generation.

Traceability:
- internal structures at `src/dot.c:39-42`
- `dot_output_handler` in `src/dot.c:60-78`

### Key Entities

#### Symbol
A symbol is the semantic input used to produce a DOT node declaration.

Relationship:
- passed into node-declaration behavior,
- may also participate in handler-driven graph output.

Traceability:
- `declare_node(FILE *fp, Symbol *sym)`

#### Output command
An output command selects the DOT-emission action the handler performs.

Relationship:
- drives handler behavior,
- determines what content is written to the output stream.

Traceability:
- `dot_output_handler(cflow_output_command cmd, ...)`

#### Output symbol tracking record
The internal `output_symbol` structure represents module-maintained state related to symbols during DOT generation.

Relationship:
- participates in internal tracking for output production,
- may connect symbol instances to list-based bookkeeping.

Traceability:
- anonymous struct at `src/dot.c:39`

#### Linked list and linked list entry
The internal `linked_list` and `linked_list_entry` structures represent relationships among tracked output items.

Relationship:
- provide internal organization of tracked symbol/output state,
- support multi-step handler processing across output events.

Traceability:
- anonymous structs at `src/dot.c:41-42`

#### Output destination
The output destination is the writable stream receiving DOT text.

Relationship:
- shared by graph-begin behavior, node declaration behavior, and the main output handler.

Traceability:
- `FILE *fp` in `dot_begin`
- `FILE *fp` in `declare_node`
- `FILE *outfile` in `dot_output_handler`

---

## Success Criteria

### SC-1: Graph start output equivalence
When the Rust module is exercised through the start-of-output behavior corresponding to `dot_begin`/`dot_output_handler`, it produces DOT graph-opening text equivalent in function to the C module’s output.

Traceability:
- `dot_begin`
- `dot_output_handler`

### SC-2: Node declaration equivalence
For a representative symbol input, the Rust module emits a DOT node declaration equivalent in role and placement to the C module’s `declare_node` behavior.

Traceability:
- `declare_node`

### SC-3: Sequential command handling
For multi-command output sessions, the Rust module maintains coherent DOT output across successive handler invocations and returns integer status results for each invocation.

Traceability:
- `dot_output_handler`
- internal tracking structures at `src/dot.c:39-42`

### SC-4: Output destination correctness
All DOT text produced by the Rust module is written only to the caller-provided output destination used for the active operation.

Traceability:
- `dot_begin`
- `declare_node`
- `dot_output_handler`

### SC-5: No unsupported capability expansion
The Rust rewrite remains limited to the evidenced DOT-output responsibilities of this module and does not require new format support, new public APIs, or unrelated processing capabilities.

Traceability:
- whole-module evidence limited to `src/dot.c`

## Acceptance Notes
- Behavioral equivalence is defined by preserved DOT-output responsibilities and command-driven handling, not by reproducing C implementation structure.
- Internal state may be represented differently in Rust, provided it preserves the evidenced output behavior and module boundaries.
- This specification intentionally excludes unevidenced features.