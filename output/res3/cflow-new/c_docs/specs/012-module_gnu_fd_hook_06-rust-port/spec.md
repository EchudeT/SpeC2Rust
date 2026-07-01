# spec.md

## Title

Functional Specification for `module_gnu_fd_hook_06` Rust Port

## Document Control

- Project: `cflow-new`
- Module: `module_gnu_fd_hook_06`
- Category: `module_cluster`
- Source file: `gnu/fd-hook.c`
- Rust branch: `012-module_gnu_fd_hook_06-rust-port`
- Generation date: `2026-06-17`

## Overview

This module defines a small hook registry for file-descriptor-related operations. It supports:

- registering a hook node into a module-managed list,
- unregistering a previously registered hook node,
- executing registered close hooks before invoking a supplied primary close operation,
- executing registered ioctl hooks before invoking a supplied primary ioctl operation.

The Rust rewrite must preserve the observable behavior of this hook orchestration layer: hook registration order, hook removal behavior, and the sequence in which hook callbacks and the primary operation are invoked.

## Feature Specification

### Summary

The module provides a linked-list-based registry of file descriptor hooks. Each registered entry may supply:

- a close-related hook callback,
- an ioctl-related hook callback,
- linkage used to participate in the registry.

The module also provides execution helpers that traverse a hook list and apply all relevant hooks for an operation, then invoke a caller-supplied primary operation on the same file descriptor context.

### Functional Scope

The Rust version must implement the following module behavior evidenced by `gnu/fd-hook.c`:

1. Maintain a module-level collection of registered hook entries.
2. Allow a caller to register a hook entry with close and ioctl hook callbacks.
3. Allow a caller to unregister a previously registered hook entry.
4. For close processing:
   - traverse the remaining registered hooks,
   - invoke close hooks associated with those entries,
   - invoke the supplied primary close function,
   - return an integer result representing the overall operation result.
5. For ioctl processing:
   - invoke ioctl hooks associated with those entries,
   - invoke the supplied primary ioctl function with file descriptor, request, and argument,

### Out of Scope

The Rust version must not claim or introduce capabilities not evidenced by this module input, including:

- new public hook types or hook categories,
- persistence or serialization of hook state,
- thread-safety guarantees,
- error recovery policies beyond the source behavior,
- expanded lifecycle management beyond register, unregister, and execute.

## User Scenarios & Testing

### Scenario 1: Register a hook and participate in close handling

A consumer prepares a hook entry and registers it with a close hook callback. Later, when close processing is executed for a file descriptor, the registered close hook is included in the execution sequence before the supplied primary close function.

#### Expected test coverage

- Register one hook entry with a close callback.
- Execute close processing with a test primary close function.
- Verify the close hook is invoked during execution.
- Verify the primary close function is also invoked.
- Verify the returned integer result matches source behavior.

### Scenario 2: Register a hook and participate in ioctl handling

A consumer prepares a hook entry and registers it with an ioctl hook callback. Later, when ioctl processing is executed, the registered ioctl hook is included in the execution sequence before the supplied primary ioctl function, using the same file descriptor, request, and argument.

#### Expected test coverage

- Register one hook entry with an ioctl callback.
- Execute ioctl processing with a test primary ioctl function.
- Verify the ioctl hook is invoked with the operation context.
- Verify the primary ioctl function is invoked with the same context.
- Verify the returned integer result matches source behavior.

### Scenario 3: Multiple hooks are active

A consumer registers multiple hook entries. When an operation is executed, all remaining registered hooks of the relevant kind participate in the traversal and the primary operation is executed afterward.

#### Expected test coverage

- Register multiple entries with relevant callbacks.
- Execute close processing and separately ioctl processing.
- Verify all registered relevant hooks are visited.
- Verify visitation order matches source behavior.
- Verify the primary function is called after hook traversal.

### Scenario 4: Unregister a hook before execution

A consumer unregisters a previously registered hook entry. Subsequent operation execution must no longer include that entry in traversal.

#### Expected test coverage

- Register two or more hook entries.
- Unregister one registered entry.
- Execute close or ioctl processing.
- Verify the unregistered entry is not invoked.
- Verify remaining registered entries still participate.

### Scenario 5: Mixed hook capabilities per entry

A consumer registers an entry that supplies one callback type and not the other. Execution of each operation type must only consider the relevant callback behavior.

#### Expected test coverage

- Register an entry with only a close hook.
- Verify close processing includes that entry.
- Verify ioctl processing does not attempt ioctl callback behavior for that entry.
- Repeat symmetrically for an entry with only an ioctl hook.

## Requirements

### Functional Requirements

- **FR-1**: The module shall provide a hook registration operation that adds a caller-provided hook entry to the module’s managed registry using the callbacks supplied at registration.
  **Traceability:** `register_fd_hook` in `gnu/fd-hook.c:72-97`

- **FR-2**: The module shall provide a hook unregistration operation that removes a caller-provided hook entry from the module’s managed registry.
  **Traceability:** `unregister_fd_hook` in `gnu/fd-hook.c:99-116`

- **FR-3**: The module shall provide close-operation execution behavior that processes the remaining hook entries for close handling and then invokes a supplied primary close function for the same file descriptor.
  **Traceability:** `execute_close_hooks` in `gnu/fd-hook.c:35-45`

- **FR-4**: The module shall provide ioctl-operation execution behavior that processes the remaining hook entries for ioctl handling and then invokes a supplied primary ioctl function for the same file descriptor, request, and argument.
  **Traceability:** `execute_ioctl_hooks` in `gnu/fd-hook.c:53-63`

- **FR-5**: The module shall preserve the source module’s observable traversal behavior for registered hook entries during close execution, including which registered entries are considered “remaining” and the order in which they are processed.
  **Traceability:** `execute_close_hooks` in `gnu/fd-hook.c:35-45`, `struct fd_hook` usage

- **FR-6**: The module shall preserve the source module’s observable traversal behavior for registered hook entries during ioctl execution, including which registered entries are considered “remaining” and the order in which they are processed.
  **Traceability:** `execute_ioctl_hooks` in `gnu/fd-hook.c:53-63`, `struct fd_hook` usage

- **FR-7**: The module shall return integer results from close and ioctl execution consistent with the source module behavior.
  **Traceability:** `execute_close_hooks` in `gnu/fd-hook.c:35-45`; `execute_ioctl_hooks` in `gnu/fd-hook.c:53-63`

### Key Entities

- **Hook entry (`fd_hook`)**
  A registry node representing one participant in the hook chain. It carries callback associations for close and ioctl behavior and contains linkage needed to be inserted into and removed from the module-managed list.
  **Traceability:** `struct fd_hook` references throughout `gnu/fd-hook.c`

- **Close hook callback**
  A callback associated with a hook entry and used during close execution traversal.
  **Traceability:** `register_fd_hook`, `execute_close_hooks`

- **Ioctl hook callback**
  A callback associated with a hook entry and used during ioctl execution traversal.
  **Traceability:** `register_fd_hook`, `execute_ioctl_hooks`

- **Primary close function**
  A caller-supplied function invoked by close execution after hook processing.
  **Traceability:** `execute_close_hooks`

- **Primary ioctl function**
  A caller-supplied function invoked by ioctl execution after hook processing.
  **Traceability:** `execute_ioctl_hooks`

- **Managed hook registry**
  The module-level collection of currently registered `fd_hook` entries traversed by execution functions and modified by register/unregister operations.
  **Traceability:** `register_fd_hook`, `unregister_fd_hook`, execution functions’ `remaining_list` traversal context

## Success Criteria

- **SC-1**: A Rust test demonstrates that a registered close hook is invoked during close execution and that the supplied primary close function is invoked afterward, matching source-visible sequencing.
  **Traceability:** `execute_close_hooks`, `register_fd_hook`

- **SC-2**: A Rust test demonstrates that a registered ioctl hook is invoked during ioctl execution and that the supplied primary ioctl function is invoked afterward with the same `fd`, `request`, and `arg` values.
  **Traceability:** `execute_ioctl_hooks`, `register_fd_hook`

- **SC-3**: A Rust test demonstrates that unregistering a hook entry removes it from subsequent execution traversal without preventing other registered entries from being processed.
  **Traceability:** `unregister_fd_hook`, execution functions

- **SC-4**: A Rust test with multiple registered entries demonstrates that traversal order during close and ioctl execution matches the source module behavior.
  **Traceability:** `execute_close_hooks`, `execute_ioctl_hooks`, `register_fd_hook`

- **SC-5**: A Rust test demonstrates that entries lacking the relevant callback for an operation do not participate in that operation beyond source-defined traversal behavior.
  **Traceability:** `register_fd_hook`, both execution functions

- **SC-6**: A Rust test demonstrates that integer return values from close and ioctl execution match the source behavior for representative success and failure cases of the supplied primary functions and hooks.
  **Traceability:** `execute_close_hooks`, `execute_ioctl_hooks`