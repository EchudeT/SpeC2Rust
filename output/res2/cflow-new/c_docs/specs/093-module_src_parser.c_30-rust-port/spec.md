# spec.md

## Title

Functional Specification for `module_src_parser.c_30` Rust Port

## Metadata

- **Project**: `cflow-new`
- **Module**: `module_src_parser.c_30`
- **Category**: `module_cluster`
- **Source file**: `src/parser.c`
- **Target Rust branch**: `093-module_src_parser.c_30-rust-port`
- **Generation date**: `2026-06-17`

## Overview

This module performs parser-side declaration and expression handling for C source analysis. Its role is to consume parser tokens, preserve token text when needed, classify declarations, distinguish functions from variables, skip over balanced syntactic regions, and process declaration forms including function declarations, variable declarations, initializer lists, and K&R-style declarations.

The Rust rewrite must preserve the observed parsing behavior and boundaries evidenced by `src/parser.c`, especially:

- token capture and later materialization into text,
- controlled skipping through token streams,
- balanced traversal of nested delimiters,
- declaration classification,
- expression traversal sufficient for declaration parsing,
- handling of function and variable declaration forms,
- support for fake or placeholder struct handling as used during declaration parsing.

This specification covers only functionality evidenced by the analyzed module.

## Scope

### In Scope

The Rust version must implement the functional behavior represented by these parser responsibilities in `src/parser.c`:

- saving parser tokens into a temporary stack,
- undoing or finalizing a saved token sequence,
- skipping input until a target token,
- skipping nested balanced constructs,
- top-level parser entry behavior represented by `yyparse`,
- recognizing whether a declaration sequence represents a function,
- parsing declarations generically and dispatching to the applicable declaration form,
- skipping declarations without producing a full parsed result,
- traversing expressions used inside declarations and initializers,
- parsing function declarations,
- parsing variable declarations,
- parsing initializer lists,
- parsing K&R declaration forms,
- handling the special struct-related path represented by `fake_struct`.

### Out of Scope

The Rust port specification does not require any behavior not evidenced here, including:

- designing new public APIs beyond what is necessary to support this module,
- concurrency guarantees,
- persistence or serialization,
- error recovery semantics beyond the parser behavior implied by the source,
- performance targets or benchmarking features,
- support for language constructs not handled by the source module.

## Feature Specification

### Feature 1: Token Capture for Deferred Declaration Text Handling

The module supports temporarily saving parser tokens during declaration parsing and later either discarding the saved sequence or converting it into final text.

Observed behaviors tied to this feature:

- `save_token` stores token material into a save stack.
- `undo_save_stack` discards the currently saved token sequence.
- `finish_save_stack` finalizes the sequence and returns text derived from the saved tokens, parameterized by a provided name.

The Rust version must preserve the behavioral distinction between:

- accumulating token content,
- abandoning that accumulation,
- completing accumulation into a textual result.

This behavior is required because later declaration parsing depends on temporary token preservation before final classification is known.

### Feature 2: Directed and Balanced Token Skipping

The module can advance through tokens in two controlled ways:

- `skip_to(int c)` advances until a target token is reached.
- `skip_balanced(int open_tok, int level)` advances across nested balanced syntax using an opening token and nesting level.

The Rust version must preserve the ability to ignore irrelevant or already-classified syntax while maintaining proper nesting semantics for balanced delimiters. This is required for declarations, expressions, parameter lists, and initializer handling.

### Feature 3: Parser Entry and Declaration-Oriented Parsing Flow

The parser entry represented by `yyparse` drives parsing behavior for the source stream handled by this module. The module also provides:

- `parse_declaration(Ident *ident, int parm)` for declaration parsing,
- `skip_declaration(void)` for declaration skipping.

The Rust version must preserve declaration-oriented parsing flow, including the ability to either parse or skip declarations according to the same control boundary evidenced in the C source.

### Feature 4: Declaration Classification

The module determines whether a parsed declarator should be treated as a function form through `is_function`.

The Rust version must preserve this classification step because subsequent parsing paths diverge between:

- function declarations handled by `parse_function_declaration`,
- variable declarations handled by `parse_variable_declaration`.

### Feature 5: Expression Traversal for Declarative Contexts

The `expression` function parses or traverses expression syntax as needed by declaration-related constructs.

The Rust version must support expression traversal sufficient for:

- declarator-associated expressions,
- initializer expressions,
- nested expression forms within balanced syntax.

This requirement is limited to the declaration and initializer behavior evidenced by the module.

### Feature 6: Function Declaration Parsing

The module parses function declaration forms using `parse_function_declaration(Ident *ident, int parm)`.

The Rust version must support parsing function declarators and their associated syntax in the same declaration-processing flow used by the source module, including parameter-context-sensitive handling indicated by the `parm` argument.

### Feature 7: Variable Declaration Parsing

The module parses variable declarations using `parse_variable_declaration(Ident *ident, int parm)`.

The Rust version must support variable declarators, including declaration continuations and initialization-related syntax as evidenced by the function cluster around variable declarations and initializer parsing.

### Feature 8: Initializer List Parsing

The module supports initializer parsing through `initializer_list`.

The Rust version must preserve support for nested or compound initializer syntax as part of variable declaration handling.

### Feature 9: K&R Declaration Handling

The module includes `parse_knr_dcl(Ident *ident)`.

The Rust version must preserve support for K&R-style declaration handling to the extent evidenced by this dedicated parsing function.

### Feature 10: Struct Placeholder Handling During Declaration Parsing

The module includes `fake_struct(Ident *ident)`.

The Rust version must preserve the special declaration-path behavior represented by this function, where struct-related parsing may require a placeholder or synthetic handling step before normal declaration processing continues.

## User Scenarios & Testing

### Scenario 1: Saving Tokens Until Declaration Form Is Known

A parser workflow begins reading a declaration and cannot immediately decide whether the token sequence belongs to a function or variable declaration. It saves tokens as they are consumed. Once classification is complete, it either:

- discards the saved sequence if no final text is needed, or
- finalizes the saved sequence into declaration text tied to a supplied name.

**Required test coverage:**

- saving multiple tokens in sequence preserves their order,
- undo clears the pending saved content,
- finalization returns text for the current saved sequence,
- finalization after saving a declarative sequence behaves consistently with a provided name argument.

### Scenario 2: Skipping to a Delimiter

While parsing, the module encounters syntax that should be skipped until a known terminating token. It advances through input until that token is reached.

**Required test coverage:**

- skipping stops at the target token,
- non-target tokens before the target are consumed,
- behavior is correct when target appears after nested content managed elsewhere.

### Scenario 3: Skipping Over Nested Balanced Syntax

The parser encounters nested parentheses, braces, or similar balanced regions inside expressions or declarators. It must skip the whole balanced region without being confused by inner nesting.

**Required test coverage:**

- a single balanced pair is skipped correctly,
- nested balanced pairs increase and decrease nesting correctly,
- skipping terminates when the requested nesting level is satisfied.

### Scenario 4: Distinguishing Function and Variable Declarations

A declaration-like input is parsed and must be classified into a function path or a variable path before detailed parsing proceeds.

**Required test coverage:**

- function-shaped declarators are routed to function declaration handling,
- variable-shaped declarators are routed to variable declaration handling,
- classification remains correct in the presence of nested declarator syntax.

### Scenario 5: Parsing a Function Declaration

The parser processes a function declaration, including declarator syntax and declaration-context parsing for parameters.

**Required test coverage:**

- function declaration inputs complete through the function-declaration path,
- parameter-context parsing is honored when the declaration is parsed with parameter mode enabled,
- function parsing integrates with token saving and balanced skipping where needed.

### Scenario 6: Parsing a Variable Declaration With Initializer

The parser processes a variable declaration that includes an initializer expression or initializer list.

**Required test coverage:**

- simple initializer expressions are traversed successfully,
- compound initializer forms are parsed through initializer-list handling,
- nested balanced syntax inside an initializer does not break parsing.

### Scenario 7: Skipping an Unneeded Declaration

A caller chooses not to fully parse a declaration and instead skips it.

**Required test coverage:**

- declaration skipping consumes the declaration boundary used by this module,
- skipping does not require function/variable result production,
- subsequent parsing can continue after the skipped declaration boundary.

### Scenario 8: Handling K&R-Style Declarations

The parser encounters an older-style function declaration form and processes it through the K&R-specific path.

**Required test coverage:**

- inputs corresponding to the K&R declaration path invoke dedicated handling,
- K&R handling integrates with the surrounding declaration parser flow.

### Scenario 9: Struct-Related Placeholder Handling

A declaration path requires the special struct-related handling represented by `fake_struct`.

**Required test coverage:**

- the struct-related special case is recognized,
- the special-case path returns control to the declaration parser in a consistent state.

## Requirements

### Functional Requirements

#### FR-1: Token Saving
The module shall support appending parser token information into a temporary saved-token sequence during parsing, as evidenced by `save_token` in `src/parser.c`.

#### FR-2: Save-Stack Reversal
The module shall support discarding the current temporary saved-token sequence without producing final text, as evidenced by `undo_save_stack` in `src/parser.c`.

#### FR-3: Saved-Token Finalization
The module shall support finalizing the current saved-token sequence into a character-text result using a provided name input, as evidenced by `finish_save_stack` in `src/parser.c`.

#### FR-4: Directed Token Skipping
The module shall support advancing through parser input until a specified token is reached, as evidenced by `skip_to` in `src/parser.c`.

#### FR-5: Balanced Syntax Skipping
The module shall support advancing across nested balanced syntax using an opening token and nesting level, as evidenced by `skip_balanced` and the `balance_state` structures in `src/parser.c`.

#### FR-6: Parser Entry Execution
The module shall provide the parser-entry behavior represented by `yyparse`, capable of driving this module’s declaration-related parsing flow in `src/parser.c`.

#### FR-7: Function Classification
The module shall determine whether the current declaration form is a function form before dispatching to a specialized declaration parser, as evidenced by `is_function` in `src/parser.c`.

#### FR-8: Generic Declaration Parsing
The module shall parse declarations through a common declaration entry point that accepts an identifier context and parameter-context flag, as evidenced by `parse_declaration(Ident *ident, int parm)` in `src/parser.c`.

#### FR-9: Declaration Skipping
The module shall support skipping a declaration without fully parsing it, as evidenced by `skip_declaration(void)` in `src/parser.c`.

#### FR-10: Expression Traversal
The module shall parse or traverse expressions needed by declaration-related syntax, as evidenced by `expression()` in `src/parser.c`.

#### FR-11: Function Declaration Parsing
The module shall parse function declarations through a specialized path that accepts identifier context and parameter-context flag, as evidenced by `parse_function_declaration(Ident *ident, int parm)` in `src/parser.c`.

#### FR-12: Variable Declaration Parsing
The module shall parse variable declarations through a specialized path that accepts identifier context and parameter-context flag, as evidenced by `parse_variable_declaration(Ident *ident, int parm)` in `src/parser.c`.

#### FR-13: Initializer List Parsing
The module shall parse initializer lists used by declarations, as evidenced by `initializer_list()` in `src/parser.c`.

#### FR-14: K&R Declaration Parsing
The module shall support the dedicated K&R declaration parsing path, as evidenced by `parse_knr_dcl(Ident *ident)` in `src/parser.c`.

#### FR-15: Struct-Related Special Handling
The module shall implement the declaration-time struct-related special handling represented by `fake_struct(Ident *ident)`, as evidenced by `src/parser.c`.

### Key Entities

#### Entity 1: Saved Token Stack
A temporary token-holding structure is used to accumulate token material during parsing before the parser decides whether to commit or discard it. This entity is directly involved in:

- token saving,
- save-stack undo,
- save-stack finalization.

The analyzed source exposes this behavior through `save_token`, `undo_save_stack`, and `finish_save_stack`.

#### Entity 2: Identifier Context (`Ident`)
An identifier-context object is passed into declaration parsing functions and struct-related special handling. It represents the declaration subject being parsed. This entity participates in:

- generic declaration parsing,
- function declaration parsing,
- variable declaration parsing,
- K&R declaration parsing,
- struct-related special handling.

#### Entity 3: Balance State
A balance-tracking structure is used to manage nested delimiter traversal while skipping balanced syntax. It supports correct nesting behavior during parser advancement across expressions, declarators, and initializer content.

This entity is evidenced by the `balance_state` structures associated with `skip_balanced`.

#### Entity 4: Declaration Context Flag (`parm`)
Several declaration-parsing functions accept an integer flag indicating parameter-context-sensitive behavior. This flag affects how declarations are interpreted within the parser flow.

This entity is evidenced by the signatures of:

- `parse_declaration`,
- `parse_function_declaration`,
- `parse_variable_declaration`.

#### Entity 5: Declaration Form
The parser distinguishes between at least these declaration forms:

- function declaration,
- variable declaration,
- K&R declaration path,
- struct-related special case.

This classification is represented across `is_function`, `parse_function_declaration`, `parse_variable_declaration`, `parse_knr_dcl`, and `fake_struct`.

## Success Criteria

1. **Saved-token behavior parity**: The Rust module can save tokens, discard a saved sequence, and finalize a saved sequence into text in workflows corresponding to `save_token`, `undo_save_stack`, and `finish_save_stack`.

2. **Directed skipping parity**: Given parser input containing a later target token, the Rust module advances until that token in a manner consistent with `skip_to`.

3. **Balanced skipping parity**: For nested balanced input, the Rust module exits balanced skipping at the correct nesting boundary in a manner consistent with `skip_balanced`.

4. **Declaration classification parity**: The Rust module distinguishes function declarations from non-function declarations sufficiently to route parsing through the same specialized paths evidenced by `is_function`, `parse_function_declaration`, and `parse_variable_declaration`.

5. **Generic declaration parsing parity**: The Rust module accepts an identifier context plus parameter-context flag and parses declarations through a common entry behavior consistent with `parse_declaration`.

6. **Declaration skipping parity**: The Rust module can consume and skip declaration syntax without producing a full declaration parse result, consistent with `skip_declaration`.

7. **Expression-handling parity**: The Rust module successfully traverses declaration-related expressions and nested expression syntax needed by declaration parsing, consistent with `expression`.

8. **Function declaration parity**: The Rust module successfully processes function declaration inputs through a dedicated function-declaration path, including parameter-context-sensitive behavior, consistent with `parse_function_declaration`.

9. **Variable declaration parity**: The Rust module successfully processes variable declaration inputs, including declaration continuations and initialization-related syntax, consistent with `parse_variable_declaration`.

10. **Initializer parsing parity**: The Rust module correctly handles initializer-list syntax used within variable declarations, consistent with `initializer_list`.

11. **K&R handling parity**: The Rust module preserves the dedicated K&R declaration handling path represented by `parse_knr_dcl`.

12. **Struct special-case parity**: The Rust module preserves the struct-related special handling path represented by `fake_struct`.

13. **Parser-flow integration parity**: The Rust implementation integrates the above behaviors into a parser execution flow corresponding to `yyparse` so that declaration-related parsing proceeds without missing any evidenced module responsibilities.