# spec.md

## Overview

- **Project**: `cat`
- **Module**: `main_root_stat_03`
- **Category**: `main_cluster`
- **Rust branch**: `004-main_root_stat_03-rust-port`
- **Source basis**: `cat.c`, `fcntl.c`
- **Primary entry points**: `main`, `klibc_fcntl`

This module defines the process entry behavior for the program and the local file-control wrapper behavior used by that entry path. The Rust rewrite must preserve the observable command-line behavior governed by this module: argument-driven startup, option handling through the local option table, file status inspection through `stat`, and delegated file-control operations through the local `fcntl` wrapper.

The specification is limited to behavior evidenced by the analyzed files and symbols.

## Feature Specification

### Summary

The Rust version must implement the module behavior that:

1. starts the program from the command line,
2. interprets command-line options using an internal option description table,
3. performs file-status-based decisions using `stat` data,
4. invokes file-control operations through a module-local variadic-style wrapper equivalent,
5. returns process status through the program entry point.

### In-Scope Functionality

#### 1. Program entry and command-line processing

The module must provide the top-level program entry behavior corresponding to `main(int argc, char **argv)`.

The Rust rewrite must:
- accept command-line arguments,
- evaluate supported options using a module-local option definition table,
- distinguish options from file operands,
- drive the module's startup and execution flow from those parsed inputs,
- terminate with an integer process status.

This requirement is directly traceable to `main` and the presence of a local `struct option` instance in `cat.c`.

#### 2. File status inspection during startup/runtime decisions

The module must inspect filesystem object metadata through `stat` structures where needed by the main execution path.

The Rust rewrite must preserve behavior in which:
- file metadata can be obtained for paths or descriptors relevant to startup/execution decisions,
- those metadata influence the control flow of the main module,
- failures in obtaining required status information affect program outcome in the same functional manner as the C module.

This is traceable to the `struct stat` usage in `cat.c` and `fcntl.c`.

#### 3. Local file-control operation dispatch

The module must include a local file-control helper corresponding to `klibc_fcntl(fd, action, ...)`.

The Rust rewrite must preserve the functional role of this helper:
- accept a file descriptor and an action selector,
- accept the action-associated extra argument when required by that action,
- perform the underlying file-control request,
- return an integer result indicating success or failure as expected by callers.

This is directly traceable to `klibc_fcntl` in `fcntl.c`.

#### 4. Integration of command-line logic and low-level file handling

The entry path must be able to combine parsed command-line state with file metadata checks and file-control actions to execute the module’s runtime behavior.

The Rust rewrite must therefore preserve:
- sequencing from argument parsing to file-oriented processing,
- use of low-level file state where the original module depends on it,
- consistent propagation of operation failure into user-visible exit status.

This is jointly traceable to `main`, `klibc_fcntl`, and the referenced `stat` and `option` structures.

## User Scenarios & Testing

### Scenario 1: Launch with no additional operands

**User scenario**
A user runs the program without file operands. The module starts successfully, processes the default command-line state, and exits with a status determined by the main entry path.

**What must be supported**
- program startup with only process arguments required by the OS,
- parsing of an empty operand list,
- completion through `main` with a valid exit code.

**Testing focus**
- invoke the Rust binary with no extra arguments,
- verify that argument parsing completes,
- verify that the process exits normally with a deterministic integer status for this case.

### Scenario 2: Launch with one or more options

**User scenario**
A user supplies command-line options supported by the module’s option table. The module recognizes those options and updates execution behavior accordingly.

**What must be supported**
- option recognition using the module-defined option descriptors,
- differentiation between options and non-option operands,
- rejection or failure signaling for unsupported/invalid option forms, if the original path does so.

**Testing focus**
- run with representative valid options evidenced by the module’s option table in `cat.c`,
- verify that options alter control flow as expected,
- run with invalid option input and verify non-success outcome if the original module produces one.

### Scenario 3: Launch with file operands requiring status inspection

**User scenario**
A user provides one or more file operands. The module inspects file metadata as needed and uses that information to proceed or fail.

**What must be supported**
- obtaining `stat` information for relevant file inputs,
- handling successful metadata reads,
- handling failures when status data cannot be obtained.

**Testing focus**
- run with an accessible regular file operand,
- verify that required status inspection succeeds and processing continues,
- run with a nonexistent or inaccessible operand,
- verify that the process reports failure through exit status.

### Scenario 4: File-control action is needed on an open descriptor

**User scenario**
During execution, the module needs to perform a file-control operation associated with a descriptor and action code.

**What must be supported**
- dispatch through the module-local file-control wrapper,
- support for actions with and without an extra argument,
- propagation of the returned result to the calling flow.

**Testing focus**
- exercise a code path that causes the Rust equivalent of `klibc_fcntl` to be called,
- verify correct success behavior on a valid descriptor/action combination,
- verify failure signaling on an invalid descriptor or unsupported action input matching original behavior.

### Scenario 5: Error propagation to process exit status

**User scenario**
A runtime failure occurs during option processing, metadata lookup, or file-control handling. The module converts that failure into the appropriate process termination status.

**What must be supported**
- centralized outcome determination in `main`,
- non-success status on failed required operations,
- success status only when the main path completes successfully.

**Testing focus**
- trigger at least one parsing-related failure case,
- trigger at least one file-status-related failure case,
- verify that exit codes differ between success and failure cases.

## Requirements

### Functional Requirements

#### FR-1: Top-level execution entry
The module shall provide the program’s top-level execution entry corresponding to `main`, accepting command-line inputs and producing an integer exit status.

**Traceability**: `cat.c` → `main`

#### FR-2: Command-line option interpretation
The module shall interpret command-line options using a defined option description set represented by the module’s option structure usage.

**Traceability**: `cat.c` → `main`; `cat.c` → local `struct option`

#### FR-3: Operand handling
The module shall distinguish command-line options from non-option operands and include those operands in the main execution flow.

**Traceability**: `cat.c` → `main`

#### FR-4: File metadata retrieval
The module shall obtain file status information through `stat`-equivalent data where required by the main execution path.

**Traceability**: `cat.c` → `main`; `cat.c` → local `struct stat`; `fcntl.c` → local `struct stat`

#### FR-5: Metadata-driven control flow
The module shall use retrieved file status information to influence runtime decisions in the same functional locations as the C module.

**Traceability**: `cat.c` → `main`; `stat` usage in analyzed types

#### FR-6: Local file-control wrapper
The module shall provide a local helper equivalent in role to `klibc_fcntl` that performs file-control requests on a descriptor using an action code and, when applicable, an associated extra argument.

**Traceability**: `fcntl.c` → `klibc_fcntl`

#### FR-7: File-control result reporting
The module shall return integer success/failure results from the file-control helper in a way usable by the calling path.

**Traceability**: `fcntl.c` → `klibc_fcntl`

#### FR-8: Failure propagation
The module shall propagate failures from command-line handling, metadata retrieval, and file-control operations to the final process exit status.

**Traceability**: `cat.c` → `main`; `fcntl.c` → `klibc_fcntl`

### Key Entities

#### 1. Program entry state
The effective input state provided to `main`: argument count and argument vector.

**Role**
- drives the module’s top-level behavior,
- provides raw option and operand input.

**Traceability**: `cat.c` → `main`

#### 2. Option descriptor
The module-local `struct option` usage in `cat.c`.

**Role**
- defines recognized command-line options,
- supports mapping user-provided option tokens into execution behavior.

**Relationship**
- consumed by the command-line processing logic in `main`.

**Traceability**: `cat.c` → local `struct option`; `cat.c` → `main`

#### 3. File status record
The `struct stat` usage referenced in both analyzed files.

**Role**
- carries filesystem metadata used for runtime decisions,
- supports status checks associated with file-oriented processing and low-level descriptor-related behavior.

**Relationship**
- populated during status lookup operations,
- read by the main control flow and potentially by file-control-adjacent logic.

**Traceability**: `cat.c` → local `struct stat`; `fcntl.c` → local `struct stat`

#### 4. File descriptor/action pair
The logical input to the local file-control helper: descriptor plus action code, optionally plus action-specific argument.

**Role**
- identifies the target open file and requested control operation.

**Relationship**
- passed into the `klibc_fcntl` equivalent,
- produces an integer result consumed by higher-level control flow.

**Traceability**: `fcntl.c` → `klibc_fcntl`

## Success Criteria

### Behavioral Parity Criteria

1. **Entry behavior parity**
   The Rust module exposes the program entry behavior for this module and returns an integer process status for success and failure paths.
   - **Measured by**: executable runs and exits with observable status codes.
   - **Traceability**: `cat.c` → `main`

2. **Option-processing parity**
   For command-line options represented by the original module’s option descriptors, the Rust version recognizes the same supported inputs and routes execution through corresponding behaviors.
   - **Measured by**: scenario tests using valid and invalid option invocations.
   - **Traceability**: `cat.c` → `main`; `cat.c` → local `struct option`

3. **Operand and status-check parity**
   When invoked with file operands, the Rust version performs required file status checks and distinguishes successful metadata retrieval from failure.
   - **Measured by**: tests using accessible and inaccessible/nonexistent file operands.
   - **Traceability**: `cat.c` → `main`; `struct stat` usage

4. **File-control wrapper parity**
   The Rust version provides a helper with the same functional role as `klibc_fcntl`, correctly handling descriptor/action requests and returning success/failure results.
   - **Measured by**: tests on valid and invalid descriptor/action combinations through the exercised code path.
   - **Traceability**: `fcntl.c` → `klibc_fcntl`

5. **Error-to-exit-status parity**
   Failures in required operations within this module result in non-success process termination status, while successful execution paths result in success status.
   - **Measured by**: end-to-end tests covering one success path and multiple failure paths.
   - **Traceability**: `cat.c` → `main`; `fcntl.c` → `klibc_fcntl`

### Completion Criteria

The Rust port of `main_root_stat_03` is complete when:
- all functional requirements in this document are implemented,
- all listed scenarios are testable and pass,
- no behavior outside the evidenced module scope is introduced as a required feature,
- the resulting module preserves the original module’s user-visible command-line and process-status behavior within the boundaries described above.