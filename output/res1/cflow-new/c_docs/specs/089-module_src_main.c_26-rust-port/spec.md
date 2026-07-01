# spec.md

## Title

Rust Functional Specification for `module_src_main.c_26`

## Metadata

- Project: `cflow-new`
- Source module: `src/main.c`
- Module category: `module_cluster`
- Target branch: `089-module_src_main.c_26-rust-port`
- Generation date: `2026-06-11`

## Overview

This module is the program entry and top-level option-processing layer for the project. It is responsible for initializing process-wide behavior, reading configuration from an rc file, parsing and applying option values from strings, handling symbol-selection rules, expanding `~` in path-like inputs, exposing filtering helpers used by later processing, and driving program startup through `main`.

The Rust rewrite must preserve the observable behavior evidenced by the source module’s named functions and local option-related structures. In particular, it must continue to:

- classify and parse option text,
- interpret level and symbol override strings,
- load configuration through the rc parsing path,
- provide inclusion filtering for symbols,
- perform startup initialization and fatal allocation failure handling,
- execute the same top-level program flow coordinated by `main`.

## Feature Specification

### 1. Process startup orchestration

The module provides the executable’s top-level control flow. It performs initialization, incorporates configuration, processes command-line-related option state, and enters the main program path.

The Rust version must implement equivalent startup sequencing represented by `init`, `parse_rc`, and `main`, including the same ordering-sensitive behavior visible through configuration loading and option application.

### 2. Option classification and matching

The module contains logic for determining the type of an option token and matching a candidate string against available option descriptors.

The Rust version must preserve behavior equivalent to:

- converting characters into a normalized classification used by option parsing,
- matching textual option names against defined option-type entries,
- using option definition tables to decide how option text should be interpreted.

This behavior is evidenced by `CHAR_TO_SM`, `find_option_type`, and the local option descriptor structures.

### 3. Parsing of numeric and level-related option values

The module parses numeric fragments from strings and interprets level-selection strings into internal option state.

The Rust version must preserve the same functional behavior for:

- reading digits from a string in a requested base with a count limit,
- consuming level strings and applying the resulting option state,
- returning or updating any remaining unconsumed text as required by the original parsing contract.

This behavior is evidenced by `number` and `parse_level_string`.

### 4. Symbol override and symbol filtering behavior

The module supports symbol-selection customization through explicit override strings and later inclusion testing against symbols.

The Rust version must preserve:

- parsing and applying symbol override directives from text,
- evaluating whether a given symbol should be included,
- respecting any global-only mode that changes filtering behavior.

This behavior is evidenced by `symbol_override`, `include_symbol`, and `globals_only`.

### 5. Configuration file handling with home-directory expansion

The module reads runtime configuration from an rc source and expands `~`-prefixed paths before use.

The Rust version must preserve:

- rc parsing entry behavior,
- path expansion for strings beginning with `~` using user-home lookup semantics consistent with the C module’s supported cases,
- failure/success outcomes observable through `parse_rc`.

This behavior is evidenced by `tildexpand` and `parse_rc`, with `passwd` referenced for user-home lookup support.

### 6. Fatal allocation failure path

The module exposes a dedicated fatal handler for memory-allocation failure.

The Rust version must preserve an equivalent unrecoverable error path for allocation failure situations routed through the module’s public behavior boundary represented by `xalloc_die`.

## User Scenarios & Testing

### Scenario 1: Program startup with default initialization

A user launches the program with ordinary arguments. The startup layer initializes global state, attempts rc parsing, and continues into main execution.

**Expected behavior**
- Initialization occurs before normal processing.
- Rc parsing is attempted through the module’s configuration path.
- The program proceeds through the same top-level execution path as the C module.

**Suggested tests**
- Invoke the Rust binary in a minimal configuration and verify successful startup.
- Verify that initialization-dependent behavior is present before later option use.
- Verify that rc parsing is reached during startup.

### Scenario 2: Configuration path contains `~`

A user relies on an rc location or related path string that begins with `~` or user-home notation supported by the source module.

**Expected behavior**
- The path is expanded to a home-directory-based path before rc processing.
- Expansion behavior matches the supported forms from the C module.
- Parsing continues using the expanded path.

**Suggested tests**
- Supply a `~`-prefixed path and verify home expansion.
- Where supported by the original module, test both current-user and named-user expansion forms.
- Verify that non-`~` paths are left unchanged in meaning.

### Scenario 3: Option text is matched against defined option types

A user supplies option text that must be resolved against known option definitions.

**Expected behavior**
- The option name is matched against available option types.
- The correct option category is selected for subsequent parsing.
- Unknown or non-matching text follows the same failure or non-match behavior as the C module.

**Suggested tests**
- Exercise known option names and verify correct option-type selection.
- Exercise non-matching names and verify consistent rejection or fallback behavior.
- Confirm case and length handling consistent with the original matching logic.

### Scenario 4: Numeric parsing within option values

A user provides an option value containing numeric content interpreted with a specific base and limited character count.

**Expected behavior**
- Digits are parsed from the current string position.
- Parsing respects the specified base.
- Parsing respects the specified count bound.
- String-consumption behavior matches the original contract.

**Suggested tests**
- Parse decimal and non-decimal inputs where supported by callers.
- Verify correct stopping at invalid digits or count exhaustion.
- Verify pointer/remaining-text behavior through caller-observable outcomes.

### Scenario 5: Level string controls output or processing level

A user provides a level-selection string in an option or configuration entry.

**Expected behavior**
- The level string is parsed and applied to internal option state.
- Any remaining unparsed suffix is made available exactly as the original function contract requires.
- Invalid or partial forms behave consistently with the C module.

**Suggested tests**
- Provide valid level strings and verify resulting state changes.
- Provide strings with valid prefixes plus trailing text and verify returned remainder behavior.
- Provide invalid forms and verify matching error-handling behavior.

### Scenario 6: Symbol override affects later symbol inclusion

A user specifies symbol override text, then the program later decides whether specific symbols should be included.

**Expected behavior**
- Override directives alter symbol-selection behavior.
- `include_symbol` reflects the effects of prior override processing.
- Global-only mode affects inclusion decisions where applicable.

**Suggested tests**
- Apply an override string, then evaluate inclusion for matching and non-matching symbols.
- Verify behavior changes when global-only mode is active.
- Confirm stable behavior for symbols unaffected by overrides.

### Scenario 7: Allocation failure reaches the fatal path

A severe memory-allocation failure occurs during module-directed processing.

**Expected behavior**
- The module enters its fatal allocation failure path.
- Execution terminates or fails in the same unrecoverable manner expected by the original module.

**Suggested tests**
- Under controlled fault injection, force allocation failure in a path using this handler.
- Verify fatal termination semantics and non-success outcome.

## Requirements

### Functional Requirements

#### FR-1: Startup control
The module shall provide the executable’s top-level startup behavior, including initialization and transition into main execution flow, as evidenced by `init` and `main` in `src/main.c`.

#### FR-2: Rc parsing entry
The module shall provide an rc parsing operation that reads and applies configuration through the same behavioral entry represented by `parse_rc` in `src/main.c`.

#### FR-3: Home-directory expansion
The module shall expand `~`-prefixed path strings for configuration-related use cases with behavior traceable to `tildexpand` and the referenced `passwd`-based home lookup support in `src/main.c`.

#### FR-4: Option-token classification
The module shall classify characters for option parsing and use that classification in option interpretation behavior traceable to `CHAR_TO_SM` in `src/main.c`.

#### FR-5: Option-type lookup
The module shall match candidate option text against defined option-type entries with behavior traceable to `find_option_type` and the module’s option descriptor tables in `src/main.c`.

#### FR-6: Numeric fragment parsing
The module shall parse numeric fragments from a string according to caller-specified base and count limits, consistent with `number` in `src/main.c`.

#### FR-7: Level string parsing
The module shall parse level-selection strings and apply the resulting option state, including any caller-visible remainder semantics, consistent with `parse_level_string` in `src/main.c`.

#### FR-8: Symbol override application
The module shall parse and apply symbol override text that affects later symbol-selection decisions, consistent with `symbol_override` in `src/main.c`.

#### FR-9: Global-only query
The module shall expose whether global-only filtering is active through behavior consistent with `globals_only` in `src/main.c`.

#### FR-10: Symbol inclusion decision
The module shall decide whether a `Symbol` is included for subsequent processing, using the module’s configured filtering state, consistent with `include_symbol` in `src/main.c`.

#### FR-11: Fatal allocation failure handling
The module shall provide a fatal allocation failure path consistent with `xalloc_die` in `src/main.c`.

### Key Entities

#### `option_type`
An option descriptor entity used to identify recognized option names and their parsing category. It is used by option-type lookup and level/option parsing paths. Traceable to the local `struct option_type` usages and `find_option_type`.

#### `parseopt`
A local option-parsing descriptor grouping behavior needed to interpret particular options. It participates in the module’s option table-driven parsing flow. Traceable to the anonymous `struct parseopt` entries in `src/main.c`.

#### `optdef`
A local option-definition entity paired with parsing descriptors to define recognized options and their handling. Traceable to the anonymous `struct optdef` entries in `src/main.c`.

#### `Symbol`
An externally defined symbol entity evaluated by this module’s inclusion filter. The module does not define the symbol itself here, but it consumes it through `include_symbol`.

#### `passwd`
An externally defined user-account entity used indirectly for home-directory expansion behavior. The module relies on it for `~` expansion support but does not define it locally.

### Entity Relationships

- `option_type` entries are searched by option text to determine how incoming strings should be interpreted.
- `parseopt` and `optdef` participate in the module’s option table definitions and drive parsing behavior coordinated by startup and configuration paths.
- Symbol override state produced from text parsing influences later decisions made for `Symbol` instances by `include_symbol`.
- Home-directory expansion may consult `passwd`-derived user information before rc parsing consumes a path.

## Success Criteria

1. **Startup parity**: Running the Rust rewrite through normal program entry performs initialization, rc parsing entry, and main execution flow in the same externally observable order as the C module. Traceable to `init`, `parse_rc`, and `main`.

2. **Option matching parity**: For a representative set of known and unknown option strings from the module’s option tables, the Rust rewrite returns the same match/non-match outcomes as the C module. Traceable to `find_option_type`, `CHAR_TO_SM`, `parseopt`, `optdef`, and `option_type`.

3. **Numeric parsing parity**: For representative inputs covering valid digits, invalid digits, different bases, and count limits, the Rust rewrite produces the same parsed value and string-consumption behavior as the C module. Traceable to `number`.

4. **Level parsing parity**: For representative valid, partial, and invalid level strings, the Rust rewrite produces the same resulting option effects and remainder behavior as the C module. Traceable to `parse_level_string`.

5. **Symbol filtering parity**: After applying the same symbol override strings, the Rust rewrite returns the same inclusion decisions as the C module for representative `Symbol` inputs, including behavior with global-only mode enabled or disabled. Traceable to `symbol_override`, `include_symbol`, and `globals_only`.

6. **Tilde expansion parity**: For representative `~`-based and non-`~` path inputs supported by the source module, the Rust rewrite yields the same expansion results or failure behavior as the C module. Traceable to `tildexpand` and `parse_rc`.

7. **Rc parsing parity**: Given the same rc content and environment assumptions, the Rust rewrite’s rc parsing entry returns the same success/failure outcome and applies equivalent configuration effects observable through later module behavior. Traceable to `parse_rc`.

8. **Fatal failure parity**: Under controlled allocation-failure injection, the Rust rewrite reaches the same fatal, non-success behavior boundary as the C module. Traceable to `xalloc_die`.