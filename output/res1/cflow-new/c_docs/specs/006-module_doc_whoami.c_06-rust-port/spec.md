# spec.md

## Overview

- **Project:** `cflow-new`
- **Module:** `module_doc_whoami.c_06`
- **Category:** `module_cluster`
- **Source:** `doc/whoami.c`
- **Primary function:** `who_am_i(void) -> int`

## Feature Specification

This module provides a small identity-reporting behavior for the current process user.

From the analyzed source, the module determines the effective user identity of the running process through the system password database interface (`struct passwd` as returned by the operating system), and reports that identity as text output. The function returns an integer status.

The Rust rewrite must preserve this functional boundary:

- obtain the current effective user identity from the host environment;
- resolve that identity to a password-entry-like record sufficient to access the user name;
- emit the resolved user name as output;
- return an integer status result consistent with successful completion when the user name is available.

No broader account-management behavior, persistence, or alternate output modes are evidenced by this module and must not be added to the specification.

## User Scenarios & Testing

### Scenario 1: Report the current effective user name
A caller invokes the module function in a normal process context where the operating system can resolve the process's effective user ID to a user entry.

**Expected behavior:**
- The module retrieves the current effective user identity.
- The module resolves that identity to a user record.
- The module outputs the corresponding user name.
- The function completes with a success status.

**Test approach:**
- Run under a known user account.
- Invoke the function.
- Verify that output contains that account's resolved login name.
- Verify that the returned status indicates success.

### Scenario 2: Use in documentation/demo style execution
A caller uses the function as a minimal demonstration of querying the current process identity.

**Expected behavior:**
- The function performs the identity lookup without requiring caller-provided input.
- The output is directly human-readable.
- The behavior is limited to reporting the current user identity.

**Test approach:**
- Invoke the function with no setup other than process execution.
- Verify that no arguments or external module state are required.
- Verify that the only observable functional result is user-name reporting plus status return.

### Scenario 3: Password entry is the source of the reported name
A caller expects the displayed name to come from the system user database entry associated with the effective user ID.

**Expected behavior:**
- The reported name corresponds to the name field in the resolved password entry.
- The module does not synthesize an unrelated identity string.

**Test approach:**
- In an environment where the effective user ID maps to a known password entry, compare the module output with the system-resolved user name for that ID.

## Requirements

### Functional Requirements

#### FR-1: Current effective identity lookup
The module shall determine the identity of the current process user from the host operating environment, using the effective user context as evidenced by the source behavior around password-entry resolution.

**Traceability:** `doc/whoami.c`, `who_am_i`

#### FR-2: User-record resolution
The module shall resolve the current effective identity to a system user record equivalent in role to a `passwd` entry, sufficient to access the account name.

**Traceability:** `doc/whoami.c`, `who_am_i`, referenced `struct passwd`

#### FR-3: User-name reporting
The module shall output the resolved user name in text form.

**Traceability:** `doc/whoami.c`, `who_am_i`, referenced `struct passwd`

#### FR-4: Integer status return
The module shall provide an integer return value representing the function result.

**Traceability:** `doc/whoami.c`, `who_am_i`

### Key Entities

#### `who_am_i`
The module's functional entry point. It performs the current-user lookup, obtains the corresponding user record, reports the user name, and returns an integer status.

**Traceability:** `doc/whoami.c`, `who_am_i`

#### `passwd` / `struct passwd`
An operating-system-provided user account record referenced by the module. Its role in this module is to supply the resolved user name associated with the current effective user identity.

**Traceability:** `doc/whoami.c`, referenced `struct passwd`

## Success Criteria

1. **Correct identity reporting:** When executed under a user account that is resolvable through the host system user database, the Rust module outputs that account's user name.
   **Traceability:** `doc/whoami.c`, `who_am_i`, `struct passwd`

2. **No caller input required:** The Rust module can perform its documented behavior without function arguments or caller-supplied identity data.
   **Traceability:** `who_am_i(void)`

3. **Uses resolved account record semantics:** The reported name matches the user name associated with the effective user identity as resolved from a password-entry-like system record.
   **Traceability:** `doc/whoami.c`, `who_am_i`, `struct passwd`

4. **Status return preserved:** The Rust function returns an integer status value on completion.
   **Traceability:** `doc/whoami.c`, `who_am_i`