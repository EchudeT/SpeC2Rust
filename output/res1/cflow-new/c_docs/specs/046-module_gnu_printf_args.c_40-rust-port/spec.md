# spec.md

## Title

Functional Specification for `module_gnu_printf-args.c_40` Rust Port

## Metadata

- **Project**: `cflow-new`
- **Module**: `module_gnu_printf-args.c_40`
- **Category**: `module_cluster`
- **Source File**: `gnu/printf-args.c`
- **Primary Function**: `PRINTF_FETCHARGS`
- **Rust Branch**: `046-module_gnu_printf_args.c_40-rust-port`
- **Generation Date**: 2026-06-11

## Overview

This module is responsible for fetching variadic arguments for a printf-style formatting workflow and storing the fetched values into a caller-provided argument collection.

The Rust rewrite must preserve the observable behavior of this role:

- consume a variadic argument stream in the order implied by previously determined argument descriptors,
- fetch each argument according to its required type,
- place fetched values into the target argument collection,
- report success or failure through the function result.

This module does not define formatting, output generation, or format-string parsing. Its functional boundary is limited to transferring runtime argument values from a variadic source into a structured argument container according to pre-established argument metadata.

## Feature Specification

### Feature: Fetch printf-style variadic arguments into structured storage

The module shall provide the behavior corresponding to `PRINTF_FETCHARGS`, whose purpose is to read arguments from a `va_list`-style source and populate an `arguments` container.

The Rust version must implement the following functional behavior:

1. Accept an input argument stream representing runtime arguments of a printf-like call.
2. Accept a mutable argument collection whose entries already describe what kinds of values must be fetched.
3. For each required argument entry, fetch exactly one value of the required kind from the input stream.
4. Store the fetched value into the corresponding slot of the argument collection.
5. Preserve positional correspondence between the argument stream and the target entries, except where the pre-established metadata requires a specific fetch pattern.
6. Return an integer status indicating whether argument fetching completed successfully.

### Supported behavior scope

The Rust implementation must support the same behavioral scope evidenced by the source module:

- typed retrieval of printf argument values,
- storage into a structured collection of argument records,
- operation over caller-supplied metadata and storage,
- failure signaling when the module cannot complete argument collection as required.

### Explicit non-scope

The Rust version is not required by this module specification to provide:

- format string parsing,
- text formatting,
- output emission,
- a new public formatting API,
- capabilities beyond the established argument-fetching responsibility.

## User Scenarios & Testing

### Scenario 1: Normal printf argument capture

A higher-level printf subsystem has already analyzed a format string and prepared an `arguments` object describing the arguments that must be collected. It invokes this module with the live variadic argument stream. The module fetches each runtime value and stores it into the prepared container, then returns success.

**Test expectations**

- Given valid argument metadata and a matching argument stream, all required entries are populated.
- The returned status indicates success.
- Stored values correspond to the runtime values passed by the caller.

### Scenario 2: Mixed argument kinds in one call

A higher-level formatting path requires several different argument kinds in a single formatting operation. The module receives the prepared `arguments` collection containing entries of differing required types. It fetches each value using the correct retrieval rule for that entry and stores them without cross-type confusion.

**Test expectations**

- Each entry is fetched according to its declared type requirement.
- Values are stored in the correct target slots.
- No entry receives the value intended for another entry.

### Scenario 3: Repeated use as part of a formatting pipeline

A formatting subsystem repeatedly invokes this module for separate formatting operations, each time with a fresh variadic argument stream and prepared destination collection. The module acts as a pure argument-acquisition stage within the pipeline.

**Test expectations**

- Each invocation depends only on its provided input stream and destination collection.
- Successful completion of one call does not alter the expected behavior of a later independent call.
- The function returns a status on every invocation.

### Scenario 4: Failure propagation on unsupported or unfulfillable fetch

The prepared argument metadata requires fetching that the module cannot complete successfully. The module must stop and report failure through its return value so the higher-level formatting path can react appropriately.

**Test expectations**

- The function returns a failure status when it cannot complete the required fetch behavior.
- Failure is observable through the function result.
- The caller can distinguish success from failure by the returned integer status.

## Requirements

### Functional Requirements

#### FR-1: Variadic source consumption
The module shall consume values from a variadic argument source provided by the caller and use that source as the origin for runtime argument values.

**Traceability**: `gnu/printf-args.c`, `PRINTF_FETCHARGS`

#### FR-2: Caller-provided argument collection population
The module shall populate a caller-provided `arguments` collection rather than creating a separate external result representation.

**Traceability**: `gnu/printf-args.c`, `PRINTF_FETCHARGS`, type `arguments`

#### FR-3: Type-directed fetching
The module shall fetch each argument according to the type requirement already recorded in the destination argument collection.

**Traceability**: `gnu/printf-args.c`, `PRINTF_FETCHARGS`, type `arguments`

#### FR-4: Per-entry storage
For each required argument entry, the module shall store the fetched runtime value into the corresponding location in the argument collection.

**Traceability**: `gnu/printf-args.c`, `PRINTF_FETCHARGS`, type `arguments`

#### FR-5: Ordered completion of argument acquisition
The module shall complete argument acquisition for the full set of required entries represented in the provided argument collection, unless a failure condition prevents completion.

**Traceability**: `gnu/printf-args.c`, `PRINTF_FETCHARGS`, type `arguments`

#### FR-6: Integer status reporting
The module shall report completion status using the integer return value of `PRINTF_FETCHARGS`, allowing callers to distinguish success from failure.

**Traceability**: `gnu/printf-args.c`, `PRINTF_FETCHARGS`

### Key Entities

#### `arguments`
A caller-owned structured collection that describes which printf-style arguments must be fetched and provides storage locations for the fetched values.

**Role in module**
- acts as the destination for fetched arguments,
- provides the per-argument metadata that determines fetch behavior,
- holds the stored runtime values after successful completion.

**Relationships**
- `PRINTF_FETCHARGS` reads fetch requirements from `arguments`,
- `PRINTF_FETCHARGS` writes fetched values back into `arguments`.

#### Variadic argument source (`va_list` input)
A caller-supplied runtime argument stream representing the actual arguments passed to a printf-like invocation.

**Role in module**
- serves as the sequential source of argument values,
- is consumed by `PRINTF_FETCHARGS` according to the needs described by `arguments`.

**Relationships**
- `PRINTF_FETCHARGS` bridges the variadic source and the `arguments` collection.

## Success Criteria

### SC-1: Successful population for valid input
For any invocation where the provided variadic input matches the requirements encoded in the provided `arguments` collection, the Rust version returns the success status and populates the destination collection with corresponding values.

**Traceability**: `gnu/printf-args.c`, `PRINTF_FETCHARGS`, type `arguments`

### SC-2: Correct type-to-slot mapping
For test cases containing multiple required argument kinds, each fetched runtime value is stored in the slot designated for that argument and interpreted according to the entry’s required type.

**Traceability**: `gnu/printf-args.c`, `PRINTF_FETCHARGS`, type `arguments`

### SC-3: No unintended responsibility expansion
The Rust version limits itself to argument fetching and storage behavior and does not require format parsing, formatting, or output generation to satisfy this module’s contract.

**Traceability**: `gnu/printf-args.c`, `PRINTF_FETCHARGS`

### SC-4: Observable failure signaling
For invocations where the module cannot complete required argument fetching, the Rust version returns a failure status that callers can test.

**Traceability**: `gnu/printf-args.c`, `PRINTF_FETCHARGS`

### SC-5: Invocation independence
In repeated independent invocations with separate input streams and separate destination collections, results are determined solely by the provided inputs of each call.

**Traceability**: `gnu/printf-args.c`, `PRINTF_FETCHARGS`

## Acceptance Notes

- Conformance is determined by matching the source module’s externally visible behavior at the argument-fetching boundary.
- The Rust rewrite may use different internal representations, but it must preserve the functional contract defined above.
- Any behavior claimed by the Rust port must be traceable to the source file and primary function listed in this specification.