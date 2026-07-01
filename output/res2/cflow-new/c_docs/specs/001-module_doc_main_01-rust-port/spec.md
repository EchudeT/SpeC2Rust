# spec.md

## Title

Rust Port Functional Specification for `module_doc_main_01`

## Metadata

- Project: `cflow-new`
- Module: `module_doc_main_01`
- Category: `module_cluster`
- Target branch: `001-module_doc_main_01-rust-port`
- Generation date: `2026-06-17`

## Overview

This module cluster consists of four small command-line programs, each centered on a `main` entry point:

- `doc/d.c`
- `doc/foo.c`
- `doc/wc.c`
- `doc/whoami.c`

The Rust rewrite must preserve the user-visible behavior of these standalone executables as documented by the analyzed source set. The cluster provides simple command-line utilities for:

- directory-oriented reporting based on filesystem metadata and directory entries,
- a minimal standalone executable with no evidenced external behavior beyond successful invocation,
- word/line/byte-style counting over command input,
- reporting the current user identity based on password database lookup.

The Rust version must remain limited to the evidenced functional scope of these programs.

## Feature Specification

### FS-1: Directory reporting utility

Traceability: `doc/d.c`, `main` at `doc/d.c:84-105`, use of `struct stat` and `struct dirent`

The module shall provide one executable that inspects filesystem directory content and reports information derived from:

- directory traversal results,
- file metadata available through file status information.

Observed source evidence shows dependence on directory entries and file status records, so the Rust port must implement behavior that:

- accepts command-line execution as a standalone program,
- reads directory contents,
- obtains metadata for encountered filesystem objects,
- emits textual output describing the result of that inspection.

The exact report formatting must remain compatible with the original program’s externally visible behavior when validated against the C implementation.

### FS-2: Minimal standalone executable

Traceability: `doc/foo.c`, `main` at `doc/foo.c:2-8`

The module shall provide one executable whose evidenced scope is only that of a minimal program with a `main` entry point.

Because the analysis provides no further functions, types, or interactions, the Rust rewrite must preserve only the demonstrated behavior that is externally observable from the original executable. No additional options, side effects, or output requirements shall be introduced unless directly required to match the C program.

### FS-3: Counting utility

Traceability: `doc/wc.c`, `main` at `doc/wc.c:126-140`

The module shall provide one executable that performs count-based reporting over input data in the style indicated by its source file.

The Rust port must implement standalone command-line behavior that:

- accepts invocation through `main(argc, argv)`,
- reads input to be counted,
- computes the counts expected by the original program,
- prints the resulting counts in textual form.

The Rust version must match the C program’s visible counting semantics for supported invocation patterns.

### FS-4: Current-user reporting utility

Traceability: `doc/whoami.c`, `main` at `doc/whoami.c:25-34`, use of `struct passwd`

The module shall provide one executable that reports the current effective user identity using password database information.

The Rust port must implement behavior that:

- runs as a standalone command-line program,
- resolves the current user identity through OS user-account information,
- prints the user name associated with the executing context.

The output must remain behaviorally compatible with the original C utility.

## User Scenarios & Testing

### Scenario 1: Run the directory reporting tool on a directory

Traceability: `doc/d.c`, `struct stat`, `struct dirent`

A user invokes the directory utility from a shell to inspect a target directory or the default working context used by the original tool.

Expected behavior:

- the program starts successfully,
- directory entries are read,
- file metadata is consulted,
- a textual report is produced for the discovered items.

Testing guidance:

- compare Rust and C outputs for the same directory contents,
- include files of different sizes and metadata states,
- verify behavior on empty directories and populated directories.

### Scenario 2: Invoke the minimal executable

Traceability: `doc/foo.c`

A user runs the minimal program with the invocation style accepted by the original executable.

Expected behavior:

- the program exits in a manner consistent with the C version,
- any stdout/stderr output matches the original behavior,
- no unsupported options or extra behavior are introduced.

Testing guidance:

- execute the Rust and C versions with no arguments,
- if the original accepts incidental extra arguments without effect, verify equivalent handling,
- compare exit status and visible output.

### Scenario 3: Count input content

Traceability: `doc/wc.c`

A user invokes the counting utility on representative text input.

Expected behavior:

- the program reads the same input sources as the C version for tested cases,
- it computes the same count values,
- it prints counts in the same order and basic textual structure as the original behavior.

Testing guidance:

- test empty input,
- test a single line,
- test multiple lines with spaces and words,
- compare Rust and C outputs exactly for supported cases.

### Scenario 4: Report the current user

Traceability: `doc/whoami.c`, `struct passwd`

A user runs the user-reporting tool in a normal login or shell session.

Expected behavior:

- the program resolves the current user from system account information,
- it prints the same user name as the C version under the same execution identity,
- it exits successfully in normal conditions.

Testing guidance:

- compare output of Rust and C executables under the same account,
- validate that the reported name corresponds to the effective runtime identity used by the original tool.

## Requirements

### Functional Requirements

#### FR-1: Standalone executable preservation

Traceability: all four `main` functions

The Rust port shall produce four standalone executable behaviors corresponding to the four analyzed C programs. Each executable shall preserve the original program’s command-line entry behavior and externally visible output/exit behavior for supported invocations.

#### FR-2: Filesystem directory inspection

Traceability: `doc/d.c`, `struct dirent`

The Rust rewrite of the directory utility shall enumerate directory entries from the filesystem in the same functional role as the C program.

#### FR-3: Filesystem metadata usage

Traceability: `doc/d.c`, `struct stat`

The Rust rewrite of the directory utility shall obtain and use file status metadata needed to produce the same user-visible report as the C program.

#### FR-4: Counting behavior

Traceability: `doc/wc.c`

The Rust rewrite of the counting utility shall read input and compute the same counts as the original executable for supported inputs.

#### FR-5: Count report emission

Traceability: `doc/wc.c`

The Rust rewrite of the counting utility shall print the computed count results in a textual format compatible with the original program’s visible behavior.

#### FR-6: User identity lookup

Traceability: `doc/whoami.c`, `struct passwd`

The Rust rewrite of the user-reporting utility shall resolve the executing user’s account identity using operating-system user information equivalent to the password database role shown in the C source.

#### FR-7: User name output

Traceability: `doc/whoami.c`

The Rust rewrite of the user-reporting utility shall print the resolved user name in the same functional manner as the C program.

#### FR-8: No unevidenced feature expansion

Traceability: `doc/foo.c` and absence of further analyzed interfaces

The Rust port shall not add new command-line features, output modes, persistence behavior, public APIs, or other extended capabilities not evidenced by the analyzed source set.

### Key Entities

#### KE-1: File status record

Traceability: `struct stat` in `doc/d.c`

Represents filesystem metadata for an inspected file or directory entry. The directory reporting executable uses this entity to derive information shown in its output.

Relationship:

- associated with directory entries discovered during traversal or inspection.

#### KE-2: Directory entry

Traceability: `struct dirent` in `doc/d.c`

Represents one discovered item in a directory listing. The directory reporting executable uses each entry as input to metadata lookup and report generation.

Relationship:

- each directory entry may map to one file status record used for output.

#### KE-3: Password database record

Traceability: `struct passwd` in `doc/whoami.c`

Represents operating-system account information for a user identity. The user-reporting executable uses this entity to obtain the name to print.

Relationship:

- resolved from the current execution identity and consumed to produce the reported user name.

## Success Criteria

### SC-1: Behavioral parity for directory reporting

Traceability: `doc/d.c`

For a defined set of test directories, the Rust directory-reporting executable shall complete successfully and produce output matching the C program’s observable behavior for the same inputs and environment.

### SC-2: Behavioral parity for minimal executable

Traceability: `doc/foo.c`

When invoked in the same tested manner as the C version, the Rust minimal executable shall match the original program’s visible output and exit status.

### SC-3: Counting parity

Traceability: `doc/wc.c`

For representative empty, single-line, and multi-line inputs, the Rust counting executable shall produce the same count values and output layout as the C program.

### SC-4: Current-user reporting parity

Traceability: `doc/whoami.c`

When run under the same user account, the Rust user-reporting executable shall print the same user name as the C implementation.

### SC-5: Scope control

Traceability: entire analyzed module cluster

The Rust port shall remain within the evidenced functional scope of the four analyzed programs and shall not expose additional user-visible capabilities beyond those required for compatibility with the original C behavior.