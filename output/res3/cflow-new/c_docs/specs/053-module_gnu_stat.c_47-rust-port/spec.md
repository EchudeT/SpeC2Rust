# spec.md

## Title

Functional Specification for `module_gnu_stat.c_47` Rust Port

## Metadata

- **Project**: `cflow-new`
- **Module**: `module_gnu_stat.c_47`
- **Category**: `module_cluster`
- **Source file**: `gnu/stat.c`
- **Rust branch**: `053-module_gnu_stat.c_47-rust-port`
- **Generation date**: `2026-06-17`

## Overview

This module provides path classification logic related to special Windows UNC root handling. The analyzed functionality is narrowly scoped to determining whether a path string denotes a UNC root. The Rust rewrite must preserve this behavior as a focused internal utility within the module boundary evidenced by `gnu/stat.c` and `is_unc_root`.

The available evidence does not show broader stat-wrapping behavior as part of this module specification. Therefore, the Rust version must implement only the path classification behavior directly evidenced here.

## Feature Specification

### Feature: UNC root path detection

The module identifies whether a provided path string represents a UNC root path.

A UNC root path, for purposes of this module, is a network-style pathname root in the form recognized by the source module’s `is_unc_root` logic. The Rust port must preserve the same classification intent:

- accept a path input as a string
- inspect its structure
- return a boolean result indicating whether the path is a UNC root
- avoid treating non-root or malformed paths as UNC roots

This functionality is internal and behavioral; no new public capability beyond this classification behavior is required by the evidence.

## User Scenarios & Testing

### Scenario 1: Recognize a valid UNC root

**Given** a caller has a pathname string that denotes a network root in UNC form
**When** the module evaluates the pathname
**Then** it returns `true`

**Testing approach**:
- provide representative UNC-root path inputs
- verify that each is classified as a UNC root

### Scenario 2: Reject ordinary local paths

**Given** a caller provides a normal local filesystem path
**When** the module evaluates the pathname
**Then** it returns `false`

**Testing approach**:
- provide absolute and relative non-UNC paths
- verify all are rejected as UNC roots

### Scenario 3: Reject UNC paths that are not roots

**Given** a caller provides a UNC-style path that includes components beyond the root portion
**When** the module evaluates the pathname
**Then** it returns `false`

**Testing approach**:
- provide UNC-style paths with additional subpath components
- verify they are not classified as roots

### Scenario 4: Reject malformed or incomplete UNC forms

**Given** a caller provides a string resembling a UNC path but lacking the structure required for a root
**When** the module evaluates the pathname
**Then** it returns `false`

**Testing approach**:
- provide incomplete, truncated, or malformed UNC-like inputs
- verify the result is consistently `false`

### Scenario 5: Deterministic classification

**Given** the same pathname string is evaluated multiple times
**When** the module performs classification
**Then** it returns the same boolean result each time

**Testing approach**:
- run repeated evaluations on the same inputs
- verify stable and repeatable results

## Requirements

### Functional Requirements

#### FR-1: Boolean UNC root classification
The module shall accept a path string input and classify it as either a UNC root or not a UNC root.

**Traceability**: `gnu/stat.c`, `is_unc_root`

#### FR-2: Positive recognition of UNC roots
The module shall return a positive result only when the input string matches the UNC root form recognized by the source module.

**Traceability**: `gnu/stat.c`, `is_unc_root`

#### FR-3: Rejection of non-root paths
The module shall return a negative result for path strings that are not UNC roots, including ordinary local paths and UNC-style paths with additional path content beyond the root.

**Traceability**: `gnu/stat.c`, `is_unc_root`

#### FR-4: Rejection of malformed or incomplete inputs
The module shall return a negative result for malformed, partial, or incomplete UNC-like path strings that do not satisfy UNC root structure.

**Traceability**: `gnu/stat.c`, `is_unc_root`

#### FR-5: Side-effect-free evaluation
The module shall determine the result from the provided string input without modifying the input or requiring mutation of filesystem state.

**Traceability**: `gnu/stat.c`, `is_unc_root`

### Key Entities

#### Path string
A character string representing a filesystem path. It is the sole direct input to the evidenced module behavior.

**Relationships**:
- consumed by UNC root classification
- yields a boolean classification result

#### UNC root classification result
A boolean outcome indicating whether the provided path string denotes a UNC root.

**Relationships**:
- derived from the path string by the module’s classification logic
- used by callers to distinguish UNC roots from all other path forms

#### `struct stat`
The source file references `struct stat`, but the analyzed evidence for this module slice does not establish direct behavioral requirements tied to those structures for `is_unc_root`.

**Relationships**:
- present in source context
- no additional required behavior is inferred from the available evidence

## Success Criteria

### SC-1: Correct positive classification
For all test inputs representing UNC roots covered by the source behavior, the Rust module returns `true`.

**Traceability**: `gnu/stat.c`, `is_unc_root`

### SC-2: Correct negative classification
For all tested non-UNC-root inputs, including local paths, extended UNC paths, and malformed UNC-like strings, the Rust module returns `false`.

**Traceability**: `gnu/stat.c`, `is_unc_root`

### SC-3: Deterministic results
Repeated evaluation of the same input produces the same boolean result across runs within the same build.

**Traceability**: `gnu/stat.c`, `is_unc_root`

### SC-4: No unsupported feature expansion
The Rust port confines its required behavior to UNC root detection evidenced by this module analysis and does not require additional externally visible capabilities not supported by the provided source evidence.

**Traceability**: `gnu/stat.c`, `is_unc_root`

## Scope Constraints

- This specification covers only the behavior evidenced by `is_unc_root` in `gnu/stat.c`.
- No broader filesystem metadata retrieval behavior is required by this specification.
- No new public API surface is implied beyond the functionality necessary to preserve the evidenced module behavior.