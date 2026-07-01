# spec.md

## Title

Rust Functional Specification for `module_gnu_if_10`

## Document Control

- Project: `cflow-new`
- Module: `module_gnu_if_10`
- Category: `module_cluster`
- Source basis: `gnu/vasnprintf.c`
- Rust branch: `016-module_gnu_if_10-rust-port`
- Generation date: 2026-06-11

## Overview

This module covers the conditional formatting behavior in `vasnprintf` for character and string conversions when the selected argument type is a wide-character form.

The Rust rewrite must preserve the source module’s functional boundary:

- handling `%s`-family formatting paths when the argument type and the active build mode disagree on narrow-vs-wide string expectations, and
- handling `%c`-family formatting paths when the argument is a wide character.

This specification is limited to the evidenced behavior from the analyzed conditional branches and the related conversion descriptor structure.

## Scope

### In Scope

- Conversion-time selection of the correct formatting path for:
  - string conversion with a wide-string typed argument under the source condition logic
  - character conversion with a wide-character typed argument
- Dependence on parsed conversion metadata and argument type classification
- Behavior necessary for integration into a `vasnprintf`-style formatting pipeline

### Out of Scope

- Full general-purpose formatting behavior not evidenced by this module slice
- Definition of unrelated conversion classes
- New APIs or capabilities beyond preserving the observed conditional formatting behavior

## Feature Specification

The module is responsible for choosing and executing the correct formatting behavior for specific conversion/type combinations inside variadic formatted output processing.

### Feature 1: String conversion path gating by conversion kind and argument type

For string conversions, the module must inspect:

- the parsed conversion code for the active directive, and
- the resolved runtime argument type for the directive’s argument index.

The Rust version must preserve the observed branch behavior for `%s` conversions involving wide-string typed arguments. The source evidence shows that this behavior is conditional on the active wide-character build mode:

- in one build mode, the branch is selected when the conversion is `s` and the argument type is not a wide string;
- in the alternate build mode, the branch is selected when the conversion is `s` and the argument type is a wide string.

The Rust rewrite must preserve this distinction exactly as module behavior, rather than normalizing or extending it.

### Feature 2: Wide-character conversion path for `%c`

For character conversions, the module must detect when:

- the parsed conversion code is `c`, and
- the selected argument is typed as a wide character.

When both conditions hold, the Rust version must route formatting through the wide-character-specific behavior corresponding to the source module.

### Feature 3: Descriptor-driven argument selection

The module must use per-conversion descriptor data to determine:

- which conversion is being processed, and
- which argument index is associated with that conversion.

The argument type check must be performed against the resolved argument entry at the descriptor’s argument index.

## User Scenarios & Testing

### Scenario 1: `%s` conversion evaluated against a wide-string typed argument

A caller uses formatted output processing and reaches a directive whose parsed conversion is `s`. The argument bound to that directive is classified as a wide string.

Expected module behavior:

- the module selects or skips the string-formatting branch exactly according to the source condition logic for the active wide-character mode;
- the Rust rewrite produces the same branch-selection outcome as the C source for the same descriptor and argument typing.

Suggested test coverage:

- validate branch selection for conversion `s` with argument type `TYPE_WIDE_STRING`
- validate the alternate outcome when the build-mode condition is inverted, if the Rust design represents both source modes

### Scenario 2: `%s` conversion evaluated against a non-wide-string typed argument

A caller processes a string conversion directive where the associated argument is not classified as a wide string.

Expected module behavior:

- the module follows the source condition logic for `%s` based on the same mode-sensitive rule;
- the Rust rewrite matches the C source decision for whether this conditional path applies.

Suggested test coverage:

- validate branch selection for conversion `s` with an argument type other than `TYPE_WIDE_STRING`
- verify behavior symmetry against Scenario 1 under the relevant source mode

### Scenario 3: `%c` conversion with a wide-character argument

A caller processes a character conversion directive with an argument classified as a wide character.

Expected module behavior:

- the module recognizes the `%c` plus `TYPE_WIDE_CHAR` combination;
- the Rust rewrite dispatches to the same functional path as the source module for this combination.

Suggested test coverage:

- conversion code `c` with argument type `TYPE_WIDE_CHAR` enters the wide-character path
- nearby non-matching combinations do not enter this path

### Scenario 4: Argument selection through descriptor index

A formatting operation contains multiple arguments and directives. The active directive references one argument index, and only that indexed argument’s type determines whether the conditional branch applies.

Expected module behavior:

- branch selection depends on the descriptor’s `arg_index`, not on neighboring arguments;
- the Rust rewrite evaluates the type check against the same indexed argument as the source.

Suggested test coverage:

- multiple arguments with mixed types
- verify that changing a non-selected argument does not affect branch choice
- verify that changing the selected argument type does affect branch choice

## Requirements

### Functional Requirements

#### FR-1: `%s` conversion recognition

The module shall recognize when the active conversion descriptor denotes string conversion using conversion code `s`.

Traceability: `gnu/vasnprintf.c:2914-3366`

#### FR-2: Mode-sensitive wide-string condition preservation

For string conversion, the module shall preserve the source module’s mode-sensitive condition involving `TYPE_WIDE_STRING`, including the distinction between the two source configurations indicated by the conditional compilation logic.

Traceability: `gnu/vasnprintf.c:2914-3366`

#### FR-3: Indexed argument type evaluation

The module shall evaluate the argument type for the active directive using the argument index stored in the conversion descriptor.

Traceability: `gnu/vasnprintf.c:2914-3366`, `gnu/vasnprintf.c:3369-3557`

#### FR-4: `%c` conversion with wide-character argument recognition

The module shall recognize when the active conversion descriptor denotes character conversion using conversion code `c` and the selected argument type is `TYPE_WIDE_CHAR`.

Traceability: `gnu/vasnprintf.c:3369-3557`

#### FR-5: Conditional path equivalence

For the conversion/type combinations covered by this module, the Rust rewrite shall make the same conditional path-selection decisions as the source module.

Traceability: `gnu/vasnprintf.c:2914-3366`, `gnu/vasnprintf.c:3369-3557`

### Key Entities

#### Conversion descriptor

An internal descriptor structure represents a parsed formatting directive and provides at least:

- the conversion code for the directive
- the argument index associated with that directive

Relationship:
- the module reads this descriptor to decide which conditional formatting rule to apply.

Traceability: anonymous struct in `gnu/vasnprintf.c:426-430`; usage in `gnu/vasnprintf.c:2914-3366`, `gnu/vasnprintf.c:3369-3557`

#### Argument table entry

An argument table contains per-argument type classification information accessible by argument index.

Relevant classifications evidenced for this module:

- `TYPE_WIDE_STRING`
- `TYPE_WIDE_CHAR`

Relationship:
- the conversion descriptor’s `arg_index` selects the argument entry whose type controls branch selection.

Traceability: `gnu/vasnprintf.c:2914-3366`, `gnu/vasnprintf.c:3369-3557`

## Success Criteria

### SC-1: `%s` branch parity

Given a conversion descriptor with conversion code `s`, the Rust implementation matches the C source branch-selection result for both:
- an argument typed as `TYPE_WIDE_STRING`, and
- an argument not typed as `TYPE_WIDE_STRING`,
under the represented source mode.

Traceability: FR-1, FR-2, FR-5

### SC-2: `%c` wide-character parity

Given a conversion descriptor with conversion code `c` and a selected argument typed as `TYPE_WIDE_CHAR`, the Rust implementation selects the same wide-character handling path as the C source.

Traceability: FR-4, FR-5

### SC-3: Correct argument indexing

For test inputs containing multiple arguments, branch selection changes only when the type of the argument referenced by the descriptor’s argument index changes.

Traceability: FR-3

### SC-4: No false positives for neighboring cases

The Rust implementation does not trigger the module’s covered conditional paths for non-matching conversion/type combinations adjacent to the evidenced cases.

Traceability: FR-1, FR-2, FR-4, FR-5

## Acceptance Notes

- Conformance is determined by behavioral equivalence for the evidenced conditional formatting decisions only.
- The rewrite should be accepted if it preserves the source module’s decision logic for the covered `%s` and `%c` cases using descriptor-driven argument type checks.