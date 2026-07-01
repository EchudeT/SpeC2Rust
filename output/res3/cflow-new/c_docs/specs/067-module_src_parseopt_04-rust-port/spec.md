# spec.md

## Title

Rust Functional Specification for `module_src_parseopt_04`

## Metadata

- Project: `cflow-new`
- Module: `module_src_parseopt_04`
- Category: `module_cluster`
- Source basis: `src/main.c`
- Rust branch target: `067-module_src_parseopt_04-rust-port`
- Generation date: `2026-06-17`

## Overview

This module covers the part of the option-processing system responsible for:

- loading option text from environment-controlled sources,
- reading option content from profile/config files,
- reporting file-originated parsing errors with source context,
- applying a profile option that points to such a file, and
- performing initialization-time setup that activates this behavior.

The Rust rewrite must preserve the observable behavior of these responsibilities as they exist in the analyzed C module boundaries. The specification is limited to behavior evidenced by the following functions:

- `parseopt_from_env`
- `fromfile_error`
- `fromfile`
- `optset_profile`
- `init_hook`

## Feature Specification

### 1. Environment-triggered option loading

The module must support initialization logic that checks whether options should be loaded from environment-controlled input and, when applicable, routes that input into the option parser.

This behavior is evidenced by `parseopt_from_env` and `init_hook`.

Required behavior:

- During parser initialization, the module must perform an environment-based option loading step.
- This step must be integrated through the module’s initialization hook rather than requiring a separate user action.
- If the environment-controlled source is absent or not applicable, initialization must still complete without requiring profile input.

The Rust version must preserve the same role in startup flow: initialization triggers any environment-based parseopt loading that this module is responsible for.

### 2. File-backed option/profile processing

The module must support reading options from a file-like profile source represented by a `parseopt_file` entity.

This behavior is evidenced by `fromfile` and `optset_profile`.

Required behavior:

- A profile option may name or provide an argument identifying an option file to read.
- The module must open/process that source through the file-backed parsing path.
- The file content must be interpreted as option input rather than opaque data.
- Processing must return a status indicating success or failure.
- Failure in file-backed processing must be reportable to the surrounding parseopt flow.

The Rust version must preserve the file-backed option ingestion path and the success/failure contract needed by the caller.

### 3. Source-aware error reporting for file parsing

When option parsing fails while reading from a file/profile source, the module must report errors using source-aware diagnostics tied to the active parse context and file context.

This behavior is evidenced by `fromfile_error`.

Required behavior:

- Errors raised while processing a profile file must be emitted through a dedicated reporting path for file-originated parsing.
- The reporting path must accept:
  - the parse option context,
  - a severity/priority indicator,
  - a formatted message, and
  - variable arguments used to build the final diagnostic.
- The diagnostic must be associated with the currently processed file context rather than treated as an unscoped generic error.

The Rust version must preserve differentiated error reporting for file/profile parsing and keep the association between the parse context and the file-originated diagnostic.

### 4. Profile option application

The module must implement handling for a profile-related option definition that causes option text to be loaded from an external source.

This behavior is evidenced by `optset_profile`.

Required behavior:

- The profile option handler must accept the active parser context, the option definition being applied, and the option argument.
- The handler must interpret the argument as input for file-backed profile loading.
- The handler must return an integer status compatible with the surrounding parseopt option-setting contract.
- The handler must use the module’s file-backed processing path rather than implementing unrelated side effects.

The Rust version must preserve this profile option semantics and the caller-visible status outcome.

### 5. Initialization hook integration

The module must integrate its setup behavior into parseopt initialization through an initialization hook.

This behavior is evidenced by `init_hook`.

Required behavior:

- Initialization must invoke module-specific setup using the active `parseopt` context.
- The initialization hook must include the environment-based option loading behavior handled by this module.
- The initialization step must be side-effectful only in the scope evidenced by this module: activating parseopt-related setup and environment-derived loading.

The Rust version must preserve this initialization integration point.

## User Scenarios & Testing

### Scenario 1: Startup with environment-provided options

A program starts and initializes its parseopt context. During initialization, the module checks the relevant environment-controlled option source and loads options from it if present.

Expected support in Rust:

- Initialization triggers the environment-loading path automatically.
- If the environment source is valid, contained options are processed.
- If the environment source is absent, initialization still succeeds without a profile-related error.

Testable outcomes:

- With environment input present, parseopt state reflects options loaded from that input.
- With environment input absent, initialization completes and no spurious file/profile error is reported.

### Scenario 2: Applying a profile option that names a file

A caller applies the profile-related option with an argument that identifies a profile/options file.

Expected support in Rust:

- The profile option handler accepts the argument and starts file-backed option processing.
- The file content is parsed as option input.
- The handler returns a success status when the file is processed successfully.

Testable outcomes:

- A valid profile file causes the expected options to be applied through the parseopt system.
- The profile option handler returns success on valid input.

### Scenario 3: Invalid content inside a profile file

A profile file exists but contains malformed option content.

Expected support in Rust:

- File-backed parsing fails through the module’s error path.
- The error is reported as originating from file/profile processing, not as an unrelated generic failure.
- The profile application path returns a failure status compatible with the caller contract.

Testable outcomes:

- A malformed profile file causes a non-success return from file-backed processing or the profile option handler.
- Diagnostic output includes file-context-aware reporting behavior.

### Scenario 4: File access or processing failure during profile load

A profile option references a source that cannot be processed successfully.

Expected support in Rust:

- The file-backed processing path reports an error through the file-specific diagnostic path.
- The caller receives a failure result.

Testable outcomes:

- Failure to process the requested profile source yields a failure status.
- A diagnostic is emitted through the dedicated file-originated error reporting mechanism.

### Scenario 5: Initialization hook chaining

The parseopt system invokes its initialization hook for a newly created parse context.

Expected support in Rust:

- The module-specific initialization behavior runs through the hook.
- Environment-based option loading is included in this hook behavior.

Testable outcomes:

- Invoking initialization performs the same environment-loading step that would otherwise be expected at startup.
- No separate manual trigger is required to activate this module’s initialization behavior.

## Requirements

### Functional Requirements

#### FR-1: Environment parseopt loading
The module shall perform an environment-based option loading step during parseopt initialization.

Traceability:
- `parseopt_from_env`
- `init_hook`

#### FR-2: Initialization hook behavior
The module shall expose initialization behavior that operates on an active `parseopt` context and includes this module’s environment-loading responsibility.

Traceability:
- `init_hook`
- `parseopt`

#### FR-3: File-backed option ingestion
The module shall process option input from a `parseopt_file` source and return a success/failure status for that processing.

Traceability:
- `fromfile`
- `parseopt_file`

#### FR-4: File-originated diagnostic reporting
The module shall provide a reporting path for errors encountered while processing file-backed option input, using the active parse context plus a severity/priority and formatted message input.

Traceability:
- `fromfile_error`
- `parseopt`

#### FR-5: Profile option handling
The module shall implement a profile option setter that receives a `parseopt` context, an `optdef`, and an argument string, and uses that argument to trigger profile/file-backed option loading.

Traceability:
- `optset_profile`
- `parseopt`
- `optdef`

#### FR-6: Propagated failure on profile load errors
The module shall return failure status from profile/file-backed processing when the referenced source cannot be processed successfully or contains invalid option input.

Traceability:
- `fromfile`
- `optset_profile`
- `fromfile_error`

### Key Entities

#### `parseopt`
The active option parsing context.

Role in this module:

- anchors initialization behavior,
- is passed into profile option handling,
- is used when reporting file-originated errors.

Relationship summary:

- `init_hook` operates on `parseopt`.
- `optset_profile` applies profile behavior within a `parseopt`.
- `fromfile_error` reports errors in relation to a `parseopt`.

#### `optdef`
The option definition being applied.

Role in this module:

- identifies the option invocation handled by the profile option setter.

Relationship summary:

- `optset_profile` receives `optdef` alongside `parseopt` and the option argument.

#### `parseopt_file`
The file/profile input context for reading option data from a file-backed source.

Role in this module:

- represents the file-oriented source processed by the module.

Relationship summary:

- `fromfile` consumes `parseopt_file`.
- file-related failures during this processing are reported through `fromfile_error`.

#### `wordsplit`
A referenced parsing-related type associated with tokenization/splitting support used by the surrounding system.

Role in this module:

- only evidenced as a referenced type name, not as a locally defined module boundary.

Relationship summary:

- no additional Rust-facing functionality shall be inferred from this module alone.

## Success Criteria

1. Initialization of the Rust parseopt context executes the module’s hook behavior and includes the environment-based option loading step.
   - Traceability: `init_hook`, `parseopt_from_env`

2. The Rust implementation can process a valid profile/options file through the file-backed path and report success to its caller.
   - Traceability: `fromfile`, `optset_profile`

3. Applying the profile option in Rust with a valid argument triggers file-backed option processing rather than being ignored or treated as a no-op.
   - Traceability: `optset_profile`

4. If file-backed option processing fails, the Rust implementation reports the error through a file-specific diagnostic path associated with the active parse context.
   - Traceability: `fromfile_error`, `fromfile`

5. If a profile source is invalid or malformed, the Rust implementation returns a failure status from the relevant processing path.

6. The Rust rewrite does not add unrelated externally visible capabilities beyond environment-triggered loading, profile/file-backed option ingestion, file-scoped diagnostics, and initialization-hook integration.
   - Traceability: bounded by `parseopt_from_env`, `fromfile_error`, `fromfile`, `optset_profile`, `init_hook`

## Out of Scope

The Rust rewrite specification does not require any capability not evidenced in the analyzed module boundary, including:

- defining new public configuration formats,
- adding new option sources beyond environment-triggered input and profile/file-backed input,
- introducing recovery behavior beyond success/failure reporting,
- adding serialization, FFI, concurrency guarantees, or performance requirements.