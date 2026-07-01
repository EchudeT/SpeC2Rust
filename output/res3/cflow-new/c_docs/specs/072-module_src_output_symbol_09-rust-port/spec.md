# spec.md

## Title

Rust Functional Specification for `module_src_output_symbol_09`

## Metadata

- Project: `cflow-new`
- Module: `module_src_output_symbol_09`
- Category: `module_cluster`
- Target Rust Branch: `072-module_src_output_symbol_09-rust-port`
- Source Basis: `src/gnu.c`, `src/output.c`, `src/posix.c`
- Generation Date: 2026-06-17

## Overview

This module is responsible for symbol-oriented output generation within the project’s reporting/output subsystem. The analyzed sources show three closely related `print_symbol` behaviors:

- a GNU-style symbol printer in `src/gnu.c`
- a POSIX-style symbol printer in `src/posix.c`
- a generic/output-tree symbol printer in `src/output.c`

The Rust rewrite must preserve the functional role of this module as a formatter/emitter of symbol records for different output styles, including direct line-oriented emission and hierarchical/tree-oriented emission. The module also participates in selecting or structuring output behavior through an output-driver concept and linked traversal state used by the generic output path.

This specification covers only behavior evidenced by the analyzed module files and referenced functions/types.

## Feature Specification

### Feature: Format and emit symbols in multiple output styles

The module shall format symbol information for output according to the active output style represented by the source variants.

Evidence:
- `src/gnu.c:41-59` `print_symbol(FILE *outfile, int line, struct output_symbol *s)`
- `src/posix.c:32-47` `print_symbol(FILE *outfile, int line, struct output_symbol *s)`
- `src/output.c:156-170` `print_symbol (int direct, int level, int last, Symbol *sym)`

Required Rust behavior:
- Support a GNU-style symbol emission path.
- Support a POSIX-style symbol emission path.
- Support a generic symbol emission path that accepts hierarchical context (`level`, `last`) and a mode flag (`direct`).
- Preserve the distinction between line-oriented output to a provided sink and generic symbol printing driven by symbol/tree traversal context.

### Feature: Emit one symbol record at a time

Each `print_symbol` function in the analyzed files operates on a single symbol and returns a status value. The Rust version must preserve single-symbol emission as the unit of work.

Evidence:
- Each listed `print_symbol` function accepts exactly one symbol argument and returns `int`.

Required Rust behavior:
- Accept one symbol object per print operation.
- Produce output only for the provided symbol and the contextual formatting parameters passed with it.
- Return success/failure status in a Rust-appropriate form that still allows callers to distinguish successful emission from failure.

### Feature: Respect caller-provided output destination or active output driver context

The GNU and POSIX variants explicitly receive an output stream; the generic output path relies on module-level output-driver behavior.

Evidence:
- `src/gnu.c` and `src/posix.c` `print_symbol(FILE *outfile, ...)`
- `src/output.c` includes `struct output_driver`

Required Rust behavior:
- Support writing GNU/POSIX formatted symbol output to a caller-specified output sink.
- Support generic output through the module’s active formatting/output-driver path.
- Keep output behavior separated by style rather than merging all formatting into one indistinct format.

### Feature: Support hierarchical symbol output context

The generic `print_symbol` includes `level` and `last` parameters, indicating formatting behavior depends on symbol depth and whether the current symbol is the last child/item in a sequence.

Evidence:
- `src/output.c:156-170` `print_symbol (int direct, int level, int last, Symbol *sym)`

Required Rust behavior:
- Use nesting/depth context when emitting generic symbol output.
- Use end-of-sequence/last-item context when emitting generic symbol output.
- Preserve output differences that depend on direct-vs-non-direct mode and tree-position context.

### Feature: Operate over module-managed symbol/output support entities

The analyzed module references `output_symbol`, `output_driver`, and linked-list entities in `src/output.c`, indicating that symbol emission is part of a broader output pipeline with driver selection and traversal/bookkeeping support.

Evidence:
- `struct output_symbol` in `src/gnu.c`, `src/posix.c`, and `src/output.c`
- `struct output_driver` in `src/output.c`
- `struct linked_list` and `struct linked_list_entry` in `src/output.c`

Required Rust behavior:
- Represent symbol output records/entities needed by style-specific printers.
- Represent output-driver behavior needed by the generic output path.
- Represent traversal/order support sufficient to preserve symbol emission order and hierarchical relationships used by the generic printer.

## User Scenarios & Testing

### Scenario 1: GNU-style output of a symbol to a selected writer

A caller has a symbol prepared in output-symbol form and wants it emitted using GNU formatting to a specific output destination, with an associated line number.

Expected support:
- The Rust module can format that symbol in GNU style.
- The result is written to the provided output sink.
- The call reports success or failure.

Traceability:
- `src/gnu.c:41-59`

Suggested tests:
- Emit a valid symbol using GNU style and verify non-empty output is written.
- Verify the output path accepts a caller-selected sink.
- Verify error status is returned if the sink write fails.

### Scenario 2: POSIX-style output of a symbol to a selected writer

A caller wants the same symbol information emitted in POSIX formatting to a specific output destination.

Expected support:
- The Rust module can format that symbol in POSIX style.
- The result is written to the provided output sink.
- The call reports success or failure.

Traceability:
- `src/posix.c:32-47`

Suggested tests:
- Emit a valid symbol using POSIX style and verify non-empty output is written.
- Confirm GNU and POSIX style paths are independently callable.
- Verify write failure is surfaced as a failed operation.

### Scenario 3: Generic hierarchical output during traversal

A caller traverses symbols in a call tree or similar hierarchy and requests output for each symbol, passing whether the output is direct, the current nesting level, and whether the symbol is the last one at that level.

Expected support:
- The Rust module emits the symbol according to the active generic output style.
- Output reflects the provided hierarchy context.
- Different context values can produce different formatting outcomes.

Traceability:
- `src/output.c:156-170`

Suggested tests:
- Print the same symbol at different levels and verify output changes accordingly.
- Print symbols with `last` toggled and verify sequence-sensitive formatting changes accordingly.
- Print with `direct` toggled and verify mode-sensitive formatting differences.

### Scenario 4: Ordered emission across a linked/tracked symbol set

The generic output path uses linked-list support entities, implying ordered traversal/output over a maintained collection of entries.

Expected support:
- The Rust rewrite preserves deterministic output order for the symbol sequence it is asked to print.
- Symbol emission during traversal maintains the caller-visible hierarchy/order semantics.

Traceability:
- `src/output.c` linked-list and linked-list-entry entities near lines 209, 212, 283, 289, 302, 327, 362

Suggested tests:
- Build a small ordered symbol set and verify emitted order matches traversal order.
- Build a parent/child arrangement and verify printed order and level handling remain consistent.

## Requirements

### Functional Requirements

#### FR-1: Style-specific symbol emission
The module shall provide symbol emission behavior for at least the three evidenced paths: GNU-style, POSIX-style, and generic hierarchical output.

Traceability:
- `src/gnu.c:41-59`
- `src/posix.c:32-47`
- `src/output.c:156-170`

#### FR-2: Single-symbol print operation
Each emission operation shall process one symbol record/entity at a time and return a status indicating success or failure.

Traceability:
- All three `print_symbol` functions

#### FR-3: Caller-supplied sink support for GNU/POSIX paths
The GNU-style and POSIX-style emission paths shall write to a caller-provided output destination.

Traceability:
- `FILE *outfile` parameter in `src/gnu.c` and `src/posix.c`

#### FR-4: Line-context-sensitive GNU/POSIX emission
The GNU-style and POSIX-style emission paths shall accept and use line-context input associated with the symbol output operation.

Traceability:
- `int line` parameter in `src/gnu.c` and `src/posix.c`

#### FR-5: Hierarchy-sensitive generic emission
The generic output path shall accept and use symbol hierarchy context, including nesting level and last-item position.

Traceability:
- `int level, int last` parameters in `src/output.c:156-170`

#### FR-6: Mode-sensitive generic emission
The generic output path shall accept and use a direct/non-direct mode flag when formatting a symbol.

Traceability:
- `int direct` parameter in `src/output.c:156-170`

#### FR-7: Output-driver-governed generic formatting
The module shall preserve the concept that generic symbol emission occurs under output-driver control or equivalent style-selection behavior.

Traceability:
- `struct output_driver` in `src/output.c`

#### FR-8: Ordered traversal support for symbol emission
The module shall preserve ordered symbol emission behavior needed by the generic path’s linked traversal/bookkeeping structures.

Traceability:
- `struct linked_list`
- `struct linked_list_entry`
- Generic `print_symbol` in `src/output.c`

### Key Entities

#### Entity: Output symbol
An output-oriented symbol record used by the GNU and POSIX printers and referenced in the generic output file.

Traceability:
- `struct output_symbol` in `src/gnu.c`, `src/posix.c`, `src/output.c`

Relationship requirements:
- A GNU/POSIX printer consumes one output symbol per call.
- The entity contains the symbol information necessary for style-specific textual emission.

#### Entity: Generic symbol
A symbol object used by the generic output path.

Traceability:
- `Symbol *sym` parameter in `src/output.c:156-170`

Relationship requirements:
- The generic printer consumes one generic symbol per call.
- The generic symbol participates in hierarchical output controlled by `level`, `last`, and `direct`.

#### Entity: Output driver
A formatting/output behavior selector or dispatcher used by the generic output subsystem.

Traceability:
- `struct output_driver` in `src/output.c`

Relationship requirements:
- The generic output path operates under a selected driver behavior.
- Driver behavior determines how generic symbol output is formatted or routed.

#### Entity: Linked list / linked list entry
Traversal or ordering support structures used within the generic output file.

Traceability:
- `struct linked_list`
- `struct linked_list_entry`

Relationship requirements:
- Linked-list entries represent ordered elements involved in generic output processing.
- Traversal order derived from these entities must be preserved in emitted output where applicable.

## Success Criteria

1. The Rust module can emit one symbol in GNU style to a provided output sink and report success/failure.
   - Traceability: `src/gnu.c:41-59`

2. The Rust module can emit one symbol in POSIX style to a provided output sink and report success/failure.
   - Traceability: `src/posix.c:32-47`

3. The Rust module can emit one generic symbol with caller-provided `direct`, `level`, and `last` context.
   - Traceability: `src/output.c:156-170`

4. For generic output, changing `level` or `last` can change the resulting formatting in a deterministic, testable way.

5. GNU-style and POSIX-style symbol emission remain distinct behaviors rather than collapsing into a single undifferentiated formatter.
   - Traceability: separate `print_symbol` implementations in `src/gnu.c` and `src/posix.c`

6. The Rust rewrite preserves ordered symbol emission semantics required by the linked traversal/bookkeeping structures used in the generic output path.
   - Traceability: linked-list entities in `src/output.c`

7. All supported emission paths surface output/write failure to the caller through an explicit failed result.
   - Traceability: `int` return from all three `print_symbol` functions

8. The implemented Rust behavior is fully traceable to the source-evidenced entities and functions listed in this specification, with no required capability added beyond those boundaries.
   - Traceability: all analyzed module files and listed functions/types