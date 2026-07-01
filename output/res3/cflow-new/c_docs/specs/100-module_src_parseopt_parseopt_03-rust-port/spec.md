# spec.md

## Title

Rust Functional Specification for `module_src_parseopt_parseopt_03`

## Metadata

- Project: `cflow-new`
- Module: `module_src_parseopt_parseopt_03`
- Category: `module_cluster`
- Source file: `src/parseopt/parseopt.c`
- Rust branch: `100-module_src_parseopt_parseopt_03-rust-port`
- Generation date: `2026-06-17`

## Overview

This module provides command-line option parser lifecycle and lookup behavior centered on a `parseopt` context and its option definition records. The Rust rewrite must preserve the module’s observable behavior for:

- parser initialization,
- option parsing over argument vectors,
- parser state cleanup,
- option definition lookup by code and name,
- option presence queries,
- internal handling of version-related option behavior,
- access to grouped option definitions used by the parser.

The specification is limited to behavior evidenced by the analyzed source functions:

- `parseopt_init0`
- `parseopt_init`
- `parseopt_free`
- `parseopt_parse`
- `parseopt_getopt`
- `parseopt_optdef_by_code`
- `parseopt_optdef_by_name`
- `parseopt_is_set`
- `set_version`
- `_parseopt_optgroup`

## Feature Specification

### 1. Parser lifecycle management

The module must support creation-ready initialization of a parser context and subsequent initialization against a specific argument vector.

Two initialization stages are evidenced:

- a base initialization stage that prepares a `parseopt` instance for use;
- an argument-aware initialization stage that binds the parser to `argc`/`argv`.

The Rust version must preserve that separation in behavior, whether or not it is represented as the same internal structure as the C code.

### 2. Argument parsing

The module must parse command-line options from an argument vector associated with a parser context. Parsing is performed through parser methods that operate on the `parseopt` instance, including a convenience entry point that both initializes and parses for a provided `argc`/`argv`.

The Rust rewrite must preserve:

- parsing driven by the parser context,
- return of a status code from parsing operations,
- state updates that allow later lookup and “is set” queries.

### 3. Option definition lookup

The module must support retrieval of option definitions from the parser configuration by:

- option code,
- option name.

The Rust version must preserve lookup semantics that distinguish successful match from no match.

### 4. Option state query

The module must support querying whether an option identified by code has been set in the current parser state.

The Rust version must preserve this as a direct parser-state query after parsing.

### 5. Parser cleanup

The module must support cleanup of parser-associated state after initialization and/or parsing.

The Rust version must preserve safe repeated module usage through a defined cleanup/reset path corresponding to `parseopt_free`.

### 6. Version-option handling

The module contains explicit version-option handling through `set_version`. The Rust rewrite must preserve the observable effect of processing the version-related option within the parser flow, including its status-code based outcome.

### 7. Grouped option definition access

The module contains an internal accessor for option groups, `_parseopt_optgroup`, showing that parser behavior depends on indexed access to grouped option definition records.

The Rust rewrite must preserve support for grouped option definitions to the extent required for initialization, parsing, and lookup behavior.

## User Scenarios & Testing

### Scenario 1: Initialize a parser before binding arguments

A caller creates or supplies a parser context and performs base initialization before any parsing occurs.

Expected behavior:

- the parser enters a valid initialized state;
- the operation reports success or failure via return status;
- subsequent argument-aware initialization is accepted.

Tests should verify:

- base initialization succeeds for a valid parser context;
- a successfully base-initialized parser can proceed to full initialization and parsing.

### Scenario 2: Initialize a parser with command-line arguments and parse them

A caller provides `argc` and `argv` and requests parser initialization followed by parsing.

Expected behavior:

- the parser binds to the provided argument vector;
- parsing returns a status code;
- parser state is updated to reflect recognized option selections.

Tests should verify:

- `parseopt_init` accepts an argument vector and prepares parsing;
- `parseopt_parse` processes the bound arguments;
- `parseopt_getopt` performs the combined usage path for provided arguments.

### Scenario 3: Query whether a known option was set

After parsing, a caller asks whether an option identified by code was present.

Expected behavior:

- set options are reported as set;
- options not present are reported as not set;
- the query result reflects the current parser state.

Tests should verify:

- positive result for a parsed option code;
- negative result for a known but absent option code;
- stable results across repeated queries without reparsing.

### Scenario 4: Look up option metadata by code

A caller needs the definition record corresponding to an option code.

Expected behavior:

- an existing code returns its associated option definition;
- a non-existing code yields no definition.

Tests should verify:

- successful lookup for a configured option code;
- no-match behavior for an unknown code.

### Scenario 5: Look up option metadata by name

A caller needs the definition record corresponding to an option name.

Expected behavior:

- an existing name returns its associated option definition;
- a non-existing name yields no definition.

Tests should verify:

- successful lookup for a configured option name;
- no-match behavior for an unknown name.

### Scenario 6: Process a version-related option

A caller includes the version-related option among the command-line arguments.

Expected behavior:

- the parser executes the module’s version-option handling path;
- the operation returns the status outcome defined by that path;
- resulting parser behavior remains consistent with the C module.

Tests should verify:

- invocation of version handling during parsing when the relevant option is present;
- consistent return/status behavior for that option path.

### Scenario 7: Free parser-associated state after use

A caller completes parsing and then frees parser state.

Expected behavior:

- parser-associated resources/state are released or reset through the module’s cleanup path;
- cleanup can be called after successful initialization/parsing.

Tests should verify:

- cleanup after initialization only;
- cleanup after initialization and parsing;
- no residual parsed-state effect when a fresh parser instance is subsequently used.

## Requirements

### Functional Requirements

#### FR-1: Base parser initialization

The module shall provide a base initialization operation for a `parseopt` context corresponding to `parseopt_init0`.

Traceability: `parseopt_init0` in `src/parseopt/parseopt.c`.

#### FR-2: Argument-aware parser initialization

The module shall provide initialization that accepts an argument count and argument vector and binds them to parser state, corresponding to `parseopt_init`.

Traceability: `parseopt_init` in `src/parseopt/parseopt.c`.

#### FR-3: Parsing of bound arguments

The module shall provide parsing over the arguments bound to a parser context and return a status code representing the parsing outcome, corresponding to `parseopt_parse`.

Traceability: `parseopt_parse` in `src/parseopt/parseopt.c`.

#### FR-4: Combined initialize-and-parse entry point

The module shall provide an operation that accepts `argc` and `argv`, uses them with a parser context, and returns a parsing status, corresponding to `parseopt_getopt`.

Traceability: `parseopt_getopt` in `src/parseopt/parseopt.c`.

#### FR-5: Parser cleanup

The module shall provide cleanup for parser-associated state, corresponding to `parseopt_free`.

Traceability: `parseopt_free` in `src/parseopt/parseopt.c`.

#### FR-6: Option definition lookup by code

The module shall provide retrieval of an option definition by integer option code, with a distinct no-match outcome, corresponding to `parseopt_optdef_by_code`.

Traceability: `parseopt_optdef_by_code` in `src/parseopt/parseopt.c`; key entity `optdef`.

#### FR-7: Option definition lookup by name

The module shall provide retrieval of an option definition by option name, with a distinct no-match outcome, corresponding to `parseopt_optdef_by_name`.

Traceability: `parseopt_optdef_by_name` in `src/parseopt/parseopt.c`; key entity `optdef`.

#### FR-8: Option presence query by code

The module shall provide a query that reports whether an option identified by code is set in current parser state, corresponding to `parseopt_is_set`.

Traceability: `parseopt_is_set` in `src/parseopt/parseopt.c`.

#### FR-9: Version-option handling within parser flow

The module shall preserve version-option processing behavior represented by `set_version`, including status-code based completion.

Traceability: `set_version` in `src/parseopt/parseopt.c`.

#### FR-10: Grouped option definition access required by parser behavior

The module shall preserve parser support for indexed access to grouped option definitions as evidenced by `_parseopt_optgroup`, to the extent necessary for initialization, parsing, and lookup behavior.

Traceability: `_parseopt_optgroup` in `src/parseopt/parseopt.c`; key entities `parseopt`, `optdef`.

### Key Entities

#### `parseopt`

`parseopt` is the parser context entity. It owns or references the parser’s current configuration and state, including:

- association with the active argument vector for parsing,
- access to configured option definitions,
- recorded option-set state used by later queries,
- any grouped option-definition organization required by parser behavior.

Relationships:

- `parseopt` is the primary state carrier for all public module operations.
- `parseopt` is queried against `optdef` records by code and name.
- `parseopt` is updated by initialization and parsing, and consumed by cleanup.

Traceability: all listed `parseopt_*` functions and `_parseopt_optgroup`.

#### `optdef`

`optdef` is the option definition entity used to describe a recognized option and to support parser lookup and option-handling behavior.

Relationships:

- `optdef` instances are associated with a `parseopt` context.
- `optdef` records are retrievable by option code or option name.
- `optdef` participates in version-option handling and grouped option access.

Traceability: `set_version`, `_parseopt_optgroup`, `parseopt_optdef_by_code`, `parseopt_optdef_by_name`.

## Success Criteria

### SC-1: Initialization behavior parity

For valid parser inputs, the Rust module exposes both base initialization and argument-aware initialization behavior with success/failure outcomes corresponding to `parseopt_init0` and `parseopt_init`.

### SC-2: Parsing behavior parity

Given the same parser configuration and argument vector as the C module, the Rust module returns compatible parsing status outcomes and updates parser state so that later queries observe equivalent set/not-set results.

### SC-3: Lookup behavior parity by code

For configured option codes, lookup returns the corresponding option definition; for unknown codes, lookup reports no match, consistent with `parseopt_optdef_by_code`.

### SC-4: Lookup behavior parity by name

For configured option names, lookup returns the corresponding option definition; for unknown names, lookup reports no match, consistent with `parseopt_optdef_by_name`.

### SC-5: Option-set query parity

After parsing, the Rust module reports option presence by code consistently with the C module for both present and absent options, matching `parseopt_is_set`.

### SC-6: Combined entry-point parity

The Rust module supports the combined argument-driven parse path corresponding to `parseopt_getopt`, producing outcomes consistent with performing initialization and parsing through the module.

### SC-7: Cleanup correctness

After cleanup corresponding to `parseopt_free`, parser-associated state is no longer relied upon as active parsed state, and fresh parser usage proceeds without contamination from prior runs.

### SC-8: Version-option behavior preservation

When the version-related option path is exercised, the Rust module follows the same functional handling path and status outcome class as the C module’s `set_version` integration.

### SC-9: Support for grouped option organization

If parser configuration depends on grouped option definitions, the Rust module preserves sufficient grouped access behavior for initialization, parsing, and lookup to succeed as in the C module.