# spec.md

## Title

Functional Specification for `module_gnu_vasnprintf.c_54` Rust Port

## Metadata

- Project: `cflow-new`
- Module: `module_gnu_vasnprintf.c_54`
- Category: `module_cluster`
- Source file: `gnu/vasnprintf.c`
- Rust branch: `060-module_gnu_vasnprintf.c_54-rust-port`
- Generation date: `2026-06-17`

## Overview

This module provides bounded formatting support within the `vasnprintf` formatting pipeline by determining the maximum buffer space required for a single formatted argument under a specific conversion context.

The analyzed functional boundary in this module is the computation performed by `MAX_ROOM_NEEDED`, which evaluates a formatting request using:

- the argument set,
- the selected argument index,
- the conversion character,
- the argument type,
- formatting flags,
- field width,
- optional precision,
- and whether padding is performed by the module itself,

and returns the maximum number of output characters that may be needed.

The Rust rewrite must preserve this behavior as an internal formatting-capacity calculation used to safely size or grow output storage for formatted results.

## Feature Specification

### Feature: Maximum formatted-room calculation

The Rust module must implement logic equivalent to the source module’s capacity-estimation behavior for a single conversion step in a `vasnprintf`-style formatter.

This feature must:

- accept the formatting context associated with one conversion;
- inspect the referenced argument and its declared formatting type;
- account for conversion kind, flags, width, precision, and padding responsibility;
- produce a maximum required output length as a `size_t`-equivalent unsigned size value;
- support use as a conservative upper bound for subsequent formatted output generation.

### Feature boundaries

The Rust port must cover only the evidenced functional boundary of this module analysis:

- maximum room estimation for a formatted argument.

The Rust port must not introduce unevidenced capabilities such as:

- a new public formatting API,
- cross-module formatting orchestration beyond this calculation role,
- serialization or persistence behavior,
- thread-safety guarantees,
- recovery workflows,
- FFI surfaces.

## User Scenarios & Testing

### Scenario 1: Estimating room before formatting an argument

A formatting pipeline prepares to render one argument from an argument list. Before writing output, it needs a safe upper bound for how many characters this argument may emit under the requested conversion and formatting options.

The Rust module must support this by returning a nonnegative size estimate that can be used to reserve or validate output capacity.

**Test focus:**
- verify that a size value is returned for each supported invocation shape;
- verify that the result is sufficient as an upper bound for the eventual conversion handled by the surrounding formatter.

### Scenario 2: Width affects required room

A conversion is requested with a field width larger than the basic textual representation of the argument.

The Rust module must include field width in its estimate whenever width can enlarge output.

**Test focus:**
- compare estimates with and without width;
- confirm that increasing width does not decrease the computed maximum room.

### Scenario 3: Precision affects required room

A conversion is requested with an explicit precision. Precision may increase or constrain the amount of output required depending on conversion semantics.

The Rust module must account for whether precision is present and use the precision value in the estimate.

**Test focus:**
- evaluate equivalent formatting contexts with `has_precision` off and on;
- confirm that the estimate reflects the precision-bearing case conservatively.

### Scenario 4: Conversion kind and argument type change the estimate

Different conversion characters and argument types can require different output bounds.

The Rust module must vary its estimate according to conversion kind and argument type rather than using one fixed size for all arguments.

**Test focus:**
- compare estimates across multiple conversion/type combinations;
- confirm that distinct combinations can yield distinct upper bounds.

### Scenario 5: Padding responsibility changes the estimate

The formatting pipeline may either perform padding within this conversion step or delegate padding elsewhere.

The Rust module must account for the `pad_ourselves` condition when calculating maximum room.

**Test focus:**
- compare otherwise identical invocations with `pad_ourselves` enabled and disabled;
- verify that the result matches the module’s padding-responsibility semantics.

### Scenario 6: Argument selection by index

The formatting pipeline selects an argument by index from the argument collection for the current conversion.

The Rust module must use the provided argument index to inspect the corresponding argument information when deriving the upper bound.

**Test focus:**
- invoke estimation using different valid argument indexes in the same argument collection;
- verify that the estimate tracks the selected argument rather than unrelated entries.

## Requirements

### Functional Requirements

#### FR-1: Single-conversion maximum-room estimation
The module shall compute the maximum output room needed for one formatted conversion instance, based on the conversion context supplied to the calculation.

**Traceability:** `gnu/vasnprintf.c`, `MAX_ROOM_NEEDED`

#### FR-2: Argument-aware estimation
The module shall derive the estimate using the provided argument collection and the selected argument index.

**Traceability:** `gnu/vasnprintf.c`, `MAX_ROOM_NEEDED`

#### FR-3: Conversion-aware estimation
The module shall incorporate the conversion character into the estimate.

**Traceability:** `gnu/vasnprintf.c`, `MAX_ROOM_NEEDED`

#### FR-4: Type-aware estimation
The module shall incorporate the argument’s formatting type into the estimate.

**Traceability:** `gnu/vasnprintf.c`, `MAX_ROOM_NEEDED`

#### FR-5: Flag-aware estimation
The module shall incorporate formatting flags into the estimate when those flags can affect required output length.

**Traceability:** `gnu/vasnprintf.c`, `MAX_ROOM_NEEDED`

#### FR-6: Width-aware estimation
The module shall incorporate field width into the estimate when width can affect required output length.

**Traceability:** `gnu/vasnprintf.c`, `MAX_ROOM_NEEDED`

#### FR-7: Precision-aware estimation
The module shall distinguish between absence and presence of precision using both the precision-presence indicator and the precision value, and shall incorporate them into the estimate when precision can affect required output length.

**Traceability:** `gnu/vasnprintf.c`, `MAX_ROOM_NEEDED`

#### FR-8: Padding-mode-aware estimation
The module shall incorporate whether padding is handled by this formatting step (`pad_ourselves`) into the estimate.

**Traceability:** `gnu/vasnprintf.c`, `MAX_ROOM_NEEDED`

#### FR-9: Unsigned size result
The module shall return the estimate as an unsigned size value suitable for output-buffer sizing decisions.

**Traceability:** `gnu/vasnprintf.c`, `MAX_ROOM_NEEDED`

#### FR-10: Conservative upper-bound behavior
The module shall produce a conservative maximum-room result intended for safe capacity planning by the surrounding formatting logic.

**Traceability:** `gnu/vasnprintf.c`, `MAX_ROOM_NEEDED`

### Key Entities

#### `arguments`
A collection of formatting arguments supplied to the formatting pipeline. The room-estimation logic reads from this collection using an argument index to determine characteristics relevant to output size.

**Relationship:** Used as the source of argument data for the maximum-room calculation.

**Traceability:** `gnu/vasnprintf.c`, `MAX_ROOM_NEEDED`

#### Conversion character (`FCHAR_T conversion`)
The conversion specifier identifying the formatting category for the current argument.

**Relationship:** Combined with argument type and formatting options to determine the required room bound.

**Traceability:** `gnu/vasnprintf.c`, `MAX_ROOM_NEEDED`

#### Argument type (`arg_type type`)
The formatting-time type classification of the selected argument.

**Relationship:** Interpreted together with the conversion character to derive the estimate.

**Traceability:** `gnu/vasnprintf.c`, `MAX_ROOM_NEEDED`

#### Formatting options
The calculation consumes:
- `flags`
- `width`
- `has_precision`
- `precision`
- `pad_ourselves`

**Relationship:** These values modify the upper bound for the selected conversion.

**Traceability:** `gnu/vasnprintf.c`, `MAX_ROOM_NEEDED`

#### Anonymous struct at `gnu/vasnprintf.c:426-430`
A local core structure identified by the analysis as part of the module’s data model.

**Relationship:** This structure exists within the source module’s internal data domain; the Rust rewrite must preserve any behaviorally necessary role it serves within this module boundary, but no additional external contract is evidenced by the analysis input.

**Traceability:** `gnu/vasnprintf.c:426-430`

## Success Criteria

### SC-1: Behavioral coverage
The Rust port provides a maximum-room calculation for a single formatting conversion using the same input categories evidenced in the source analysis: argument collection, argument index, conversion, type, flags, width, precision state, precision value, and padding mode.

**Traceability:** `gnu/vasnprintf.c`, `MAX_ROOM_NEEDED`

### SC-2: Conservative sizing
For supported formatting cases exercised by module tests, the returned size is never smaller than the actual number of output characters required by the corresponding conversion in the surrounding formatting logic.

**Traceability:** `gnu/vasnprintf.c`, `MAX_ROOM_NEEDED`

### SC-3: Width sensitivity
Tests demonstrate that width-bearing cases are handled and that width-dependent estimates remain valid upper bounds.

**Traceability:** `gnu/vasnprintf.c`, `MAX_ROOM_NEEDED`

### SC-4: Precision sensitivity
Tests demonstrate that precision-bearing cases are handled and that precision-dependent estimates remain valid upper bounds.

**Traceability:** `gnu/vasnprintf.c`, `MAX_ROOM_NEEDED`

### SC-5: Conversion and type sensitivity
Tests demonstrate that multiple conversion/type combinations are accepted and can produce distinct valid estimates where formatting semantics differ.

**Traceability:** `gnu/vasnprintf.c`, `MAX_ROOM_NEEDED`

### SC-6: Padding-mode sensitivity
Tests demonstrate that the estimate reflects the `pad_ourselves` mode in otherwise equivalent formatting contexts.

**Traceability:** `gnu/vasnprintf.c`, `MAX_ROOM_NEEDED`

### SC-7: Argument-index sensitivity
Tests demonstrate that selecting different valid argument indexes from the same argument collection can affect the estimate according to the selected argument.

**Traceability:** `gnu/vasnprintf.c`, `MAX_ROOM_NEEDED`

### SC-8: No unsupported scope expansion
The Rust module does not claim or require new external capabilities beyond the evidenced room-estimation role of this source module boundary.

**Traceability:** `gnu/vasnprintf.c`, `MAX_ROOM_NEEDED`