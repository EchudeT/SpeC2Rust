# spec.md

## Title

Rust Port Functional Specification: `module_src_parseopt_file_07`

## Metadata

- **Project**: `cflow-new`
- **Module**: `module_src_parseopt_file_07`
- **Category**: `module_cluster`
- **Source file(s)**: `src/main.c`
- **Primary source functions**:
  - `optfile_lookup`
  - `parseopt_from_rc`
- **Rust branch target**: `070-module_src_parseopt_file_07-rust-port`
- **Generation date**: `2026-06-11`

## Overview

This module is responsible for locating and loading option definitions from configuration-style option files used during program startup or option parsing. Its behavior is centered on two linked responsibilities:

1. Resolving a named option file into a usable `parseopt_file` description.
2. Initiating option parsing from an rc/configuration file path.

The Rust rewrite must preserve these functional boundaries: it must support lookup of option file input by name, populate the option-file descriptor used by the surrounding option parser, and trigger parsing of options from rc-style files in the same situations covered by the C module.

## Feature Specification

### Feature: Option file lookup

The module shall provide the ability to resolve a user- or program-supplied option file name into a `parseopt_file` instance suitable for subsequent parsing.

Observed functional scope from `optfile_lookup`:

- Accept an option file name and a mutable option-file descriptor.
- Determine whether the named file can be located and used.
- Fill the descriptor with the information needed by downstream option parsing logic.
- Return a status code indicating success or failure of lookup.

The Rust version must preserve this behavior, including:

- name-based lookup behavior,
- production of a populated option-file descriptor on success,
- failure reporting through function result/status,
- compatibility with downstream parsing that expects a `parseopt_file`-equivalent entity.

### Feature: Parse options from rc/configuration file

The module shall support reading options from an rc/configuration file by file name and feeding them into the existing parseopt flow.

Observed functional scope from `parseopt_from_rc`:

- Accept a file path/name for an rc/configuration source.
- Use option-file parsing support to process options from that file.
- Return a status code representing whether parsing from that rc source succeeded.

The Rust version must preserve this behavior, including:

- initiating parseopt processing from a named rc file,
- using the same logical option-file mechanism as lookup/population,
- propagating failure when the rc file cannot be used or parsed through this module’s scope.

### Feature boundary

This module’s scope is limited to option-file discovery and rc-file-driven option parsing orchestration. The Rust rewrite must not introduce unrelated capabilities beyond those evidenced by the source analysis.

## User Scenarios & Testing

### Scenario 1: Program loads startup options from a named rc file

A program starts and has a configured rc file path. The module is invoked with that file name. The module resolves the file as needed and passes its contents into the option parser.

**Expected outcome**:
- A successful status is returned when the file is valid and usable.
- Options contained in the rc source are made available to the surrounding parseopt logic.

**Testing guidance**:
- Provide a valid rc file path.
- Verify the Rust function returns success.
- Verify downstream parseopt-visible state reflects options from the file.

### Scenario 2: Program attempts to use a missing or unusable option file

A file name is supplied for option loading, but the file cannot be located or cannot be used as an option source.

**Expected outcome**:
- Lookup or rc parsing returns failure status.
- No success result is reported for the unresolved file.

**Testing guidance**:
- Supply a nonexistent file name.
- Verify lookup fails.
- Supply the same name to rc parsing entry and verify failure is propagated.

### Scenario 3: Option file lookup prepares parse descriptor for later parsing

The surrounding parser needs a `parseopt_file` descriptor before processing file contents. The module performs lookup and populates that descriptor.

**Expected outcome**:
- On success, the descriptor is populated sufficiently for subsequent parser use.
- The result is accepted by the downstream option-file parsing path.

**Testing guidance**:
- Invoke lookup with a valid file name and an empty descriptor.
- Verify success is returned.
- Verify required descriptor fields are populated in a manner accepted by the consuming parser logic.

### Scenario 4: rc parsing uses the same option-file path handling as direct lookup

An rc file path that is valid for direct option-file resolution is also passed to the rc parsing entrypoint.

**Expected outcome**:
- The rc parsing entrypoint succeeds for the same valid path class supported by lookup.
- Failures in lookup-equivalent conditions are reflected by rc parsing.

**Testing guidance**:
- Test valid and invalid file names through both pathways.
- Verify status consistency between direct lookup failure/success and rc parsing behavior where applicable.

## Requirements

### Functional Requirements

#### FR-1: Resolve named option files
The module shall accept a file name and attempt to resolve it into a usable option-file source for parseopt processing.

**Traceability**: `src/main.c`, `optfile_lookup`, `parseopt_file`

#### FR-2: Populate option-file descriptor on successful lookup
When lookup succeeds, the module shall populate the provided option-file descriptor with the data required by the surrounding option-file parsing logic.

**Traceability**: `src/main.c`, `optfile_lookup`, `parseopt_file`

#### FR-3: Report lookup success or failure by return status
The module shall return an integer/status result indicating whether option-file lookup succeeded.

**Traceability**: `src/main.c`, `optfile_lookup`

#### FR-4: Initiate option parsing from rc/configuration file input
The module shall accept an rc/configuration file name and invoke the option-file parsing flow for that file.

**Traceability**: `src/main.c`, `parseopt_from_rc`, `parseopt_file`

#### FR-5: Report rc parsing success or failure by return status
The module shall return an integer/status result indicating whether parsing from the rc/configuration file succeeded.

**Traceability**: `src/main.c`, `parseopt_from_rc`

#### FR-6: Interoperate with the module’s existing parseopt data model
The Rust rewrite shall preserve compatibility with the surrounding parseopt/optdef-based option parsing model used in `src/main.c`.

**Traceability**: `src/main.c`, `parseopt`, `optdef`, `parseopt_from_rc`, `optfile_lookup`

#### FR-7: Support file-backed option input used by surrounding parser infrastructure
The module shall continue to operate on file-backed option input as indicated by its interaction with `parseopt_file` and referenced file/status-related types.

**Traceability**: `src/main.c`, `optfile_lookup`, `parseopt_from_rc`, `parseopt_file`, `stat`

### Key Entities

#### `parseopt_file`
A file-oriented option input descriptor used by this module as the bridge between file lookup and the surrounding option parser. It is populated during lookup and consumed by subsequent parsing flow.

**Relationship**:
- Produced or filled by `optfile_lookup`.
- Used as the file representation supporting `parseopt_from_rc`.

#### `parseopt`
The broader option parsing context/model used elsewhere in `src/main.c`. This module operates within that ecosystem and must remain behaviorally compatible with it.

**Relationship**:
- `parseopt_from_rc` feeds option input into this parsing framework.
- `parseopt_file` acts as file-backed input to this framework.

#### `optdef`
The option definition model associated with `parseopt`. It represents the known option schema that rc-file parsing ultimately targets.

**Relationship**:
- Parsed rc-file content is interpreted against `optdef` definitions through the surrounding parseopt system.

#### `wordsplit`
A referenced tokenization/splitting structure indicating that file-sourced option text may be processed as split words/tokens by surrounding parser support.

**Relationship**:
- Relevant to how rc-file content is transformed into parseable options in the larger parseopt pipeline.

#### `stat`
A referenced file metadata structure indicating interaction with filesystem-backed option sources.

**Relationship**:
- Supports file existence/usability decisions associated with lookup behavior.

## Success Criteria

### SC-1: Valid option file lookup succeeds
Given a valid option file name supported by the original module behavior, the Rust implementation returns success and produces a usable `parseopt_file`-equivalent descriptor.

**Traceability**: `optfile_lookup`, `parseopt_file`

### SC-2: Invalid or missing option file lookup fails
Given a missing or unusable option file name, the Rust implementation returns failure from lookup.

**Traceability**: `optfile_lookup`

### SC-3: Valid rc file parsing succeeds
Given an rc/configuration file that the original module would process successfully, the Rust implementation returns success from rc parsing and drives the surrounding option parser accordingly.

**Traceability**: `parseopt_from_rc`, `parseopt`

### SC-4: rc parsing failure is propagated
Given an rc/configuration file that cannot be located or processed through this module’s supported flow, the Rust implementation returns failure rather than reporting success.

**Traceability**: `parseopt_from_rc`, `optfile_lookup`

### SC-5: Descriptor output is accepted by downstream parse logic
The descriptor state produced by successful lookup is sufficient for downstream option-file parsing without requiring out-of-module reconstruction.

**Traceability**: `optfile_lookup`, `parseopt_file`, `parseopt`

### SC-6: No unsupported feature expansion is introduced
The Rust module remains limited to option-file lookup and rc/configuration-file-driven parse initiation, without adding unrelated capabilities not evidenced by `src/main.c`.

**Traceability**: `src/main.c`, `optfile_lookup`, `parseopt_from_rc`

## Non-Goals

The Rust rewrite specification does not require this module to introduce:

- new public APIs beyond those needed to preserve the original functional boundary,
- new configuration formats,
- concurrency/thread-safety guarantees,
- serialization/export behavior,
- recovery workflows beyond status-based success/failure reporting,
- functionality unrelated to option-file lookup and rc/configuration-file parsing.

## Acceptance Notes

Conformance should be evaluated by comparing the Rust module’s externally observable behavior against the C module’s behavior in the covered scenarios:

- successful lookup of valid option files,
- failure on invalid/unavailable files,
- successful initiation of rc-file option parsing,
- correct failure propagation when rc parsing cannot proceed.