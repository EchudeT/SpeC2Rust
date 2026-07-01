# spec.md

## Title

Functional Specification for `module_gnu_printf-parse.c_41` Rust Port

## Metadata

- **Project**: `cflow-new`
- **Module**: `module_gnu_printf-parse.c_41`
- **Category**: `module_cluster`
- **Source File**: `gnu/printf-parse.c`
- **Primary Function**: `PRINTF_PARSE`
- **Rust Branch**: `047-module_gnu_printf_parse.c_41-rust-port`
- **Generation Date**: `2026-06-11`

## Overview

This module parses a GNU-style `printf` format string and extracts formatting directives and argument usage information into caller-provided result structures.

The Rust rewrite must preserve the module’s observable parsing role:

- scan a format string,
- identify conversion directives embedded in literal text,
- record parsed directives into the directives result object,
- record argument usage information into the arguments result object,
- report success or failure through the function result.

This specification covers only the behavior evidenced by the module interface and file analysis. It does not introduce new public APIs or capabilities beyond parsing and result population.

## Feature Specification

### Feature Summary

The module provides format-string parsing for GNU `printf`-style formatting. It accepts an input format string and produces structured parse results describing:

- where directives occur,
- how directives are composed,
- which arguments are referenced by those directives.

The Rust version must implement equivalent functional behavior for the module’s parsing responsibility.

### In Scope

The Rust port must:

- accept a format string input compatible with the source module’s parsing entry point,
- parse literal text intermixed with `printf` directives,
- detect and interpret valid formatting directives supported by this module’s source behavior,
- populate a directives collection/result structure supplied by the caller,
- populate an arguments collection/result structure supplied by the caller,
- return an integer status indicating parse success or failure.

### Out of Scope

The Rust port must not add unevidenced behavior, including:

- formatting/output generation,
- emitting rendered strings,
- introducing a different public parsing API,
- argument value evaluation,
- thread-safety guarantees,
- serialization,
- recovery systems beyond the source module’s success/failure contract.

## User Scenarios & Testing

### Scenario 1: Parse a format string with no conversion directives

A caller passes a format string containing only literal text.

Expected support:

- parsing completes successfully,
- the directives result indicates that no formatting directives were found,
- the arguments result indicates that no format arguments are required.

### Scenario 2: Parse a format string with one conversion directive

A caller passes a format string containing literal text and a single valid `printf` conversion.

Expected support:

- the parser identifies exactly one directive,
- the directive is recorded in the directives result,
- any argument consumed by the directive is recorded in the arguments result,
- the function reports success.

### Scenario 3: Parse a format string with multiple directives

A caller passes a format string containing multiple valid conversions.

Expected support:

- all directives are discovered in input order,
- each directive is recorded without losing earlier directives,
- argument usage across directives is accumulated into the arguments result,
- the function reports success.

### Scenario 4: Parse a format string with argument-dependent directive components

A caller passes a format string whose directives consume arguments not only for final conversion values but also for directive-controlled components such as width or precision when supported by the source behavior.

Expected support:

- the parser records argument usage for all directive-consumed arguments evidenced by the format string,
- directive metadata and argument metadata remain consistent with each other,
- the function reports success when the format string is valid.

### Scenario 5: Parse an invalid or unsupported format sequence

A caller passes a malformed format string or a sequence that the source parser treats as a parse failure.

Expected support:

- the function returns failure status,
- the parser does not claim successful directive parsing for invalid input.

### Testing Guidance

The Rust version should be tested with cases covering:

- empty format string,
- literal-only format string,
- single-directive input,
- multiple-directive input,
- mixed literal and directive text,
- directives that reference arguments through different directive components,
- malformed directives that must fail,
- boundary cases where directives appear at the start or end of the format string,
- repeated percent signs or escaped percent behavior as accepted by the source parser.

Tests must verify both status return and structured result contents.

## Requirements

### Functional Requirements

#### FR-1: Format String Parsing Entry

The module shall provide the parsing behavior represented by `PRINTF_PARSE`, accepting:

- a format string input,
- a caller-provided directives result object,
- a caller-provided arguments result object,
- and returning an integer status.

**Traceability**: `gnu/printf-parse.c`, `PRINTF_PARSE`

#### FR-2: Sequential Scan of Input Format

The module shall examine the complete format string and distinguish literal text from formatting directives.

**Traceability**: `gnu/printf-parse.c`, `PRINTF_PARSE`

#### FR-3: Directive Identification

The module shall recognize formatting directives present in the input and record them in the directives result object.

**Traceability**: `gnu/printf-parse.c`, `PRINTF_PARSE`, `DIRECTIVES`

#### FR-4: Directive Structure Extraction

For each recognized directive, the module shall extract and preserve the directive information needed by downstream consumers of the parse results, as represented by the directives result structure.

**Traceability**: `gnu/printf-parse.c`, `PRINTF_PARSE`, `DIRECTIVES`

#### FR-5: Argument Usage Extraction

The module shall determine which arguments are referenced by parsed directives and record that information in the arguments result object.

**Traceability**: `gnu/printf-parse.c`, `PRINTF_PARSE`, `arguments`

#### FR-6: Aggregation Across Multiple Directives

When a format string contains multiple directives, the module shall accumulate directive and argument information across the full input rather than only for the first directive.

**Traceability**: `gnu/printf-parse.c`, `PRINTF_PARSE`, `DIRECTIVES`, `arguments`

#### FR-7: Success/Failure Reporting

The module shall return a status value that allows the caller to distinguish successful parsing from parse failure.

**Traceability**: `gnu/printf-parse.c`, `PRINTF_PARSE`

#### FR-8: Caller-Visible Results on Success

On successful parsing, the directives and arguments result objects shall reflect the format string content parsed by the module.

**Traceability**: `gnu/printf-parse.c`, `PRINTF_PARSE`, `DIRECTIVES`, `arguments`

#### FR-9: Invalid Input Rejection

If the format string contains a malformed or otherwise rejected directive sequence according to source behavior, the module shall report failure rather than reporting a successful parse.

**Traceability**: `gnu/printf-parse.c`, `PRINTF_PARSE`

### Key Entities

#### `format` input

The input format string is the source text to be parsed. It contains literal text and zero or more `printf` formatting directives.

**Relationship**:
- consumed by `PRINTF_PARSE`,
- drives creation of directive records and argument usage records.

#### `DIRECTIVES`

A caller-provided result structure that receives the parsed directive information for the input format string.

**Relationship**:
- populated by `PRINTF_PARSE`,
- contains information derived from one or more directives discovered in the format string,
- corresponds to directive-level parse output.

#### `arguments`

A caller-provided result structure that receives argument usage information inferred from parsed directives.

**Relationship**:
- populated by `PRINTF_PARSE`,
- reflects arguments consumed by directive parsing,
- complements `DIRECTIVES` by representing argument-level parse output.

#### `PRINTF_PARSE`

The module’s parsing operation.

**Relationship**:
- reads the `format` input,
- writes parse results into `DIRECTIVES`,
- writes argument usage into `arguments`,
- returns parse status.

## Success Criteria

### SC-1: Literal-Only Success

Given a valid literal-only format string, the Rust implementation returns success and produces result objects indicating no parsed directives and no required arguments.

**Traceability**: FR-1, FR-2, FR-8

### SC-2: Single-Directive Recording

Given a valid format string containing one directive, the Rust implementation returns success, records one directive in the directives result, and records the corresponding argument usage in the arguments result.

**Traceability**: FR-3, FR-4, FR-5, FR-8

### SC-3: Multiple-Directive Accumulation

Given a valid format string containing multiple directives, the Rust implementation returns success and preserves complete directive and argument information across all directives in input order.

**Traceability**: FR-5, FR-6, FR-8

### SC-4: Argument-Dependent Component Handling

Given a valid format string in which directive components consume arguments, the Rust implementation records those argument uses consistently in the arguments result and keeps directive metadata aligned with them.

**Traceability**: FR-4, FR-5, FR-8

### SC-5: Invalid Format Failure

Given malformed input that the source parser rejects, the Rust implementation returns failure and does not misreport the input as successfully parsed.

**Traceability**: FR-7, FR-9

### SC-6: Interface Preservation

The Rust rewrite preserves the module’s functional contract as a parsing unit: one format string input, caller-supplied directive and argument result objects, and integer-like success/failure status semantics.

**Traceability**: FR-1, FR-7

## Acceptance Notes

- Conformance is based on matching the source module’s parsing behavior at the functional level.
- The Rust rewrite may change internal implementation details, but it must not change the module’s externally observable parsing responsibilities.
- Any behavior not evidenced by `gnu/printf-parse.c` and `PRINTF_PARSE` is outside this specification.