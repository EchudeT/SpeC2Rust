# spec.md

## Title

Rust Functional Specification for `module_gnu_if_09`

## Metadata

- **Project**: `cflow-new`
- **Module**: `module_gnu_if_09`
- **Category**: `module_cluster`
- **Source scope**: `gnu/printf-parse.c`, `gnu/vasnprintf.c`
- **Rust branch target**: `015-module_gnu_if_09-rust-port`
- **Generation date**: 2026-06-11

## Overview

This module belongs to the formatted-output pipeline implemented across GNU-style formatting sources. The analyzed scope shows behavior in two places:

- format-string parsing logic that evaluates character validity during parsing, including a branch for non-ASCII handling in `gnu/printf-parse.c`
- formatted output generation logic that dispatches on conversion kind, including a branch for conversion `'U'` in `gnu/vasnprintf.c`

The Rust rewrite must preserve the same functional role within that pipeline: it must accept parsed formatting directives and participate in producing formatted output with behavior consistent with the C module for the evidenced cases in this source scope.

## Feature Specification

### Summary

The module provides functionality required to:

1. inspect and parse format-string content while distinguishing characters that are not ASCII during parsing
2. process formatting directives during output generation by branching on the directive conversion code
3. handle the conversion case identified by conversion character `'U'`
4. carry directive-related state in an internal structured record used by formatting logic

### In-Scope Functionality

The Rust version must implement the following module behavior:

- **Format parsing character classification**
  - During parsing of formatting input, the module must recognize and correctly handle the case where a character is not ASCII.
  - This behavior must remain part of the parser’s decision flow rather than being ignored or normalized away.

- **Directive-driven formatting dispatch**
  - During formatted output generation, the module must inspect a directive/conversion descriptor and choose behavior based on the conversion character.
  - The decision structure must include the evidenced `'U'` conversion branch.

- **`'U'` conversion support**
  - The Rust version must preserve support for the conversion path associated with conversion code `'U'`.
  - If the surrounding formatter accepts directives that reach this branch in the C implementation, the Rust version must accept and process the same case without treating it as unsupported solely because it is uncommon.

- **Internal directive state representation**
  - The Rust version must include an internal data representation corresponding to the anonymous struct evidenced in `gnu/vasnprintf.c`.
  - That representation must be sufficient to carry the directive state needed for conversion-based formatting decisions.

### Out of Scope

The following are not specified here because they are not evidenced by the provided analysis input:

- new public APIs beyond what is needed to preserve existing module behavior
- thread-safety guarantees
- serialization formats
- FFI boundaries
- performance targets or benchmark requirements
- recovery or fallback mechanisms not already implied by existing formatting behavior

## User Scenarios & Testing

### Scenario 1: Parsing a format string containing a non-ASCII character

**Context:** A caller passes a format string into the formatting pipeline, and parsing reaches a character outside the ASCII range.

**Expected module behavior:**
- The parser-side logic identifies that the current character is not ASCII.
- The parser preserves C-module-equivalent control flow for this case.
- The result must match the original module’s handling for equivalent input.

**Test guidance:**
- Provide format input containing at least one byte/character that is not ASCII at a parser-relevant position.
- Verify that parser outcome in Rust matches the C module outcome for acceptance/rejection/branch behavior.

### Scenario 2: Formatting a directive whose conversion code is `'U'`

**Context:** A parsed directive reaches output generation with its conversion field set to `'U'`.

**Expected module behavior:**
- The output-generation logic routes processing through the `'U'` conversion path.
- The directive is not misclassified as a different conversion.
- The formatting result or control outcome matches the C behavior for the same directive state and arguments.

**Test guidance:**
- Construct or obtain a parsed directive record with conversion `'U'`.
- Run the Rust formatting path and compare result behavior with the C implementation.

### Scenario 3: Mixed formatting with multiple directive types including `'U'`

**Context:** A larger format operation includes ordinary text, standard directives, and at least one `'U'` directive.

**Expected module behavior:**
- The parser and formatter cooperate correctly across multiple directives.
- Non-`'U'` directives continue through normal dispatch.
- The `'U'` directive is handled by its specific branch without disrupting surrounding output.

**Test guidance:**
- Use a format input containing literal text and multiple directives.
- Verify ordering, directive selection, and final output consistency relative to the C module.

### Scenario 4: Internal directive state drives conversion dispatch

**Context:** The formatter consumes internal directive metadata created by parsing or setup logic.

**Expected module behavior:**
- The internal record provides the conversion field and any associated state needed for dispatch.
- Conversion-based branching uses this state consistently.
- Invalid loss of directive metadata does not occur between parse and format stages.

**Test guidance:**
- Validate that a directive record created or populated for formatting retains the conversion code through formatting dispatch.
- Verify especially that conversion `'U'` reaches the correct branch.

## Requirements

### Functional Requirements

- **FR-1: Non-ASCII parse-path handling**
  - The module shall distinguish non-ASCII characters during format parsing and preserve the original parser decision behavior for that case.
  - **Traceability:** `gnu/printf-parse.c`, branch at `if` on `!c_isascii(c)`.

- **FR-2: Conversion-based formatting dispatch**
  - The module shall inspect directive conversion metadata during output generation and branch behavior based on the conversion code.
  - **Traceability:** `gnu/vasnprintf.c`, branch on `dp->conversion`.

- **FR-3: `'U'` conversion branch support**
  - The module shall implement the formatting behavior associated with conversion code `'U'` as a supported dispatch case.
  - **Traceability:** `gnu/vasnprintf.c`, `else if (dp->conversion == 'U')`.

- **FR-4: Directive state carrier**
  - The module shall maintain an internal structured representation for directive-related state sufficient to support conversion dispatch during formatting.
  - **Traceability:** anonymous `struct` in `gnu/vasnprintf.c:426-430`.

- **FR-5: Parse-to-format behavioral continuity**
  - The module shall preserve the functional linkage between parser-side classification decisions and formatter-side directive processing so that equivalent inputs follow equivalent end-to-end behavior in Rust and C for the evidenced cases.
  - **Traceability:** combined evidence from `gnu/printf-parse.c` and `gnu/vasnprintf.c`.

### Key Entities

- **Directive state record**
  - An internal structured entity evidenced by the anonymous struct in `gnu/vasnprintf.c`.
  - Relationship:
    - carries conversion-related metadata
    - is consumed by formatting logic to choose conversion-specific behavior

- **Conversion code**
  - A directive property used during formatting dispatch.
    - stored in the directive state record
    - read by output-generation logic
    - includes the evidenced special case `'U'`

- **Parser input character**
  - A character examined during format parsing.
    - classified by parser logic, including whether it is ASCII
    - influences parser control flow before formatting occurs

## Success Criteria

- **SC-1: Non-ASCII parsing equivalence**
  - For test inputs that exercise the non-ASCII parse branch, the Rust module produces the same parser-level outcome as the C module.
  - **Traceability:** `gnu/printf-parse.c`, non-ASCII branch.

- **SC-2: `'U'` dispatch reachability**
  - For inputs or directive records with conversion code `'U'`, the Rust module reaches the `'U'` formatting path rather than rejecting or remapping it.
  - **Traceability:** `gnu/vasnprintf.c`, `'U'` conversion branch.

- **SC-3: Conversion-driven behavior preservation**
  - For directive records used in formatting, conversion selection in Rust matches the C module’s conversion-based dispatch behavior for the evidenced cases.
  - **Traceability:** `gnu/vasnprintf.c`, dispatch on `dp->conversion`.

- **SC-4: Directive state sufficiency**
  - The Rust internal directive representation contains the information required to support conversion dispatch without loss of the conversion field before formatting.
  - **Traceability:** anonymous struct in `gnu/vasnprintf.c`.

- **SC-5: End-to-end module consistency**
  - In integration tests covering parse and formatting flow, the Rust rewrite matches C-module behavior for scenarios involving non-ASCII parser input and formatting directives including `'U'`.
  - **Traceability:** `gnu/printf-parse.c`, `gnu/vasnprintf.c`.

## Acceptance Notes

- Equivalence should be judged on observable formatting and parser outcomes for the evidenced branch cases.
- This specification is intentionally limited to behavior supported by the provided module analysis and does not require undocumented extensions.