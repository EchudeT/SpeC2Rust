# spec.md

## Title

Rust Functional Specification for `module_src_parseopt_03`

## Metadata

- **Project**: `cflow-new`
- **Module**: `module_src_parseopt_03`
- **Category**: `module_cluster`
- **Source basis**: `src/main.c`
- **Rust branch**: `066-module_src_parseopt_03-rust-port`
- **Generation date**: `2026-06-11`

## Overview

This module defines a focused portion of the command-line and environment-driven option handling behavior for the program. Its responsibility is to apply specific option values and option side effects to the program’s parse-option state, and to provide auxiliary output hooks related to help, version, and environment-option error reporting.

The Rust rewrite must preserve the observable behavior evidenced by the source for:

- option setters that translate parsed option arguments into program configuration changes,
- option setters that toggle or clear specific configuration state,
- option setters that register string and integer values into the parse-option state or related global program configuration,
- help and version output hooks tied to parse-option processing,
- formatted reporting of environment-derived option errors.

This module is not specified as a general parser by itself. It operates within a larger parse-option framework through `parseopt`, `optdef`, and output/error hook integration.

## Feature Specification

### Feature: Apply specialized option setters

The module provides specialized handlers invoked by the parse-option framework for a defined subset of program options. Each handler accepts the current parse-option context, the option definition being applied, and the option argument where applicable, then updates program configuration accordingly.

The Rust version must implement the behavior represented by the following option-setting responsibilities:

- setting included classes from an option argument,
- selecting an output driver from an option argument,
- enabling or configuring cross-reference related mode,
- setting a symbol-related option from an argument,
- adding a preprocessor-related option from an argument,
- enabling preprocess behavior,
- setting level indentation from an argument,
- setting the main symbol from an argument,
- clearing the main symbol state,
- setting an install target from an argument,
- forcing an integer-valued option to `1`,
- prepending a path value into the relevant path list or search path configuration.

Where the C module returns integer status from these handlers, the Rust version must preserve equivalent success/failure signaling semantics to the surrounding option-processing system.

### Feature: Provide help and version output hooks

The module exposes output hooks used by the parse-option system to render program version information and help information through a word-wrapped output stream abstraction.

The Rust version must preserve:

- a version hook that emits version-related output using the provided wrapped-output handle and parse-option context,
- a help hook that emits help-related output using the provided wrapped-output handle and parse-option context.

Only the hook behavior evidenced by the source should be implemented; the rewrite must not add undocumented output sections or new control flows.

### Feature: Report environment option errors

The module includes a dedicated formatted error-reporting hook for problems associated with environment-provided options.

The Rust version must preserve the ability to:

- format an error message from a printf-style format and arguments,
- associate the report with the parse-option context,
- emit the message with a supplied priority/severity value.

The observable result must remain suitable for the surrounding option framework’s handling of environment parsing failures.

## User Scenarios & Testing

### Scenario 1: Applying a value-bearing option during argument parsing

A user invokes the program with an option that requires an argument, such as one that sets the output driver, main symbol, level indentation, install target, or a preprocessor option.

**Expected behavior**:
- The parse-option framework dispatches to the corresponding setter.
- The supplied argument is consumed by that setter.
- Program configuration is updated to reflect the new value.
- The setter reports success or failure in the same situations as the C module.

**Testing focus**:
- valid non-empty argument is accepted when expected,
- configuration state after parsing matches the C behavior,
- invalid or rejected input produces the same failure signal class as the C behavior.

### Scenario 2: Enabling a flag-like option

A user invokes an option that enables a mode without requiring a meaningful data payload, such as enabling preprocessing, cross-reference behavior, or an option whose effect is to set an integer state to `1`.

**Expected behavior**:
- The corresponding setter updates the relevant flag or numeric state.
- Repeated invocation behaves consistently with the source semantics.

**Testing focus**:
- state transitions from default to enabled,
- numeric field becomes `1` where that is the documented handler effect,
- return status matches the original module behavior.

### Scenario 3: Clearing previously established main-symbol state

A user first provides an option that sets the main symbol and later provides an option intended to clear it.

**Expected behavior**:
- the set operation records the main symbol,
- the clear operation removes or resets that state,
- subsequent configuration reflects the cleared state rather than the previous value.

**Testing focus**:
- set followed by clear leaves no retained main-symbol value,
- clear operation succeeds in the same circumstances as in C.

### Scenario 4: Accumulating preprocessor-related options

A user passes multiple preprocessor-related options and include-path-related options during parsing.

**Expected behavior**:
- each preprocessor option is registered,
- prepended path handling places the new path at the front of the relevant path sequence or equivalent search configuration,
- ordering effects remain consistent with the C module.

**Testing focus**:
- multiple preprocessor options are all preserved,
- prepend behavior affects resulting order correctly,
- no option silently overwrites unrelated preprocessor state unless that is the original behavior.

### Scenario 5: Requesting help or version output

A user invokes the program in a mode that triggers help or version rendering through parse-option hooks.

**Expected behavior**:
- the proper hook is called,
- output is written through the provided wrapped-output destination,
- the emitted content is functionally equivalent to the C module’s content and purpose.

**Testing focus**:
- hook invocation produces non-empty expected category output,
- output destination abstraction is honored,
- help and version are distinguishable and correctly routed.

### Scenario 6: Environment-derived option error reporting

The program reads options from an environment source and encounters an invalid value or malformed option.

**Expected behavior**:
- the environment error reporting hook formats the message,
- severity/priority is passed through,
- the report is associated with parse-option processing in the same way as in the C version.

**Testing focus**:
- formatted arguments appear correctly in the emitted message,
- priority parameter is preserved,
- environment error path is distinguishable from normal option success flow.

## Requirements

### Functional Requirements

- **FR-1**: The module shall implement specialized option-application handlers for the subset of program options evidenced by `optset_include_classes`, `optset_output_driver`, `optset_xref`, `optset_symbol`, `optset_preproc_option`, `optset_preprocess`, `optset_level_indent`, `optset_main_symbol`, `optset_clear_main_symbol`, `optset_install_target`, `optset_int_1`, and `optset_prepend_path` in `src/main.c`.
- **FR-2**: Each option-application handler shall accept the current parse-option context and option definition context, and shall apply the option’s effect using the provided argument when the underlying handler is argument-bearing.
- **FR-3**: Each option-application handler shall return a success/failure status compatible with the calling parse-option framework, matching the C module’s observable outcomes.
- **FR-4**: The module shall support option effects that set string-like configuration values, including at minimum output driver, symbol-related values, main symbol, install target, and path/preprocessor-related values, as evidenced by the named setters in `src/main.c`.
- **FR-5**: The module shall support option effects that enable or toggle configuration modes, including preprocessing and cross-reference related behavior, as evidenced by `optset_preprocess` and `optset_xref`.
- **FR-6**: The module shall support option effects that assign integer-valued configuration, including setting level indentation from an argument and forcing a value to `1`, as evidenced by `optset_level_indent` and `optset_int_1`.
- **FR-7**: The module shall support explicit clearing/resetting of previously set main-symbol configuration, as evidenced by `optset_clear_main_symbol`.
- **FR-8**: The module shall support prepending a path entry in a manner that preserves prepend semantics relative to existing path state, as evidenced by `optset_prepend_path`.
- **FR-9**: The module shall provide a version-output hook callable by the parse-option system, as evidenced by `version_hook`.
- **FR-10**: The module shall provide a help-output hook callable by the parse-option system, as evidenced by `help_hook`.
- **FR-11**: The module shall provide formatted environment-option error reporting that accepts parse-option context, severity/priority, and formatted message arguments, as evidenced by `po_env_error`.
- **FR-12**: The Rust rewrite shall preserve the module’s role as an internal parse-option integration component and shall not require capabilities beyond those evidenced by the `parseopt`, `optdef`, and wrapped-output hook usage in `src/main.c`.

### Key Entities

- **`parseopt`**: Parse-option processing context passed to every option setter and to the help/version/error hooks. It is the central state carrier through which option effects are applied or reported.
- **`optdef`**: Option-definition descriptor passed to option setter functions. It identifies the option currently being handled and participates in applying the correct semantics.
- **`option_type`**: Local option classification/description structure referenced in `src/main.c`. It contributes to the program’s option-definition model used by the parse-option subsystem.
- **Wrapped output handle (`WORDWRAP_FILE`)**: Output abstraction used by help and version hooks to render formatted text.
- **Option argument (`char *arg` in C)**: Input value supplied for value-bearing options. In Rust, this must be represented safely while preserving the distinction between options that consume an argument and those that do not.

**Relationships**:
- A `parseopt` context is supplied together with an `optdef` when invoking an option setter.
- Some setters additionally consume an option argument and apply it to state reachable through `parseopt` or associated configuration.
- Help and version hooks consume a wrapped output handle plus `parseopt`.
- Environment error reporting consumes `parseopt`, a priority value, and a formatted message payload.

## Success Criteria

- **SC-1**: For every supported option setter in this module, invoking the Rust implementation with the same effective inputs as the C implementation results in the same category of state change and success/failure status.
- **SC-2**: Options that set textual values preserve the provided value in the same target configuration role as the C module.
- **SC-3**: Options that enable modes or set numeric flags produce the same enabled/disabled or numeric result as the C module, including the explicit `1`-setting behavior.
- **SC-4**: A set-main-symbol operation followed by a clear-main-symbol operation leaves main-symbol state cleared in the Rust implementation.
- **SC-5**: Repeated application of preprocessor-option and prepend-path handlers preserves accumulation and ordering semantics consistent with the C module.
- **SC-6**: The Rust version exposes help and version hooks that can be invoked by the surrounding option framework and that emit functionally equivalent output through the provided wrapped-output abstraction.
- **SC-7**: Environment option parsing failures routed through the Rust environment error reporter preserve message formatting behavior and severity propagation equivalent to the C module.
- **SC-8**: All requirements in this specification are traceable to behaviors evidenced in `src/main.c` and no additional externally visible capabilities are introduced beyond those behaviors.

## Traceability

| Requirement / Criterion | Source Evidence |
|---|---|
| FR-1 to FR-8, SC-1 to SC-5 | `optset_include_classes`, `optset_output_driver`, `optset_xref`, `optset_symbol`, `optset_preproc_option`, `optset_preprocess`, `optset_level_indent`, `optset_main_symbol`, `optset_clear_main_symbol`, `optset_install_target`, `optset_int_1`, `optset_prepend_path` in `src/main.c` |
| FR-9, SC-6 | `version_hook` in `src/main.c` |
| FR-10, SC-6 | `help_hook` in `src/main.c` |
| FR-11, SC-7 | `po_env_error` in `src/main.c` |
| FR-12 | Shared use of `parseopt`, `optdef`, `WORDWRAP_FILE`, and local option metadata structures in `src/main.c` |
| Key Entities section | Referenced `parseopt`, `optdef`, `option_type`, and hook signatures in `src/main.c` |