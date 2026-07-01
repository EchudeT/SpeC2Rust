# spec.md

## Title

Rust Functional Specification: `module_gnu_printf-parse.c_41`

## Metadata

- Project: `cflow-new`
- Module: `module_gnu_printf-parse.c_41`
- Category: `module_cluster`
- Source file: `gnu/printf-parse.c`
- Primary function: `PRINTF_PARSE`
- Target Rust branch: `047-module_gnu_printf_parse.c_41-rust-port`
- Generation date: `2026-06-17`

## Overview

This module parses a printf-style format string and records the formatting directives it contains into caller-provided directive and argument descriptors.

The Rust rewrite must preserve the same functional boundary: it must analyze a format string, identify conversion directives and their parameter usage, and populate output structures that describe the parsed result. The module is a parser only; its responsibility is to interpret the format string and report the parsed directive/argument information through the provided outputs and return status.

## Feature Specification

### Summary

The Rust module must implement format-string parsing equivalent in scope to the C module entry point `PRINTF_PARSE`. Given a format string and output holders for directives and arguments, it must:

- scan the full format string,
- distinguish ordinary text from printf conversion directives,
- parse each directive sufficiently to describe it in the directives result,
- detect and record argument usage associated with parsed directives,
- return a status result indicating whether parsing succeeded.

### In-Scope Behavior

The Rust version must support the behavior evidenced by the module interface:

- Accept a format string as input.
- Parse printf-style directives embedded in that string.
- Populate a directives aggregate describing the directives found.
- Populate an arguments aggregate describing arguments referenced by the parsed directives.
- Produce an integer-like success/failure outcome equivalent in purpose to the C function return value.

### Out-of-Scope Behavior

The Rust version must not introduce capabilities not evidenced by this module boundary, including:

- formatting values,
- emitting rendered output,
- adding new public parsing modes,
- defining unrelated validation or recovery features beyond parse success/failure reporting.

## User Scenarios & Testing

### Scenario 1: Format string with no conversion directives

A caller provides a string containing only literal characters and initialized directive and argument result objects.

Expected support:

- The parser accepts the string.
- It completes successfully.
- The resulting directives description indicates that no conversion directives were present.
- The arguments description indicates that no arguments were consumed.

### Scenario 2: Format string with one conversion directive

A caller provides a string containing one printf conversion.

Expected support:

- The parser identifies the directive.
- The directive is represented in the directives result.
- The referenced argument is represented in the arguments result.
- The parser returns success.

### Scenario 3: Format string with multiple directives

A caller provides a string containing several conversion directives mixed with literal text.

Expected support:

- Each directive is found in sequence.
- Literal text does not become a directive entry.
- The directives result contains one entry per parsed conversion directive.
- The arguments result reflects all arguments referenced by those directives.
- The parser returns success if the full string is valid.

### Scenario 4: Format string using directive components that affect argument usage

A caller provides a string where a conversion directive includes components that influence which arguments are referenced by that directive.

Expected support:

- The parser records argument usage implied by the parsed directive structure.
- The directives and arguments outputs remain consistent with one another.
- The parser returns success when the directive syntax is valid.

### Scenario 5: Invalid or unsupported format syntax

A caller provides a malformed format string.

Expected support:

- The parser reports failure through its return status.
- The directives and arguments outputs are not reported as a successful parse result.

## Requirements

### Functional Requirements

#### FR-1: Parse format strings
The module shall accept a format string and analyze it according to printf-style directive syntax, as evidenced by `PRINTF_PARSE` in `gnu/printf-parse.c`.

#### FR-2: Identify conversion directives
The module shall distinguish conversion directives from ordinary literal text within the format string and record the directives found, as evidenced by the `DIRECTIVES *d` output of `PRINTF_PARSE`.

#### FR-3: Record parsed directive information
For each successfully parsed conversion directive, the module shall populate the directives result with information describing that directive, as evidenced by the `DIRECTIVES *d` output of `PRINTF_PARSE`.

#### FR-4: Record argument usage
The module shall populate the arguments result with the arguments referenced by the parsed format string, as evidenced by the `arguments *a` output of `PRINTF_PARSE`.

#### FR-5: Keep directive and argument results consistent
The module shall ensure that the directives description and the arguments description correspond to the same parse of the input string, as evidenced by the single parsing operation performed by `PRINTF_PARSE`.

#### FR-6: Report parse outcome
The module shall return an integer-like status indicating whether parsing completed successfully or failed, as evidenced by the return type `int` of `PRINTF_PARSE`.

#### FR-7: Process the full input string
The module shall parse the provided format string as a complete unit rather than only a prefix, as evidenced by the module role of whole-format parsing through `PRINTF_PARSE`.

### Key Entities

#### Format String
The input character sequence supplied to `PRINTF_PARSE`. It is the source text from which directives and argument references are parsed.

#### Directives
The caller-provided aggregate output referenced by `DIRECTIVES *d`. It stores the set of conversion directives identified in the format string.

Relationship:
- Produced from the format string by parsing.
- Must align with the argument information for the same parse.

#### Arguments
The caller-provided aggregate output referenced by `arguments *a`. It stores information about arguments referenced by the parsed directives.

Relationship:
- Derived from directive parsing.
- Must correspond to the directives found in the same input string.

#### Parse Result
The status value returned by `PRINTF_PARSE`.

Relationship:
- Indicates whether the directives and arguments outputs represent a successful parse.

## Success Criteria

### SC-1: Correct empty-format handling
For an input containing no conversion directives, the Rust module returns success and produces directive and argument results indicating no parsed directives and no referenced arguments.

Traceability:
- `gnu/printf-parse.c`
- `PRINTF_PARSE`

### SC-2: Correct single-directive handling
For an input containing one valid conversion directive, the Rust module returns success and records exactly one directive with corresponding argument usage.

Traceability:
- `gnu/printf-parse.c`
- `PRINTF_PARSE`

### SC-3: Correct multi-directive handling
For an input containing multiple valid conversion directives, the Rust module returns success and records all directives and referenced arguments in parse order and within one coherent parse result.

Traceability:
- `gnu/printf-parse.c`
- `PRINTF_PARSE`

### SC-4: Correct failure signaling
For malformed format syntax, the Rust module returns a failure status rather than reporting a successful parse.

Traceability:
- `gnu/printf-parse.c`
- `PRINTF_PARSE`

### SC-5: Output consistency
For any successful parse, the directives output and arguments output are mutually consistent representations of the same format string parse.

Traceability:
- `gnu/printf-parse.c`
- `PRINTF_PARSE`

### SC-6: Functional equivalence at module boundary
The Rust rewrite exposes behavior equivalent to the C module boundary: parsing a provided format string into caller-visible directive and argument descriptors with success/failure reporting, without expanding the module scope.

Traceability:
- `gnu/printf-parse.c`
- `PRINTF_PARSE`