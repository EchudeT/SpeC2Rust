# spec.md

## Title

Functional Specification for `module_src_parseopt_parseopt_02` Rust Port

## Metadata

- Project: `cflow-new`
- Module: `module_src_parseopt_parseopt_02`
- Category: `module_cluster`
- Source file: `src/parseopt/parseopt.c`
- Rust branch: `099-module_src_parseopt_parseopt_02-rust-port`
- Generation date: `2026-06-17`

## Overview

This module provides command-line option parsing behavior around a `parseopt` parsing state and a set of option definitions (`optdef`). Its responsibilities are to identify short and long options, determine whether options consume arguments, handle long-option negation forms where applicable, advance through argument vectors, optionally reorder non-option arguments during parsing, expose the next unconsumed argument for lookahead, skip pending input, report parser errors, and prepare option-definition metadata used during parsing.

The Rust rewrite must preserve the observable parsing behavior supported by this source module, including:

- recognition of short options,
- recognition of long options,
- support for option arguments when expected,
- support for long-option negation matching as implemented by the source module,
- iteration over arguments through a parser state object,
- parser-state updates after consuming options or skipping input,
- access to the remaining argument vector after parsing progress,
- parser-driven error reporting,
- preparation and collection of option-definition data needed by parsing and built-in help/usage handling.

## Scope

In scope:

- Functional behavior represented by parsing-state operations and option matching in `src/parseopt/parseopt.c`.
- Behavior exposed through the module entry points:
  - `parseopt_lookahead`
  - `parseopt_skip`
  - `parseopt_next`
  - `parseopt_argv`
  - `parseopt_error`

Also in scope are internal behaviors that must be preserved because they determine observable results:

- short-option matching,
- long-option matching,
- long-option negation matching,
- option permutation during scanning,
- option-definition preparation and collection,
- built-in help/usage option setters.

Out of scope:

- Any new command-line syntax not evidenced by this module.
- Any new public API beyond what is required to preserve this module’s role.
- Concurrency, persistence, serialization, FFI, or performance guarantees not evidenced by the source.

## Feature Specification

### 1. Parser state driven option iteration

The module shall parse an argument vector through a mutable parser-state object. Each parsing step shall inspect the current scan position and determine whether the next item is:

- a short option,
- a long option,
- an option argument associated with a recognized option,
- a non-option argument,
- or an end-of-options condition.

The parser shall advance state consistently so repeated calls continue from the correct remaining position.

Traceability: `parseopt_next_internal`, `parseopt_next`, `parseopt_lookahead`, `parseopt_skip`, `parseopt_argv`, `struct parseopt`.

### 2. Short option recognition

The module shall match short options against the configured option definitions and identify the corresponding option definition when present. It shall also determine whether the matched short option expects an argument and whether that argument is available inline or from the following argv element according to the module’s current parsing rules.

Traceability: `option_find_short`, `parseopt_next_internal`, `struct optdef`.

### 3. Long option recognition

The module shall match long options against the configured option definitions and identify the corresponding option definition when present. It shall determine whether the long option expects an argument and whether an argument is supplied in the current token or must be obtained from the following argv element according to the module’s parsing rules.

Traceability: `option_find_long`, `parseopt_next_internal`, `struct optdef`.

### 4. Long-option negation matching

The module shall support the long-option negation matching behavior implemented by the source module. When a long option token is in a negated form recognized by the parser, matching shall resolve according to the option definition and negation decision rules used by the source.

This requirement is limited to preserving the source module’s existing behavior and does not imply support for any additional negation syntax.

Traceability: `negmatch`, `option_find_long`, `parseopt_next_internal`.

### 5. Option argument extraction

When a matched option requires or accepts an argument under the source module’s rules, the parser shall return the associated argument value to the caller through the option-iteration interface. When an expected argument is missing, the module shall follow the source module’s error path and state updates.

Traceability: `option_find_short`, `option_find_long`, `parseopt_next_internal`, `parseopt_next`, `parseopt_error`.

### 6. Non-option handling and permutation

The module shall handle non-option arguments encountered during scanning. Where the source parser mode requires argument permutation, the parser shall reorder argv entries so option scanning can continue while preserving the module’s resulting remaining-argv view.

Traceability: `permute`, `parseopt_next_internal`, `parseopt_argv`.

### 7. Lookahead and skip support

The module shall provide:

- a lookahead operation that returns the next unconsumed argument without consuming it,
- and a skip operation that advances past the current argument without parsing it as an option.

These operations must reflect the current parser state and interact correctly with subsequent parsing steps.

Traceability: `parseopt_lookahead`, `parseopt_skip`, `struct parseopt`.

### 8. Remaining argv exposure

The module shall expose the parser’s current remaining `argc/argv` view after scanning progress and any in-module permutation. This allows callers to access unconsumed arguments exactly as represented by the parser state.

Traceability: `parseopt_argv`, `permute`, `struct parseopt`.

### 9. Parser error reporting

The module shall report parsing errors through the parser-associated error-reporting path, accepting a severity/priority indicator and formatted message content. Rust behavior must preserve the module’s role of emitting parser-context error messages rather than silently ignoring detected parse errors.

Traceability: `parseopt_error`.

### 10. Option-definition preparation

Before or during parsing setup, the module shall prepare option definitions for efficient and correct matching. This includes collecting option definitions, assigning or resolving option index placement as needed by the parser, and updating parser scan flags derived from the option set.

Traceability: `optidx_slot`, `collect_optdef`, `prepare_optdef`, `struct parseopt`, `struct optdef`.

### 11. Built-in help and usage option setters

The module shall preserve the built-in setter behavior associated with help and usage options. When those options are parsed and dispatched through the option-definition mechanism, the module shall perform the corresponding parser-side action represented by the source setters.

Traceability: `set_help`, `set_usage`, `prepare_optdef`.

## User Scenarios & Testing

### Scenario 1: Iterate through recognized short options

A caller initializes a parser with argv and option definitions containing short options. Repeated calls to the parse-next interface return recognized options one by one, including any associated argument values, until no more options remain.

The Rust version must support tests that verify:

- recognized short options are returned in scan order,
- required option arguments are returned correctly,
- parser state advances after each call,
- lookahead reflects the next unconsumed token before and after each parse step.

Traceability: `option_find_short`, `parseopt_next`, `parseopt_lookahead`.

### Scenario 2: Iterate through recognized long options

A caller passes argv containing long-form options. The parser identifies the correct option definitions and returns associated arguments when present either inline or in the next token according to source behavior.

The Rust version must support tests that verify:

- exact long-option matches resolve correctly,
- argument-bearing long options return the intended value,
- parser state after consumption matches the expected remaining argv.

Traceability: `option_find_long`, `parseopt_next`, `parseopt_argv`.

### Scenario 3: Parse a negated long option form

A caller provides a long option token in a negated form that the source module recognizes. The parser resolves whether the token matches an option definition under the module’s negation rules.

The Rust version must support tests that verify:

- recognized negated forms match only when the source logic would match them,
- non-matching negated forms are not accepted as valid options,
- the parse result and state reflect the source behavior.

Traceability: `negmatch`, `option_find_long`.

### Scenario 4: Handle a missing required argument

A caller provides an option that requires an argument but does not supply one. The parser attempts to parse the option, detects the missing argument, and triggers the module’s error-reporting behavior.

The Rust version must support tests that verify:

- the missing-argument condition is detected,
- the parser reports an error through the module’s error path,
- parser state after the error remains consistent with the source module’s behavior.

Traceability: `option_find_short`, `option_find_long`, `parseopt_next_internal`, `parseopt_error`.

### Scenario 5: Intermix options and non-option arguments

A caller provides argv with both options and positional arguments. The parser processes this mixed input and, where required by source behavior, permutes argv entries so option scanning can continue.

The Rust version must support tests that verify:

- non-option items are handled without being misidentified as options,
- permutation occurs only when the parser mode requires it,
- the remaining argc/argv view returned by the module matches post-scan state.

Traceability: `permute`, `parseopt_next_internal`, `parseopt_argv`.

### Scenario 6: Use lookahead and skip to control scanning

A caller inspects the next token with lookahead, decides not to parse it as an option yet, and calls skip. Subsequent parsing begins at the following token.

The Rust version must support tests that verify:

- lookahead does not consume input,
- skip advances by exactly one pending argument,
- subsequent parse calls start from the correct location.

Traceability: `parseopt_lookahead`, `parseopt_skip`, `parseopt_next`.

### Scenario 7: Access remaining argv after partial parsing

A caller parses some options, then requests the current argc/argv remainder for downstream positional-argument processing.

The Rust version must support tests that verify:

- the returned argc/argv corresponds to the parser’s current state,
- any in-parser permutation is reflected in the returned view,
- unconsumed arguments remain in their parser-managed order.

Traceability: `parseopt_argv`, `permute`.

### Scenario 8: Built-in help and usage option dispatch

A caller includes option definitions that map help or usage handling through the built-in setter behavior. When those options are parsed, the module performs the expected setter action.

The Rust version must support tests that verify:

- help setter dispatch is reachable through option preparation and parsing,
- usage setter dispatch is reachable through option preparation and parsing,
- the effect matches the source module’s observable parser-side behavior.

Traceability: `prepare_optdef`, `set_help`, `set_usage`.

## Requirements

### Functional Requirements

#### FR-1: Parser state progression
The module shall maintain a mutable parsing state that supports repeated option-scanning calls and consistent advancement through argv.

Traceability: `parseopt_next_internal`, `parseopt_next`, `struct parseopt`.

#### FR-2: Short option matching
The module shall match short-form option tokens against configured option definitions and return the matched definition to the parsing flow.

Traceability: `option_find_short`, `struct optdef`.

#### FR-3: Long option matching
The module shall match long-form option tokens against configured option definitions and return the matched definition to the parsing flow.

Traceability: `option_find_long`, `struct optdef`.

#### FR-4: Negated long option support
The module shall implement the source module’s long-option negation matching behavior and apply it during long-option resolution.

Traceability: `negmatch`, `option_find_long`.

#### FR-5: Option argument determination
The module shall determine, for matched options, whether an argument is expected and how that argument is sourced from the current token or subsequent argv position according to source behavior.

Traceability: `option_find_short`, `option_find_long`, `parseopt_next_internal`.

#### FR-6: Parse-next result production
The module shall expose a parse-next operation that returns the next parsing outcome and an associated argument pointer/value when applicable.

Traceability: `parseopt_next_internal`, `parseopt_next`.

#### FR-7: Lookahead support
The module shall expose a lookahead operation that reports the next unconsumed argument without modifying parser state.

Traceability: `parseopt_lookahead`.

#### FR-8: Skip support
The module shall expose a skip operation that advances parser state past the current pending argument.

Traceability: `parseopt_skip`.

#### FR-9: Non-option handling
The module shall distinguish option tokens from non-option arguments during scanning and preserve source behavior for how non-options affect parsing progress.

Traceability: `parseopt_next_internal`.

#### FR-10: Argument permutation
The module shall reorder argv entries when required by the parser’s source-defined scanning mode so that option processing and remaining-argv reporting reflect the source behavior.

Traceability: `permute`, `parseopt_next_internal`, `parseopt_argv`.

#### FR-11: Remaining argv reporting
The module shall expose the current remaining argc/argv view from parser state after any parsing progress and parser-managed reordering.

Traceability: `parseopt_argv`.

#### FR-12: Error reporting
The module shall provide parser-context error reporting for detected parse failures, including formatted message emission through the parser-associated reporting path.

Traceability: `parseopt_error`.

#### FR-13: Option-definition collection
The module shall collect option definitions into the parser-managed set used for matching and indexing.

Traceability: `collect_optdef`.

#### FR-14: Option-index slot resolution
The module shall resolve option-index placement needed for parser-managed option-definition handling.

Traceability: `optidx_slot`.

#### FR-15: Option-definition preparation
The module shall prepare option definitions before parsing use, including updating parser scan flags derived from the option set.

Traceability: `prepare_optdef`.

#### FR-16: Built-in help setter behavior
The module shall preserve the built-in help-option setter behavior represented by the source module.

Traceability: `set_help`, `prepare_optdef`.

#### FR-17: Built-in usage setter behavior
The module shall preserve the built-in usage-option setter behavior represented by the source module.

Traceability: `set_usage`, `prepare_optdef`.

### Key Entities

#### `parseopt`
Parser state entity that owns or references the current argv scan position, current parsing mode, parser-managed option-definition set, and any state needed for lookahead, skipping, permutation, error reporting, and remaining-argv exposure.

Relationships:

- is consumed and updated by parse operations,
- is consulted by short and long option matching,
- is mutated by permutation,
- is the context for error reporting,
- is prepared with option-definition metadata before parsing.

Traceability: `struct parseopt`, all exported parser-state functions, `prepare_optdef`, `permute`.

#### `optdef`
Option-definition entity that describes the recognized options available to the parser, including identities needed for short and long matching, argument expectations, and special dispatch such as built-in help and usage handling.

Relationships:

- is searched by short-option and long-option lookup,
- is collected into parser-managed structures,
- is assigned index placement for parser use,
- is prepared before active parsing,
- may trigger built-in setter behavior when matched.

Traceability: `struct optdef`, `option_find_short`, `option_find_long`, `collect_optdef`, `optidx_slot`, `prepare_optdef`, `set_help`, `set_usage`.

## Success Criteria

1. The Rust module accepts a parser state and option-definition set and supports repeated parse-next calls that advance through argv without losing or duplicating tokens.
   - Traceability: `parseopt_next_internal`, `parseopt_next`, `struct parseopt`.

2. For test inputs containing valid short options, the Rust module matches the same options and exposes the same option arguments as the source behavior.
   - Traceability: `option_find_short`, `parseopt_next_internal`.

3. For test inputs containing valid long options, the Rust module matches the same options and exposes the same option arguments as the source behavior.
   - Traceability: `option_find_long`, `parseopt_next_internal`.

4. For test inputs containing supported negated long-option forms, the Rust module produces the same match or non-match decisions as the source behavior.
   - Traceability: `negmatch`, `option_find_long`.

5. For options missing required arguments, the Rust module triggers parser error reporting rather than silently treating the input as a valid complete option.
   - Traceability: `parseopt_next_internal`, `parseopt_error`.

6. Lookahead returns the next unconsumed argument without advancing parser state, and skip advances exactly one pending argument in tests covering mixed input.
   - Traceability: `parseopt_lookahead`, `parseopt_skip`.

7. In tests with intermixed options and non-options, the Rust module preserves source-consistent non-option handling and argv permutation outcomes.
   - Traceability: `permute`, `parseopt_next_internal`.

8. After partial parsing, the Rust module returns a remaining argc/argv view consistent with the parser state and any source-consistent permutation.
   - Traceability: `parseopt_argv`, `permute`.

9. Option-definition preparation in Rust produces parser behavior consistent with the source for the same option-definition set, including scan-flag-derived behavior.
   - Traceability: `collect_optdef`, `optidx_slot`, `prepare_optdef`.

10. Built-in help and usage option handling remain reachable and behave consistently with the source module when those prepared option definitions are parsed.
    - Traceability: `set_help`, `set_usage`, `prepare_optdef`.