# spec.md

## Title

Functional Specification for Rust Port of `src/wordsplit/wordsplit.c`

## Document Control

- Project: `cflow-new`
- Module: `module_src_wordsplit_wordsplit.c_08`
- Category: `module_cluster`
- Source file: `src/wordsplit/wordsplit.c`
- Target branch: `119-module_src_wordsplit_wordsplit.c_08-rust-port`
- Generation date: `2026-06-17`

## Overview

This module provides core word-splitting support centered on a mutable `wordsplit` state object and per-segment node records. The evidenced functionality includes:

- accumulation of split results into an existing split state;
- recognition and balancing of delimited constructs such as parenthesized regions;
- recognition helpers for variable and command substitutions;
- detection of glob-like patterns;
- skipping over sed-style substitution expressions while scanning command text;
- conversion of bounded digit sequences to numeric values;
- quoting and unquoting of characters via translation tables;
- internal formatting of error messages and internal rendering of node flags for diagnostics.

The Rust rewrite must preserve the same functional boundaries and observable behavior of these responsibilities as exercised through the module’s state and helper operations. The rewrite must not add new user-visible capabilities beyond the evidenced scope.

## Feature Specification

### Feature 1: Split-State Result Accumulation

The module must support appending additional argument strings into an existing word-splitting result state.

Traceability:
- `wordsplit_append`
- `struct wordsplit`

Behavior:
- Accept an existing split state plus an argument count and argument vector.
- Extend the state’s accumulated result set with the supplied arguments.
- Report success or failure through the function result.
- Preserve previously accumulated entries while adding new ones in order.

### Feature 2: Delimiter and Parenthesis Balance Scanning

The module must support scanning text to find the closing delimiter corresponding to an opening parenthesized construct.

Traceability:
- `find_closing_paren`
- referenced `balance_state`

Behavior:
- Scan within a bounded source range.
- Track balanced nested delimiters sufficiently to locate the matching close delimiter.
- Return whether a matching close delimiter was found.
- Report the located offset when successful.
- Respect the delimiter pair supplied by the caller.

### Feature 3: Shell-Like Expansion Start Recognition

The module must recognize whether a character can begin a variable expansion or a command substitution.

Traceability:
- `begin_var_p`
- `begin_cmd_p`

Behavior:
- Distinguish characters that start variable-oriented constructs.
- Distinguish characters that start command-oriented constructs.
- Provide these decisions as boolean-like predicate results for scanner logic.

### Feature 4: Glob Pattern Detection

The module must detect whether a bounded character sequence contains glob-style pattern syntax.

Traceability:
- `isglob`

Behavior:
- Inspect a string segment of known length.
- Return whether the segment should be treated as containing glob metacharacter usage.

### Feature 5: Sed Expression Skipping During Scans

The module must support advancing over a sed-style expression embedded in command text.

Traceability:
- `skip_sed_expr`

Behavior:
- Start from a caller-provided position within a bounded command string.
- Recognize the extent of the sed expression from that position.
- Return the position reached after the expression, or an error indication when the expression is not valid enough to skip.

### Feature 6: Bounded Numeric Conversion

The module must convert a fixed-count substring of digits in a given base to an integer.

Traceability:
- `xtonum`

Behavior:
- Read at most the caller-specified number of source characters.
- Interpret them in the caller-specified numeric base.
- Store the parsed value through an output parameter on success.
- Return success or failure.

### Feature 7: Character Quote/Unquote Translation

The module must map characters through quote and unquote translation tables.

Traceability:
- `wsplt_unquote_char`
- `wsplt_quote_char`

Behavior:
- Given a translation table and an input character, return the translated quoted or unquoted form.
- Support scanner and renderer logic that needs reversible quote handling for individual characters.

### Feature 8: Internal Error and Diagnostic Text Support

The module must support internal formatting of error text and internal rendering of node flags for diagnostics.

Traceability:
- `_wsplt_error`
- `wsnode_flagstr`
- `struct wordsplit_node`

Behavior:
- Format internal error messages from printf-style input.
- Produce a textual representation of node flags suitable for diagnostics or tracing.
- Keep these as internal support capabilities; no broader logging subsystem is required by this specification.

## User Scenarios & Testing

### Scenario 1: Append Additional Split Results

A caller already has a populated split state and receives more argument strings that must be added.

Expected support:
- The caller appends `argc/argv` data to the existing state.
- Existing entries remain intact.
- New entries appear after prior entries in the same order as provided.
- Failure is indicated if the append operation cannot complete.

Relevant traceability:
- `wordsplit_append`
- `struct wordsplit`

Suggested tests:
- Append to an empty state.
- Append to a non-empty state.
- Append multiple arguments and verify ordering.
- Verify that a failure path does not report success.

### Scenario 2: Find a Matching Closing Parenthesis in Nested Text

A scanner encounters an opening delimited construct and must find its matching close character within a bounded string.

Expected support:
- Nested matching constructs are handled correctly.
- The returned offset identifies the matching closing position.
- If no closing delimiter exists in range, the function reports failure.

Relevant traceability:
- `find_closing_paren`
- referenced `balance_state`

Suggested tests:
- Simple non-nested pair.
- Nested pairs.
- Different delimiter pairs as allowed by the function input.
- Unterminated input.

### Scenario 3: Detect Expansion Starters While Parsing

A parser examines characters one by one and needs to know whether current input can begin a variable expansion or command substitution.

Expected support:
- Characters appropriate to variable-start recognition return true from the variable predicate.
- Characters appropriate to command-start recognition return true from the command predicate.
- Non-starter characters return false.

Relevant traceability:
- `begin_var_p`
- `begin_cmd_p`

Suggested tests:
- Known valid starters for each predicate.
- Characters valid for one predicate but not the other.
- Ordinary non-special characters.

### Scenario 4: Identify Glob Patterns

A caller needs to decide whether a token contains glob syntax before later processing.

Expected support:
- The detector returns true for strings containing glob metacharacter patterns.
- It returns false for plain text without glob syntax.

Relevant traceability:
- `isglob`

Suggested tests:
- Plain literal string.
- Strings containing common glob metacharacters.
- Empty and bounded-length edge cases.

### Scenario 5: Skip a Sed Expression During Command Scan

A command scanner reaches what appears to be a sed expression and must skip over it safely.

Expected support:
- The returned index advances past the complete expression when valid.
- Malformed or incomplete expressions do not produce a false successful skip.

Relevant traceability:
- `skip_sed_expr`

Suggested tests:
- Valid sed substitution expression.
- Expressions with escaped separators.
- Incomplete expression.
- Expression beginning at different offsets within a larger command string.

### Scenario 6: Parse a Fixed-Width Numeric Escape or Count

A parser needs to decode a fixed number of digits from text in a specific base.

Expected support:
- Valid input yields a parsed integer and success result.
- Invalid digits or insufficient parseable content yield failure.

Relevant traceability:
- `xtonum`

Suggested tests:
- Valid decimal input with exact count.
- Valid octal or hexadecimal input as allowed by base argument.
- Invalid digit for base.
- Short or malformed source segment.

### Scenario 7: Quote and Unquote Individual Characters

A scanner or emitter must translate characters using the module’s quote mapping rules.

Expected support:
- A quoted character maps back through unquote behavior consistently where the table defines such mapping.
- Characters not requiring mapping are handled according to existing table-driven behavior.

Relevant traceability:
- `wsplt_unquote_char`
- `wsplt_quote_char`

Suggested tests:
- Character present in translation table.
- Character absent from translation table.
- Round-trip checks where applicable.

### Scenario 8: Produce Internal Diagnostic Text

Internal module logic needs formatted error text and readable flag names for node diagnostics.

Expected support:
- Error formatting accepts printf-style arguments.
- Node flags can be rendered to stable textual descriptions for debugging-oriented output.

Relevant traceability:
- `_wsplt_error`
- `wsnode_flagstr`
- `struct wordsplit_node`

Suggested tests:
- Error formatting with literal and substituted values.
- Known node flag combinations produce non-empty descriptive text.

## Requirements

### Functional Requirements

#### FR-1: Mutable Split State
The Rust module shall provide a `wordsplit` state model that can hold accumulated split results and support extension of that state by appending argument vectors.

Traceability:
- `struct wordsplit`
- `wordsplit_append`

#### FR-2: Ordered Append Semantics
The append operation shall preserve prior contents of the split state and add newly supplied arguments in input order.

Traceability:
- `wordsplit_append`

#### FR-3: Append Result Signaling
The append operation shall return a success/failure status equivalent in role to the C function result.

Traceability:
- `wordsplit_append`

#### FR-4: Matching Delimiter Search
The module shall provide bounded scanning for a matching closing delimiter corresponding to an opening parenthesized construct.

Traceability:
- `find_closing_paren`

#### FR-5: Nested Balance Handling
Matching-delimiter scanning shall account for nested balanced constructs as needed to identify the correct closing offset.

Traceability:
- `find_closing_paren`
- referenced `balance_state`

#### FR-6: Variable-Start Predicate
The module shall provide a predicate that determines whether a character can begin a variable construct.

Traceability:
- `begin_var_p`

#### FR-7: Command-Start Predicate
The module shall provide a predicate that determines whether a character can begin a command substitution construct.

Traceability:
- `begin_cmd_p`

#### FR-8: Glob Detection
The module shall provide a function that determines whether a bounded string segment contains glob-style pattern syntax.

Traceability:
- `isglob`

#### FR-9: Sed Expression Skip Support
The module shall provide a function that advances over a sed-style expression from a specified offset within bounded command text.

Traceability:
- `skip_sed_expr`

#### FR-10: Fixed-Count Base Conversion
The module shall provide conversion of a bounded character sequence to an integer using a caller-specified base and character count.

Traceability:
- `xtonum`

#### FR-11: Quote Translation
The module shall provide character-level translation from unquoted form to quoted form using a caller-supplied translation table.

Traceability:
- `wsplt_quote_char`

#### FR-12: Unquote Translation
The module shall provide character-level translation from quoted form to unquoted form using a caller-supplied translation table.

Traceability:
- `wsplt_unquote_char`

#### FR-13: Internal Error Formatting
The module shall support internal construction of formatted error messages from printf-style format input.

Traceability:
- `_wsplt_error`

#### FR-14: Node Flag Text Rendering
The module shall support rendering node flag values as descriptive text for internal diagnostics.

Traceability:
- `wsnode_flagstr`
- `struct wordsplit_node`

### Key Entities

#### `wordsplit`
Core mutable module state for word-splitting operations.

Evidenced role:
- Receives appended argument data.
- Participates broadly in module state management.

Traceability:
- `struct wordsplit`
- `wordsplit_append`

Relationship notes:
- Owns or references accumulated split results.
- Interacts with node records and diagnostic/error support.

#### `wordsplit_node`
Per-node or per-segment record used by the module while representing parsed or split content.

Evidenced role:
- Carries flags that can be rendered as text by internal diagnostics.

Traceability:
- `struct wordsplit_node`
- `wsnode_flagstr`

Relationship notes:
- Associated with `wordsplit` processing state.
- Supplies flag state for internal reporting.

#### Balance state
Auxiliary balancing concept used while scanning nested delimited constructs.

Evidenced role:
- Supports matching-close search behavior.

Traceability:
- referenced `balance_state`
- `find_closing_paren`

Relationship notes:
- Used transiently during delimiter scanning rather than as a top-level result object.

## Success Criteria

### SC-1: Append Behavior Preservation
Given an existing split state and an input argument vector, the Rust port appends all supplied arguments without losing existing entries and preserves input order.

Traceability:
- `wordsplit_append`
- `struct wordsplit`

### SC-2: Append Status Equivalence
The Rust port exposes success/failure signaling for append operations that distinguishes successful extension from failure conditions.

Traceability:
- `wordsplit_append`

### SC-3: Correct Closing-Delimiter Resolution
For bounded input containing balanced nested delimiters, the Rust port returns the offset of the correct matching closing delimiter; for unterminated input, it reports failure.

Traceability:
- `find_closing_paren`
- referenced `balance_state`

### SC-4: Predicate Correctness
For characters used by the original parser logic, the Rust port’s variable-start and command-start predicates return results consistent with the C module behavior.

Traceability:
- `begin_var_p`
- `begin_cmd_p`

### SC-5: Glob Detection Correctness
For representative literal and glob-containing tokens, the Rust port correctly distinguishes glob-like input from non-glob input.

Traceability:
- `isglob`

### SC-6: Sed Skip Correctness
For valid sed-style expressions, the Rust port advances to the correct end position; for malformed or incomplete expressions, it does not report a false successful skip.

Traceability:
- `skip_sed_expr`

### SC-7: Numeric Conversion Correctness
For valid bounded numeric substrings in the requested base, the Rust port yields the expected integer value; for invalid input, it reports failure.

Traceability:
- `xtonum`

### SC-8: Quote Translation Correctness
For characters covered by the translation table, the Rust port applies quote and unquote mappings consistent with the C module behavior.

Traceability:
- `wsplt_quote_char`
- `wsplt_unquote_char`

### SC-9: Diagnostic Support Availability
The Rust port retains internal support for formatted error text and node-flag textual rendering sufficient to support module diagnostics.

Traceability:
- `_wsplt_error`
- `wsnode_flagstr`
- `struct wordsplit_node`