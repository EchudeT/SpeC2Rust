# spec.md

## Title

Rust Functional Specification for `module_src_parseopt_help_context_05`

## Metadata

- Project: `cflow-new`
- Module: `module_src_parseopt_help_context_05`
- Category: `module_cluster`
- Source file: `src/parseopt/help.c`
- Rust branch target: `102-module_src_parseopt_help_context_05-rust-port`
- Generation date: `2026-06-11`

## Overview

This module is a help-output subcomponent of the parse-option help system. Its evidenced responsibility is limited to preparing and emitting one help option group from an existing help context.

The Rust rewrite must preserve the functional behavior represented by:

- `sort_options(struct help_context *ctx)`
- `print_option_group(WORDWRAP_FILE wf, struct help_context *ctx)`

From the available evidence, this module does not define option parsing itself. Instead, it works on already collected option definitions within a help-generation context, orders those options for display, and prints the current option group through a word-wrapping output target.

## Feature Specification

### Feature: Help option group preparation and output

The module shall support generation of formatted help output for a group of options that already exist in a help context.

Its functional scope is:

1. consume a `help_context` containing option-definition records;
2. arrange the option records into display order;
3. emit the option group to a word-wrapped output sink.

The Rust version must implement the same functional boundary: it must operate on an in-memory help context and produce group help output for that context without taking over broader command-line parsing responsibilities.

### Included behavior

#### 1. Option ordering for help display

The module shall provide behavior equivalent to sorting the option set stored in the help context before printing.

This sorting behavior is evidenced by `sort_options` and the presence of `optsort`, `optdef`, and `help_context` structures. The Rust version must therefore support:

- deriving a display order from the option definitions present in the context;
- storing or applying the sorted order in a way usable by group printing;
- ensuring the printed group follows that order.

The specification does not require exposing sorting as a public API unless needed by the Rust design, but the behavior must exist.

#### 2. Group-oriented help output

The module shall print one option group from the prepared help context to a wrapped output destination.

This behavior is evidenced by `print_option_group(WORDWRAP_FILE wf, struct help_context *ctx)`. The Rust version must therefore support:

- reading the group state from the help context;
- iterating over the option entries belonging to that printable group context;
- emitting the group content to the provided output abstraction;
- honoring the intended wrapped-output destination role represented by `WORDWRAP_FILE`.

#### 3. Use of existing option metadata

The module shall use existing option-definition metadata rather than recomputing or reparsing command-line specification text.

This is evidenced by repeated references to `parseopt`, `optdef`, and `help_context`, where `help_context` holds or references option definitions gathered elsewhere in the same source file. The Rust version must preserve that dependency direction:

- option definitions are input to this module;
- display ordering and printing are outputs of this module.

### Out of scope

The Rust rewrite shall not be required by this specification to add functionality not evidenced in the analyzed source, including:

- command-line argument parsing;
- definition of new public help formatting features beyond existing group sorting and printing behavior;
- persistence, serialization, FFI, concurrency guarantees, or recovery frameworks.

## User Scenarios & Testing

### Scenario 1: Print a help section after options have been collected

A caller has already assembled a help context containing multiple option definitions. The caller invokes the group-printing path to render a section of help text.

Expected result:

- the module orders the options according to its help-display rules;
- the module prints the option group to the target output sink;
- the resulting output reflects the sorted order rather than arbitrary insertion order.

### Scenario 2: Print help through a wrapped-output destination

A caller directs help output to a word-wrapping writer abstraction. The module prints the current option group using that output target.

Expected result:

- the module writes to the supplied output abstraction, not to an implicit global destination;
- option-group output is produced through the same destination passed by the caller.

### Scenario 3: Help context contains multiple option definitions with differing display positions

A help context contains several option definitions that require ordering for readable help display.

Expected result:

- all printable option entries in the target group are included;
- ordering is stable with respect to the module's defined comparison rules from the original behavior;
- output group traversal follows the ordered result.

### Scenario 4: Group printing after repeated invocations

A caller uses the module multiple times on prepared contexts or repeated output generation passes.

Expected result:

- each invocation prints from the provided context state;
- the produced group output is consistent for equivalent input contexts.

### Testing expectations

The Rust version must be testable with fixtures that construct `help_context` data and verify:

- options are sorted before group output;
- printed order matches the expected order for the fixture;
- output is written to an injected output sink;
- a context with zero printable entries for the group does not produce unexpected option lines.

## Requirements

### Functional Requirements

#### FR-1: Sort option definitions for help display

The module shall sort option definitions contained in the help context before they are used for group output.

Traceability:
- `sort_options` in `src/parseopt/help.c`
- `struct help_context`
- `struct optsort`
- `struct optdef`

#### FR-2: Base sorting on option-definition data already present in context

The module shall derive display order from option-definition records already associated with the help context.

Traceability:
- `sort_options`
- `struct help_context`
- `struct optdef`

#### FR-3: Print one option group from the help context

The module shall emit the help text for a group of options represented by the current help context.

Traceability:
- `print_option_group`
- `struct help_context`

#### FR-4: Send help group output to the provided wrapped-output target

The module shall write option-group help output to the supplied output destination abstraction corresponding to `WORDWRAP_FILE`.

Traceability:
- `print_option_group(WORDWRAP_FILE wf, struct help_context *ctx)`

#### FR-5: Print options in sorted display order

The module shall ensure that the order visible in printed group output matches the order produced by the sorting step.

Traceability:
- `sort_options`
- `print_option_group`
- `struct optsort`

#### FR-6: Operate on prepared parse-option metadata rather than raw command-line input

The module shall consume preexisting parse-option/help metadata and shall not require raw argument parsing as part of its group-printing responsibility.

Traceability:
- `struct parseopt`
- `struct optdef`
- `struct help_context`
- functions limited to sorting and printing in `src/parseopt/help.c`

### Key Entities

#### `help_context`

Primary working context for help generation in this module.

Required role in Rust:

- holds or references the option/help data needed for sorting;
- provides the current group-printing state;
- is the input context for both sorting and printing behavior.

Traceability:
- `struct help_context` references in `src/parseopt/help.c`
- parameters of `sort_options` and `print_option_group`

#### `optdef`

Represents an option definition used for help output.

Required role in Rust:

- acts as the printable option metadata unit;
- participates in ordering and group output;
- is associated with the help context.

Traceability:
- `struct optdef` references in `src/parseopt/help.c`

#### `optsort`

Represents sort-related organization of option definitions.

Required role in Rust:

- supports the module’s option ordering behavior;
- connects option definitions to the display order used during printing.

Traceability:
- `struct optsort` references in `src/parseopt/help.c`
- `sort_options`

#### `parseopt`

Represents broader parse-option metadata that the help context references.

Required role in Rust:

- serves as upstream option/help specification input to the help context;
- is not itself the output of this module, but provides source metadata for displayed options.

Traceability:
- `struct parseopt` references in `src/parseopt/help.c`

#### Wrapped output target (`WORDWRAP_FILE` equivalent)

Represents the destination used by group-printing logic.

Required role in Rust:

- accepts emitted help text;
- is supplied by the caller during group printing.

Traceability:
- parameter of `print_option_group`

### Entity Relationships

- A `help_context` contains or references multiple `optdef` records.
- Sorting logic organizes those `optdef` records through `optsort` or an equivalent Rust representation.
- Group printing reads the ordered option data from `help_context`.
- `parseopt` metadata is an upstream source for the option definitions represented in the help context.
- Printed output is emitted to the wrapped output target provided by the caller.

## Success Criteria

### SC-1: Sorted-output equivalence

Given a constructed help context with multiple option definitions requiring ordering, the Rust module prints the group in the same display order as produced by the original C module behavior.

Traceability:
- `sort_options`
- `print_option_group`

### SC-2: Output-target correctness

Given a supplied output sink, the Rust module writes group help output to that sink and does not require an implicit output destination.

Traceability:
- `print_option_group(WORDWRAP_FILE wf, struct help_context *ctx)`

### SC-3: Context-driven operation

Given a prepared help context, the Rust module can sort and print the option group without reparsing raw command-line arguments.

Traceability:
- `struct help_context`
- `struct parseopt`
- `struct optdef`

### SC-4: Coverage of printable group entries

For a help context whose current group contains printable options, the Rust module emits all of those group entries in the resulting output.

Traceability:
- `print_option_group`
- `struct help_context`
- `struct optdef`

### SC-5: Deterministic repeated behavior

For repeated invocations with equivalent input contexts, the Rust module produces equivalent group output.

Traceability:
- `sort_options`
- `print_option_group`

## Notes for Rust Port Scope

The Rust port should preserve functional behavior at the module boundary shown by the analyzed functions and types. Internal representation may change, but the port must still express:

- a help-generation context;
- option-definition records;
- sorting of those records for display;
- group printing to a wrapped writer abstraction.

No additional capabilities are required beyond the evidenced behavior in `src/parseopt/help.c`.