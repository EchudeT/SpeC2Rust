# spec.md

## Title

Rust Port Functional Specification: `module_src_parseopt_04`

## Overview

This module covers the portion of `cflow-new` option parsing that:

- loads option settings from environment-driven profile input,
- reports errors encountered while processing option files,
- parses option file content into arguments,
- applies a profile option value to a parse-option state object, and
- runs initialization logic that prepares profile-based option processing.

The Rust rewrite on branch `067-module_src_parseopt_04-rust-port` must preserve the observable behavior of this module as evidenced by the analyzed functions in `src/main.c`:

- `parseopt_from_env`
- `fromfile_error`
- `fromfile`
- `optset_profile`
- `init_hook`

This specification describes only the functional boundary visible from those functions and the referenced option-parsing data structures.

## Scope

In scope:

- reading profile-related option input from environment-controlled sources,
- processing option/profile file input through the module’s parse-option context,
- producing formatted diagnostics for option-file processing failures,
- setting a profile option value on a parse-option object,
- running initialization behavior required before or during this profile-loading path.

Out of scope:

- unrelated command-line option behaviors not evidenced by the listed functions,
- redesign of the project’s overall option parser,
- new configuration sources or new option semantics not evidenced by this module.

## Feature Specification

### Feature: Environment-driven option profile loading

The module supports loading option settings from environment-derived configuration input. The environment-loading entry point must trigger profile parsing using the module’s parse-option state and must do so during initialization flow where required.

The Rust version must preserve:

- the presence of an environment-based option loading step,
- the dependency on parse-option state objects and option definitions,
- the ability for initialization to invoke or prepare this environment/profile logic.

Traceability:

- `parseopt_from_env`
- `init_hook`
- `parseopt`

### Feature: Option-file parsing

The module supports reading and parsing option content from a profile/options file abstraction. The file-processing behavior must convert file content into option arguments and feed them into the parse-option system.

The Rust version must preserve:

- processing through a `parseopt_file`-like entity,
- success/failure completion status for a file parse operation,
- integration with the parser context used by profile option handling,
- support for word-splitting/tokenization semantics implied by the `wordsplit` dependency.

Traceability:

- `fromfile`
- `parseopt_file`
- `wordsplit`
- `parseopt`

### Feature: File-processing diagnostics

The module provides a dedicated error-reporting path for failures that occur while processing option/profile files. Diagnostic output is tied to the parse-option context and supports severity/priority plus formatted messages.

The Rust version must preserve:

- context-aware reporting for option-file errors,
- formatted message generation with caller-supplied text and arguments,
- a priority/severity input that influences emitted diagnostics.

Traceability:

- `fromfile_error`
- `parseopt`

### Feature: Profile option application

The module includes behavior for handling a specific option-setting action that assigns or activates a profile value on the parse-option state.

The Rust version must preserve:

- an operation corresponding to setting a profile-related option,
- use of the current parse-option state and the option definition being applied,
- an integer success/failure result compatible with the surrounding parser flow.

Traceability:

- `optset_profile`
- `parseopt`
- `optdef`

### Feature: Initialization hook integration

The module includes initialization-time logic that prepares parse-option behavior relevant to profile/environment processing.

The Rust version must preserve:

- an initialization hook operating on parse-option state,
- the hook’s role in enabling or triggering environment/profile option handling.

Traceability:

- `init_hook`
- `parseopt`
- `parseopt_from_env`

## User Scenarios & Testing

### Scenario 1: Load options from environment during parser initialization

A program initializes its parse-option context. During initialization, environment-based profile handling is executed or enabled, causing option values defined through the expected environment source to be considered.

The Rust version must support testing that:

- initialization on a parse-option context can lead to environment/profile processing,
- environment-driven profile data affects option parsing behavior through the same module boundary,
- absence of relevant environment input does not falsely report a file-processing error.

Traceability:

- `init_hook`
- `parseopt_from_env`

### Scenario 2: Apply a profile option explicitly

A caller passes a profile-related option and argument to the option parser. The parser invokes the profile option setter, which accepts the parse-option context, the option definition, and the provided argument, then returns status.

The Rust version must support testing that:

- a valid profile argument is accepted through the profile option setter path,
- the parse-option state reflects that the profile option was applied,
- the returned status distinguishes accepted versus rejected input as in the C behavior.

Traceability:

- `optset_profile`

### Scenario 3: Parse an options file successfully

A profile/options file abstraction is provided to the parser. The module reads its contents, splits them into words/arguments, and processes them through the parse-option flow without diagnostic failure.

The Rust version must support testing that:

- file content containing valid option text is parsed successfully,
- tokenization/word-splitting produces arguments consumable by the parse-option system,
- the file-processing function returns a success status when parsing completes without error.

Traceability:

- `fromfile`
- `parseopt_file`
- `wordsplit`

### Scenario 4: Report an error while parsing an options file

An invalid file, malformed option text, or processing failure occurs while reading profile/options input. The module emits a formatted diagnostic associated with the parse-option context and returns failure from the file-processing path.

The Rust version must support testing that:

- file-processing failures invoke the dedicated error-reporting path,
- the emitted diagnostic includes the caller’s formatted message content,
- file parsing returns failure after a reported error.

Traceability:

- `fromfile_error`
- `fromfile`

### Scenario 5: Environment-sourced profile leads to file-based option loading

Environment-derived profile selection causes the parser to process an associated options source through the file-processing path.

The Rust version must support testing that:

- environment/profile handling can drive the option-file parser,
- any resulting file-processing error is reported through the module’s diagnostic path,
- successful environment-driven loading updates parse-option state consistently with explicit profile application.

Traceability:

- `parseopt_from_env`
- `fromfile`
- `fromfile_error`
- `optset_profile`
- `init_hook`

## Requirements

### Functional Requirements

#### FR-1: Environment profile processing
The module shall provide behavior that reads profile-related option input from environment-controlled state and applies it to option parsing.

Traceability:

- `parseopt_from_env`

#### FR-2: Initialization-time integration
The module shall provide an initialization hook on the parse-option context that performs or enables environment/profile processing needed by this module.

Traceability:

- `init_hook`
- `parseopt_from_env`

#### FR-3: Profile option assignment
The module shall support applying a profile option value using a parse-option context, an option definition, and an argument value, and shall return status indicating the result of that operation.

Traceability:

- `optset_profile`
- `parseopt`
- `optdef`

#### FR-4: Option-file parsing
The module shall support processing a `parseopt_file` input source and converting its content into parser arguments for the option parser.

Traceability:

- `fromfile`
- `parseopt_file`
- `wordsplit`

#### FR-5: File parse completion status
The option-file parsing operation shall return an integer status that distinguishes successful processing from failure.

Traceability:

- `fromfile`

#### FR-6: File-processing diagnostics
The module shall provide a diagnostic-reporting function for option-file processing errors that accepts parser context, severity/priority, and a formatted message.

Traceability:

- `fromfile_error`
- `parseopt`

#### FR-7: Error propagation from file parsing
When option-file processing encounters an error condition, the module shall report that condition through the dedicated file-error reporting path and indicate failure to the caller.

Traceability:

- `fromfile`
- `fromfile_error`

### Key Entities

#### `parseopt`
The central parse-option context used across all functions in this module. It is the state carrier for initialization, profile application, environment-driven processing, and diagnostics.

Relationships:

- used by `optset_profile` to apply profile option values,
- used by `fromfile_error` to associate diagnostics with parser state,
- used indirectly by environment and initialization paths.

Traceability:

- `parseopt_from_env`
- `fromfile_error`
- `optset_profile`
- `init_hook`

#### `optdef`
The option definition descriptor associated with a parse-option action. In this module it participates in profile option handling.

Relationships:

- supplied together with `parseopt` and an argument to `optset_profile`.

Traceability:

- `optset_profile`

#### `parseopt_file`
The abstraction representing a profile/options file input to be parsed.

Relationships:

- consumed by `fromfile`,
- serves as the source of option text that is transformed into parser arguments.

Traceability:

- `fromfile`

#### `wordsplit`
The word/token splitting facility referenced by the option-file parsing path.

Relationships:

- supports `fromfile` by turning file content into argument-like tokens compatible with the parser.

Traceability:

- `fromfile`

## Success Criteria

1. The Rust module exposes behavior equivalent to environment-driven profile loading, and initialization can invoke or enable that behavior on a parse-option context.
   - Traceability: `parseopt_from_env`, `init_hook`

2. Applying a profile option through the module accepts the same categories of inputs as the C module and returns success/failure status in a way compatible with parser control flow.
   - Traceability: `optset_profile`

3. Parsing a valid `parseopt_file` input succeeds and yields parser-consumable arguments derived from file content.
   - Traceability: `fromfile`

4. Parsing an invalid or malformed `parseopt_file` input fails and emits a context-aware formatted diagnostic through the dedicated error-reporting path.
   - Traceability: `fromfile`, `fromfile_error`

5. The Rust port preserves the functional relationships among parse-option context, option definitions, environment/profile selection, and file-based option loading without adding unsupported configuration behaviors.
   - Traceability: `parseopt`, `optdef`, `parseopt_file`, `parseopt_from_env`, `fromfile`, `optset_profile`, `init_hook`