# spec.md

## Title

Rust Functional Specification for `module_gnu_obstack.c_38`

## Metadata

- Project: `cflow-new`
- Module: `module_gnu_obstack.c_38`
- Category: `module_cluster`
- Source file: `gnu/obstack.c`
- Target branch: `044-module_gnu_obstack.c_38-rust-port`
- Generation date: `2026-06-17`

## 1. Feature Specification

### 1.1 Purpose

This module provides the failure-handling behavior associated with GNU obstack support in `gnu/obstack.c`. The analyzed evidence shows one module-local function, `print_and_abort`, and repeated use of the obstack state structures `struct obstack` and `struct _obstack_chunk`.

The Rust rewrite must preserve the module behavior that is evidenced here:

- maintain the obstack-related module domain and its chunk-based state model,
- provide the module’s fatal error path,
- terminate execution in the same class of situations where the C module uses its noreturn abort path.

### 1.2 In-Scope Functionality

The Rust version must implement:

- a module-local fatal failure routine corresponding to `print_and_abort`,
- behavior that emits an error message before abnormal termination,
- integration with the module’s obstack state model so that fatal internal failures do not continue execution.

### 1.3 Out of Scope

The following are not required unless separately evidenced elsewhere in the project:

- new public APIs,
- expanded memory-management features beyond the obstack model already present,
- thread-safety guarantees,
- persistence or serialization,
- recovery from fatal module errors,
- FFI requirements,
- performance targets or benchmark parity.

## 2. User Scenarios & Testing

### 2.1 Scenario: Fatal obstack-related internal failure

A caller uses functionality in this module’s obstack domain, and the module reaches a condition that the original C implementation handles through `print_and_abort`.

Expected Rust behavior:

- the module emits its fatal diagnostic output,
- control does not return to the caller,
- process execution terminates abnormally.

#### Test expectations

- A test harness that triggers the fatal path observes process termination rather than a returned error value.
- Diagnostic output is produced before termination.

### 2.2 Scenario: Obstack state exists across chunk-linked allocation state

Code using this module interacts with obstack state represented by an `obstack` and linked chunk records.

Expected Rust behavior:

- the Rust module represents the same functional relationship between allocator state and backing chunks,
- fatal handling does not leave the module in a resumable state.

#### Test expectations

- Structural tests confirm that the Rust design models an obstack with access to current chunk state and linked chunk progression.
- A failure injected while such state is active still follows the fatal path.

### 2.3 Scenario: Non-returning failure contract

A dependent part of the system relies on the fact that the C helper is declared `__attribute_noreturn__`.

Expected Rust behavior:

- the Rust equivalent is semantically non-returning,
- downstream logic is not required to handle a normal return from the fatal helper.

#### Test expectations

- Static review or type-level inspection shows the function has a non-returning signature or equivalent semantics.
- Runtime testing confirms no normal continuation after invocation.

## 3. Requirements

### 3.1 Functional Requirements

#### FR-1: Fatal diagnostic and abort
The Rust module shall provide behavior equivalent to `print_and_abort` from `gnu/obstack.c:310-324`: when invoked, it must emit a fatal diagnostic and then terminate without returning.

**Traceability:** `print_and_abort` in `gnu/obstack.c:310-324`.

#### FR-2: Non-returning failure semantics
The Rust equivalent of the fatal helper shall preserve the C function’s noreturn contract in observable behavior.

**Traceability:** `static __attribute_noreturn__ void print_and_abort (void);` in `gnu/obstack.c:310-324`.

#### FR-3: Obstack-domain structural modeling
The Rust rewrite shall model the module’s obstack domain using entities corresponding to `struct obstack` and `_obstack_chunk`, including their relationship as allocator state associated with chunk records.

**Traceability:** `struct obstack` occurrences at `gnu/obstack.c:65, 74, 90, 131, 143, 163, 219, 222, 243, 273`; `struct _obstack_chunk` occurrences at `gnu/obstack.c:93, 165, 166, 224, 225, 245, 246, 275`.

#### FR-4: Fatal handling usable within obstack operations
The Rust module shall allow the fatal helper to be used from obstack-related internal operations such that a fatal condition results in immediate termination rather than continued processing.

**Traceability:** module scope in `gnu/obstack.c`; fatal helper `print_and_abort`; obstack and chunk structures listed above.

### 3.2 Key Entities

#### Entity: Obstack state
A stateful obstack entity corresponds to `struct obstack`. It represents the active state of the module’s object-stack allocation domain.

**Traceability:** `struct obstack` occurrences in `gnu/obstack.c`.

#### Entity: Obstack chunk
A chunk entity corresponds to `struct _obstack_chunk`. It represents chunk storage associated with an obstack and linked progression between chunks.

**Traceability:** `struct _obstack_chunk` occurrences in `gnu/obstack.c`.

#### Relationship: Obstack-to-chunk management
An obstack manages or references chunk records as part of its active allocation state. The Rust rewrite must preserve this relationship at the functional level.

**Traceability:** paired use of `struct obstack` and `struct _obstack_chunk` throughout `gnu/obstack.c`.

#### Entity: Fatal helper
A module-local fatal helper corresponds to `print_and_abort`. It is responsible for reporting a fatal condition and ending execution.

**Traceability:** `print_and_abort` in `gnu/obstack.c:310-324`.

## 4. Success Criteria

### 4.1 Behavioral Success Criteria

- Invoking the Rust equivalent of `print_and_abort` produces diagnostic output and does not return.
- Process termination occurs on the fatal path in every tested case.
- No tested caller can continue normal execution after the fatal helper is invoked.

### 4.2 Structural Success Criteria

- The Rust module contains representations for the obstack state entity and chunk entity corresponding to `struct obstack` and `struct _obstack_chunk`.
- The relationship between obstack state and chunk-backed storage is preserved in the Rust design.

### 4.3 Traceability Success Criteria

- Every implemented fatal-path behavior is traceable to `print_and_abort` in `gnu/obstack.c:310-324`.
- Every implemented core entity is traceable to the analyzed `struct obstack` and `struct _obstack_chunk` occurrences in `gnu/obstack.c`.
- The Rust rewrite introduces no required functional promises beyond the evidenced fatal helper behavior and obstack/chunk domain modeling.