# spec.md

## Title

Rust Port Functional Specification for `module_gnu_if_09`

## Metadata

- **Project**: `cflow-new`
- **Module**: `module_gnu_if_09`
- **Category**: `module_cluster`
- **Rust Branch**: `015-module_gnu_if_09-rust-port`
- **Generation Date**: `2026-06-17`

## Overview

This module is part of the project's GNU-style formatted output handling. Based on the analyzed source files, it covers two linked responsibilities:

1. **Parsing format directives** from printf-like format strings.
2. **Producing formatted output** from parsed directives, including handling conversion-specific branches during variable-argument formatting.

The Rust rewrite must preserve the observable behavior of this parsing-and-formatting path as evidenced by:

- format parsing logic in `gnu/printf-parse.c`
- formatted output construction logic in `gnu/vasnprintf.c`

This specification defines only the functional behavior evidenced by those files and identified control points. It does not introduce new features or public APIs beyond the existing module role.

## Feature Specification

### Summary

The Rust version must implement the module behavior needed to:

- inspect printf-style format strings,
- recognize and step through conversion directives,
- distinguish ordinary text from directive content,
- enforce directive parsing rules that include ASCII-sensitive handling during parse flow,
- apply conversion-dependent formatting behavior during output generation,
- support the conversion-path branching evidenced in the formatting engine, including the branch for conversion code `'U'`.

### In Scope

The Rust module must provide functionality equivalent to the analyzed C module behavior for:

- parsing a format string into usable formatting directive information,
- handling directive characters according to parser rules,
- using parsed directive information to drive formatted output assembly,
- selecting formatting behavior based on conversion kind,
- participating in dynamic output production for variadic-style formatting workflows.

### Out of Scope

The Rust version must not assume or add:

- new formatting language features not evidenced by the source files,
- new external/public APIs beyond what is required to replace this module,
- thread-safety guarantees,
- serialization behavior,
- fault recovery beyond current functional behavior,
- FFI-specific behavior,
- benchmark-driven optimizations as part of the functional spec.

## User Scenarios & Testing

### Scenario 1: Parse a format string containing ordinary text and directives

A caller provides a printf-like format string containing literal text plus one or more conversion directives. The module parses the string and identifies directive boundaries and directive characteristics so later formatting can occur correctly.

**Expected support in Rust:**
- Literal text remains distinguishable from directives.
- Directives are recognized and processed according to parser rules.
- Parsing flow correctly handles characters that are not treated as acceptable directive content under the parser's ASCII-related condition.

**Test focus:**
- Mixed literal/directive input.
- Inputs containing directive characters near parser edge conditions.
- Inputs containing non-ASCII-sensitive cases relevant to the parser branch evidenced in `gnu/printf-parse.c`.

### Scenario 2: Generate formatted output from parsed directives

A caller uses this module as part of formatted string construction. Parsed directive information is consumed by the formatting engine, which emits output according to the conversion selected for each directive.

**Expected support in Rust:**
- Output generation uses parsed conversion metadata.
- Different conversions can follow distinct formatting paths.
- Conversion dispatch includes the `'U'` conversion branch evidenced in `gnu/vasnprintf.c`.

**Test focus:**
- Format strings with multiple conversions.
- Verification that conversion-specific branches are selected correctly.
- Output shape and directive handling remain consistent with the C behavior for supported cases.

### Scenario 3: Handle conversion-dependent formatting path selection

A directive reaches formatting with a conversion code that determines special handling. The module must select the correct formatting path based on the conversion field stored in directive metadata.

**Expected support in Rust:**
- Directive metadata includes conversion identity.
- Formatting decisions are driven by that stored conversion value.
- `'U'` conversion is not collapsed into unrelated conversion behavior.

**Test focus:**
- Direct inspection or behavior-level validation of conversion dispatch.
- Cases comparing `'U'` with other conversion codes to confirm distinct path selection.

### Scenario 4: Preserve parser-to-formatter handoff consistency

A parsed directive structure produced by the parser is consumed by the formatter without loss of the fields needed for conversion-based output.

**Expected support in Rust:**
- Parsed directive information is represented in a form consumable by formatting logic.
- Data needed for conversion dispatch survives the handoff.
- The relation between parser output and formatter input matches the role evidenced by the C module.

**Test focus:**
- End-to-end parse-and-format scenarios.
- Validation that directives parsed from input lead to expected formatter branch behavior.

## Requirements

### Functional Requirements

#### FR-1: Format string directive parsing

The module shall parse printf-like format strings and identify formatting directives needed by downstream output generation.

**Traceability:** `gnu/printf-parse.c`

#### FR-2: ASCII-sensitive parser behavior

The module shall preserve parser behavior that distinguishes cases based on an ASCII-related character check during directive parsing.

**Traceability:** `gnu/printf-parse.c:682-686` (`else if (!c_isascii (c));`)

#### FR-3: Parsed directive information for formatting

The module shall produce or maintain directive information sufficient for the formatting stage to determine conversion-specific behavior.

**Traceability:** parser role in `gnu/printf-parse.c`; formatter consumption in `gnu/vasnprintf.c`

#### FR-4: Formatted output construction from directive metadata

The module shall construct formatted output using directive metadata derived from parsing and conversion-specific formatting rules.

**Traceability:** `gnu/vasnprintf.c`

#### FR-5: Conversion-based formatting dispatch

The module shall select formatting behavior according to the conversion code stored in directive data.

**Traceability:** `gnu/vasnprintf.c:2387-2768`

#### FR-6: Support for `'U'` conversion branch

The module shall preserve the distinct formatting branch for directives whose conversion code is `'U'`.

**Traceability:** `gnu/vasnprintf.c:2387-2768` (`else if (dp->conversion == 'U');`)

### Key Entities

#### 1. Format Directive Metadata

A directive representation carries the parsed properties of one format directive and is used by the formatting engine to decide how output should be produced.

Evidence for this entity includes the formatter's use of a directive object with a `conversion` field.

**Traceability:** directive access pattern in `gnu/vasnprintf.c:2387-2768`

#### 2. Conversion Field

The conversion field identifies the directive's conversion code and drives conversion-specific branch selection in formatting.

**Relationship:**
- belongs to format directive metadata,
- is produced or preserved from parsing,
- is consumed by output formatting logic.

**Traceability:** `dp->conversion` in `gnu/vasnprintf.c:2387-2768`

#### 3. Formatter Working Structure

The analyzed module includes an internal anonymous struct used in the formatting path. The Rust rewrite must preserve the functional role of internal formatting state where required for equivalent behavior, without requiring the same representation.

**Traceability:** anonymous `struct` in `gnu/vasnprintf.c:426-430`

## Success Criteria

### SC-1: Parser correctness for directive recognition

For representative printf-like input strings used by this module path, the Rust version correctly distinguishes literal text from directives and provides directive information required for formatting.

**Measured by:** parser-level or end-to-end tests covering mixed literal/directive inputs.
**Traceability:** `gnu/printf-parse.c`

### SC-2: Preservation of ASCII-sensitive parse behavior

Inputs that exercise the parser's ASCII-related conditional behavior produce Rust results consistent with the C module's accepted/rejected or branched handling in that parse path.

**Measured by:** targeted tests around characters that trigger the `c_isascii`-related branch.
**Traceability:** `gnu/printf-parse.c:682-686`

### SC-3: Conversion-dispatch correctness

For parsed directives reaching the formatter, the Rust version selects formatting behavior according to the stored conversion code.

**Measured by:** tests that validate distinct behavior across multiple conversion kinds.
**Traceability:** `gnu/vasnprintf.c:2387-2768`

### SC-4: `'U'` conversion path preservation

When a directive's conversion code is `'U'`, the Rust version follows the corresponding distinct formatting path rather than treating it as an unrelated conversion.

**Measured by:** targeted tests that exercise `'U'` conversion handling and compare behavior against non-`'U'` cases.
**Traceability:** `gnu/vasnprintf.c:2387-2768`

### SC-5: End-to-end parser/formatter consistency

Directive information produced from parsing is sufficient for the formatter to generate output without losing conversion identity needed for dispatch.

**Measured by:** end-to-end tests from format string input through output generation.
**Traceability:** `gnu/printf-parse.c`, `gnu/vasnprintf.c`