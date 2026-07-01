# spec.md

## Overview

This module provides a compatibility implementation of `copy_file_range` for environments where the function may be unavailable from the standard C library. Its functional role is to expose a `copy_file_range` entry point that attempts to perform a byte-range copy between two file descriptors while honoring optional input and output offsets, a requested byte count, and flags.

The Rust rewrite must preserve the same functional boundary:

- provide the module-level behavior of `copy_file_range`
- accept source and destination file descriptors
- support optional source and destination offsets
- accept a requested copy length and flags
- return the number of bytes copied on success or an error indication on failure
- preserve behavior needed by callers in the main program cluster that rely on this function as a system-capability wrapper

This specification is limited to behavior evidenced by `copy-file-range.c` and the exported function it defines.

## Scope

In scope:

- range-based copying between file descriptors through the module’s `copy_file_range` behavior
- handling of optional offsets for input and output positions
- accepting and forwarding flags as part of the operation contract
- reporting success or failure through the function result

Out of scope:

- defining any broader file-copy policy outside this function
- introducing additional public APIs
- promising behavior unrelated to the `copy_file_range` compatibility function

## Feature Specification

### Feature: Compatibility file-range copy operation

The module defines a function-compatible file-range copy operation for the main program to use when copying data between open file descriptors.

The Rust version must implement behavior equivalent to the C module’s exported function contract:

- The caller supplies:
  - an input file descriptor
  - an optional pointer/reference to an input offset
  - an output file descriptor
  - an optional pointer/reference to an output offset
  - a requested length in bytes
  - flags
- The module attempts to copy up to the requested number of bytes from the input descriptor to the output descriptor.
- The module returns a signed byte-count result:
  - nonnegative for successful copying
  - negative/error result semantics consistent with the surrounding platform contract when the copy cannot be completed
- Optional offsets are part of the operation interface and must be supported as caller-controlled positions distinct from implicit descriptor position where applicable.
- Flags are accepted as part of the interface and must participate in the operation contract even if the underlying environment constrains which flag values are valid.

### Supported behavior boundaries

The Rust rewrite must preserve these observable boundaries:

1. **Descriptor-based copy**
   - Operates on already-open file descriptors supplied by the caller.

2. **Range length request**
   - Uses the caller’s requested byte count as the maximum intended copy amount.

3. **Optional offset participation**
   - Supports invocation with or without explicit input/output offsets.

4. **System/environment dependent execution**
   - Behaves as a compatibility layer whose success may depend on host kernel or platform support.

5. **Error propagation**
   - Exposes failure through the function result rather than hiding or transforming it into unrelated module-level behavior.

## User Scenarios & Testing

### Scenario 1: Copy bytes between two open file descriptors

A caller has an open source descriptor and an open destination descriptor and requests that up to `length` bytes be copied.

Expected support:

- the operation can be invoked with both descriptors
- success returns a nonnegative byte count
- the returned count does not exceed the requested length

Testing focus:

- call with valid readable input and writable output descriptors
- verify that a successful result is nonnegative
- verify that copied output content length matches the returned byte count

### Scenario 2: Copy using explicit source and destination offsets

A caller wants copying to occur from specified positions rather than relying only on current descriptor positions.

Expected support:

- the function accepts explicit input and output offsets
- the operation uses those offsets as part of the copy request
- successful completion reports the number of bytes copied

Testing focus:

- prepare files with known content
- invoke with non-null source and destination offsets
- verify that data appears at the expected destination range
- verify that the returned byte count is consistent with the copied region

### Scenario 3: Copy with null offsets

A caller relies on descriptor-associated file positions and passes no explicit offsets.

Expected support:

- the function accepts absent offsets
- the copy still executes under the same result contract

Testing focus:

- invoke with null/absent input and output offsets
- verify successful copying for valid descriptors
- verify returned count does not exceed requested length

### Scenario 4: Unsupported or invalid operation reports failure

A caller invokes the function in an environment or with arguments such that the copy cannot be performed.

Expected support:

- the function reports failure through its result contract
- the Rust rewrite does not invent fallback behavior outside the evidenced module boundary

Testing focus:

- invoke with invalid file descriptors or invalid usage expected to fail
- verify that the result indicates error
- verify no false success is reported

### Scenario 5: Flags are part of the call contract

A caller passes a flags value as part of the operation.

Expected support:

- the function accepts a flags argument on every call
- behavior remains consistent with platform-supported `copy_file_range` semantics

Testing focus:

- invoke with supported/default flag usage
- verify the call path accepts flags and returns success or failure according to platform behavior

## Requirements

### Functional Requirements

#### FR-1: Provide the `copy_file_range` operation
The module shall provide a `copy_file_range` function-compatible operation matching the role defined in `copy-file-range.c`.

Traceability:
- File: `copy-file-range.c`
- Function: `copy_file_range`

#### FR-2: Accept source and destination file descriptors
The operation shall accept an input file descriptor and an output file descriptor as the source and destination of the copy.

Traceability:
- Function signature: `copy_file_range (int infd, ... int outfd, ...)`

#### FR-3: Accept optional input and output offsets
The operation shall accept optional input and output offsets as part of the function contract.

Traceability:
- Function signature: `off_t *pinoff`, `off_t *poutoff`

#### FR-4: Accept a requested copy length
The operation shall accept a byte-count length parameter specifying the requested maximum amount to copy.

Traceability:
- Function signature: `size_t length`

#### FR-5: Accept flags
The operation shall accept a flags parameter as part of the function contract.

Traceability:
- Function signature: `unsigned int flags`

#### FR-6: Return copied byte count or failure indication
The operation shall return a signed size result indicating either the amount copied or failure.

Traceability:
- Function signature return type: `ssize_t`

#### FR-7: Preserve compatibility-wrapper behavior for the main program
The Rust rewrite shall preserve this module’s role as a compatibility-facing file-range copy facility used by the main cluster, without adding unrelated public behavior.

Traceability:
- File: `copy-file-range.c`
- Function: `copy_file_range`

### Key Entities

#### Entity: File descriptor
Represents an already-open source or destination file object used by the copy operation.

Relationships:
- one input file descriptor supplies bytes
- one output file descriptor receives bytes
- both are required inputs to the operation

Traceability:
- Function parameters: `int infd`, `int outfd`

#### Entity: File offset
Represents an optional position associated with the input or output side of the copy request.

Relationships:
- the input offset identifies where reading should occur when provided
- the output offset identifies where writing should occur when provided
- each offset is optional and participates only when supplied by the caller

Traceability:
- Function parameters: `off_t *pinoff`, `off_t *poutoff`

#### Entity: Copy length
Represents the requested maximum number of bytes for a single copy operation.

Relationships:
- constrains the copy request issued by the operation
- is provided by the caller for each invocation

Traceability:
- Function parameter: `size_t length`

#### Entity: Flags
Represents call-time control bits associated with the copy request.

Relationships:
- accompanies every operation invocation
- affects operation validity and outcome according to supported semantics

Traceability:
- Function parameter: `unsigned int flags`

#### Entity: System identification data
The module references `struct utsname`, indicating dependence on host system identification during compatibility handling.

Relationships:
- used only as environment-related support data within the module boundary
- not a primary caller-facing entity of the API contract

Traceability:
- Type reference: `struct utsname`

## Success Criteria

### SC-1: Function contract parity
The Rust module exposes a `copy_file_range` behaviorally equivalent operation with the same caller-visible inputs: input descriptor, optional input offset, output descriptor, optional output offset, length, and flags.

Traceability:
- Function: `copy_file_range`

### SC-2: Successful copy result is measurable
For valid source and destination descriptors in a supported environment, the operation returns a nonnegative result and the number of bytes observed in the destination for the requested region matches the returned value.

Traceability:
- Function: `copy_file_range`

### SC-3: Length bound is respected
In tests invoking the operation with a requested length `N`, the reported successful byte count is never greater than `N`.

Traceability:
- Function parameter: `size_t length`

### SC-4: Optional offsets are supported
Tests covering both explicit offsets and absent offsets complete using the same operation entry point, with successful cases producing data in the expected destination location.

Traceability:
- Function parameters: `off_t *pinoff`, `off_t *poutoff`

### SC-5: Failure is observable
For invalid descriptors or unsupported execution conditions, the operation reports failure through its return contract rather than reporting false success.

Traceability:
- Function: `copy_file_range`

### SC-6: No unsupported API expansion
The Rust rewrite remains limited to the evidenced module boundary and does not require additional public APIs beyond the `copy_file_range` functionality described here.

Traceability:
- File: `copy-file-range.c`
- Function: `copy_file_range`