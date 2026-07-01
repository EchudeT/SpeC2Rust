# spec.md

## Title

Rust Port Functional Specification: `module_src_main.c_26`

## Overview

This module is the program-entry and top-level option/configuration processing unit for `cflow-new`, implemented from `src/main.c`. It is responsible for:

- initializing process-wide state,
- reading configuration from the runtime control file,
- parsing and applying selected textual option values,
- expanding `~`-prefixed paths for configuration file handling,
- determining symbol inclusion policy for later processing stages,
- handling fatal allocation failure,
- and driving the main program flow from process startup to exit.

The Rust rewrite on branch `089-module_src_main.c_26-rust-port` must preserve the observable behavior of these responsibilities as evidenced by the functions and data structures identified for this module.

## Scope

In scope for this module:

- program startup orchestration,
- configuration file parsing entry behavior,
- option token interpretation helpers,
- symbol override parsing behavior,
- level-string parsing behavior,
- home-directory expansion for configuration paths,
- symbol inclusion filtering decisions,
- initialization and fatal out-of-memory termination behavior.

Out of scope:

- inventing new CLI features,
- adding new configuration formats,
- adding concurrency or async behavior,
- introducing public APIs not evidenced by this module,
- extending symbol filtering semantics beyond what current functions imply.

## Source Traceability

Primary source file:

- `src/main.c`

Primary traced functions:

- `CHAR_TO_SM`
- `find_option_type`
- `symbol_override`
- `number`
- `parse_level_string`
- `tildexpand`
- `parse_rc`
- `globals_only`
- `include_symbol`
- `xalloc_die`
- `init`
- `main`

Primary traced entities:

- `option_type`
- `parseopt`
- `optdef`
- `Symbol`
- `passwd`

---

## 1. Feature Specification

### 1.1 Module Purpose

This module provides the top-level control plane for the program. It accepts configuration inputs from command startup and runtime control-file sources, translates textual option representations into internal settings, and determines which symbols should be processed by later parts of the application.

### 1.2 Functional Behavior to Preserve

#### A. Program initialization and startup flow

The module must initialize required global/process-wide state before normal operation begins and must execute the application’s main startup flow, including configuration handling and final program exit behavior.

Traceability:

- `init`
- `main`

#### B. Runtime control-file parsing entry

The module must support parsing of a runtime control file through a dedicated entry point. This includes resolving any configuration-file path components that require `~` expansion before opening or processing the file.

Traceability:

- `parse_rc`
- `tildexpand`

#### C. Textual option identification

The module must support matching textual option names or option fragments against known option definitions, using a helper that determines the matching option type from a provided option table and input string segment.

Traceability:

- `find_option_type`
- `option_type`

#### D. Symbol override parsing

The module must support parsing a textual symbol-override specification and applying the resulting override effect to internal configuration/state used later for symbol handling.

Traceability:

- `symbol_override`

#### E. Numeric parsing within textual option strings

The module must support parsing fixed-count numeric components from strings in a specified base, advancing the source pointer as parsing consumes characters.

Traceability:

- `number`

#### F. Level-string parsing

The module must support parsing a textual “level” specification string and returning the unconsumed remainder pointer when applicable. This behavior is part of interpreting structured option values.

Traceability:

- `parse_level_string`
- `number`
- `find_option_type`

#### G. Home-directory path expansion

The module must support expansion of path strings beginning with `~`, including behavior that relies on user account information. For inputs that do not require expansion, behavior must remain consistent with the source module’s intent of returning a usable path string for downstream configuration parsing.

Traceability:

- `tildexpand`
- `passwd`

#### H. Symbol inclusion filtering

The module must expose the current symbol-selection policy through:

- a query indicating whether only global symbols are to be considered, and
- a predicate deciding whether a given `Symbol` is included under current settings.

Traceability:

- `globals_only`
- `include_symbol`
- `Symbol`

#### I. Fatal allocation failure termination

The module must provide the fatal allocation-failure handler used by allocation utilities and must terminate execution in that condition.

Traceability:

- `xalloc_die`

---

## 2. User Scenarios & Testing

### Scenario 1: Program startup with default initialization

A user launches the program normally. The module initializes application state and executes the main program flow without requiring external callers to set up internal state first.

Expected support:

- initialization occurs before dependent behavior,
- main flow reaches normal program execution or clean termination.

Traceability:

- `init`
- `main`

Suggested tests:

- invoke the Rust port’s main entry under a minimal valid startup condition;
- verify that initialization-dependent operations do not fail due to missing setup.

### Scenario 2: Startup reads runtime configuration

A user has a runtime control file in the expected location. During startup, the module attempts to parse that configuration source.

Expected support:

- runtime configuration parsing is invoked,
- the parsing entry returns success/failure consistently with source behavior.

Traceability:

- `parse_rc`
- `main`

Suggested tests:

- run with a present configuration file and verify successful parsing path;
- run with configuration unavailable or unusable and verify returned/propagated failure behavior matches source expectations.

### Scenario 3: Configuration path uses `~`

A user stores the runtime control file under a home-relative path such as `~/.something` or an equivalent user-home form supported by the C module.

Expected support:

- `~`-prefixed path text is expanded before file access,
- expansion uses account/home-directory information,
- resulting path is suitable for configuration parsing.

Traceability:

- `tildexpand`
- `parse_rc`

Suggested tests:

- provide a home-relative path and verify the resolved path points into the user home directory;
- provide a path without `~` and verify it remains usable without unintended transformation.

### Scenario 4: Textual option name is matched to a known option

A configuration or command-processing path provides an option name fragment. The module determines which option definition it refers to.

Expected support:

- known option text maps to the correct option type,
- unknown or non-matching text is rejected consistently.

Traceability:

- `find_option_type`
- `option_type`

Suggested tests:

- match representative known option names from the option tables in `src/main.c`;
- verify non-matching input produces the same class of failure/non-match outcome as the C module.

### Scenario 5: Structured option contains numeric fields

A textual option includes embedded numeric fields with fixed width or limited count and a defined base. The parser consumes those fields and advances through the string.

Expected support:

- valid digits are parsed correctly,
- only the allowed number of characters are consumed,
- the caller-visible input pointer advances accordingly.

Traceability:

- `number`

Suggested tests:

- parse decimal and non-decimal examples consistent with source usage;
- verify the pointer/remainder position after parsing;
- verify invalid characters stop or fail parsing as in the source behavior.

### Scenario 6: Level specification string is parsed

A user provides a level-specification string in configuration or option context. The module parses the recognized portion and identifies the remaining unparsed suffix when relevant.

Expected support:

- valid level strings are accepted and applied,
- returned remainder pointer corresponds to the unconsumed input tail.

Traceability:

- `parse_level_string`

Suggested tests:

- parse valid level strings reflected by the option tables and parser logic;
- verify remainder handling on strings with additional trailing content;
- verify malformed strings are rejected or handled consistently with source behavior.

### Scenario 7: Symbol override text changes symbol handling policy

A configuration entry specifies symbol overrides. The module parses that string and updates the active override behavior.

Expected support:

- override syntax accepted by the C module is recognized,
- resulting state affects later symbol inclusion decisions.

Traceability:

- `symbol_override`
- `include_symbol`

Suggested tests:

- apply a known-valid override string and verify downstream inclusion behavior changes accordingly;
- verify malformed override text follows source-compatible error behavior.

### Scenario 8: Caller queries global-only filtering mode

Another part of the program needs to know whether current settings restrict processing to global symbols only.

Expected support:

- the query reflects current configuration state accurately.

Traceability:

- `globals_only`

Suggested tests:

- evaluate before and after configuration/settings that toggle global-only behavior.

### Scenario 9: A symbol is tested for inclusion

A downstream stage asks whether a specific `Symbol` should be included in processing under the current configuration.

Expected support:

- inclusion decision reflects global-only mode and any configured symbol-selection behavior evident in the C module.

Traceability:

- `include_symbol`
- `globals_only`
- `Symbol`

Suggested tests:

- test representative symbols that differ in visibility/eligibility attributes used by the source logic;
- verify decisions change appropriately when filtering configuration changes.

### Scenario 10: Memory allocation failure occurs

A fatal memory allocation failure is encountered through allocation support tied to this module.

Expected support:

- the module terminates execution through its fatal allocation handler,
- behavior is non-recoverable.

Traceability:

- `xalloc_die`

Suggested tests:

- inject allocation-failure behavior in a controlled test harness and verify fatal termination path is invoked.

---

## 3. Requirements

### 3.1 Functional Requirements

#### FR-1: Initialization
The Rust module shall provide initialization behavior equivalent to `init`, establishing the process-wide state required by later configuration parsing, filtering, and main-flow execution.

Traceability:

- `init`

#### FR-2: Main entry orchestration
The Rust module shall provide the program-entry behavior equivalent to `main`, including invocation ordering needed for initialization, configuration handling, and overall process termination/result reporting.

Traceability:

- `main`
- `init`
- `parse_rc`

#### FR-3: Runtime control-file parsing
The Rust module shall provide a runtime control-file parsing entry equivalent to `parse_rc`, returning success/failure in a way consistent with the source module’s observable behavior.

Traceability:

- `parse_rc`

#### FR-4: Home-relative path expansion
The Rust module shall support expansion of `~`-prefixed path strings for configuration-related file handling, using user account/home-directory information where required.

Traceability:

- `tildexpand`
- `passwd`

#### FR-5: Option-type lookup
The Rust module shall support lookup of an option definition from a textual input segment against an option table, equivalent in matching behavior to `find_option_type`.

Traceability:

- `find_option_type`
- `option_type`

#### FR-6: Symbol override parsing
The Rust module shall parse and apply symbol-override text equivalent to `symbol_override`, affecting later symbol filtering decisions.

Traceability:

- `symbol_override`
- `include_symbol`

#### FR-7: Fixed-count numeric parsing
The Rust module shall parse numeric content from a caller-supplied string pointer in a specified base and character count, advancing the input position equivalently to `number`.

Traceability:

- `number`

#### FR-8: Level-string parsing
The Rust module shall parse level specification strings and provide remainder-position reporting equivalent to `parse_level_string`.

Traceability:

- `parse_level_string`
- `number`
- `find_option_type`

#### FR-9: Character-to-state/class conversion helper
The Rust port shall preserve the behavior of the `CHAR_TO_SM` helper where it affects parsing or option interpretation semantics in this module.

Traceability:

- `CHAR_TO_SM`

#### FR-10: Global-only mode query
The Rust module shall provide a query equivalent to `globals_only` that reports whether current configuration restricts processing to global symbols only.

Traceability:

- `globals_only`

#### FR-11: Symbol inclusion decision
The Rust module shall provide symbol inclusion logic equivalent to `include_symbol`, evaluating a `Symbol` against the currently active filtering and override state.

Traceability:

- `include_symbol`
- `globals_only`
- `symbol_override`

#### FR-12: Fatal allocation failure handling
The Rust module shall provide fatal allocation failure handling equivalent to `xalloc_die`, terminating the program in allocation-exhaustion conditions rather than attempting recovery.

Traceability:

- `xalloc_die`

### 3.2 Key Entities

#### `option_type`
Represents an option-definition record used for mapping textual option names or fragments to internal option meaning. It is consumed by option lookup and level-string parsing support.

Relationships:

- used by `find_option_type`,
- referenced by parser paths such as `parse_level_string`.

#### `parseopt`
Represents a parsed-option descriptor used by the module’s option parsing tables and processing paths.

Relationships:

- associated with `optdef`,
- participates in the module’s option/configuration interpretation logic in `src/main.c`.

#### `optdef`
Represents an option-definition entry that describes recognized options and how they are parsed or applied.

Relationships:

- associated with `parseopt`,
- underlies the option-processing structures present in the module.

#### `Symbol`
Represents a symbol candidate considered for inclusion or exclusion by the module’s filtering logic.

Relationships:

- input to `include_symbol`,
- affected by global-only mode and symbol override state.

#### `passwd`
Represents user-account information consulted during `~` expansion.

Relationships:

- used conceptually by `tildexpand` to resolve home-directory paths.

---

## 4. Success Criteria

### SC-1: Startup behavior parity
For equivalent startup inputs, the Rust port initializes state and executes the top-level startup flow without omitting any source-observable stage handled by `init`, `parse_rc`, and `main`.

Traceability:

- `init`
- `parse_rc`
- `main`

### SC-2: Configuration path expansion parity
For representative `~`-prefixed and non-`~` paths used by runtime configuration parsing, the Rust port resolves paths in a manner functionally equivalent to `tildexpand`.

Traceability:

- `tildexpand`
- `parse_rc`

### SC-3: Option lookup parity
For representative valid and invalid option name inputs drawn from the module’s option tables, the Rust port returns the same match/non-match outcomes as `find_option_type`.

Traceability:

- `find_option_type`
- `option_type`

### SC-4: Numeric parser parity
For representative numeric substrings, bases, and count limits used by this module, the Rust port produces the same parsed value and input-consumption position as `number`.

Traceability:

- `number`

### SC-5: Level-string parser parity
For representative valid and invalid level strings, including strings with trailing unconsumed text, the Rust port matches `parse_level_string` in acceptance and remainder-position behavior.

Traceability:

- `parse_level_string`

### SC-6: Symbol override effect parity
Applying equivalent symbol-override strings in the Rust port changes subsequent symbol inclusion decisions the same way as in the source module.

Traceability:

- `symbol_override`
- `include_symbol`

### SC-7: Symbol filtering parity
For representative `Symbol` instances and relevant configuration states, the Rust port returns the same `globals_only` status and `include_symbol` decisions as the C module.

Traceability:

- `globals_only`
- `include_symbol`

### SC-8: Fatal allocation behavior parity
When allocation failure is injected into code paths using this module’s fatal handler, the Rust port terminates through the equivalent of `xalloc_die` rather than continuing execution.

Traceability:

- `xalloc_die`

### SC-9: No unsupported feature expansion
The Rust rewrite does not introduce new externally observable configuration semantics, symbol-selection modes, or startup behaviors beyond those evidenced by `src/main.c`.

Traceability:

- entire module scope from `src/main.c`