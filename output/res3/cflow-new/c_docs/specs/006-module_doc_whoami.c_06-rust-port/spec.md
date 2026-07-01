# spec.md

## Title

Rust Port Functional Specification for `module_doc_whoami.c_06`

## Metadata

- **Project**: `cflow-new`
- **Module**: `module_doc_whoami.c_06`
- **Category**: `module_cluster`
- **Source file**: `doc/whoami.c`
- **Primary function**: `who_am_i(void) -> int`
- **Rust branch**: `006-module_doc_whoami.c_06-rust-port`
- **Generation date**: 2026-06-17

## Overview

This module provides a single behavior: identifying the current user through the system account database information available for the running process and producing a success-or-failure result.

The Rust rewrite must preserve that functional boundary: it must perform the module’s current-user identification behavior and return an integer status result representing successful completion or failure.

## Feature Specification

### Feature: Current-user identification

The module determines the identity information for the user associated with the running process by consulting system password-account data represented by `struct passwd`.

The Rust version must implement the same functional outcome:

- obtain the current process user identity from the operating environment,
- resolve that identity to account information equivalent to the C module’s use of `struct passwd`,
- complete with an integer status result indicating success or failure.

No additional module features are evidenced by the source analysis and none are required.

## User Scenarios & Testing

### Scenario 1: Current user information is available

A caller invokes the module behavior in a normal user session where the operating system can resolve the running user to a password-account entry.

**Expected result**

- The operation completes successfully.
- The function returns a success status code.

**Test approach**

- Execute the Rust port in an environment with a valid current user account entry.
- Verify that the function reports success.

### Scenario 2: Current user information cannot be resolved

A caller invokes the module behavior in an environment where the current user cannot be mapped to password-account data or the lookup fails.

**Expected result**

- The operation reports failure through its integer return value.
- The failure is contained to the function result; no extra capabilities are required.

**Test approach**

- Run under a controlled test setup that forces user-account lookup failure.
- Verify that the function returns a failure status code.

### Scenario 3: Repeated invocation

A caller invokes the function more than once during process lifetime.

**Expected result**

- Each call independently attempts current-user identification.
- Each call returns a status code based on the lookup result for that invocation.

**Test approach**

- Call the Rust implementation multiple times in the same process.
- Verify that each call returns an appropriate success/failure code without requiring retained module state.

## Requirements

### Functional Requirements

- **FR-1**: The module shall provide a callable operation corresponding to `who_am_i(void)` from `doc/whoami.c`.
- **FR-2**: The operation shall identify the current process user using operating-system user-account information.
- **FR-3**: The operation shall use account information equivalent in role to the C module’s referenced `struct passwd` data.
- **FR-4**: The operation shall return an integer status result.
- **FR-5**: The returned status shall distinguish at minimum between successful current-user identification and failure to complete that identification.

### Key Entities

- **Current-user identification operation**
  - The module’s single functional entry point.
  - Produces the module’s integer success/failure result.

- **Password-account record**
  - Represented in the C module by referenced `struct passwd`.
  - Serves as the account-information source used to resolve the running user’s identity.

### Entity Relationships

- The current-user identification operation consults password-account record data to determine the identity of the running user.
- The operation’s returned integer status reflects whether that consultation succeeded.

## Success Criteria

- **SC-1**: The Rust module exposes behavior functionally equivalent to the C module’s single `who_am_i` operation.
- **SC-2**: In an environment where current-user account lookup succeeds, the Rust implementation returns the defined success status.
- **SC-3**: In an environment where current-user account lookup fails, the Rust implementation returns the defined failure status.
- **SC-4**: The Rust implementation requires no persistent module state to support repeated invocation.
- **SC-5**: The implementation scope remains limited to current-user identification behavior evidenced by `doc/whoami.c` and does not require unrelated public capabilities.