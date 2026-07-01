# spec.md

## Title

Functional Specification for `module_gnu_stat.c_47` Rust Port

## Metadata

- Project: `cflow-new`
- Module: `module_gnu_stat.c_47`
- Category: `module_cluster`
- Source file: `gnu/stat.c`
- Rust branch: `053-module_gnu_stat.c_47-rust-port`
- Generation date: `2026-06-17`

## Overview

This module provides path classification logic related to filesystem status handling. Based on the available analysis, its evidenced responsibility is to identify whether a provided path string denotes a UNC root path. The Rust rewrite must preserve this behavior and its decision boundaries.

The available evidence identifies one internal function:

- `is_unc_root(const char *rname)` in `gnu/stat.c:79-102`

The Rust version must therefore implement equivalent UNC-root detection behavior for path inputs, preserving the module’s functional role as a path classifier used by surrounding stat-related logic.

## Feature Specification

### Feature: UNC root path detection

The module determines whether an input path string represents a UNC root.

The Rust version must implement:

- Acceptance of a path-like string input.
- Classification of that input as either:
  - a UNC root path, or
  - not a UNC root path.
- Behavior consistent with the source module’s path-focused stat support role.

### Functional boundary

Only the behavior evidenced by the analyzed module is in scope:

- UNC root detection for a provided path string.

The Rust rewrite must not assume or introduce additional filesystem metadata retrieval features beyond what is directly evidenced by this module analysis.

## User Scenarios & Testing

### Scenario 1: Recognize a valid UNC root

A caller supplies a path string that denotes a UNC root.
The module classifies it as a UNC root.

**Expected result:**

- The returned classification is positive.

**Testing guidance:**

- Provide representative UNC-root inputs.
- Verify the Rust implementation returns the same classification as the C module behavior.

### Scenario 2: Reject a non-root UNC-like path

A caller supplies a path string that uses UNC-style syntax but refers to a deeper location rather than the root.
The module classifies it as not being a UNC root.

**Expected result:**

- The returned classification is negative.

**Testing guidance:**

- Provide UNC-style paths containing additional path components beyond the root boundary.
- Verify they are not classified as UNC roots.

### Scenario 3: Reject ordinary non-UNC paths

A caller supplies a path string in another format that is not a UNC root.
The module classifies it as not being a UNC root.

**Expected result:**

- The returned classification is negative.

**Testing guidance:**

- Provide non-UNC path inputs.
- Verify the classifier does not report them as UNC roots.

### Scenario 4: Deterministic classification for identical input

A caller evaluates the same input path multiple times.
The module returns the same classification each time.

**Expected result:**

- Repeated evaluation of the same path produces identical results.

**Testing guidance:**

- Re-run classification on identical test inputs and compare outputs.

## Requirements

### Functional Requirements

#### FR-1: Path input classification

The module shall accept a path string input and determine whether it represents a UNC root path.

**Traceability:** `gnu/stat.c`, `is_unc_root`

#### FR-2: Positive identification of UNC roots

The module shall return a positive result for inputs that match the UNC root form recognized by the source module.

**Traceability:** `gnu/stat.c`, `is_unc_root`

#### FR-3: Rejection of non-root paths

The module shall return a negative result for inputs that do not match the UNC root form, including paths that are not UNC roots.

**Traceability:** `gnu/stat.c`, `is_unc_root`

#### FR-4: Pure classification behavior

The module shall provide classification behavior based solely on the provided path input, with no requirement to obtain or depend on filesystem status data in order to determine whether the path is a UNC root.

**Traceability:** `gnu/stat.c`, `is_unc_root`

### Key Entities

#### Path input

A character-string path value provided to the module for classification.

**Relationship:**

- This is the sole direct input to UNC root detection.

**Traceability:** `is_unc_root(const char *rname)`

#### UNC root classification result

A boolean-style result indicating whether the provided path is a UNC root.

**Relationship:**

- Produced from evaluation of the path input.

**Traceability:** `static BOOL is_unc_root(...)`

#### Filesystem status structure context

The source file references `struct stat`, indicating that the module exists within a stat-related context, but the analyzed evidence does not show direct `struct stat` participation in the identified UNC-root detection function.

**Relationship:**

- Contextual to the file.
- Not required as an input or output of the evidenced function behavior.

**Traceability:** `gnu/stat.c:39`, `gnu/stat.c:112`

## Success Criteria

### SC-1: Behavioral equivalence for classification

For a test set containing UNC-root inputs and non-UNC-root inputs, the Rust port produces the same positive/negative classification outcomes as the source module.

**Traceability:** `gnu/stat.c`, `is_unc_root`

### SC-2: Correct rejection of non-root variants

For UNC-style paths that extend beyond the root boundary, the Rust port classifies them as not being UNC roots.

**Traceability:** `gnu/stat.c`, `is_unc_root`

### SC-3: Deterministic results

Given the same input string, repeated calls in the Rust port return the same classification result.

**Traceability:** `gnu/stat.c`, `is_unc_root`

### SC-4: No required dependency on stat structure for this behavior

The Rust implementation of the evidenced functionality can evaluate UNC-root status from the path input alone, without requiring a populated filesystem status structure for this decision.

**Traceability:** `gnu/stat.c`, `is_unc_root`; contextual contrast with `struct stat` references at `gnu/stat.c:39` and `gnu/stat.c:112`

## Out of Scope

The following are not specified because they are not evidenced by the provided module analysis:

- General-purpose file status retrieval behavior
- Additional public APIs beyond the evidenced functionality
- Error reporting models beyond boolean-style classification
- Filesystem mutation
- Any guarantees about platform support beyond preserving the observed UNC-root classification role