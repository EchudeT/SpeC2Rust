# Functional Specification: `module_src_main.c_26`

- **Project**: `cflow-new`
- **Module**: `module_src_main.c_26`
- **Category**: `module_cluster`
- **Source scope**: `src/main.c`
- **Rust target branch**: `089-module_src_main.c_26-rust-port`
- **Generation date**: `2026-06-17`

## 1. Overview

This module is the program entry and option-orchestration layer for the `cflow-new` executable. It is responsible for initializing process state, reading runtime configuration, interpreting option values expressed as strings, applying symbol-selection rules, expanding `~` in path-like inputs, and driving top-level execution from `main`.

The Rust rewrite must preserve the behavior evidenced by the analyzed functions in `src/main.c`, specifically:

- translation of individual characters into internal small-integer classes;
- matching textual option values against predefined option tables;
- parsing symbol override directives;
- parsing bounded numeric fragments from strings;
- parsing level/format control strings and advancing the caller’s input position;
- tilde expansion for user-relative paths;
- loading configuration from the runtime rc source;
- evaluating whether only global symbols should be considered;
- deciding whether a symbol is included under current filtering rules;
- terminating on allocation failure through the module’s fatal path;
- performing startup initialization;
- executing program startup and argument-driven top-level control flow.

This module is behavioral glue: it does not define the full symbol model itself, but it consumes external symbol data and global option state to decide how the program starts and what symbols are processed.

## 2. Feature Specification

### 2.1 Program startup and initialization

The module shall provide the executable entry behavior for the program.

It shall:

- initialize module/program state before main processing begins;
- install any startup defaults evidenced by `init`;
- enter top-level processing from `main(argc, argv)`;
- invoke configuration parsing as part of startup flow when required by the original behavior;
- terminate with an integer process status from `main`.

### 2.2 Runtime configuration parsing

The module shall support parsing of configuration from the program rc source through `parse_rc`.

This behavior includes:

- locating and reading the rc source according to existing program behavior;
- parsing recognized option directives through the module’s option parsing machinery;
- reporting success/failure through the `parse_rc` return value.

The Rust version must preserve rc parsing effects on program state that are observable through later option-dependent behavior.

### 2.3 Option value classification and lookup

The module shall support interpretation of textual option values using predefined option tables.

This includes:

- converting a character to an internal class/value with `CHAR_TO_SM`;
- matching a candidate string against an array/table of option descriptors with `find_option_type`;
- using these classifications during parsing of higher-level option strings.

The Rust rewrite must preserve the same matching outcomes for recognized values and non-matches.

### 2.4 Symbol override parsing

The module shall parse symbol override expressions via `symbol_override`.

This feature exists to alter symbol-related behavior using a string directive. The exact internal storage may change in Rust, but externally observable results must remain the same:

- recognized override text must be interpreted and applied to program state;
- malformed or unsupported override text must follow the original module’s failure behavior;
- later symbol inclusion decisions must reflect applied overrides.

### 2.5 Numeric fragment parsing

The module shall parse integer values from string input through `number`.

This parsing is bounded by:

- a caller-provided numeric base;
- a caller-provided maximum digit count;
- advancement of the caller’s input pointer as digits are consumed.

The Rust version must preserve:

- accepted digit forms for the supplied base;
- the numeric value produced from the consumed prefix;
- input-position advancement semantics.

### 2.6 Level string parsing

The module shall parse a level-control string through `parse_level_string`.

This feature includes:

- interpreting a textual level specification;
- updating module state according to the parsed level directives;
- returning an updated position pointer to the caller via the output pointer argument.

The Rust rewrite must preserve which prefixes are consumed and the resulting level-related configuration state.

### 2.7 Tilde expansion

The module shall support expansion of path strings beginning with `~` through `tildexpand`.

This includes:

- handling user-home based expansion for supported forms;
- returning a resulting path string for downstream use;
- preserving non-expandable inputs according to original behavior.

The Rust rewrite must preserve the observable expansion result for valid inputs and fallback behavior when expansion cannot be performed.

### 2.8 Symbol filtering decisions

The module shall expose symbol-selection behavior through:

- `globals_only()`: report whether current settings restrict processing to global symbols;
- `include_symbol(Symbol *sym)`: decide whether a specific symbol is included under current filters.

The Rust version must preserve the decision logic as driven by current configuration and symbol override state.

### 2.9 Fatal allocation handling

The module shall provide the allocation-failure terminal path via `xalloc_die`.

The Rust rewrite must preserve the module’s fail-fast behavior when unrecoverable allocation failure is routed to this handler.

## 3. User Scenarios & Testing

### Scenario 1: Program starts with default initialization
A user invokes the executable with standard arguments. The module initializes state, processes startup configuration, and enters the main program flow.

**Expected result**:
- initialization occurs before option-dependent behavior;
- `main` returns a process status;
- startup behavior matches the C module’s observable order and effects.

**Testing**:
- run with minimal arguments and compare exit status and emitted behavior against the C version;
- verify initialization-dependent defaults are present before symbol processing.

### Scenario 2: Configuration file affects runtime behavior
A user has an rc configuration that sets options affecting symbol inclusion or level handling. The program reads this rc source during startup.

**Expected result**:
- `parse_rc` applies recognized settings;
- subsequent symbol filtering and parsing behavior reflect the configuration;
- parse_rc success/failure matches the original module for the same rc content.

**Testing**:
- provide representative rc inputs with valid directives;
- verify resulting global/filter state through downstream behavior;
- test invalid rc content and confirm failure path matches original outcomes.

### Scenario 3: Option values are matched from text
A user supplies option text that must be resolved against predefined textual choices.

**Expected result**:
- recognized strings match the correct option type entry;
- unrecognized strings are reported or rejected according to current caller behavior;
- character classification used in parsing remains consistent.

**Testing**:
- test exact recognized values from each relevant option table;
- test near-matches, wrong length, and unknown strings;
- verify character classification on representative characters used by parsers.

### Scenario 4: Symbol override directive changes selection behavior
A user provides a symbol override string that changes which symbols are included or how they are treated.

**Expected result**:
- the directive is parsed and applied;
- later calls to `include_symbol` reflect the override;
- malformed override strings follow the original error behavior.

**Testing**:
- apply valid override strings and observe changed symbol inclusion decisions;
- test malformed directives and compare error handling to the C version.

### Scenario 5: Numeric parsing consumes only the intended prefix
A parser consumes a number embedded in a larger option string.

**Expected result**:
- only valid digits up to the count limit are consumed;
- the numeric value is returned correctly for the specified base;
- the caller-visible input pointer advances to the first unconsumed character.

**Testing**:
- test bases used by the module with valid and invalid digits;
- test zero digits, max-count truncation, and trailing non-digit characters;
- verify returned value and advanced pointer position.

### Scenario 6: Level string parsing consumes directives and returns remainder
A user supplies a level specification embedded in a longer string.

**Expected result**:
- the level portion is interpreted;
- module state is updated accordingly;
- the returned pointer identifies the remainder after the consumed prefix.

**Testing**:
- test valid level strings with and without trailing content;
- test edge cases around separators and partial directives;
- verify both resulting state and remainder pointer behavior.

### Scenario 7: Tilde-prefixed paths are expanded
A user or configuration source provides a path beginning with `~`.

**Expected result**:
- supported `~` forms expand to the appropriate home-based path;
- inputs not eligible for expansion preserve original semantics;
- failure to resolve expansion follows original fallback/error behavior.

**Testing**:
- test bare `~`, `~/...`, and user-specific forms if supported by the original behavior;
- test non-tilde paths and unresolved forms;
- compare returned path strings with C behavior.

### Scenario 8: Global-only mode restricts included symbols
Program options enable a mode where only global symbols should be processed.

**Expected result**:
- `globals_only()` reports the active restriction;
- `include_symbol(sym)` excludes symbols not permitted by that mode.

**Testing**:
- construct representative global and non-global `Symbol` instances as required by the wider program;
- verify inclusion decisions before and after enabling the relevant setting.

### Scenario 9: Allocation failure follows the fatal path
An unrecoverable allocation failure is routed to the module’s allocation failure handler.

**Expected result**:
- the process terminates through the module’s fatal behavior;
- no normal success return occurs from the failure path.

**Testing**:
- use fault injection or a controlled allocation hook in Rust tests where feasible;
- verify termination behavior matches the C module’s expectations.

## 4. Requirements

### 4.1 Functional Requirements

#### FR-1: Executable entry behavior
The module shall provide top-level executable control through `main`, including startup sequencing and process exit status behavior evidenced in `src/main.c`.

**Traceability**: `main`, `init`, `parse_rc`

#### FR-2: Initialization
The module shall initialize module/program state through `init` before dependent parsing and filtering behavior is used.

**Traceability**: `init`, `main`

#### FR-3: RC configuration parsing
The module shall parse runtime configuration through `parse_rc` and apply recognized configuration effects to program state.

**Traceability**: `parse_rc`

#### FR-4: Option table lookup
The module shall resolve textual option values against predefined option tables using string-and-length matching.

**Traceability**: `find_option_type`, `struct option_type`

#### FR-5: Character classification for parsing
The module shall convert a character to its internal parser classification/value as required by the module’s string parsers.

**Traceability**: `CHAR_TO_SM`

#### FR-6: Numeric substring parsing
The module shall parse an integer from the current input position for a specified base and digit-count bound, and advance the input cursor to the first unconsumed character.

**Traceability**: `number`

#### FR-7: Level string parsing
The module shall parse level-control strings, update the related configuration state, and return the remainder position to the caller.

**Traceability**: `parse_level_string`

#### FR-8: Symbol override application
The module shall parse and apply symbol override directives expressed as strings.

**Traceability**: `symbol_override`

#### FR-9: Home-directory path expansion
The module shall expand tilde-prefixed path strings in the forms supported by the original module.

**Traceability**: `tildexpand`, `passwd`

#### FR-10: Global-only state query
The module shall report whether the current configuration restricts processing to global symbols only.

**Traceability**: `globals_only`

#### FR-11: Symbol inclusion decision
The module shall decide whether a given symbol is included under the current configuration and override/filter state.

**Traceability**: `include_symbol`

#### FR-12: Fatal allocation failure handling
The module shall expose the module’s fatal allocation-failure handler.

**Traceability**: `xalloc_die`

### 4.2 Key Entities

#### `option_type`
A table-driven option descriptor used to map textual values to internal meanings during parsing. It is consumed by option lookup logic and underlies several module parsing paths.

**Relationships**:
- used by `find_option_type`;
- supports parsing of option-like strings in startup/configuration flows.

#### `parseopt` / `optdef`
Option parsing descriptor structures defined in `src/main.c` and used to represent supported command-line and/or rc options for the module’s parser machinery.

**Relationships**:
- participate in startup and configuration parsing driven from `main` and `parse_rc`;
- connect textual options to configuration changes.

#### `Symbol`
An externally defined symbol entity consumed by this module for filtering decisions.

**Relationships**:
- passed to `include_symbol`;
- affected by global-only state and symbol override configuration.

#### `passwd`
An external system account record type referenced by tilde expansion behavior.

**Relationships**:
- used conceptually by `tildexpand` for resolving user-home paths.

## 5. Success Criteria

### SC-1: Startup parity
For equivalent inputs, the Rust executable reaches the same top-level startup outcomes as the C module, including initialization-dependent behavior and process exit status categories.

**Traceability**: `main`, `init`

### SC-2: RC parsing parity
Given the same rc content, the Rust module returns the same success/failure result as `parse_rc` and produces the same observable configuration effects on later parsing and filtering behavior.

**Traceability**: `parse_rc`

### SC-3: Option lookup parity
For all recognized and unrecognized option strings used by this module, Rust lookup results match the C behavior, including length-sensitive matching.

**Traceability**: `find_option_type`, `option_type`

### SC-4: Numeric parser parity
For representative inputs across the bases and count limits used by the module, Rust returns the same numeric result and same remainder position as the C `number` function.

**Traceability**: `number`

### SC-5: Level parser parity
For representative valid and invalid level strings, Rust consumes the same prefix, produces the same resulting level configuration, and returns the same remainder position as the C module.

**Traceability**: `parse_level_string`

### SC-6: Symbol override parity
For representative override directives, Rust produces the same downstream symbol-selection behavior as the C module, including error handling for malformed directives.

**Traceability**: `symbol_override`, `include_symbol`

### SC-7: Tilde expansion parity
For supported tilde-prefixed inputs, Rust returns the same expanded or fallback path result as the C module.

**Traceability**: `tildexpand`

### SC-8: Symbol filtering parity
For representative `Symbol` inputs under the same configuration, Rust `globals_only` and `include_symbol` results match the C module.

**Traceability**: `globals_only`, `include_symbol`

### SC-9: Fatal-path parity
When the allocation failure path is intentionally invoked, Rust terminates through the corresponding fatal behavior rather than returning normally.

**Traceability**: `xalloc_die`