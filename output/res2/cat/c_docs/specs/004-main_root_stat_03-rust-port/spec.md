# spec.md

## Title

Functional Specification: `main_root_stat_03` Rust Port

## Metadata

- Project: `cat`
- Module: `main_root_stat_03`
- Category: `main_cluster`
- Target branch: `004-main_root_stat_03-rust-port`
- Source basis: `cat.c`, `fcntl.c`
- Generation date: `2026-06-07`

## Overview

This module contains the program entry-point behavior for the `cat` utility and a local file-control helper used by that entry-point flow. The Rust rewrite must preserve the observable behavior evidenced by the source module: argument-driven startup, option handling through a command-line option table, file status inspection through `stat`-related data, and file-descriptor control through the local `klibc_fcntl` helper.

The specification covers only the functionality evidenced by:

- `main` in `cat.c`
- `klibc_fcntl` in `fcntl.c`
- the locally referenced `struct stat` and `struct option` data used by those flows

## Scope

### In Scope

- Program startup behavior driven by `main`
- Consumption of command-line arguments
- Use of an option-definition table to interpret supported options
- File metadata inspection needed by startup/control flow
- File-descriptor control behavior exposed through the internal `klibc_fcntl` helper
- Return of process exit status based on execution outcome

### Out of Scope

- Any capabilities not evidenced by this module slice
- New command-line options not represented by the module’s option-table-driven behavior
- New public APIs beyond the Rust equivalent of the module’s existing role
- Concurrency, recovery, persistence, serialization, networking, or benchmark requirements

## Feature Specification

### Feature 1: Command-line entry-point execution

The Rust version must provide the functional equivalent of the C module’s `main` entry point.

Behavior required:

- Accept the program argument vector and evaluate invocation parameters.
- Interpret options through the module’s option-table-driven command-line handling.
- Determine execution flow for the invocation based on parsed options and operands.
- Perform any file-status checks evidenced by the use of `struct stat` in the entry-point flow.
- Produce an integer process result indicating success or failure.

The Rust version must preserve externally observable behavior expected from a command-line utility entry point: arguments are read once at startup, option handling influences subsequent execution, and the program terminates with an exit status.

### Feature 2: Option-table-based argument handling

The module references an option structure local to the `main` flow. The Rust rewrite must preserve the role of that structure:

- represent supported command-line options
- enable mapping from received arguments to internal control decisions
- support the same option-driven branches required by the source flow

This specification does not require reproducing C-specific parsing mechanics, but the Rust implementation must preserve the same supported option set and resulting behavior that the source module uses.

### Feature 3: File status inspection in startup flow

The entry-point logic references `struct stat`, evidencing a dependency on file metadata checks. The Rust rewrite must perform equivalent file status inspection wherever that metadata influences behavior.

At minimum, the Rust version must preserve:

- obtaining file status information for paths or descriptors involved in the entry flow
- making the same user-visible decisions that depend on that status information
- preserving success/failure outcomes when file status retrieval succeeds or fails

### Feature 4: Internal file-descriptor control helper

The module includes `klibc_fcntl`, a local helper wrapping file-control actions on a file descriptor with a variable argument.

The Rust rewrite must preserve the helper’s functional role:

- accept a file descriptor identifier
- accept a control action selector
- accept an optional action argument when the action requires one
- perform the corresponding file-control operation
- return success or failure in a way that preserves the caller-visible outcome of the original module flow

The Rust implementation may use safe or low-level Rust facilities internally, but the supported control actions and their effect on module behavior must remain equivalent to the source.

## User Scenarios & Testing

### Scenario 1: Program invoked with no additional operands

A user runs the utility with only the program name or with the minimal invocation supported by the module.

Expected support:

- startup completes without argument parsing failure
- default control flow is selected according to the source behavior
- the process exits with the same success/failure semantics as the C module

Testing focus:

- invocation with `argc == 1`
- exit status matches source behavior
- no unsupported-option error is produced unless the source does so

### Scenario 2: Program invoked with supported options

A user supplies one or more supported command-line options.

Expected support:

- the option table recognizes the same supported options as the source module
- recognized options alter execution flow exactly as in the C module
- conflicting or invalid combinations are handled with the same observable outcome as the source

Testing focus:

- each supported option is accepted
- option-dependent exit status and output behavior match the source
- repeated or combined options follow source semantics

### Scenario 3: Program invoked with file operands requiring status checks

A user supplies one or more file inputs whose handling depends on file metadata.

Expected support:

- the Rust version retrieves the needed file status information
- metadata-dependent branches match the C behavior
- failures to retrieve status produce the same visible failure class and exit status behavior

Testing focus:

- regular existing input
- non-existent or inaccessible input
- cases where status information changes control flow

### Scenario 4: Internal descriptor-control operation is needed

During execution, the module needs to apply a file-control action to an open descriptor.

Expected support:

- the internal helper accepts the descriptor and action
- actions requiring an additional argument receive and use it
- success or failure is reported consistently with the source behavior

Testing focus:

- action invocation succeeds for valid input
- invalid descriptor or unsupported action failure propagates consistently
- caller-visible result matches C behavior

### Scenario 5: Invalid command-line usage

A user provides arguments that the module does not accept.

Expected support:

- invalid usage is detected during startup processing
- the program terminates with failure semantics matching the source
- the resulting observable behavior is consistent for all unsupported inputs evidenced by the option handling flow

Testing focus:

- unknown option
- malformed option argument, if any are supported by the source option table
- invalid operand/option combinations

## Requirements

### Functional Requirements

#### FR-1: Entry-point argument processing

The Rust module shall implement the functional equivalent of the C `main` entry point, consuming `argc`/`argv`-equivalent command-line input and determining program execution outcome from it.

Traceability: `cat.c: main`

#### FR-2: Supported option recognition

The Rust module shall recognize and process the same supported command-line options represented by the module’s option-table usage.

Traceability: `cat.c: main`, local `struct option`

#### FR-3: Option-driven control flow

The Rust module shall apply option results to execution decisions in the same way as the source `main` flow, including success and failure paths.

Traceability: `cat.c: main`, local `struct option`

#### FR-4: File status retrieval

The Rust module shall retrieve file status information wherever the source entry-point logic depends on `struct stat`.

Traceability: `cat.c: main`, local `struct stat`

#### FR-5: Metadata-dependent behavior preservation

The Rust module shall preserve all user-visible decisions in `main` that depend on file status results, including behavior when status retrieval fails.

Traceability: `cat.c: main`, local `struct stat`

#### FR-6: Process exit status propagation

The Rust module shall terminate with success or failure codes consistent with the execution result determined by the source entry-point logic.

Traceability: `cat.c: main`

#### FR-7: Internal descriptor control support

The Rust module shall provide an internal file-descriptor control operation equivalent to `klibc_fcntl`, accepting a descriptor, an action code, and an optional action argument.

Traceability: `fcntl.c: klibc_fcntl`

#### FR-8: Descriptor control result handling

The Rust module shall preserve the success/failure semantics of the descriptor control helper, including propagation of failure to caller-visible outcomes where applicable.

Traceability: `fcntl.c: klibc_fcntl`

### Key Entities

#### Entity: Program invocation context

Represents the command-line input supplied to the entry point.

Relationships:

- consumed by the entry-point logic
- interpreted through the option definition table
- may identify file operands that require status inspection

Traceability: `cat.c: main`

#### Entity: Option definition entry (`struct option`)

Represents one supported command-line option used by the startup parsing flow.

Relationships:

- grouped into the option table used by `main`
- maps input arguments to internal control decisions

Traceability: local `struct option` reference in `cat.c`

#### Entity: File status record (`struct stat`)

Represents file metadata consulted by the module.

Relationships:

- populated by file status retrieval
- read by the entry-point flow to choose behavior
- also referenced in `fcntl.c`, evidencing shared reliance on POSIX-style file metadata concepts

Traceability: local `struct stat` references in `cat.c` and `fcntl.c`

#### Entity: File descriptor control request

Represents the combination of:

- file descriptor
- control action
- optional action argument

Relationships:

- supplied to the internal descriptor-control helper
- yields success/failure that affects higher-level execution flow

Traceability: `fcntl.c: klibc_fcntl`

## Success Criteria

### SC-1: Entry-point parity

For the same supported command-line inputs, the Rust module produces the same success/failure class and exit status as the source module.

Traceability: `cat.c: main`

### SC-2: Option handling parity

All command-line options represented by the source module’s option-table-driven flow are accepted or rejected identically in the Rust version.

Traceability: `cat.c: main`, local `struct option`

### SC-3: File status decision parity

For inputs whose behavior depends on file metadata, the Rust version makes the same observable control-flow decisions as the source, including failure behavior when status retrieval is unsuccessful.

Traceability: `cat.c: main`, local `struct stat`

### SC-4: Descriptor control parity

For all descriptor control actions used by this module, the Rust helper returns success or failure consistently with `klibc_fcntl` and preserves the same caller-visible outcome.

Traceability: `fcntl.c: klibc_fcntl`

### SC-5: Invalid invocation handling parity

For unsupported or malformed invocation patterns evidenced by the source parsing flow, the Rust version fails in the same observable manner, including exit status class.

Traceability: `cat.c: main`

## Acceptance Notes

- Conformance is based on observable behavior from the source module, not on reproducing C-specific internals.
- Where the source module relies on platform file status or file-control behavior, the Rust port must preserve the same effective semantics on the target platform.
- No additional functionality is required beyond what is evidenced by the cited source elements.