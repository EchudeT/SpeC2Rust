# spec.md

## Title

Functional Specification for `module_src_parseopt_03` Rust Port

## Metadata

- Project: `cflow-new`
- Module: `module_src_parseopt_03`
- Category: `module_cluster`
- Source file scope: `src/main.c`
- Rust branch target: `066-module_src_parseopt_03-rust-port`
- Generation date: `2026-06-17`

## Overview

This module defines a focused portion of command-line/environment option handling for the program. Its responsibility is to apply specific option values and hooks to the program’s option state, especially for:

- include-class selection
- output driver selection
- cross-reference mode enablement
- symbol-related option updates
- preprocessor option accumulation and preprocess mode enablement
- indentation level setting
- installation target setting
- integer flag assignment
- path list prepending
- help/version related output hooks
- parse-option environment error reporting

The Rust rewrite must preserve the same observable behavior boundaries: given a parsed option definition, parse context, and optional argument text, the module updates program option state and emits help/version or environment-related messages in the same situations supported by the C module.

## Feature Specification

### In Scope

The Rust version must implement the option-setting behaviors evidenced by the analyzed functions in `src/main.c`:

- Applying an include-class option from an argument string.
- Applying an output-driver option from an argument string.
- Enabling or setting cross-reference behavior.
- Setting a symbol-related option from an argument.
- Accepting and storing preprocessor-related option text.
- Enabling preprocess behavior.
- Setting indentation level from an argument.
- Setting the main symbol from an argument.
- Clearing the main symbol.
- Setting an installation target from an argument.
- Assigning a constant integer value through an option definition.
- Prepending a path value into an existing path/search-path style setting.
- Producing version output through the version help hook.
- Producing help output through the help hook.
- Reporting parse-option environment errors with formatted diagnostic text.

### Behavioral Intent

This module acts as a bridge between generic option parsing infrastructure and concrete program configuration. Each option setter consumes parser context plus option metadata and applies a specific configuration effect to the active option state.

The Rust version must therefore preserve:

- option-to-state mapping behavior
- argument-driven state updates where an option requires text input
- no-argument toggles where an option simply enables or clears a setting
- help/version hook behavior as output-producing parseopt callbacks
- formatted reporting of environment-derived option errors

### Out of Scope

The following are not separately specified here because they are only referenced, not defined, by the analyzed module slice:

- the full generic `parseopt` parser implementation
- the complete `optdef` schema beyond fields required by this module’s behavior
- unrelated command-line options from other source regions
- broader application execution after option parsing completes

## User Scenarios & Testing

### Scenario 1: User selects include classes

A user provides an option whose argument specifies include classes. The module receives the parser context, option definition, and argument text, and updates the active configuration accordingly.

**Test expectation:** after applying the setter, the include-class related state reflects the supplied argument and the setter reports success/failure consistently with the original module behavior.

### Scenario 2: User selects an output driver

A user requests a specific output driver through an option argument. The module maps that argument into the program’s output-driver setting.

**Test expectation:** valid driver input updates the output-driver state; unsupported or invalid input is handled with the same success/error behavior as the C implementation.

### Scenario 3: User enables cross-reference mode

A user invokes the option associated with cross-reference output. The module enables the corresponding mode without requiring a complex argument.

**Test expectation:** the relevant option state is enabled after the setter runs.

### Scenario 4: User sets symbol-focused options

A user supplies an option argument identifying a symbol or main symbol. The module stores that symbol into the corresponding configuration field. Another option clears the main symbol.

**Test expectation:**
- setting functions record the provided symbol text;
- the clear function removes or resets the main-symbol state.

### Scenario 5: User adds preprocessor options

A user provides one or more preprocessor-related options and also enables preprocess mode.

**Test expectation:**
- each preprocessor option argument is accepted and incorporated into preprocessor configuration;
- preprocess mode becomes enabled when requested.

### Scenario 6: User sets indentation level

A user supplies a numeric or textual argument for level indentation.

**Test expectation:** the indentation-related configuration reflects the requested level according to the original option semantics.

### Scenario 7: User specifies install target

A user passes an option that sets an installation target string.

**Test expectation:** the installation-target state is updated from the provided argument.

### Scenario 8: Option definition assigns a constant integer value

An option is defined such that invoking it writes a fixed integer value into the target configuration.

**Test expectation:** the target integer field is set to the option-defined value when the setter runs.

### Scenario 9: User prepends a path component

A user supplies a path argument that must be inserted before existing path content.

**Test expectation:** resulting path configuration places the new path segment before prior content, preserving prepend semantics.

### Scenario 10: User requests version output

The parser invokes the version hook.

**Test expectation:** version information is emitted to the provided wrapped-output destination.

### Scenario 11: User requests help output

The parser invokes the help hook.

**Test expectation:** help information is emitted to the provided wrapped-output destination.

### Scenario 12: Environment option parsing fails

An option sourced from environment input causes an error and the module reports it.

**Test expectation:** the diagnostic is emitted with the supplied severity/priority and formatted message content tied to the parseopt context.

## Requirements

### Functional Requirements

#### FR-1: Include class option application
The module shall accept an include-class option argument and apply it to parser/program option state.

**Traceability:** `optset_include_classes` in `src/main.c:369-401`.

#### FR-2: Output driver option application
The module shall accept an output-driver option argument and apply the selected driver to option state.

**Traceability:** `optset_output_driver` in `src/main.c:403-412`.

#### FR-3: Cross-reference option enablement
The module shall apply the cross-reference option effect when that option is invoked.

**Traceability:** `optset_xref` in `src/main.c:414-420`.

#### FR-4: Symbol option application
The module shall accept a symbol-related argument and store/apply it to option state.

**Traceability:** `optset_symbol` in `src/main.c:422-427`.

#### FR-5: Preprocessor option accumulation/application
The module shall accept a preprocessor-related argument and apply it to preprocessor configuration.

**Traceability:** `optset_preproc_option` in `src/main.c:429-444`.

#### FR-6: Preprocess mode enablement
The module shall enable preprocess behavior when the associated option is invoked.

**Traceability:** `optset_preprocess` in `src/main.c:446-452`.

#### FR-7: Indentation level application
The module shall accept an argument for indentation configuration and apply it to option state.

**Traceability:** `optset_level_indent` in `src/main.c:454-459`.

#### FR-8: Main symbol set
The module shall accept an argument for the main symbol and store/apply it to option state.

**Traceability:** `optset_main_symbol` in `src/main.c:461-466`.

#### FR-9: Main symbol clear
The module shall clear or reset the configured main symbol when the associated option is invoked.

**Traceability:** `optset_clear_main_symbol` in `src/main.c:468-474`.

#### FR-10: Install target set
The module shall accept an install-target argument and store/apply it to option state.

**Traceability:** `optset_install_target` in `src/main.c:476-481`.

#### FR-11: Constant integer assignment via option definition
The module shall support an option behavior that assigns a fixed integer value defined by option metadata into option state.

**Traceability:** `optset_int_1` in `src/main.c:483-490`; `optdef` referenced by setter signature.

#### FR-12: Path prepend behavior
The module shall accept a path argument and prepend it to the existing path-related configuration rather than replacing existing content.

**Traceability:** `optset_prepend_path` in `src/main.c:492-504`.

#### FR-13: Version output hook
The module shall provide a version-output hook callable by the parse option framework and writing to the provided word-wrap output handle.

**Traceability:** `version_hook` in `src/main.c:900-911`.

#### FR-14: Help output hook
The module shall provide a help-output hook callable by the parse option framework and writing to the provided word-wrap output handle.

**Traceability:** `help_hook` in `src/main.c:913-920`.

#### FR-15: Environment parse-option error reporting
The module shall provide formatted error reporting for environment-sourced parse-option issues, accepting parseopt context, a priority/severity indicator, and printf-style message content.

**Traceability:** `po_env_error` in `src/main.c:938-955`.

### Key Entities

#### `parseopt`
The parse-option context passed into every setter and hook in this module. It represents the active parsing session and provides the context through which option application, help/version hook integration, and environment error reporting occur.

**Traceability:** referenced across all listed functions, including `optset_*`, `version_hook`, `help_hook`, and `po_env_error`.

#### `optdef`
The option-definition descriptor passed into option setter functions. It identifies option-specific metadata needed to determine the effect of a particular parsed option, including cases where the option applies a constant value rather than directly using the argument text.

**Traceability:** referenced across all `optset_*` functions, especially `optset_int_1`.

#### `option_type`
A local option classification/description structure defined in multiple places within `src/main.c` and used as part of the surrounding option-definition machinery this module participates in.

**Traceability:** anonymous `struct option_type` definitions at `src/main.c:31-35`, `107`, `122`, and `241`.

#### Output handle for wrapped text
The help/version hooks accept a wrapped-output destination used to emit user-facing informational text.

**Traceability:** `WORDWRAP_FILE` parameter in `version_hook` and `help_hook`.

#### Option-state fields implied by setters
This module manipulates configuration fields for include classes, output driver, cross-reference mode, symbol selection, preprocessor options, preprocess mode, indentation, main symbol, installation target, integer-valued flags, and path-related settings.

**Traceability:** one or more dedicated `optset_*` functions for each field/category.

## Success Criteria

1. The Rust module exposes behavior-equivalent option setters for every evidenced setter in this module slice and preserves their functional scope.
   - **Traceability:** `optset_include_classes`, `optset_output_driver`, `optset_xref`, `optset_symbol`, `optset_preproc_option`, `optset_preprocess`, `optset_level_indent`, `optset_main_symbol`, `optset_clear_main_symbol`, `optset_install_target`, `optset_int_1`, `optset_prepend_path`.

2. For argument-taking setters, supplying the same parser context, option definition, and argument text leads to equivalent option-state updates as in the C module.
   - **Traceability:** all argument-taking `optset_*` functions in `src/main.c:369-504`.

3. Invoking the clear-main-symbol behavior leaves the main-symbol configuration in the cleared/reset state expected by the original module.
   - **Traceability:** `optset_clear_main_symbol`.

4. Invoking the constant-integer setter writes the option-defined fixed value into the target state without requiring interpretation from the argument string beyond the original function’s contract.
   - **Traceability:** `optset_int_1`; `optdef`.

5. Path prepend behavior in the Rust version preserves prepend ordering rather than append or overwrite behavior.
   - **Traceability:** `optset_prepend_path`.

6. The Rust version provides callable help and version hooks that write user-facing output to the supplied output destination.
   - **Traceability:** `version_hook`, `help_hook`.

7. The Rust version provides environment parse-option error reporting that accepts formatted message content and parseopt context and emits diagnostics consistent in purpose with the C module.
   - **Traceability:** `po_env_error`.

8. The usage scenarios listed in this document are all supportable by tests against the Rust port.
   - **Traceability:** all listed functions in this module slice.

## Acceptance Notes

- Equivalence should be judged on observable behavior of option-state changes and emitted help/version/error outputs within the boundaries evidenced by this module.
- The Rust rewrite should not introduce unrelated capabilities or broaden the contract beyond the behaviors listed above.
- Where exact textual output is governed by the original hooks, tests should validate functional equivalence to the source behavior available from this module’s integration points.