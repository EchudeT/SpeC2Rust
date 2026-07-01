# spec.md

## Title

Rust Functional Specification for `module_src_parseopt_04`

## Metadata

- Project: `cflow-new`
- Module: `module_src_parseopt_04`
- Category: `module_cluster`
- Source file coverage: `src/main.c`
- Rust branch target: `067-module_src_parseopt_04-rust-port`
- Generation date: `2026-06-11`

## Overview

This module covers the part of the option-processing system responsible for:

- loading option settings from environment-controlled input,
- reporting errors that occur while reading option files,
- parsing option data from a file-backed source,
- applying a profile-related option setting,
- performing parse-option initialization hook behavior.

The Rust rewrite must preserve the observable behavior of these responsibilities as evidenced by the source functions:

- `parseopt_from_env`
- `fromfile_error`
- `fromfile`
- `optset_profile`
- `init_hook`

This specification is limited to the functional boundary shown by those functions and the referenced option-parsing data structures. It does not require unrelated command-line features beyond what these functions directly participate in.

## Feature Specification

### 1. Environment-driven option loading

The module must support initializing option parsing from environment-provided configuration input.

Observed boundary:
- `parseopt_from_env`

Required behavior:
- Read configuration source information from environment-driven context used by the option parser.
- If the environment indicates that option text or an option file should be processed, invoke the same option-file parsing pathway used for file-backed input.
- Integrate environment-derived options into the active parse-option state rather than treating them as an unrelated configuration channel.

Out of scope:
- Defining new environment variables or expanding environment behavior not evidenced by the module.

### 2. File-based option parsing

The module must support reading options from a `parseopt_file` source and applying them to parser state.

Observed boundary:
- `fromfile`
- `parseopt_file`
- `wordsplit`
- `parseopt`
- `optdef`

Required behavior:
- Consume option content from a file-associated parse context.
- Interpret file contents as option tokens suitable for the same option-processing system used elsewhere by the program.
- Use token splitting appropriate for configuration text input.
- Return a success/failure status for the overall file parse operation.
- Preserve the relationship between the file input context and the destination parser state.

This feature is specifically about processing configuration/profile-like option input from a file source; it is not a general-purpose file parser beyond that role.

### 3. File-source error reporting

The module must emit contextual errors encountered while processing a file-based option source.

Observed boundary:
- `fromfile_error`
- `fromfile`
- `parseopt`
- `parseopt_file`

Required behavior:
- Report errors associated with a specific parse-option file context.
- Support formatted error messages.
- Include enough file-associated context so that an error can be tied back to the source being read.
- Support reporting with caller-provided severity/priority information.

The Rust rewrite may adapt the mechanics of formatting and emission, but it must preserve the functional outcome: file-origin parsing failures produce contextual diagnostics.

### 4. Profile option handling

The module must support an option-setting action that applies a profile argument to the active option parser.

Observed boundary:
- `optset_profile`
- `parseopt`
- `optdef`

Required behavior:
- Accept the active parser state, the option definition being processed, and the associated argument text.
- Treat the argument as a profile selection or profile source input within the option system.
- Return an integer status indicating whether the option-setting action succeeded or failed.
- Integrate profile processing with the file/environment option loading flow when required by the parser design evidenced in the source.

This requirement is limited to the profile-setting behavior directly represented by `optset_profile`.

### 5. Parser initialization hook

The module must support an initialization hook invoked for a `parseopt` instance before or during option processing.

Observed boundary:
- `init_hook`
- `parseopt`

Required behavior:
- Perform parser-state initialization actions assigned to this hook.
- Ensure the parser is prepared for the environment/file/profile option behaviors covered by this module.
- Operate on the provided parse-option state only within the initialization role evidenced by the source.

This hook must exist in the Rust rewrite if the surrounding parser lifecycle requires it to preserve behavior.

## User Scenarios & Testing

### Scenario 1: Load options from environment at startup

A program starts with environment state that requests extra parser options to be loaded. The module initializes the parser, inspects environment-controlled configuration, and applies the resulting options to the parse state.

Expected result:
- Environment-derived options are processed through the module’s option-loading flow.
- Any invalid environment-sourced file/input produces contextual diagnostics.
- The active parser state reflects the loaded settings.

Suggested tests:
- Provide environment configuration that points to valid option input and verify settings are applied.
- Provide environment configuration that points to invalid or unreadable option input and verify failure is reported.
- Verify no spurious option loading occurs when the relevant environment input is absent.

### Scenario 2: Parse options from a file source

A parse-option file context is created for a configuration file containing option text. The module reads the file, splits its content into option tokens, and applies them through the parser.

Expected result:
- Valid file content is accepted and applied.
- The function reports success on valid input.
- Invalid option syntax or file-processing failure produces failure status and diagnostics tied to the source file context.

Suggested tests:
- Parse a file containing one valid option assignment.
- Parse a file containing multiple option tokens, including profile-related input if supported by the surrounding parser.
- Parse malformed content and verify failure status plus contextual error output.

### Scenario 3: Report a file parsing error with context

During file-backed option processing, an error occurs. The module emits a diagnostic through the file-specific error path.

Expected result:
- The diagnostic includes source context from the active `parseopt_file`.
- The supplied message text is reflected in the output.
- The severity/priority parameter affects the reporting path consistently with existing behavior.

Suggested tests:
- Trigger a parse failure and verify the diagnostic references the current file source.
- Trigger multiple error classes and verify the reporting function can emit formatted messages for each.

### Scenario 4: Apply a profile option

An option definition resolves to the profile-setting handler, and an argument string naming or identifying a profile is supplied.

Expected result:
- The profile handler processes the argument in the active parser context.
- Success/failure is returned as an integer status.
- On success, resulting parser state matches the selected profile’s effect as supported by the option system.

Suggested tests:
- Call the profile option handler with a valid argument and verify success.
- Call the handler with an invalid or unusable argument and verify failure behavior.
- Verify profile handling composes correctly with subsequent or prior file/environment option loading when applicable.

### Scenario 5: Initialize parser state before option loading

A new parse-option state is prepared before reading command-line, environment, or profile input. The initialization hook is invoked.

Expected result:
- The parser is left in a valid state for later environment and file-based option processing.
- Required initial fields or relationships used by this module are ready.
- Regressions do not occur in profile or file-loading flows due to missing initialization.

Suggested tests:
- Invoke initialization followed by environment loading and verify no uninitialized-state failure occurs.
- Invoke initialization followed by file parsing and verify the parser accepts input normally.

## Requirements

### Functional Requirements

#### FR-1: Environment option ingestion
The module shall inspect environment-controlled configuration relevant to option parsing and, when present, load that configuration into the active parse-option workflow.

Traceability:
- `parseopt_from_env` in `src/main.c`

#### FR-2: Unified application of environment-sourced options
The module shall apply environment-sourced option input through the parser’s existing option-processing path rather than as unrelated ad hoc state changes.

Traceability:
- `parseopt_from_env`
- `fromfile`
- `parseopt`

#### FR-3: File-backed option parsing
The module shall read option content from a `parseopt_file` context and process that content as parser options.

Traceability:
- `fromfile`
- `parseopt_file`
- `parseopt`

#### FR-4: Tokenization of file option content
The module shall split file-provided option text into words/tokens suitable for parser processing.

Traceability:
- `fromfile`
- `wordsplit`

#### FR-5: File parsing status reporting
The module shall return explicit success/failure status from file-backed option parsing.

Traceability:
- `fromfile`

#### FR-6: Contextual file error reporting
The module shall provide an error-reporting path for file-origin parsing problems that includes source-related context and a formatted message.

Traceability:
- `fromfile_error`
- `parseopt_file`

#### FR-7: Severity-aware diagnostics
The module shall accept caller-provided priority/severity information when reporting file parsing errors.

Traceability:
- `fromfile_error`

#### FR-8: Profile option application
The module shall support an option-setting handler that accepts a profile argument and applies it within the active parser context.

Traceability:
- `optset_profile`
- `parseopt`
- `optdef`

#### FR-9: Profile handler status reporting
The module shall return explicit integer success/failure status from profile option handling.

Traceability:
- `optset_profile`

#### FR-10: Parser initialization hook support
The module shall provide initialization-hook behavior for `parseopt` state used by this option-loading subsystem.

Traceability:
- `init_hook`
- `parseopt`

### Key Entities

#### `parseopt`
Core parser state for the option-processing system. This module reads from and mutates this state during initialization, environment loading, file processing, and profile application.

Traceability:
- Referenced by `parseopt_from_env`
- Referenced by `fromfile_error`
- Referenced by `fromfile`
- Referenced by `optset_profile`
- Referenced by `init_hook`

#### `parseopt_file`
Represents a file-associated option input source. This module uses it as the source context for reading option text and for attaching contextual diagnostics.

Traceability:
- Referenced by `fromfile_error`
- Referenced by `fromfile`

#### `optdef`
Represents an option definition routed through the parser’s handler mechanism. This module uses it when applying profile option behavior.

Traceability:
- Referenced by `optset_profile`

#### `wordsplit`
Represents the tokenization/splitting facility used to convert file text into parser-consumable option tokens.

Traceability:
- Referenced by `fromfile`

### Entity Relationships

- A `parseopt_file` is processed in the context of a `parseopt` instance.
- `fromfile` reads from `parseopt_file` and applies results to `parseopt`.
- `fromfile_error` reports errors about a `parseopt_file` while operating within the parser subsystem.
- `optdef` selects handler behavior, including profile application via `optset_profile`, against a `parseopt` instance.
- `init_hook` prepares `parseopt` state for the option-loading operations above.
- `wordsplit` supports `fromfile` by turning file text into option tokens.

## Success Criteria

### SC-1: Environment loading equivalence
When environment-controlled option input is present, the Rust module processes it and updates parser state in a manner functionally equivalent to the C module’s environment-loading path.

Traceability:
- `parseopt_from_env`

### SC-2: Valid file input succeeds
For valid `parseopt_file` input accepted by the original module, the Rust module returns success and applies the same effective options to parser state.

Traceability:
- `fromfile`

### SC-3: Invalid file input fails with context
For invalid, unreadable, or unparsable file-backed option input that the original module reports as an error, the Rust module returns failure and emits a diagnostic tied to the relevant file source context.

Traceability:
- `fromfile`
- `fromfile_error`
- `parseopt_file`

### SC-4: File tokenization is behaviorally compatible
File contents that rely on the original module’s word-splitting behavior are tokenized compatibly enough to preserve effective option parsing outcomes for supported inputs.

Traceability:
- `fromfile`
- `wordsplit`

### SC-5: Profile handler compatibility
For profile option invocations supported by the original module, the Rust module’s profile handler returns compatible success/failure results and produces equivalent parser-state effects.

Traceability:
- `optset_profile`

### SC-6: Initialization supports downstream flows
After parser initialization through the module’s hook, environment loading, file parsing, and profile processing can execute without initialization-related regressions relative to the original module.

Traceability:
- `init_hook`
- `parseopt_from_env`
- `fromfile`
- `optset_profile`

## Non-Goals

The Rust rewrite is not required by this module specification to provide:

- new option syntaxes not evidenced by the source functions,
- new configuration sources beyond environment and file/profile-related flows already present,
- new public APIs unrelated to the existing parser integration,
- thread-safety guarantees,
- serialization, persistence, or schema validation features,
- recovery workflows beyond the existing success/failure and diagnostic behavior.

## Acceptance Notes

Acceptance should be based on behavior observed through parser integration tests that exercise:

- parser initialization,
- environment-driven option loading,
- file-backed option parsing,
- contextual error reporting for file input,
- profile option handling.

The Rust rewrite may reorganize internal code structure, but the externally observable behavior defined above must remain compatible with the original module boundary.