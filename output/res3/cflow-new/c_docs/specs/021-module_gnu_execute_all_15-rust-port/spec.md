# spec.md

## Title

Rust Functional Specification for `module_gnu_execute_all_15`

## Metadata

- **Project**: `cflow-new`
- **Module**: `module_gnu_execute_all_15`
- **Category**: `module_cluster`
- **Source basis**: `gnu/fd-hook.c`
- **Rust branch**: `021-module_gnu_execute_all_15-rust-port`
- **Generation date**: `2026-06-17`

## Overview

This module provides grouped execution of file-descriptor-related hook functions. It is responsible for invoking a supplied primary operation together with any registered hooks of the same operation class, for at least two operation families evidenced by the source: close handling and ioctl handling.

The Rust rewrite must preserve the module’s functional role as a dispatcher over hook lists for these file-descriptor operations, including support for:
- executing all close hooks for a file descriptor, and
- executing all ioctl hooks for a file descriptor, request, and argument pointer.

The specification is limited to functionality directly evidenced by:
- `execute_all_close_hooks`
- `execute_all_ioctl_hooks`
- the module’s `struct fd_hook` data structure instances in `gnu/fd-hook.c`

## Feature Specification

### Feature: Execute all close-related hooks

The module must provide behavior equivalent to invoking the complete close-hook chain for a target file descriptor.

This behavior includes:
- accepting a primary close operation and a file descriptor,
- participating in execution of all close-related hook entries managed by the module, and
- returning an integer result for the overall close-hook execution.

The Rust version must preserve the module’s role as the single place that coordinates this multi-hook close execution.

### Feature: Execute all ioctl-related hooks

The module must provide behavior equivalent to invoking the complete ioctl-hook chain for a target file descriptor operation.

This behavior includes:
- accepting a primary ioctl operation,
- accepting the target file descriptor,
- accepting the ioctl request code, and
- accepting the associated argument pointer/reference equivalent,
- participating in execution of all ioctl-related hook entries managed by the module, and
- returning an integer result for the overall ioctl-hook execution.

The Rust version must preserve this grouped dispatch behavior for ioctl hooks.

### Feature: Maintain operation-specific hook groupings

The source shows repeated use of `struct fd_hook` in relation to both close and ioctl execution paths. The Rust version must therefore represent operation-specific hook groupings so that:
- close hook execution uses close-associated hook entries, and
- ioctl hook execution uses ioctl-associated hook entries.

No broader hook categories are required by this specification beyond those evidenced.

## User Scenarios & Testing

### Scenario 1: Close path dispatch

A caller needs to perform a close-related operation on a file descriptor while ensuring all close hooks associated with this module are executed as part of the operation flow.

**Expected support in Rust:**
- The caller supplies a primary close function and a file descriptor.
- The module executes the module-managed close hook chain for that descriptor.
- The module returns an integer result.

**Testing focus:**
- Verify the close execution entry point accepts a primary function and descriptor.
- Verify all registered/module-held close hooks are included in the execution flow.
- Verify an integer result is produced.

### Scenario 2: Ioctl path dispatch

A caller needs to perform an ioctl-related operation on a file descriptor while ensuring all ioctl hooks associated with this module are executed as part of the operation flow.

**Expected support in Rust:**
- The caller supplies a primary ioctl function, a file descriptor, a request value, and an argument value equivalent to the C pointer parameter.
- The module executes the module-managed ioctl hook chain for those inputs.
- The module returns an integer result.

**Testing focus:**
- Verify the ioctl execution entry point accepts all four inputs.
- Verify all registered/module-held ioctl hooks are included in the execution flow.
- Verify an integer result is produced.

### Scenario 3: Distinct hook classes remain separated

A caller using close dispatch must not trigger ioctl hook execution, and a caller using ioctl dispatch must not trigger close hook execution.

**Expected support in Rust:**
- Operation family selection is determined by the invoked execution entry point.
- Hook traversal is scoped to the relevant hook grouping.

**Testing focus:**
- Verify close execution uses only close hook entries.
- Verify ioctl execution uses only ioctl hook entries.

## Requirements

### Functional Requirements

#### FR-1: Close hook chain execution
The module shall provide an operation equivalent to `execute_all_close_hooks` that performs grouped execution for close-related hooks using:
- a supplied primary close function, and
- a supplied file descriptor.

**Traceability**: `gnu/fd-hook.c`, `execute_all_close_hooks`

#### FR-2: Ioctl hook chain execution
The module shall provide an operation equivalent to `execute_all_ioctl_hooks` that performs grouped execution for ioctl-related hooks using:
- a supplied primary ioctl function,
- a supplied file descriptor,
- a supplied request value, and
- a supplied argument value corresponding to the C `void *arg` parameter.

**Traceability**: `gnu/fd-hook.c`, `execute_all_ioctl_hooks`

#### FR-3: Integer result reporting
Each grouped execution operation shall return an integer result corresponding to the overall operation result.

**Traceability**: `gnu/fd-hook.c`, `execute_all_close_hooks`, `execute_all_ioctl_hooks`

#### FR-4: Operation-specific hook grouping
The module shall maintain hook data in a form that supports at least separate execution groupings for:
- close-related hooks, and
- ioctl-related hooks.

**Traceability**: `gnu/fd-hook.c`, `struct fd_hook`, `execute_all_close_hooks`, `execute_all_ioctl_hooks`

#### FR-5: Hook execution coordinated by this module
The Rust rewrite shall preserve this module as the coordinator of executing the full relevant hook set for each supported operation family, rather than exposing only the primary function call.

**Traceability**: `gnu/fd-hook.c`, `execute_all_close_hooks`, `execute_all_ioctl_hooks`

### Key Entities

#### Entity: FD Hook
A hook record represented in the C source by `struct fd_hook`.

**Required role in Rust:**
- represent an entry in a hook set associated with file-descriptor-related operations,
- support association with a specific operation family such as close or ioctl,
- participate in grouped execution through the module’s dispatch operations.

**Traceability**: `gnu/fd-hook.c`, `struct fd_hook`

#### Entity: Primary Operation Function
A caller-supplied function for the operation being executed:
- close primary function for close dispatch,
- ioctl primary function for ioctl dispatch.

**Required role in Rust:**
- be accepted as input to the corresponding grouped execution operation,
- be part of the operation flow coordinated by the module.

**Traceability**: `gnu/fd-hook.c`, `execute_all_close_hooks`, `execute_all_ioctl_hooks`

#### Entity: Hook Grouping by Operation Type
A logical separation of hook entries according to operation family.

**Required role in Rust:**
- enable close dispatch to target only close hooks,
- enable ioctl dispatch to target only ioctl hooks.

**Traceability**: `gnu/fd-hook.c`, `struct fd_hook`, `execute_all_close_hooks`, `execute_all_ioctl_hooks`

## Success Criteria

### SC-1: Close dispatch parity
A Rust implementation exposes functionality equivalent to the C module’s close grouped execution, accepting a primary close operation and file descriptor and returning an integer result.

**Traceability**: `execute_all_close_hooks`

### SC-2: Ioctl dispatch parity
A Rust implementation exposes functionality equivalent to the C module’s ioctl grouped execution, accepting a primary ioctl operation, file descriptor, request value, and argument value, and returning an integer result.

**Traceability**: `execute_all_ioctl_hooks`

### SC-3: Operation-family separation
Tests demonstrate that invoking close grouped execution uses the close hook grouping only, and invoking ioctl grouped execution uses the ioctl hook grouping only.

**Traceability**: `struct fd_hook`, `execute_all_close_hooks`, `execute_all_ioctl_hooks`

### SC-4: Full hook-set coordination
Tests demonstrate that each execution path coordinates the complete relevant hook set managed by the module, not just the provided primary function alone.

**Traceability**: `gnu/fd-hook.c`, `execute_all_close_hooks`, `execute_all_ioctl_hooks`

### SC-5: No unsupported feature expansion
The Rust rewrite introduces no required public functionality beyond the evidenced grouped execution of close and ioctl hooks and the supporting hook data representation.

**Traceability**: `gnu/fd-hook.c`