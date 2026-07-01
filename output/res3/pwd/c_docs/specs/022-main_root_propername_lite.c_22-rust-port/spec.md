# spec.md

## Overview

- **Project**: `pwd`
- **Module**: `main_root_propername-lite.c_22`
- **Category**: `main_cluster`
- **Source file**: `propername-lite.c`
- **Primary interface**: `proper_name_lite(name_ascii, name_utf8) -> char const *`

## Feature Specification

This module provides a small name-selection function for presenting a proper name in either ASCII or UTF-8 form.

The Rust rewrite must implement the same observable behavior as the C module’s exposed function:

- Accept two input string references:
  - an ASCII form of a name
  - a UTF-8 form of the same name
- Return one of those provided name representations for display/use by callers.
- Preserve the module’s lightweight role: it is a selector between the provided representations, not a formatter, translator, validator, or allocator.

### Functional scope

The Rust version must support:

- choosing an appropriate proper-name representation from the two provided inputs
- handling the common case where both forms represent the same logical name
- returning an existing provided representation rather than constructing a new semantic value with altered content

### Out of scope

The Rust version must not introduce unevidenced functionality such as:

- transliteration beyond selecting between the two provided forms
- locale-sensitive formatting rules
- persistence or serialization
- new public APIs beyond what is needed to preserve the module’s role
- normalization, translation, or recovery behavior not evidenced by the source interface

## User Scenarios & Testing

### Scenario 1: ASCII-only safe presentation

A caller has an ASCII spelling of a proper name and also has a UTF-8 spelling, but the selected behavior for this lightweight module resolves to the ASCII-safe form.

**Expected support**:
- The function accepts both inputs.
- The returned string is exactly one of the supplied inputs.
- No additional characters are inserted or removed.

**Test focus**:
- Provide distinct ASCII and UTF-8 strings.
- Verify the result matches one of the inputs exactly.
- Verify the function does not produce a third, transformed string.

### Scenario 2: UTF-8 proper-name presentation

A caller provides both an ASCII fallback and a UTF-8 spelling containing non-ASCII characters for more accurate display of the proper name.

**Expected support**:
- The function accepts the UTF-8 name form without altering it.
- If the module behavior selects the UTF-8 form, the returned value matches that supplied UTF-8 string exactly.

**Test focus**:
- Use a UTF-8 name with non-ASCII characters.
- Verify exact content preservation if the UTF-8 form is selected.

### Scenario 3: Identical ASCII and UTF-8 inputs

A caller passes the same logical string in both parameters.

**Expected support**:
- The function succeeds with identical inputs.
- The result is that same name value.

**Test focus**:
- Pass equal strings for both parameters.
- Verify the returned string equals the provided name.

### Scenario 4: Integration as a helper in user-facing naming

Another part of the program needs a lightweight helper to obtain the proper display name from already-available ASCII and UTF-8 forms.

**Expected support**:
- The function can be called directly as a pure selector/helper in name presentation paths.
- Callers do not need to perform output post-processing to undo transformations by this module.

**Test focus**:
- Invoke the function from a higher-level formatting path.
- Confirm the returned value can be used directly as the chosen name string.

## Requirements

### Functional Requirements

#### FR-1: Dual-input proper-name selection
The module shall accept two string inputs representing the same proper name in ASCII and UTF-8 forms, respectively.

**Traceability**: `proper_name_lite(name_ascii, name_utf8)` in `propername-lite.c`

#### FR-2: Single-result selection
The module shall return a single string result representing the chosen proper-name form.

**Traceability**: return type and behavior of `proper_name_lite` in `propername-lite.c`

#### FR-3: Input-derived result
The returned result shall be derived from the provided inputs and shall correspond exactly to one of the supplied name forms, not to a newly reformatted semantic variant.

**Traceability**: selector-style interface of `proper_name_lite` in `propername-lite.c`

#### FR-4: Preservation of supplied content
When the module returns either the ASCII input or the UTF-8 input, the textual content of that selected input shall be preserved exactly.

**Traceability**: `proper_name_lite` output contract implied by char-pointer string selection in `propername-lite.c`

#### FR-5: Lightweight helper role
The module shall remain limited to proper-name representation choice and shall not perform unrelated name-processing tasks.

**Traceability**: sole exported functionality in `propername-lite.c` is `proper_name_lite`

### Key Entities

#### Proper name representations
The module operates on two input entities:

- **ASCII proper name**
  - A string containing the ASCII representation of the name.
- **UTF-8 proper name**
  - A string containing the UTF-8 representation of the same name.

#### Relationship
These two entities are alternate representations of one logical proper name. The module’s role is to choose which representation to return to the caller.

## Success Criteria

### Behavioral correctness

- The Rust module exposes behavior equivalent to the C module’s `proper_name_lite` interface in purpose: accepting ASCII and UTF-8 name forms and returning one selected string result.
- For every supported call, the returned value is exactly equal to either the provided ASCII input or the provided UTF-8 input.
- The Rust version does not emit transformed, concatenated, normalized, or otherwise newly synthesized name content.

### Scenario coverage

- Tests cover:
  - distinct ASCII and UTF-8 inputs
  - UTF-8 input containing non-ASCII characters
  - identical values for both inputs
  - use as a helper in a caller that consumes the returned name directly

### Port fidelity

- The Rust rewrite preserves the module’s narrow functional boundary as a lightweight proper-name selector.
- No additional public functional capability is required beyond what is evidenced by `proper_name_lite` in `propername-lite.c`.