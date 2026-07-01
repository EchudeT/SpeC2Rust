# spec.md

## Title

Rust Functional Specification for `module_gnu_obstack.c_38`

## Metadata

- Project: `cflow-new`
- Module: `module_gnu_obstack.c_38`
- Category: `module_cluster`
- Source file: `gnu/obstack.c`
- Rust branch: `044-module_gnu_obstack.c_38-rust-port`
- Generation date: `2026-06-11`

## Overview

This module provides the failure-path support used by GNU obstack behavior in `gnu/obstack.c`. The analyzed evidence exposes one concrete function, `print_and_abort`, and repeated references to the obstack state and chunk structures used by the surrounding file.

The Rust rewrite must preserve the module’s observable functional role within obstack operation failure handling:

- support the obstack subsystem’s allocation/chunk management model as represented by `struct obstack` and `struct _obstack_chunk`,
- provide the fatal error path used when the module reaches an unrecoverable state,
- terminate execution in a non-returning way when that fatal path is invoked.

This specification is limited to behavior evidenced by the analyzed file and symbols. It does not introduce new capabilities beyond that scope.

## Feature Specification

### Feature: Fatal termination for unrecoverable obstack failure

The module must provide the fatal behavior represented by `print_and_abort`.

Behavior required:

- When the fatal path is entered, the module emits an error indication suitable for the obstack failure case.
- After emitting that indication, control must not return to the caller.
- Program termination must occur through an abort-style failure outcome rather than normal return.

Traceability:

- `print_and_abort` in `gnu/obstack.c:310-324`
- `__attribute_noreturn__` on `print_and_abort`

### Feature: Compatibility with obstack state/chunk model

The module exists in the context of GNU obstack management and must remain behaviorally compatible with the obstack model represented in the source file.

Behavior required:

- The Rust version must preserve the functional relationship between:
  - an obstack state object, and
  - its linked chunk objects.
- Any failure path exposed by this module must be applicable to operations over that obstack/chunk model.
- The rewrite must not alter the module’s role from failure handling within obstack management to a different abstraction.

Traceability:

- `struct obstack` occurrences in `gnu/obstack.c`
- `struct _obstack_chunk` occurrences in `gnu/obstack.c`

## User Scenarios & Testing

### Scenario 1: Unrecoverable obstack-related failure terminates execution

A caller reaches a condition in obstack processing that the original C module handles through its fatal routine.

Expected result:

- the Rust module produces the fatal error indication,
- execution does not continue past the fatal call site,
- the process ends abnormally.

Test approach:

- invoke the Rust equivalent of the fatal routine from a controlled test process,
- assert non-zero abnormal termination,
- assert that control is not observed after invocation.

Traceability:

- `print_and_abort`

### Scenario 2: Obstack state remains modeled around chunks

A caller uses the Rust rewrite as part of the obstack subsystem and relies on the same conceptual entities as the C source: obstack state and chunk records.

Expected result:

- the rewrite exposes or internally preserves an obstack state object associated with chunk storage,
- failure handling remains tied to this obstack context rather than becoming detached generic process termination.

Test approach:

- inspect integration points or unit structure for the presence of equivalent obstack/chunk entities,
- verify that the fatal path is reachable from obstack-related logic.

Traceability:

- `struct obstack`
- `struct _obstack_chunk`

### Scenario 3: Fatal path is non-returning by contract

A caller must be able to rely on the fatal routine never returning.

Expected result:

- the Rust implementation has non-returning semantics equivalent to the C function attribute,
- no post-call state change or computation occurs in the same execution path.

Test approach:

- validate signature/typing and runtime behavior in a subprocess test,
- verify that code after the call is unreachable in practice.

Traceability:

- `print_and_abort` with `__attribute_noreturn__`

## Requirements

### Functional Requirements

#### FR-1: Fatal obstack failure handling

The module shall implement the fatal failure behavior represented by `print_and_abort` for unrecoverable obstack-related error states.

Traceability:

- `gnu/obstack.c:310-324`

#### FR-2: Error indication before termination

The fatal failure behavior shall emit an error indication before process termination.

Traceability:

- `print_and_abort` in `gnu/obstack.c:310-324`

#### FR-3: Non-returning termination

The fatal failure behavior shall never return to its caller.

Traceability:

- `print_and_abort`
- `__attribute_noreturn__`

#### FR-4: Abort-style process end

The fatal failure behavior shall end execution through abort-style abnormal termination rather than normal completion.

Traceability:

- `print_and_abort`

#### FR-5: Preserve obstack/chunk functional context

The Rust rewrite shall preserve the module’s functional context within GNU obstack handling, specifically the relationship between obstack state and obstack chunks as represented in the source file.

Traceability:

- `struct obstack`
- `struct _obstack_chunk`
- containing file `gnu/obstack.c`

### Key Entities

#### Entity: Obstack state

Represents the active state of an obstack-managed allocation context.

Evidence:

- repeated `struct obstack` definitions/usages in `gnu/obstack.c`

Role in module:

- provides the state context in which chunk-based storage management occurs,
- is the logical owner/controller of chunk progression relevant to this file’s behavior.

#### Entity: Obstack chunk

Represents a storage chunk associated with obstack state.

Evidence:

- repeated `struct _obstack_chunk` definitions/usages in `gnu/obstack.c`

Role in module:

- forms the chunk-level storage units managed under an obstack state,
- participates in the context from which fatal failure can arise.

#### Relationship: Obstack state to chunks

An obstack state is associated with one or more chunks that constitute its managed storage progression.

Evidence:

- co-occurrence of `struct obstack` and `struct _obstack_chunk` across the file

Role in module:

- defines the subsystem context for the fatal path implemented by this module.

## Success Criteria

### SC-1: Fatal routine does not return

In tests, invoking the Rust equivalent of `print_and_abort` must never produce an observable return to the caller.

Traceability:

- `print_and_abort`
- `__attribute_noreturn__`

### SC-2: Fatal routine terminates abnormally

In subprocess-based tests, invoking the fatal routine must end the process abnormally rather than with a normal success exit.

Traceability:

- `print_and_abort`

### SC-3: Error indication is emitted on fatal path

In tests capturing output streams, the fatal path must emit an error indication before termination.

Traceability:

- `print_and_abort`

### SC-4: Obstack/chunk model remains represented in the rewrite

The Rust module or its direct integration boundary must retain entities corresponding to obstack state and obstack chunks, with an evident relationship between them.

Traceability:

- `struct obstack`
- `struct _obstack_chunk`

### SC-5: No unsupported behavioral expansion

The Rust rewrite must not claim or require capabilities not evidenced by this module analysis, including unrelated public features beyond fatal handling in the obstack context.

Traceability:

- single evidenced function `print_and_abort`
- obstack-related entities in `gnu/obstack.c`