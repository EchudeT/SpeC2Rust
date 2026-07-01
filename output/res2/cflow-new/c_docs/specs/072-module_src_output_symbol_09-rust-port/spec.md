# spec.md

## Overview

This module cluster is responsible for formatting and emitting symbol output for `cflow-new` in multiple output styles. The analyzed files show three coordinated responsibilities:

- style-specific symbol line formatting for GNU-style output (`src/gnu.c`)
- style-specific symbol line formatting for POSIX-style output (`src/posix.c`)
- generic output orchestration, including driver-based dispatch and direct/tree-oriented symbol printing (`src/output.c`)

The Rust rewrite for branch `072-module_src_output_symbol_09-rust-port` must preserve the functional behavior of symbol emission and output-style selection evidenced by these files and functions.

## Scope

In scope for this module:

- representation of output-capable symbol records used during emission
- output-driver selection/dispatch behavior needed by the module
- printing a symbol in generic output flow
- printing a symbol in GNU-style output flow
- printing a symbol in POSIX-style output flow
- maintaining ordering/traversal support used by the output layer through linked-list entities present in `src/output.c`

Out of scope:

- parsing source files
- symbol discovery algorithms
- command-line parsing
- capabilities not evidenced by the analyzed files

## Feature Specification

### Feature: Emit symbols through output-style-specific formatting

The module must emit symbol information using a selected output style.

Evidence:
- `print_symbol(FILE *outfile, int line, struct output_symbol *s)` in `src/gnu.c`
- `print_symbol(FILE *outfile, int line, struct output_symbol *s)` in `src/posix.c`
- `struct output_driver` references in `src/output.c`

Required behavior:

- The module must support at least the output styles evidenced by the analyzed files:
  - GNU-style formatting
  - POSIX-style formatting
- Each style-specific formatter must accept:
  - a target output stream/sink
  - a line-related value used during emission
  - an output symbol record
- Style-specific formatting must produce symbol output according to the selected driver/style rather than using one unified formatting path for all modes.

### Feature: Emit symbols through generic output control flow

The module must provide generic symbol-printing behavior used by the broader output subsystem.

Evidence:
- `print_symbol (int direct, int level, int last, Symbol *sym)` in `src/output.c`

Required behavior:

- The generic output layer must accept a symbol and contextual flags/values controlling how it is emitted.
- The generic output layer must support at least:
  - a direct/non-direct emission distinction
  - a nesting/depth level value
  - an end-of-group or last-element indicator
- The generic output layer must coordinate with the selected output style rather than replacing it.

### Feature: Maintain symbol ordering/traversal support for output emission

The module must preserve the output-layer relationships implied by linked-list entities in `src/output.c`.

Evidence:
- `struct linked_list` and multiple `struct linked_list_entry` occurrences in `src/output.c`

Required behavior:

- The Rust rewrite must preserve the ability of the output subsystem to maintain ordered collections of entries used during output.
- Traversal order used for emission must remain stable relative to the source module’s semantics.
- Generic symbol printing must be able to operate with the collection/traversal model used by the output layer.

## User Scenarios & Testing

### Scenario 1: Emit a symbol in GNU-style output

A caller has selected GNU-style output and asks the module to emit one symbol record to an output sink with an associated line value.

Expected result:

- The GNU-specific formatter is used.
- Output is written to the provided sink.
- The symbol is formatted according to GNU-style behavior, not POSIX-style behavior.

Test focus:
- verify driver/style dispatch reaches the GNU formatter
- verify a valid symbol record produces output
- verify the provided line argument affects emission as required by the original behavior

Traceability:
- `src/gnu.c` `print_symbol`
- `src/output.c` `struct output_driver`

### Scenario 2: Emit a symbol in POSIX-style output

A caller has selected POSIX-style output and asks the module to emit one symbol record to an output sink with an associated line value.

Expected result:

- The POSIX-specific formatter is used.
- Output is written to the provided sink.
- The symbol is formatted according to POSIX-style behavior, not GNU-style behavior.

Test focus:
- verify driver/style dispatch reaches the POSIX formatter
- verify a valid symbol record produces output
- verify the provided line argument is accepted and used consistently with the original behavior

Traceability:
- `src/posix.c` `print_symbol`
- `src/output.c` `struct output_driver`

### Scenario 3: Emit a symbol from the generic output path

A caller invokes the generic output-layer symbol printer with a symbol plus contextual values indicating direct output, nesting level, and whether the symbol is the last item in a sequence.

Expected result:

- The symbol is emitted through the generic output path.
- Context values influence formatting/placement as in the source behavior.
- The generic layer interoperates with the active output style.

Test focus:
- verify direct vs non-direct calls are both accepted
- verify level changes alter emitted structure or indentation as applicable
- verify the `last` indicator is honored in output shaping

Traceability:
- `src/output.c` `print_symbol (int direct, int level, int last, Symbol *sym)`

### Scenario 4: Emit multiple symbols in maintained order

The output subsystem processes multiple entries stored in its linked-list-based ordering structures and emits them sequentially.

Expected result:

- Output preserves list traversal order.
- No entries are skipped or duplicated during normal traversal.
- Per-symbol formatting still follows the active output style.

Test focus:
- verify ordered collections can be traversed for emission
- verify sequence output is stable
- verify end-of-sequence handling is compatible with the generic `last` context

Traceability:
- `src/output.c` `struct linked_list`
- `src/output.c` `struct linked_list_entry`
- `src/output.c` generic `print_symbol`

## Requirements

### Functional Requirements

#### FR-1: Style-specific symbol emission
The module shall provide style-specific symbol emission behavior for GNU and POSIX output modes.

Traceability:
- `src/gnu.c` `print_symbol`
- `src/posix.c` `print_symbol`

#### FR-2: Output sink targeting
The module shall emit formatted symbol output to a caller-provided output destination corresponding to the stream parameter evidenced in GNU and POSIX style printers.

Traceability:
- `src/gnu.c` `print_symbol(FILE *outfile, ...)`
- `src/posix.c` `print_symbol(FILE *outfile, ...)`

#### FR-3: Line-context-aware emission
The module shall accept a line-related input value during GNU and POSIX style symbol emission and use it as part of the formatting behavior preserved from the source module.

Traceability:
- `src/gnu.c` `print_symbol(FILE *outfile, int line, struct output_symbol *s)`
- `src/posix.c` `print_symbol(FILE *outfile, int line, struct output_symbol *s)`

#### FR-4: Generic symbol printing entry
The module shall provide generic symbol-printing behavior that accepts a symbol together with directness, nesting level, and last-element context.

Traceability:
- `src/output.c` `print_symbol (int direct, int level, int last, Symbol *sym)`

#### FR-5: Direct/non-direct handling
The generic output path shall preserve behavior distinctions associated with the `direct` argument.

Traceability:
- `src/output.c` `print_symbol (int direct, int level, int last, Symbol *sym)`

#### FR-6: Hierarchical or leveled output context
The generic output path shall preserve behavior distinctions associated with the `level` argument.

Traceability:
- `src/output.c` `print_symbol (int direct, int level, int last, Symbol *sym)`

#### FR-7: Sequence-end context handling
The generic output path shall preserve behavior distinctions associated with the `last` argument.

Traceability:
- `src/output.c` `print_symbol (int direct, int level, int last, Symbol *sym)`

#### FR-8: Driver-based formatting selection
The module shall retain driver-based separation between generic output control and concrete output formatting styles.

Traceability:
- `src/output.c` `struct output_driver`
- `src/gnu.c` `print_symbol`
- `src/posix.c` `print_symbol`

#### FR-9: Ordered output traversal support
The module shall preserve the ordered collection semantics used by the output subsystem so that symbol emission can proceed through linked entries in deterministic order.

Traceability:
- `src/output.c` `struct linked_list`
- `src/output.c` `struct linked_list_entry`

### Key Entities

#### Output Symbol
A symbol record used specifically for output formatting.

Evidence:
- `struct output_symbol` in `src/gnu.c`
- `struct output_symbol` in `src/posix.c`
- referenced in `src/output.c`

Relationship:
- consumed by GNU and POSIX style formatters
- represents the style-facing symbol data needed for emission

#### Generic Symbol
A broader symbol entity accepted by the generic output-layer printer.

Evidence:
- `Symbol *sym` in `src/output.c` `print_symbol`

Relationship:
- passed into the generic output path
- may be adapted or interpreted for style-specific emission

#### Output Driver
A driver entity that separates generic output orchestration from concrete formatting behavior.

Evidence:
- `struct output_driver` in `src/output.c`

Relationship:
- selected by the output layer
- routes symbol emission toward the appropriate formatting implementation

#### Linked List / Linked List Entry
Ordered container entities used within the output subsystem.

Evidence:
- `struct linked_list`
- `struct linked_list_entry`
- multiple linked-list-entry references in `src/output.c`

Relationship:
- maintain ordered traversal state for output processing
- support sequencing of symbols or related output entries

## Success Criteria

1. The Rust module can emit a valid symbol through a GNU-style formatting path and a POSIX-style formatting path, with each path remaining distinct.
   - Traceability: `src/gnu.c` `print_symbol`, `src/posix.c` `print_symbol`

2. The Rust module accepts an output destination and a line-context value for GNU and POSIX style symbol emission.
   - Traceability: GNU/POSIX `print_symbol(FILE *outfile, int line, ...)`

3. The Rust module provides a generic symbol-printing path that accepts and honors `direct`, `level`, and `last` context inputs.
   - Traceability: `src/output.c` generic `print_symbol`

4. Changing the active output style changes which formatter is used without requiring changes to the generic symbol-printing caller flow.
   - Traceability: `src/output.c` `struct output_driver`, style-specific printers

5. When emitting multiple ordered entries, traversal is deterministic and does not skip or duplicate entries under normal operation.
   - Traceability: `src/output.c` `struct linked_list`, `struct linked_list_entry`

6. The Rust rewrite reproduces observable output behavior for supported styles and generic context inputs closely enough to replace this module in the branch rewrite target.
   - Traceability: all analyzed functions and core entities in `src/gnu.c`, `src/output.c`, `src/posix.c`