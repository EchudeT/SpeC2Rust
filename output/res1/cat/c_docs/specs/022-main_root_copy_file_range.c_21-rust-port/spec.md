# spec.md

## Title

Functional Specification for `main_root_copy-file-range.c_21`

## Document Control

- **Project**: `cat`
- **Module**: `main_root_copy-file-range.c_21`
- **Category**: `main_cluster`
- **Rust Branch**: `022-main_root_copy_file_range.c_21-rust-port`
- **Source File**: `copy-file-range.c`
- **Generation Date**: 2026-06-06

## Overview

This module provides a single file-range copy capability through the function `copy_file_range`. Its role is to copy up to a requested byte count from one open file descriptor to another open file descriptor, with optional explicit input and output offsets and a flags parameter.

The Rust rewrite must preserve the observable behavior of this module as a file-copy primitive with the same functional boundary: accepting source and destination file descriptors, honoring optional offsets when provided, attempting to copy the requested range length, and reporting the result as a byte-count or failure indicator consistent with the source module’s behavior.

## Feature Specification

### Provided Feature

The module exposes one functional feature:

- Copy a range of bytes from an input file descriptor to an output file descriptor, optionally using caller-supplied offsets for either endpoint.

### Required Rust Behavior

The Rust version must implement the same feature boundary as the source module:

- Accept an input file descriptor and an output file descriptor.
- Accept optional mutable offsets for input and output positions.
- Accept a requested maximum copy length.
- Accept a flags value.
- Perform a file-range copy attempt between the two descriptors.
- Return the number of bytes copied on success, or a failure result equivalent to the C module’s contract for this operation.

### Behavioral Notes

The specification is limited to behavior directly evidenced by the module interface and referenced types:

- The operation is defined in terms of open file descriptors, not path names.
- Offsets are optional and may be passed by pointer.
- The operation may interact with system capability detection related to the running environment, as implied by the module’s reference to `struct utsname`.
- No additional public behavior beyond this copy operation is in scope for the Rust rewrite.

## User Scenarios & Testing

### Scenario 1: Copy using current file positions

A caller has an already-open source descriptor and destination descriptor and requests a copy of a specific number of bytes without supplying explicit offsets.

**Expected support in Rust**:
- The module performs the copy using the descriptors as provided.
- The result reports how many bytes were copied or indicates failure.

**Testing focus**:
- Copy from a readable input descriptor to a writable output descriptor with both offsets absent.
- Verify returned byte count is non-negative on success.
- Verify destination content reflects the copied bytes.

### Scenario 2: Copy using explicit source offset

A caller wants to copy bytes beginning at a specific source position without relying on the descriptor’s current file position.

**Expected support in Rust**:
- The module accepts a mutable source offset reference.
- The copy starts at the caller-provided source offset.

**Testing focus**:
- Provide a source offset and no destination offset.
- Verify data copied corresponds to the specified source range.
- Verify the function reports the number of bytes copied.

### Scenario 3: Copy using explicit destination offset

A caller wants to place copied data at a specific destination position.

**Expected support in Rust**:
- The module accepts a mutable destination offset reference.
- The copy targets the requested destination position.

**Testing focus**:
- Provide a destination offset and no source offset.
- Verify output content appears at the expected destination range.

### Scenario 4: Copy using explicit source and destination offsets

A caller controls both read and write positions for the copy.

**Expected support in Rust**:
- The module accepts both offsets simultaneously.
- The module attempts to copy the requested length between those explicit positions.

**Testing focus**:
- Provide both offsets.
- Verify copied data matches the chosen source range and destination placement.

### Scenario 5: Zero or partial transfer conditions

A caller requests a copy length, but the operation may transfer fewer bytes than requested due to input availability or system behavior.

**Expected support in Rust**:
- The module returns the actual copied byte count rather than assuming the full requested length was transferred.

**Testing focus**:
- Use a source shorter than the requested length.
- Verify the returned count reflects the actual transferred bytes.

### Scenario 6: Invalid descriptor or unsupported operation failure

A caller invokes the module with invalid descriptors or in an environment where the requested file-range copy operation cannot be completed.

**Expected support in Rust**:
- The module reports failure rather than inventing fallback semantics outside the original module boundary.

**Testing focus**:
- Pass an invalid source or destination descriptor.
- Verify failure is surfaced to the caller.
- Where applicable, verify unsupported operation paths also report failure.

## Requirements

### Functional Requirements

#### FR-1: File-range copy interface
The Rust module shall provide a `copy_file_range` operation corresponding to the source module’s functional interface in `copy-file-range.c`, taking:
- an input file descriptor,
- an optional input offset,
- an output file descriptor,
- an optional output offset,
- a maximum length,
- and flags.

**Traceability**: `copy-file-range.c`, `copy_file_range`

#### FR-2: Descriptor-based copying
The operation shall copy data between already-open file descriptors rather than opening, closing, or naming files itself.

**Traceability**: `copy-file-range.c`, `copy_file_range`

#### FR-3: Optional source offset handling
When a source offset is supplied, the operation shall use that explicit source position for the copy attempt.

**Traceability**: `copy-file-range.c`, `copy_file_range`

#### FR-4: Optional destination offset handling
When a destination offset is supplied, the operation shall use that explicit destination position for the copy attempt.

**Traceability**: `copy-file-range.c`, `copy_file_range`

#### FR-5: Length-bounded transfer
The operation shall attempt to copy no more than the caller-specified length.

**Traceability**: `copy-file-range.c`, `copy_file_range`

#### FR-6: Flags-accepting interface
The operation shall accept a flags argument as part of its callable interface and preserve the source module’s functional contract regarding that parameter.

**Traceability**: `copy-file-range.c`, `copy_file_range`

#### FR-7: Result reporting
The operation shall report the result as a signed byte count on success or a failure result consistent with the source function contract.

**Traceability**: `copy-file-range.c`, `copy_file_range`

#### FR-8: Environment-sensitive behavior preservation
The Rust rewrite shall preserve any externally observable behavior tied to environment or platform determination that is part of the source module’s handling of the file-range copy operation, as evidenced by the module’s use of `struct utsname`.

**Traceability**: `copy-file-range.c`, `copy_file_range`, `struct utsname`

### Key Entities

#### `copy_file_range`
The module’s sole functional entity. It defines the module boundary and governs all supported behavior:
- source endpoint: input file descriptor,
- destination endpoint: output file descriptor,
- source positioning: optional input offset,
- destination positioning: optional output offset,
- transfer bound: requested length,
- call modifier: flags,
- outcome: signed byte-count or failure.

**Relationship**:
This function coordinates all module-visible behavior and is the only operation that acts on the other entities listed below.

#### File descriptors
Integer handles representing already-open input and output files or file-like objects.

**Relationship**:
They are the source and destination endpoints consumed by `copy_file_range`.

#### Offsets (`off_t *`)
Optional mutable position references for source and destination copy points.

**Relationship**:
They parameterize where `copy_file_range` reads from and writes to when provided by the caller.

#### `struct utsname`
A referenced system information structure.

**Relationship**:
It indicates that the module may condition behavior on runtime system identity or version information relevant to the copy operation.

## Success Criteria

### Functional Correctness

- The Rust module exposes one copy-range operation matching the source module’s functional boundary and parameter roles.
- Given valid readable and writable file descriptors, the operation can successfully copy bytes from source to destination.
- When explicit source and/or destination offsets are supplied, copied data is taken from and written to the requested positions.
- The operation does not report copying more bytes than the requested length.
- The operation returns the actual byte count transferred on success.

### Failure Behavior

- When invoked with invalid file descriptors, the operation reports failure.
- When the underlying file-range copy cannot be completed in the current environment, the operation reports failure consistent with the source module’s contract.
- The Rust rewrite does not introduce unrelated fallback features or alternate APIs not present in the source module.

### Traceable Acceptance Checks

- **AC-1**: A test copying a known byte range between two valid file descriptors without explicit offsets succeeds and returns a non-negative byte count.
  **Traceability**: `copy-file-range.c`, `copy_file_range`

- **AC-2**: A test using an explicit input offset copies data starting from the specified source position.
  **Traceability**: `copy-file-range.c`, `copy_file_range`

- **AC-3**: A test using an explicit output offset places copied bytes at the specified destination position.
  **Traceability**: `copy-file-range.c`, `copy_file_range`

- **AC-4**: A test requesting more bytes than available verifies that the returned byte count is less than or equal to the requested length and matches actual data transferred.
  **Traceability**: `copy-file-range.c`, `copy_file_range`

- **AC-5**: A test with an invalid descriptor produces a failure result.
  **Traceability**: `copy-file-range.c`, `copy_file_range`

- **AC-6**: Any observable environment-dependent behavior relevant to the operation remains consistent with the source module’s contract.
  **Traceability**: `copy-file-range.c`, `copy_file_range`, `struct utsname`

## Out of Scope

The Rust rewrite specification does not require any capability not evidenced by the source module analysis, including:
- new public APIs beyond the `copy_file_range` operation,
- pathname-based file copying,
- buffering abstractions,
- recursive copying,
- recovery workflows,
- serialization,
- performance claims or benchmarks,
- thread-safety guarantees,
- or compatibility layers beyond the source module’s observable contract.