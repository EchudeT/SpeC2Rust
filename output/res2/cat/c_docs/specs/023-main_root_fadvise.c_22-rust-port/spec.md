# spec.md

## Title
Rust Functional Specification for `main_root_fadvise.c_22`

## Metadata
- Project: `cat`
- Module: `main_root_fadvise.c_22`
- Category: `main_cluster`
- Source file: `fadvise.c`
- Rust branch: `023-main_root_fadvise.c_22-rust-port`
- Generation date: `2026-06-07`

## Overview
This module provides a small advisory I/O interface for file data access patterns. It exposes one operation that applies advisory access information to a file descriptor over a specified byte range, and one convenience operation that applies advisory access information to an already opened C stream.

The Rust rewrite must preserve this module’s functional boundary: it issues advisory file access hints for an open file object and does not take ownership of opening, closing, buffering policy selection, or data transfer.

## Feature Specification

### Summary
The module supplies:
- an advisory operation on a raw file descriptor with caller-provided range parameters
- an advisory operation on a file stream that targets the underlying file descriptor

### Required behavior
The Rust version must implement the following behavior evidenced by the source module:
1. Accept an open file descriptor, a byte offset, a byte length, and an advice value, and apply that advice to the referenced file data range.
2. Accept an open file stream and an advice value, obtain the stream’s underlying file descriptor, and apply the advice to the whole file range represented by offset `0` and length `0` semantics used by the original module.
3. Treat this functionality as advisory only: the operation provides a hint to the operating environment and does not change file contents.
4. Preserve the wrapper relationship between the stream-based operation and the descriptor-based operation.

### Non-goals
The Rust version must not introduce unevidenced capabilities such as:
- new public advisory modes beyond the existing advice type used by the module
- file opening or closing APIs
- data reading or writing behavior
- retry, recovery, or caching frameworks
- thread-safety or async guarantees

## User Scenarios & Testing

### Scenario 1: Advise access pattern for an open file descriptor
A caller already has a valid open file descriptor and knows the relevant byte range. The caller invokes the descriptor-based advisory function with:
- the file descriptor
- starting offset
- length
- advice value

Expected result:
- the advisory request is issued for that descriptor and range
- no file content is modified by this module
- the call is usable as a side-effecting hint operation

### Scenario 2: Advise access pattern for an open stream
A caller already has a valid open file stream and wants to apply an access hint without manually extracting the descriptor. The caller invokes the stream-based advisory function with:
- the stream handle
- advice value

Expected result:
- the module derives the underlying file descriptor from the stream
- the module applies the same advisory mechanism used by the descriptor-based function
- the request uses the whole-file convention represented by zero offset and zero length in the original module behavior

### Scenario 3: Use as a helper inside larger file-processing code
A higher-level file-processing path wants to express expected access behavior before or during its own processing. It uses this module only to communicate the hint and performs all actual I/O elsewhere.

Expected result:
- the module remains limited to advisory signaling
- higher-level code can call it without changing ownership or lifecycle of the file object

### Testing guidance
The Rust version must support tests that verify:
1. Calling the descriptor-based operation with a valid open file descriptor completes without altering file contents.
2. Calling the stream-based operation on an open stream routes advisory behavior through the underlying descriptor path.
3. The stream-based operation uses offset `0` and length `0` semantics when delegating.
4. The module can be invoked from code that separately manages file opening, reading, writing, and closing.

## Requirements

### Functional Requirements
- **FR-1**: The module shall provide a function corresponding to `fdadvise` that accepts an open file descriptor, an offset, a length, and an advice value, and issues an advisory access hint for that file range.
  **Traceability:** `fadvise.c`, `fdadvise`.

- **FR-2**: The module shall provide a function corresponding to `fadvise` that accepts an open file stream and an advice value, obtains the underlying file descriptor for that stream, and issues the advisory hint through the descriptor-based functionality.
  **Traceability:** `fadvise.c`, `fadvise`.

- **FR-3**: The stream-based function shall delegate using offset `0` and length `0`, matching the source module’s whole-range invocation convention.
  **Traceability:** `fadvise.c`, `fadvise`, `fdadvise`.

- **FR-4**: The module shall preserve advisory-only behavior; it shall not itself read file data, write file data, resize files, or manage file open/close lifecycle.
  **Traceability:** `fadvise.c`, `fdadvise`, `fadvise`.

### Key Entities
- **File descriptor**: An open OS-level file handle used by the descriptor-based advisory operation.
  **Traceability:** `fdadvise`.

- **File stream**: An open stream object from which the module derives a file descriptor for the stream-based advisory operation.
  **Traceability:** `fadvise`.

- **Advice value (`fadvice_t`)**: The advisory mode parameter passed through the module to identify the intended access pattern.
  **Traceability:** `fdadvise`, `fadvise`.

- **Byte range**: The `(offset, len)` pair used by the descriptor-based operation to identify the advised file region.
  **Traceability:** `fdadvise`.

### Entity Relationships
- A file stream maps to an underlying file descriptor for purposes of this module.
- An advice value is applied either directly to a file descriptor and byte range, or indirectly through a stream after descriptor extraction.
- The stream-based operation is a convenience wrapper over the descriptor-based operation.

## Success Criteria
- **SC-1**: The Rust module exposes functionality equivalent to both source operations: one descriptor-based advisory function and one stream/file-handle-based convenience function.
  **Traceability:** `fadvise.c`, `fdadvise`, `fadvise`.

- **SC-2**: In tests using an open file object, invoking the descriptor-based advisory function does not modify file contents.
  **Traceability:** `fadvise.c`, `fdadvise`.

- **SC-3**: In tests of the stream-based path, the advisory request is delegated through the descriptor-based path using offset `0` and length `0`.
  **Traceability:** `fadvise.c`, `fadvise`, `fdadvise`.

- **SC-4**: The Rust rewrite remains confined to advisory signaling and does not add unrelated file-management or data-transfer responsibilities.
  **Traceability:** `fadvise.c`, `fdadvise`, `fadvise`.