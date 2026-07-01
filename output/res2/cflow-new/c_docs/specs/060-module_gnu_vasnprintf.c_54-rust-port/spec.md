# spec.md

## Title

Functional Specification for Rust Port of `module_gnu_vasnprintf.c_54`

## Metadata

- Project: `cflow-new`
- Module: `module_gnu_vasnprintf.c_54`
- Category: `module_cluster`
- Source file: `gnu/vasnprintf.c`
- Rust branch: `060-module_gnu_vasnprintf.c_54-rust-port`
- Generation date: 2026-06-17

## Overview

This module provides format-size planning behavior for GNU-style variadic string formatting, specifically the computation of the maximum output room needed for a single conversion under the active formatting parameters.

The Rust rewrite must preserve the module behavior evidenced by `MAX_ROOM_NEEDED`: given parsed formatting context, argument metadata, conversion kind, field width and precision information, and padding ownership rules, determine an upper bound on the number of output characters required for that conversion.

This specification is limited to the functional boundary evidenced by the analyzed source input and does not define unrelated formatting features beyond this size-computation responsibility.

## Feature Specification

### Summary

The Rust version must implement output-capacity estimation for a formatting conversion. The estimation must account for:

- the argument set and current argument index,
- conversion character/category,
- argument type,
- formatting flags,
- minimum field width,
- optional precision,
- whether the formatter itself performs padding.

The result must be a maximum room value expressed as a `size_t`-equivalent unsigned size.

### In-Scope Functionality

The Rust module must:

1. Accept the formatting-planning inputs corresponding to those consumed by `MAX_ROOM_NEEDED`.
2. Compute a maximum required output length for the addressed conversion.
3. Include the effect of conversion-specific formatting parameters on the bound.
4. Distinguish behavior by conversion kind and argument type.
5. Support argument-aware estimation through access to the parsed argument collection and selected argument position.
6. Respect the presence or absence of precision.
7. Respect width and flag effects when they can increase the required room.
8. Respect whether padding is handled internally by this formatting step or externally.

### Out-of-Scope

The Rust specification does not require new capabilities not evidenced by the source analysis, including:

- creation of a new public formatting API unrelated to this sizing role,
- guarantees beyond maximum-room computation,
- unrelated parsing, I/O, persistence, concurrency, or FFI behavior.

## User Scenarios & Testing

### Scenario 1: Planning buffer growth before formatting a conversion

A higher-level formatter has already parsed a format directive and needs to know how much output space may be needed before rendering one argument.

- Input includes the argument list, selected argument position, conversion specifier, width, flags, and optional precision.
- The module returns a conservative maximum character count.
- The caller uses this value to decide whether the destination buffer is large enough.

**Test expectation:** For representative conversion cases, the Rust port returns a non-underestimating bound consistent with the source module’s behavior.

### Scenario 2: Width-driven expansion

A format directive applies a field width that may exceed the natural formatted length.

- The caller provides a width and indicates whether this formatting step applies the padding itself.
- The module incorporates width when width can dominate output size.

**Test expectation:** The returned maximum is at least the specified width when internal padding responsibility requires width to be reflected.

### Scenario 3: Precision-sensitive estimation

A format directive includes precision, affecting the possible number of emitted characters.

- The caller marks precision as present and provides its numeric value.
- The module uses this to compute a bound appropriate to the conversion and type.

**Test expectation:** Precision-bearing cases produce bounds that reflect precision-dependent growth or restriction according to source behavior.

### Scenario 4: Different argument types under similar conversions

Two directives may use similar formatting syntax but refer to arguments of different internal types.

- The caller supplies the argument type and index.
- The module derives the bound using both the conversion kind and argument typing context.

**Test expectation:** Type-distinct cases that differ in maximum output need are distinguished by the Rust implementation.

### Scenario 5: Positional access into parsed arguments

A formatter using pre-parsed arguments references a specific argument index for a conversion.

- The module receives the argument collection and argument index.
- The computed bound corresponds to the referenced argument rather than an implicit sequential state.

**Test expectation:** Changing only the argument index can change the estimated maximum where the referenced arguments differ.

## Requirements

### Functional Requirements

- **FR-1**: The module shall compute and return a maximum required output size for one formatting conversion.
  **Traceability:** `gnu/vasnprintf.c`, `MAX_ROOM_NEEDED`

- **FR-2**: The module shall accept argument-context input sufficient to inspect the referenced formatting argument through an argument collection and argument index.
  **Traceability:** `gnu/vasnprintf.c`, `MAX_ROOM_NEEDED`

- **FR-3**: The module shall vary the computed maximum according to the conversion designator/category supplied for the formatting operation.
  **Traceability:** `gnu/vasnprintf.c`, `MAX_ROOM_NEEDED`

- **FR-4**: The module shall vary the computed maximum according to the supplied argument type.
  **Traceability:** `gnu/vasnprintf.c`, `MAX_ROOM_NEEDED`

- **FR-5**: The module shall account for formatting flags when those flags influence the maximum output room.
  **Traceability:** `gnu/vasnprintf.c`, `MAX_ROOM_NEEDED`

- **FR-6**: The module shall account for field width when width can increase the required room.
  **Traceability:** `gnu/vasnprintf.c`, `MAX_ROOM_NEEDED`

- **FR-7**: The module shall distinguish between the absence and presence of precision and shall use the precision value when present.
  **Traceability:** `gnu/vasnprintf.c`, `MAX_ROOM_NEEDED`

- **FR-8**: The module shall account for whether padding is performed by this formatting step, and shall incorporate or omit padding effects accordingly.
  **Traceability:** `gnu/vasnprintf.c`, `MAX_ROOM_NEEDED`

- **FR-9**: The module shall return the computed maximum as an unsigned size value compatible with `size_t` semantics.
  **Traceability:** `gnu/vasnprintf.c`, `MAX_ROOM_NEEDED`

### Key Entities

- **Argument collection (`arguments`)**
  Represents the parsed set of formatting arguments available to the sizing routine. It is queried together with an argument index to determine the context for the target conversion.
  **Traceability:** `gnu/vasnprintf.c`, `MAX_ROOM_NEEDED`

- **Argument index**
  Identifies which argument in the argument collection is being sized for the current conversion.
  **Traceability:** `gnu/vasnprintf.c`, `MAX_ROOM_NEEDED`

- **Conversion kind (`FCHAR_T conversion`)**
  Identifies the conversion being planned and drives conversion-specific maximum-room behavior.
  **Traceability:** `gnu/vasnprintf.c`, `MAX_ROOM_NEEDED`

- **Argument type (`arg_type type`)**
  Describes the effective type classification of the referenced argument for estimation purposes.
  **Traceability:** `gnu/vasnprintf.c`, `MAX_ROOM_NEEDED`

- **Formatting controls (`flags`, `width`, `has_precision`, `precision`, `pad_ourselves`)**
  Captures the formatting modifiers that can alter the maximum output room for the conversion.
  **Traceability:** `gnu/vasnprintf.c`, `MAX_ROOM_NEEDED`

- **Anonymous struct at `gnu/vasnprintf.c:426-430`**
  A local structural type present in the source module. The Rust rewrite must preserve any functional role this structure serves within the sizing logic, but no broader external contract is evidenced by the analysis input.
  **Traceability:** `gnu/vasnprintf.c:426-430`

## Success Criteria

- **SC-1**: For all conversion-planning cases covered by the source module’s sizing logic, the Rust port returns a deterministic unsigned size result from the same effective inputs.
  **Traceability:** `gnu/vasnprintf.c`, `MAX_ROOM_NEEDED`

- **SC-2**: In validation cases where width is the dominant formatting constraint and padding is applied by the module, the Rust port’s reported maximum is not less than the required field width.
  **Traceability:** `gnu/vasnprintf.c`, `MAX_ROOM_NEEDED`

- **SC-3**: In validation cases that differ only by precision presence or precision value, the Rust port reflects the same precision-sensitive bound behavior as the source module.
  **Traceability:** `gnu/vasnprintf.c`, `MAX_ROOM_NEEDED`

- **SC-4**: In validation cases that differ by conversion kind or argument type, the Rust port distinguishes those cases wherever the source logic distinguishes them.
  **Traceability:** `gnu/vasnprintf.c`, `MAX_ROOM_NEEDED`

- **SC-5**: For all tested cases derived from the source behavior, the Rust port never underestimates the maximum room needed for the addressed conversion.
  **Traceability:** `gnu/vasnprintf.c`, `MAX_ROOM_NEEDED`

- **SC-6**: The Rust implementation remains confined to the evidenced module responsibility of maximum-room computation and does not require unrelated external behaviors to satisfy this specification.
  **Traceability:** `gnu/vasnprintf.c`, `MAX_ROOM_NEEDED`