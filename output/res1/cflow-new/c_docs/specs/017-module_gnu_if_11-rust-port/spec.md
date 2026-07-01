# spec.md

## Title

Functional Specification: `module_gnu_if_11` Rust Port

## Document Control

- **Project**: `cflow-new`
- **Module**: `module_gnu_if_11`
- **Category**: `module_cluster`
- **Source Basis**: `gnu/vasnprintf.c`
- **Rust Branch**: `017-module_gnu_if_11-rust-port`
- **Generation Date**: 2026-06-11

## Overview

This module is part of the formatted-output processing logic in `gnu/vasnprintf.c`. The analyzed behavior is centered on conditional handling for specific format conversions:

- character conversion handling for `'c'`, with behavior dependent on whether the argument is a wide character type
- floating-point general-format handling for `'g'` and `'G'`

The Rust rewrite must preserve the functional decision-making associated with these conversion cases within the module’s formatting flow. The specification is limited to behavior evidenced by the analyzed source locations and related data structures.

## Scope

### In Scope

- Conditional recognition of `'c'` conversion cases in which the referenced argument is **not** a wide character argument
- Conditional recognition of `'g'` and `'G'` conversion cases
- Use of parsed conversion metadata and argument type metadata to drive those decisions
- Preservation of observable formatting-path selection implied by these conditions

### Out of Scope

- Defining new formatting features beyond the evidenced conversion handling
- New public APIs not required by the source behavior
- Unrelated conversion categories not evidenced in the analyzed module results
- Non-functional guarantees not evidenced by the source basis

## Feature Specification

### Feature: Conversion-Specific Formatting Path Selection

The module participates in formatted string generation by selecting behavior according to a parsed conversion descriptor and associated argument metadata.

The Rust version must implement the following evidenced functionality:

1. **Narrow character conversion discrimination**
   - When the active conversion is `'c'`, the module must distinguish between:
     - arguments identified as wide character type
     - arguments not identified as wide character type
   - The non-wide-character case must be treated as a distinct formatting path.

2. **General floating-point conversion recognition**
   - When the active conversion is `'g'` or `'G'`, the module must recognize these as belonging to the same decision group for formatting-path purposes.
   - The Rust version must preserve any shared handling implied by the original grouped condition.

3. **Decision-making driven by parsed descriptors**
   - Conversion handling must be based on conversion metadata from a parsed directive-like structure and argument type information indexed through that descriptor.

4. **Behavior preservation within formatting flow**
   - The Rust rewrite must preserve the original module’s branching behavior for these cases so that the same input descriptor and argument-type combinations reach equivalent functional handling categories.

## User Scenarios & Testing

### Scenario 1: Formatting a non-wide character with `%c`

A caller provides a format directive whose conversion is `'c'`, and the referenced argument is classified as not being a wide character.

**Expected support in Rust version**
- The module identifies the conversion as `'c'`.
- It checks the argument type metadata for the referenced argument.
- It routes processing through the non-wide-character `'c'` handling path.

**Test focus**
- Parsed conversion value is `'c'`
- Argument type is any non-`TYPE_WIDE_CHAR` value
- Resulting branch selection matches the original module behavior

### Scenario 2: Formatting a wide character with `%c`

A caller provides a format directive whose conversion is `'c'`, and the referenced argument is classified as a wide character.

**Expected support in Rust version**
- The module identifies the conversion as `'c'`.
- It checks the argument type metadata.
- It does **not** use the non-wide-character `'c'` path reserved for non-wide arguments.

**Test focus**
- Parsed conversion value is `'c'`
- Argument type is `TYPE_WIDE_CHAR`
- Branch behavior differs from the non-wide-character case

### Scenario 3: Formatting with `%g`

A caller provides a format directive whose conversion is `'g'`.

**Expected support in Rust version**
- The module recognizes `'g'` as part of the general floating-point conversion group.
- It routes processing through the same decision group used for `'G'`.

**Test focus**
- Parsed conversion value is `'g'`
- Branch selection matches the shared `'g'`/`'G'` handling category

### Scenario 4: Formatting with `%G`

A caller provides a format directive whose conversion is `'G'`.

**Expected support in Rust version**
- The module recognizes `'G'` as part of the same general floating-point conversion group as `'g'`.
- It routes processing equivalently at the decision level.

**Test focus**
- Parsed conversion value is `'G'`
- Branch selection matches the shared `'g'`/`'G'` handling category

### Scenario 5: Indexed argument lookup for conversion handling

A caller uses a parsed directive that references an argument by index, and the module must determine handling based on both conversion code and the referenced argument’s type.

**Expected support in Rust version**
- The module reads the directive’s conversion field.
- The module reads the directive’s argument index.
- The module evaluates the corresponding argument type entry before selecting the handling path.

**Test focus**
- Conversion descriptor and argument table are both required
- Branch selection changes when only the argument type at the indexed position changes

## Requirements

### Functional Requirements

- **FR-1**: The module shall evaluate the active conversion code from parsed formatting metadata when determining behavior.
  **Traceability**: `gnu/vasnprintf.c`, analyzed conditions at lines 3560-3628 and 5301-5551.

- **FR-2**: The module shall recognize the `'c'` conversion as a special case whose handling depends on the referenced argument’s type classification.
  **Traceability**: `gnu/vasnprintf.c`, analyzed condition at lines 3560-3628.

- **FR-3**: For `'c'` conversion, the module shall distinguish arguments whose type is `TYPE_WIDE_CHAR` from arguments whose type is not `TYPE_WIDE_CHAR`.
  **Traceability**: `gnu/vasnprintf.c`, analyzed condition at lines 3560-3628.

- **FR-4**: For `'c'` conversion with an argument type other than `TYPE_WIDE_CHAR`, the module shall select the non-wide-character handling path represented by the analyzed branch.
  **Traceability**: `gnu/vasnprintf.c`, analyzed condition at lines 3560-3628.

- **FR-5**: The module shall obtain the type classification for a conversion from the argument metadata entry referenced by the parsed argument index.
  **Traceability**: `gnu/vasnprintf.c`, analyzed condition at lines 3560-3628.

- **FR-6**: The module shall recognize both `'g'` and `'G'` conversion codes as members of the same decision group for formatting-path selection.
  **Traceability**: `gnu/vasnprintf.c`, analyzed condition at lines 5301-5551.

- **FR-7**: The module shall preserve equivalent branch-selection behavior for the `'g'` and `'G'` cases in the Rust rewrite.
  **Traceability**: `gnu/vasnprintf.c`, analyzed condition at lines 5301-5551.

### Key Entities

- **Conversion Descriptor**
  - A parsed formatting descriptor containing at least:
    - a conversion code
    - an argument index
  - This entity drives which formatting branch is selected.

  **Traceability**: Access patterns in `gnu/vasnprintf.c` conditions using `dp->conversion` and `dp->arg_index`.

- **Argument Metadata Collection**
  - A collection of argument descriptors indexed by argument position.
  - Each entry includes a type classification used for conversion-specific decisions.

  **Traceability**: Access pattern `a.arg[dp->arg_index].type` in `gnu/vasnprintf.c`.

- **Argument Type Classification**
  - A type category associated with an argument entry.
  - Includes the specific classification `TYPE_WIDE_CHAR`, which is used to differentiate `'c'` conversion behavior.

  **Traceability**: `gnu/vasnprintf.c`, analyzed condition at lines 3560-3628.

- **Anonymous Struct in Source**
  - The source includes an anonymous struct identified in the analyzed data.
  - In the Rust port, any corresponding representation must preserve the fields and relationships necessary to support conversion and argument-type-based decisions evidenced above.

  **Traceability**: `gnu/vasnprintf.c:426-430`, anonymous `struct`.

## Success Criteria

- **SC-1**: Given a conversion descriptor with conversion `'c'` and an indexed argument whose type is not `TYPE_WIDE_CHAR`, the Rust module selects the same functional handling category as the original non-wide-character `'c'` branch.
  **Traceability**: `gnu/vasnprintf.c`, lines 3560-3628.

- **SC-2**: Given a conversion descriptor with conversion `'c'` and an indexed argument whose type is `TYPE_WIDE_CHAR`, the Rust module does not select the non-wide-character `'c'` branch.
  **Traceability**: `gnu/vasnprintf.c`, lines 3560-3628.

- **SC-3**: Given otherwise equivalent inputs, changing only the indexed argument type between `TYPE_WIDE_CHAR` and a non-`TYPE_WIDE_CHAR` type changes branch selection for `'c'` conversion accordingly.
  **Traceability**: `gnu/vasnprintf.c`, lines 3560-3628.

- **SC-4**: Given a conversion descriptor with conversion `'g'`, the Rust module selects the same decision group as for conversion `'G'`.
  **Traceability**: `gnu/vasnprintf.c`, lines 5301-5551.

- **SC-5**: Tests covering `'g'` and `'G'` confirm equivalent decision-level handling in the Rust rewrite.
  **Traceability**: `gnu/vasnprintf.c`, lines 5301-5551.

- **SC-6**: Tests confirm that conversion handling decisions are based on both conversion code and indexed argument metadata where the source behavior requires both.
  **Traceability**: `gnu/vasnprintf.c`, lines 3560-3628 and 5301-5551.

## Acceptance Notes

- Acceptance is based on behavioral equivalence for the evidenced conversion-dispatch cases only.
- The Rust rewrite may reorganize internal structure, but it must preserve the decision semantics defined in this specification.
- Any unsupported claims beyond the traced behaviors are excluded from acceptance.