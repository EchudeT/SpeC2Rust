# spec.md

## Title

Rust Functional Specification for `module_gnu_msvc-nothrow.c_37`

## Metadata

- **Project**: `cflow-new`
- **Module**: `module_gnu_msvc-nothrow.c_37`
- **Category**: `module_cluster`
- **Source file**: `gnu/msvc-nothrow.c`
- **Rust branch**: `043-module_gnu_msvc_nothrow.c_37-rust-port`
- **Generation date**: `2026-06-11`

## Overview

This module provides a single functional boundary: obtaining an OS-level file handle corresponding to a C file descriptor through a nothrow interface.

The Rust rewrite must preserve that boundary and behavior: given an integer file descriptor, it must attempt to obtain the corresponding OS file handle value and return it using an integer handle type compatible with the source module’s contract. The Rust version must remain limited to this conversion responsibility and must not introduce unrelated file-descriptor management features.

## Feature Specification

### Feature: Nothrow retrieval of OS file handle from file descriptor

The module exposes functionality equivalent to `_gl_nothrow_get_osfhandle(int fd) -> intptr_t`.

The Rust version must implement behavior matching this functional role:

- Accept an integer file descriptor as input.
- Attempt to obtain the OS file handle associated with that descriptor.
- Return the handle as an integer-sized value.
- Operate through a nothrow interface boundary: callers must receive the result through normal return semantics rather than exception-style failure.

### Functional Scope

Included in scope:

- File-descriptor to OS-handle lookup.
- Integer return of the looked-up handle value.
- Preservation of the module’s narrow single-purpose behavior.

Out of scope:

- Opening or closing files.
- Ownership transfer of descriptors or handles.
- Descriptor validation beyond what is required to perform the lookup.
- Additional public APIs beyond the evidenced function boundary.
- Any guarantees not evidenced by the source module, including thread-safety or recovery behavior.

## User Scenarios & Testing

### Scenario 1: Retrieve an OS handle for a valid descriptor

A caller has a valid file descriptor originating from normal file or stream usage and needs the corresponding OS-level handle.

**Expected behavior**:
- The module accepts the descriptor.
- The module returns an integer handle result corresponding to that descriptor.

**Testing**:
- Create or obtain a valid file descriptor.
- Invoke the Rust port with that descriptor.
- Verify that a handle-shaped integer result is returned and that repeated lookup for the same still-valid descriptor is consistent with the underlying platform behavior.

### Scenario 2: Call the module with an invalid descriptor

A caller passes a descriptor value that is not currently valid for OS-handle lookup.

**Expected behavior**:
- The module completes through its normal return path.
- The module does not surface failure through exceptions/panics as part of the intended interface boundary.
- The returned value reflects the underlying lookup outcome for an invalid descriptor.

**Testing**:
- Invoke the Rust port with a clearly invalid descriptor such as a negative or closed descriptor, where meaningful on the target platform.
- Verify the call returns normally.
- Verify the returned value matches the platform outcome for failed lookup as defined by the ported behavior.

### Scenario 3: Use the module as a compatibility shim

A caller depends on a compatibility-layer function that exposes OS-handle lookup without changing surrounding code structure.

**Expected behavior**:
- The Rust version serves as a drop-in functional replacement for this module’s narrow contract.
- The caller can perform the same descriptor-to-handle conversion step without requiring additional state objects or setup.

**Testing**:
- Integrate the Rust function at the same call site role as the original module.
- Confirm that no extra configuration, object construction, or ownership protocol is required.

## Requirements

### Functional Requirements

- **FR-1**: The Rust module shall provide functionality equivalent to `_gl_nothrow_get_osfhandle` for converting an input file descriptor (`int`) into an OS file handle value (`intptr_t`-equivalent).
  **Traceability**: `gnu/msvc-nothrow.c`, `_gl_nothrow_get_osfhandle`.

- **FR-2**: The Rust module shall accept a single file descriptor input and produce its result directly as a return value, preserving the source module’s single-call lookup behavior.
  **Traceability**: `gnu/msvc-nothrow.c`, `_gl_nothrow_get_osfhandle`.

- **FR-3**: The Rust module shall preserve the nothrow functional boundary of the source module, meaning callers interact with the API through normal return semantics rather than exception-style signaling.
  **Traceability**: `gnu/msvc-nothrow.c`, `_gl_nothrow_get_osfhandle`.

- **FR-4**: The Rust module shall remain limited to OS-handle retrieval behavior and shall not add descriptor lifecycle operations such as open, close, duplicate, or ownership transfer.
  **Traceability**: module scope evidenced by `gnu/msvc-nothrow.c`, `_gl_nothrow_get_osfhandle`.

### Key Entities

- **File descriptor**
  - An integer input identifying an open file-like resource in the C-runtime descriptor domain.
  - Relationship: serves as the sole input to the module function.

- **OS file handle**
  - An integer-sized return value representing the operating-system handle associated with the descriptor.
  - Relationship: derived from the file descriptor by the module’s lookup operation.

- **Nothrow lookup function**
  - The module’s sole functional entity, performing the mapping from file descriptor to OS file handle.
  - Relationship: consumes a file descriptor and returns the associated handle value.

## Success Criteria

- **SC-1**: The Rust rewrite exposes one functional capability matching the source module’s evidenced responsibility: file-descriptor to OS-handle retrieval.
  **Traceability**: `gnu/msvc-nothrow.c`, `_gl_nothrow_get_osfhandle`.

- **SC-2**: For valid descriptors used in test scenarios, the Rust module returns an integer-sized handle result through the function return path.
  **Traceability**: `gnu/msvc-nothrow.c`, `_gl_nothrow_get_osfhandle`.

- **SC-3**: For invalid-descriptor test scenarios, the Rust module completes without exception-style failure and returns the platform-result value through the same return channel.
  **Traceability**: `gnu/msvc-nothrow.c`, `_gl_nothrow_get_osfhandle`.

- **SC-4**: The Rust port introduces no additional public functionality beyond the evidenced single-purpose lookup interface of the source module.
  **Traceability**: module scope evidenced by `gnu/msvc-nothrow.c`, `_gl_nothrow_get_osfhandle`.