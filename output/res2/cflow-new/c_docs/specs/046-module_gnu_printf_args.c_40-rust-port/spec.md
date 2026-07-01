# spec.md

## Overview

This module is responsible for fetching variadic printf-style argument values from a `va_list` and storing them into a prepared argument container. The C module exposes one main function, `PRINTF_FETCHARGS`, whose behavior defines the module boundary.

The Rust rewrite must preserve the same functional role:

- consume a variadic argument source corresponding to previously analyzed format arguments,
- read each required argument value in sequence according to its recorded type/classification,
- store the fetched values into the destination argument collection,
- report success or failure through the function result.

This module does not define format-string parsing. It operates on an existing argument description/container and populates it from the provided variadic argument list.

## Scope

In scope for the Rust version:

- fetching argument values from a variadic argument source based on pre-established argument metadata,
- populating the destination argument records with fetched values,
- returning status indicating whether all required arguments were fetched successfully.

Out of scope:

- parsing format strings,
- rendering or printing formatted output,
- defining new argument kinds beyond those required by the source module,
- introducing new public APIs or behavior not evidenced by `gnu/printf-args.c`.

## Feature Specification

### Feature: Variadic printf argument extraction

The module shall populate an `arguments` container from an input `va_list` by fetching each argument according to the type information already associated with that container.

Behavioral expectations:

- The module reads arguments in the order required by the prepared argument descriptors.
- Each fetched value is stored into the corresponding slot in the destination container.
- The module supports the set of argument categories expected by GNU-style printf argument handling as represented in the source module.
- The module returns an integer status so callers can detect whether population succeeded.

### Supported functional boundary

The Rust version must implement the functional equivalent of:

- taking a source of variadic arguments,
- iterating over the expected argument entries in the destination collection,
- dispatching fetch behavior based on each entry’s required type,
- storing the fetched typed value into the destination collection,
- stopping with failure status if the source module would fail for an unsupported or invalid argument expectation.

## User Scenarios & Testing

### Scenario 1: Populate arguments after format analysis

A caller has already analyzed a printf-style format string and prepared an `arguments` structure describing the expected arguments. The caller passes the active variadic argument list and the prepared container to this module.

Expected result:

- all required values are fetched from the variadic source,
- each value is written into its matching destination slot,
- the function returns success.

Test coverage should verify:

- values are fetched in the expected sequence,
- stored values match the original call arguments,
- no expected slot remains unpopulated on success.

### Scenario 2: Mixed argument kinds in a single call

A caller uses a format requiring different argument kinds, such as integers, character or string pointers, floating-point values, and other supported printf argument classes. The prepared container includes multiple type classifications.

Expected result:

- each argument is fetched using the correct interpretation for its recorded kind,
- the destination collection preserves the correspondence between descriptor and value,
- the function returns success when all kinds are supported.

Test coverage should verify:

- heterogeneous argument lists are handled correctly,
- adjacent arguments of different kinds do not shift or corrupt one another,
- retrieved values compare equal to the provided inputs.

### Scenario 3: Width/precision or other non-output arguments included in the descriptor set

A caller’s prepared argument metadata includes arguments that are consumed for formatting control as well as arguments used as formatted values.

Expected result:

- the module fetches every described argument from the variadic source, not only output-value arguments,
- control arguments and value arguments occupy their correct destination positions.

Test coverage should verify:

- control-related integer arguments are fetched and stored like other expected arguments,
- later value arguments remain aligned after such fetches.

### Scenario 4: Invalid or unsupported descriptor state

A caller provides an `arguments` container containing an argument expectation that cannot be fetched according to the module’s supported type classifications.

Expected result:

- the function reports failure through its return value,
- the destination container is not falsely reported as fully populated.

Test coverage should verify:

- failure is reported for unsupported or invalid argument classifications evidenced by the source behavior,
- success is not returned when required fetching cannot be completed.

## Requirements

### Functional Requirements

#### FR-1: Accept prepared argument metadata and a variadic argument source
The module shall accept an input variadic argument list and a destination `arguments` container whose entries already describe the arguments to fetch.

Traceability: `PRINTF_FETCHARGS` in `gnu/printf-args.c`.

#### FR-2: Fetch values according to recorded argument kind
The module shall fetch each argument using the type/classification recorded for that argument entry in the destination container.

Traceability: `PRINTF_FETCHARGS` in `gnu/printf-args.c`.

#### FR-3: Preserve argument order
The module shall consume values from the variadic source in the same logical order as required by the prepared argument entries.

Traceability: `PRINTF_FETCHARGS` in `gnu/printf-args.c`.

#### FR-4: Populate destination entries with fetched values
The module shall store each fetched value into its corresponding entry in the destination argument collection.

Traceability: `PRINTF_FETCHARGS` in `gnu/printf-args.c`.

#### FR-5: Support heterogeneous printf argument categories represented by the source module
The module shall handle the set of argument categories that the source function fetches for GNU printf argument processing, including ordinary value arguments and formatting-control arguments when present in the prepared descriptors.

Traceability: `PRINTF_FETCHARGS` in `gnu/printf-args.c`.

#### FR-6: Return explicit status
The module shall return an integer status indicating success or failure of argument fetching.

Traceability: `PRINTF_FETCHARGS` signature and behavior in `gnu/printf-args.c`.

#### FR-7: Detect unsupported or invalid fetch expectations
If an argument entry requires a fetch operation not supported by the module’s represented argument classifications, the module shall return failure.

Traceability: `PRINTF_FETCHARGS` in `gnu/printf-args.c`.

### Key Entities

#### `arguments`
The destination container holding the prepared set of expected printf arguments and their fetched values.

Relationship:
- consumed by `PRINTF_FETCHARGS` as the output target,
- contains per-argument metadata sufficient to determine how each value must be fetched,
- receives the fetched typed values.

Traceability: `PRINTF_FETCHARGS (va_list args, arguments *a)`.

#### Variadic argument source (`va_list`)
The ordered runtime source of argument values supplied by the caller.

Relationship:
- consumed sequentially by the module,
- interpreted according to the metadata stored in `arguments`.

Traceability: `PRINTF_FETCHARGS (va_list args, arguments *a)`.

#### Per-argument descriptor/value entry
An entry within the `arguments` container representing one expected argument, including its required type/classification and storage for the fetched value.

Relationship:
- determines which fetch operation is applied,
- is populated from the variadic source during module execution.

Traceability: behavior of `PRINTF_FETCHARGS` in `gnu/printf-args.c`.

## Success Criteria

### SC-1: Correct population for valid prepared inputs
Given a valid prepared `arguments` container and matching variadic inputs, the Rust module populates every described entry with the correct value and returns success.

Traceability: FR-1, FR-2, FR-4, FR-6.

### SC-2: Correct handling of mixed argument kinds
For test cases containing multiple supported argument kinds in one variadic sequence, each stored result matches the original supplied argument under the corresponding descriptor.

Traceability: FR-2, FR-3, FR-5.

### SC-3: Positional integrity of fetched arguments
For test cases that include formatting-control arguments alongside ordinary value arguments, all later fetched values remain aligned with their intended descriptors.

Traceability: FR-3, FR-5.

### SC-4: Failure signaling on invalid or unsupported expectations
For prepared descriptor states that the source module does not support for fetching, the Rust module returns failure rather than success.

Traceability: FR-6, FR-7.

### SC-5: Functional parity at module boundary
At the module boundary defined by `PRINTF_FETCHARGS`, the Rust rewrite exhibits equivalent observable behavior: it consumes the provided variadic source according to prepared argument metadata, populates the destination container, and reports success or failure.

Traceability: `PRINTF_FETCHARGS` in `gnu/printf-args.c`.