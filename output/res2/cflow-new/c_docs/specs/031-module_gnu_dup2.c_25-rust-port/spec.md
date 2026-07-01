# spec.md

## Title

Functional Specification for `module_gnu_dup2.c_25` Rust Port

## Metadata

- Project: `cflow-new`
- Module: `module_gnu_dup2.c_25`
- Category: `module_cluster`
- Source file: `gnu/dup2.c`
- Rust branch: `031-module_gnu_dup2.c_25-rust-port`
- Generation date: `2026-06-17`

## Overview

This module provides a replacement `dup2`-style operation that duplicates an existing file descriptor onto a requested target descriptor while preserving the source module’s platform-correct behavior across supported environments.

The Rust rewrite must implement the same observable behavior as the source module’s replacement entry point, including:
- normal descriptor duplication onto a requested descriptor number,
- handling of the case where source and target descriptors are equal,
- propagation of failure for invalid or unsupported descriptors,
- platform-specific correction for environments where the native `dup2` behavior is known to be incomplete or unreliable.

The module’s functional boundary is limited to descriptor duplication behavior and the validation needed to make that behavior conform to the source module.

## Scope

### In Scope

- Replacement behavior corresponding to the source module’s exported duplication routine.
- Validation and handling needed to make duplication semantics match the source module on supported platforms.
- Preservation of success/failure outcomes and target-descriptor result conventions.

### Out of Scope

- General file opening, closing, or ownership management beyond what duplication requires.
- New public APIs beyond the replacement functionality evidenced by the source module.
- Any capability unrelated to file descriptor duplication.

## Feature Specification

### Summary

The Rust module must provide a `dup2`-equivalent replacement that duplicates `fd` onto `desired_fd` and returns the resulting descriptor on success or failure status consistent with the source behavior.

### Required Behavior

1. **Requested-descriptor duplication**
   - When given a valid open source descriptor and a valid target descriptor number, the module must make the target descriptor refer to the same open file description as the source descriptor.
   - If `desired_fd` was already open and differs from `fd`, its previous association must no longer remain active after successful duplication.

2. **Equal source and target handling**
   - When `fd == desired_fd`, the module must not create a second descriptor.
   - In this case, success is allowed only when the descriptor is valid/open; invalid input must fail rather than silently succeed.

3. **Invalid source descriptor handling**
   - If `fd` does not identify a valid open descriptor, the operation must fail.

4. **Invalid target descriptor handling**
   - If `desired_fd` is not an acceptable target descriptor number for the platform operation, the operation must fail.

5. **Platform-correct replacement behavior**
   - The Rust rewrite must preserve the source module’s intent of compensating for platform-specific `dup2` behavior differences, specifically the cases covered by the source helper routines.
   - This includes preserving correctness for environments where:
     - direct no-throw duplication behavior is needed,
     - native Windows-specific duplication behavior requires special handling,
     - descriptor validation for directory-related cases is needed before accepting an equal-descriptor success path.

6. **Return-value convention**
   - On success, the operation must return the target descriptor number.
   - On failure, the operation must report failure in a way equivalent to the source module’s replacement function.

## User Scenarios & Testing

### Scenario 1: Duplicate one valid descriptor onto another descriptor number

A caller has an open descriptor `fd` and requests duplication onto `desired_fd`, where `desired_fd != fd`.

Expected behavior:
- the operation succeeds,
- the returned value equals `desired_fd`,
- subsequent I/O-capability equivalence reflects that `desired_fd` now refers to the same open file description as `fd`.

### Scenario 2: Reuse the same descriptor number

A caller requests duplication with `fd == desired_fd` for a valid open descriptor.

Expected behavior:
- the operation succeeds without changing descriptor identity,
- the returned value equals that descriptor number.

### Scenario 3: Equal descriptors but invalid source

A caller passes `fd == desired_fd`, but the descriptor is not open/valid.

Expected behavior:
- the operation fails,
- the invalid descriptor must not be treated as a successful no-op.

### Scenario 4: Invalid source descriptor with different target

A caller passes a source descriptor that is not valid and a different target descriptor.

Expected behavior:
- the operation fails.

### Scenario 5: Invalid target descriptor number

A caller passes a valid source descriptor but an invalid or unsupported `desired_fd`.

Expected behavior:
- the operation fails.

### Scenario 6: Existing target descriptor is replaced

A caller passes a valid `fd` and a currently open `desired_fd` that refers to a different open file.

Expected behavior:
- the operation succeeds,
- `desired_fd` now refers to the same open file description as `fd`,
- the previous target association is replaced.

### Scenario 7: Platform-sensitive correctness path

A caller runs the module on a platform covered by the source module’s special-case handling.

Expected behavior:
- behavior matches the replacement semantics of the source module rather than relying on potentially incorrect native behavior.

### Testing Expectations

The Rust version must be testable against the above scenarios with checks for:
- success/failure outcome,
- returned descriptor value on success,
- correct handling of `fd == desired_fd`,
- failure on invalid descriptors,
- replacement of an already-open target descriptor,
- parity with source behavior for platform-specific edge cases evidenced by the module.

## Requirements

### Functional Requirements

- **FR-1**: The module shall provide a replacement descriptor-duplication operation corresponding to `rpl_dup2` in `gnu/dup2.c`.
- **FR-2**: The operation shall duplicate a valid source descriptor onto a requested target descriptor number and, on success, return that target descriptor number.
  - Traceability: `rpl_dup2`
- **FR-3**: When source and target descriptor numbers are equal, the operation shall succeed only if the descriptor is valid/open; otherwise it shall fail.
  - Traceability: `rpl_dup2`, `klibc_dup2dirfd`
- **FR-4**: The operation shall fail when the source descriptor is invalid.
  - Traceability: `rpl_dup2`, `dup2_nothrow`, `ms_windows_dup2`
- **FR-5**: The operation shall fail when the requested target descriptor number is invalid for duplication.
  - Traceability: `rpl_dup2`, `ms_windows_dup2`
- **FR-6**: When source and target differ and the target is already open, a successful operation shall replace the target so that it refers to the same open file description as the source.
  - Traceability: `rpl_dup2`, `ms_windows_dup2`, `dup2_nothrow`
- **FR-7**: The module shall preserve the source module’s platform-correct replacement semantics for the supported environments handled by its helper paths.
  - Traceability: `dup2_nothrow`, `ms_windows_dup2`, `klibc_dup2dirfd`, `rpl_dup2`
- **FR-8**: The module shall expose only the functionality evidenced by the source module’s replacement behavior and required validation logic.
  - Traceability: `gnu/dup2.c`

### Key Entities

- **File descriptor (`fd`)**
  - Integer handle naming an existing open file description.
  - Serves as the source of duplication.

- **Requested target descriptor (`desired_fd`)**
  - Integer handle number onto which the source descriptor is to be duplicated.
  - May be equal to the source descriptor or may refer to an already-open descriptor that is replaced on success.

- **File status information (`struct stat`)**
  - Used by the source module as part of descriptor validity/type checking for special-case handling.
  - Relationship: supports validation logic for descriptor acceptance, especially in equal-descriptor or platform-specific paths.

## Success Criteria

- **SC-1**: For a valid open source descriptor and a valid distinct target descriptor number, the Rust implementation returns the target descriptor number and makes the target refer to the same open file description as the source.
  - Traceability: `rpl_dup2`
- **SC-2**: For `fd == desired_fd` with a valid open descriptor, the Rust implementation succeeds and returns that descriptor number.
- **SC-3**: For `fd == desired_fd` with an invalid descriptor, the Rust implementation fails rather than reporting success.
  - Traceability: `rpl_dup2`, `klibc_dup2dirfd`
- **SC-4**: For an invalid source descriptor, the Rust implementation fails in both equal and distinct target cases.
  - Traceability: `rpl_dup2`, `dup2_nothrow`, `ms_windows_dup2`
- **SC-5**: For an invalid target descriptor number, the Rust implementation fails.
  - Traceability: `rpl_dup2`, `ms_windows_dup2`
- **SC-6**: When the target descriptor is already open and differs from the source, a successful Rust operation replaces the target association with the source association.
- **SC-7**: On platforms requiring special handling covered by the source module, the Rust implementation matches the source module’s replacement semantics for the tested edge cases.
  - Traceability: `dup2_nothrow`, `ms_windows_dup2`, `klibc_dup2dirfd`, `rpl_dup2`

## Traceability Notes

This specification is derived from the functional roles of:
- `rpl_dup2` as the module’s replacement entry point,
- `dup2_nothrow` as a helper for safe/native duplication behavior,
- `ms_windows_dup2` as Windows-specific duplication handling,
- `klibc_dup2dirfd` as validation logic for a special descriptor-handling case,
- `struct stat` usage as supporting validity/type inspection.