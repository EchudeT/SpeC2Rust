# spec.md

## Document Control

- **Project**: `pwd`
- **Module**: `main_root_propername-lite.c_22`
- **Category**: `main_cluster`
- **Source file**: `propername-lite.c`
- **Primary function**: `proper_name_lite`
- **Rust branch**: `022-main_root_propername_lite.c_22-rust-port`
- **Generation date**: `2026-06-07`

## 1. Feature Specification

### 1.1 Purpose

This module provides a minimal proper-name selection function for display-oriented use. It accepts two candidate name strings:

- an ASCII form
- a UTF-8 form

The module returns a single name string to use as the effective proper name.

### 1.2 In-Scope Functionality

The Rust rewrite must implement the behavior of the existing module function:

- Accept two input string values representing the same name in different encodings/forms.
- Decide which input string should be returned as the selected proper name.
- Return one of the provided inputs unchanged as the result.

### 1.3 Behavioral Summary

Based on the module boundary evidenced by `proper_name_lite`, the function is a lightweight selector rather than a formatter, parser, allocator, or normalizer. The Rust version must preserve this role:

- it does not construct a new name value
- it does not expose additional public behaviors
- it chooses between the supplied name forms and yields the chosen one

### 1.4 Out of Scope

The Rust version must not introduce capabilities not evidenced by this module, including:

- name normalization
- transliteration
- locale-aware formatting
- validation beyond what is necessary to preserve the original function’s behavior
- storage, caching, or registration of names
- new public APIs beyond the Rust equivalent of the module functionality

## 2. User Scenarios & Testing

### 2.1 Scenario: UTF-8 form is preferred for display

A caller has both an ASCII-safe name and a UTF-8 name containing the intended proper spelling. The caller invokes this module to obtain the display name.

**Expected behavior**:
- The module returns the selected proper name according to the original module behavior.
- If the original C behavior prefers the UTF-8 form in this case, the Rust version must do the same.

**Testing guidance**:
- Provide distinct ASCII and UTF-8 inputs.
- Verify that the Rust result matches the C module’s selected input.

### 2.2 Scenario: ASCII form remains the effective result

A caller supplies both name forms, but the original module behavior selects the ASCII form for the given inputs.

**Expected behavior**:
- The Rust version returns the ASCII input when that is what the C module would return.

**Testing guidance**:
- Use inputs that exercise the branch in which ASCII is selected.
- Verify exact behavioral equivalence with the C function.

### 2.3 Scenario: Inputs are already identical in meaning or content

A caller provides two strings that are equivalent for the module’s selection logic.

**Expected behavior**:
- The module returns the effective name exactly as determined by the original function.
- No extra rewriting or transformation is introduced.

**Testing guidance**:
- Use identical strings and observe which supplied value is considered the effective result in the C behavior.
- Match that behavior in Rust.

### 2.4 Scenario: Integration call site uses the return value directly

A higher-level caller uses the result immediately for message text, diagnostics, or user-visible output.

**Expected behavior**:
- The module produces a directly usable selected name value.
- The Rust version preserves the same selection semantics expected by callers.

**Testing guidance**:
- Use the return value in a simple formatting path.
- Confirm the displayed text matches what the C module would have produced.

## 3. Requirements

### 3.1 Functional Requirements

#### FR-1: Dual-input proper-name selection
The module shall accept two input name values: one ASCII-form name and one UTF-8-form name.

**Traceability**: `propername-lite.c`, `proper_name_lite`

#### FR-2: Single selected result
The module shall produce one effective proper name result from the two supplied inputs.

**Traceability**: `propername-lite.c`, `proper_name_lite`

#### FR-3: Result is one of the supplied inputs
The module shall return a result corresponding to one of the provided name inputs, not a newly synthesized alternative.

**Traceability**: `propername-lite.c`, `proper_name_lite`

#### FR-4: Behavioral equivalence with module selection logic
The Rust rewrite shall preserve the observable selection behavior of the C module for the same pair of inputs.

**Traceability**: `propername-lite.c`, `proper_name_lite`

#### FR-5: Lightweight module scope
The Rust rewrite shall remain limited to proper-name selection behavior and shall not add unrelated processing responsibilities.

**Traceability**: module boundary evidenced by `propername-lite.c`, `proper_name_lite`

### 3.2 Key Entities

#### Entity: ASCII proper name
A caller-supplied name string representing an ASCII form of the proper name.

**Relationship**:
- Provided as one of the two inputs to the selector function.

**Traceability**: `proper_name_lite(name_ascii, ...)`

#### Entity: UTF-8 proper name
A caller-supplied name string representing a UTF-8 form of the proper name.

**Relationship**:
- Provided as the second candidate input to the selector function.

**Traceability**: `proper_name_lite(..., name_utf8)`

#### Entity: Selected proper name
The effective name chosen by the module and returned to the caller.

**Relationship**:
- Derived by selecting between the ASCII proper name and UTF-8 proper name.
- Returned as the sole output of the function.

**Traceability**: return value of `proper_name_lite`

## 4. Success Criteria

### 4.1 Behavioral Criteria

- The Rust module exposes functionality equivalent to the C module’s proper-name selection behavior for all tested input pairs.
- For every conformance test case derived from `proper_name_lite`, the Rust result matches the C result in which candidate input is selected.
- The Rust module does not introduce additional user-visible transformations of the chosen name.

### 4.2 Interface-Level Criteria

- The Rust rewrite accepts two input name values corresponding to the ASCII and UTF-8 forms.
- The Rust rewrite returns a single effective proper name value suitable for immediate use by callers.

### 4.3 Scope Control Criteria

- The Rust module remains confined to the functionality evidenced by `proper_name_lite`.
- No extra public capabilities are added beyond the proper-name selection role of this module.

### 4.4 Test Completion Criteria

The rewrite is considered successful when all of the following are true:

1. A test set covers cases where the UTF-8 candidate is selected.
2. A test set covers cases where the ASCII candidate is selected.
3. A test set covers cases where both inputs are identical or equivalent for selection purposes.
4. All such tests confirm Rust/C behavioral equivalence at the module boundary.