# spec.md

## Title
Rust Functional Specification for `module_src_output_symbol_09`

## Metadata
- Project: `cflow-new`
- Module: `module_src_output_symbol_09`
- Category: `module_cluster`
- Target branch: `072-module_src_output_symbol_09-rust-port`
- Generation date: `2026-06-11`

## Overview
This module is responsible for rendering symbol information into output formats used by the project’s call-flow reporting paths. The analyzed code shows three closely related output paths:

- a GNU-style symbol printer in `src/gnu.c`
- a POSIX-style symbol printer in `src/posix.c`
- a general output-layer symbol printer and output-driver coordination logic in `src/output.c`

The Rust rewrite must preserve the module’s functional role as an output-formatting layer for symbols, including:

- accepting symbol records prepared by upstream analysis stages
- formatting those records according to the selected output style
- writing formatted symbol entries to the configured output sink
- supporting tree/direct-call style output context where level, last-child state, and direction affect formatting

This specification covers only behavior evidenced by the analyzed functions and data structures.

## Scope
In scope for this module:

- Formatting and printing individual symbol entries
- Supporting multiple output styles through output-driver selection/dispatch
- Using output context such as line number, traversal level, directness, and last-item status during symbol rendering
- Managing output-order helper collections used by the output layer

Out of scope:

- Symbol discovery, parsing, or semantic analysis
- CLI option parsing
- File-system policy beyond writing to an already selected output sink
- New output formats or public APIs not evidenced by the source module set

## Feature Specification

### Feature 1: Format-specific symbol rendering
The module shall render a symbol entry in the output syntax required by the active output style.

Evidence:
- `src/gnu.c`: `print_symbol(FILE *outfile, int line, struct output_symbol *s)`
- `src/posix.c`: `print_symbol(FILE *outfile, int line, struct output_symbol *s)`

Required Rust behavior:
- Support a GNU-style symbol rendering path
- Support a POSIX-style symbol rendering path
- Accept a destination writer/output sink, a line/context value, and an output symbol record
- Produce the textual representation for exactly one symbol entry per invocation

### Feature 2: General output-layer symbol printing
The module shall provide an output-layer symbol-printing path that formats a symbol according to traversal context.

Evidence:
- `src/output.c`: `print_symbol (int direct, int level, int last, Symbol *sym)`

Required Rust behavior:
- Accept symbol-printing context including:
  - whether the relation is direct
  - nesting/indentation level
  - whether the symbol is the last item in the current group
- Use that context to produce output appropriate for hierarchical or graph-like symbol presentation
- Preserve distinction between symbol content and output context

### Feature 3: Output driver coordination
The module shall support multiple output modes through output-driver data structures that encapsulate output behavior.

Evidence:
- `src/output.c`: `struct output_driver` references at multiple locations

Required Rust behavior:
- Represent output-mode selection as a driver/strategy concept
- Route symbol printing through the active driver behavior
- Keep GNU-style and POSIX-style rendering behavior distinguishable within the module

### Feature 4: Output symbol abstraction
The module shall use an intermediate output-symbol representation distinct from the project’s broader symbol type where required by format-specific printers.

Evidence:
- `struct output_symbol` used in `src/gnu.c`, `src/posix.c`, and `src/output.c`
- `Symbol *sym` used separately in `src/output.c`

Required Rust behavior:
- Preserve a dedicated output-symbol data model for format-specific rendering
- Preserve the distinction between:
  - general project symbol input
  - output-ready symbol representation

### Feature 5: Output-order helper list management
The module shall maintain internal linked-list style collections used by the output layer to organize symbol-related output work.

Evidence:
- `src/output.c`: `struct linked_list` and repeated `struct linked_list_entry` usage

Required Rust behavior:
- Preserve the module’s ability to maintain ordered internal collections needed by output processing
- Preserve behaviorally equivalent ordering semantics relied on by symbol output generation
- The Rust rewrite may change storage implementation, but must not change externally visible output behavior

## User Scenarios & Testing

### Scenario 1: Emit one symbol in GNU output mode
A caller has selected GNU-style output and provides an output-ready symbol plus line/context information. The module writes one GNU-formatted symbol entry to the output sink.

Test expectations:
- Given a valid output symbol and writer, one symbol entry is emitted
- GNU-mode formatting differs from POSIX-mode formatting when source behavior requires it
- The function reports success/failure consistently with write outcome

### Scenario 2: Emit one symbol in POSIX output mode
A caller has selected POSIX-style output and provides an output-ready symbol plus line/context information. The module writes one POSIX-formatted symbol entry to the output sink.

Test expectations:
- Given a valid output symbol and writer, one symbol entry is emitted
- Output matches the POSIX-style path rather than GNU-style path
- The line/context input is incorporated in the same cases as in the C module

### Scenario 3: Print a symbol within hierarchical output
A caller prints a symbol from the general output layer with traversal metadata indicating nesting level, whether the edge is direct, and whether the item is the last child.

Test expectations:
- Changing `level` changes the hierarchical presentation as required by the original behavior
- Changing `last` changes end-of-group or branch formatting where applicable
- Changing `direct` changes the rendered relation marker or equivalent formatting where applicable

### Scenario 4: Switch output driver
A caller selects a different output driver and then prints symbols through the output layer.

Test expectations:
- The active driver determines which formatting path is used
- Switching drivers changes emitted text without requiring changes to symbol content
- No cross-contamination occurs between formatting modes

### Scenario 5: Preserve output ordering from internal collections
The output layer accumulates symbol-related items in internal ordered collections before emission.

Test expectations:
- Items are emitted in the same observable order as the C module
- Rewriting internal list structures in Rust does not alter output sequence
- End-of-list behavior remains correct for formatting that depends on “last item” state

## Requirements

### Functional Requirements

#### FR-1: Single-symbol formatted output
The module shall format and emit a single symbol entry to an output sink in each format-specific print operation.

Traceability:
- `src/gnu.c:41-59`
- `src/posix.c:32-47`

#### FR-2: GNU output support
The module shall implement the GNU-style symbol output behavior represented by the GNU print path.

Traceability:
- `src/gnu.c:41-59`
- `struct output_symbol` usage in `src/gnu.c`

#### FR-3: POSIX output support
The module shall implement the POSIX-style symbol output behavior represented by the POSIX print path.

Traceability:
- `src/posix.c:32-47`
- `struct output_symbol` usage in `src/posix.c`

#### FR-4: Context-sensitive general symbol printing
The module shall support general symbol printing that depends on direction, nesting level, and last-item status.

Traceability:
- `src/output.c:156-170`

#### FR-5: Driver-based output mode selection
The module shall preserve output behavior selection via output-driver abstractions.

Traceability:
- `struct output_driver` in `src/output.c`

#### FR-6: Separate handling of general symbols and output symbols
The module shall preserve the distinction between the general `Symbol` input used by the output layer and the `output_symbol` representation used by format-specific printers.

Traceability:
- `src/output.c:156-170` (`Symbol *sym`)
- `src/output.c:159` (`struct output_symbol`)
- `src/gnu.c:42`
- `src/posix.c:33`

#### FR-7: Ordered internal collection support for output processing
The module shall preserve the internal ordered-collection behavior used by the output layer to support symbol emission.

Traceability:
- `src/output.c:209`
- `src/output.c:212`
- `src/output.c:283`
- `src/output.c:289`
- `src/output.c:302`
- `src/output.c:327`
- `src/output.c:362`

#### FR-8: Write-result propagation
The module shall report success or failure of symbol-print operations in a way that preserves the original module’s observable outcome semantics.

Traceability:
- `print_symbol` functions in:
  - `src/gnu.c:41-59`
  - `src/output.c:156-170`
  - `src/posix.c:32-47`

### Key Entities

#### Entity: Output Driver
A module-level abstraction that defines which output behavior is active.

Role:
- Selects the formatting strategy used by the output layer
- Separates GNU-style and POSIX-style behavior paths

Traceability:
- `struct output_driver` in `src/output.c`

#### Entity: Output Symbol
An output-oriented symbol record used by format-specific printers.

Role:
- Carries the symbol information required for GNU/POSIX rendering
- Serves as the format-facing representation distinct from the broader project symbol type

Traceability:
- `struct output_symbol` in `src/gnu.c`
- `struct output_symbol` in `src/posix.c`
- `struct output_symbol` reference in `src/output.c`

#### Entity: General Symbol
A project-level symbol object received by the general output-layer printer.

Role:
- Input to context-sensitive output logic
- Source object from which output-ready symbol data may be derived or selected

Traceability:
- `Symbol *sym` in `src/output.c:156-170`

#### Entity: Linked List / Linked List Entry
Internal ordered collection entities used by the output layer.

Role:
- Maintain output-related item ordering
- Support traversal and formatting decisions that depend on sequence position

Traceability:
- `struct linked_list` in `src/output.c:209`
- `struct linked_list_entry` in `src/output.c:212, 283, 289, 302, 327, 362`

#### Relationship Summary
- The general output layer receives a `Symbol` plus traversal context.
- The active `Output Driver` determines which rendering behavior applies.
- Format-specific rendering consumes an `Output Symbol`.
- Internal ordered collections support stable output sequencing and last-item-sensitive formatting.

## Success Criteria

1. The Rust module can emit one symbol entry in GNU mode and one in POSIX mode, with distinct formatting behavior matching the corresponding C paths.
   - Traceability: `src/gnu.c:41-59`, `src/posix.c:32-47`

2. The Rust module can print symbols from the general output layer while honoring `direct`, `level`, and `last` context in the emitted result.
   - Traceability: `src/output.c:156-170`

3. Switching the active output driver changes the formatting path used for symbol emission without requiring changes to the symbol content supplied by callers.
   - Traceability: `struct output_driver` in `src/output.c`

4. The Rust rewrite preserves the distinction between general symbol input and output-symbol formatting data.
   - Traceability: `Symbol *sym` in `src/output.c`, `struct output_symbol` references across all three files

5. Output ordering observable at the module boundary remains stable relative to the C implementation for sequences managed through the module’s internal collections.
   - Traceability: `struct linked_list`, `struct linked_list_entry` usage in `src/output.c`

6. Symbol-print operations return success/failure results consistent with whether output emission succeeds.
   - Traceability: all analyzed `print_symbol` functions

7. Regression tests covering GNU-style output, POSIX-style output, hierarchical output context, driver switching, and output ordering all pass on the Rust branch.
   - Traceability: all analyzed files and named functions

## Acceptance Notes
- Conformance is defined by preserving observable formatting behavior and output selection semantics from the analyzed C module.
- Internal Rust data structures may differ from the C implementation, but only if they preserve the same externally visible output content, ordering, and success/failure behavior.