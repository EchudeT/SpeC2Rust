# spec.md

## Title

Rust Functional Specification for `module_src_parseopt_parseopt_03`

## Metadata

- Project: `cflow-new`
- Module: `module_src_parseopt_parseopt_03`
- Category: `module_cluster`
- Source basis: `src/parseopt/parseopt.c`
- Target branch: `100-module_src_parseopt_parseopt_03-rust-port`
- Generation date: `2026-06-11`

## Overview

This module provides option-parser lifecycle and query behavior centered on a `parseopt` state object and its associated option definitions. The evidenced public behavior includes:

- initialization of parser state,
- parsing of command-line arguments,
- cleanup of parser-owned resources,
- lookup of option definitions by code or name,
- testing whether an option has been set,
- built-in handling for a version-related option,
- access to option-group definitions maintained by parser state.

The Rust rewrite must preserve this module’s functional boundaries as evidenced by the source module API and its data relationships. The rewrite must not introduce additional externally visible capabilities beyond these behaviors.

## Feature Specification

### 1. Parser lifecycle

The module manages a parser state object through explicit initialization, use, and cleanup.

Required lifecycle behavior:

- A parser state can be initialized in a base form using `parseopt_init0`.
- A parser state can be initialized with argument-vector context using `parseopt_init`.
- The parser state can be reused for parsing through `parseopt_parse` and `parseopt_getopt` according to the current or supplied argument inputs.
- Parser-owned resources can be released through `parseopt_free`.

The Rust version must preserve the distinction between base initialization and initialization that binds argument input.

### 2. Command-line option parsing

The module parses options from command-line arguments using parser state plus option definitions already associated with that state.

Required parsing behavior:

- `parseopt_parse` performs parsing using the parser state’s configured arguments.
- `parseopt_getopt` supports parse execution with explicit `argc`/`argv` input.
- Parsing behavior is definition-driven: results are determined against the available `optdef` entries attached to the parser state.
- Built-in version-option handling is present through `set_version`, indicating that at least one recognized option triggers version-related state or action.

The Rust version must support parsing outcomes that can later be queried through lookup and set-status functions.

### 3. Option definition access

The module supports retrieval of option definitions from parser state in multiple ways.

Required access behavior:

- `parseopt_optdef_by_code` returns the option definition corresponding to a numeric option code.
- `parseopt_optdef_by_name` returns the option definition corresponding to an option name.
- `_parseopt_optgroup` provides indexed access to an option-group entry maintained by parser state.

The Rust version must preserve these forms of definition access for behaviorally equivalent internal or module-visible use, consistent with the original module boundary.

### 4. Option state query

After initialization and parsing, the module supports checking whether an option was set.

Required query behavior:

- `parseopt_is_set` reports whether the option identified by a code is set in the parser state.

The Rust version must ensure that this query reflects the parser state produced by parsing and any built-in option handling present in this module.

## User Scenarios & Testing

### Scenario 1: Initialize parser state without binding arguments yet

A caller prepares a `parseopt` object whose option definitions are already configured, then performs base initialization.

Expected behavior:

- Base initialization succeeds when parser state is valid for setup.
- The parser is placed into a usable state for later argument binding or parsing operations.
- No parsing result is reported yet.

Test coverage:

- Initialize a valid parser state using the base initializer.
- Verify that subsequent parser operations are accepted in the normal lifecycle order.

### Scenario 2: Initialize parser state with command-line arguments and parse

A caller provides `argc` and `argv` during initialization and then parses.

Expected behavior:

- Argument-bearing initialization stores or prepares the supplied argument context.
- Parsing consumes that argument context against the parser’s option definitions.
- Recognized options become queryable by code and/or name through the parser state.

Test coverage:

- Initialize with a representative argument vector containing recognized options.
- Invoke parse.
- Verify that affected options can be found through definition lookup and set-status query.

### Scenario 3: Parse using explicit argument input

A caller uses `parseopt_getopt` with an explicit `argc`/`argv` pair instead of relying only on previously stored parser arguments.

Expected behavior:

- The module accepts explicit argument input for parsing through this call path.
- Parsing results are reflected in the parser state.
- Set-status queries after the call match the parsed arguments.

Test coverage:

- Provide explicit arguments containing one or more recognized options.
- Verify that the corresponding option codes report as set.

### Scenario 4: Retrieve an option definition by code

A caller needs to inspect parser metadata for a known option code.

Expected behavior:

- If a matching option definition exists, it is returned.
- If no matching definition exists, the operation indicates absence.

Test coverage:

- Query with a code known to exist in the parser’s option definitions.
- Query with a code that is not defined.
- Verify correct presence/absence behavior.

### Scenario 5: Retrieve an option definition by name

A caller needs to locate an option definition using its option name.

Expected behavior:

- If a matching option name exists, it is returned.
- If no matching name exists, the operation indicates absence.

Test coverage:

- Query with a defined option name.
- Query with an undefined name.
- Verify correct presence/absence behavior.

### Scenario 6: Check whether an option was set

After parsing, a caller checks whether a particular option code was selected.

Expected behavior:

- The query reports set for options recognized in the parsed input.
- The query reports not set for options not present in the parsed input.

Test coverage:

- Parse an argument vector containing one known option and omitting another.
- Verify positive and negative results for `parseopt_is_set`.

### Scenario 7: Built-in version option handling

A caller passes the version-related option recognized by this module.

Expected behavior:

- The parse path routes that option through built-in version handling associated with `set_version`.
- The parser state and observable query behavior remain consistent after this handling.

Test coverage:

- Parse input containing the version-related option defined by the parser configuration.
- Verify that the parse completes according to module behavior and that related state/query outcomes are consistent.

### Scenario 8: Cleanup after use

A caller frees parser-associated resources after initialization and/or parsing.

Expected behavior:

- Cleanup releases resources owned by the parser state for this module’s responsibilities.
- Cleanup is valid after successful initialization and use.

Test coverage:

- Initialize, parse, then free.
- Initialize, free without parsing.
- Verify no remaining parser-owned resources are required for correct lifecycle completion.

## Requirements

### Functional Requirements

#### FR-1: Base parser initialization
The module shall initialize a `parseopt` state into a usable parser state through `parseopt_init0`.

Traceability:
- `parseopt_init0` in `src/parseopt/parseopt.c`

#### FR-2: Argument-bound parser initialization
The module shall initialize a `parseopt` state with supplied command-line argument context through `parseopt_init`.

Traceability:
- `parseopt_init` in `src/parseopt/parseopt.c`

#### FR-3: Parser resource cleanup
The module shall provide cleanup of parser-associated resources through `parseopt_free`.

Traceability:
- `parseopt_free` in `src/parseopt/parseopt.c`

#### FR-4: Parsing using parser-held arguments
The module shall parse options from arguments associated with the parser state through `parseopt_parse`.

Traceability:
- `parseopt_parse` in `src/parseopt/parseopt.c`

#### FR-5: Parsing using explicit arguments
The module shall parse options from explicitly supplied `argc`/`argv` input through `parseopt_getopt`.

Traceability:
- `parseopt_getopt` in `src/parseopt/parseopt.c`

#### FR-6: Lookup by option code
The module shall support retrieval of an option definition from parser state by numeric code through `parseopt_optdef_by_code`.

Traceability:
- `parseopt_optdef_by_code` in `src/parseopt/parseopt.c`

#### FR-7: Lookup by option name
The module shall support retrieval of an option definition from parser state by option name through `parseopt_optdef_by_name`.

Traceability:
- `parseopt_optdef_by_name` in `src/parseopt/parseopt.c`

#### FR-8: Option set-status query
The module shall report whether an option identified by code is set in the parser state through `parseopt_is_set`.

Traceability:
- `parseopt_is_set` in `src/parseopt/parseopt.c`

#### FR-9: Built-in version-option handling
The module shall include built-in handling for a version-related option through the `set_version` behavior used by the parser.

Traceability:
- `set_version` in `src/parseopt/parseopt.c`

#### FR-10: Option-group indexed access
The module shall provide indexed access to option-group definitions associated with the parser state through `_parseopt_optgroup`.

Traceability:
- `_parseopt_optgroup` in `src/parseopt/parseopt.c`

### Key Entities

#### `parseopt`
The central parser state entity.

Role evidenced by API usage:

- passed to all lifecycle functions,
- holds or references option definitions,
- carries parse state used by parsing and querying,
- serves as the source for option-group access and set-status checks.

Relationships:

- contains or references multiple `optdef` entries,
- is mutated by initialization and parsing operations,
- is queried by lookup and set-status functions.

Traceability:
- `parseopt_init0`
- `parseopt_init`
- `parseopt_free`
- `parseopt_parse`
- `parseopt_getopt`
- `parseopt_optdef_by_code`
- `parseopt_optdef_by_name`
- `parseopt_is_set`
- `_parseopt_optgroup`

#### `optdef`
The option-definition entity associated with parser state.

Role evidenced by API usage:

- represents a single option definition,
- is retrievable by option code,
- is retrievable by option name,
- participates in built-in option handling,
- may be organized into groups addressable by parser state.

Relationships:

- belongs to or is referenced from `parseopt`,
- is the target of lookup operations by code and name,
- is the definition unit used during parse-time interpretation.

Traceability:
- `set_version`
- `_parseopt_optgroup`
- `parseopt_optdef_by_code`
- `parseopt_optdef_by_name`

## Success Criteria

### SC-1: Lifecycle completeness
The Rust module provides behaviorally equivalent support for initialization, parsing, and cleanup corresponding to `parseopt_init0`, `parseopt_init`, `parseopt_parse`, `parseopt_getopt`, and `parseopt_free`.

Measured by:

- tests that exercise both initialization paths,
- tests that execute both parsing entry points,
- tests that complete cleanup after use.

### SC-2: Correct option lookup
The Rust module returns matching option definitions for existing codes and names, and indicates absence for non-existing ones, corresponding to `parseopt_optdef_by_code` and `parseopt_optdef_by_name`.

Measured by:

- passing tests for successful and unsuccessful lookup by code,
- passing tests for successful and unsuccessful lookup by name.

### SC-3: Correct set-status reporting
The Rust module reports whether an option has been set in a manner consistent with parsed input, corresponding to `parseopt_is_set`.

Measured by:

- passing tests showing set status for options present in input,
- passing tests showing unset status for options absent from input.

### SC-4: Version-option behavior preserved
The Rust module preserves the module’s built-in version-option handling evidenced by `set_version`.

Measured by:

- a passing test in which the version-related option is parsed through the module’s normal parse path,
- consistent parser/query state after handling.

### SC-5: Option-group access preserved
The Rust module preserves indexed access to parser-maintained option-group definitions corresponding to `_parseopt_optgroup`.

Measured by:

- tests verifying valid indexed retrieval against configured parser groups,
- tests verifying defined absence or failure behavior for out-of-range access, consistent with the source module’s behavior.

### SC-6: State coherence across APIs
The Rust module maintains coherent parser state across initialization, parsing, definition lookup, and set-status queries.

Measured by:

- end-to-end tests in which options recognized during parsing are retrievable by lookup and reflected in set-status queries using the same parser instance.