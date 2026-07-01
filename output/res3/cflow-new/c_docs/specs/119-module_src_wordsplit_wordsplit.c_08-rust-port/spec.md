# spec.md

## Title

Rust Functional Specification for `module_src_wordsplit_wordsplit.c_08`

## Status

Draft

## Scope

This specification defines the functional behavior that must be preserved when rewriting the `src/wordsplit/wordsplit.c` module of **cflow-new** into Rust on branch `119-module_src_wordsplit_wordsplit.c_08-rust-port`.

The covered module is responsible for a portion of shell-style word splitting support centered on:

- maintaining and extending split results,
- identifying balanced and delimited subexpressions,
- recognizing variable and command substitution starts,
- detecting glob patterns,
- skipping over `sed`-style expressions during parsing,
- converting short numeric escape fragments,
- quoting and unquoting characters through translation tables,
- reporting internal parsing or processing errors,
- rendering node flags into readable text for diagnostics.

This specification covers only behavior evidenced by the analyzed module members and types. It does not define new public capabilities beyond those.

## Feature Specification

### Summary

The Rust rewrite must provide the same functional boundaries as the analyzed C module segment:

1. **Result accumulation**
   - Append additional argument vectors into an existing word-splitting result structure.

2. **Delimited expression matching**
   - Find the matching closing parenthesis or paired delimiter from a given starting position while honoring nested structure rules implied by the source parser.

3. **Shell-token start recognition**
   - Recognize whether a character can begin a variable expansion.
   - Recognize whether a character can begin a command substitution.

4. **Pattern recognition**
   - Detect whether a string segment contains shell glob syntax.

5. **Embedded command parsing assistance**
   - Advance past a `sed` expression embedded in a command string so later parsing resumes at the correct position.

6. **Short numeric fragment decoding**
   - Convert a bounded-count numeric substring in a specified base into an integer result.

7. **Character quoting transforms**
   - Translate characters according to a quote/unquote mapping table.

8. **Diagnostics support**
   - Emit formatted internal error messages.
   - Produce a readable flag description for split nodes.

### In-Scope Behavior

The Rust version must preserve behavior for the module-level operations evidenced by these functions:

- `_wsplt_error`
- `wsnode_flagstr`
- `wordsplit_append`
- `find_closing_paren`
- `begin_var_p`
- `begin_cmd_p`
- `isglob`
- `skip_sed_expr`
- `xtonum`
- `wsplt_unquote_char`
- `wsplt_quote_char`

### Out of Scope

The following are not required unless already implied by the existing module interfaces and structures:

- new parsing modes,
- new public configuration interfaces,
- thread-safety guarantees,
- serialization or persistence,
- recovery workflows beyond existing return/error signaling,
- foreign-function interfaces,
- performance targets beyond functional equivalence.

## User Scenarios & Testing

### Scenario 1: Append newly produced words to an existing split result

A caller has an existing `wordsplit` result and receives another `argc`/`argv` list that must be merged into it.

**Expected behavior**
- The module appends all provided entries to the existing result in order.
- Existing entries remain unchanged.
- The result count reflects the combined number of entries.
- Failure is reported through the function return path used by the C module.

**Test focus**
- Append into an empty result.
- Append into a non-empty result.
- Append multiple strings and preserve order.
- Verify failure reporting on invalid inputs or allocation-related failure paths if represented by the API contract.

### Scenario 2: Locate a closing delimiter in a nested expression

A parser has identified an opening delimiter at a known offset and needs the position of the corresponding closing delimiter.

**Expected behavior**
- The module scans forward from the supplied index.
- It returns the matching closing position for the provided delimiter class.
- Nested paired delimiters are handled consistently with the original parser logic.
- If no valid closing delimiter exists before the end bound, the function reports failure.

**Test focus**
- Simple balanced form.
- Nested balanced forms.
- Missing closing delimiter.
- Alternate delimiter pairing indicated by the `paren` argument.

### Scenario 3: Detect the start of variable expansion

A parser examines a character after a shell expansion marker and must decide if variable-style parsing should begin.

**Expected behavior**
- Characters accepted by the original module as valid variable starters are accepted.
- Characters not accepted are rejected.

**Test focus**
- Typical identifier-start characters.
- Delimiter or punctuation characters that should not start variables.

### Scenario 4: Detect the start of command substitution

A parser examines a character sequence that may begin command substitution.

**Expected behavior**
- Characters accepted by the original module as valid command-substitution starters are accepted.
- Others are rejected.

**Test focus**
- Supported command-substitution opening forms.
- Near-miss characters that must not be treated as command substitution.

### Scenario 5: Recognize glob syntax within a token

A parser must decide whether a token should be treated as containing glob metacharacters.

**Expected behavior**
- Tokens containing glob syntax recognized by the module are reported as globbing patterns.
- Plain strings are not reported as globs.

**Test focus**
- Strings with wildcard characters.
- Strings with bracket expressions if supported by original logic.
- Plain literal strings.
- Boundary conditions using the supplied length parameter.

### Scenario 6: Skip over a `sed` expression embedded in a command

A parser processing a command line reaches a `sed` expression and must move past it without misinterpreting internal delimiters.

**Expected behavior**
- The module returns the next parsing position after the recognized `sed` expression.
- Expression delimiters internal to the `sed` syntax are handled consistently with the C behavior.
- If the expression is incomplete or malformed under the module rules, failure or non-advance behavior matches the original contract.

**Test focus**
- Simple substitution command.
- Expressions with escaped delimiters.
- Incomplete expressions.

### Scenario 7: Decode a bounded numeric escape fragment

A parser needs to convert a short substring in a known base into an integer value.

**Expected behavior**
- Up to the specified count of digits are consumed under the given base.
- The output integer is updated on success.
- Invalid digits or malformed input are rejected consistently with the original contract.

**Test focus**
- Valid octal/hex/decimal fragments.
- Input shorter than the requested count.
- Invalid digit in the middle of a fragment.
- Zero-length or otherwise invalid cases if accepted by the source contract.

### Scenario 8: Apply quote/unquote translation rules

A parser or formatter must map a character using a translation table for quoting or unquoting.

**Expected behavior**
- If the character has a defined mapping in the translation table, the mapped character is returned.
- Otherwise, fallback behavior matches the C module contract.

**Test focus**
- Character present in the mapping.
- Character absent from the mapping.
- Consistency between quote and unquote operations when using the same table.

### Scenario 9: Produce diagnostic text for node flags

Debugging or diagnostic output needs a readable representation of a node's flags.

**Expected behavior**
- The module returns a stable textual representation for the flag combinations handled by the source module.

**Test focus**
- Known individual flags.
- Combined flags if supported by original formatting behavior.
- Unknown or empty flag cases.

### Scenario 10: Emit formatted internal errors

An internal module path needs to report an error message.

**Expected behavior**
- A formatted message is produced consistently with the internal error facility behavior.
- Formatting placeholders are expanded correctly.

**Test focus**
- Plain string message.
- Formatted message with arguments.

## Requirements

### Functional Requirements

#### FR-1: Append split results
The Rust module shall support appending an `argc`/`argv` argument list to an existing `wordsplit` result structure, preserving argument order and updating the result state accordingly.

**Traceability:** `wordsplit_append`, `struct wordsplit`

#### FR-2: Match closing paired delimiters
The Rust module shall support locating the matching closing delimiter for a paired expression within a bounded input string region and shall report whether a valid match was found.

**Traceability:** `find_closing_paren`

#### FR-3: Identify variable-expansion starts
The Rust module shall support classifying whether a character is a valid start for variable-oriented parsing.

**Traceability:** `begin_var_p`

#### FR-4: Identify command-substitution starts
The Rust module shall support classifying whether a character is a valid start for command-substitution parsing.

**Traceability:** `begin_cmd_p`

#### FR-5: Detect glob patterns
The Rust module shall support determining whether a string segment contains glob syntax recognized by the original parser logic.

**Traceability:** `isglob`

#### FR-6: Skip `sed` expressions during command parsing
The Rust module shall support advancing across a `sed` expression embedded in a command string so subsequent parsing resumes at the correct offset.

**Traceability:** `skip_sed_expr`

#### FR-7: Decode short numeric substrings
The Rust module shall support converting a bounded-length numeric substring in a caller-specified base into an integer and shall report conversion success or failure.

**Traceability:** `xtonum`

#### FR-8: Translate characters for unquoting
The Rust module shall support mapping a character through an unquote translation table.

**Traceability:** `wsplt_unquote_char`

#### FR-9: Translate characters for quoting
The Rust module shall support mapping a character through a quote translation table.

**Traceability:** `wsplt_quote_char`

#### FR-10: Provide node-flag diagnostic text
The Rust module shall support producing a readable textual representation of `wordsplit_node` flags for diagnostics.

**Traceability:** `wsnode_flagstr`, `struct wordsplit_node`

#### FR-11: Provide internal formatted error reporting
The Rust module shall preserve the module’s ability to emit formatted internal error messages used by its parsing and processing logic.

**Traceability:** `_wsplt_error`

### Key Entities

#### `wordsplit`
Primary module state for word-splitting operations.

**Role evidenced by module usage**
- Holds accumulated split results.
- Participates in append operations.
- Appears broadly across parsing and processing paths in this source file.

**Required relationship**
- Owns or references the current list/vector of split words.
- Is associated with parsing state and configuration already present in the source module, to the extent needed by the covered functions.

**Traceability:** `struct wordsplit`, `wordsplit_append`

#### `wordsplit_node`
Represents an internal parsed node with flags used for diagnostics and parser state interpretation.

**Role evidenced by module usage**
- Carries flags that can be rendered as diagnostic text.

**Required relationship**
- Is associated with `wordsplit` processing internals.
- Exposes or retains flag state sufficient for `wsnode_flagstr`-equivalent behavior.

**Traceability:** `struct wordsplit_node`, `wsnode_flagstr`

#### Node flags
Bitwise or enumerated markers attached to a `wordsplit_node`.

**Role evidenced by module usage**
- Used to describe parser node state in human-readable form.

**Required relationship**
- Must remain representable in Rust so diagnostic text can be generated for the same meaningful cases.

**Traceability:** `wsnode_flagstr`, `struct wordsplit_node`

#### Delimiter/balance state
Internal state used while locating matching delimiters and traversing nested expressions.

**Role evidenced by module usage**
- Supports balanced scanning behavior.
- Includes reference to `balance_state` in the analyzed source.

**Required relationship**
- Must be representable sufficiently to preserve the matching behavior of paired-expression scanning.

**Traceability:** `find_closing_paren`, referenced type `balance_state`

#### Translation table
A character mapping table passed into quoting and unquoting helpers.

**Role evidenced by module usage**
- Defines how a character is transformed during quote/unquote operations.

**Required relationship**
- Must preserve the same input-to-output mapping semantics as the C helper functions.

**Traceability:** `wsplt_unquote_char`, `wsplt_quote_char`

## Success Criteria

### Behavioral Equivalence Criteria

1. **Append correctness**
   - Given an existing `wordsplit` result and an input `argc`/`argv`, the Rust implementation appends all arguments in the same order and yields the same resulting count and content as the C behavior for equivalent inputs.

   **Traceability:** `wordsplit_append`, `struct wordsplit`

2. **Delimiter matching correctness**
   - For balanced, nested, and unterminated test inputs, the Rust implementation returns match/failure results and offsets consistent with the C module logic.

   **Traceability:** `find_closing_paren`

3. **Variable-start classification correctness**
   - For a representative character matrix, the Rust implementation accepts and rejects the same characters as the C implementation for variable-start detection.

   **Traceability:** `begin_var_p`

4. **Command-start classification correctness**
   - For a representative character matrix, the Rust implementation accepts and rejects the same characters as the C implementation for command-substitution-start detection.

   **Traceability:** `begin_cmd_p`

5. **Glob detection correctness**
   - For tokens containing wildcard syntax and literal-only tokens, the Rust implementation reports glob presence identically to the C behavior.

   **Traceability:** `isglob`

6. **`sed` skip correctness**
   - For valid and malformed `sed` expression samples, the Rust implementation advances to the same next index or reports failure in the same cases as the C behavior.

   **Traceability:** `skip_sed_expr`

7. **Numeric conversion correctness**
   - For bounded substrings across supported bases, the Rust implementation returns the same success/failure outcome and integer value as the C helper.

   **Traceability:** `xtonum`

8. **Quote/unquote mapping correctness**
   - For characters present and absent in a translation table, the Rust implementation returns the same mapped or fallback values as the C helpers.

   **Traceability:** `wsplt_unquote_char`, `wsplt_quote_char`

9. **Diagnostic flag text equivalence**
   - For all flag combinations exercised by module tests, the Rust implementation produces the same or semantically equivalent readable flag descriptions as the C function.

   **Traceability:** `wsnode_flagstr`, `struct wordsplit_node`

10. **Internal error formatting preservation**
    - Formatted error messages produced through the Rust replacement preserve the same message content structure as the C module for equivalent format strings and arguments.

    **Traceability:** `_wsplt_error`

### Acceptance Criteria

- All functional requirements in this document are implemented without adding unsupported external capabilities.
- The Rust rewrite preserves the functional relationships between `wordsplit`, `wordsplit_node`, node flags, delimiter-matching state, and translation-table-based character transforms.
- Scenario-based tests covering the listed usage cases pass against the Rust implementation.
- Measured outputs for the covered helpers match the C module’s observable behavior for equivalent inputs.