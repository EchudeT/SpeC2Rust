# spec.md

## Title

Rust Functional Specification for `module_gnu_if_11`

## Document Metadata

- **Project**: `cflow-new`
- **Module**: `module_gnu_if_11`
- **Category**: `module_cluster`
- **Source Basis**: `gnu/vasnprintf.c`
- **Rust Branch**: `017-module_gnu_if_11-rust-port`
- **Generation Date**: `2026-06-17`

## Overview

This module specification covers a narrowly scoped portion of formatted output behavior in `gnu/vasnprintf.c`, specifically conditional handling for:

- character conversion with distinction between narrow and wide character arguments
- floating-point general-format conversion for `g` and `G`

The Rust rewrite must preserve the same observable formatting decisions and output behavior represented by these conditional branches. The scope is limited to behavior evidenced by the analyzed module results and must not introduce additional capabilities beyond that scope.

## Feature Specification

### Summary

The Rust module must implement the formatting behavior required when a parsed format directive indicates:

1. a `%c`-style character conversion whose argument is not a wide character
2. a `%g` or `%G` floating-point conversion

This functionality exists within a larger formatted-string construction flow, so the Rust version must correctly participate in directive-based formatting for these cases.

### In-Scope Behavior

#### 1. Non-wide character conversion handling

When the active conversion is character conversion (`c`) and the associated argument is not classified as a wide character, the module must apply the narrow-character formatting path.

The Rust rewrite must therefore:

- recognize that the formatting directive requests character conversion
- distinguish non-wide character arguments from wide character arguments
- produce output according to the non-wide character branch for this conversion case

This requirement is directly evidenced by the conditional logic tied to conversion `c` and argument type comparison against `TYPE_WIDE_CHAR`.

#### 2. General floating-point conversion handling

When the active conversion is `g` or `G`, the module must apply the general floating-point formatting path associated with these conversions.

The Rust rewrite must therefore:

- recognize both lowercase `g` and uppercase `G` as distinct accepted directive conversions within this module scope
- route both conversions through the corresponding general floating-point handling behavior
- preserve any observable distinction required by choosing `g` versus `G`, to the extent that distinction is part of formatting output behavior

This requirement is directly evidenced by the conditional branch selecting on `dp->conversion == 'g' || dp->conversion == 'G'`.

### Out of Scope

The following are not specified here unless required only as supporting context for the above behavior:

- unrelated conversion specifiers
- parsing features not needed to reach the evidenced branches
- public APIs beyond what is necessary to support this module’s role in formatted output generation
- guarantees about concurrency, FFI, persistence, recovery, or performance

## User Scenarios & Testing

### Scenario 1: Formatting a narrow character argument

A caller provides a format directive that resolves to character conversion and supplies an argument that is not a wide character. The module must select the non-wide character behavior and emit the corresponding formatted character output.

**Test expectations:**
- Given a directive with conversion `c`
- And an argument classified as not `TYPE_WIDE_CHAR`
- The Rust module follows the non-wide character branch
- And the output contains the expected character-formatted result

### Scenario 2: Avoiding narrow-character handling for wide-character input

A caller provides a character conversion directive, but the argument is classified as a wide character. The Rust module must not treat this input as the non-wide character case covered by this module branch.

**Test expectations:**
- Given a directive with conversion `c`
- And an argument classified as wide character
- The condition for the non-wide character branch is not satisfied
- And behavior diverges from the narrow-character path

### Scenario 3: Formatting a general floating-point value with `%g`

A caller provides a general floating-point directive using lowercase `g`. The module must recognize this conversion and process the value through the `g/G` formatting path.

**Test expectations:**
- Given a directive with conversion `g`
- The Rust module selects the general floating-point branch
- And produces output consistent with the module’s `g` conversion behavior

### Scenario 4: Formatting a general floating-point value with `%G`

A caller provides a general floating-point directive using uppercase `G`. The module must recognize this conversion and process the value through the same evidenced branch family as `g`, while preserving any observable uppercase-specific output behavior.

**Test expectations:**
- Given a directive with conversion `G`
- The Rust module selects the general floating-point branch
- And produces output consistent with the module’s `G` conversion behavior

### Scenario 5: Directive-driven branching within formatted output assembly

A caller uses formatted output generation containing mixed directives, including one of the covered conversions. The module must choose behavior based on the current directive and argument classification rather than applying a generic fallback for these cases.

**Test expectations:**
- In a formatting sequence containing covered directives
- Branch selection depends on the directive conversion and argument type metadata
- And the resulting output reflects the covered conversion-specific behavior

## Requirements

### Functional Requirements

- **FR-1**: The Rust module shall support directive-based branching for character conversion when the conversion code is `c`.
  **Traceability**: `gnu/vasnprintf.c`, conditional at lines 3560-3628.

- **FR-2**: The Rust module shall distinguish whether the argument associated with a `c` conversion is a wide character or not.
  **Traceability**: `gnu/vasnprintf.c`, condition comparing argument type against `TYPE_WIDE_CHAR` at lines 3560-3628.

- **FR-3**: The Rust module shall apply the non-wide-character formatting behavior only when the conversion is `c` and the argument type is not wide character.
  **Traceability**: `gnu/vasnprintf.c`, `dp->conversion == 'c' && a.arg[dp->arg_index].type != TYPE_WIDE_CHAR` at lines 3560-3628.

- **FR-4**: The Rust module shall support directive-based branching for general floating-point conversion when the conversion code is `g`.
  **Traceability**: `gnu/vasnprintf.c`, conditional at lines 5301-5551.

- **FR-5**: The Rust module shall support directive-based branching for general floating-point conversion when the conversion code is `G`.
  **Traceability**: `gnu/vasnprintf.c`, conditional at lines 5301-5551.

- **FR-6**: The Rust module shall treat `g` and `G` as accepted members of the general floating-point conversion handling path.
  **Traceability**: `gnu/vasnprintf.c`, `dp->conversion == 'g' || dp->conversion == 'G'` at lines 5301-5551.

- **FR-7**: The Rust module shall preserve observable output behavior differences that arise from selecting `g` versus `G`, if such differences are part of the conversion result in the source behavior.
  **Traceability**: Selection of distinct conversion characters `g` and `G` in `gnu/vasnprintf.c`, lines 5301-5551.

### Key Entities

- **Format directive descriptor (`dp`)**: A directive record representing the currently processed format item, including at least:
  - a conversion code
  - an argument index used to locate the corresponding argument
  **Relationship**: Drives branch selection for both covered behaviors.

- **Argument collection (`a.arg`)**: A set of supplied formatting arguments indexed by the directive’s argument index.
  **Relationship**: Provides the argument whose type classification is inspected for `c` conversion handling.

- **Argument type classification**: Metadata indicating whether an argument is a wide character, evidenced by comparison against `TYPE_WIDE_CHAR`.
  **Relationship**: Determines whether `%c` uses the non-wide-character branch covered by this module.

- **Anonymous struct from `gnu/vasnprintf.c` lines 426-430**: A core internal structure present in the source basis for this module.
  **Relationship**: Part of the module’s internal data model; the Rust rewrite must represent equivalent state as needed to support the evidenced directive and argument classification behavior.

## Success Criteria

- **SC-1**: For a `c` conversion with a non-wide-character argument classification, tests show that the Rust module selects the non-wide-character handling path and produces corresponding character output.
  **Traceability**: `gnu/vasnprintf.c` lines 3560-3628.

- **SC-2**: For a `c` conversion with a wide-character argument classification, tests show that the Rust module does not execute the non-wide-character branch covered by this specification.
  **Traceability**: `gnu/vasnprintf.c` lines 3560-3628.

- **SC-3**: For a `g` conversion, tests show that the Rust module recognizes the directive and produces output through the general floating-point conversion path.
  **Traceability**: `gnu/vasnprintf.c` lines 5301-5551.

- **SC-4**: For a `G` conversion, tests show that the Rust module recognizes the directive and produces output through the general floating-point conversion path.
  **Traceability**: `gnu/vasnprintf.c` lines 5301-5551.

- **SC-5**: Branch-selection tests demonstrate that the Rust implementation uses both directive conversion metadata and argument type metadata where required, rather than treating all covered inputs identically.
  **Traceability**: `gnu/vasnprintf.c` lines 3560-3628 and 5301-5551.

- **SC-6**: Regression tests covering the evidenced branches pass on the Rust branch `017-module_gnu_if_11-rust-port` without introducing unsupported new behavior outside the specified scope.
  **Traceability**: Entire module scope as evidenced by `gnu/vasnprintf.c`.