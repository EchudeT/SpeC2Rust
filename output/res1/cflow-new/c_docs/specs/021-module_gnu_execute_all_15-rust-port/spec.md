# spec.md

## Overview

- **Project**: `cflow-new`
- **Module**: `module_gnu_execute_all_15`
- **Category**: `module_cluster`
- **Source scope**: `gnu/fd-hook.c`
- **Rust target branch**: `021-module_gnu_execute_all_15-rust-port`
- **Generation date**: `2026-06-11`

## Feature Specification

This module provides coordinated execution of file-descriptor hook chains for two operation classes:

- close-related hooks
- ioctl-related hooks

The module’s functional role is to invoke all registered hooks relevant to a given operation on a file descriptor, while also invoking the primary operation supplied by the caller. The Rust rewrite must preserve this behavior for both supported operation classes.

The module is centered on hook collections represented by `struct fd_hook` entries in `gnu/fd-hook.c`. These entries define the callable units that participate in the execution sequence for a requested operation.

The Rust version must implement:

- execution of all close hooks for a supplied file descriptor, through `execute_all_close_hooks` behavior
- execution of all ioctl hooks for a supplied file descriptor, request code, and argument pointer, through `execute_all_ioctl_hooks` behavior
- preservation of the module’s role as a hook dispatcher over the module-defined hook entities

No additional operation classes, registration models, recovery behavior, or concurrency guarantees are part of this specification.

## User Scenarios & Testing

### Scenario 1: Execute all close-related hooks for a descriptor

A caller needs to perform a close operation on a file descriptor while ensuring that every close hook managed by this module is given a chance to run as part of that operation.

**Expected support in Rust:**
- The caller provides a primary close function and a file descriptor.
- The module executes the full close hook chain associated with this operation class.
- The operation yields an integer result consistent with the module’s close-hook execution behavior.

**Test focus:**
- A supplied primary close callable is accepted and invoked through the module’s close execution path.
- Multiple configured close hooks are all executed as part of one call.
- The returned integer result is the module’s defined result for the close execution path.

### Scenario 2: Execute all ioctl-related hooks for a descriptor

A caller needs to perform an ioctl-style operation on a file descriptor while ensuring that every ioctl hook managed by this module participates.

**Expected support in Rust:**
- The caller provides a primary ioctl function, a file descriptor, a request value, and an operation argument.
- The module executes the full ioctl hook chain associated with this operation class.
- The operation yields an integer result consistent with the module’s ioctl-hook execution behavior.

**Test focus:**
- A supplied primary ioctl callable is accepted and invoked through the module’s ioctl execution path.
- Multiple configured ioctl hooks are all executed as part of one call.
- The file descriptor, request, and argument are forwarded through the ioctl execution path.
- The returned integer result is the module’s defined result for the ioctl execution path.

### Scenario 3: Distinguish hook execution by operation class

A caller uses both close and ioctl operation paths and expects the module to dispatch only the hooks relevant to each path.

**Expected support in Rust:**
- Close execution uses close hook entities only.
- Ioctl execution uses ioctl hook entities only.

**Test focus:**
- Invoking the close path does not trigger ioctl hooks.
- Invoking the ioctl path does not trigger close hooks.

## Requirements

### Functional Requirements

#### FR-1: Close hook-chain execution
The module shall provide behavior equivalent to `execute_all_close_hooks(gl_close_fn primary, int fd)` from `gnu/fd-hook.c`, executing the module’s close hook chain for the supplied file descriptor together with the supplied primary close operation.

**Traceability**: `gnu/fd-hook.c`, `execute_all_close_hooks`

#### FR-2: Ioctl hook-chain execution
The module shall provide behavior equivalent to `execute_all_ioctl_hooks(gl_ioctl_fn primary, int fd, int request, void *arg)` from `gnu/fd-hook.c`, executing the module’s ioctl hook chain for the supplied file descriptor, request, and argument together with the supplied primary ioctl operation.

**Traceability**: `gnu/fd-hook.c`, `execute_all_ioctl_hooks`

#### FR-3: Integer operation result
Each supported execution path shall return an integer result, matching the observable contract of the corresponding module function.

**Traceability**: `gnu/fd-hook.c`, `execute_all_close_hooks`, `execute_all_ioctl_hooks`

#### FR-4: Operation-specific dispatch
The module shall keep close-hook execution and ioctl-hook execution behavior distinct, so that each function executes only the hooks belonging to its operation class.

**Traceability**: `gnu/fd-hook.c`, `execute_all_close_hooks`, `execute_all_ioctl_hooks`, `struct fd_hook`

### Key Entities

#### Hook entry (`struct fd_hook`)
The module defines hook-entry structures identified in the source as `struct fd_hook`. These structures represent the hook items that belong to the module’s managed execution sets.

**Role:**
- represent executable hook participants
- belong to an operation-specific hook collection
- are consumed by the module’s execution functions

**Relationships:**
- `execute_all_close_hooks` operates over the hook entries associated with close handling
- `execute_all_ioctl_hooks` operates over the hook entries associated with ioctl handling

#### Primary operation callable
Each execution function accepts a caller-supplied primary callable for the requested operation class.

**Role:**
- serves as the primary operation being wrapped or coordinated by hook execution
- is specific to the operation class:
  - close primary for close execution
  - ioctl primary for ioctl execution

**Relationships:**
- used by the corresponding execution function together with the relevant hook entries

#### Operation inputs
The execution functions accept operation inputs that identify the target descriptor operation.

**Role:**
- `fd` identifies the file descriptor for both execution paths
- `request` and `arg` further parameterize ioctl execution

**Relationships:**
- passed into the corresponding operation-specific execution path

## Success Criteria

### SC-1: Close path availability
A Rust implementation exposes module behavior equivalent to the source module’s close execution function and accepts:
- a primary close callable
- a file descriptor

**Measured by:**
- integration tests invoking the close execution path successfully

**Traceability**: `gnu/fd-hook.c`, `execute_all_close_hooks`

### SC-2: Ioctl path availability
A Rust implementation exposes module behavior equivalent to the source module’s ioctl execution function and accepts:
- a primary ioctl callable
- a file descriptor
- a request value
- an argument value

**Measured by:**
- integration tests invoking the ioctl execution path successfully

**Traceability**: `gnu/fd-hook.c`, `execute_all_ioctl_hooks`

### SC-3: Full hook-chain participation per operation class
For each supported operation class, all configured hooks in that class are executed during a single module call.

**Measured by:**
- tests with multiple hook entries confirming that every expected hook is observed during:
  - close execution
  - ioctl execution

**Traceability**: `gnu/fd-hook.c`, `execute_all_close_hooks`, `execute_all_ioctl_hooks`, `struct fd_hook`

### SC-4: Correct operation-class separation
Close execution and ioctl execution remain separated by hook type.

**Measured by:**
- tests confirming that close execution triggers only close hooks
- tests confirming that ioctl execution triggers only ioctl hooks

**Traceability**: `gnu/fd-hook.c`, `execute_all_close_hooks`, `execute_all_ioctl_hooks`, `struct fd_hook`

### SC-5: Integer result preservation
Both execution paths return integer results consistent with the source module’s observable contract.

**Measured by:**
- tests asserting integer return values are produced for both operation paths

**Traceability**: `gnu/fd-hook.c`, `execute_all_close_hooks`, `execute_all_ioctl_hooks`