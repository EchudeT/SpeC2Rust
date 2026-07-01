# spec.md

## Overview

- **Project**: `cat`
- **Module**: `main_root_copy-file-range.c_21`
- **Category**: `main_cluster`
- **Source basis**: `copy-file-range.c`
- **Rust target branch**: `022-main_root_copy_file_range.c_21-rust-port`
- **Generation date**: `2026-06-07`

## Feature Specification

This module provides a compatibility-layer implementation of `copy_file_range` for systems where a direct, usable interface may not be available through the normal build environment.

The Rust rewrite must implement the same functional boundary:

- Provide a functionally equivalent `copy_file_range` operation with the same observable role:
  - copy up to a requested byte count from an input file descriptor to an output file descriptor,
  - optionally use caller-provided input and output offsets,
  - accept a flags argument,
  - return a signed byte-count result or failure indication consistent with the underlying platform behavior.
- Preserve the module’s purpose as a thin system-facing wrapper rather than introducing higher-level copy semantics.
- Preserve runtime behavior needed for platform compatibility, including behavior that depends on system support detection.

This module does not define a broader file-copy subsystem. Its scope is limited to exposing this single file-range copy capability through the module boundary evidenced by the source.

## User Scenarios & Testing

### Scenario 1: Copy between two open file descriptors
A caller has already opened a readable input file descriptor and a writable output file descriptor and needs to request that a range of bytes be copied from one to the other.

The Rust version must support:
- passing the two file descriptors,
- requesting a specific maximum number of bytes,
- receiving the number of bytes actually copied or an error result.

### Scenario 2: Copy using explicit source and destination offsets
A caller wants to copy bytes from a specific source offset to a specific destination offset without relying solely on the current file positions.

The Rust version must support:
- accepting optional mutable offsets for input and output,
- using those offsets for the copy request,
- reflecting post-call offset advancement according to the underlying operation semantics.

### Scenario 3: Operate on systems with varying kernel support
A caller uses the module on systems where `copy_file_range` support may vary by environment or kernel behavior.

The Rust version must support:
- attempting the file-range copy in a way compatible with platform support detection used by this module’s source basis,
- surfacing failure when the operation is unavailable or rejected by the platform,
- avoiding invention of fallback copy loops not evidenced by this module.

### Scenario 4: Zero or partial transfer outcomes
A caller requests a transfer length that may produce a short copy or no copy, depending on end-of-file or platform behavior.

The Rust version must support:
- returning the actual result reported by the underlying operation,
- allowing partial progress to be observable through the return value,
- not converting partial-copy outcomes into all-or-nothing behavior.

### Testing implications
The Rust port should be tested with:
- valid input/output file descriptors and a nonzero length,
- explicit input and output offsets,
- a zero-length or no-progress case where supported by the platform,
- platform/error cases where the operation is unsupported or fails,
- verification that return values and offset updates match expected system semantics.

## Requirements

### Functional Requirements

#### FR-1: File-range copy interface
The module shall provide a `copy_file_range` capability matching the source module’s functional role: copying data between an input file descriptor and an output file descriptor for a caller-specified maximum length.

**Traceability**: `copy-file-range.c`, `copy_file_range`

#### FR-2: Input and output offsets
The module shall accept optional input and output offsets and use them as part of the copy request when supplied.

**Traceability**: `copy-file-range.c`, `copy_file_range`

#### FR-3: Length-controlled transfer
The module shall accept a byte-length parameter and request copying of up to that number of bytes.

**Traceability**: `copy-file-range.c`, `copy_file_range`

#### FR-4: Flags parameter handling
The module shall accept a flags argument as part of the operation interface and preserve its role in the underlying copy request semantics.

**Traceability**: `copy-file-range.c`, `copy_file_range`

#### FR-5: Result reporting
The module shall report the operation result as a signed size value that distinguishes successful byte-count results from failure.

**Traceability**: `copy-file-range.c`, `copy_file_range`

#### FR-6: Platform compatibility behavior
The module shall preserve the compatibility purpose evidenced by the source file, including behavior tied to determining whether the platform/kernel supports the operation.

**Traceability**: `copy-file-range.c`, `copy_file_range`, `struct utsname`

### Key Entities

#### `copy_file_range`
The module’s central operation. It links:
- an input file descriptor,
- an optional input offset,
- an output file descriptor,
- an optional output offset,
- a requested transfer length,
- operation flags,
- and a signed result indicating copied byte count or failure.

**Relationship**: This function is the sole functional boundary exposed by the analyzed module.

#### `struct utsname`
A system identity structure referenced by the module in support of compatibility behavior related to environment or kernel characteristics.

**Relationship**: It supports platform-sensitive behavior within the module but is not itself the module’s primary public operation.

## Success Criteria

### SC-1: Functional equivalence
For valid file descriptors on a supported platform, the Rust version performs a file-range copy request and returns a signed result with the same functional meaning as the source module.

**Traceability**: `copy-file-range.c`, `copy_file_range`

### SC-2: Offset-aware behavior
When caller-provided input and/or output offsets are supplied, the Rust version uses them and preserves observable offset-update behavior consistent with the source module’s operation.

**Traceability**: `copy-file-range.c`, `copy_file_range`

### SC-3: Accurate transfer reporting
When the underlying operation copies fewer bytes than requested, including zero bytes, the Rust version reports that outcome directly rather than converting it into a synthetic success/failure convention.

**Traceability**: `copy-file-range.c`, `copy_file_range`

### SC-4: Error propagation
When the underlying platform rejects or does not support the operation, the Rust version surfaces failure consistent with the module’s compatibility role and does not substitute unsupported higher-level fallback behavior.

**Traceability**: `copy-file-range.c`, `copy_file_range`, `struct utsname`

### SC-5: Scope preservation
The Rust rewrite remains limited to the module’s evidenced boundary: a compatibility-oriented `copy_file_range` operation, without adding unrelated file-copy APIs or expanded module responsibilities.

**Traceability**: `copy-file-range.c`, `copy_file_range`