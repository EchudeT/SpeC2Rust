# spec.md

## Title

Rust Functional Specification for `module_src_parseopt_help_context_05`

## Metadata

- Project: `cflow-new`
- Module: `module_src_parseopt_help_context_05`
- Category: `module_cluster`
- Source file: `src/parseopt/help.c`
- Rust branch target: `102-module_src_parseopt_help_context_05-rust-port`
- Generation date: `2026-06-17`

## Overview

This module is the option-help context stage responsible for preparing and emitting a help output section for a group of options.

Based on the analyzed functions and data structures in `src/parseopt/help.c`, this module has two core responsibilities:

1. Order the option definitions associated with a help context.
2. Print one option group from that prepared context to a word-wrapped output sink.

The Rust rewrite must preserve the observable behavior of this stage within the help-generation pipeline: a context containing option definitions is sorted into display order, and the current option group is emitted using the same contextual data relationships used by the C module.

## Scope

### In Scope

- Representing the help-printing context needed by this stage.
- Sorting option definitions for presentation.
- Emitting one option group to the provided wrapped-output target.
- Preserving context-driven grouping behavior implied by `help_context`, `optdef`, and option-sorting support data.

### Out of Scope

- Defining the full command-line parser.
- Inventing new formatting features beyond the evidenced help-group sorting and printing behavior.
- Adding new public APIs not required to support the existing module behavior.
- Changing option semantics or introducing new grouping models.

## Feature Specification

### Feature: Context-based option ordering

The module must accept a help-generation context containing option definitions and reorder the relevant options into the display order used for help output.

This behavior is evidenced by:

- `sort_options(struct help_context *ctx)`
- `struct help_context`
- `struct optdef`
- `struct optsort`

The Rust version must implement equivalent ordering behavior at the module boundary:

- Sorting operates on the option data associated with a help context.
- Sorting is performed before printing a group when ordered output is required by the original module flow.
- The resulting order is stable and deterministic for the same input context.

### Feature: Printing a single option group

The module must print one help option group from the current help context to a wrapped output destination.

This behavior is evidenced by:

- `print_option_group(WORDWRAP_FILE wf, struct help_context *ctx)`
- `struct help_context`
- `struct optdef`

The Rust version must implement equivalent group emission behavior:

- It reads the option-group state from the help context.
- It emits that group to an output abstraction corresponding to the original wrapped file sink.
- It uses the already prepared option definitions in their effective display order.
- It preserves group-oriented output semantics rather than flattening all options into one undifferentiated block.

## User Scenarios & Testing

### Scenario 1: Help output for a context with multiple options

A caller prepares a help context containing multiple option definitions that belong to the same displayed group. The module sorts those options and prints the group to the wrapped output sink.

The Rust version must support:

- Accepting a populated context.
- Producing deterministic option ordering.
- Emitting all options in the group to the output sink.

#### Test expectations

- Given the same input context twice, the rendered group output is identical.
- All options present in the targeted group are printed exactly once.
- Output order reflects the module’s sorting behavior.

### Scenario 2: Help output with more than one group in the wider help flow

A caller iterates through help groups using one shared or sequentially prepared context values. For each group, this module prints the current group only.

The Rust version must support:

- Isolated printing of a single group from context.
- No leakage of entries from other groups into the current group output.

#### Test expectations

- When contexts differ only by active/current group contents, each printed result contains only that group’s options.
- Adjacent group prints remain distinct in content and ordering.

### Scenario 3: Pre-sorted and unsorted option definitions

A caller may provide option definitions in arbitrary order. The module normalizes presentation order before output.

The Rust version must support:

- Sorting unsorted input.
- Preserving the same output when the input is already in effective display order.

#### Test expectations

- Two contexts containing the same option set in different insertion orders produce the same rendered group output after sorting.
- Already ordered input does not change the visible result.

### Scenario 4: Empty or no-print group content

A caller provides a help context in which the current group contains no printable options.

The Rust version must support whatever observable behavior the original module exhibits for an empty group, without inventing extra text.

#### Test expectations

- The module does not panic or crash on empty group input.
- Output for an empty group is consistent and deterministic.
- No unrelated options are emitted.

## Requirements

### Functional Requirements

#### FR-1: Help context consumption

The module shall operate on a help-generation context that carries the option data needed for sorting and group printing.

**Traceability:** `struct help_context` in `src/parseopt/help.c`; functions `sort_options`, `print_option_group`.

#### FR-2: Option-definition based processing

The module shall use option-definition records as the units to be ordered and printed.

**Traceability:** `struct optdef`; functions `sort_options`, `print_option_group`.

#### FR-3: Deterministic option sorting

The module shall provide deterministic sorting of the option definitions associated with the active help context before group output is rendered.

**Traceability:** `sort_options`; `struct optsort`; `struct help_context`.

#### FR-4: Group-scoped printing

The module shall print one option group at a time from the provided help context rather than emitting unrelated groups together.

**Traceability:** `print_option_group`; `struct help_context`.

#### FR-5: Wrapped-output emission

The module shall emit printed group content to an output abstraction equivalent in role to the original word-wrapped file target.

**Traceability:** `print_option_group(WORDWRAP_FILE wf, struct help_context *ctx)`.

#### FR-6: Sorted-order rendering

The module shall render option entries using the effective order established for the help context, so that visible output reflects the module’s sort stage.

**Traceability:** `sort_options`; `print_option_group`.

#### FR-7: Empty-input safety

The module shall handle a help context whose current group has no printable options without failure.

**Traceability:** `print_option_group`; `struct help_context`; presence of grouped context-based printing implies possible empty group states.

### Key Entities

#### `help_context`

The central state carrier for this module. It links the broader parse/help state to the option definitions that are being prepared and printed for a specific help-output step.

Relationship summary:

- Owns or references the option collection relevant to help output.
- Provides the current scope needed by sorting.
- Provides the current group scope needed by printing.

**Traceability:** `struct help_context` (`src/parseopt/help.c:397-405` and related references).

#### `optdef`

The option-definition entity processed by this module. Each `optdef` represents one help-displayable option entry or one unit participating in ordering and group output.

Relationship summary:

- Multiple `optdef` records are associated with one `help_context`.
- `sort_options` reorders these records for presentation.
- `print_option_group` emits them for one group.

**Traceability:** `struct optdef` references throughout `src/parseopt/help.c`; direct use in both listed functions.

#### `optsort`

A sorting support entity used to establish option presentation order.

Relationship summary:

- Derived from or associated with `optdef` items during sorting.
- Used by the sorting stage to determine final display order inside a `help_context`.

**Traceability:** `struct optsort` (`src/parseopt/help.c:330-337`, related references); `sort_options`.

## Non-Goals

The Rust rewrite must not assume or introduce:

- New end-user formatting controls.
- New grouping semantics beyond those already represented by the help context.
- New persistence or interchange formats.
- New concurrency guarantees.
- New error-reporting channels unless required to preserve existing observable behavior.

## Success Criteria

1. **Deterministic ordering:** For identical help-context input, the Rust module produces identical option-group output across repeated runs.
   **Traceability:** `sort_options`, `print_option_group`.

2. **Order normalization:** For the same set of option definitions supplied in different initial orders, the Rust module produces the same printed group output after sorting.
   **Traceability:** `sort_options`; `struct optsort`; `struct optdef`.

3. **Group isolation:** Printing one group emits only the options belonging to that group context and does not include options from unrelated groups.
   **Traceability:** `print_option_group`; `struct help_context`.

4. **Output sink compatibility at behavior level:** Group output is written through a wrapped-output abstraction corresponding to the original `WORDWRAP_FILE` role.
   **Traceability:** `print_option_group(WORDWRAP_FILE wf, struct help_context *ctx)`.

5. **Empty-group robustness:** A context with no printable options for the current group completes without panic, crash, or emission of unrelated option entries.
   **Traceability:** `print_option_group`; `struct help_context`.

6. **Traceable entity preservation:** The Rust design retains explicit representations for the help context, option definitions, and sorting support sufficient to preserve the original module behavior.
   **Traceability:** `struct help_context`; `struct optdef`; `struct optsort`.

## Acceptance Notes

Conformance should be assessed by black-box tests around:

- Context preparation with known option sets.
- Sorting invariance under input permutation.
- Rendering of single-group output.
- Empty-group behavior.

The Rust rewrite is acceptable when these behaviors match the functional boundaries evidenced by `src/parseopt/help.c` for this module slice.