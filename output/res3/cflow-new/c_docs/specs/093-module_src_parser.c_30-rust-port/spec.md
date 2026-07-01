# spec.md

## Title
Functional Specification for `module_src_parser.c_30` Rust Port

## Document Information
- Project: `cflow-new`
- Module: `module_src_parser.c_30`
- Category: `module_cluster`
- Source basis: `src/parser.c`
- Target branch: `093-module_src_parser.c_30-rust-port`
- Generation date: `2026-06-17`

## Overview
This module is responsible for parser-side handling of C declarations and related expression forms within the project’s source analysis flow. Its role is to consume parser tokens and classify declaration constructs, distinguishing functions from variables, handling initializer syntax, skipping balanced token regions when required, and producing saved token text for later use.

The Rust rewrite must preserve the observable behavior of this parsing layer as evidenced by `src/parser.c`, including:
- saving and later finalizing token sequences,
- skipping ahead in token streams to specific or balanced delimiters,
- determining whether a declaration shape represents a function,
- parsing declarations, function declarations, variable declarations, K&R-style declaration fragments, and initializer lists,
- and supporting the parser entry flow represented by `yyparse`.

This specification defines only the functionality evidenced by the analyzed module and does not introduce new APIs or capabilities.

## Scope
In scope:
- Declaration-oriented parsing behavior implemented in `src/parser.c`
- Token-save stack behavior used to accumulate declaration text
- Parser control behavior for expressions, initializers, and balanced token skipping
- Function/variable declaration differentiation
- Support for declaration forms that require placeholder or synthetic struct handling

Out of scope:
- Lexer implementation details not evidenced in this module
- Broader project graph-building behavior unless directly required by this module’s parsing outputs
- New parsing modes, recovery guarantees, or unsupported language extensions not evidenced here

## Feature Specification

### 1. Saved token sequence management
The module maintains a save stack for parser tokens. While parsing declarations or related constructs, tokens may be accumulated in order and later either discarded or finalized into a contiguous name/text result.

The Rust version must implement behavior equivalent to:
- appending tokens from parser state into a saved-token stack,
- undoing the active saved-token accumulation,
- finalizing the saved-token accumulation into a returned character sequence associated with a supplied name/input anchor.

This behavior is evidenced by:
- `save_token`
- `undo_save_stack`
- `finish_save_stack`

### 2. Parser-directed skipping
The module can advance parsing input until a target token is reached and can also skip nested balanced constructs based on opening token type and nesting level.

The Rust version must implement:
- skipping input until a specified token class/value is encountered,
- skipping over balanced nested regions such as parenthesized, bracketed, or similar delimiter-controlled forms as required by declaration and expression parsing.

This behavior is evidenced by:
- `skip_to`
- `skip_balanced`
- `expression`
- `initializer_list`

### 3. Declaration classification
The module determines whether a declaration currently being examined corresponds to a function form. This classification affects which parsing path is taken next.

The Rust version must preserve the decision behavior that:
- inspects declaration-related parser state,
- identifies function-shaped declarations,
- dispatches to function-specific or variable-specific declaration parsing accordingly.

This behavior is evidenced by:
- `is_function`
- `parse_declaration`
- `parse_function_declaration`
- `parse_variable_declaration`

### 4. Declaration parsing
The module parses declaration constructs associated with an identifier context and a parameter/declaration-mode flag. It supports general declaration dispatch, variable declarations, function declarations, declaration skipping, and K&R declaration handling.

The Rust version must implement equivalent behavior for:
- top-level or context-sensitive declaration parsing,
- skipping declaration content when required,
- function declaration parsing,
- variable declaration parsing,
- K&R-style declaration fragment handling where invoked by parser flow.

This behavior is evidenced by:
- `parse_declaration`
- `skip_declaration`
- `parse_function_declaration`
- `parse_variable_declaration`
- `parse_knr_dcl`

### 5. Expression and initializer handling within declarations
The module parses or skips expression-like content that appears in declaration contexts, especially variable initializers and initializer lists.

The Rust version must preserve behavior that:
- processes declaration-associated expressions sufficiently for parser progression,
- handles nested initializer-list syntax,
- correctly resumes declaration parsing after expression or initializer content.

This behavior is evidenced by:
- `expression`
- `initializer_list`
- `parse_variable_declaration`

### 6. Placeholder struct handling for declaration parsing
The module includes logic that creates or recognizes a synthetic/placeholder struct-related form to support declaration parsing when a concrete declarative structure is not otherwise available.

The Rust version must preserve the parser-visible effect of this behavior where applicable in declaration parsing.

This behavior is evidenced by:
- `fake_struct`

### 7. Parser entry integration
The module exposes the parser entry function used by the surrounding parsing workflow.

The Rust version must provide equivalent module behavior for:
- invoking and executing the parser entry path,
- coordinating the declaration-parsing routines described above within parser execution.

This behavior is evidenced by:
- `yyparse`

## User Scenarios & Testing

### Scenario 1: Saving declaration tokens for later finalization
A caller enters a declaration parsing path where token text must be preserved while the parser continues to inspect structure. Tokens are saved incrementally. If the parse path succeeds, the saved sequence is finalized into output text; if the path is abandoned, the saved sequence is undone.

The Rust version must support tests that verify:
- tokens can be saved in sequence,
- undo removes the current saved accumulation effect,
- finalize produces a character sequence from the saved tokens and supplied finalization input,
- saved token handling integrates correctly with declaration parsing.

Traceability:
- `save_token`
- `undo_save_stack`
- `finish_save_stack`

### Scenario 2: Skipping to a delimiter in malformed or unneeded declaration content
While parsing a declaration, the parser may need to ignore intermediate content until a specific token is reached, such as a declaration terminator or synchronization point.

The Rust version must support tests that verify:
- parsing advances until the requested token is found,
- skipped content does not cause premature declaration classification,
- the parser state after skipping allows subsequent parsing to continue.

Traceability:
- `skip_to`
- `skip_declaration`

### Scenario 3: Skipping balanced nested syntax
A declaration contains nested parenthesized or similar balanced syntax, such as function parameter lists, casts, array dimensions, or initializer expressions. The parser must move across the balanced region without losing nesting correctness.

The Rust version must support tests that verify:
- nested balanced delimiters are skipped correctly,
- multiple nesting levels are handled,
- the parser resumes at the correct position after the balanced region.

Traceability:
- `skip_balanced`
- `expression`
- `initializer_list`

### Scenario 4: Distinguishing function declarations from variable declarations
The parser examines a declaration associated with an identifier and must determine whether it describes a function or a variable. The correct specialized parsing path is then selected.

The Rust version must support tests that verify:
- function-shaped declarations are routed to function declaration parsing,
- non-function declarations are routed to variable declaration parsing,
- ambiguous declarator shapes are handled consistently with the C module behavior.

Traceability:
- `is_function`
- `parse_declaration`
- `parse_function_declaration`
- `parse_variable_declaration`

### Scenario 5: Parsing variable declarations with initializers
A variable declaration includes an initializer expression or initializer list. The parser must consume the initializer content and continue through the declaration correctly.

The Rust version must support tests that verify:
- simple assignment-style initializers are accepted,
- nested initializer lists are handled,
- parsing completes without misclassifying the declaration as a function.

Traceability:
- `parse_variable_declaration`
- `expression`
- `initializer_list`

### Scenario 6: Parsing function declarations and old-style declaration fragments
A function declaration or definition is encountered, potentially involving K&R-style declaration fragments. The parser must consume the function-related declaration form in the expected parser flow.

The Rust version must support tests that verify:
- function declarations are parsed through the function-specific path,
- K&R declaration handling is invoked where that form is present,
- subsequent parser flow remains aligned after function declaration parsing.

Traceability:
- `parse_function_declaration`
- `parse_knr_dcl`

### Scenario 7: Handling struct-related declaration placeholders
A declaration path requires struct-related placeholder handling so parsing can proceed even when the concrete struct form is not fully established at that point.

The Rust version must support tests that verify:
- the placeholder/synthetic struct behavior can be triggered through declaration parsing inputs that require it,
- declaration parsing continues consistently after this handling.

Traceability:
- `fake_struct`
- `parse_variable_declaration`

## Requirements

### Functional Requirements

#### FR-1: Token save stack
The module shall support accumulation of parser tokens into an ordered saved-token stack during parsing.

Traceability:
- `save_token`

#### FR-2: Token save rollback
The module shall support discarding the current saved-token accumulation when the active parse path is abandoned.

Traceability:
- `undo_save_stack`

#### FR-3: Token save finalization
The module shall support finalizing the saved-token accumulation into a returned character sequence using the module’s declaration-token finalization flow.

Traceability:
- `finish_save_stack`

#### FR-4: Target-token skipping
The module shall support advancing parser input until a specified token is encountered.

Traceability:
- `skip_to`

#### FR-5: Balanced-region skipping
The module shall support skipping nested balanced token regions while preserving delimiter nesting semantics.

Traceability:
- `skip_balanced`

#### FR-6: Parser entry execution
The module shall provide the parser entry behavior that coordinates parsing using this module’s declaration and expression handling routines.

Traceability:
- `yyparse`

#### FR-7: Function-form detection
The module shall determine whether the current declaration form represents a function.

Traceability:
- `is_function`

#### FR-8: Declaration dispatch
The module shall parse a declaration for a provided identifier context and dispatch to the appropriate specialized declaration parser based on declaration form.

Traceability:
- `parse_declaration`

#### FR-9: Declaration skipping
The module shall support skipping declaration content when required by parser flow.

Traceability:
- `skip_declaration`

#### FR-10: Expression handling in declaration contexts
The module shall process or skip expression content that occurs inside declaration parsing contexts sufficiently to maintain correct parser progression.

Traceability:
- `expression`

#### FR-11: Function declaration parsing
The module shall parse function declaration forms in the function-specific declaration path.

Traceability:
- `parse_function_declaration`

#### FR-12: Placeholder struct handling
The module shall support the struct-related placeholder behavior used during declaration parsing.

Traceability:
- `fake_struct`

#### FR-13: Variable declaration parsing
The module shall parse variable declaration forms, including declaration suffixes and initializer-related content as required by parser flow.

Traceability:
- `parse_variable_declaration`

#### FR-14: Initializer list handling
The module shall parse or skip initializer-list syntax, including nested forms, within variable declaration handling.

Traceability:
- `initializer_list`

#### FR-15: K&R declaration handling
The module shall support parsing K&R-style declaration fragments where encountered in function-related parsing.

Traceability:
- `parse_knr_dcl`

### Key Entities

#### 1. Saved token stack
A parser-managed accumulation of tokens used to preserve declaration text until it is either discarded or finalized.

Relationships:
- receives token entries from token-save operations,
- is consumed by finalization,
- may be reset by rollback.

Traceability:
- `save_token`
- `undo_save_stack`
- `finish_save_stack`
- `TOKSTK` parameter usage in `save_token`

#### 2. Identifier context
An identifier-associated parsing context used as input to declaration parsing routines and specialized declaration handlers.

Relationships:
- passed into general declaration parsing,
- passed into function and variable declaration handlers,
- used in placeholder struct handling and K&R-related flows.

Traceability:
- `parse_declaration`
- `parse_function_declaration`
- `fake_struct`
- `parse_variable_declaration`
- `parse_knr_dcl`
- `Ident` parameter usage

#### 3. Declaration form
The current parser-recognized declaration shape being analyzed as function, variable, skipped declaration, or old-style declaration fragment.

Relationships:
- classified by function detection,
- dispatched by general declaration parsing,
- may include initializer content or balanced nested syntax.

Traceability:
- `is_function`
- `parse_declaration`
- `skip_declaration`
- `parse_function_declaration`
- `parse_variable_declaration`
- `parse_knr_dcl`

#### 4. Balanced state
Parser state used to track nesting while traversing balanced token regions.

Relationships:
- used by balanced skipping,
- supports expression and initializer traversal where nested syntax appears.

Traceability:
- `skip_balanced`
- `expression`
- `initializer_list`
- `struct balance_state`

#### 5. Initializer/expression region
A declaration-associated token region representing assignment expressions or initializer lists.

Relationships:
- reached from variable declaration parsing,
- may contain nested balanced syntax,
- consumed by expression and initializer-list handlers.

Traceability:
- `expression`
- `parse_variable_declaration`
- `initializer_list`

## Success Criteria

### SC-1: Token accumulation correctness
For representative parser inputs that trigger token saving, the Rust module preserves token order and produces finalized token text consistent with the C module’s behavior.

Traceability:
- `save_token`
- `finish_save_stack`

### SC-2: Save-stack rollback correctness
For parser paths that are abandoned after token saving begins, the Rust module discards the active saved-token accumulation so that later parsing is unaffected.

Traceability:
- `undo_save_stack`

### SC-3: Delimiter skipping correctness
For inputs requiring synchronization to a target token, the Rust module advances to that token without stopping earlier on unrelated tokens.

Traceability:
- `skip_to`

### SC-4: Nested balance correctness
For declarations containing nested balanced delimiters, the Rust module exits skipping/parsing at the same structural boundary as the C module.

Traceability:
- `skip_balanced`
- `expression`
- `initializer_list`

### SC-5: Declaration classification correctness
For a test set containing both function and variable declaration forms, the Rust module selects the same parsing branch as the C module for each case.

Traceability:
- `is_function`
- `parse_declaration`

### SC-6: Variable declaration handling correctness
For variable declarations with and without initializers, including nested initializer lists, the Rust module completes declaration parsing with the same classification and parse progression as the C module.

Traceability:
- `parse_variable_declaration`
- `expression`
- `initializer_list`

### SC-7: Function declaration handling correctness
For function declaration inputs, including cases involving K&R-style declaration handling where applicable, the Rust module follows the same function-oriented parse path as the C module.

Traceability:
- `parse_function_declaration`
- `parse_knr_dcl`

### SC-8: Placeholder struct behavior preservation
For declaration inputs that trigger struct-placeholder handling, the Rust module exhibits the same parser-visible continuation behavior as the C module.

Traceability:
- `fake_struct`

### SC-9: Parser entry compatibility
When driven through the module’s parser entry flow, the Rust version supports declaration parsing scenarios covered by this specification without loss of the documented behaviors.

Traceability:
- `yyparse`

## Constraints and Non-Goals
- The Rust port must preserve evidenced behavior only; it must not add unsupported parsing capabilities.
- The specification does not require new public interfaces beyond what is necessary to replace the module behavior.
- The specification does not require guarantees about error recovery beyond parser progression behavior evidenced by the module.
- The specification does not define lexer internals or unrelated parser subsystems.

## Acceptance Approach
Acceptance should be based on behaviorally equivalent tests comparing the Rust port against the C module for:
- saved token lifecycle,
- delimiter and balanced-region skipping,
- function versus variable declaration routing,
- variable initializers and initializer lists,
- function declarations including K&R-style fragments,
- and parser continuation after struct-placeholder handling.