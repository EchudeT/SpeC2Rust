# spec.md

## Title

Rust Functional Specification for `module_gnu_execute_all_15`

## Document Metadata

- Project: `cflow-new`
- Module: `module_gnu_execute_all_15`
- Category: `module_cluster`
- Source basis: `gnu/fd-hook.c`
- Rust branch: `021-module_gnu_execute_all_15-rust-port`
- Generation date: `2026-06-17`

## Overview

This module provides hook-dispatch behavior for file-descriptor-related operations. Its evidenced responsibility is to execute all registered hooks for two operation families:

- close handling
- ioctl handling

For each supported operation, the module accepts a primary operation callback and a file descriptor context, invokes the primary callback, and also executes all registered hooks associated with that operation family. The Rust rewrite must preserve this behavior and its observable result handling for both supported dispatch paths.

## Scope

In scope for this specification:

- Functional behavior of executing all close hooks.
- Functional behavior of executing all ioctl hooks.
- Representation of hook groups sufficient to support those executions.
- Result propagation behavior of the dispatch functions.

Out of scope:

- Any new hook categories beyond those evidenced.
- Any new public API not evidenced by the source analysis.
- Thread-safety guarantees.
- Persistence, serialization, recovery, or benchmarking features.
- FFI design requirements.

## Feature Specification

### Feature: Execute all close hooks

The module must provide behavior equivalent to `execute_all_close_hooks`, whose purpose is to perform close-related dispatch for a given file descriptor.

The Rust version must:

- accept a primary close callback and a target file descriptor
- execute the primary callback as part of the dispatch
- execute every registered close hook associated with the module
- return an integer result consistent with the source module’s observable dispatch outcome

This feature exists to ensure that close-related processing is not limited to the primary operation alone and that all registered close observers/handlers are included in the operation flow.

### Feature: Execute all ioctl hooks

The module must provide behavior equivalent to `execute_all_ioctl_hooks`, whose purpose is to perform ioctl-related dispatch for a given file descriptor, request, and argument pointer/context.

The Rust version must:

- accept a primary ioctl callback, a target file descriptor, a request value, and an argument context
- execute the primary callback as part of the dispatch
- execute every registered ioctl hook associated with the module
- return an integer result consistent with the source module’s observable dispatch outcome

This feature exists to ensure that ioctl-related processing is dispatched across the full registered hook set for that operation family.

## User Scenarios & Testing

### Scenario 1: Dispatching close processing for a descriptor

A caller has a file descriptor that is being closed and provides the module with the primary close operation. The module runs the close dispatch path so that the primary operation and all registered close hooks are executed for that descriptor.

The Rust version must support testing that:

- the close dispatch path accepts a file descriptor and primary callback
- registered close hooks are all included in the dispatch
- the function returns an integer result
- behavior is consistent across repeated invocations with the same registered hook set

### Scenario 2: Dispatching ioctl processing for a descriptor

A caller needs to perform an ioctl-related operation on a file descriptor and provides the module with the primary ioctl operation, request code, and argument context. The module runs the ioctl dispatch path so that the primary operation and all registered ioctl hooks are executed.

The Rust version must support testing that:

- the ioctl dispatch path accepts a file descriptor, request, argument context, and primary callback
- registered ioctl hooks are all included in the dispatch
- the function returns an integer result
- distinct request and argument inputs are forwarded through the dispatch path

### Scenario 3: Operation-specific hook separation

A caller uses both dispatch paths in the same program. Close dispatch should use only close hooks, and ioctl dispatch should use only ioctl hooks.

The Rust version must support testing that:

- invoking the close path does not trigger ioctl hooks
- invoking the ioctl path does not trigger close hooks
- operation-family dispatch remains separated even when both hook families are present

## Requirements

### Functional Requirements

#### FR-1: Close hook dispatch
Traceable to: `gnu/fd-hook.c`, `execute_all_close_hooks`

The module shall provide a close-operation dispatch function that executes all close hooks for a supplied file descriptor using the supplied primary close callback.

#### FR-2: Ioctl hook dispatch
Traceable to: `gnu/fd-hook.c`, `execute_all_ioctl_hooks`

The module shall provide an ioctl-operation dispatch function that executes all ioctl hooks for a supplied file descriptor, request value, and argument context using the supplied primary ioctl callback.

#### FR-3: Primary callback participation
Traceable to: `gnu/fd-hook.c`, `execute_all_close_hooks`, `execute_all_ioctl_hooks`

For each supported dispatch path, the module shall include the caller-supplied primary callback in the operation flow rather than limiting execution to registered hooks only.

#### FR-4: Per-operation hook family separation
Traceable to: `gnu/fd-hook.c`, `struct fd_hook`, `execute_all_close_hooks`, `execute_all_ioctl_hooks`

The module shall maintain operation-family-specific hook groupings such that close dispatch executes close hooks and ioctl dispatch executes ioctl hooks.

#### FR-5: Integer result return
Traceable to: `gnu/fd-hook.c`, `execute_all_close_hooks`, `execute_all_ioctl_hooks`

Each dispatch function shall return an integer result representing the dispatch outcome in a manner compatible with the source module’s observable behavior.

### Key Entities

#### Entity: File-descriptor hook group
Traceable to: `gnu/fd-hook.c`, anonymous `struct fd_hook`

A hook group represents the module-managed collection of callbacks associated with a specific file-descriptor operation family.

Observed roles include:

- storing hook membership for close-related dispatch
- storing hook membership for ioctl-related dispatch

#### Entity: Primary operation callback
Traceable to: `gnu/fd-hook.c`, `gl_close_fn`, `gl_ioctl_fn`, dispatch functions

A primary operation callback is caller-supplied behavior for the underlying file-descriptor operation that the module dispatches alongside registered hooks.

Relationships:

- one close dispatch call uses one primary close callback and the close hook group
- one ioctl dispatch call uses one primary ioctl callback and the ioctl hook group

#### Entity: Dispatch input context
Traceable to: `gnu/fd-hook.c`, dispatch function signatures

Dispatch input context consists of the operation arguments forwarded through a dispatch call:

- close context: file descriptor
- ioctl context: file descriptor, request value, argument context

Relationships:

- dispatch functions consume input context and apply it to the primary callback and corresponding hook family

## Success Criteria

### SC-1: Close dispatch completeness
Traceable to: `execute_all_close_hooks`

Given a configured set of close hooks and a primary close callback, invoking the Rust close dispatch shall execute the close dispatch path and produce an integer return result.

### SC-2: Ioctl dispatch completeness
Traceable to: `execute_all_ioctl_hooks`

Given a configured set of ioctl hooks and a primary ioctl callback, invoking the Rust ioctl dispatch shall execute the ioctl dispatch path and produce an integer return result.

### SC-3: Operation-family isolation
Traceable to: `struct fd_hook`, `execute_all_close_hooks`, `execute_all_ioctl_hooks`

Tests exercising both supported dispatch paths shall show that close dispatch uses the close hook group and ioctl dispatch uses the ioctl hook group, without cross-invocation between those groups.

### SC-4: Argument-path preservation
Traceable to: `execute_all_close_hooks`, `execute_all_ioctl_hooks`

Tests shall confirm that:
- close dispatch receives and uses the supplied file descriptor
- ioctl dispatch receives and uses the supplied file descriptor, request value, and argument context

### SC-5: Source-compatible functional coverage
Traceable to: `gnu/fd-hook.c`

The Rust rewrite shall implement both evidenced dispatch behaviors present in the source module and shall not omit either supported operation family.