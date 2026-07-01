# spec.md

## Title

Functional Specification: `module_gnu_printf-args.c_40` Rust Port

## Metadata

- Project: `cflow-new`
- Source module: `gnu/printf-args.c`
- Primary function: `PRINTF_FETCHARGS`
- Module category: `module_cluster`
- Target Rust branch: `046-module_gnu_printf_args.c_40-rust-port`
- Generation date: `2026-06-17`

## Overview

This module is responsible for fetching variadic argument values from an input argument list and storing them into an argument collection according to argument type metadata already prepared elsewhere.

The Rust rewrite must preserve this functional role:

- consume an incoming variadic-style argument sequence,
- interpret the expected argument kinds from the destination argument collection,
- retrieve each argument using the correct type category,
- store the retrieved value into the corresponding slot in the destination collection,
- report success or failure through the module’s result status.

This module does not define format-string parsing. It operates on argument descriptors that already specify what each argument’s type is expected to be.

## Scope

### In Scope

- Populating an existing argument collection from an input variadic argument list.
- Selecting the correct fetch behavior based on per-argument type classification.
- Writing fetched values into the destination argument storage.
- Returning a status code indicating whether argument fetching completed successfully.

### Out of Scope

- Parsing format strings.
- Building argument type metadata from format directives.
- Producing formatted output.
- Defining new external APIs beyond the behavior represented by the source module.

## Feature Specification

The Rust version must implement the module behavior represented by `PRINTF_FETCHARGS`.

The function’s purpose is to traverse the argument descriptors contained in an `arguments` collection and fetch each required value from the supplied argument list using the descriptor’s expected type. The fetched value is then stored into the corresponding entry in the destination collection.

Observed functional boundary:

1. The caller provides:
   - an input argument list,
   - a destination `arguments` object that already contains argument count and type information.

2. The module processes the destination collection entry by entry.

3. For each entry, the module determines the expected argument category from the existing metadata.

4. The module fetches exactly one argument value of the required category from the input sequence and stores it into that entry’s value slot.

5. The module completes with an integer status result indicating success or failure.

The Rust rewrite must preserve this behavior, including the dependency on preexisting type metadata in the destination collection.

## User Scenarios & Testing

### Scenario 1: Populate arguments for a parsed printf call

A higher-level printf subsystem has already analyzed a format string and built an `arguments` collection describing the expected types of each argument. It then passes the active variadic argument list and the prepared collection to this module.

Expected behavior:

- each described argument is fetched in order,
- values are stored into the matching positions in the destination collection,
- the function returns success when all arguments are fetched correctly.

Acceptance test:

- prepare an `arguments` collection with multiple argument descriptors of differing scalar/pointer categories,
- invoke the Rust implementation with a matching argument source,
- verify that all destination entries contain the expected values,
- verify success status is returned.

### Scenario 2: Handle repeated use across different argument type mixes

The surrounding formatting machinery may invoke this module for different format strings that yield different argument type layouts.

Expected behavior:

- the module uses only the supplied argument metadata,
- it does not assume a fixed set or order of argument kinds beyond what the collection describes,
- it correctly fetches values for each supported descriptor category represented by the source behavior.

Acceptance test:

- run multiple cases with different prepared argument collections,
- include integer, character, string/pointer, and floating-point style categories as supported by the original module behavior,
- verify that each case populates stored values according to its own descriptors.

### Scenario 3: Report failure when an argument type cannot be processed

If the prepared metadata contains an argument classification the module cannot validly fetch or store under its supported behavior, the function must indicate failure rather than silently succeeding.

Expected behavior:

- the module returns a failure status,
- partially processed state is not treated as full success.

Acceptance test:

- construct a destination collection containing an unsupported or invalid argument type marker as permitted by test scaffolding,
- invoke the Rust implementation,
- verify that failure status is returned.

### Scenario 4: Preserve argument order semantics

Variadic argument consumption is order-dependent. The module must consume arguments in the sequence defined by the destination collection.

Expected behavior:

- fetched values correspond to descriptor order,
- no descriptor is skipped or fetched out of order.

Acceptance test:

- prepare descriptors for several adjacent arguments of distinct values,
- invoke the function,
- verify that each stored value matches the source position intended for that descriptor index.

## Requirements

### Functional Requirements

#### FR-1: Metadata-driven fetching

The module shall fetch argument values based solely on the argument type information already present in the destination `arguments` collection.

Traceability: `gnu/printf-args.c`, `PRINTF_FETCHARGS`

#### FR-2: Ordered traversal

The module shall process the argument entries in collection order and consume the input argument list accordingly.

Traceability: `gnu/printf-args.c`, `PRINTF_FETCHARGS`

#### FR-3: Type-correct retrieval

For each argument entry, the module shall retrieve the value from the input argument list using the retrieval behavior corresponding to that entry’s declared type category.

Traceability: `gnu/printf-args.c`, `PRINTF_FETCHARGS`

#### FR-4: Destination storage

The module shall store each fetched value into the corresponding slot of the destination `arguments` collection.

Traceability: `gnu/printf-args.c`, `PRINTF_FETCHARGS`

#### FR-5: Full-collection processing

When all entries are valid and fetchable, the module shall process the complete destination collection and return success.

Traceability: `gnu/printf-args.c`, `PRINTF_FETCHARGS`

#### FR-6: Failure signaling

When the module encounters an argument entry whose type classification cannot be handled under the module’s supported behavior, it shall return failure status.

Traceability: `gnu/printf-args.c`, `PRINTF_FETCHARGS`

#### FR-7: No format parsing responsibility

The module shall not derive argument types from a format string; it shall rely on the caller-provided `arguments` metadata.

Traceability: `gnu/printf-args.c`, `PRINTF_FETCHARGS`

### Key Entities

#### `arguments`

A destination collection that holds:

- the number of argument entries to process,
- per-entry type metadata describing how each argument must be fetched,
- per-entry storage for the fetched argument value.

Relationship:
- `PRINTF_FETCHARGS` reads type metadata from `arguments` and writes fetched values back into the same collection.

#### Variadic argument input

An ordered input argument source supplied to the module.

Relationship:
- values are consumed from this source in the same order as the entries described by `arguments`.

#### Argument entry metadata/value slot pairing

Each logical argument entry consists of:

- a type/classification used to determine fetch behavior,
- a storage location for the fetched result.

Relationship:
- each metadata item controls one fetch operation and one destination write.

## Success Criteria

### SC-1: Correct population for valid descriptors

Given an `arguments` collection whose entries all use supported type classifications and a matching input argument sequence, the Rust implementation populates every entry with the correct value and returns success.

Traceability: `gnu/printf-args.c`, `PRINTF_FETCHARGS`

### SC-2: Order preservation

For any test case with distinguishable argument values, stored values correspond exactly to descriptor order in the destination collection.

Traceability: `gnu/printf-args.c`, `PRINTF_FETCHARGS`

### SC-3: Type-directed behavior

For each argument category supported by the source module, the Rust implementation fetches and stores the value using behavior consistent with that category rather than treating all entries uniformly.

Traceability: `gnu/printf-args.c`, `PRINTF_FETCHARGS`

### SC-4: Failure on unsupported classification

When provided an unsupported or invalid argument classification in the destination metadata, the Rust implementation returns failure status.

Traceability: `gnu/printf-args.c`, `PRINTF_FETCHARGS`

### SC-5: No hidden parsing dependency

The Rust implementation can operate using only the prepared `arguments` collection and the supplied argument list, without requiring format-string parsing inside this module.

Traceability: `gnu/printf-args.c`, `PRINTF_FETCHARGS`

## Constraints

- The Rust port must preserve the module’s functional boundary as an argument-fetching stage, not a format-analysis stage.
- The specification is limited to behavior evidenced by `gnu/printf-args.c` and its exported function role.
- No additional public capabilities are required beyond reproducing the source module’s fetch-and-store behavior.

## Notes for Validation

Validation should center on prepared `arguments` fixtures that encode expected argument types and destination slots, then compare stored results after invocation. Coverage should include:

- multiple valid type mixes,
- ordered multi-argument cases,
- unsupported-type failure cases.