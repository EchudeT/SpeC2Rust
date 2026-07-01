# spec.md

## Title

Rust Functional Specification for `module_src_parser.c_31`

## Metadata

- Project: `cflow-new`
- Module: `module_src_parser.c_31`
- Category: `module_cluster`
- Source basis: `src/parser.c`
- Rust branch target: `094-module_src_parser.c_31-rust-port`
- Generation date: `2026-06-11`

## Overview

This module is responsible for parsing C declaration and function-related source constructs that are needed to identify symbols, recognize declarations, handle parameter forms, skip non-emitting type bodies, and record call/reference relationships.

The Rust rewrite must preserve the observable parsing behavior evidenced by the source module functions in `src/parser.c`, specifically for:

- skipping over `struct`/`union`/`enum` style type bodies when they are not themselves the output target of declaration parsing,
- parsing `typedef` and ordinary declarations,
- distinguishing direct declarators, nested declarators, and parameter declarators,
- recognizing possible parameter lists, including K&R-style function argument declarations,
- consuming function bodies sufficiently to find symbol references and calls,
- resolving or creating symbols by name,
- recording references and function calls with source line association.

This specification covers functional behavior only. It does not require reproducing C implementation details that are not externally observable from these parsing outcomes.

## Scope

### In Scope

The Rust module must implement the functional boundaries evidenced by these parser responsibilities:

- declaration parsing for identifiers and their declarator forms,
- typedef parsing,
- parameter list handling,
- K&R argument declaration handling,
- function body consumption for reference/call extraction,
- symbol lookup and creation used by this parsing flow,
- reference and call recording tied to names and line numbers.

### Out of Scope

The Rust module is not required by this specification to provide:

- new parsing features beyond those evidenced by the listed functions,
- new public APIs unrelated to the current parser responsibilities,
- serialization, persistence, concurrency guarantees, or FFI behavior,
- diagnostic behavior not evidenced by the module analysis.

## Source Evidence

Primary source evidence for this specification is `src/parser.c`, with behavior centered in the following functions:

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

## Feature Specification

### 1. Declaration Parsing

The module must parse C declarations sufficiently to identify declared names and their declaration form.

This includes:

- accepting declaration parsing entry points for ordinary declarations and typedefs,
- recognizing declaration components through declarator parsing,
- handling nested/direct declarators,
- extracting identifier information from declarators when present,
- supporting declaration parsing paths that may need to distinguish normal declaration contexts from possible K&R function declaration contexts.

The Rust version must preserve the ability to continue from declaration parsing into later symbol-oriented actions such as declaration recording or function parsing when the parsed construct represents a function.

### 2. Type Body Skipping

The module must be able to skip over structured type bodies that should be consumed syntactically without being recursively treated as ordinary declarations of interest in this parsing path.

The Rust version must support skipping such type definitions or bodies so that later parsing resumes at the correct token boundary.

### 3. Typedef Handling

The module must parse `typedef` declarations as a distinct declaration form.

The Rust version must preserve behavior needed to recognize typedef-based declarations and pass resulting identifier/type information into the same declaration-processing flow used by the source module.

### 4. Declarator and Identifier Extraction

The module must parse declarators deeply enough to:

- identify whether a declarator yields a named identifier,
- support direct declarator parsing,
- support nested declarator parsing,
- support parameter declarator parsing,
- return enough status information for callers to determine whether declaration parsing succeeded and whether a parameter-oriented path was encountered.

This behavior is evidenced by the interaction of `dcl`, `dirdcl`, `parmdcl`, and `getident`.

### 5. Parameter List Recognition

The module must recognize and consume possible function parameter lists.

This includes:

- detecting whether a declarator is followed by a parameter list,
- counting or otherwise preserving the number of parameters when that information is produced by the source behavior,
- handling parameter declarators within the list,
- supporting the ambiguity point where the syntax may represent either a parameter-name list or another declaration-related sequence.

### 6. K&R Argument Declaration Handling

The module must support the C K&R-style function declaration form evidenced by `get_knr_args` and the `maybe_knr` flow in declaration parsing.

The Rust version must:

- recognize when a function declarator may be followed by old-style argument declarations,
- parse those argument declarations sufficiently to associate them with the function declaration flow,
- preserve behavior needed for subsequent function body parsing and symbol handling.

### 7. Function Body Consumption and Reference Discovery

The module must consume function bodies sufficiently to locate name references and calls that occur within them.

The Rust version must preserve the parser’s role as a source-level extractor rather than a full semantic compiler:

- it must continue through a function body,
- it must identify references to names encountered in that body as evidenced by `add_reference`,
- it must identify function-call style uses as evidenced by `call`,
- it must attach the relevant source line to recorded references/calls.

### 8. Declaration Finalization

The module must finalize parsed declarations into symbol-related effects.

This includes:

- processing completed declarations through `declare`,
- processing type-oriented declarations through `declare_type`,
- handling function declarations in a way that can lead into parameter/K&R/body parsing as appropriate.

### 9. Symbol Resolution and Recording

The module must support symbol resolution by name and recording of references/calls.

The Rust version must preserve these functional behaviors:

- lookup of an existing symbol by textual name,
- creation or retrieval of a symbol when a reference is added,
- recording of a reference occurrence with a line number,
- recording of a call occurrence with a line number.

## User Scenarios & Testing

### Scenario 1: Parse a typedef declaration

A caller feeds tokens/source corresponding to a `typedef` declaration.

Expected module behavior:

- the declaration is recognized as a typedef form,
- the declared identifier is extracted if present,
- declaration finalization occurs through the typedef-capable path,
- parsing resumes after the end of the typedef declaration without mis-consuming following constructs.

### Scenario 2: Parse a declaration with nested declarators

A caller feeds a declaration that uses direct and nested declarator syntax.

Expected module behavior:

- the module traverses the declarator structure,
- the underlying identifier is recovered,
- the declaration result is returned to the declaration-processing flow,
- parsing status reflects success or failure consistently with the source behavior.

### Scenario 3: Parse a function declarator with a parameter list

A caller presents a function declaration or definition header with a parameter list.

Expected module behavior:

- the parameter list is detected,
- parameter declarators are consumed,
- parameter count information is produced where the source behavior produces it,
- the parser remains positioned correctly for either a trailing semicolon, further declaration material, or a function body.

### Scenario 4: Parse a K&R-style function definition

A caller provides a function definition using an identifier list followed by old-style parameter declarations and then a body.

Expected module behavior:

- the identifier list is recognized as a possible parameter-name list,
- subsequent K&R argument declarations are consumed,
- the parser transitions into function body parsing,
- body references and calls are still recorded.

### Scenario 5: Skip a structured type body inside declaration parsing

A caller presents a declaration containing a structured type body that must be consumed as syntax but not treated as a normal declarator target for this parsing step.

Expected module behavior:

- the structured body is skipped completely,
- surrounding declaration parsing continues correctly,
- subsequent identifiers or declarators are still parsed from the correct position.

### Scenario 6: Record a symbol reference inside a function body

While parsing a function body, the module encounters a name usage that constitutes a reference.

Expected module behavior:

- the module resolves or creates the symbol for that name,
- a reference occurrence is added,
- the source line of the occurrence is retained.

### Scenario 7: Record a function call inside a function body

While parsing a function body, the module encounters a function-call style use.

Expected module behavior:

- the module records the call against the callee name,
- the source line of the call is retained,
- normal body parsing continues after the call expression.

### Scenario 8: Lookup an existing symbol during parsing

During declaration or body parsing, the module needs to resolve a symbol by name.

Expected module behavior:

- existing symbols are found by textual name,
- if reference-recording flow requires a missing symbol to be materialized, that occurs through the reference path rather than requiring the caller to manage symbol creation separately.

## Requirements

### Functional Requirements

#### FR-1: Structured type body skipping
The module shall consume and skip structured type bodies encountered during declaration parsing when that body must not be parsed as an ordinary declaration target in the current flow.

**Traceability:** `skip_struct` in `src/parser.c`.

#### FR-2: Typedef parsing
The module shall recognize and parse `typedef` declarations and feed the resulting declaration information into declaration handling.

**Traceability:** `parse_typedef`, `declare_type` in `src/parser.c`.

#### FR-3: General declaration parsing
The module shall parse ordinary declarations, including declarator-driven extraction of declared identifiers.

**Traceability:** `parse_dcl`, `declare`, `dcl`, `getident` in `src/parser.c`.

#### FR-4: Direct and nested declarator support
The module shall support direct declarators and nested declarator forms sufficiently to determine the declared identifier and declaration shape required by the parser flow.

**Traceability:** `dcl`, `dirdcl`, `getident` in `src/parser.c`.

#### FR-5: Parameter declarator parsing
The module shall parse parameter declarators used in function declaration contexts.

**Traceability:** `parmdcl`, `dirdcl`, `maybe_parm_list` in `src/parser.c`.

#### FR-6: Possible parameter list recognition
The module shall detect and consume possible parameter lists associated with function declarators and provide resulting parameter count information where produced by the original behavior.

**Traceability:** `maybe_parm_list` in `src/parser.c`.

#### FR-7: K&R argument declaration support
The module shall recognize and parse K&R-style function argument declaration sequences when a function declaration context may use that form.

**Traceability:** `parse_dcl`, `declare`, `get_knr_args` in `src/parser.c`.

#### FR-8: Function body parsing for symbol-use extraction
The module shall consume function bodies sufficiently to identify symbol references and call sites occurring within the body.

**Traceability:** `func_body`, `add_reference`, `call` in `src/parser.c`.

#### FR-9: Declaration finalization
The module shall finalize parsed declarations into symbol/type handling actions consistent with whether the declaration is ordinary or type-oriented.

**Traceability:** `declare`, `declare_type` in `src/parser.c`.

#### FR-10: Symbol lookup by name
The module shall provide symbol lookup by textual name for parser use during declaration and reference processing.

**Traceability:** `get_symbol` in `src/parser.c`.

#### FR-11: Reference recording
The module shall record a named symbol reference together with the source line of occurrence, resolving or creating the symbol as needed by the original behavior.

**Traceability:** `add_reference` in `src/parser.c`.

#### FR-12: Call recording
The module shall record a function call occurrence by callee name together with the source line of occurrence.

**Traceability:** `call` in `src/parser.c`.

### Key Entities

#### Ident
A declaration-focused entity used to carry identifier information through declarator parsing, declaration processing, parameter parsing, and K&R argument parsing.

**Relationships:**
- populated by declarator parsing functions,
- consumed by declaration finalization functions,
- used when parsing K&R argument declarations for a function context.

**Traceability:** `parse_dcl`, `dcl`, `getident`, `dirdcl`, `parmdcl`, `get_knr_args`, `declare`, `declare_type`.

#### Symbol
A symbol-record entity representing a named program element that can be looked up and associated with references and calls.

**Relationships:**
- retrieved through symbol lookup,
- returned by reference-adding logic,
- targeted by call/reference recording.

**Traceability:** `get_symbol`, `add_reference`, `call`.

#### Parameter count/result state
A small result-carrying entity represented in the source as integer output/state used to communicate parameter-list outcomes and declaration-context decisions.

**Relationships:**
- produced during possible parameter list parsing,
- used by declaration and function parsing flow.

**Traceability:** `getident`, `maybe_parm_list`, `parse_dcl`, `declare`.

#### Balance state
A parser state structure used by body/declaration scanning logic to keep syntactic balance while consuming nested source constructs.

This specification treats it as internal parser state, but its functional significance is that balanced consumption must be preserved while skipping or traversing complex syntax.

**Traceability:** `struct balance_state` declarations in `src/parser.c`; functionally relevant to body/skipping behavior in this parser area.

## Success Criteria

### SC-1: Typedef handling correctness
Given source containing typedef declarations, the Rust module correctly recognizes them as typedefs and completes parsing without losing the declared identifier or corrupting subsequent parse position.

**Traceability:** `parse_typedef`, `declare_type`.

### SC-2: Declarator identifier recovery
Given declarations with direct and nested declarators, the Rust module recovers the same declared identifier presence/absence decisions as the source module’s declarator flow.

**Traceability:** `dcl`, `getident`, `dirdcl`.

### SC-3: Parameter list handling
Given function declarators with parameter lists, the Rust module detects and consumes the parameter list and preserves parameter-count outcomes produced by the source behavior.

**Traceability:** `parmdcl`, `maybe_parm_list`.

### SC-4: K&R function support
Given K&R-style function definitions, the Rust module recognizes the old-style argument declaration section and proceeds into body parsing without misparsing the definition boundary.

**Traceability:** `get_knr_args`, `parse_dcl`, `declare`.

### SC-5: Structured type body skipping
Given declarations containing structured type bodies to be skipped, the Rust module resumes parsing at the same logical post-body position as the source module.

**Traceability:** `skip_struct`.

### SC-6: Function body reference extraction
Given function bodies containing name references, the Rust module records references with the correct referenced name and associated source line.

**Traceability:** `func_body`, `add_reference`.

### SC-7: Function call extraction
Given function bodies containing function-call syntax, the Rust module records call occurrences with the correct callee name and associated source line.

**Traceability:** `func_body`, `call`.

### SC-8: Symbol resolution behavior
When reference processing encounters a name, the Rust module can retrieve an existing symbol by that name and can supply a symbol result from reference addition consistent with the source flow.

**Traceability:** `get_symbol`, `add_reference`.

### SC-9: Declaration-path completeness
For declaration inputs handled by this module’s evidenced functions, the Rust rewrite covers ordinary declarations, typedef declarations, parameter declarators, and function-related declaration paths without requiring unsupported fallback behavior.

**Traceability:** `parse_typedef`, `parse_dcl`, `dcl`, `dirdcl`, `parmdcl`, `declare`, `declare_type`.