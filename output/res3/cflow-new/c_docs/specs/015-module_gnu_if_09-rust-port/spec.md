# spec.md

## Title
Rust Functional Specification for `module_gnu_if_09`

## Document Metadata
- Project: `cflow-new`
- Module: `module_gnu_if_09`
- Category: `module_cluster`
- Source scope:
  - `gnu/printf-parse.c`
  - `gnu/vasnprintf.c`
- Rust branch target: `015-module_gnu_if_09-rust-port`
- Generation date: `2026-06-17`

## Overview
This module is part of the formatted output pipeline. Within the analyzed source scope, it contributes two bounded behaviors:

1. format-string parsing behavior for a conditional path involving non-ASCII handling during `printf`-style directive parsing, evidenced in `gnu/printf-parse.c`;
2. formatted rendering behavior for a conditional path involving the `U` conversion during variadic string formatting, evidenced in `gnu/vasnprintf.c`.

The Rust rewrite must preserve the observable behavior of these source-backed responsibilities within the formatting pipeline. The specification is intentionally limited to behavior evidenced by the provided files, functions, and data structure.

## Feature Specification

### Feature 1: Conditional handling during format directive parsing
The module must participate in parsing `printf`-style format content and must preserve the source behavior for the branch that checks whether a relevant character is not ASCII.

This means the Rust version must:
- examine characters relevant to format parsing;
- preserve the conditional distinction between ASCII and non-ASCII input in the same parsing stage represented by the source module;
- ensure that non-ASCII input in this path is handled consistently with the original module’s parser behavior, without broadening the accepted feature set beyond what the source behavior implies.

Traceability:
- `gnu/printf-parse.c`
- main function evidence: `else if (!c_isascii (c));`

### Feature 2: Conditional handling of `U` conversion in formatted output
The module must support the formatting pipeline branch that is selected when a parsed directive’s conversion designator is `U`.

This means the Rust version must:
- represent parsed directives in a form that includes a conversion designator;
- detect when the conversion designator is `U`;
- preserve the source-backed behavior specific to that conversion path during formatted output construction.

The Rust version is not required by this specification to invent or expose any new conversion semantics beyond reproducing the source-observed `U` branch behavior.

Traceability:
- `gnu/vasnprintf.c`
- main function evidence: `else if (dp->conversion == 'U');`

### Feature 3: Directive/state carrier used by formatting logic
The module must include an internal representation for formatting state/directive data sufficient to support conditional behavior in output formatting, including access to the conversion designator used by the `U` branch.

Traceability:
- `gnu/vasnprintf.c`
- core data structure evidence: anonymous `struct` at lines `426-430`

## User Scenarios & Testing

### Scenario 1: Parsing a format string containing only ASCII-relevant content
A caller supplies format content to the formatting subsystem, and the parser processes characters through the normal parsing path.

Expected support in Rust:
- parsing proceeds without misclassifying ASCII characters as non-ASCII;
- downstream formatting receives directive information consistent with source behavior.

Test focus:
- parser behavior on ASCII-only format text;
- no unintended activation of the non-ASCII conditional path.

Traceability:
- `gnu/printf-parse.c`

### Scenario 2: Parsing a format string that reaches the non-ASCII conditional path
A caller supplies format content containing a character that is not ASCII at the parser stage covered by this module.

Expected support in Rust:
- the parser distinguishes this input from ASCII input;
- the resulting parser behavior matches the source module’s handling for this branch.

Test focus:
- branch activation on non-ASCII input;
- stable parser outcome relative to source behavior.

Traceability:
- `gnu/printf-parse.c`
- evidence: `!c_isascii (c)`

### Scenario 3: Formatting with a directive whose conversion is `U`
A caller triggers formatted output generation using a directive that, after parsing, carries conversion designator `U`.

Expected support in Rust:
- the formatting engine recognizes the `U` conversion from directive state;
- the output-generation path taken for `U` matches the source module’s behavior.

Test focus:
- detection of `U` conversion;
- output behavior for the `U` path;
- no fallback to unrelated conversion behavior.

Traceability:
- `gnu/vasnprintf.c`
- evidence: `dp->conversion == 'U'`

### Scenario 4: End-to-end parse-to-format flow through directive state
A caller uses formatted output functionality such that parsed directive data flows from parsing into output formatting.

Expected support in Rust:
- directive state is preserved sufficiently for output logic to inspect conversion information;
- the parser and formatter interoperate without losing the conditions needed for the non-ASCII parser branch and `U` conversion formatter branch.

Test focus:
- compatibility between parsed directive representation and formatting logic;
- preservation of conversion designator across the pipeline.

Traceability:
- `gnu/printf-parse.c`
- `gnu/vasnprintf.c`
- anonymous struct in `gnu/vasnprintf.c`

## Requirements

### Functional Requirements

#### FR-1: Format parsing must distinguish non-ASCII input in the evidenced conditional path
The Rust module shall implement the parsing-stage behavior necessary to distinguish a non-ASCII character from ASCII input at the branch evidenced in `gnu/printf-parse.c`.

Rationale/traceability:
- `gnu/printf-parse.c`
- `else if (!c_isascii (c));`

#### FR-2: Parsing behavior must remain compatible with formatted output processing
The Rust module shall preserve enough parse result information for subsequent formatting logic to evaluate directive-specific behavior.

Rationale/traceability:
- parser source scope: `gnu/printf-parse.c`
- formatter source scope: `gnu/vasnprintf.c`

#### FR-3: Directive state must expose a conversion designator for formatting decisions
The Rust module shall represent directive or formatting state in a way that allows the formatter to inspect the conversion designator.

Rationale/traceability:
- anonymous `struct` in `gnu/vasnprintf.c:426-430`
- `dp->conversion == 'U'`

#### FR-4: Formatting logic must recognize the `U` conversion branch
The Rust module shall implement the conditional formatting path selected when the directive conversion designator is `U`.

Rationale/traceability:
- `gnu/vasnprintf.c`
- `else if (dp->conversion == 'U');`

#### FR-5: Observable behavior for the covered parsing and formatting branches must match the source module
For the non-ASCII parser branch and the `U` conversion formatter branch, the Rust rewrite shall preserve the same externally observable outcomes as the C module in equivalent inputs.

Rationale/traceability:
- `gnu/printf-parse.c`
- `gnu/vasnprintf.c`

### Key Entities

#### Entity 1: Parsed format directive/state record
A directive/state record carries formatting-related information through the output pipeline. The evidence available for this module shows that this record includes at least a conversion field used by formatting branch selection.

Properties evidenced by source:
- contains conversion-designator information;
- is consumed by formatting logic to choose behavior.

Traceability:
- anonymous `struct` in `gnu/vasnprintf.c:426-430`
- `dp->conversion == 'U'`

#### Entity 2: Format character under parser inspection
A character value is inspected during parsing and evaluated for ASCII versus non-ASCII status.

Relationship:
- affects parser branch selection before formatting-stage directive handling.

Traceability:
- `gnu/printf-parse.c`
- `!c_isascii (c)`

#### Entity 3: Conversion designator `U`
A specific conversion value, `U`, is used as a formatting decision point.

Relationship:
- stored in the directive/state record;
- read by output formatting logic.

Traceability:
- `gnu/vasnprintf.c`
- `dp->conversion == 'U'`

## Success Criteria

### SC-1: Non-ASCII parser branch parity
Given test inputs that reach the parser condition evidenced by `!c_isascii (c)`, the Rust implementation produces the same parser outcome as the source module for equivalent inputs.

Traceability:
- `gnu/printf-parse.c`

### SC-2: ASCII inputs do not spuriously trigger the non-ASCII path
Given ASCII-only inputs relevant to the same parser stage, the Rust implementation does not take the non-ASCII branch and remains behaviorally aligned with the source module.

Traceability:
- `gnu/printf-parse.c`

### SC-3: `U` conversion branch parity
Given directives whose conversion designator is `U`, the Rust implementation selects the corresponding formatting path and produces output behavior matching the source module.

Traceability:
- `gnu/vasnprintf.c`

### SC-4: Directive state supports end-to-end branch decisions
Integration tests confirm that the Rust directive/state representation preserves conversion information from parse/format preparation into output-generation logic sufficiently for the `U` branch to operate correctly.

Traceability:
- anonymous `struct` in `gnu/vasnprintf.c`
- `dp->conversion == 'U'`

### SC-5: No unsupported capability expansion
Review of the Rust module interface and behavior confirms that it does not introduce unscoped new formatting capabilities, public APIs, or behavioral guarantees beyond those required to preserve the evidenced parser and formatter responsibilities.

Traceability:
- constrained by the provided source scope and branch evidence

## Out of Scope
The following are not required by this specification unless needed strictly to preserve the evidenced source behavior:
- new public APIs;
- support for conversions or parser branches not evidenced in the provided analysis;
- thread-safety guarantees;
- serialization formats;
- recovery or fault-tolerance mechanisms;
- FFI design;
- performance or benchmark targets.

## Traceability Summary
- Parsing non-ASCII conditional behavior:
  - `gnu/printf-parse.c`
  - `else if (!c_isascii (c));`
- Formatting conditional behavior for `U` conversion:
  - `gnu/vasnprintf.c`
  - `else if (dp->conversion == 'U');`
- Directive/state carrier:
  - anonymous `struct` in `gnu/vasnprintf.c:426-430`