# spec.md

## Title

Rust Port Functional Specification: `module_include`

## Metadata

- **Project**: `cat`
- **Module**: `module_include`
- **Category**: `module`
- **Source basis**: `include/safe-read.c`
- **Primary function evidence**: `safe_rw (int fd, void const *buf, size_t count) -> size_t`
- **Rust branch**: `001-module_include-rust-port`
- **Generation date**: `2026-06-06`

## Overview

This module provides a bounded file-descriptor I/O operation wrapper for transferring a caller-specified byte count between a memory buffer and a file descriptor. The Rust rewrite must preserve the module’s functional role as a low-level, reusable helper that performs the requested transfer and returns the amount of data transferred as a byte count.

The specification is limited to behavior evidenced by the analyzed module content. No additional public capabilities are introduced beyond this bounded I/O helper role.

## Feature Specification

### Summary

The Rust version must implement the module’s core functionality as a helper for safe byte-counted file-descriptor I/O. Its responsibility is to accept:

- a file descriptor,
- a buffer reference,
- and a requested transfer size,

and perform the corresponding transfer operation while reporting the number of bytes actually transferred.

### Functional Scope

The Rust port must provide functionality equivalent in scope to the source module:

- support a file-descriptor-based transfer operation,
- operate on caller-provided memory,
- use an explicit byte count supplied by the caller,
- return the completed transfer length as a `size_t`-equivalent value in Rust.

### Behavioral Boundaries

The Rust rewrite must preserve these observable boundaries:

- The operation is count-bounded by the caller-provided `count`.
- The function reports completion in bytes, not records or lines.
- The module acts as a low-level utility and does not define higher-level text, parsing, formatting, or buffering behavior.
- The module does not introduce new externally visible configuration or policy surfaces not evidenced by the source analysis.

## User Scenarios & Testing

### Scenario 1: Bounded descriptor transfer with nonzero byte count

A caller has an open file descriptor and a valid memory buffer and requests transfer of `N` bytes.

**Expected support in Rust:**

- The module accepts the descriptor, buffer, and `N`.
- The operation attempts the transfer for at most `N` bytes.
- The module returns a byte count representing the amount transferred.

**Test focus:**

- Verify the function accepts valid inputs.
- Verify the returned value is a byte count.
- Verify the returned count does not exceed the requested count.

### Scenario 2: Zero-length transfer request

A caller invokes the helper with a transfer count of `0`.

**Expected support in Rust:**

- The operation remains well-defined for a zero-length request.
- The returned byte count reflects that no bytes were transferred.

**Test focus:**

- Verify zero-length input is accepted.
- Verify the returned count is `0`.

### Scenario 3: Partial transfer outcome

A caller requests a transfer size larger than what is completed in a single operation.

**Expected support in Rust:**

- The module reports the actual completed byte count rather than an invented or expanded value.

**Test focus:**

- Verify the returned count matches the actual completed transfer amount.
- Verify the return value remains less than or equal to the requested count.

### Scenario 4: Use as a reusable low-level helper

Another module in the Rust port uses this helper as a descriptor-I/O primitive rather than implementing direct raw transfer logic itself.

**Expected support in Rust:**

- The module remains suitable for internal reuse as a small, focused utility.
- Its interface stays limited to descriptor, buffer, and count driven transfer behavior.

**Test focus:**

- Verify the helper can be called from another internal module boundary.
- Verify no extra setup, state objects, or configuration are required beyond the evidenced inputs.

## Requirements

### Functional Requirements

- **FR-1**: The module shall provide one bounded file-descriptor I/O helper corresponding to the source module’s `safe_rw` functionality evidenced in `include/safe-read.c`.
- **FR-2**: The helper shall accept a file descriptor input, a caller-provided buffer input, and a caller-provided byte count.
- **FR-3**: The helper shall perform a transfer operation limited by the provided byte count.
- **FR-4**: The helper shall return the number of bytes actually transferred as an unsigned byte-count value equivalent in role to C `size_t`.
- **FR-5**: The helper shall support zero-byte requests and report a zero-byte result for such requests.
- **FR-6**: The helper shall not report a transferred byte count greater than the caller-provided count.
- **FR-7**: The module shall remain a low-level utility concerned only with byte transfer by file descriptor and shall not add unrelated behaviors such as parsing, formatting, persistent state management, or higher-level stream abstractions.

### Key Entities

- **File descriptor**: Integer-like handle identifying the target of the transfer operation.
- **Memory buffer**: Caller-supplied memory region used as the source or destination of transferred bytes.
- **Byte count**: Requested maximum number of bytes to transfer in one call.
- **Transferred byte count**: Returned result describing the actual completed transfer size.

### Entity Relationships

- The helper operates on a **file descriptor** and a **memory buffer** under a caller-specified **byte count** limit.
- The helper returns a **transferred byte count** that corresponds to the completed operation for that descriptor-buffer-count combination.
- The **transferred byte count** must be bounded by the input **byte count**.

## Success Criteria

- **SC-1**: The Rust module exposes functionality traceable to `safe_rw` in `include/safe-read.c` and no broader public functional surface is required for this module.
- **SC-2**: For valid test cases with requested transfer size `N`, the Rust implementation returns a byte count `R` such that `0 <= R <= N`.
- **SC-3**: For a zero-length request, the Rust implementation returns `0`.
- **SC-4**: In tests exercising partial completion behavior, the Rust implementation reports the actual completed byte count rather than the full requested count when fewer bytes are transferred.
- **SC-5**: The Rust port can be used as an internal low-level helper by other project code without requiring additional module state or configuration beyond the evidenced call inputs.
- **SC-6**: The rewritten module stays within the evidenced functional boundary of descriptor-based bounded byte transfer and does not introduce unrelated user-visible features.