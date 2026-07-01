# spec.md

## Title

Rust Port Functional Specification: `module_src_parser.c_30`

## Overview

This module provides parser-side declaration and expression handling for the `cflow-new` project. It operates within the C source parsing pipeline and is responsible for:

- capturing and replaying token sequences used while recognizing declarations,
- skipping or balancing over token regions that should not be fully parsed at the current step,
- distinguishing function declarators from other declarations,
- parsing declarations, variable declarators, function declarators, initializer lists, and expressions,
- supporting K&R-style declaration handling where required by the original parser flow.

The Rust rewrite on branch `093-module_src_parser.c_30-rust-port` must preserve the observable parsing behavior represented by `src/parser.c` for this module boundary.

## Scope

In scope for this module:

- token-save stack behavior used during declaration parsing,
- parser control flow for declaration and expression processing,
- balanced skipping over nested syntactic regions,
- recognition and handling of function versus variable declarations,
- handling of initializer lists and old-style K&R declaration forms.

Out of scope:

- lexer/token production not evidenced in this module,
- unrelated parser subsystems outside the functions listed for `src/parser.c`,
- new parsing features or grammar extensions not present in the analyzed module.

## Source Basis

This specification is derived from:

- File: `src/parser.c`
- Functions:
  - `save_token`
  - `undo_save_stack`
  - `finish_save_stack`
  - `skip_to`
  - `skip_balanced`
  - `yyparse`
  - `is_function`
  - `parse_declaration`
  - `skip_declaration`
  - `expression`
  - `parse_function_declaration`
  - `fake_struct`
  - `parse_variable_declaration`
  - `initializer_list`
  - `parse_knr_dcl`
- Key types referenced in the module:
  - token-save stack state structures
  - `balance_state`
  - `Ident`
  - `declaration`

## Feature Specification

### 1. Parser entry behavior

The module must provide the parser entry behavior represented by `yyparse`, coordinating declaration-oriented parsing work within this source parser component.

The Rust version must preserve:

- parser progress through top-level source constructs handled by this module,
- invocation of declaration parsing paths when declaration-like input is encountered,
- use of helper logic for classification, skipping, and sub-parsing.

### 2. Token save stack management

The module must support temporary preservation of token sequences during parsing decisions.

This includes behavior corresponding to:

- saving tokens into a temporary stack or accumulation area,
- discarding the current saved sequence,
- finalizing the saved sequence into a name/string result for later use by parser logic.

The Rust version must preserve the functional distinction between:

- adding to the current saved token sequence,
- abandoning the saved sequence without committing it,
- finishing the sequence and obtaining the resulting text form.

### 3. Directed token skipping

The module must be able to advance parsing until a specified token is reached, and also skip across balanced nested syntax.

The Rust version must preserve behavior for:

- skipping directly to a target token,
- skipping nested balanced constructs using opener/closer tracking,
- handling nested grouping levels while scanning expressions or declarators.

### 4. Function declarator recognition

The module must determine whether the current declaration context represents a function declarator.

The Rust version must preserve the classification behavior used by downstream declaration parsing so that function declarations are routed to function-specific handling and non-function declarations are routed to variable handling.

### 5. Declaration parsing dispatch

The module must parse declarations in a way that dispatches to the correct specialized path.

The Rust version must preserve behavior for:

- receiving an `Ident` context and parameter/declaration mode,
- choosing between function declaration parsing and variable declaration parsing,
- supporting declaration skipping when full parsing is not required.

### 6. Expression parsing support

The module must parse or consume expressions sufficiently for declaration-related parsing flow.

The Rust version must preserve expression handling needed by:

- declarator parsing,
- initializer processing,
- skipping over nested expression syntax where balanced grouping matters.

### 7. Function declaration parsing

The module must parse function declaration forms handled by this parser component.

The Rust version must preserve behavior for:

- function declarator handling tied to an `Ident`,
- parameter-context-sensitive declaration parsing,
- support for old-style declaration follow-up through the K&R path where used.

### 8. Variable declaration parsing

The module must parse variable declaration forms handled by this parser component.

The Rust version must preserve behavior for:

- variable declarator handling tied to an `Ident`,
- declaration parsing in parameter and non-parameter contexts,
- interaction with initializer parsing,
- behavior around special struct-like fallback handling represented by `fake_struct`.

### 9. Initializer list handling

The module must handle initializer lists associated with declarations.

The Rust version must preserve behavior for:

- consuming initializer syntax,
- supporting nested or grouped initializer content to the extent represented by the current parser logic.

### 10. K&R declaration support

The module must support old-style K&R declaration handling where this parser path invokes it.

The Rust version must preserve the behavior represented by `parse_knr_dcl` as part of function declaration processing.

## User Scenarios & Testing

### Scenario 1: Parse a declaration and classify it correctly

A caller drives the parser over C source containing a declaration-like construct.
The module determines whether the construct is a function declaration or a variable declaration and routes parsing to the appropriate path.

The Rust version must support tests where:

- a function declarator is recognized and parsed through the function-declaration path,
- a non-function declarator is recognized and parsed through the variable-declaration path.

Traceability: `is_function`, `parse_declaration`, `parse_function_declaration`, `parse_variable_declaration`.

### Scenario 2: Save tokens during tentative parsing

While inspecting a declaration, the parser needs to preserve tokens temporarily, then either discard them or finalize them into a resulting name/text value.

The Rust version must support tests where:

- tokens are saved in order,
- `undo`-style behavior discards the uncommitted sequence,
- `finish`-style behavior produces the finalized text corresponding to the saved sequence.

Traceability: `save_token`, `undo_save_stack`, `finish_save_stack`.

### Scenario 3: Skip over nested syntax safely

The parser encounters a region that should be consumed without detailed parsing, such as nested grouped syntax inside declarators or expressions.

The Rust version must support tests where:

- skipping to a target token stops at the requested delimiter,
- balanced skipping correctly traverses nested parentheses/brackets/braces as represented by parser tokens,
- nested depth is handled without prematurely stopping on inner closing tokens.

Traceability: `skip_to`, `skip_balanced`, `balance_state`.

### Scenario 4: Parse declarations with initializers

A variable declaration includes an initializer, potentially including grouped or list-based syntax.

The Rust version must support tests where:

- a declaration with a simple initializer is consumed correctly,
- an initializer list is consumed correctly,
- nested grouped elements within the initializer do not break parser progress.

Traceability: `parse_variable_declaration`, `initializer_list`, `expression`.

### Scenario 5: Handle function declarations with old-style parameter declarations

The parser processes a function declaration or definition using K&R-style declaration handling supported by the original module.

The Rust version must support tests where:

- the function declaration path invokes old-style declaration parsing when applicable,
- parsing progresses through the old-style declaration segment without misclassifying it as a variable declaration.

Traceability: `parse_function_declaration`, `parse_knr_dcl`.

### Scenario 6: Skip declarations when only structural progress is needed

The parser needs to move past a declaration without fully processing all internals.

The Rust version must support tests where declaration skipping consumes the declaration extent expected by the original parser flow and leaves the parser positioned for subsequent input.

Traceability: `skip_declaration`, `skip_to`, `skip_balanced`.

## Requirements

### Functional Requirements

#### FR-1: Parser coordination
The module shall provide parser control behavior equivalent to the `yyparse` role in `src/parser.c`, enabling declaration- and expression-related parsing progress within this module boundary.

Traceability: `yyparse`.

#### FR-2: Token sequence preservation
The module shall support saving tokens from the current parse stream into temporary parser-managed state for later commitment or discard.

Traceability: `save_token`.

#### FR-3: Token save rollback
The module shall support abandoning the current saved token sequence without producing a finalized result.

Traceability: `undo_save_stack`.

#### FR-4: Token save finalization
The module shall support finalizing the current saved token sequence into a resulting text/name value used by parser logic.

Traceability: `finish_save_stack`.

#### FR-5: Direct skip-to-token behavior
The module shall support advancing through input until a specified token is reached.

Traceability: `skip_to`.

#### FR-6: Balanced nested skipping
The module shall support skipping across nested balanced syntax using tracked nesting state, so that inner grouped structures are consumed before the outer structure is considered closed.

Traceability: `skip_balanced`, `balance_state`.

#### FR-7: Function declarator detection
The module shall determine whether the current declaration context corresponds to a function declarator.

Traceability: `is_function`.

#### FR-8: Declaration parsing dispatch
The module shall parse a declaration by accepting an `Ident` context and parameter-mode indicator, and dispatching to the appropriate specialized declaration parser.

Traceability: `parse_declaration`, `Ident`.

#### FR-9: Declaration skipping
The module shall support consuming a declaration without performing full declaration parsing when the parser flow requires only structural advancement.

Traceability: `skip_declaration`.

#### FR-10: Expression consumption for parser flow
The module shall parse or consume expressions to the extent required for declaration parsing, initializer parsing, and balanced parser advancement.

Traceability: `expression`.

#### FR-11: Function declaration parsing
The module shall parse function declaration forms, using the provided `Ident` context and parameter-mode information.

Traceability: `parse_function_declaration`, `Ident`.

#### FR-12: Variable declaration parsing
The module shall parse variable declaration forms, using the provided `Ident` context and parameter-mode information.

Traceability: `parse_variable_declaration`, `Ident`.

#### FR-13: Initializer list parsing
The module shall consume initializer-list syntax associated with declarations.

Traceability: `initializer_list`.

#### FR-14: Old-style K&R declaration parsing
The module shall support the old-style K&R declaration parsing path used during function declaration handling.

Traceability: `parse_knr_dcl`.

#### FR-15: Struct-like fallback handling in variable declarations
The module shall preserve the variable-declaration behavior associated with the `fake_struct` decision point when parsing declaration forms that require this fallback or classification step.

Traceability: `fake_struct`, `parse_variable_declaration`.

### Key Entities

#### `Ident`
Represents the declaration identity/context passed into declaration parsing functions. It is the primary per-declaration entity shared across:

- general declaration parsing,
- function declaration parsing,
- variable declaration parsing,
- K&R declaration follow-up.

Traceability: `parse_declaration`, `parse_function_declaration`, `parse_variable_declaration`, `parse_knr_dcl`.

#### Token save stack state
Represents temporary parser-managed saved token content accumulated during tentative or staged parsing. It supports three lifecycle operations:

- append/save,
- rollback/discard,
- finalize into text.

Traceability: `save_token`, `undo_save_stack`, `finish_save_stack`.

#### `balance_state`
Represents nesting/balancing state used while skipping over grouped syntax. It relates opening-token handling to current nesting level and supports correct traversal of nested constructs.

Traceability: `skip_balanced`, anonymous `struct balance_state` definitions/usages in `src/parser.c`.

#### `declaration`
Represents declaration-related parser context referenced by this module, even though its local definition is outside the analyzed file. It is part of the declaration parsing domain in which `Ident` and specialized declaration parsers operate.

Traceability: referenced type name in module analysis; declaration parsing functions in `src/parser.c`.

## Success Criteria

1. The Rust module can distinguish function declarators from non-function declarators in all parser paths currently handled by `is_function` and route them to the corresponding declaration parser.
   - Traceability: `is_function`, `parse_declaration`, `parse_function_declaration`, `parse_variable_declaration`.

2. Saved token handling in Rust preserves the original operation semantics: a saved sequence can be appended, discarded, or finalized into text without mixing the effects of rollback and finalization.
   - Traceability: `save_token`, `undo_save_stack`, `finish_save_stack`.

3. Balanced skipping in Rust correctly consumes nested grouped syntax and does not terminate at an inner unmatched closing token when the original parser would continue to the matching outer boundary.
   - Traceability: `skip_balanced`, `balance_state`.

4. Direct skip behavior in Rust advances to the requested token boundary in the same parser situations supported by the original module.
   - Traceability: `skip_to`.

5. Variable declaration parsing in Rust accepts declarations with and without initializers, including initializer-list forms handled by the current module.
   - Traceability: `parse_variable_declaration`, `initializer_list`, `expression`.

6. Function declaration parsing in Rust preserves the original module’s handling of parameter-context-sensitive parsing and K&R declaration follow-up where applicable.
   - Traceability: `parse_function_declaration`, `parse_knr_dcl`.

7. Declaration skipping in Rust leaves parser progress aligned for subsequent parsing in the same situations supported by the original `skip_declaration` flow.
   - Traceability: `skip_declaration`, `skip_to`, `skip_balanced`.

8. The Rust parser entry behavior for this module preserves successful progression through source constructs handled by `yyparse` without introducing unsupported new grammar behavior.
   - Traceability: `yyparse`.