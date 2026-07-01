# Functional Specification: main_root_version_etc_06

## 1. Overview

This module provides the program version-reporting text used by the `pwd` project. Its responsibility is to write a formatted version-and-authorship notice to a caller-supplied output stream.

The Rust rewrite must preserve the observable behavior of the C module in `version-etc.c`, specifically for the exported interfaces that accept:
- an explicit author count and author array,
- a null-terminated author array,
- a variadic author list wrapper,
- and a variadic convenience entry point.

The module is output-oriented only: it formats and emits version information, package identity, authorship lines, and warranty/license notice text to the target stream.

## 2. Feature Specification

### 2.1 Primary Capability

The module shall generate and write a complete version notice for a command or package to a provided output stream.

The generated notice must include:
- program/package identification,
- version text,
- author attribution derived from provided author names,
- and the standard notice text emitted by this module.

### 2.2 Supported Input Forms

The Rust version must support the same functional input forms evidenced by the source module:

1. **Explicit author array with count**
   - Accepts a stream, command name, package name, version string, author array, and author count.
   - Uses exactly the provided number of authors.

2. **Null-terminated author array**
   - Accepts a stream, command name, package name, version string, and author array terminated by a null sentinel.
   - Determines the number of authors by scanning until the terminator.

3. **Variadic author forwarding**
   - Accepts a stream, command name, package name, version string, and a variable author argument list.
   - Treats the author list as ending at a null sentinel.

4. **Variadic convenience entry point**
   - Provides the same behavior as the variadic forwarding form, exposed as the top-level convenience interface.

These are alternate entry forms to the same formatting behavior, not separate features with differing output semantics.

### 2.3 Output Behavior

The module shall write formatted text to the caller-provided stream and shall not require ownership of that stream.

The output shall be determined by:
- `command_name`,
- `package`,
- `version`,
- and the provided authors.

The Rust rewrite must preserve the behavior that author presentation varies according to how many authors are supplied. The formatting must correctly handle at least:
- zero authors,
- one author,
- two authors,
- three or more authors.

The Rust rewrite must also preserve the module’s role as a pure reporting utility: invoking it produces text output and does not return formatted data as a separate value.

## 3. User Scenarios & Testing

### 3.1 Scenario: Command prints its version notice to standard output

A command in the `pwd` project requests version output for `--version` handling and passes:
- the output stream for standard output,
- the command name,
- package name,
- version string,
- and a known set of authors.

**Expected result:**
A complete version notice is written to the stream, including version identification and author attribution.

**Test coverage:**
- Verify output is written to a writable stream.
- Verify command/package/version strings appear in the emitted text.
- Verify the notice completes without requiring additional caller formatting.

### 3.2 Scenario: Caller has authors in a fixed array with explicit length

A caller already knows the number of authors and provides an array plus count.

**Expected result:**
The module writes the version notice using exactly that number of authors.

**Test coverage:**
- Count = 0 produces valid version output without author overread.
- Count = 1 uses one author.
- Count = 2 uses two authors.
- Count >= 3 uses the multi-author formatting path.
- Extra array elements beyond `n_authors` are ignored.

### 3.3 Scenario: Caller has a null-terminated author list

A caller stores authors in a null-terminated list and uses the array-based convenience entry point.

**Expected result:**
The module determines the author count from the terminator and produces the same output as the explicit-count form would for the same authors.

**Test coverage:**
- Null terminator at first element is treated as zero authors.
- A one-, two-, and multi-author list is accepted.
- Output matches the explicit-count form for equivalent inputs.

### 3.4 Scenario: Caller uses variadic author arguments

A caller passes author names as variadic arguments terminated with a null sentinel.

**Expected result:**
The module consumes authors until the terminator and emits the same formatted notice as the array-based forms.

**Test coverage:**
- One-author variadic call.
- Two-author variadic call.
- Multi-author variadic call.
- Null sentinel immediately after fixed parameters yields zero-author behavior.
- Output matches equivalent array-based calls.

### 3.5 Scenario: Output destination is a non-stdout stream

A caller directs version text to another writable stream, such as a memory-backed or file-backed stream.

**Expected result:**
The same version notice is written to the supplied destination.

**Test coverage:**
- Verify emitted text is captured in an alternate stream.
- Verify no dependency on a specific global output stream exists.

## 4. Requirements

### 4.1 Functional Requirements

**FR-1. Version notice emission**
The module shall write a formatted version notice to a caller-supplied output stream.
**Traceability:** `version-etc.c`, `version_etc_arn`

**FR-2. Identification inputs**
The emitted notice shall be parameterized by command name, package name, and version string supplied by the caller.
**Traceability:** `version-etc.c`, `version_etc_arn`, `version_etc_ar`, `version_etc_va`, `version_etc`

**FR-3. Author handling with explicit count**
The module shall support author attribution from an author array plus explicit author count, using exactly `n_authors` entries.
**Traceability:** `version-etc.c`, `version_etc_arn`

**FR-4. Author handling with null-terminated array**
The module shall support author attribution from a null-terminated author array by deriving the author count from the terminator.
**Traceability:** `version-etc.c`, `version_etc_ar`

**FR-5. Author handling with variadic input**
The module shall support author attribution from variadic inputs terminated by a null sentinel.
**Traceability:** `version-etc.c`, `version_etc_va`, `version_etc`

**FR-6. Consistent behavior across entry forms**
For equivalent command name, package, version, and author data, all supported entry forms shall produce equivalent observable output.
**Traceability:** `version-etc.c`, `version_etc_arn`, `version_etc_ar`, `version_etc_va`, `version_etc`

**FR-7. Author-count-sensitive formatting**
The module shall format author attribution appropriately for the number of supplied authors, including distinct handling for zero, one, two, and three-or-more authors.
**Traceability:** `version-etc.c`, `version_etc_arn`

**FR-8. Standard notice text inclusion**
The emitted output shall include the standard notice text produced by this module in addition to identification and author lines.
**Traceability:** `version-etc.c`, `version_etc_arn`

### 4.2 Key Entities

This module does not define project-specific structs or persistent state. Its key entities are input values and their relationships:

- **Output stream**
  - Destination receiving the formatted version notice.
  - Supplied by the caller to every public entry point.

- **Command name**
  - The invoked program name or display name used in the emitted notice.

- **Package name**
  - The package identity included in the emitted notice.

- **Version string**
  - The version identifier included in the emitted notice.

- **Author list**
  - A logical ordered list of author names.
  - Represented in one of three caller-facing forms:
    - array with explicit count,
    - null-terminated array,
    - variadic list terminated by null.
  - All forms map to the same logical author sequence consumed by the formatter.

## 5. Success Criteria

### 5.1 Behavioral Equivalence

1. For the same stream target and equivalent logical inputs, the Rust implementation produces the same visible version notice content across all supported entry forms.
   **Traceability:** `version_etc_arn`, `version_etc_ar`, `version_etc_va`, `version_etc`

2. The Rust implementation correctly handles zero, one, two, and three-or-more authors without reading beyond the supplied logical author list.
   **Traceability:** `version_etc_arn`, `version_etc_ar`, `version_etc_va`, `version_etc`

3. The Rust implementation writes output to the caller-provided stream rather than assuming a fixed global destination.
   **Traceability:** `version_etc_arn`, `version_etc_ar`, `version_etc_va`, `version_etc`

### 5.2 Testable Output Requirements

4. In test cases using explicit-count and null-terminated-array forms with equivalent author data, captured output is identical.
   **Traceability:** `version_etc_arn`, `version_etc_ar`

5. In test cases using explicit-count and variadic forms with equivalent author data, captured output is identical.
   **Traceability:** `version_etc_arn`, `version_etc_va`, `version_etc`

6. Captured output contains caller-supplied command name, package name, and version string in the emitted notice.
   **Traceability:** `version_etc_arn`

7. Captured output includes author attribution text when one or more authors are supplied and remains valid when zero authors are supplied.
   **Traceability:** `version_etc_arn`

8. Captured output includes the standard notice text emitted by the module.
   **Traceability:** `version_etc_arn`