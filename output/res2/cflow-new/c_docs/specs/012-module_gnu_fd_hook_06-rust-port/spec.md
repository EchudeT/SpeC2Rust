# spec.md

## Title

Functional Specification: `module_gnu_fd_hook_06` Rust Port

## Metadata

- Project: `cflow-new`
- Module: `module_gnu_fd_hook_06`
- Category: `module_cluster`
- Source basis: `gnu/fd-hook.c`
- Rust branch: `012-module_gnu_fd_hook_06-rust-port`
- Generation date: `2026-06-17`

## Overview

This module provides a small file-descriptor hook registry and dispatcher.

Its purpose is to let participating code register hook callbacks associated with file-descriptor operations, then invoke those hooks in sequence when a close-related or ioctl-related action is being processed. The module also supports removing a previously registered hook entry.

The Rust rewrite must preserve the module’s observable behavior:

- maintain a registry of hook entries,
- allow registration and unregistration of an entry represented by a link object,
- execute registered close hooks together with a supplied primary close operation,
- execute registered ioctl hooks together with a supplied primary ioctl operation,
- preserve the module’s hook-chain behavior as evidenced by the source functions.

## Scope

In scope:

- registration of a hook entry containing close and ioctl hook callbacks,
- removal of a previously registered hook entry,
- sequential dispatch of close hooks,
- sequential dispatch of ioctl hooks,
- propagation of the integer result returned by hook execution.

Out of scope:

- any new hook kinds,
- any public API beyond the behavior evidenced by the source module,
- thread-safety guarantees,
- persistence, serialization, or recovery,
- policy decisions about file descriptors beyond invoking registered hooks.

## Feature Specification

### Feature 1: Hook Registration

The module must support adding a hook entry to the module-managed hook chain.

A registration operation accepts:

- a close hook callback,
- an ioctl hook callback,
- a link object representing the hook entry.

The registration behavior must make the provided link participate in subsequent hook execution. After registration, the link is considered part of the active hook sequence until unregistered.

Traceability:

- `register_fd_hook` in `gnu/fd-hook.c:72-97`
- `fd_hook` link structure usage in the same function

### Feature 2: Hook Unregistration

The module must support removing a previously registered hook entry from the active hook chain.

Unregistering a link must stop that link from participating in future close-hook and ioctl-hook dispatch.

Traceability:

- `unregister_fd_hook` in `gnu/fd-hook.c:99-116`
- `fd_hook` link structure traversal/manipulation in that function

### Feature 3: Close Hook Dispatch

The module must support executing a close-related hook chain for a given file descriptor.

The dispatch operation accepts:

- a remaining hook list position,
- a primary close function,
- a file descriptor.

The module must execute the close hook chain starting from the provided list position and produce an integer result from that execution path. The Rust version must preserve the same externally visible dispatch semantics as the source module, including use of the primary operation as part of the chain.

Traceability:

- `execute_close_hooks` in `gnu/fd-hook.c:35-45`
- `fd_hook` usage in that function

### Feature 4: Ioctl Hook Dispatch

The module must support executing an ioctl-related hook chain for a given file descriptor and request.

The dispatch operation accepts:

- a remaining hook list position,
- a primary ioctl function,
- a file descriptor,
- an ioctl request value,
- an argument pointer/value.

The module must execute the ioctl hook chain starting from the provided list position and produce an integer result from that execution path. The Rust version must preserve the same externally visible dispatch semantics as the source module, including use of the primary operation as part of the chain.

Traceability:

- `execute_ioctl_hooks` in `gnu/fd-hook.c:53-63`
- `fd_hook` usage in that function

## User Scenarios & Testing

### Scenario 1: Register one hook and dispatch close

A consumer creates one hook entry, registers it, and then initiates close-hook execution for a file descriptor.

Expected support in Rust:

- the registered entry participates in dispatch,
- dispatch returns an integer result,
- the dispatch path includes the supplied primary close operation according to chain semantics.

Suggested tests:

- register a single link and verify that close dispatch reaches that hook,
- verify the returned integer result matches the hook-chain outcome.

Traceability:

- `register_fd_hook`
- `execute_close_hooks`

### Scenario 2: Register one hook and dispatch ioctl

A consumer creates one hook entry, registers it, and then initiates ioctl-hook execution for a file descriptor, request, and argument.

Expected support in Rust:

- the registered entry participates in ioctl dispatch,
- the fd, request, and argument are passed through the hook chain,
- dispatch returns an integer result.

Suggested tests:

- register a single link and verify that ioctl dispatch reaches that hook with the provided inputs,
- verify the returned integer result matches the hook-chain outcome.

Traceability:

- `register_fd_hook`
- `execute_ioctl_hooks`

### Scenario 3: Multiple registered hooks form an ordered chain

A consumer registers multiple hook entries and dispatches a close or ioctl operation.

Expected support in Rust:

- dispatch begins from a specific remaining-list position,
- the chain can continue across multiple registered entries,
- each dispatch produces one integer result for the overall chain.

Suggested tests:

- register multiple links and verify that dispatch can traverse more than one hook entry,
- verify that continuing from a provided list position behaves consistently for both close and ioctl cases.

Traceability:

- `execute_close_hooks`
- `execute_ioctl_hooks`
- `register_fd_hook`
- `fd_hook`

### Scenario 4: Unregister removes a hook from future dispatch

A consumer registers a hook entry, later unregisters that same entry, and then dispatches a close or ioctl operation.

Expected support in Rust:

- the unregistered link no longer participates in dispatch,
- remaining registered links, if any, still participate.

Suggested tests:

- register then unregister one link and verify it is not reached by later dispatch,
- register multiple links, remove one, and verify the others remain active.

Traceability:

- `register_fd_hook`
- `unregister_fd_hook`
- `execute_close_hooks`
- `execute_ioctl_hooks`

### Scenario 5: Dispatch starting at end of chain

A consumer invokes dispatch with a list position that represents no remaining hooks.

Expected support in Rust:

- dispatch still produces an integer result via the primary operation path,
- no removed or absent hook is invoked.

Suggested tests:

- call close dispatch with no remaining hooks and verify only the primary close path determines the result,
- call ioctl dispatch with no remaining hooks and verify only the primary ioctl path determines the result.

Traceability:

- `execute_close_hooks`
- `execute_ioctl_hooks`

## Requirements

### Functional Requirements

#### FR-1: Maintain a hook chain

The module shall maintain hook entries represented by `fd_hook` link objects in a chain that can be used for later dispatch.

Traceability:

- `register_fd_hook`
- `unregister_fd_hook`
- `fd_hook`

#### FR-2: Support registration of close and ioctl callbacks together

The module shall allow a caller to register a hook entry by supplying a close hook callback, an ioctl hook callback, and the associated link object.

Traceability:

- `register_fd_hook`

#### FR-3: Support removal by link object

The module shall allow a caller to remove a previously registered hook entry by passing its link object.

Traceability:

- `unregister_fd_hook`

#### FR-4: Dispatch close hooks from a supplied remaining-list position

The module shall execute close-hook processing starting from a caller-supplied remaining hook list position, using a supplied primary close function and file descriptor, and shall return an integer result from that processing.

Traceability:

- `execute_close_hooks`

#### FR-5: Dispatch ioctl hooks from a supplied remaining-list position

The module shall execute ioctl-hook processing starting from a caller-supplied remaining hook list position, using a supplied primary ioctl function, file descriptor, request, and argument, and shall return an integer result from that processing.

Traceability:

- `execute_ioctl_hooks`

#### FR-6: Registered entries affect future dispatch until removed

The module shall ensure that a successfully registered hook entry participates in subsequent dispatch operations until that entry is unregistered.

Traceability:

- `register_fd_hook`
- `unregister_fd_hook`
- `execute_close_hooks`
- `execute_ioctl_hooks`

#### FR-7: Unregistered entries do not affect future dispatch

The module shall ensure that an unregistered hook entry no longer participates in subsequent dispatch operations.

Traceability:

- `unregister_fd_hook`
- `execute_close_hooks`
- `execute_ioctl_hooks`

### Key Entities

#### `fd_hook`

A hook-link entity used to represent one registered position in the module’s hook chain.

Role:

- identifies the entry to add or remove,
- participates in chain traversal during dispatch,
- associates registered callbacks with a position in the active sequence.

Traceability:

- `fd_hook` references throughout `gnu/fd-hook.c`
- manipulated by `register_fd_hook`, `unregister_fd_hook`
- consumed by `execute_close_hooks`, `execute_ioctl_hooks`

#### Close hook callback

A callback associated with an `fd_hook` entry for close-related processing.

Role:

- contributes behavior during close-hook dispatch for a file descriptor,
- participates in producing the integer dispatch result.

Traceability:

- accepted by `register_fd_hook`
- used by `execute_close_hooks`

#### Ioctl hook callback

A callback associated with an `fd_hook` entry for ioctl-related processing.

Role:

- contributes behavior during ioctl-hook dispatch for a file descriptor, request, and argument,
- participates in producing the integer dispatch result.

Traceability:

- accepted by `register_fd_hook`
- used by `execute_ioctl_hooks`

#### Primary close function

A close operation supplied to close-hook dispatch.

Role:

- acts as the primary close action within the chain execution path.

Traceability:

- parameter of `execute_close_hooks`

#### Primary ioctl function

An ioctl operation supplied to ioctl-hook dispatch.

Role:

- acts as the primary ioctl action within the chain execution path.

Traceability:

- parameter of `execute_ioctl_hooks`

## Success Criteria

### Behavioral Success Criteria

1. A registered `fd_hook` entry is reached by subsequent matching dispatch operations until it is removed.
   - Verified by registration followed by close and ioctl dispatch tests.
   - Traceability: `register_fd_hook`, `execute_close_hooks`, `execute_ioctl_hooks`

2. After unregistration, the removed `fd_hook` entry is no longer reached by subsequent dispatch operations.
   - Verified by register/unregister/dispatch tests.
   - Traceability: `unregister_fd_hook`, `execute_close_hooks`, `execute_ioctl_hooks`

3. Close dispatch returns an integer result for a supplied file descriptor and primary close function both when hooks are present and when no hooks remain.
   - Verified by dispatch tests with populated and empty remaining-list positions.
   - Traceability: `execute_close_hooks`

4. Ioctl dispatch returns an integer result for supplied fd, request, argument, and primary ioctl function both when hooks are present and when no hooks remain.
   - Traceability: `execute_ioctl_hooks`

5. Multiple registered hook entries can participate in one dispatch chain in a manner consistent with the source module’s remaining-list traversal behavior.
   - Verified by multi-hook dispatch tests for both close and ioctl paths.
   - Traceability: `execute_close_hooks`, `execute_ioctl_hooks`, `fd_hook`

### Port Completion Criteria

1. The Rust module exposes functionality sufficient to perform registration, unregistration, close dispatch, and ioctl dispatch corresponding to the four source functions.
2. The Rust module preserves the source module’s observable hook-chain behavior for all scenarios listed in this specification.
3. All scenarios in the “User Scenarios & Testing” section pass in automated tests.
4. No extra unsupported capabilities are required for acceptance beyond the behavior specified here.