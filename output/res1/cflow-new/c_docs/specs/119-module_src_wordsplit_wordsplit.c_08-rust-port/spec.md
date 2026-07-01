# spec.md

## Title

Rust Functional Specification for `module_src_wordsplit_wordsplit.c_08`

## Document Control

- Project: `cflow-new`
- Module: `module_src_wordsplit_wordsplit.c_08`
- Category: `module_cluster`
- Source file: `src/wordsplit/wordsplit.c`
- Target branch: `119-module_src_wordsplit_wordsplit.c_08-rust-port`
- Generation date: `2026-06-11`

## Overview

This module is part of the project’s word-splitting subsystem. The analyzed portion of `src/wordsplit/wordsplit.c` is responsible for support behavior used by shell-style word parsing and transformation, including:

- internal error reporting,
- node flag description for diagnostics,
- appending already-formed argument vectors into a wordsplit result,
- locating balanced closing delimiters,
- detecting starts of variable and command substitutions,
- identifying glob patterns,
- skipping over sed-style substitution expressions during scanning,
- converting fixed-width digit sequences to numeric values, and
- translating characters between quoted and unquoted forms.

The Rust rewrite must preserve the observable behavior of these responsibilities as used within the wordsplitting pipeline and associated diagnostics.

## Feature Specification

### 1. Internal diagnostic support

The module provides internal error formatting support for reporting wordsplit-related failures or invariant violations.

Rust implementation must:

- support formatted internal error emission equivalent in role to `_wsplt_error`,
- make diagnostic output suitable for debugging or failure reporting within this module,
- preserve the distinction between internal diagnostics and normal parsing results.

Traceability:
- `_wsplt_error` at `src/wordsplit/wordsplit.c:85-94`

### 2. Node flag description

The module can render a textual description of node flags associated with parsed wordsplit nodes.

Rust implementation must:

- produce a stable textual representation of node flag state for diagnostics,
- support combinations of flags used by `wordsplit_node`,
- preserve the diagnostic purpose of the feature rather than redefining flag semantics.

Traceability:
- `wsnode_flagstr` at `src/wordsplit/wordsplit.c:432-466`
- `struct wordsplit_node` at `src/wordsplit/wordsplit.c:416-430`

### 3. Appending argument vectors to an existing wordsplit result

The module can append a provided `argc`/`argv` sequence onto an existing `wordsplit_t` result object.

Rust implementation must:

- accept an existing wordsplit state and an externally supplied ordered list of argument strings,
- append those arguments in order to the current result set,
- update the receiving wordsplit state consistently so later consumers observe the extended result,
- return success or failure status corresponding to the append operation.

This feature is functional augmentation of an existing split result; it is not a new parse.

Traceability:
- `wordsplit_append` at `src/wordsplit/wordsplit.c:924-951`
- `struct wordsplit`

### 4. Balanced closing delimiter search

The module can scan text to find the matching closing delimiter for an opening construct, including parenthesis-like pairs supplied by the caller.

Rust implementation must:

- scan within a bounded substring using an input string, start offset, and total length,
- recognize delimiter pairing according to the supplied `paren` description,
- return whether a matching closing delimiter was found,
- if found, report the closing offset,
- correctly handle nested balanced constructs to the extent implied by the original function’s purpose.

Traceability:
- `find_closing_paren` at `src/wordsplit/wordsplit.c:988-1043`
- referenced type `balance_state`

### 5. Start-of-expansion detection

The module contains classification helpers for recognizing the start of variable substitution and command substitution constructs.

Rust implementation must:

- classify whether a character can begin a variable expansion,
- classify whether a character can begin a command substitution,
- preserve these helpers’ role as lexical predicates used by the parser/scanner.

Traceability:
- `begin_var_p` at `src/wordsplit/wordsplit.c:1725-1729`
- `begin_cmd_p` at `src/wordsplit/wordsplit.c:1822-1826`

### 6. Glob pattern detection

The module can determine whether a string segment contains shell glob syntax.

Rust implementation must:

- inspect a character sequence and length-bounded region,
- detect whether the input should be treated as containing glob metacharacter syntax,
- preserve its use as a yes/no classifier during word processing.

Traceability:
- `isglob` at `src/wordsplit/wordsplit.c:2057-2066`

### 7. Sed expression skipping during scanning

The module can advance over sed-style expressions embedded in command text so that later parsing logic does not misinterpret internal delimiters.

Rust implementation must:

- scan command text starting at a given offset,
- recognize the span of a sed-style expression,
- return the position reached after skipping the expression or a failure indication consistent with the C behavior,
- preserve bounded scanning by respecting the provided command length.

Traceability:
- `skip_sed_expr` at `src/wordsplit/wordsplit.c:2171-2202`

### 8. Fixed-width numeric conversion

The module can convert a bounded count of characters from a source string into a numeric value in a specified base.

Rust implementation must:

- parse exactly the relevant bounded portion of the source text,
- support caller-provided numeric base,
- write or return the parsed integer result,
- report success or failure if the source segment is not valid for the requested conversion.

Traceability:
- `xtonum` at `src/wordsplit/wordsplit.c:2376-2390`

### 9. Quote/unquote character translation

The module provides character-level translation helpers for applying and reversing quoting transformations using a translation table.

Rust implementation must:

- translate a character from quoted to unquoted form using a caller-supplied translation mapping,
- translate a character from unquoted to quoted form using a caller-supplied translation mapping,
- preserve one-character-at-a-time behavior,
- keep the transformation bounded to the supplied translation table semantics.

Traceability:
- `wsplt_unquote_char` at `src/wordsplit/wordsplit.c:2422-2432`
- `wsplt_quote_char` at `src/wordsplit/wordsplit.c:2434-2443`

## User Scenarios & Testing

### Scenario 1: Extend an existing split result with additional arguments

A caller already has a populated wordsplit result and needs to append another list of arguments gathered from elsewhere.

Expected support:

- the existing result remains valid,
- new arguments are appended in the same order as supplied,
- the total result reflects both prior and appended entries,
- an error status is returned if the append cannot be completed.

Primary traceability:
- `wordsplit_append`
- `struct wordsplit`

Suggested tests:

- append zero arguments to a non-empty result,
- append one argument to an empty result,
- append multiple arguments to a populated result and verify ordering,
- verify failure reporting when append cannot succeed.

### Scenario 2: Find the matching end of a balanced construct

A scanner encounters an opening parenthesis-like construct and must locate its matching closing delimiter within the current input.

Expected support:

- scanning begins from the caller-provided offset,
- nested balanced constructs do not cause premature termination,
- the closing offset is reported on success,
- the function reports failure when no matching close exists before the bounded end.

Primary traceability:
- `find_closing_paren`

Suggested tests:

- simple balanced pair with no nesting,
- nested balanced pairs,
- unmatched opening delimiter,
- bounded substring where a matching close exists outside the permitted range.

### Scenario 3: Detect whether input begins a variable or command expansion

Parsing logic needs a quick decision about whether an encountered character can start a variable reference or command substitution.

Expected support:

- characters valid for variable start are accepted only by the variable predicate,
- characters valid for command substitution start are accepted only by the command predicate where applicable,
- non-starter characters are rejected.

Primary traceability:
- `begin_var_p`
- `begin_cmd_p`

Suggested tests:

- representative valid variable-start characters,
- representative valid command-start characters,
- ordinary characters that should return false for both.

### Scenario 4: Decide whether a token contains glob syntax

Before deciding later expansion steps, the parser needs to know whether a token contains glob metacharacters.

Expected support:

- bounded scanning over the supplied token segment,
- positive detection for inputs containing glob syntax,
- negative detection for plain text inputs.

Primary traceability:
- `isglob`

Suggested tests:

- token with `*`,
- token with `?`,
- token with bracket-style glob syntax if recognized by the original logic,
- token with no glob metacharacters.

### Scenario 5: Skip over a sed substitution expression embedded in command text

While scanning a shell command, the parser encounters what should be treated as a sed expression and must advance past it without misparsing internal delimiters.

Expected support:

- the scanner recognizes the extent of the expression,
- delimiters internal to the expression are not mistaken for outer syntax,
- the returned index advances to the correct post-expression position or reports failure.

Primary traceability:
- `skip_sed_expr`

Suggested tests:

- valid sed substitution expression,
- expression using a non-default delimiter,
- malformed or truncated expression,
- command text with additional content following the expression.

### Scenario 6: Parse numeric escapes or bounded numeric fragments

A parser needs to interpret a fixed number of digits from text in a specified base.

Expected support:

- only the requested bounded character count is considered,
- valid input yields the expected integer,
- invalid input reports failure.

Primary traceability:
- `xtonum`

Suggested tests:

- valid octal fragment,
- valid hexadecimal fragment,
- invalid digit for the base,
- input shorter than required count if applicable to original behavior.

### Scenario 7: Apply and reverse quote translation for individual characters

Character processing needs to map characters through a quote translation table in both directions.

Expected support:

- quoting a translatable character yields the mapped form,
- unquoting the mapped character restores the original form,
- unmapped characters behave consistently with original semantics.

Primary traceability:
- `wsplt_quote_char`
- `wsplt_unquote_char`

Suggested tests:

- character present in translation table,
- character absent from translation table,
- round-trip quote then unquote on mapped values.

### Scenario 8: Produce diagnostics involving node flags

Diagnostic code needs a readable textual description of a node’s flags.

Expected support:

- a node with no relevant flags has a defined representation,
- a node with one or more flags produces a readable combined representation,
- output is stable enough for testing and troubleshooting.

Primary traceability:
- `wsnode_flagstr`
- `struct wordsplit_node`

Suggested tests:

- zero-flag node,
- single-flag node,
- multiple-flag node.

## Requirements

### Functional Requirements

#### FR-1: Internal error reporting
The Rust module shall provide internal formatted error reporting behavior corresponding to `_wsplt_error` for use by this module’s parsing and diagnostic paths.

Traceability:
- `_wsplt_error`

#### FR-2: Node flag stringification
The Rust module shall provide diagnostic stringification of `wordsplit_node` flag values corresponding to `wsnode_flagstr`.

Traceability:
- `wsnode_flagstr`
- `struct wordsplit_node`

#### FR-3: Append operation on wordsplit results
The Rust module shall support appending an ordered list of argument strings to an existing wordsplit result object, preserving input order and returning operation status.

Traceability:
- `wordsplit_append`
- `struct wordsplit`

#### FR-4: Balanced delimiter matching
The Rust module shall support bounded scanning for a matching closing delimiter associated with an opening balanced construct and shall report the matching offset on success.

Traceability:
- `find_closing_paren`

#### FR-5: Variable-start classification
The Rust module shall provide a predicate that determines whether a character can begin a variable expansion.

Traceability:
- `begin_var_p`

#### FR-6: Command-start classification
The Rust module shall provide a predicate that determines whether a character can begin a command substitution.

Traceability:
- `begin_cmd_p`

#### FR-7: Glob detection
The Rust module shall provide a bounded classifier that determines whether a string segment contains glob syntax.

Traceability:
- `isglob`

#### FR-8: Sed-expression skipping
The Rust module shall provide bounded scanning that skips over a sed-style expression within command text and returns the resulting scan position or failure.

Traceability:
- `skip_sed_expr`

#### FR-9: Bounded numeric conversion
The Rust module shall provide conversion of a fixed-width source fragment to an integer using a caller-specified base, with explicit success or failure reporting.

Traceability:
- `xtonum`

#### FR-10: Quote translation
The Rust module shall provide character-level translation from unquoted to quoted form using a caller-provided translation table.

Traceability:
- `wsplt_quote_char`

#### FR-11: Unquote translation
The Rust module shall provide character-level translation from quoted to unquoted form using a caller-provided translation table.

Traceability:
- `wsplt_unquote_char`

### Key Entities

#### `wordsplit`
Primary state object for wordsplitting operations and results. In this analyzed portion, it is the receiver for append behavior and the broader owner context for diagnostics and parsing support.

Relationships:
- receives appended argument vectors through `wordsplit_append`,
- associates with `wordsplit_node` instances used during parsing or diagnostics.

Traceability:
- `struct wordsplit`
- `wordsplit_append`

#### `wordsplit_node`
Represents a parsed or intermediate node in the wordsplitting process with associated flags.

Relationships:
- its flags are rendered by `wsnode_flagstr`,
- it exists under the broader wordsplit processing context.

Traceability:
- `struct wordsplit_node`
- `wsnode_flagstr`

#### Balanced scanning state
Auxiliary balancing context used while searching for matching delimiters.

Relationships:
- supports the logic of `find_closing_paren`.

Traceability:
- `find_closing_paren`
- referenced type `balance_state`

#### Translation table
Caller-supplied character mapping used for quote and unquote transformations.

Relationships:
- consumed by `wsplt_quote_char`,
- consumed by `wsplt_unquote_char`.

Traceability:
- `wsplt_quote_char`
- `wsplt_unquote_char`

## Success Criteria

1. The Rust module can append supplied argument lists to an existing wordsplit result while preserving argument order and reporting success or failure consistently with the C module.
   - Traceability: `wordsplit_append`, `struct wordsplit`

2. The Rust module correctly identifies matching closing delimiters for balanced constructs within bounded input and reports failure for unmatched cases.
   - Traceability: `find_closing_paren`

3. The Rust module provides character classification results for variable-start and command-start detection that match the original module’s accepted and rejected inputs.
   - Traceability: `begin_var_p`, `begin_cmd_p`

4. The Rust module correctly distinguishes glob-containing input segments from plain text segments for representative shell-style patterns handled by the C module.
   - Traceability: `isglob`

5. The Rust module can skip valid sed-style expressions and reject or fail on malformed bounded inputs in a manner compatible with the original scanning behavior.
   - Traceability: `skip_sed_expr`

6. The Rust module converts bounded numeric fragments for supported bases and rejects invalid fragments consistently with the original function contract.
   - Traceability: `xtonum`

7. The Rust module’s quote and unquote character translation functions preserve expected mapped behavior, including successful round-trip behavior for mapped characters.
   - Traceability: `wsplt_quote_char`, `wsplt_unquote_char`

8. The Rust module produces readable node flag descriptions suitable for diagnostics for zero, single, and combined flag states.
   - Traceability: `wsnode_flagstr`, `struct wordsplit_node`

9. The Rust module retains internal diagnostic capability corresponding to `_wsplt_error` so module failures can still be surfaced during development and testing.
   - Traceability: `_wsplt_error`