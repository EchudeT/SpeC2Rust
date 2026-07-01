# spec.md

## Title

Rust Functional Specification for `module_src_parseopt_parseopt_03`

## Metadata

- Project: `cflow-new`
- Module: `module_src_parseopt_parseopt_03`
- Category: `module_cluster`
- Source file: `src/parseopt/parseopt.c`
- Target branch: `100-module_src_parseopt_parseopt_03-rust-port`
- Generation date: `2026-06-17`

## Overview

This module provides command-line option parser lifecycle and lookup behavior centered on a `parseopt` parser state and its `optdef` option definitions.

The Rust rewrite must implement the same functional boundary evidenced by the source module:

- initialize parser state,
- prepare and register built-in parser behavior needed during initialization,
- parse command-line arguments,
- expose parsed-option query operations,
- provide option-definition lookup by code and by name,
- report whether a given option has been set,
- release parser-owned resources.

The module also includes built-in handling for a version-related option through `set_version`, and internal access to option-group entries through `_parseopt_optgroup`.

## Scope

### In scope

The Rust version must cover the behavior represented by these module entry points:

- `parseopt_init0`
- `parseopt_init`
- `parseopt_free`
- `parseopt_parse`
- `parseopt_getopt`
- `parseopt_optdef_by_code`
- `parseopt_optdef_by_name`
- `parseopt_is_set`

It must also preserve module-internal behavior evidenced by:

- `set_version`
- `_parseopt_optgroup`

### Out of scope

The Rust version must not introduce requirements beyond the source evidence, including:

- new public parser capabilities,
- new option syntax or semantics,
- thread-safety guarantees,
- serialization formats,
- FFI contracts,
- recovery features beyond the original parse lifecycle,
- performance or benchmark targets not stated by the module.

## Feature Specification

### 1. Parser initialization

The module supports two initialization stages:

1. base initialization of a parser object (`parseopt_init0`);
2. initialization with command-line inputs (`parseopt_init`).

The Rust version must support both stages as distinct functional behaviors.

Base initialization must prepare a parser object into a usable state for later parsing and lookup operations. Initialization with arguments must bind the parser state to an argument vector and make the parser ready for parsing.

Initialization behavior must include any built-in option setup evidenced by the module, including support needed for the version-related option handler.

### 2. Argument parsing

The module parses command-line arguments associated with a parser state.

The Rust version must support:

- parsing using previously initialized parser state (`parseopt_parse`);
- a convenience flow that initializes against an argument vector and performs option processing (`parseopt_getopt`).

Parsing must update parser state so that later lookup and query operations reflect which options were recognized and set.

### 3. Option definition lookup

The module allows retrieval of option definitions from the parser state by:

- numeric or symbolic option code (`parseopt_optdef_by_code`);
- option name text (`parseopt_optdef_by_name`).

The Rust version must preserve these two lookup modes and their dependence on parser-held option definitions.

### 4. Set-state query

The module exposes a query to determine whether an option identified by code has been set (`parseopt_is_set`).

The Rust version must preserve this observable behavior after parsing.

### 5. Resource release

The module provides explicit parser cleanup (`parseopt_free`).

The Rust version must ensure parser-owned resources associated with initialization and parsing are released when cleanup is invoked or when the Rust ownership model drops the parser state, while preserving the original lifecycle semantics at the functional level.

### 6. Built-in version option handling

The module contains a dedicated version-option handler (`set_version`).

The Rust version must preserve the module behavior that a version-related option can be handled through parser option-processing machinery, updating parser-visible state and/or return behavior consistently with the original module.

### 7. Internal option-group access

The module includes internal indexed access to option-group definitions (`_parseopt_optgroup`).

The Rust rewrite must preserve any functionality that depends on parser-maintained option groups, to the extent this is required for equivalent initialization, parsing, and lookup behavior.

## User Scenarios & Testing

### Scenario 1: Initialize a parser before use

A caller creates a parser state and performs base initialization before supplying arguments.

**Expected support:**

- base initialization succeeds for a valid parser object;
- the parser enters a usable state for later initialization with arguments, parsing, and lookups;
- built-in parser behaviors required by the module are available after initialization.

**Test focus:**

- initialize an empty parser state;
- verify subsequent parser operations can be invoked in valid sequence;
- verify no stale parse results are present immediately after initialization.

### Scenario 2: Initialize with command-line arguments and parse

A caller has `argc/argv` style inputs and wants the parser configured and ready to process them.

**Expected support:**

- parser initialization with arguments accepts the provided command-line vector;
- a subsequent parse consumes parser input according to defined options;
- parser state reflects recognized options after parsing.

**Test focus:**

- initialize with representative command lines;
- run parse;
- verify expected options are marked as set;
- verify option definitions remain queryable after parsing.

### Scenario 3: One-call option processing flow

A caller wants a convenience entry point that combines parser setup with option processing.

**Expected support:**

- the convenience flow accepts a parser and argument vector;
- it performs the same effective parser preparation and option processing lifecycle exposed by the module;
- its result is suitable for the same post-parse queries as the staged flow.

**Test focus:**

- compare results of `parseopt_getopt` against the staged sequence using `parseopt_init` plus `parseopt_parse`;
- verify option set-state and definition lookups are consistent between both paths.

### Scenario 4: Retrieve an option definition by code

A caller needs metadata for a known option code.

**Expected support:**

- lookup by code returns the matching option definition when present;
- absent codes are reported as not found.

**Test focus:**

- query known option codes after initialization;
- query unknown codes;
- confirm returned definitions correspond to the parser’s registered options.

### Scenario 5: Retrieve an option definition by name

A caller needs metadata for an option identified by textual name.

**Expected support:**

- lookup by name returns the matching option definition when present;
- absent names are reported as not found.

**Test focus:**

- query known option names;
- query unknown names;
- verify name-based lookup resolves the same definitions as code-based lookup for equivalent options.

### Scenario 6: Check whether an option was set

After parsing, a caller wants to know whether a specific option occurred in the command line.

**Expected support:**

- querying by code reports set/not-set based on parse results;
- repeated queries are stable unless parser state is reinitialized or reparsed.

**Test focus:**

- parse inputs where an option is present and absent;
- verify `is_set` changes accordingly;
- verify querying an unknown or unset code does not report a false positive.

### Scenario 7: Handle version-related option behavior

A caller includes the module’s version-related option in the command line.

**Expected support:**

- the parser recognizes and routes the option through the built-in version handler;
- resulting parser behavior matches the original module’s externally visible outcome for that option path.

**Test focus:**

- include the version-related option in input;
- verify the parser takes the expected branch and records or returns the expected result as evidenced by the original behavior.

### Scenario 8: Release parser resources

A caller finishes using the parser and releases its resources.

**Expected support:**

- cleanup is safe after successful initialization and/or parsing;
- cleanup returns the parser lifecycle to a released state without leaving queryable active parse state.

**Test focus:**

- initialize, parse, and free;
- initialize and free without parsing;
- confirm no resource-retention behavior remains observable after release.

## Requirements

### Functional Requirements

#### FR-1: Base parser initialization

The module shall provide a base initialization operation for a parser state, corresponding to `parseopt_init0`.

**Traceability:** `parseopt_init0` in `src/parseopt/parseopt.c`.

#### FR-2: Initialization with argument vector

The module shall provide initialization that binds a parser state to an `argc/argv` command-line input set, corresponding to `parseopt_init`.

**Traceability:** `parseopt_init` in `src/parseopt/parseopt.c`.

#### FR-3: Parse execution from initialized state

The module shall provide a parse operation that processes arguments already associated with the parser state, corresponding to `parseopt_parse`.

**Traceability:** `parseopt_parse` in `src/parseopt/parseopt.c`.

#### FR-4: Combined setup-and-process option flow

The module shall provide a convenience operation that accepts a parser and `argc/argv` inputs and performs the module’s integrated option-processing flow, corresponding to `parseopt_getopt`.

**Traceability:** `parseopt_getopt` in `src/parseopt/parseopt.c`.

#### FR-5: Option-definition lookup by code

The module shall provide lookup of an option definition by option code within a parser state, corresponding to `parseopt_optdef_by_code`.

**Traceability:** `parseopt_optdef_by_code` in `src/parseopt/parseopt.c`.

#### FR-6: Option-definition lookup by name

The module shall provide lookup of an option definition by textual name within a parser state, corresponding to `parseopt_optdef_by_name`.

**Traceability:** `parseopt_optdef_by_name` in `src/parseopt/parseopt.c`.

#### FR-7: Query whether an option is set

The module shall provide a query that reports whether an option identified by code has been set in parser state, corresponding to `parseopt_is_set`.

**Traceability:** `parseopt_is_set` in `src/parseopt/parseopt.c`.

#### FR-8: Parser cleanup

The module shall provide parser cleanup functionality for releasing parser-associated resources, corresponding to `parseopt_free`.

**Traceability:** `parseopt_free` in `src/parseopt/parseopt.c`.

#### FR-9: Built-in version option handling

The module shall preserve built-in handling for a version-related option through parser option-processing behavior, corresponding to `set_version`.

**Traceability:** `set_version` in `src/parseopt/parseopt.c`.

#### FR-10: Option-group dependent behavior preservation

The module shall preserve parser functionality that depends on internal option-group access by index, as evidenced by `_parseopt_optgroup`, to the extent required for equivalent initialization, parsing, and lookup outcomes.

**Traceability:** `_parseopt_optgroup` in `src/parseopt/parseopt.c`.

### Key Entities

#### `parseopt`

Primary parser-state entity.

Functional role evidenced by the module:

- owns parser lifecycle across initialization, parsing, lookup, set-state querying, and cleanup;
- holds or references command-line inputs for parsing;
- holds or references option definitions available for lookup;
- stores parse results needed for option set-state queries;
- participates in internal option-group access.

**Traceability:** referenced throughout `parseopt_init0`, `parseopt_init`, `parseopt_free`, `parseopt_parse`, `parseopt_getopt`, `parseopt_optdef_by_code`, `parseopt_optdef_by_name`, `parseopt_is_set`, `_parseopt_optgroup`, `set_version`.

#### `optdef`

Option-definition entity associated with a parser.

Functional role evidenced by the module:

- represents an individual option known to the parser;
- can be retrieved by code or by name;
- may participate in option groups;
- may be supplied to handler logic such as version-option processing.

**Traceability:** referenced throughout `set_version`, `_parseopt_optgroup`, `parseopt_optdef_by_code`, `parseopt_optdef_by_name`, and parser initialization/processing functions.

#### Relationship: `parseopt` ↔ `optdef`

A `parseopt` instance manages or references a collection of `optdef` entries used for:

- parser initialization,
- argument parsing,
- option lookup by code,
- option lookup by name,
- determining set-state by option code,
- internal grouping behavior.

**Traceability:** `parseopt_init0`, `parseopt_init`, `parseopt_parse`, `parseopt_getopt`, `parseopt_optdef_by_code`, `parseopt_optdef_by_name`, `parseopt_is_set`, `_parseopt_optgroup`.

## Success Criteria

### SC-1: Initialization lifecycle parity

Given a valid parser state, the Rust module can perform base initialization and argument-vector initialization in the same supported lifecycle stages as the source module.

**Measured by:**

- tests invoking base initialization alone;
- tests invoking initialization with `argc/argv`;
- both paths leave the parser usable for subsequent supported operations.

**Traceability:** `parseopt_init0`, `parseopt_init`.

### SC-2: Parse result observability parity

For representative command lines using known options, parsing updates parser state so that post-parse queries reflect whether options were encountered.

**Measured by:**

- after parsing, `is_set` returns true for present options and false for absent ones;
- results are stable across repeated queries without reinitialization.

**Traceability:** `parseopt_parse`, `parseopt_is_set`.

### SC-3: Convenience flow parity

The combined option-processing flow produces parser-observable results consistent with the staged initialization-plus-parse flow for the same inputs.

**Measured by:**

- equivalent option set-state outcomes between `parseopt_getopt` and `parseopt_init` + `parseopt_parse`;
- equivalent lookup visibility of option definitions after both flows.

**Traceability:** `parseopt_getopt`, `parseopt_init`, `parseopt_parse`, `parseopt_optdef_by_code`, `parseopt_optdef_by_name`, `parseopt_is_set`.

### SC-4: Lookup parity by code

Known option codes can be resolved to definitions, and unknown codes are reported as not found.

**Measured by:**

- positive tests for known codes;
- negative tests for absent codes.

**Traceability:** `parseopt_optdef_by_code`, `optdef`, `parseopt`.

### SC-5: Lookup parity by name

Known option names can be resolved to definitions, and unknown names are reported as not found.

**Measured by:**

- positive tests for known names;
- negative tests for absent names;
- consistency checks between code-based and name-based retrieval of the same option.

**Traceability:** `parseopt_optdef_by_name`, `parseopt_optdef_by_code`, `optdef`, `parseopt`.

### SC-6: Version-option behavior preservation

When the version-related option is supplied through the parser’s option-processing path, the Rust module follows the same externally observable handling path as the source module.

**Measured by:**

- test coverage of the version-option input path;
- verification of matching parse outcome, handler invocation effect, or parser-visible state change as evidenced from the original module.

**Traceability:** `set_version`, `parseopt_parse` and/or `parseopt_getopt`.

### SC-7: Cleanup lifecycle support

After parser use, cleanup can be performed from supported states without preventing correct earlier behavior.

**Measured by:**

- successful cleanup after initialization only;
- successful cleanup after initialization and parsing;
- no further active parse-state behavior is observed after release in test-controlled usage.

**Traceability:** `parseopt_free`, `parseopt`.

### SC-8: Internal option-group dependent behavior is not regressed

Any parser behavior in this module that relies on grouped option definitions continues to function equivalently in the Rust rewrite.

**Measured by:**

- tests covering parser setups that exercise grouped option definitions where present in the source-driven test corpus;
- no regression in initialization, parsing, or lookup outcomes attributable to grouped-option handling.

**Traceability:** `_parseopt_optgroup`, `parseopt_init0`, `parseopt_init`, `parseopt_parse`, `parseopt_optdef_by_code`, `parseopt_optdef_by_name`.

## Acceptance Notes

- The Rust rewrite should be judged by behavioral equivalence of the module boundary evidenced in `src/parseopt/parseopt.c`.
- Where the original C code expresses behavior through internal helper functions, only externally observable effects required by the public module operations are mandatory unless the helper itself supports another evidenced required behavior.
- No undocumented expansion of parser features is part of acceptance.