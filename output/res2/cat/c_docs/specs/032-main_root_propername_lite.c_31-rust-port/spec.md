# spec.md

## Title

Functional Specification for `main_root_propername-lite.c_31`

## Document Information

- Project: `cat`
- Module: `main_root_propername-lite.c_31`
- Category: `main_cluster`
- Source file: `propername-lite.c`
- Primary function: `proper_name_lite`
- Rust branch: `032-main_root_propername_lite.c_31-rust-port`
- Generation date: `2026-06-07`

## 1. Feature Specification

### 1.1 Purpose

This module provides a lightweight proper-name selection function. It accepts two string inputs:

- an ASCII form of a name
- a UTF-8 form of the same name

The module returns a single string to be used by the caller as the proper name representation.

### 1.2 In-Scope Functionality

The Rust rewrite must implement the behavior of the source module’s exported function:

- select and return a proper-name string from the provided ASCII and UTF-8 inputs
- preserve the module’s lightweight role as a name-selection utility
- expose behavior equivalent to the C module’s functional contract: input is two name strings, output is one name string

### 1.3 Out of Scope

The Rust version must not introduce capabilities not evidenced by the module input:

- no additional public APIs beyond the required equivalent of the existing function
- no name normalization, transliteration, localization, or formatting features unless required to preserve observed behavior
- no persistence, serialization, concurrency guarantees, or recovery features
- no policy beyond choosing the returned proper-name string from the given inputs

## 2. User Scenarios & Testing

### 2.1 Scenario: Caller provides ASCII and UTF-8 representations

A caller has a name available in both ASCII and UTF-8 forms and invokes the module to obtain the proper-name string to display or propagate.

Expected result:

- the module returns one string result corresponding to the module’s defined proper-name selection behavior

### 2.2 Scenario: Caller uses result as a borrowed/display string

A caller uses the returned proper-name string directly in message generation, logging, or user-facing output.

Expected result:

- the returned value is usable as a string result without requiring additional transformation by the caller to fulfill the module’s role

### 2.3 Scenario: ASCII and UTF-8 forms differ

A caller passes two non-identical representations of the same name.

Expected result:

- the module deterministically returns the same representation that the source module would return for those inputs

### 2.4 Testing Guidance

The Rust port must be tested with cases that cover:

- identical ASCII and UTF-8 name inputs
- differing ASCII and UTF-8 name inputs
- simple ASCII-only names
- UTF-8 names containing non-ASCII characters
- repeat invocation with the same inputs producing the same result

Tests must validate behavioral equivalence to the C module’s function-level contract.

## 3. Requirements

### 3.1 Functional Requirements

#### FR-1: Proper-name selection interface

The module shall provide one functionally equivalent operation to `proper_name_lite` that accepts:

- an ASCII name input
- a UTF-8 name input

and returns a single proper-name string result.

Traceability:

- `propername-lite.c`
- `proper_name_lite`

#### FR-2: Dual-input handling

The module shall base its result on the two provided input strings only.

Traceability:

- `propername-lite.c`
- `proper_name_lite`

#### FR-3: Deterministic result

For the same pair of inputs, the module shall return the same proper-name result on repeated calls.

Traceability:

- `propername-lite.c`
- `proper_name_lite`

#### FR-4: Lightweight utility scope

The module shall remain a lightweight name-selection utility and shall not require callers to use any additional module-managed state or configuration to obtain the proper-name result.

Traceability:

- `propername-lite.c`
- `proper_name_lite`

#### FR-5: Behavioral equivalence

The Rust implementation shall preserve the observable behavior of the source module for valid inputs accepted by the original function contract.

Traceability:

- `propername-lite.c`
- `proper_name_lite`

### 3.2 Key Entities

#### Entity: ASCII name input

A string representing the ASCII form of a proper name.

Relationship:

- provided by the caller as one of the two inputs to the proper-name selection function

Traceability:

- `proper_name_lite`

#### Entity: UTF-8 name input

A string representing the UTF-8 form of the same proper name.

Relationship:

- provided by the caller as the second input to the proper-name selection function

Traceability:

- `proper_name_lite`

#### Entity: Proper-name result

A string result returned by the module as the selected proper-name representation.

Relationship:

- derived from the two input strings by the module’s single exported function

Traceability:

- `proper_name_lite`

## 4. Success Criteria

### 4.1 Functional Correctness

- The Rust module provides one functionally equivalent proper-name selection operation matching the source module’s input/output role.
- For every test case derived from the source module behavior, the Rust result matches the C module result.

Traceability:

- `propername-lite.c`
- `proper_name_lite`

### 4.2 Scenario Coverage

- Tests exist for identical input forms, differing input forms, ASCII-only input, and UTF-8 input with non-ASCII characters.
- In each scenario, the module returns a single string result consistent with source behavior.

Traceability:

- `propername-lite.c`
- `proper_name_lite`

### 4.3 Determinism

- Repeated calls with the same inputs produce the same result in the Rust port.

Traceability:

- `propername-lite.c`
- `proper_name_lite`

### 4.4 Scope Fidelity

- The Rust rewrite does not expose additional evidenced-unrelated functionality beyond the behavior required to replace the source module in this area.

Traceability:

- `propername-lite.c`
- `proper_name_lite`