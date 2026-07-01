# spec.md

## Title

Rust Port Functional Specification: `module_doc_main_01`

## Document Control

- **Project**: `cflow-new`
- **Module**: `module_doc_main_01`
- **Category**: `module_cluster`
- **Rust Branch**: `001-module_doc_main_01-rust-port`
- **Generation Date**: 2026-06-17

## Overview

This module cluster contains four standalone command-line program entrypoints under `doc/`. The Rust rewrite must preserve the observable behavior of these small utilities as documented by the available source evidence:

- a directory-listing style program in `doc/d.c`
- a minimal output program in `doc/foo.c`
- a word/line/byte counting style program in `doc/wc.c`
- a current-user reporting program in `doc/whoami.c`

The Rust version must implement the same functional scope present in these sources: command-line execution, interaction with filesystem or user-account information where evidenced, and text output of computed or fixed results.

## In Scope

The Rust port must cover the behavior implied by the following source-backed entrypoints and entities:

- `doc/d.c` main program behavior, including use of file status and directory entry information
- `doc/foo.c` main program behavior
- `doc/wc.c` main program behavior
- `doc/whoami.c` main program behavior, including use of password database user identity information

## Out of Scope

The Rust port specification does not require any capability not evidenced by the provided module analysis, including but not limited to:

- new subcommands or merged CLI design
- network behavior
- persistent storage
- concurrency guarantees
- library/FFI interfaces
- configuration file handling
- extended error recovery beyond normal command-line utility behavior

## Feature Specification

### Feature 1: Directory-oriented command execution

A standalone executable corresponding to `doc/d.c` must run from the command line and produce output based on directory and file metadata.

Evidence shows use of:

- `main (int argc, char **argv)` in `doc/d.c`
- `struct stat`
- `struct dirent`

From this, the Rust version must support:

- command-line invocation with arguments
- inspection of filesystem objects
- reading directory entries
- producing textual output derived from directory contents and/or file status information

The port must preserve the original utility’s externally visible behavior for valid and invalid invocations as expressed by the C source.

### Feature 2: Minimal standalone output program

A standalone executable corresponding to `doc/foo.c` must run with no required structured inputs beyond process invocation and produce its defined output.

Evidence shows only:

- `main()` in `doc/foo.c`

From this, the Rust version must support:

- direct command execution
- the same observable program termination behavior and text output as the C version

No additional modes or options are required unless directly present in the source.

### Feature 3: Text counting command execution

A standalone executable corresponding to `doc/wc.c` must run from the command line and report counts computed from text input.

Evidence shows:

- `main (int argc, char **argv)` in `doc/wc.c`

Given the file naming and command-style entrypoint structure, the Rust version must preserve the source-backed behavior of this counting utility, including:

- command-line invocation
- consumption of input from the source forms supported by the C program
- computation and reporting of the counts performed by the original utility
- output formatting compatible with the original utility’s observed behavior

The exact counted dimensions and accepted input forms must match the C source, not exceed it.

### Feature 4: Current-user identity reporting

A standalone executable corresponding to `doc/whoami.c` must run from the command line and print the current user identity.

Evidence shows use of:

- `main (int argc, char **argv)` in `doc/whoami.c`
- `struct passwd`

From this, the Rust version must support:

- command-line invocation
- obtaining the current process user identity through system account information
- printing the resolved user name in the same observable form as the C version

## User Scenarios & Testing

### Scenario 1: Run the directory utility against a target location

**Given** a user invokes the Rust replacement for the `doc/d.c` program
**When** the target path and any supported arguments are provided
**Then** the program reads directory contents and relevant file metadata
**And** prints results in the same form as the original C utility.

**Testing approach**
- Compare Rust and C outputs for the same directory inputs.
- Include directories with multiple entries.
- Include at least one invalid or inaccessible path and compare failure behavior.

### Scenario 2: Run the minimal utility

**Given** a user executes the Rust replacement for `doc/foo.c`
**When** the program starts
**Then** it emits the same output and exits with the same success/failure behavior as the C version.

**Testing approach**
- Run both binaries with no arguments and compare stdout, stderr, and exit status.

### Scenario 3: Count text input

**Given** a user invokes the Rust replacement for `doc/wc.c`
**When** text input is supplied using the same supported invocation forms as the C version
**Then** the program computes and prints the same counts as the original utility.

**Testing approach**
- Compare outputs for empty input.
- Compare outputs for single-line and multi-line text.
- Compare outputs for input containing spaces, tabs, and trailing newlines.
- If file operands are supported by the C source, compare results on one and multiple files.

### Scenario 4: Report the current user

**Given** a user executes the Rust replacement for `doc/whoami.c`
**When** the process resolves the current account identity
**Then** the program prints the same user name text as the C version.

**Testing approach**
- Run both binaries under the same user account and compare stdout and exit status.
- If identity lookup fails in a controlled test environment, compare error behavior.

## Requirements

### Functional Requirements

#### FR-1: Standalone executable parity
The Rust port must provide executable behavior corresponding to each of the four documented C entrypoints: `doc/d.c`, `doc/foo.c`, `doc/wc.c`, and `doc/whoami.c`.

**Traceability**: `main` functions in all four source files.

#### FR-2: Command-line invocation handling
For programs whose C entrypoints accept `argc` and `argv`, the Rust version must accept command-line arguments and preserve the original invocation-dependent behavior.

**Traceability**: `doc/d.c:84-105`, `doc/wc.c:126-140`, `doc/whoami.c:25-34`.

#### FR-3: Filesystem metadata and directory traversal behavior
The Rust replacement for `doc/d.c` must obtain and use directory entry information and file status information to produce its output.

**Traceability**: `doc/d.c`; `struct dirent`; `struct stat`.

#### FR-4: Minimal fixed-behavior execution
The Rust replacement for `doc/foo.c` must perform the same top-level observable action as the C program when executed, without requiring unsupported additional inputs.

**Traceability**: `doc/foo.c:2-8`.

#### FR-5: Input counting behavior
The Rust replacement for `doc/wc.c` must consume input in the forms accepted by the original program and emit the same computed counts and output structure.

**Traceability**: `doc/wc.c:126-140`.

#### FR-6: Current user lookup behavior
The Rust replacement for `doc/whoami.c` must resolve the current user via system account information and print the same user-facing identity result as the C program.

**Traceability**: `doc/whoami.c:25-34`; `struct passwd`.

#### FR-7: Observable error-path compatibility
For invalid invocations or lookup/access failures covered by the original programs, the Rust version must preserve equivalent observable behavior at the CLI level, including failure indication and user-visible messages where present.

**Traceability**: all four `main` entrypoints; directly applicable to command-line utility behavior in `doc/d.c`, `doc/wc.c`, and `doc/whoami.c`.

### Key Entities

#### Entity 1: Filesystem status record
Represents metadata about a filesystem object used by the directory-oriented program.

- Source entity: `struct stat` in `doc/d.c`
- Role: supports output decisions based on file properties

#### Entity 2: Directory entry record
Represents an entry read from a directory.

- Source entity: `struct dirent` in `doc/d.c`
- Role: provides the set of items examined by the directory-oriented program

#### Entity 3: User account record
Represents the current user’s account information.

- Source entity: `struct passwd` in `doc/whoami.c`
- Role: provides the user name printed by the identity-reporting program

#### Entity Relationships

- The directory-oriented utility relates **directory entry records** to **filesystem status records** in order to inspect directory contents and report results.
- The user-identity utility reads a **user account record** to derive the printed current-user name.
- The counting and minimal-output utilities are defined primarily by process input/output behavior, with no additional core structured entities evidenced in the module analysis.

## Success Criteria

### SC-1: Executable coverage
All four source-backed utilities are available in the Rust branch as runnable command-line programs preserving the original utility boundaries.

**Measured by**
- Presence of four corresponding runnable executables or equivalently separated program entrypoints.
- Manual or automated execution succeeds for each program.

### SC-2: Output parity for the minimal utility
For the `doc/foo.c` replacement, stdout, stderr, and exit status match the C program for default execution.

**Measured by**
- Byte-for-byte comparison of process outputs and status under no-argument invocation.

### SC-3: Directory utility behavioral parity
For representative valid and invalid directory/path inputs, the Rust replacement for `doc/d.c` produces the same user-visible results as the C program.

**Measured by**
- Matching stdout/stderr and exit status across a regression set including:
  - an existing readable directory
  - a missing path
  - an inaccessible path, if reproducible in test setup

### SC-4: Counting utility result parity
For representative text inputs accepted by the original `doc/wc.c`, the Rust replacement reports identical counts and output formatting.

**Measured by**
- Matching stdout/stderr and exit status for:
  - empty input
  - simple ASCII text
  - multi-line text
  - whitespace-rich text
  - file-based inputs if supported by the C source

### SC-5: User identity parity
For the same executing account, the Rust replacement for `doc/whoami.c` prints the same user name text and terminates equivalently to the C version.

**Measured by**
- Matching stdout/stderr and exit status when both binaries are run under the same user context.

### SC-6: No unsupported feature expansion
The Rust port does not introduce new documented end-user functionality beyond the behavior evidenced by the provided C sources.

**Measured by**
- Review of CLI behavior and user-facing outputs against source-backed scope in `doc/d.c`, `doc/foo.c`, `doc/wc.c`, and `doc/whoami.c`.