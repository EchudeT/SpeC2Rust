# spec.md

## Title
Functional Specification for `main_root_propername-lite.c_22`

## Document Information
- Project: `pwd`
- Module: `main_root_propername-lite.c_22`
- Category: `main_cluster`
- Source file: `propername-lite.c`
- Primary function: `proper_name_lite`
- Rust branch target: `022-main_root_propername_lite.c_22-rust-port`
- Generation date: 2026-06-07

## Overview
This module provides a minimal name-selection function for presenting a preferred proper name string. It accepts two caller-supplied name representations: one ASCII form and one UTF-8 form. The module returns a string pointer representing the name variant that should be used by the caller.

The Rust rewrite must preserve this module’s externally observable behavior as a lightweight selector over the two input name strings, without introducing additional public capabilities beyond the evidenced function.

## Feature Specification

### Feature Summary
Provide a lightweight proper-name selection facility that:
- accepts an ASCII name representation,
- accepts a UTF-8 name representation,
- returns a single selected name representation for caller use.

### Required Rust Behavior
The Rust version must implement behavior equivalent to the C module’s public function:
- It must expose module functionality corresponding to `proper_name_lite`.
- It must operate only on the two provided name inputs.
- It must return the selected name result without modifying the caller’s provided name content.
- It must remain a minimal facility focused on choosing the proper display name variant.

### Functional Boundary
Included:
- selection of one of the provided name representations for use as the proper name result.

Excluded:
- generation of new name strings,
- normalization or transliteration beyond what is evidenced by input selection,
- storage management features not evidenced by the interface,
- locale negotiation,
- formatting systems,
- additional public APIs.

## User Scenarios & Testing

### Scenario 1: ASCII-only effective usage
A caller has an ASCII-safe name form and also passes a UTF-8 form, but the effective result used by the module corresponds to the ASCII-safe representation.

Test expectation:
- Given valid caller-supplied ASCII and UTF-8 name inputs, the module returns the selected proper name string as defined by the source behavior.
- The returned result is usable by the caller as a read-only name string.

### Scenario 2: UTF-8 preferred proper name usage
A caller provides an ASCII fallback name and a UTF-8 proper spelling. The module returns the UTF-8 spelling when that is the module-defined preferred result.

Test expectation:
- The Rust port returns the same selected variant as the C behavior for the same pair of inputs.

### Scenario 3: Stable pass-through behavior
A caller uses the module only as a selector and expects no mutation of inputs or side effects.

Test expectation:
- Input name data remains unchanged after the call.
- Repeated calls with the same inputs yield the same selected result.

### Scenario 4: Integration in higher-level output logic
A higher-level command or message-building path calls this module to obtain the preferred proper name string before composing user-visible text.

Test expectation:
- The returned string can be directly consumed by surrounding logic as the chosen proper name representation.
- No extra transformation step is required from this module beyond the documented selection behavior.

## Requirements

### Functional Requirements

#### FR-1: Two-input proper name selection
The module shall accept exactly two caller-provided name inputs representing:
- an ASCII form of the name, and
- a UTF-8 form of the name.

Traceability:
- `propername-lite.c`
- `proper_name_lite`

#### FR-2: Single selected result
The module shall produce one selected proper name result based solely on the two provided inputs.

Traceability:
- `propername-lite.c`
- `proper_name_lite`

#### FR-3: Read-only string result semantics
The module shall provide the selected result as a read-only string view/reference equivalent in behavior to the C function’s returned `char const *`.

Traceability:
- `propername-lite.c`
- `proper_name_lite`

#### FR-4: No input mutation
The module shall not modify the provided ASCII or UTF-8 name contents.

Traceability:
- `propername-lite.c`
- `proper_name_lite`

#### FR-5: Behavioral equivalence to source module
For the same input pair, the Rust rewrite shall return the same selected logical result as the C module.

Traceability:
- `propername-lite.c`
- `proper_name_lite`

### Key Entities

#### Entity: ASCII name input
A caller-provided string representing the name in ASCII form.

Relationship:
- Paired with the UTF-8 name input as one of the two candidate representations evaluated by the module.

Traceability:
- `proper_name_lite(name_ascii, ...)`

#### Entity: UTF-8 name input
A caller-provided string representing the name in UTF-8 form.

Relationship:
- Paired with the ASCII name input as one of the two candidate representations evaluated by the module.

Traceability:
- `proper_name_lite(..., name_utf8)`

#### Entity: Selected proper name result
The string chosen by the module for caller use.

Relationship:
- Derived from the two input entities.
- Returned directly to the caller as the module output.

Traceability:
- `proper_name_lite`

## Success Criteria

### SC-1: Public behavior coverage
The Rust module provides functionality corresponding to the source module’s sole evidenced public function, `proper_name_lite`.

Traceability:
- `propername-lite.c`
- `proper_name_lite`

### SC-2: Output equivalence
For all test cases derived from observed source behavior, the Rust implementation returns the same selected name variant as the C implementation.

Traceability:
- `propername-lite.c`
- `proper_name_lite`

### SC-3: Input preservation
Tests verify that calls do not alter the caller-provided ASCII or UTF-8 input contents.

Traceability:
- `propername-lite.c`
- `proper_name_lite`

### SC-4: Deterministic selection
Tests verify that repeated calls with identical inputs produce the same result.

Traceability:
- `propername-lite.c`
- `proper_name_lite`

### SC-5: Integration suitability
Tests verify that the returned selected name can be consumed directly by higher-level caller logic as the proper name string result.

Traceability:
- `propername-lite.c`
- `proper_name_lite`

## Out of Scope
The Rust rewrite specification does not require:
- any new public API beyond the evidenced module function,
- locale-specific policy beyond source behavior,
- ownership-transfer APIs,
- persistent storage,
- string normalization services,
- transliteration services,
- concurrency guarantees,
- serialization,
- FFI layers,
- performance benchmarking targets.

## Notes for Validation
Validation should compare Rust behavior against the C module at the function boundary using representative pairs of ASCII and UTF-8 name inputs. The acceptance focus is behavioral equivalence of selected output and preservation of caller-provided input data.