# spec.md

## Title
Rust Functional Specification for `module_gnu_vasnprintf.c_54`

## Metadata
- Project: `cflow-new`
- Module: `module_gnu_vasnprintf.c_54`
- Category: `module_cluster`
- Source file: `gnu/vasnprintf.c`
- Rust branch: `060-module_gnu_vasnprintf.c_54-rust-port`
- Generation date: `2026-06-11`

## Overview
This module provides format-planning support for GNU-style `vasnprintf` behavior by determining the maximum buffer room needed for a single formatted conversion, based on a parsed argument set and conversion metadata.

The Rust rewrite must preserve this functional role: given an argument description, conversion kind, formatting flags, width, precision state, and whether padding is handled internally, it must compute a safe upper bound for output space required for that conversion.

This specification is limited to functionality evidenced by:
- `MAX_ROOM_NEEDED` in `gnu/vasnprintf.c`
- the local anonymous struct identified in the module analysis

## Scope
In scope:
- Computing worst-case output room for one conversion step in formatting
- Using formatting parameters that affect output size
- Consulting argument metadata for the referenced argument

Out of scope:
- Full formatted output generation
- Parsing format strings
- Defining new public APIs beyond what is needed to preserve the evidenced module behavior
- Any capabilities not evidenced in the analyzed module input

## Feature Specification

### Feature: Maximum room estimation for a formatted conversion
The module must compute the maximum amount of output space required for a single conversion described by:
- the available arguments collection
- an argument index
- a conversion selector
- an argument type classification
- formatting flags
- field width
- precision presence and precision value
- whether padding is performed by this formatting step

The result must be a size value suitable for planning or reserving output capacity before formatting is performed.

### Supported behavioral dimensions
The room estimate must account for the formatting dimensions explicitly accepted by the source function:
- conversion kind
- argument type
- formatting flags
- minimum field width
- optional precision
- whether padding is applied by the conversion itself
- the referenced argument metadata from the argument collection

### Safety-oriented bound behavior
The returned size must be a maximum-room estimate rather than a minimal or exact rendering length. The Rust version must preserve this upper-bound semantics so that callers can rely on the value for capacity planning without underestimating required room.

## User Scenarios & Testing

### Scenario 1: Planning capacity for a numeric conversion
A formatting pipeline has already parsed a conversion and knows its argument type, width, flags, and precision. Before rendering, it asks this module for the maximum room needed for that conversion so it can ensure sufficient output capacity.

The Rust version must support:
- width-only planning
- width-plus-precision planning
- planning for different conversion selectors and numeric-like argument classifications, as represented by the source module inputs

### Scenario 2: Planning capacity when the argument affects formatted size
A formatting pipeline references an argument by index in the parsed arguments collection. The maximum room estimate depends on the argument information stored there. The module looks up the referenced argument metadata and incorporates it into the room estimate.

The Rust version must support:
- selecting the argument by index
- deriving the estimate from both conversion metadata and argument metadata

### Scenario 3: Planning where padding responsibility changes the estimate
A caller distinguishes between conversions that pad internally and those whose padding is handled elsewhere. The module must return a bound consistent with the provided `pad_ourselves` setting.

The Rust version must support:
- computing a bound with internal padding considered
- computing a bound when padding is not the responsibility of this step

### Scenario 4: Precision-sensitive capacity planning
A caller passes whether precision is present and, if so, its value. The estimate must change consistently with precision-sensitive conversions.

The Rust version must support:
- conversions with no precision specified
- conversions with precision specified
- precision values that enlarge the maximum output requirement

### Testing approach
The Rust rewrite must be tested with cases that vary:
- conversion selector
- argument type classification
- width
- precision presence
- precision value
- flags
- `pad_ourselves`
- argument index and corresponding argument metadata

Tests must verify that:
- the returned bound is deterministic for identical inputs
- changing any size-relevant formatting input can affect the estimate where applicable
- the result is never smaller than the actual formatted output length for the corresponding supported conversion behavior

## Requirements

### Functional Requirements

#### FR-1: Single-conversion room estimation
The module shall provide functionality equivalent to `MAX_ROOM_NEEDED` that computes a maximum output room value for one formatted conversion from the provided formatting inputs and referenced argument metadata.

Traceability:
- `gnu/vasnprintf.c`
- `MAX_ROOM_NEEDED`

#### FR-2: Argument-indexed lookup
The module shall use the supplied argument collection and argument index to determine the bound for the referenced conversion argument.

Traceability:
- `gnu/vasnprintf.c`
- `MAX_ROOM_NEEDED`

#### FR-3: Conversion-sensitive estimation
The module shall vary the maximum room estimate according to the supplied conversion selector.

Traceability:
- `gnu/vasnprintf.c`
- `MAX_ROOM_NEEDED`

#### FR-4: Type-sensitive estimation
The module shall vary the maximum room estimate according to the supplied argument type classification.

Traceability:
- `gnu/vasnprintf.c`
- `MAX_ROOM_NEEDED`

#### FR-5: Flag-sensitive estimation
The module shall incorporate formatting flags into the maximum room calculation when those flags affect formatted size.

Traceability:
- `gnu/vasnprintf.c`
- `MAX_ROOM_NEEDED`

#### FR-6: Width-sensitive estimation
The module shall incorporate the supplied field width into the maximum room calculation.

Traceability:
- `gnu/vasnprintf.c`
- `MAX_ROOM_NEEDED`

#### FR-7: Precision-sensitive estimation
The module shall incorporate both the presence of precision and the precision value into the maximum room calculation.

Traceability:
- `gnu/vasnprintf.c`
- `MAX_ROOM_NEEDED`

#### FR-8: Padding-responsibility sensitivity
The module shall incorporate the `pad_ourselves` condition into the maximum room calculation.

Traceability:
- `gnu/vasnprintf.c`
- `MAX_ROOM_NEEDED`

#### FR-9: Upper-bound semantics
The module shall return a maximum-room value intended for safe capacity planning and shall not intentionally underestimate required room for supported conversion behavior.

Traceability:
- `gnu/vasnprintf.c`
- `MAX_ROOM_NEEDED`

### Key Entities

#### Arguments collection
A formatting argument container supplies metadata or values for conversions by index. The room estimator consults this collection to derive size bounds for the referenced argument.

Traceability:
- `MAX_ROOM_NEEDED` parameter: `const arguments *ap`

#### Argument index
A positional selector identifies which argument in the arguments collection is relevant to the conversion being estimated.

Traceability:
- `MAX_ROOM_NEEDED` parameter: `size_t arg_index`

#### Conversion selector
A conversion designator identifies the formatting conversion whose output room is being estimated.

Traceability:
- `MAX_ROOM_NEEDED` parameter: `FCHAR_T conversion`

#### Argument type classification
A type descriptor identifies the kind of argument being formatted and influences the room bound.

Traceability:
- `MAX_ROOM_NEEDED` parameter: `arg_type type`

#### Formatting controls
Formatting controls consist of:
- flags
- width
- precision presence
- precision value
- padding responsibility

These controls collectively influence the maximum room estimate for the conversion.

Traceability:
- `MAX_ROOM_NEEDED` parameters:
  - `int flags`
  - `size_t width`
  - `int has_precision`
  - `size_t precision`
  - `int pad_ourselves`

#### Local anonymous struct
The analyzed source includes a local anonymous struct in `gnu/vasnprintf.c`. The Rust rewrite may model this only insofar as needed to preserve the evidenced behavior of room estimation. No independent externally visible behavior is specified for it from the available evidence.

Traceability:
- `gnu/vasnprintf.c:426-430`

## Success Criteria

### SC-1: Behavioral equivalence for bounded room calculation
For representative supported conversions and argument kinds, the Rust implementation returns a maximum-room value consistent with the source module’s role of precomputing required space for one conversion.

Traceability:
- `MAX_ROOM_NEEDED`

### SC-2: Width and precision coverage
Tests demonstrate that the Rust implementation correctly incorporates:
- width
- precision presence
- precision value

into the returned bound.

Traceability:
- `MAX_ROOM_NEEDED`

### SC-3: Conversion and type coverage
Tests demonstrate that differing conversion selectors and argument type classifications can produce differing room estimates where size behavior differs.

Traceability:
- `MAX_ROOM_NEEDED`

### SC-4: Padding-responsibility coverage
Tests demonstrate that the Rust implementation respects the `pad_ourselves` input in the returned bound.

Traceability:
- `MAX_ROOM_NEEDED`

### SC-5: No underestimation in tested scenarios
For tested supported formatting scenarios, the returned room value is greater than or equal to the actual produced output length for the corresponding conversion behavior.

Traceability:
- `MAX_ROOM_NEEDED`

### SC-6: Deterministic output
Repeated calls with identical arguments and formatting controls produce identical room estimates.

Traceability:
- `MAX_ROOM_NEEDED`

## Non-Goals
The Rust rewrite is not required by this specification to:
- expose a new standalone formatting API
- parse format strings
- perform complete string rendering
- add behavior beyond the evidenced maximum-room estimation role
- define semantics for module elements not evidenced by the analyzed input