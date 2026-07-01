# spec.md

## Title

Functional Specification: `module_gnu_fd_hook_06` Rust Port

## Document Control

- Project: `cflow-new`
- Module: `module_gnu_fd_hook_06`
- Category: `module_cluster`
- Source basis: `gnu/fd-hook.c`
- Rust branch: `012-module_gnu_fd_hook_06-rust-port`
- Generation date: 2026-06-11

## Overview

This module provides a registry of file-descriptor hook entries and supports executing registered hook chains around two descriptor-related operations:

- close-like operations
- ioctl-like operations

The source module exposes behavior for:

- registering a hook entry into a global hook list
- unregistering a previously registered hook entry
- executing close hooks across the registered chain with a supplied primary operation
- executing ioctl hooks across the registered chain with a supplied primary operation

The Rust rewrite must preserve this functional behavior and the observable ordering and chaining semantics evidenced by the C module.

## Scope

Included in scope:

- management of a module-level collection of registered `fd_hook` entries
- chaining behavior for close-hook execution
- chaining behavior for ioctl-hook execution
- add/remove behavior for hook entries supplied by callers

Out of scope unless evidenced elsewhere:

- any new public API beyond the behaviors represented by the analyzed functions
- thread-safety guarantees
- persistence, serialization, or recovery behavior
- extended descriptor operations other than close and ioctl
- validation rules not evidenced by the source module

## Feature Specification

### Feature: File Descriptor Hook Registry

The module maintains a registry of hook entries represented by `fd_hook` link objects supplied by callers. Each entry may carry:

- a close hook callback
- an ioctl hook callback
- link information used to participate in the registry chain

The Rust version must support registration of a supplied hook entry into the active registry and removal of a previously registered entry from that registry.

### Feature: Close Operation Hook Chaining

The module supports execution of a close operation through a hook chain.

Behavior evidenced by `execute_close_hooks`:

- execution begins with a provided list position and a provided primary close-like function
- if hook entries remain, the chain proceeds through registered close hooks
- when no more hook entries remain, the primary close-like function is the terminal operation
- the overall result is an integer status returned by the hook chain or terminal primary operation

The Rust version must preserve the same chaining role: registered close hooks participate before delegation reaches the supplied primary operation.

### Feature: Ioctl Operation Hook Chaining

The module supports execution of an ioctl operation through a hook chain.

Behavior evidenced by `execute_ioctl_hooks`:

- execution begins with a provided list position and a provided primary ioctl-like function
- the operation includes `fd`, `request`, and `arg`
- if hook entries remain, the chain proceeds through registered ioctl hooks
- when no more hook entries remain, the primary ioctl-like function is the terminal operation
- the overall result is an integer status returned by the hook chain or terminal primary operation

The Rust version must preserve the same chaining role for ioctl hooks and terminal delegation to the supplied primary operation.

## User Scenarios & Testing

### Scenario 1: Register a hook entry for later interception

A caller prepares an `fd_hook` link object containing hook callbacks and registers it.

Expected behavior:

- the registry accepts the supplied link object
- later hook-chain execution can encounter this entry as part of traversal
- no new hook object allocation by the module is required by the observable contract; the caller-provided link object is the registered entity

Test coverage:

- register one hook entry, then execute a hook chain and verify that the registered callback is reached

### Scenario 2: Multiple hook entries participate in a chain

A caller registers multiple `fd_hook` entries and then triggers a close or ioctl hook execution.

Expected behavior:

- more than one registered entry can participate in the chain
- chain traversal continues from one remaining entry to the next until the primary operation is reached
- the returned integer result is the one produced by the effective chain execution

Test coverage:

- register multiple entries with observable side effects
- execute close-hook chain and verify each intended hook is invoked before the terminal primary function
- execute ioctl-hook chain and verify propagation of `fd`, `request`, and `arg`

### Scenario 3: Unregister a hook entry

A caller unregisters a previously registered `fd_hook` link object.

Expected behavior:

- the removed entry is no longer part of future chain traversal
- other still-registered entries remain usable

Test coverage:

- register two entries
- unregister one
- execute a hook chain and verify only the remaining registered entry is observed

### Scenario 4: Empty or exhausted chain falls back to the primary operation

A caller executes hook processing from a position with no remaining hooks.

Expected behavior:

- the supplied primary operation is invoked directly
- the return value from the primary operation becomes the module result

Test coverage:

- execute close-hook chain with no remaining entries and verify direct primary invocation
- execute ioctl-hook chain with no remaining entries and verify direct primary invocation with unchanged arguments

### Scenario 5: Result propagation through hooks

A registered hook or the terminal primary operation returns a status code.

Expected behavior:

- the integer return value produced by chain execution is propagated unchanged to the caller of the execute function

Test coverage:

- use deterministic hook and primary callbacks returning distinct values
- verify the execute function returns the exact expected value for both close and ioctl paths

## Requirements

### Functional Requirements

#### FR-1: Hook registration

The module shall allow a caller to register a caller-supplied `fd_hook` link object into the module's active hook registry.

Traceability:

- `register_fd_hook` in `gnu/fd-hook.c`

#### FR-2: Hook unregistration

The module shall allow a caller to remove a previously registered caller-supplied `fd_hook` link object from the module's active hook registry.

Traceability:

- `unregister_fd_hook` in `gnu/fd-hook.c`

#### FR-3: Close hook chain execution

The module shall execute a close-hook chain beginning from a supplied remaining-list position and using a supplied primary close-like function as terminal fallback.

Traceability:

- `execute_close_hooks` in `gnu/fd-hook.c`

#### FR-4: Close fallback behavior

When close-hook execution reaches a state with no remaining hook entry to process, the module shall invoke the supplied primary close-like function with the provided file descriptor and return that result.

Traceability:

- `execute_close_hooks` in `gnu/fd-hook.c`

#### FR-5: Ioctl hook chain execution

The module shall execute an ioctl-hook chain beginning from a supplied remaining-list position and using a supplied primary ioctl-like function as terminal fallback.

Traceability:

- `execute_ioctl_hooks` in `gnu/fd-hook.c`

#### FR-6: Ioctl argument forwarding

During ioctl-hook execution, the module shall preserve and forward the provided `fd`, `request`, and `arg` values through chain processing and terminal primary invocation.

Traceability:

- `execute_ioctl_hooks` in `gnu/fd-hook.c`

#### FR-7: Return-value propagation

The module shall return the integer status produced by the effective close or ioctl chain execution, including direct terminal primary invocation.

Traceability:

- `execute_close_hooks` in `gnu/fd-hook.c`
- `execute_ioctl_hooks` in `gnu/fd-hook.c`

### Key Entities

#### `fd_hook`

A hook-link entity used by the module registry and chain execution.

Observed role from the module:

- represents one node in the hook chain
- associates close-hook and ioctl-hook behavior with link identity
- is supplied by the caller to registration and unregistration operations
- can be referenced as the current or remaining position during execute operations

Relationships:

- `register_fd_hook` adds an `fd_hook` to the active registry
- `unregister_fd_hook` removes an `fd_hook` from the active registry
- `execute_close_hooks` traverses `fd_hook` entries for close behavior
- `execute_ioctl_hooks` traverses `fd_hook` entries for ioctl behavior

#### Close hook callback

A callback associated with an `fd_hook` entry for close-like processing.

Observed role from the module:

- participates in close chain execution before terminal fallback to the supplied primary function

Relationship:

- attached to an `fd_hook`
- used by `register_fd_hook`
- consumed by `execute_close_hooks`

#### Ioctl hook callback

A callback associated with an `fd_hook` entry for ioctl-like processing.

Observed role from the module:

- participates in ioctl chain execution before terminal fallback to the supplied primary function

Relationship:

- attached to an `fd_hook`
- used by `register_fd_hook`
- consumed by `execute_ioctl_hooks`

#### Primary close-like function

A supplied operation that serves as the terminal close action when no further close hooks remain.

Relationship:

- consumed by `execute_close_hooks`

#### Primary ioctl-like function

A supplied operation that serves as the terminal ioctl action when no further ioctl hooks remain.

Relationship:

- consumed by `execute_ioctl_hooks`

## Success Criteria

### SC-1: Registration affects later execution

After registering a valid hook entry, a subsequent relevant execute call shall be able to observe that entry in chain processing.

Traceability:

- `register_fd_hook`
- `execute_close_hooks`
- `execute_ioctl_hooks`

Verification:

- automated tests demonstrate callback invocation after registration

### SC-2: Unregistration removes future participation

After unregistering a previously registered hook entry, subsequent execute calls shall not observe that entry in chain processing.

Traceability:

- `unregister_fd_hook`
- `execute_close_hooks`
- `execute_ioctl_hooks`

Verification:

- automated tests demonstrate absence of invocation after unregistration

### SC-3: Terminal fallback works for close

When close execution starts with no remaining hook entry, the supplied primary close function shall be invoked exactly for that execution path and its integer result shall be returned.

Traceability:

- `execute_close_hooks`

Verification:

- automated test with no remaining hooks confirms direct primary invocation and returned status

### SC-4: Terminal fallback works for ioctl

When ioctl execution starts with no remaining hook entry, the supplied primary ioctl function shall be invoked with the same `fd`, `request`, and `arg` values and its integer result shall be returned.

Traceability:

- `execute_ioctl_hooks`

Verification:

- automated test confirms argument equality and returned status

### SC-5: Multiple registered hooks are chain-participating entities

With multiple registered hook entries present, execute operations shall support traversal across more than one entry before terminal fallback.

Traceability:

- `register_fd_hook`
- `execute_close_hooks`
- `execute_ioctl_hooks`
- `fd_hook`

Verification:

- automated tests with at least two registered entries confirm multi-entry participation

### SC-6: Return statuses are preserved

For both close and ioctl execution paths, the integer result observed by the caller shall match the result produced by the effective chain outcome.

Traceability:

- `execute_close_hooks`
- `execute_ioctl_hooks`

Verification:

- automated tests use distinct callback and primary return codes and assert exact propagation

## Acceptance Notes

- The Rust port should match the source module's evidenced functional behavior only.
- Any design choices needed to express the registry and chaining semantics in Rust must not alter the above observable requirements.
- Undefined or unproven behavior from the analyzed source should not be added to the specification.