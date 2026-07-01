# spec.md

## Title

Rust Functional Specification for `module_gnu_printf-parse.c_41`

## Metadata

- Project: `cflow-new`
- Module: `module_gnu_printf-parse.c_41`
- Category: `module_cluster`
- Source file: `gnu/printf-parse.c`
- Primary function: `PRINTF_PARSE`
- Target branch: `047-module_gnu_printf_parse.c_41-rust-port`
- Generation date: `2026-06-17`

## Overview

This module is responsible for parsing a `printf`-style format string and producing a structured description of the directives contained in that string, together with the argument usage implied by those directives.

The Rust rewrite must preserve the functional role of the source module: given a format string and output containers for parsed directives and argument information, it must analyze the format text, recognize conversion directives, and report the parsed result in the provided directive and argument structures. It must also report success or failure through its return value.

This specification covers only functionality evidenced by the provided module entry point and its associated parsing role. It does not define any new APIs or capabilities beyond parsing and structured result production.

## Feature Specification

### Feature: `printf`-style format parsing

The module shall parse an input format string expressed as `const CHAR_T *format` and identify `printf`-style directives embedded within ordinary text.

For each recognized directive, the module shall populate the provided directives container with structured information describing the parsed directive. The exact internal representation belongs to the directive-related entities already used by the module interface, but the Rust version must preserve the same observable parsing outcomes expected by callers of `PRINTF_PARSE`.

The module shall also derive argument usage information from the parsed directives and populate the provided arguments container. This includes reflecting which arguments are required by the format string as implied by the directives encountered during parsing.

The module shall return an integer status indicating whether parsing completed successfully or failed.

### Feature boundaries

Included behavior:

- Reading and analyzing a `printf`-style format string.
- Distinguishing literal text from conversion directives.
- Recording parsed directive information into the directives output.
- Recording argument usage implied by directives into the arguments output.
- Returning parse status.

Excluded behavior not evidenced by the module input:

- Formatting runtime values into output text.
- Printing, buffering, or I/O.
- Defining a new public formatting API.
- Providing capabilities beyond parsing the format specification itself.

## User Scenarios & Testing

### Scenario 1: Format string with no directives

A caller provides a format string containing only ordinary text and empty output containers.

Expected behavior:

- Parsing succeeds.
- The directives output represents that no conversion directives were found.
- The arguments output represents that no arguments are required by the format string.

### Scenario 2: Format string with one conversion directive

A caller provides a format string containing ordinary text and a single valid `printf`-style directive.

Expected behavior:

- Parsing succeeds.
- Exactly one directive is recorded in the directives output.
- The arguments output reflects the argument required by that directive.

### Scenario 3: Format string with multiple directives

A caller provides a format string containing multiple valid directives.

Expected behavior:

- Parsing succeeds.
- The directives output records all directives in format-string order.
- The arguments output reflects the aggregate argument usage required by the full format string.

### Scenario 4: Format string mixing literals and directives

A caller provides a format string where directives are surrounded by literal characters.

Expected behavior:

- Parsing succeeds.
- Literal text does not create false directives.
- Only actual `printf`-style directives contribute entries to the directives output and requirements to the arguments output.

### Scenario 5: Invalid or malformed directive sequence

A caller provides a format string containing a malformed directive.

Expected behavior:

- Parsing reports failure through its return value.
- Output structures are not reported as a successful parse result.

### Scenario 6: Argument analysis through parsing

A caller uses the module specifically to learn what argument structure a format string requires.

Expected behavior:

- Parsing succeeds for valid input.
- The arguments output reflects the argument requirements implied by the directives recognized in the format string.

## Requirements

### Functional Requirements

#### FR-1: Parse format input
The Rust module shall accept a `printf`-style format string input corresponding to the source module’s `format` parameter and analyze it as the basis for all results.

Traceability: `gnu/printf-parse.c`, `PRINTF_PARSE`

#### FR-2: Recognize directives
The Rust module shall identify conversion directives present in the format string and distinguish them from non-directive literal text.

Traceability: `gnu/printf-parse.c`, `PRINTF_PARSE`

#### FR-3: Produce directive parse results
For a successful parse, the Rust module shall populate the directives result container corresponding to parameter `d` with the parsed directives discovered in the input.

Traceability: `gnu/printf-parse.c`, `PRINTF_PARSE`

#### FR-4: Produce argument usage results
For a successful parse, the Rust module shall populate the argument result container corresponding to parameter `a` with the argument usage implied by the parsed directives.

Traceability: `gnu/printf-parse.c`, `PRINTF_PARSE`

#### FR-5: Preserve directive ordering
When multiple directives are present, the Rust module shall represent them in the same logical order as they appear in the format string.

Traceability: `gnu/printf-parse.c`, `PRINTF_PARSE`

#### FR-6: Report parse status
The Rust module shall return an integer-compatible success/failure status corresponding to whether the format string was parsed successfully.

Traceability: `gnu/printf-parse.c`, `PRINTF_PARSE`

#### FR-7: Reject malformed format input
If the format string contains an invalid or malformed directive sequence that cannot be parsed as required by the source behavior, the Rust module shall report failure rather than a successful parse.

Traceability: `gnu/printf-parse.c`, `PRINTF_PARSE`

### Key Entities

#### `format`
The input character sequence representing a `printf`-style format string to be parsed.

Relationship:
- Consumed by `PRINTF_PARSE`.
- Drives creation of directive records and argument usage information.

Traceability: `PRINTF_PARSE` parameter `const CHAR_T *format`

#### `DIRECTIVES`
The output entity that receives the structured list or collection of parsed directives identified in the format string.

Relationship:
- Populated by `PRINTF_PARSE`.
- Derived directly from directive occurrences in `format`.

Traceability: `PRINTF_PARSE` parameter `DIRECTIVES *d`

#### `arguments`
The output entity that receives the structured description of argument usage required by the parsed directives.

Relationship:
- Populated by `PRINTF_PARSE`.
- Derived from the directives recognized while parsing `format`.

Traceability: `PRINTF_PARSE` parameter `arguments *a`

#### `PRINTF_PARSE`
The module’s parsing operation and primary functional boundary.

Relationship:
- Reads `format`.
- Produces `DIRECTIVES` and `arguments` results.
- Returns parse status.

Traceability: `gnu/printf-parse.c:75-117`

## Success Criteria

### SC-1: Successful parse for valid directive-free input
Given a valid format string containing no directives, the Rust implementation returns success and produces a directives result indicating no parsed directives and an arguments result indicating no required arguments.

Traceability: `PRINTF_PARSE`

### SC-2: Successful parse for valid single-directive input
Given a valid format string containing one directive, the Rust implementation returns success, records one directive in the directives output, and records the corresponding argument usage in the arguments output.

Traceability: `PRINTF_PARSE`

### SC-3: Successful parse for valid multi-directive input
Given a valid format string containing multiple directives, the Rust implementation returns success, records all directives in input order, and reflects their combined argument usage.

Traceability: `PRINTF_PARSE`

### SC-4: Literal text does not create false directives
Given a valid format string containing literal text and directives, the Rust implementation records only actual directives and ignores ordinary text for directive counting and argument derivation.

Traceability: `PRINTF_PARSE`

### SC-5: Failure on malformed directive input
Given a malformed format string that the source parser would reject, the Rust implementation returns failure.

Traceability: `PRINTF_PARSE`

### SC-6: Output containers are populated from parse results
For valid inputs, both output entities supplied to the parser are populated consistently with the directives found in the format string and the argument requirements implied by them.

Traceability: `PRINTF_PARSE`, `DIRECTIVES`, `arguments`

## Non-Goals

The Rust rewrite is not required by this specification to:

- Perform actual formatting or output generation.
- Expose new public APIs beyond the source module’s functional boundary.
- Add capabilities unrelated to parsing format directives and argument usage.
- Guarantee behaviors not evidenced by the provided source module summary.