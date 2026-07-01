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

This module covers the portion of parse-option help generation responsible for ordering collected option definitions and emitting one formatted option group within a help output stream.

The Rust rewrite must preserve the observed behavior of this module segment:

- arrange the current option set in the help context into display order;
- use the help context’s current option range/group state when producing output;
- emit the formatted text for one option group to the help-output writer.

This specification is limited to the functional boundary evidenced by the analyzed file and the identified functions:

- `sort_options`
- `print_option_group`

No additional capabilities, public interfaces, or behaviors are required beyond those needed to match this module’s existing role in help generation.

## Scope

### In Scope

- Sorting option definitions held by the help-generation context before printing.
- Printing a single option group from the help-generation context to a wrapped output destination.
- Respecting the relationships between help context state, option definitions, and sort ordering.

### Out of Scope

- Parsing command-line arguments.
- Defining option semantics outside help rendering.
- Full help-page orchestration outside this module segment.
- Any new formatting modes, persistence, FFI, or concurrency guarantees.

## Feature Specification

### Feature: Help-context option ordering

The module must support ordering option definitions associated with a help-generation context before group output is produced.

Observed evidence indicates that the module maintains:

- a help context containing parse-option and option-definition state;
- a sorting representation (`optsort`) associated with option definitions;
- a sort step invoked before printing grouped help output.

The Rust version must therefore:

1. accept or access the current collection of option definitions represented in the help context;
2. derive a stable display order consistent with the original module behavior for the same input set;
3. update the context state used by subsequent group printing so that printed options appear in sorted order.

### Feature: Single-group help output

The module must emit one option group to a word-wrapping help output sink.

The Rust version must therefore:

1. read the current help context and its current option-group boundaries or active subset;
2. print the option entries belonging to that group;
3. write through the provided wrapped-output abstraction;
4. preserve group-local formatting behavior expected by the surrounding help system.

### Feature: Integration with help-generation context

This module is context-driven rather than standalone. The Rust rewrite must preserve that model.

The Rust version must:

- operate on the module’s help context entity rather than inventing detached standalone inputs;
- treat option definitions as the source records for both sort and print steps;
- preserve the dependency chain where sorting prepares data for printing.

## User Scenarios & Testing

### Scenario 1: Options are sorted before help display

A caller has collected multiple option definitions in a help context and requests help output for a group.

Expected behavior:

- the module orders the options according to the original help-module rules;
- the printed group reflects that order rather than raw insertion order, if those differ.

Test guidance:

- provide a context with several options in unsorted order;
- invoke the Rust-equivalent sort step and then group printing;
- verify output ordering matches the C module for the same fixture.

### Scenario 2: A single option group is printed to wrapped output

A caller has a help context positioned on one group of options and a word-wrapping output writer.

Expected behavior:

- only the current group is emitted;
- output is written to the supplied writer;
- option lines follow the sorted context state.

Test guidance:

- create a context containing more than one conceptual group or subset;
- print one group;
- verify only that group’s entries appear in the produced text.

### Scenario 3: Empty or no-op group handling

A caller asks the module to print a group when the current group contains no printable options.

Expected behavior:

- the module completes without inventing option entries;
- output remains consistent with the original module’s handling of empty input.

Test guidance:

- compare Rust and C behavior for an empty group fixture;
- verify no spurious option lines are emitted.

### Scenario 4: Sorting affects subsequent printing only through context state

A caller performs sorting once and then prints a group from the same help context.

Expected behavior:

- printed output reflects the context’s sorted option arrangement;
- no separate parallel option list is required by the caller.

Test guidance:

- inspect output before and after sort in a controlled fixture, if the surrounding harness allows;
- verify the print phase consumes the sorted context-managed data.

## Requirements

### Functional Requirements

#### FR-1: Sort option definitions in help context

The module shall provide behavior equivalent to `sort_options`, ordering the option definitions associated with a help context for later formatted help output.

Traceability:
- `src/parseopt/help.c`
- `sort_options`
- `help_context`
- `optsort`
- `optdef`

#### FR-2: Use help-context-managed option records as sort input

The sort behavior shall operate on the option records already gathered in the help context and shall not require a separate external option collection model.

Traceability:
- `src/parseopt/help.c`
- `sort_options`
- `help_context`
- `optdef`

#### FR-3: Preserve deterministic print order for identical input

For the same help context contents, the Rust version shall produce the same option ordering as the C module segment when exercised through equivalent surrounding code.

Traceability:
- `src/parseopt/help.c`
- `sort_options`
- `print_option_group`

#### FR-4: Print one option group to the wrapped output sink

The module shall provide behavior equivalent to `print_option_group`, emitting a single formatted option group to the supplied word-wrapping output target.

Traceability:
- `src/parseopt/help.c`
- `print_option_group`
- `help_context`

#### FR-5: Print using sorted context state

Group printing shall consume the option ordering established in the help context by the sort phase, so displayed options appear in the intended order.

Traceability:
- `src/parseopt/help.c`
- `sort_options`
- `print_option_group`
- `help_context`

#### FR-6: Restrict output to the active group/subset represented by context

The print behavior shall emit only the option entries belonging to the current group or active subset represented by the help context.

Traceability:
- `src/parseopt/help.c`
- `print_option_group`
- `help_context`
- `optdef`

#### FR-7: Support empty-group execution without fabricated entries

When the current group contains no printable options, the module shall complete without generating invented option records or unrelated text.

Traceability:
- `src/parseopt/help.c`
- `print_option_group`
- `help_context`

### Key Entities

#### `help_context`

The central state carrier for this module segment.

Functional role:

- owns or references parse-option help-generation state;
- holds the option-definition collection used for sorting;
- provides the current group/subset context for printing;
- links the sort phase to the print phase.

Relationship summary:

- contains or references `optdef` records;
- is consumed by both `sort_options` and `print_option_group`;
- may use `parseopt` state as upstream help/input context.

#### `optdef`

Represents an option definition as used for help generation.

Functional role:

- serves as the source item being ordered;
- supplies the printable option entry data used during group output.

Relationship summary:

- collected under `help_context`;
- may be wrapped or referenced by `optsort` during ordering.

#### `optsort`

Represents sort-oriented metadata or pairing used to derive display order.

Functional role:

- supports the ordering of `optdef` entries before printing.

Relationship summary:

- used by `sort_options`;
- maps back to `optdef` items associated with `help_context`.

#### `parseopt`

Represents broader parse-option configuration referenced by help-generation state.

Functional role:

- provides upstream option/help information that feeds the help context.

Relationship summary:

- referenced by `help_context`;
- not the direct print target in this module segment.

## Success Criteria

### SC-1: Ordering parity

Given identical help-context fixtures, the Rust module produces the same option order as the C module segment for the paths covered by `sort_options`.

### SC-2: Group-output parity

Given identical sorted context fixtures and the same effective output settings, the Rust module produces text for one option group that is behaviorally equivalent to the C module segment for the paths covered by `print_option_group`.

### SC-3: Context-driven operation

Tests demonstrate that sorting and printing operate through `help_context` and `optdef` relationships, without requiring callers to supply an extra out-of-band ordered list.

### SC-4: Empty-group correctness

For fixtures where the active group has no printable options, the Rust module completes successfully and emits no fabricated option entries.

### SC-5: End-to-end local integration

In a harness that mirrors the surrounding help-generation flow for this module segment, invoking sort followed by single-group print yields output matching the C implementation for representative fixtures, including:
- unsorted option input;
- multi-entry group input;
- empty-group input.

## Acceptance Notes

- Behavioral matching to the C module is the acceptance standard.
- Where exact formatting details depend on surrounding help infrastructure, the Rust port must preserve the behavior observable at this module boundary.
- No requirement in this document authorizes expanding the module beyond sorting and single-group help printing responsibilities.