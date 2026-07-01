# spec.md

## Title
Functional Specification for `main_root_fadvise.c_22` Rust Port

## Metadata
- Project: `cat`
- Module: `main_root_fadvise.c_22`
- Category: `main_cluster`
- Source file: `fadvise.c`
- Rust branch: `023-main_root_fadvise.c_22-rust-port`
- Generation date: `2026-06-09`

## Overview
This module provides advisory file-access hints for an already opened file or file descriptor. Its role is limited to issuing a file access advice request based on a caller-provided advice value, either directly from a file descriptor or from a `FILE *` stream.

The Rust rewrite must preserve this boundary:

- support advising by file descriptor with caller-supplied offset, length, and advice;
- support advising by file stream using the stream’s underlying file descriptor and a whole-file style range;
- preserve the module’s non-returning, best-effort behavior exposed by the current interfaces.

## Scope
In scope:
- Applying file access advice to an open file descriptor.
- Applying file access advice to an open C stream by resolving its file descriptor.
- Accepting an advice parameter of the existing advice type used by the surrounding project.

Out of scope:
- Opening or closing files.
- Managing file ownership or buffering policy beyond issuing the advisory request.
- Defining new advice kinds beyond those already represented by the existing project advice type.
- Reporting rich error results through new APIs.

## Feature Specification

### Feature: File-descriptor-based advice
The module shall provide functionality that accepts:
- an open file descriptor,
- an offset,
- a length,
- an advice value,

and issues the corresponding file access advice request for that descriptor and range.

This feature is the direct functional role of `fdadvise`.

### Feature: Stream-based advice
The module shall provide functionality that accepts:
- an open file stream,
- an advice value,

and applies file access advice to the file associated with that stream by obtaining the underlying file descriptor and using a full-range style advisory request.

This feature is the functional role of `fadvise`.

### Feature: Best-effort advisory behavior
The module shall behave as an advisory helper rather than a stateful controller. Its exported operations do not return a status value to the caller. The Rust rewrite must preserve this externally observable behavior: callers use the module to request advice, not to receive structured success/failure output.

## User Scenarios & Testing

### Scenario 1: Advise sequential or other caller-selected access for an open descriptor
A higher-level file-processing path already has an open file descriptor and knows the range and advice type it wants to apply. It calls the descriptor-based function once before or during file processing.

Expected support:
- The Rust module accepts the descriptor, offset, length, and advice.
- The Rust module attempts to apply the advisory request without requiring the caller to manage any additional module state.

Test focus:
- Call with a valid descriptor and representative offset/length values.
- Verify the function completes without returning a value.
- Verify the call path accepts all advice values supported by the surrounding project’s advice type.

### Scenario 2: Advise access for an open stream
A caller has a `FILE *` stream rather than a raw descriptor and wants the same advisory behavior without extracting the descriptor itself. It calls the stream-based function with an advice value.

Expected support:
- The Rust module derives the underlying descriptor from the provided stream-equivalent input.
- The Rust module applies advice over the whole-file style range used by the source module.

Test focus:
- Provide an open stream backed by a real file.
- Verify the function completes without returning a value.
- Verify the stream-based path delegates equivalent advisory behavior to the descriptor-based path or its functional equivalent.

### Scenario 3: Module used as a transparent helper in file-reading flow
A higher-level command path uses this module only to hint expected access patterns and continues normal file reading regardless of advisory outcome.

Expected support:
- The Rust module does not impose additional control flow requirements on the caller.
- The caller can proceed with existing file operations after invoking the module.

Test focus:
- Invoke the module before reading from an open file.
- Confirm the surrounding read flow is not blocked by the advisory call interface.
- Confirm no new required error-handling contract is introduced by the Rust port.

## Requirements

### Functional Requirements

#### FR-1: Descriptor advice operation
The Rust module shall implement a functionally equivalent operation to `fdadvise` that accepts an open file descriptor, an offset, a length, and an advice value, and issues file access advice for that range on that descriptor.

Traceability:
- `fadvise.c`
- `fdadvise`

#### FR-2: Stream advice operation
The Rust module shall implement a functionally equivalent operation to `fadvise` that accepts an open file stream and an advice value, resolves the stream’s underlying file descriptor, and applies file access advice using that descriptor.

Traceability:
- `fadvise.c`
- `fadvise`

#### FR-3: Whole-range behavior for stream-based advice
For the stream-based operation, the Rust module shall preserve the source module’s whole-range style behavior by applying advice using the same effective range semantics as the C module’s stream wrapper.

Traceability:
- `fadvise.c`
- `fadvise`
- `fdadvise`

#### FR-4: Void-style caller contract
The Rust module shall preserve the source module’s observable contract that these advisory operations do not return a caller-visible status value through their primary interface.

Traceability:
- `fadvise.c`
- `fdadvise`
- `fadvise`

#### FR-5: No module-managed persistent state
The Rust module shall remain a stateless helper with behavior driven only by the call inputs provided for each invocation.

Traceability:
- `fadvise.c`
- `fdadvise`
- `fadvise`

### Key Entities

#### `file descriptor`
An already opened file handle represented as an integer-like OS descriptor. It is the direct target of the descriptor-based advisory operation.

Traceability:
- `fdadvise`

#### `file stream`
An already opened C stream handle that can be mapped to an underlying file descriptor. It is the input to the stream-based advisory operation.

Traceability:
- `fadvise`

#### `offset`
The starting position for advisory application in the descriptor-based operation.

Traceability:
- `fdadvise`

#### `length`
The advisory range length for the descriptor-based operation.

Traceability:
- `fdadvise`

#### `fadvice_t`
The project-defined advice classification supplied by callers to both operations. It determines which advisory mode is requested.

Traceability:
- `fdadvise`
- `fadvise`

#### Relationship summary
- A `file stream` is converted to a `file descriptor` for stream-based advice.
- `offset`, `length`, and `fadvice_t` parameterize descriptor-based advice.
- `fadvice_t` also parameterizes stream-based advice.

## Success Criteria

### SC-1: Descriptor interface parity
A Rust caller can invoke the descriptor-based operation with descriptor, offset, length, and advice inputs corresponding to the C module’s `fdadvise` interface.

Traceability:
- `fdadvise`

### SC-2: Stream interface parity
A Rust caller can invoke the stream-based operation with a stream-equivalent input and an advice value corresponding to the C module’s `fadvise` interface.

Traceability:
- `fadvise`

### SC-3: Preserved stream-to-descriptor behavior
The stream-based Rust operation uses the stream’s underlying file descriptor and preserves the source module’s effective whole-range advisory behavior.

Traceability:
- `fadvise`
- `fdadvise`

### SC-4: No required status handling by callers
The Rust port preserves the source module’s void-style usage pattern so that callers are not required to consume a returned success/error status from these advisory operations.

Traceability:
- `fdadvise`
- `fadvise`

### SC-5: Stateless operation
Repeated invocations of the Rust module depend only on the arguments provided to each call and do not require prior initialization or maintain persistent module state.

Traceability:
- `fdadvise`
- `fadvise`