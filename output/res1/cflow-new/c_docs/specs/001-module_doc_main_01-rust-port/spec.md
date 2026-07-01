# spec.md

## Title
Rust Port Functional Specification: `module_doc_main_01`

## Metadata
- Project: `cflow-new`
- Module: `module_doc_main_01`
- Category: `module_cluster`
- Target branch: `001-module_doc_main_01-rust-port`
- Generation date: `2026-06-11`

## Overview
This module consists of four small command-line programs under `doc/` that demonstrate or provide basic Unix-style command behavior through their `main` entry points.

The Rust rewrite must preserve the observable behavior evidenced by the source set:
- a directory-oriented command in `doc/d.c`
- a minimal argument-reporting/demo command in `doc/foo.c`
- a word/line/byte counting command in `doc/wc.c`
- a current-user reporting command in `doc/whoami.c`

The Rust version must implement the same functional boundaries as these standalone executables, with behavior traceable to the original entry points and their referenced system data.

## Feature Specification

### FS-1: Directory listing style command
Traceability: `doc/d.c`, `main`, `struct stat`, `struct dirent`

The module includes a command that inspects directory contents and reports information derived from directory entries and file status.

The Rust version must:
- accept command-line invocation through a `main` entry point
- access directory entries from a target directory context
- obtain file status information for entries
- emit output representing discovered entries and/or their associated status-derived information
- complete successfully for valid directory input and fail through process exit for invalid invocation or inaccessible filesystem targets, consistent with a command-line utility model

This requirement is limited to directory traversal and reporting behavior evidenced by use of directory entry and file status structures.

### FS-2: Minimal standalone command execution
Traceability: `doc/foo.c`, `main`

The module includes a very small standalone executable with no evidenced external data structures.

The Rust version must:
- provide an executable entry point for this program
- support direct execution from the command line
- preserve its role as a minimal demonstration or output-producing command

Because only the `main` function is evidenced, the Rust port must preserve externally observable command behavior without introducing additional documented capabilities.

### FS-3: Text counting command
Traceability: `doc/wc.c`, `main`

The module includes a command-line text counting utility analogous in role to a word-count program.

The Rust version must:
- accept command-line invocation through a `main` entry point
- read textual or byte-stream input from files and/or standard input as supported by the original command behavior
- compute counts corresponding to the utility’s purpose
- emit the computed counts in command-line output form
- support successful completion for readable input and failure reporting for unreadable or invalid inputs

At minimum, the Rust version must preserve counting behavior evidenced by the command’s identity and standalone `main` implementation.

### FS-4: Current user reporting command
Traceability: `doc/whoami.c`, `main`, `struct passwd`

The module includes a command that reports the identity of the current user using system account information.

The Rust version must:
- accept command-line invocation through a `main` entry point
- resolve the current process user identity via system account data
- emit the current user name in command-line output form
- terminate with an appropriate nonzero outcome if user resolution cannot be performed

This requirement is bounded to obtaining and reporting the current user identity as evidenced by use of password database information.

## User Scenarios & Testing

### Scenario 1: Inspect directory contents
Traceability: `doc/d.c`

A user runs the directory-oriented command against a directory path to inspect what entries exist and receive output reflecting those entries and their filesystem status.

#### Expected support
- Invocation from a shell with command-line arguments
- Processing of a real filesystem directory
- Output that corresponds to the directory contents
- Failure behavior when the directory cannot be accessed

#### Test ideas
- Run the command on a directory containing regular files and subdirectories; verify output is produced for entries discovered there.
- Run the command on a nonexistent path; verify the process exits with failure.
- Run the command on an unreadable directory; verify it reports failure rather than silent success.

### Scenario 2: Execute the minimal demo command
Traceability: `doc/foo.c`

A user runs the small standalone command directly to observe its output or completion behavior.

#### Expected support
- Direct process execution
- Deterministic observable behavior consistent with the original command

#### Test ideas
- Execute the binary without arguments and verify it produces the expected observable result.
- Execute it repeatedly and verify behavior is stable across runs.

### Scenario 3: Count content from a file
Traceability: `doc/wc.c`

A user runs the counting command on a text file to obtain counts for that file.

#### Expected support
- File-based input processing
- Emission of count values
- Failure reporting for unreadable files

#### Test ideas
- Provide a file with known line, word, and byte totals; verify output matches expected counts.
- Provide an empty file; verify zero-valued counts are reported appropriately.
- Provide a missing file path; verify the process exits with failure.

### Scenario 4: Count content from standard input
Traceability: `doc/wc.c`

A user pipes text into the counting command and expects counts for the provided stream.

#### Expected support
- Reading from standard input when invoked in that mode
- Emission of count values for the piped content

#### Test ideas
- Pipe a known string into the command and verify counts match the known content.
- Pipe multiline content and verify line counting behavior.

### Scenario 5: Report the current username
Traceability: `doc/whoami.c`, `struct passwd`

A user runs the identity-reporting command to print the current user name.

#### Expected support
- Lookup of the current process user in system account data
- Printing of the resolved username
- Failure if the user cannot be resolved

#### Test ideas
- Run under a normal user account; verify the printed name matches the environment’s actual current username.
- In a controlled test seam where user lookup fails, verify nonzero termination.

## Requirements

### Functional Requirements

#### FR-1: Multiple standalone executables
Traceability: `doc/d.c`, `doc/foo.c`, `doc/wc.c`, `doc/whoami.c`

The Rust port must provide four standalone executable behaviors corresponding to the four original `main` programs.

#### FR-2: Command-line entry behavior
Traceability: all evidenced `main` functions

Each executable must be invocable from the command line and perform its work entirely through process execution and standard command-line I/O conventions.

#### FR-3: Directory entry inspection
Traceability: `doc/d.c`, `struct dirent`

The directory command must enumerate entries from a directory source and use those entries as the basis of its output.

#### FR-4: File status inspection
Traceability: `doc/d.c`, `struct stat`

The directory command must obtain filesystem status information associated with inspected entries and reflect that information in its behavior or output.

#### FR-5: Counting input content
Traceability: `doc/wc.c`

The counting command must process input content and compute the counts that define its command purpose.

#### FR-6: Input source handling for counting
Traceability: `doc/wc.c`

The counting command must support the input source forms evidenced by command-line utility behavior in the original module, specifically file input and standard input operation as applicable to a `wc`-style program.

#### FR-7: Current user lookup
Traceability: `doc/whoami.c`, `struct passwd`

The identity command must resolve the current user through system account information and obtain a username for output.

#### FR-8: User-visible output
Traceability: all `main` functions

Each executable must produce user-visible output appropriate to its command purpose.

#### FR-9: Failure signaling
Traceability: all `main` functions

Each executable must terminate with a failure outcome when required input or system interaction cannot be completed.

### Key Entities

#### KE-1: Directory entry
Traceability: `doc/d.c`, `struct dirent`

Represents one item discovered during directory traversal.
Relationship: each directory entry may be paired with file status information for reporting.

#### KE-2: File status
Traceability: `doc/d.c`, `struct stat`

Represents metadata about a filesystem object associated with a directory entry.
Relationship: file status is derived per inspected entry in the directory command.

#### KE-3: User account record
Traceability: `doc/whoami.c`, `struct passwd`

Represents system account information used to resolve the current username.
Relationship: the identity command reads the username from this record for output.

#### KE-4: Process arguments and input streams
Traceability: all `main` functions; especially `doc/d.c`, `doc/wc.c`, `doc/whoami.c`

Represent the user-provided invocation context for each command.
Relationship: arguments select targets or modes; input streams provide content for the counting command.

## Success Criteria

### SC-1: Executable coverage
Traceability: all four source files

The Rust port builds and exposes four runnable command-line executables corresponding to the four original programs.

### SC-2: Directory command correctness
Traceability: `doc/d.c`, `struct dirent`, `struct stat`

When run against an accessible test directory, the directory command emits output derived from actual directory entries and their filesystem status, and exits successfully.

### SC-3: Directory command failure behavior
Traceability: `doc/d.c`

When run against a nonexistent or inaccessible directory, the directory command exits with a nonzero status.

### SC-4: Minimal command preservation
Traceability: `doc/foo.c`

The minimal standalone command executes successfully and preserves the original program’s observable behavior.

### SC-5: Counting accuracy
Traceability: `doc/wc.c`

For controlled inputs with known totals, the counting command reports counts that match expected results.

### SC-6: Standard input support for counting
Traceability: `doc/wc.c`

When supplied content through standard input, the counting command completes successfully and reports counts for that stream.

### SC-7: Counting error handling
Traceability: `doc/wc.c`

When asked to process an unreadable or missing file, the counting command exits with a nonzero status.

### SC-8: Current user reporting
Traceability: `doc/whoami.c`, `struct passwd`

Under normal system conditions, the identity command prints the current username and exits successfully.

### SC-9: Identity lookup failure signaling
Traceability: `doc/whoami.c`

If current-user resolution fails, the identity command exits with a nonzero status.

## Out of Scope
The Rust port specification does not require any capabilities not evidenced by the provided module analysis, including:
- new public APIs beyond the executable behaviors
- network services
- concurrency guarantees
- serialization formats
- configuration systems
- compatibility layers beyond preserving command behavior