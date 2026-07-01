# spec.md

## Title

Functional Specification: `module_src_parseopt_parseopt_02` Rust Port

## Document Control

- Project: `cflow-new`
- Module: `module_src_parseopt_parseopt_02`
- Category: `module_cluster`
- Source file: `src/parseopt/parseopt.c`
- Target Rust branch: `099-module_src_parseopt_parseopt_02-rust-port`
- Generation date: `2026-06-17`

## Overview

This module provides command-line option parsing over a mutable parser state and an argument vector. It supports recognition of short and long options, handling of option arguments, controlled advancement through the argument stream, permutation of non-option arguments when scanning mode allows it, and reporting of parse errors through the parser’s configured error path.

The Rust rewrite must preserve the observable parsing behavior evidenced by the source module, including:

- lookup of short options and long options
- distinction between options and non-option arguments
- support for option arguments when attached or supplied separately
- negative-form matching behavior for long options where defined by option metadata
- parser lookahead and skip operations
- exposure of the current post-parse `argc`/`argv` view
- preparation of option definitions before parsing, including help/usage setters present in this module
- error emission through the parser context

## Scope

In scope:

- Behavior implemented by the parser operations in `src/parseopt/parseopt.c`
- The parser state transitions needed to parse an argument list incrementally
- Option-definition preparation and indexing needed by parsing

Out of scope:

- Any capabilities not evidenced in this module
- New public APIs beyond those required to preserve this module’s behavior
- Thread-safety, persistence, serialization, FFI, or performance guarantees

## Feature Specification

### 1. Incremental option parsing

The module parses arguments one item at a time from a parser-owned argument vector. A caller advances parsing through a “next option” operation and may also inspect the upcoming argument without consuming it.

The Rust version must implement incremental parsing semantics equivalent to the C module:

- consume recognized options in sequence
- distinguish option tokens from ordinary positional arguments
- return associated option arguments when an option accepts or requires one
- preserve parser progress across repeated calls

### 2. Short option recognition

The module recognizes short-form options from the current argument position. It also supports short-option processing where the parser may continue within the same argument text rather than requiring each short option to occupy its own argv element, as evidenced by the use of an argument pointer during short-option lookup.

The Rust version must preserve the behavior of short option lookup and consumption, including:

- matching a short option definition against the current short option character
- tracking any remaining unconsumed text in the same argument token
- determining whether an argument is expected for the matched option

### 3. Long option recognition

The module recognizes long-form options and resolves them against defined options. Long-option processing also determines whether an argument is attached within the same token or expected from a following token.

The Rust version must preserve:

- matching a long option token against available option definitions
- identifying the originating option definition when aliases or transformed matches are involved
- determining whether the long option supplies or expects an argument

### 4. Negative-form long-option matching

The module contains dedicated logic for negative matching against long options (`negmatch`). This indicates that some long-option definitions may be recognized in a negated textual form according to option metadata.

The Rust version must implement the same observable negative-match behavior evidenced by the source:

- evaluate whether the current long-option token is a negated form of a defined option
- distinguish no-match from the recognized negative-match cases used by long-option lookup
- feed the result into long-option resolution consistently

### 5. Non-option handling and permutation

The module supports handling of non-option arguments during scanning and includes an explicit permutation step. This indicates that, in at least one scan mode, non-option arguments may be rearranged relative to options so parsing can continue over later options.

The Rust version must preserve the module’s observable non-option behavior:

- identify when the current argv element is not an option
- support parser advancement past non-options where allowed
- perform argument permutation in the cases required by the parser state and scan flags
- maintain a correct final argv/argc view after permutation

### 6. Lookahead and skip

The module exposes parser lookahead and skip helpers.

The Rust version must preserve:

- lookahead returning the next argument text that would be considered by parsing without consuming it
- skip advancing parser state past the current argument position without parsing it as an option

### 7. Public parse step wrapper

The module contains an internal parse routine and a public wrapper. The wrapper exposes the incremental parse result and any returned option argument.

The Rust version must preserve the externally observable behavior of the public parse step, including delegation to the internal parser logic and propagation of parse results and returned argument values.

### 8. Current argv exposure

The module exposes the parser’s current `argc`/`argv` after internal scanning and any reordering.

The Rust version must provide equivalent access to the parser’s current remaining argument view so callers can observe the same effective argument ordering and count as in the C module.

### 9. Error reporting through parser context

The module emits parse-related errors through a parser-associated error routine.

The Rust version must preserve:

- formatting and routing of parser errors through the parser context rather than inventing a separate reporting channel
- use of the provided priority/input fields represented by the parser API in this module

### 10. Option-definition collection and preparation

Before parsing, the module prepares option definitions. The preparation logic includes collecting option definitions, assigning option-index slots, deriving scan flags, and attaching built-in setters for help and usage entries present in this file.

The Rust version must preserve the functional outcome of preparation:

- collect the option definitions needed by the parser
- assign stable index slots as required by parsing and option bookkeeping
- derive parser scan behavior from option metadata
- support the built-in help and usage setters defined in this module

## User Scenarios & Testing

### Scenario 1: Parse a sequence of standard options incrementally

A caller initializes parser state with an argument vector and prepared option definitions, then repeatedly requests the next parsed option.

Expected behavior:

- each recognized option is returned in order determined by the parser’s scan mode
- if an option has an argument, the returned argument value is provided
- parser state advances correctly across calls until parsing is complete

Test coverage:

- repeated calls over a mixed option list
- options both with and without arguments
- completion at end of input

### Scenario 2: Parse grouped or attached short-option input

A caller supplies short options in a compact token form where more than one short option or an attached short-option argument may be present in a single argv element.

Expected behavior:

- the parser recognizes the next short option from within the current token
- remaining text in the token is either parsed as subsequent short options or used as the option’s attached argument, according to option definition rules
- no extra argv consumption occurs unless required

Test coverage:

- multiple short options in one token
- short option with attached argument
- short option requiring the next argv element as its argument

### Scenario 3: Parse long options with and without attached values

A caller supplies long options as standalone names or with attached values in the same token.

Expected behavior:

- long options are matched against defined option names
- attached values are returned as the option argument when applicable
- when no attached value is present and the option expects one, the parser obtains it from the following argv element if available

Test coverage:

- long option without argument
- long option with attached argument
- long option with separate following argument

### Scenario 4: Recognize negated long-option forms

A caller supplies a long option token in a negated form supported by the option definition set.

Expected behavior:

- the parser recognizes the negated form when it is valid for that option
- the same option definition resolution path is used as required by the module’s negative-match logic
- invalid negated forms are not silently accepted as normal matches

Test coverage:

- valid negated long option
- token similar to a negated form but not accepted by the option definition set

### Scenario 5: Handle non-option arguments during scanning

A caller supplies positional arguments interleaved with options.

Expected behavior:

- non-option arguments are either skipped, left in place, or permuted according to parser scan behavior prepared from option definitions
- subsequent options remain discoverable when the module’s scan mode allows it
- the final exported argv/argc view reflects the parser’s resulting ordering

Test coverage:

- leading positional argument before options
- positional argument between options
- verification of current argv/argc after parsing

### Scenario 6: Inspect next argument and skip it

A caller needs to inspect the next token before deciding whether to consume it through normal parsing.

Expected behavior:

- lookahead returns the next current token without consuming it
- skip advances past that token
- a later parse step starts from the token after the skipped one

Test coverage:

- lookahead before any parse step
- skip on an option token
- skip on a non-option token

### Scenario 7: Observe parse errors

A caller supplies invalid or incomplete option input.

Expected behavior:

- the parser reports the error through the parser error facility
- parse-step results remain consistent with the module’s failure handling
- missing required arguments and unknown options trigger error reporting

Test coverage:

- unknown short option
- unknown long option
- option missing required argument

### Scenario 8: Prepare option definitions including help and usage setters

A caller configures option definitions and invokes module preparation before parsing.

Expected behavior:

- option definitions are collected and indexed
- parser scan flags are derived from the option set
- help and usage options handled by this module invoke their dedicated setter behavior when matched

Test coverage:

- option set containing help entry
- option set containing usage entry
- preparation effects visible during later parsing

## Requirements

### Functional Requirements

#### FR-1: Parser lookahead
The module shall provide a lookahead operation that returns the current next argument token without consuming it, based on the parser’s current scan position.

Traceability: `parseopt_lookahead`

#### FR-2: Parser skip
The module shall provide a skip operation that advances the parser past the current argument token without parsing it as an option.

Traceability: `parseopt_skip`

#### FR-3: Incremental parse step
The module shall provide a parse-step operation that advances parsing by one logical step and may return an associated option argument.

Traceability: `parseopt_next_internal`, `parseopt_next`

#### FR-4: Short-option matching
The parse-step logic shall recognize short options from the current token and resolve them against option definitions.

Traceability: `option_find_short`, `parseopt_next_internal`

#### FR-5: Long-option matching
The parse-step logic shall recognize long options from the current token and resolve them against option definitions.

Traceability: `option_find_long`, `parseopt_next_internal`

#### FR-6: Option-argument handling
When a matched option expects or accepts an argument, the module shall determine whether the argument is attached to the same token or must be taken from subsequent input, and shall return the argument value accordingly.

Traceability: `option_find_short`, `option_find_long`, `parseopt_next_internal`

#### FR-7: Negative long-option matching
The module shall apply the dedicated negative-match rules during long-option resolution and distinguish recognized negative matches from non-matches.

Traceability: `negmatch`, `option_find_long`

#### FR-8: Non-option detection and scan-mode behavior
The parse-step logic shall distinguish non-option arguments from option tokens and apply the parser’s prepared scan behavior to them.

Traceability: `parseopt_next_internal`, `prepare_optdef`

#### FR-9: Argument permutation
When scan behavior requires it, the module shall reorder arguments so that option parsing can proceed while preserving a correct post-parse argv/argc view.

Traceability: `permute`, `parseopt_next_internal`, `parseopt_argv`

#### FR-10: Current argv exposure
The module shall expose the parser’s current effective `argc` and `argv` after any internal advancement or permutation.

Traceability: `parseopt_argv`

#### FR-11: Error reporting
The module shall report parse errors through the parser-associated error facility using the formatting input supplied by callers.

Traceability: `parseopt_error`

#### FR-12: Option-definition indexing
The module shall assign and retrieve option-index slots needed for prepared option definitions.

Traceability: `optidx_slot`

#### FR-13: Option-definition collection
The module shall collect relevant option definitions into the parser’s prepared option set.

Traceability: `collect_optdef`

#### FR-14: Option-definition preparation
The module shall prepare option definitions before parsing, including deriving scan-related flags from the option set.

Traceability: `prepare_optdef`

#### FR-15: Built-in help setter support
The module shall support the dedicated help-setting behavior defined for prepared option entries handled by this module.

Traceability: `set_help`, `prepare_optdef`

#### FR-16: Built-in usage setter support
The module shall support the dedicated usage-setting behavior defined for prepared option entries handled by this module.

Traceability: `set_usage`, `prepare_optdef`

### Key Entities

#### `parseopt`
Parser state for incremental command-line processing.

Observed responsibilities from this module:

- owns or references the current argument vector and count
- tracks current scan position
- carries prepared option-definition state
- carries scan behavior derived during preparation
- provides the context used by error reporting
- provides the mutable state needed for lookahead, skip, parsing, and permutation

Relationships:

- consumes a collection of `optdef` entries
- is mutated by parse-step, skip, permutation, and preparation operations
- is queried by lookahead and argv-exposure operations

Traceability: all listed parser functions operate on `struct parseopt`

#### `optdef`
Option definition record used to describe recognized options and their parse behavior.

Observed responsibilities from this module:

- describes short and/or long option identity used by lookup
- indicates whether and how an option takes an argument
- participates in negative-form long-option matching
- contributes to parser scan-flag derivation during preparation
- may occupy an option index slot
- may carry handler behavior used by built-in help and usage setters

Relationships:

- is collected and prepared into parser state
- is returned by short and long lookup logic
- may have an original-definition association during lookup resolution

Traceability: `option_find_short`, `negmatch`, `option_find_long`, `optidx_slot`, `collect_optdef`, `prepare_optdef`, `set_help`, `set_usage`

## Success Criteria

1. The Rust module can parse short options, long options, and their arguments incrementally with results matching the C module’s observable behavior for equivalent inputs.
   - Traceability: `option_find_short`, `option_find_long`, `parseopt_next_internal`, `parseopt_next`

2. The Rust module correctly handles attached and separate option arguments for both short and long options.
   - Traceability: `option_find_short`, `option_find_long`, `parseopt_next_internal`

3. The Rust module preserves negative long-option matching behavior implemented by the source module.
   - Traceability: `negmatch`, `option_find_long`

4. The Rust module supports lookahead and skip with correct parser-position effects.
   - Traceability: `parseopt_lookahead`, `parseopt_skip`

5. The Rust module preserves non-option handling and performs permutation when required by prepared scan behavior.
   - Traceability: `permute`, `parseopt_next_internal`, `prepare_optdef`

6. After parsing activity that advances or permutes input, the Rust module exposes an effective argv/argc view consistent with the source module.
   - Traceability: `parseopt_argv`

7. Unknown options and missing required arguments produce parser-context error reporting through the module’s error facility.
   - Traceability: `parseopt_error`, `parseopt_next_internal`

8. Option-definition preparation in Rust collects definitions, assigns option-index bookkeeping as required, and derives scan flags needed by later parsing.
   - Traceability: `optidx_slot`, `collect_optdef`, `prepare_optdef`

9. Built-in help and usage setter behaviors present in this module remain callable and functionally integrated into prepared option handling.
   - Traceability: `set_help`, `set_usage`, `prepare_optdef`

10. All required behaviors are implemented without adding unsupported externally visible capabilities beyond those evidenced by `src/parseopt/parseopt.c`.
   - Traceability: module scope and all functions above