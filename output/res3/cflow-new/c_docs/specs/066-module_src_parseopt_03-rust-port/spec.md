# spec.md

## Title

Rust Functional Specification: `module_src_parseopt_03` for `cflow-new`

## Overview

This module defines a focused subset of command-line/environment option handling behavior from `src/main.c`. Its responsibility is to process specific option values and flags, apply their effects to the active parse-option state, and provide related help/version and environment-option error reporting hooks.

The Rust rewrite must preserve the observable behavior evidenced by the analyzed functions, specifically:

- applying option arguments to parser/configuration state,
- toggling or clearing selected option-controlled values,
- delegating some option values into existing subsystems such as symbol/class selection, output driver selection, preprocessing options, and path handling,
- emitting version/help text through the existing word-wrapping output interface,
- reporting environment-originated option parsing errors with formatted diagnostics.

This specification covers only the functionality evidenced by the analyzed module slice and does not define unrelated parsing features.

## Scope

In scope:

- Option setter behavior implemented by the analyzed `optset_*` functions.
- Help and version output hooks.
- Environment option error formatting/reporting.

Out of scope:

- Full command-line parsing framework behavior not evidenced here.
- Definitions of external parser, preprocessor, output-driver, or path-search subsystems beyond the interactions shown by this module.
- Any new options, new public APIs, or expanded validation behavior not evidenced by the source slice.

## Source Traceability

Primary source file:

- `src/main.c`

Primary functions covered by this specification:

- `optset_include_classes`
- `optset_output_driver`
- `optset_xref`
- `optset_symbol`
- `optset_preproc_option`
- `optset_preprocess`
- `optset_level_indent`
- `optset_main_symbol`
- `optset_clear_main_symbol`
- `optset_install_target`
- `optset_int_1`
- `optset_prepend_path`
- `version_hook`
- `help_hook`
- `po_env_error`

Primary entities referenced by this specification:

- `parseopt`
- `optdef`
- option-type records used to describe option metadata/behavior

## Feature Specification

### 1. Option-specific state application

The module must support option handlers that receive:

- the active parse-option context,
- the option definition being applied,
- the option argument when present.

Each supported handler must apply the option’s effect to program state according to its specific role.

The supported behaviors evidenced by the source are:

#### 1.1 Include-class selection

The module must accept an option argument describing include classes and apply that selection to the active configuration state.

Observed role:
- `optset_include_classes`

Required behavior:
- consume the provided argument string,
- update the relevant include-class selection state,
- return success/failure in the same success model used by the option framework.

#### 1.2 Output driver selection

The module must accept an option argument that selects the output driver.

Observed role:
- `optset_output_driver`

Required behavior:
- consume the provided driver argument,
- update the active output-driver selection in configuration state.

#### 1.3 Cross-reference mode enablement

The module must support an option that enables cross-reference-related behavior.

Observed role:
- `optset_xref`

Required behavior:
- set the corresponding state when the option is applied.

#### 1.4 Symbol selection/update

The module must support an option that applies a symbol-related argument to configuration state.

Observed role:
- `optset_symbol`

Required behavior:
- consume the provided symbol argument,
- update the relevant symbol-selection state.

#### 1.5 Preprocessor option accumulation

The module must support an option that forwards a preprocessor-related argument into preprocessing configuration.

Observed role:
- `optset_preproc_option`

Required behavior:
- consume the provided preprocessor option argument,
- record or forward it into preprocessor option state in the same semantic direction as the C module.

#### 1.6 Preprocessing mode control

The module must support an option that changes preprocessing behavior.

Observed role:
- `optset_preprocess`

Required behavior:
- set the corresponding preprocess mode/state when the option is applied.

#### 1.7 Level indentation configuration

The module must support an option that sets indentation level behavior from an argument.

Observed role:
- `optset_level_indent`

Required behavior:
- consume the provided argument,
- update the indentation-related setting used by the wider program.

#### 1.8 Main-symbol assignment and clearing

The module must support:
- assigning a main symbol from an argument,
- clearing any previously assigned main symbol.

Observed roles:
- `optset_main_symbol`
- `optset_clear_main_symbol`

Required behavior:
- assignment stores/replaces the configured main symbol,
- clearing removes the configured main symbol and leaves the state in the equivalent “no main symbol configured” condition.

#### 1.9 Install-target assignment

The module must support an option that assigns an installation target value from an argument.

Observed role:
- `optset_install_target`

Required behavior:
- consume the provided argument,
- update the corresponding installation-target state.

#### 1.10 Generic integer-to-one setter

The module must support an option handler that sets an integer-backed option state to `1`.

Observed role:
- `optset_int_1`

Required behavior:
- identify the target integer-backed setting via option metadata/context,
- set it to enabled/true by assigning the value `1`.

#### 1.11 Search path prepending

The module must support an option that prepends a path argument into the relevant search path list/order.

Observed role:
- `optset_prepend_path`

Required behavior:
- consume the provided path argument,
- place it before existing entries in the relevant path configuration sequence.

### 2. Help and version hooks

The module must provide hooks that write version/help information to the configured word-wrapping output destination.

Observed roles:
- `version_hook`
- `help_hook`

Required behavior:
- accept a `WORDWRAP_FILE`-style output destination abstraction and parse-option context,
- write version text for the version hook,
- write help text for the help hook,
- preserve the distinction between the two hooks.

This specification does not require wording beyond preserving the existing functional role of each hook.

### 3. Environment option error reporting

The module must provide a formatter/reporting function for errors associated with environment-originated option processing.

Observed role:
- `po_env_error`

Required behavior:
- accept:
  - parse-option context,
  - message priority/severity indicator,
  - printf-style format string and arguments,
- emit a formatted diagnostic associated with environment option handling,
- preserve severity/priority input in the same reporting pathway used by the original module.

## User Scenarios & Testing

### Scenario 1: Selecting include classes from an option

A caller applies an option whose argument specifies include classes.
Expected result:

- the include-class state is updated from the provided argument,
- the handler reports success when the argument is accepted.

Test evidence target:
- behavior traceable to `optset_include_classes`.

### Scenario 2: Selecting an output driver

A caller applies an option with an output-driver name.
Expected result:

- the output driver setting changes to match the argument.

Test evidence target:
- behavior traceable to `optset_output_driver`.

### Scenario 3: Enabling cross-reference behavior

A caller applies the cross-reference option.
Expected result:

- the corresponding mode/flag becomes enabled.

Test evidence target:
- behavior traceable to `optset_xref`.

### Scenario 4: Providing a symbol-oriented option value

A caller applies a symbol-related option with a symbol argument.
Expected result:

- the symbol-related state is updated using the provided argument.

Test evidence target:
- behavior traceable to `optset_symbol`.

### Scenario 5: Adding a preprocessor option

A caller applies a preprocessor option such as a macro definition or include-related flag represented as a raw argument string.
Expected result:

- the preprocessor configuration receives the supplied option.

Test evidence target:
- behavior traceable to `optset_preproc_option`.

### Scenario 6: Switching preprocessing mode

A caller applies the preprocessing control option.
Expected result:

- the preprocess-related state changes accordingly.

Test evidence target:
- behavior traceable to `optset_preprocess`.

### Scenario 7: Setting indentation level behavior

A caller applies an indentation option with an argument.
Expected result:

- the indentation setting is updated from that argument.

Test evidence target:
- behavior traceable to `optset_level_indent`.

### Scenario 8: Setting and then clearing the main symbol

A caller first applies a main-symbol option with a symbol name, then applies the clear-main-symbol option.
Expected result:

- after the first option, the configured main symbol matches the supplied name,
- after the second option, no main symbol remains configured.

Test evidence target:
- behavior traceable to `optset_main_symbol` and `optset_clear_main_symbol`.

### Scenario 9: Setting install target

A caller applies an install-target option with an argument.
Expected result:

- the install-target state is updated from the argument.

Test evidence target:
- behavior traceable to `optset_install_target`.

### Scenario 10: Enabling a boolean/integer-backed flag through the generic setter

A caller applies an option wired to the generic integer-to-one handler.
Expected result:

- the targeted integer-backed setting becomes `1`.

Test evidence target:
- behavior traceable to `optset_int_1`.

### Scenario 11: Prepending a path

A caller applies a path option with a directory argument when one or more directories are already configured.
Expected result:

- the new directory is inserted ahead of existing configured entries.

Test evidence target:
- behavior traceable to `optset_prepend_path`.

### Scenario 12: Printing version text

A caller invokes the version hook with an output destination.
Expected result:

- version information is written to that destination.

Test evidence target:
- behavior traceable to `version_hook`.

### Scenario 13: Printing help text

A caller invokes the help hook with an output destination.
Expected result:

- help information is written to that destination.

Test evidence target:
- behavior traceable to `help_hook`.

### Scenario 14: Reporting an environment option error

An environment-derived option causes an error, and the reporting function is called with a severity and formatted message.
Expected result:

- a formatted diagnostic is emitted through the parse-option error-reporting pathway,
- the environment-origin nature of the error remains associated with the report as in the C behavior.

Test evidence target:
- behavior traceable to `po_env_error`.

## Requirements

### Functional Requirements

#### FR-1: Option handler interface compatibility
The Rust module shall implement handlers for the evidenced option behaviors using the same functional inputs as the C module conceptually requires: parse-option context, option definition context, and option argument where applicable.

Traceability:
- `optset_include_classes`
- `optset_output_driver`
- `optset_xref`
- `optset_symbol`
- `optset_preproc_option`
- `optset_preprocess`
- `optset_level_indent`
- `optset_main_symbol`
- `optset_clear_main_symbol`
- `optset_install_target`
- `optset_int_1`
- `optset_prepend_path`

#### FR-2: Include-class option application
The Rust module shall apply include-class selection from an option argument to configuration state.

Traceability:
- `optset_include_classes`

#### FR-3: Output-driver option application
The Rust module shall apply output-driver selection from an option argument to configuration state.

Traceability:
- `optset_output_driver`

#### FR-4: Cross-reference option application
The Rust module shall enable the cross-reference-related state when the corresponding option is applied.

Traceability:
- `optset_xref`

#### FR-5: Symbol option application
The Rust module shall apply symbol-related option arguments to configuration state.

Traceability:
- `optset_symbol`

#### FR-6: Preprocessor option forwarding/storage
The Rust module shall accept preprocessor option arguments and apply them to preprocessing configuration.

Traceability:
- `optset_preproc_option`

#### FR-7: Preprocess mode control
The Rust module shall change preprocess-related state when the corresponding option is applied.

Traceability:
- `optset_preprocess`

#### FR-8: Indentation setting update
The Rust module shall update indentation-related configuration from the corresponding option argument.

Traceability:
- `optset_level_indent`

#### FR-9: Main symbol set and clear
The Rust module shall support both assigning a main symbol from an argument and clearing that assignment.

Traceability:
- `optset_main_symbol`
- `optset_clear_main_symbol`

#### FR-10: Install-target update
The Rust module shall update installation-target configuration from the corresponding option argument.

Traceability:
- `optset_install_target`

#### FR-11: Integer-backed enable setter
The Rust module shall support a generic handler that sets the targeted integer-backed option state to `1`.

Traceability:
- `optset_int_1`

#### FR-12: Path prepend behavior
The Rust module shall prepend a provided path argument ahead of existing configured path entries.

Traceability:
- `optset_prepend_path`

#### FR-13: Version output hook
The Rust module shall provide a version-output hook that writes version information to the supplied word-wrapping output destination.

Traceability:
- `version_hook`

#### FR-14: Help output hook
The Rust module shall provide a help-output hook that writes help information to the supplied word-wrapping output destination.

Traceability:
- `help_hook`

#### FR-15: Environment option error reporting
The Rust module shall provide a formatted diagnostic function for environment-related option errors that accepts a parse-option context, severity/priority, and format arguments.

Traceability:
- `po_env_error`

### Key Entities

#### `parseopt`
Represents the active option-processing context passed into all option handlers and hooks in this module. It is the primary carrier of mutable option state and the reporting context for help/version/error functions.

Relationships:
- consumed by every `optset_*` function in this module,
- consumed by `version_hook`, `help_hook`, and `po_env_error`,
- links option application with diagnostic/reporting behavior.

Traceability:
- all listed functions

#### `optdef`
Represents the metadata/definition of the currently processed option. In this module it supplies contextual information needed by generic and option-specific handlers.

Relationships:
- paired with `parseopt` during option application,
- especially relevant where handler behavior depends on option definition rather than only the raw argument.

Traceability:
- all listed `optset_*` functions

#### Option-type records
Represent tables or descriptors used by the surrounding option system to classify and bind options to handlers. The analyzed file contains several anonymous `struct option_type` instances/definitions supporting this role.

Relationships:
- define how option definitions are categorized and dispatched to handlers,
- connect external option syntax to the handler functions specified above.

Traceability:
- anonymous `struct option_type` occurrences in `src/main.c`

#### Word-wrapping output destination
Represents the output sink used by help/version hooks.

Relationships:
- passed into `version_hook` and `help_hook`,
- receives formatted user-facing informational text.

Traceability:
- `version_hook`
- `help_hook`

## Success Criteria

1. Applying each supported option handler updates the same category of state as the corresponding C function, with no missing handler from the analyzed set.
   - Measured by handler-level tests for:
     - include classes,
     - output driver,
     - xref,
     - symbol,
     - preprocessor option,
     - preprocess mode,
     - level indent,
     - main symbol set,
     - main symbol clear,
     - install target,
     - integer-to-one setter,
     - prepend path.

2. Main-symbol lifecycle behavior is preserved.
   - Measured by a test that sets a main symbol, verifies presence, clears it, and verifies absence.
   - Traceability:
     - `optset_main_symbol`
     - `optset_clear_main_symbol`

3. Path prepending preserves ordering semantics.
   - Measured by a test that starts with existing path entries, prepends a new path, and verifies the new path becomes the first effective entry.
     - `optset_prepend_path`

4. The generic integer-backed setter assigns the target state the exact enabled value `1`.
   - Measured by a test using option metadata/context wired to the generic setter.
     - `optset_int_1`

5. Version and help hooks each emit output to the provided destination and remain behaviorally distinct.
   - Measured by tests confirming non-empty output from each hook and that the version hook is not substituted for help output or vice versa.
     - `version_hook`
     - `help_hook`

6. Environment option error reporting emits a formatted diagnostic through the parse-option reporting pathway using the supplied severity/priority and message arguments.
   - Measured by a test harness that captures diagnostics and verifies formatted content and severity propagation.
     - `po_env_error`

7. The Rust rewrite introduces no required functional regression in the evidenced option-processing subset.
   - Measured by scenario-based tests covering all scenarios listed in this document.
     - all functions in scope