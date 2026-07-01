# spec.md

## Title

Functional Specification: `module_src_parseopt_parseopt_02` Rust Port

## Metadata

- Project: `cflow-new`
- Module: `module_src_parseopt_parseopt_02`
- Category: `module_cluster`
- Source file: `src/parseopt/parseopt.c`
- Target Rust branch: `099-module_src_parseopt_parseopt_02-rust-port`
- Generation date: `2026-06-11`

## Overview

This module provides command-line option parsing behavior centered on a `parseopt` parser state and a set of option definitions (`optdef`). Its responsibilities are to:

- inspect upcoming arguments without consuming them,
- advance or skip argument positions,
- recognize short and long options,
- resolve option arguments,
- handle long-option negation matching,
- reorder arguments when permutation behavior is active,
- expose the remaining `argv` view after parsing progress,
- report parsing errors through the parser context,
- prepare and index option definitions for later parsing,
- support built-in help and usage option actions.

The Rust rewrite must preserve these behaviors at the functional level for the parser state, option matching, argument consumption, option-definition preparation, and parser-facing error reporting evidenced by the module functions.

## Scope

### In Scope

The Rust version must implement the functionality evidenced by these module behaviors:

- short-option lookup and resolution,
- long-option lookup and resolution, including negation matching,
- parser lookahead and skip behavior,
- iterative parsing of arguments through the parser state,
- exposure of the current remaining `argc`/`argv` view,
- argument permutation when configured by parser state,
- parser-mediated error emission,
- preparation and collection of option definitions for parsing,
- built-in handlers for help and usage options.

### Out of Scope

The Rust version must not claim or require capabilities not evidenced here, including:

- new option syntax beyond the recognized short/long/negated forms,
- new public APIs not required to cover the observed module behavior,
- thread-safety guarantees,
- serialization or persistence,
- FFI-facing behavior,
- benchmark or performance commitments beyond functional equivalence.

## Feature Specification

### 1. Parser-State-Driven Option Scanning

The module scans command-line arguments using a persistent parser state (`parseopt`). Parsing proceeds incrementally and may inspect, consume, or skip arguments based on current parser position and parser configuration.

The Rust version must support:

- reading the next argument candidate without advancing,
- advancing parsing state intentionally without interpreting the current item,
- repeatedly parsing until options are exhausted or parsing terminates,
- returning any associated option argument when a parsed option requires or accepts one.

Traceability:
- `parseopt_lookahead`
- `parseopt_skip`
- `parseopt_next_internal`
- `parseopt_next`
- `parseopt_argv`

### 2. Short Option Recognition

The module recognizes short options from parser input and resolves them against the available option definitions.

The Rust version must support:

- matching a short option token to its option definition,
- determining whether the matched option has an attached or separate argument,
- reporting both the matched option and the originating definition when aliasing or transformed lookup is involved,
- distinguishing whether an argument is expected for the matched short option.

Traceability:
- `option_find_short`
- `parseopt_next_internal`

### 3. Long Option Recognition

The module recognizes long options from parser input and resolves them against the available option definitions.

The Rust version must support:

- matching long option names against the option definition set,
- handling long options with explicit inline argument text when present,
- determining whether the option expects an argument,
- returning both the effective matched option and the originating definition when needed for parser behavior.

Traceability:
- `option_find_long`
- `parseopt_next_internal`

### 4. Long Option Negation Matching

The module contains explicit negation-matching behavior for long options.

The Rust version must preserve the observable behavior whereby:

- a long option candidate may be evaluated against an option definition in a negated form,
- the result of this comparison influences long-option resolution,
- the parser can distinguish whether a long-option token corresponds to a normal match, a negated match, or no negation-related match.

Traceability:
- `negmatch`
- `option_find_long`

### 5. Argument Permutation During Parsing

The module can reorder arguments through parser-controlled permutation behavior.

The Rust version must preserve the functional effect that:

- when parser configuration requires it, non-option and option arguments may be rearranged within the parser's working `argv` view,
- the resulting `argc`/`argv` state exposed after parsing reflects that permutation.

Traceability:
- `permute`
- `parseopt_next_internal`
- `parseopt_argv`

### 6. Remaining Argument View Exposure

The module can expose the parser's current position as an updated `argc`/`argv` pair.

The Rust version must support:

- returning the current remaining argument count,
- returning the current remaining argument vector view,
- ensuring this view reflects parsing progress and any permutation already applied.

Traceability:
- `parseopt_argv`

### 7. Parser Error Reporting

The module emits parsing-related error messages through the parser context.

The Rust version must preserve the capability for the parser to:

- emit formatted error text,
- associate an error priority/severity value with the emission,
- use parser context rather than direct return-only signaling as part of observable module behavior.

Traceability:
- `parseopt_error`

### 8. Option Definition Collection and Preparation

Before parsing, the module organizes option definitions and prepares parser scan behavior.

The Rust version must preserve the evidenced behavior that:

- option definitions can be collected from available definitions into parser-usable form,
- options can be assigned or resolved into index slots,
- parser scan flags/configuration can be derived from option definitions,
- built-in help and usage handlers can be attached or recognized during this preparation path.

Traceability:
- `optidx_slot`
- `collect_optdef`
- `prepare_optdef`
- `set_help`
- `set_usage`

## User Scenarios & Testing

### Scenario 1: Inspecting the Next Argument Before Parsing

A caller has initialized parser state and wants to know what raw argument is next before deciding whether to parse it or skip it.

Expected behavior:

- lookahead returns the current next argument text without advancing the parser,
- calling skip after lookahead advances past that argument,
- a later lookahead reflects the next item after the skipped one.

Traceability:
- `parseopt_lookahead`
- `parseopt_skip`

Test coverage should verify:

- repeated lookahead without skip does not advance,
- skip changes the remaining `argv` view,
- end-of-input behavior is handled consistently.

### Scenario 2: Parsing a Short Option Without a Separate Argument

A caller processes an option token that denotes a short option not requiring a value.

Expected behavior:

- the parser recognizes the short option,
- parsing advances past the option,
- no option argument is returned.

Traceability:
- `option_find_short`
- `parseopt_next_internal`
- `parseopt_next`

Test coverage should verify:

- a defined short option is recognized,
- returned argument output is empty or absent when no argument is expected,
- the parser moves to the next input item.

### Scenario 3: Parsing a Short Option With an Argument

A caller processes a short option that requires or expects an argument.

Expected behavior:

- the parser recognizes the short option,
- the associated argument is identified from the current token or subsequent parser input according to parser behavior,
- the returned option argument matches the consumed input.

Traceability:
- `option_find_short`
- `parseopt_next_internal`
- `parseopt_next`

Test coverage should verify:

- retrieval of an attached argument form if supported by the source behavior,
- retrieval of a separate following argument when required by the source behavior,
- parser state advances correctly after consumption.

### Scenario 4: Parsing a Long Option

A caller processes a token representing a long option.

Expected behavior:

- the parser matches the long option name to a definition,
- any explicit argument associated with the token is extracted when applicable,
- the parser returns the correct matched option and advances appropriately.

Traceability:
- `option_find_long`
- `parseopt_next_internal`
- `parseopt_next`

Test coverage should verify:

- exact long-name matching,
- handling of long options with and without arguments,
- correct parser advancement after successful parsing.

### Scenario 5: Parsing a Negated Long Option

A caller supplies a long option token in a form that triggers negation matching.

Expected behavior:

- the parser evaluates the token against negation rules for the option definition,
- the match outcome affects which option is considered resolved,
- parser-visible behavior is consistent with the source module's negation matching.

Traceability:
- `negmatch`
- `option_find_long`

Test coverage should verify:

- a token that should negate a matching option is recognized as such,
- a non-matching token is not incorrectly treated as negated,
- ambiguous or unmatched negation cases follow the source behavior.

### Scenario 6: Permuting Non-Option Arguments

A caller uses parser configuration under which options are parsed even when non-option arguments appear before later options.

Expected behavior:

- the parser permutes its working argument order as needed,
- later options remain discoverable,
- the remaining `argv` view reflects the permuted state.

Traceability:
- `permute`
- `parseopt_next_internal`
- `parseopt_argv`

Test coverage should verify:

- non-option arguments can be moved relative to options when enabled by parser state,
- parsing order of options matches the source behavior,
- final remaining argument layout reflects the applied permutation.

### Scenario 7: Obtaining Remaining Arguments After Partial Parsing

A caller parses some options, then requests the remaining `argc`/`argv`.

Expected behavior:

- the returned count and vector begin at the current parser position,
- already consumed items are excluded,
- any permutation already applied is visible in the remaining view.

Traceability:
- `parseopt_argv`

Test coverage should verify:

- correct remaining count after zero, partial, and full parsing,
- correct first remaining argument,
- consistency after skip and after option parsing.

### Scenario 8: Emitting a Parser Error

A parsing step detects an invalid option or argument condition and reports it through the parser.

Expected behavior:

- an error message is emitted through parser-owned reporting behavior,
- the supplied priority/severity is preserved,
- formatted message content incorporates caller-provided data.

Traceability:
- `parseopt_error`

Test coverage should verify:

- formatted error emission occurs,
- severity/priority reaches the reporting sink unchanged,
- parser context is used for emission.

### Scenario 9: Preparing Option Definitions Before Parsing

A caller constructs or supplies option definitions and prepares the parser before scanning command-line input.

Expected behavior:

- option definitions are collected into parser-usable form,
- scan-related flags are derived,
- option indexing is prepared consistently,
- built-in help and usage actions are recognized where defined.

Traceability:
- `optidx_slot`
- `collect_optdef`
- `prepare_optdef`
- `set_help`
- `set_usage`

Test coverage should verify:

- prepared definitions are available for subsequent short and long lookup,
- scan flags reflect the supplied definitions,
- help/usage definitions invoke the corresponding built-in action behavior.

## Requirements

### Functional Requirements

#### FR-1: Incremental parser inspection and advancement
The module shall provide parser-state operations to inspect the next argument without consuming it and to skip the current argument explicitly.

Traceability:
- `parseopt_lookahead`
- `parseopt_skip`

#### FR-2: Iterative option parsing
The module shall parse command-line input incrementally through parser state and return the next recognized option result together with any associated argument output.

Traceability:
- `parseopt_next_internal`
- `parseopt_next`

#### FR-3: Short option matching
The module shall match short option input against available option definitions and determine whether the matched option has or consumes an argument.

Traceability:
- `option_find_short`
- `parseopt_next_internal`

#### FR-4: Long option matching
The module shall match long option input against available option definitions and determine whether the matched option has or consumes an argument.

Traceability:
- `option_find_long`
- `parseopt_next_internal`

#### FR-5: Long-option negation handling
The module shall evaluate long option tokens for negation-related matching and incorporate that result into long option resolution.

Traceability:
- `negmatch`
- `option_find_long`

#### FR-6: Parser-controlled argument permutation
The module shall support permutation of the parser's working argument sequence when parser behavior requires reordering to continue option parsing.

Traceability:
- `permute`
- `parseopt_next_internal`

#### FR-7: Remaining argument export
The module shall expose the current remaining argument count and vector as seen from the parser's current position.

Traceability:
- `parseopt_argv`

#### FR-8: Parser-context error reporting
The module shall emit formatted parsing errors through the parser context, including the provided priority/severity value.

Traceability:
- `parseopt_error`

#### FR-9: Option definition collection
The module shall collect available option definitions into parser-usable form before or during parser preparation.

Traceability:
- `collect_optdef`

#### FR-10: Option index slot resolution
The module shall resolve or assign option-definition index slots used by parser preparation and lookup behavior.

Traceability:
- `optidx_slot`

#### FR-11: Scan-flag and option-definition preparation
The module shall prepare option definitions and derive parser scan-related flags/configuration needed for parsing behavior.

Traceability:
- `prepare_optdef`

#### FR-12: Built-in help action support
The module shall support a built-in action associated with help option definitions.

Traceability:
- `set_help`
- `prepare_optdef`

#### FR-13: Built-in usage action support
The module shall support a built-in action associated with usage option definitions.

Traceability:
- `set_usage`
- `prepare_optdef`

### Key Entities

#### `parseopt`
Parser state entity representing the current command-line parsing session.

Observed responsibilities:

- tracks current argument scanning position,
- provides access to the next argument for lookahead,
- advances or skips parser position,
- owns or references the working `argv` view,
- participates in argument permutation,
- serves as the context for error reporting,
- holds prepared option-definition and scan-configuration state.

Relationships:

- uses one or more `optdef` entries for option matching,
- is mutated by parsing, skipping, permutation, and preparation operations,
- provides the remaining `argc`/`argv` view to callers.

Traceability:
- `parseopt_lookahead`
- `parseopt_skip`
- `parseopt_next_internal`
- `parseopt_next`
- `parseopt_argv`
- `parseopt_error`
- `optidx_slot`
- `collect_optdef`
- `prepare_optdef`

#### `optdef`
Option definition entity representing one parseable option.

Observed responsibilities:

- identifies a short and/or long option form,
- carries information used to determine whether an argument is expected,
- participates in long-option negation matching,
- can be collected and prepared for parser lookup,
- can be associated with built-in help or usage action behavior.

Relationships:

- referenced by the parser during short and long matching,
- organized into parser-usable indexing/slot structures,
- may be returned as matched or originating definitions during lookup flows.

Traceability:
- `option_find_short`
- `negmatch`
- `option_find_long`
- `optidx_slot`
- `collect_optdef`
- `prepare_optdef`
- `set_help`
- `set_usage`

## Success Criteria

### SC-1: Lookahead and skip equivalence
Given a parser with remaining input, repeated lookahead calls shall return the same next argument until skip or parsing advancement occurs; after skip, the remaining view shall begin at the following argument.

Traceability:
- `parseopt_lookahead`
- `parseopt_skip`
- `parseopt_argv`

### SC-2: Short option resolution correctness
For every prepared short option definition used in tests, the parser shall resolve the corresponding short option token to that definition and correctly indicate whether an option argument was consumed.

Traceability:
- `option_find_short`
- `parseopt_next_internal`
- `prepare_optdef`

### SC-3: Long option resolution correctness
For every prepared long option definition used in tests, the parser shall resolve the corresponding long option token to that definition and correctly return any associated argument when present.

Traceability:
- `option_find_long`
- `parseopt_next_internal`
- `prepare_optdef`

### SC-4: Negated long-option behavior preservation
Test cases covering long-option negation shall produce the same match classification and resulting parser behavior as the source module for matching, negated, and non-matching inputs.

Traceability:
- `negmatch`
- `option_find_long`

### SC-5: Parsing progression correctness
Successive calls to the parser's next-option operation shall monotonically advance parser state until no further parseable options remain or termination conditions are reached.

Traceability:
- `parseopt_next_internal`
- `parseopt_next`

### SC-6: Remaining argument view correctness
After any tested combination of lookahead, skip, successful parsing, and permutation, the exported remaining `argc`/`argv` view shall match the parser's actual current state.

Traceability:
- `parseopt_skip`
- `parseopt_next_internal`
- `parseopt_argv`
- `permute`

### SC-7: Permutation behavior preservation
In scenarios where parser-controlled permutation is active, option parsing results and the final remaining argument layout shall match the source module's behavior for the same input.

Traceability:
- `permute`
- `parseopt_next_internal`
- `parseopt_argv`

### SC-8: Error reporting fidelity
When parser errors are triggered in test scenarios, emitted messages shall preserve the provided formatting arguments and priority/severity value.

Traceability:
- `parseopt_error`

### SC-9: Option-definition preparation usability
After option-definition preparation, all definitions used in parsing tests shall be discoverable through the parser's short or long matching paths, and preparation-derived scan behavior shall be effective during parsing.

Traceability:
- `collect_optdef`
- `optidx_slot`
- `prepare_optdef`

### SC-10: Built-in help and usage action availability
When option definitions corresponding to built-in help or usage behavior are prepared and invoked in tests, the parser shall execute the associated built-in action path.

Traceability:
- `set_help`
- `set_usage`
- `prepare_optdef`