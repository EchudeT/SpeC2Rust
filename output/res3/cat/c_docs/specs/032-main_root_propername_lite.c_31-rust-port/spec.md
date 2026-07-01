# spec.md

## Title

Functional Specification: `main_root_propername-lite.c_31`

## Document Metadata

- Project: `cat`
- Module: `main_root_propername-lite.c_31`
- Category: `main_cluster`
- Source file: `propername-lite.c`
- Primary function: `proper_name_lite`
- Rust branch target: `032-main_root_propername_lite.c_31-rust-port`
- Generation date: `2026-06-09`

## Overview

This module provides a small name-selection utility for presenting a proper name in a preferred form. It accepts two caller-supplied string inputs:

- an ASCII-form name
- a UTF-8-form name

Its functional role is to return a pointer to the name representation that should be used by the caller.

The Rust rewrite must preserve this module’s externally observable behavior as a pure selection utility over the two provided string inputs. The module specification is limited to the single evidenced function and does not assume any broader formatting, storage, localization, or validation responsibilities beyond choosing which name form to expose.

## Feature Specification

### Summary

The module exposes one function that chooses a proper-name string from two provided candidate representations. The Rust version must provide equivalent functionality: given an ASCII name and a UTF-8 name, it must yield the appropriate display/result string according to the same observable behavior as the C module.

### Functional Scope

In scope:

- Accepting two input strings representing the same proper name in different encodings/forms.
- Returning one selected string result for caller use.
- Preserving caller-visible selection behavior of the C function.

Out of scope:

- Creating new name variants.
- Persisting names.
- Performing broad text normalization or transliteration beyond what is required to match the C module’s behavior.
- Managing ownership beyond what is necessary for the Rust equivalent API.
- Any additional public API not evidenced by the module analysis.

## User Scenarios & Testing

### Scenario 1: ASCII and UTF-8 forms are both supplied

A caller has both an ASCII-safe form and a UTF-8 form of a proper name and asks the module for the preferred result.

Expected support in Rust:

- The module accepts both forms.
- The module returns the same effective selection as the C implementation would for the same inputs.

Test focus:

- Compare Rust output against expected behavior from the C module for representative pairs of ASCII and UTF-8 inputs.

### Scenario 2: Caller uses the result directly for display or messaging

A caller uses the returned proper-name string in user-facing output.

Expected support in Rust:

- The returned value is usable as the selected name result without requiring additional transformation by this module.
- The module does not alter its contract from “select a name string” into broader formatting behavior.

Test focus:

- Confirm the selected string content matches the C module’s output behavior.

### Scenario 3: Inputs differ only by representation

A caller provides two strings intended to represent the same name, where one is ASCII-oriented and the other UTF-8-oriented.

Expected support in Rust:

- The module chooses consistently for equivalent repeated calls with the same inputs.
- Selection behavior remains traceable to the original single C function.

Test focus:

- Repeated calls with identical inputs produce identical selected results.

### Scenario 4: Boundary string inputs accepted by the original function

A caller passes string inputs that are valid under the original C function contract.

Expected support in Rust:

- The Rust port handles all input cases supported by the original module contract.
- No new restrictions are introduced unless required by Rust safety and reflected in equivalent observable behavior.

Test focus:

- Exercise empty and non-empty valid string cases if these are accepted by the original behavior.
- Verify result selection remains consistent with the C module.

## Requirements

### Functional Requirements

#### FR-1: Dual-input proper-name selection

The module shall accept two caller-provided name inputs corresponding to an ASCII form and a UTF-8 form of a proper name.

Traceability:

- `propername-lite.c`
- `proper_name_lite`

#### FR-2: Single selected result

The module shall produce one selected name result from the two provided inputs.

Traceability:

- `propername-lite.c`
- `proper_name_lite`

#### FR-3: Behavior preservation

The Rust rewrite shall preserve the caller-visible behavior of `proper_name_lite` for the same valid inputs.

Traceability:

- `propername-lite.c`
- `proper_name_lite`

#### FR-4: No expansion beyond selection responsibility

The Rust rewrite shall remain limited to the evidenced responsibility of selecting/returning a proper-name string and shall not require unrelated features such as storage, serialization, localization frameworks, or additional formatting APIs.

Traceability:

- `propername-lite.c`
- `proper_name_lite`

### Key Entities

#### Entity 1: ASCII proper name input

A caller-supplied string representing a proper name in ASCII form.

Relationship:

- Paired with the UTF-8 proper name input as an alternative representation of the same conceptual name.
- Consumed by the selection function.

Traceability:

- `proper_name_lite (char const *name_ascii, char const *name_utf8)`

#### Entity 2: UTF-8 proper name input

A caller-supplied string representing a proper name in UTF-8 form.

Relationship:

- Paired with the ASCII proper name input as an alternative representation of the same conceptual name.
- Consumed by the selection function.

Traceability:

- `proper_name_lite (char const *name_ascii, char const *name_utf8)`

#### Entity 3: Selected proper name result

The string result returned by the module as the chosen representation.

Relationship:

- Derived from the two caller-provided input strings.
- Returned directly by the selection function.

Traceability:

- `char const * proper_name_lite (...)`

## Success Criteria

### SC-1: Functional equivalence

For all valid test inputs covered from the original module contract, the Rust port returns the same selected string content as the C function.

Traceability:

- `proper_name_lite`

### SC-2: Supported usage scenarios

The Rust module supports the documented scenarios of:
- selecting between ASCII and UTF-8 name forms,
- returning a directly usable result string,
- producing consistent results for repeated identical inputs.

Traceability:

- `proper_name_lite`

### SC-3: Interface scope discipline

The Rust rewrite exposes only the functionality necessary to preserve the documented behavior of the original module and does not introduce unrelated module responsibilities.

Traceability:

- `propername-lite.c`
- `proper_name_lite`

### SC-4: Testable deterministic behavior

Given the same valid inputs, the Rust implementation produces the same result on repeated invocations within the same build and test environment.

Traceability:

- `proper_name_lite`

## Notes

This specification is intentionally narrow because the analyzed module evidence consists of a single function with a small, focused contract. Any exact preference rules between the ASCII and UTF-8 inputs must be matched from the original C behavior during porting and validation, but no broader capabilities should be inferred beyond that evidenced selection behavior.