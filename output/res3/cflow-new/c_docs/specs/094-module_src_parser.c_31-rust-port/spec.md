# spec.md

## Title

Rust Functional Specification for `module_src_parser.c_31`

## Overview

This module is responsible for parsing C declaration and function-related syntax from `src/parser.c` and updating symbol/reference information derived from that syntax.

The Rust rewrite must preserve the observable parsing behavior evidenced by the analyzed functions in this module cluster:

- declaration parsing for typedefs and ordinary declarators
- direct declarator parsing, including nested declarators
- parameter declaration and parameter-list recognition
- function-body traversal sufficient to discover references/calls
- K&R-style argument handling where applicable
- symbol lookup and creation of reference/call records
- skipping over struct/union/enum-like declaration bodies when required by declaration parsing

This specification covers only the functionality evidenced by the listed parser functions and related symbol/reference operations. It does not require new parsing features, new public APIs, or behaviors not traceable to `src/parser.c`.

## Scope

### In Scope

The Rust module must implement the behavior represented by these functions from `src/parser.c`:

- `skip_struct`
- `parse_typedef`
- `parse_dcl`
- `dcl`
- `getident`
- `dirdcl`
- `parmdcl`
- `maybe_parm_list`
- `func_body`
- `get_knr_args`
- `declare`
- `declare_type`
- `get_symbol`
- `add_reference`
- `call`

### Out of Scope

The following are not required unless already necessary to preserve the evidenced behavior above:

- new language support beyond the declaration/call handling shown here
- redesigned public interfaces unrelated to the original parser role
- concurrency guarantees
- persistence or serialization
- error recovery beyond the behavior needed to continue parsing as this module does
- performance targets not evidenced by the source analysis

## Feature Specification

### 1. Declaration Parsing

The module must parse C declarator forms associated with identifiers and determine declaration-related outcomes needed by the surrounding parser.

This includes:

- parsing typedef declarations
- parsing general declarations through a declaration entry point
- parsing pointer-prefixed declarators
- parsing direct declarators, including grouped declarators
- recognizing parameter declaration syntax associated with functions
- distinguishing declaration contexts that may correspond to K&R-style function declarations

The Rust version must preserve the functional result of these parsing operations as used by later symbol and reference handling.

Traceability:
- `parse_typedef`
- `parse_dcl`
- `dcl`
- `getident`
- `dirdcl`
- `parmdcl`
- `declare`
- `declare_type`

### 2. Struct-like Body Skipping During Parsing

The module must be able to skip over a structured type body when declaration parsing encounters such syntax and the parser needs to advance past it without treating its interior as the current declarator target.

This behavior is required so declaration parsing can continue correctly after structured-type syntax.

Traceability:
- `skip_struct`

### 3. Parameter List Recognition

The module must recognize and process possible function parameter lists associated with a declarator.

This includes:

- detecting whether a declarator is followed by a parameter list
- counting or otherwise returning parameter-list information as needed by the parser flow
- parsing individual parameter declarators
- supporting the declaration forms relevant to both standard and K&R-style parsing paths evidenced by the module

Traceability:
- `maybe_parm_list`
- `parmdcl`
- `get_knr_args`
- `dirdcl`
- `getident`

### 4. Function Body Traversal for References and Calls

Once a function definition body is encountered, the module must traverse the body sufficiently to identify symbol references and function calls relevant to the project’s flow extraction behavior.

The Rust version must preserve:

- entering function-body parsing after declaration parsing indicates a function definition
- detecting references encountered within the body
- recording call relationships when call syntax is recognized
- tracking source line information for references/calls where the original behavior records it

Traceability:
- `func_body`
- `add_reference`
- `call`

### 5. K&R-style Argument Handling

The module must support the declaration path where a function declarator may be followed by old-style K&R argument declarations before the body.

The Rust version must preserve the behavior needed to:

- recognize this parsing path
- collect the applicable argument declarations
- integrate them with the function declaration state before function-body traversal continues

Traceability:
- `parse_dcl`
- `get_knr_args`
- `declare`

### 6. Symbol Lookup and Reference/Call Registration

The module must support symbol-oriented operations needed by declaration and function-body parsing:

- lookup of a symbol by name
- creation or augmentation of reference information for a symbol
- registration of call usage for a named target

The Rust version must preserve the distinction between symbol retrieval and the addition of usage information.

Traceability:
- `get_symbol`
- `add_reference`
- `call`

## User Scenarios & Testing

### Scenario 1: Parsing a Typedef Declaration

A higher-level parser encounters a typedef statement in C source. This module parses the declaration, identifies the declared name(s), and updates declaration state so later parsing treats the typedef name appropriately.

The Rust version must support tests where:

- a typedef with a simple declarator is parsed successfully
- a typedef using nested declarator syntax is parsed successfully
- structured-type syntax associated with the typedef does not prevent completion of the declaration parse

Traceability:
- `parse_typedef`
- `parse_dcl`
- `dcl`
- `dirdcl`
- `skip_struct`

### Scenario 2: Parsing an Ordinary Declaration

A declaration statement for an identifier is encountered. The module parses the declarator form, including pointer and direct declarator syntax, and passes declaration information onward.

The Rust version must support tests where:

- a simple identifier declaration is recognized
- pointer-prefixed declarations are recognized
- parenthesized/nested declarators are recognized
- declaration parsing completes without incorrectly treating non-function declarations as function definitions

Traceability:
- `parse_dcl`
- `dcl`
- `getident`
- `dirdcl`
- `declare`
- `declare_type`

### Scenario 3: Parsing a Function Declarator with Parameters

A function declaration or definition header is encountered. The module detects the parameter-list syntax, parses parameter declarations, and returns the parameter-related result needed by subsequent parser logic.

The Rust version must support tests where:

- a function with an empty parameter list is recognized
- a function with one or more parameter declarations is recognized
- parameter parsing works through the direct-declarator path
- the parser returns parameter-list information consistent with the source form

Traceability:
- `dirdcl`
- `parmdcl`
- `maybe_parm_list`
- `getident`

### Scenario 4: Parsing an Old-style K&R Function Definition

A function definition uses K&R-style argument declarations after the identifier list. The module must recognize the old-style form, collect argument declarations, and then continue into the function body.

The Rust version must support tests where:

- an identifier-list function header is recognized as a K&R-style candidate
- subsequent argument declarations are consumed by the K&R handling path
- body parsing begins only after the K&R declarations have been processed

Traceability:
- `parse_dcl`
- `get_knr_args`
- `declare`
- `func_body`

### Scenario 5: Recording References in a Function Body

While traversing a function body, the module encounters a name usage that should be treated as a symbol reference. The module looks up or creates the symbol usage record and associates the current line.

The Rust version must support tests where:

- a referenced identifier produces symbol usage data
- repeated references to the same name are associated with that symbol
- recorded reference information includes the source line input used by the parser flow

Traceability:
- `func_body`
- `get_symbol`
- `add_reference`

### Scenario 6: Recording a Function Call

While traversing a function body, the module recognizes call syntax for a named target and records the call relationship.

The Rust version must support tests where:

- a named call in a function body is identified as a call rather than only a plain reference
- call recording associates the target name with the relevant source line
- repeated calls to the same target continue to resolve through symbol lookup/registration behavior

Traceability:
- `func_body`
- `get_symbol`
- `call`

## Requirements

### Functional Requirements

#### FR-1: Structured-type skipping
The module shall skip over structured-type declaration bodies when declaration parsing requires advancing past them without parsing their internal members as the current declarator target.

Traceability:
- `skip_struct`

#### FR-2: Typedef parsing
The module shall parse typedef declarations and apply the same declaration-parsing path needed to identify the declared typedef name(s).

Traceability:
- `parse_typedef`
- `parse_dcl`

#### FR-3: General declarator parsing
The module shall parse a declarator for a candidate identifier, including pointer-indirection prefixes and the direct-declarator component.

Traceability:
- `parse_dcl`
- `dcl`
- `dirdcl`

#### FR-4: Identifier extraction from declarators
The module shall extract identifier information from declarator syntax and return any parameter-related information required by subsequent parsing steps.

Traceability:
- `getident`

#### FR-5: Direct declarator handling
The module shall parse direct declarators, including identifier forms, grouped/nested forms, and function-related suffixes needed by this module’s declaration flow.

Traceability:
- `dirdcl`

#### FR-6: Parameter declaration parsing
The module shall parse individual parameter declarations associated with function declarators.

Traceability:
- `parmdcl`

#### FR-7: Possible parameter-list recognition
The module shall detect and process a possible parameter list and provide parameter-count or equivalent parameter-list outcome data needed by parser control flow.

Traceability:
- `maybe_parm_list`

#### FR-8: Function-body parsing
The module shall traverse function-definition bodies sufficiently to detect symbol references and named calls relevant to flow extraction.

Traceability:
- `func_body`

#### FR-9: K&R argument declaration handling
The module shall support old-style K&R function argument declaration parsing when the declaration path indicates that form.

Traceability:
- `get_knr_args`
- `parse_dcl`
- `declare`

#### FR-10: Declaration finalization
The module shall handle declaration completion logic for parsed identifiers, including the path that distinguishes ordinary declarations from function definitions and related declaration forms.

Traceability:
- `declare`
- `parse_dcl`

#### FR-11: Type-oriented declaration handling
The module shall support declaration handling where the parser needs to process type-oriented declaration state for an identifier.

Traceability:
- `declare_type`

#### FR-12: Symbol lookup
The module shall provide lookup of a symbol record by name for use during reference and call registration.

Traceability:
- `get_symbol`

#### FR-13: Reference registration
The module shall add or update reference information for a named symbol, including the source line supplied by the parsing flow.

Traceability:
- `add_reference`

#### FR-14: Call registration
The module shall register a named function call occurrence, including the source line supplied by the parsing flow.

Traceability:
- `call`

### Key Entities

#### Ident
An identifier/declarator entity used as the parsing result carrier for declaration-related functions. It participates in declarator parsing, function declaration handling, and K&R argument processing.

Relationships:
- populated by `parse_dcl`, `dcl`, `getident`, `dirdcl`, `parmdcl`
- consumed by `get_knr_args`, `declare`, and `declare_type`

#### Symbol
A symbol entity representing a named program element used for lookup and usage tracking.

Relationships:
- retrieved by `get_symbol`
- referenced or updated by `add_reference`
- used for call registration by `call`

#### Reference/Call usage record
A usage representation associated with a symbol name and source line as evidenced by reference-adding and call-recording functions.

Relationships:
- created or updated through `add_reference`
- created or updated through `call`
- discovered during `func_body`

#### Parameter list outcome
A parser result describing whether a function-like declarator has parameters and, where needed, how many.

Relationships:
- produced by `maybe_parm_list`
- contributes to `getident`, `dirdcl`, `parmdcl`, and K&R handling flow

#### Structured-type body span
A balanced source region representing a struct/union/enum-style body that must be skipped to continue declaration parsing correctly.

Relationships:
- consumed by `skip_struct`
- relevant to `parse_typedef` and general declaration parsing paths

## Success Criteria

1. The Rust module correctly parses typedef declarations through the same declaration flow as the C module, including declarators that require nested/direct declarator handling.
   - Verified by scenario-based tests derived from `parse_typedef`, `parse_dcl`, `dcl`, and `dirdcl`.

2. The Rust module correctly skips structured-type bodies when they appear in declaration contexts handled by this module, allowing parsing to resume at the correct post-body position.
   - Verified by tests traceable to `skip_struct`.

3. The Rust module correctly distinguishes and parses ordinary declarators, pointer declarators, and grouped direct declarators without losing identifier extraction.
   - Verified by tests traceable to `dcl`, `getident`, and `dirdcl`.

4. The Rust module correctly recognizes function parameter-list syntax and produces the parameter-related parsing outcome needed by subsequent control flow.
   - Verified by tests traceable to `maybe_parm_list`, `parmdcl`, `getident`, and `dirdcl`.

5. The Rust module correctly supports K&R-style argument declaration parsing in the declaration path where `maybe_knr` behavior applies.
   - Verified by tests traceable to `parse_dcl`, `get_knr_args`, and `declare`.

6. The Rust module correctly traverses function bodies sufficiently to detect named references and named calls encountered by this parser logic.
   - Verified by tests traceable to `func_body`.

7. For a referenced name encountered during function-body parsing, the Rust module performs symbol lookup and records reference usage with the supplied source line.
   - Verified by tests traceable to `get_symbol` and `add_reference`.

8. For a named call encountered during function-body parsing, the Rust module records a call occurrence with the supplied source line and preserves symbol association behavior.
   - Verified by tests traceable to `call` and `get_symbol`.

9. The Rust rewrite preserves the functional boundaries of this module cluster without requiring capabilities not evidenced in `src/parser.c`.
   - Verified by review of implemented behavior against the scoped functions in this specification.