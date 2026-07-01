# spec.md

## Title

Functional Specification for `module_src_parser.c_31` Rust Port

## Metadata

- Project: `cflow-new`
- Module: `module_src_parser.c_31`
- Category: `module_cluster`
- Source file coverage: `src/parser.c`
- Target Rust branch: `094-module_src_parser.c_31-rust-port`
- Generation date: `2026-06-17`

## Overview

This module is responsible for parsing C declarative syntax and related source constructs needed to recognize declarations, function definitions, parameter lists, type aliases, structure blocks, and symbol references arising from function calls. It also updates symbol/reference state so later stages can use discovered definitions and call relationships.

The Rust rewrite must preserve the observable parsing behavior of this module within the parser subsystem, including:

- skipping over structure bodies when they are not further analyzed here,
- recognizing `typedef` declarations,
- parsing declarators and direct declarators,
- identifying function declarations versus function definitions,
- handling parameter declarations, including K&R-style argument declarations where supported by the source parser,
- consuming function bodies sufficiently to discover references/calls handled by this module,
- resolving and creating symbol records by name,
- recording references and call relationships with source line information.

## Scope

This specification covers the functionality evidenced by the following parser responsibilities in `src/parser.c`:

- declarator parsing,
- parameter list handling,
- function body traversal,
- K&R argument handling,
- declaration classification,
- type-oriented declaration handling,
- symbol lookup,
- reference creation,
- call recording.

It does not specify unrelated lexer behavior, full C semantic analysis, code generation, or external persistence.

## Feature Specification

### Feature 1: C declaration parsing

The module shall parse C declarations sufficiently to extract identifier-oriented information from declarators and declaration forms used by the parser.

This includes:

- consuming declaration syntax through the declarator layer,
- recognizing direct declarators,
- associating parsed declarators with an identifier record,
- supporting declaration contexts that may permit K&R-style parameter declarations.

The Rust version must preserve the same classification outcomes expected by the surrounding parser for declarations handled by this module.

Traceability:
- `parse_dcl`
- `dcl`
- `getident`
- `dirdcl`
- `declare`
- `declare_type`

### Feature 2: `typedef` recognition

The module shall recognize and process `typedef` declarations as a distinct declaration form.

The Rust version must preserve the parser behavior that treats `typedef` declarations differently from ordinary declarations when updating identifier/type-related state.

Traceability:
- `parse_typedef`

### Feature 3: Structure-body skipping

The module shall skip over structure or similar balanced declaration bodies when the parser needs to advance beyond them without producing detailed member-level output in this module.

The Rust version must preserve balanced consumption behavior so subsequent parsing resumes at the correct token after the skipped construct.

Traceability:
- `skip_struct`

### Feature 4: Parameter parsing

The module shall parse function parameter syntax and detect parameter-list presence when parsing function declarators.

This includes:

- parsing parameter declarators,
- determining whether a parenthesized construct is a parameter list,
- reporting parameter count information where this module does so,
- supporting old-style/K&R argument handling required by the parser.

Traceability:
- `parmdcl`
- `maybe_parm_list`
- `get_knr_args`

### Feature 5: Function-definition handling

The module shall recognize and process function definitions by advancing through their bodies and gathering reference/call information handled at this parsing level.

The Rust version must preserve behavior for:

- entering function-body parsing after a function definition is recognized,
- consuming the body correctly with balanced nesting,
- allowing symbol/reference updates during body parsing.

Traceability:
- `func_body`
- `declare`

### Feature 6: Symbol resolution and reference tracking

The module shall resolve symbols by name and record references associated with source lines.

The Rust version must preserve behavior for:

- looking up an existing symbol by name,
- creating or registering a reference entry when needed,
- returning symbol information for further parser use.

Traceability:
- `get_symbol`
- `add_reference`

### Feature 7: Call relationship recording

The module shall record function-call references identified during parsing, including the called name and source line.

The Rust version must preserve the module’s observable behavior for call registration as distinct parser-discovered references.

Traceability:
- `call`

## User Scenarios & Testing

### Scenario 1: Parsing an ordinary declaration

A parser client feeds tokens for a standard C declaration into this module. The module parses the declaration, extracts the identifier-oriented declarator information, and updates declaration-related parser state without misclassifying it as a function definition.

Expected support:
- declaration is consumed to the correct endpoint,
- identifier/declarator information is populated,
- no function body traversal occurs.

Traceability:
- `parse_dcl`
- `dcl`
- `dirdcl`
- `declare`

### Scenario 2: Parsing a `typedef`

A parser client encounters a `typedef` statement. The module processes it as a type-alias declaration rather than an ordinary object/function declaration.

Expected support:
- `typedef` form is recognized,
- type-related declaration handling is applied,
- later parsing can treat the declared name as type-related according to surrounding parser rules.

Traceability:
- `parse_typedef`
- `declare_type`

### Scenario 3: Skipping a structure declaration body

While parsing declarations, the parser encounters a structure body that this module does not need to analyze member-by-member. The module skips over the balanced structure contents and resumes parsing at the first token after the structure construct.

Expected support:
- nested balancing is respected,
- parser position after skipping is correct,
- later declarations remain parseable.

Traceability:
- `skip_struct`

### Scenario 4: Parsing a function declarator with parameters

A parser client provides a function declarator. The module determines whether a parameter list is present, parses parameters, and records parameter-related information needed by the parser.

Expected support:
- parameter list is detected,
- parameter declarators are consumed,
- parameter count/result is reported where applicable.

Traceability:
- `dirdcl`
- `parmdcl`
- `maybe_parm_list`

### Scenario 5: Parsing a K&R-style function definition

The parser encounters an old-style function definition with identifier list and trailing argument declarations. The module recognizes the form, collects K&R argument declarations, and continues into the function body.

Expected support:
- old-style argument list is accepted when allowed,
- subsequent argument declarations are parsed,
- function body parsing begins only after argument declaration handling completes.

Traceability:
- `get_knr_args`
- `declare`
- `func_body`

### Scenario 6: Traversing a function body and recording calls

A parser client enters a function definition body. As the body is traversed, the module detects call sites handled at this level and records references/calls with line numbers.

Expected support:
- body is consumed with correct balancing,
- symbol lookup and/or creation occurs by called name,
- call/reference records include line association.

Traceability:
- `func_body`
- `get_symbol`
- `add_reference`
- `call`

### Scenario 7: Looking up an already known symbol

During parsing, the module needs to resolve a symbol name already encountered earlier. The symbol is returned without duplicating symbol identity.

Expected support:
- same logical symbol name resolves consistently,
- callers can use the returned symbol for reference/call updates.

Traceability:
- `get_symbol`

## Requirements

### Functional Requirements

#### FR-1: Declarator consumption
The module shall consume C declarator syntax and determine identifier-oriented declaration structure for declarations handled in this source file.

Traceability:
- `parse_dcl`
- `dcl`
- `getident`
- `dirdcl`

#### FR-2: Identifier extraction
The module shall extract or resolve the identifier associated with a declarator when one is present and make it available to declaration-processing logic.

Traceability:
- `getident`
- `dcl`
- `dirdcl`

#### FR-3: `typedef` handling
The module shall recognize `typedef` declarations and process them through type-oriented declaration handling distinct from ordinary declaration processing.

Traceability:
- `parse_typedef`
- `declare_type`

#### FR-4: Structure skipping
The module shall skip balanced structure contents so parsing can continue after the structure construct without detailed structure-member analysis by this module.

Traceability:
- `skip_struct`

#### FR-5: Parameter-list detection
The module shall detect whether a function declarator is followed by a parameter list and report parameter-list information needed by the parser.

Traceability:
- `maybe_parm_list`
- `dirdcl`

#### FR-6: Parameter declaration parsing
The module shall parse function parameter declarators and integrate them into declarator/declaration processing.

Traceability:
- `parmdcl`
- `dirdcl`

#### FR-7: K&R argument declaration support
The module shall support old-style/K&R function argument declaration parsing where the parser indicates that such handling may apply.

Traceability:
- `parse_dcl`
- `get_knr_args`
- `declare`

#### FR-8: Function definition recognition and body handling
The module shall distinguish function definitions from non-defining declarations and, for definitions, consume the function body.

Traceability:
- `declare`
- `func_body`

#### FR-9: Balanced function body traversal
The module shall traverse function bodies in a way that preserves correct nesting/balancing so parser state resumes correctly after the body.

Traceability:
- `func_body`

#### FR-10: Symbol lookup by name
The module shall retrieve an existing symbol record corresponding to a provided name for use in reference and call processing.

Traceability:
- `get_symbol`

#### FR-11: Reference recording
The module shall add or register a reference associated with a symbol name and source line.

Traceability:
- `add_reference`

#### FR-12: Call recording
The module shall record a call occurrence using the called name and source line.

Traceability:
- `call`

### Key Entities

#### Entity 1: Identifier record (`Ident`)
An identifier record carries declaration-related information while parsing declarators, parameter declarations, K&R arguments, and function definitions.

Relationships:
- populated during declarator parsing,
- passed through declaration-processing functions,
- used when classifying declarations and function definitions.

Traceability:
- `parse_dcl`
- `dcl`
- `getident`
- `dirdcl`
- `parmdcl`
- `get_knr_args`
- `declare`
- `declare_type`

#### Entity 2: Symbol record (`Symbol`)
A symbol record represents a named symbol used for lookup, reference tracking, and call tracking.

Relationships:
- retrieved by name,
- referenced when recording source-line usage,
- used by call tracking.

Traceability:
- `get_symbol`
- `add_reference`
- `call`

#### Entity 3: Parameter count/result value
A parameter count/result value communicates whether parameter-list parsing found parameters and how many were recognized where this module reports such information.

Relationships:
- produced during possible parameter-list parsing,
- consumed by declarator/declaration logic.

Traceability:
- `maybe_parm_list`

#### Entity 4: Source line number
A source line number identifies where a reference or call was discovered during parsing.

Relationships:
- attached to reference creation,
- attached to call recording.

Traceability:
- `add_reference`
- `call`

#### Entity 5: Balance state
A balance-tracking state supports correct consumption of nested syntactic regions such as bodies or skipped structured constructs.

Relationships:
- used to ensure parser advancement across balanced regions,
- relevant to structure skipping and function-body traversal behavior.

Traceability:
- `struct balance_state` in `src/parser.c`
- `skip_struct`
- `func_body`

## Success Criteria

### SC-1: Declaration parsing correctness
For declaration inputs handled by this module, the Rust port produces the same declaration/function-definition classification outcomes as the C module at the module boundary.

Traceability:
- `parse_dcl`
- `declare`

### SC-2: Declarator identifier preservation
For declarators containing an identifier, the Rust port preserves identifier extraction behavior needed by downstream declaration processing.

Traceability:
- `getident`
- `dcl`
- `dirdcl`

### SC-3: `typedef` distinction
For inputs containing `typedef`, the Rust port treats the declaration through the module’s type-oriented path rather than the ordinary declaration path.

Traceability:
- `parse_typedef`
- `declare_type`

### SC-4: Balanced skipping correctness
For structure constructs with nested balanced content, the Rust port resumes parsing at the same post-construct position as the C module.

Traceability:
- `skip_struct`

### SC-5: Parameter parsing compatibility
For function declarators with parameter lists, the Rust port matches the C module in whether a parameter list is recognized and in consuming the same parameter declaration extent.

Traceability:
- `maybe_parm_list`
- `parmdcl`
- `dirdcl`

### SC-6: K&R handling compatibility
For old-style function definitions supported by the original parser, the Rust port matches the C module in accepting and consuming K&R argument declaration sections.

Traceability:
- `get_knr_args`
- `declare`

### SC-7: Function body consumption correctness
For function definitions with nested statement/block structure, the Rust port consumes the full body and resumes parsing at the same following token position as the C module.

Traceability:
- `func_body`

### SC-8: Symbol lookup consistency
For repeated lookup of the same known symbol name within one parse session, the Rust port returns a consistent symbol identity usable by reference/call processing.

Traceability:
- `get_symbol`

### SC-9: Reference recording fidelity
For each reference added through this module, the Rust port preserves name-based association and source-line association equivalent to the C module.

Traceability:
- `add_reference`

### SC-10: Call recording fidelity
For each call recorded through this module, the Rust port preserves the called-name and source-line information equivalent to the C module.

Traceability:
- `call`

## Non-Goals

The Rust port is not required by this specification to provide:

- a new public API beyond what is needed to replace this module in the existing parser,
- full C semantic validation beyond the behavior evidenced here,
- parsing or exposing structure members as a new feature,
- concurrency guarantees,
- serialization or persistence,
- error recovery mechanisms not evidenced by the source module.

## Acceptance Notes

Conformance should be assessed using parser-level tests that compare the Rust port against the C module on representative inputs covering:

- ordinary declarations,
- `typedef` declarations,
- nested structure declarations requiring skipping,
- function declarators with parameters,
- K&R-style function definitions,
- function bodies containing nested blocks and call sites,
- repeated symbol lookup and reference/call registration.