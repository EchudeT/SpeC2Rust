# spec.md

## Title

Rust Port Functional Specification for `module_gnu_if_11`

## Overview

This module is part of the formatted output processing in `gnu/vasnprintf.c`. The analyzed logic is centered on conditional handling for specific conversion categories within printf-style formatting:

- character conversion handling for `'c'`, with distinct behavior when the argument is not a wide character
- floating-point general-format handling for `'g'` and `'G'`

The Rust rewrite must preserve the observable formatting behavior represented by these conditional branches and their interaction with parsed format directives and argument metadata.

## Scope

In scope for this module:

- behavior tied to evaluating a parsed conversion specifier
- selecting the correct formatting path for:
  - narrow character conversion via `'c'`
  - general floating-point conversion via `'g'` and `'G'`
- use of directive metadata and argument type metadata to decide which behavior applies

Out of scope:

- introducing new formatting features not evidenced by the source analysis
- changing the set of supported conversion meanings
- adding new public APIs beyond what is needed to preserve current module behavior

## Source Basis

This specification is derived from the following analyzed source evidence:

- File: `gnu/vasnprintf.c`
- Conditional branch: character conversion logic at lines 3560-3628
- Conditional branch: floating-point `'g'` / `'G'` logic at lines 5301-5551
- Core data structure evidence: anonymous struct at lines 426-430

## Feature Specification

The Rust version must implement the module behavior that dispatches formatting based on a parsed conversion directive and the corresponding argument type information.

### Feature 1: Character Conversion Selection

When a format directive has conversion code `'c'`, the module must distinguish the applicable formatting path based on the argument type metadata. In the analyzed source, this branch is specifically taken when the argument associated with the directive is **not** of wide-character type.

Required behavior:

- inspect the directive conversion code
- inspect the referenced argument's type classification
- apply the non-wide-character `'c'` formatting behavior only when the argument type is not `TYPE_WIDE_CHAR`
- preserve the distinction between wide-character and non-wide-character handling; the Rust port must not collapse them into one path if that changes behavior

This requirement is directly traceable to the conditional:
`dp->conversion == 'c' && a.arg[dp->arg_index].type != TYPE_WIDE_CHAR`

### Feature 2: General Floating-Point Conversion Selection

When a format directive has conversion code `'g'` or `'G'`, the module must apply the general floating-point formatting behavior associated with these specifiers.

Required behavior:

- recognize both lowercase `'g'` and uppercase `'G'`
- route both conversions into the formatting behavior corresponding to general-format floating-point output
- preserve any externally visible distinction implied by the conversion code case where applicable to output behavior

This requirement is directly traceable to the conditional:
`dp->conversion == 'g' || dp->conversion == 'G'`

### Feature 3: Directive-Driven Formatting Decisions

The module must base its behavior on parsed directive information and indexed argument metadata rather than on ad hoc input inspection.

Required behavior:

- use the directive's conversion field to determine formatting category
- use the directive's argument index to access the associated argument metadata
- make conversion decisions consistently from those entities

This is evidenced by both analyzed conditionals, which depend on `dp->conversion`, `dp->arg_index`, and argument type lookup.

## User Scenarios & Testing

### Scenario 1: Formatting a Narrow Character with `%c`

A caller provides a parsed format directive representing `%c` and the referenced argument is classified as a non-wide character.

Expected module behavior:

- the module selects the non-wide-character `%c` handling path
- output behavior matches the C module's `%c` behavior for that argument classification

Tests should verify:

- `%c` with a non-wide-character argument is accepted into the correct branch
- the resulting formatted output matches the original module for representative character values

### Scenario 2: Formatting a Wide Character with `%c`

A caller provides a parsed format directive representing `%c`, but the referenced argument is classified as `TYPE_WIDE_CHAR`.

Expected module behavior:

- the module does not use the non-wide-character `%c` path covered by this branch
- behavior remains distinct from the narrow-character path

Tests should verify:

- branch selection differs from Scenario 1 when argument metadata is wide-character
- the Rust implementation preserves the same classification boundary as the C source

### Scenario 3: Formatting a Floating-Point Value with `%g`

A caller provides a parsed format directive with conversion `'g'`.

Expected module behavior:

- the module selects the general floating-point formatting path for lowercase `'g'`
- output behavior matches the source module for representative floating-point inputs

Tests should verify:

- lowercase `'g'` is recognized
- representative values produce outputs matching the original behavior

### Scenario 4: Formatting a Floating-Point Value with `%G`

A caller provides a parsed format directive with conversion `'G'`.

Expected module behavior:

- the module selects the general floating-point formatting path for uppercase `'G'`
- any visible case-sensitive output distinctions remain preserved

Tests should verify:

- uppercase `'G'` is recognized
- outputs match the original behavior for representative floating-point inputs
- behavior differs from `%g` only where the original module does

### Scenario 5: Indexed Argument Resolution During Formatting

A caller provides multiple arguments and a parsed directive whose argument index points to one of them.

Expected module behavior:

- the module resolves the directive to the correct argument metadata entry
- conversion decisions are based on that indexed argument, not on positionally adjacent arguments

Tests should verify:

- varying `arg_index` changes which argument type controls `%c` path selection
- incorrect cross-argument selection does not occur

## Requirements

### Functional Requirements

- **FR-1**: The Rust module must evaluate parsed conversion directives by reading the directive conversion code.
  Traceability: `gnu/vasnprintf.c`, analyzed conditionals using `dp->conversion`.

- **FR-2**: For conversion code `'c'`, the Rust module must select the non-wide-character character formatting behavior only when the referenced argument type is not wide-character.
  Traceability: `gnu/vasnprintf.c:3560-3628`, condition `dp->conversion == 'c' && a.arg[dp->arg_index].type != TYPE_WIDE_CHAR`.

- **FR-3**: For conversion code `'c'`, the Rust module must preserve a behaviorally distinct path boundary between wide-character and non-wide-character argument classifications.
  Traceability: `gnu/vasnprintf.c:3560-3628`, same condition demonstrates explicit type-based branch exclusion.

- **FR-4**: For conversion code `'g'`, the Rust module must select the module's general floating-point formatting behavior.
  Traceability: `gnu/vasnprintf.c:5301-5551`, condition `dp->conversion == 'g' || dp->conversion == 'G'`.

- **FR-5**: For conversion code `'G'`, the Rust module must select the same general floating-point formatting category as `'g'`, while preserving any observable output distinctions associated with the uppercase specifier.
  Traceability: `gnu/vasnprintf.c:5301-5551`, same conditional.

- **FR-6**: The Rust module must resolve formatting decisions against the argument referenced by the directive's argument index.
  Traceability: `gnu/vasnprintf.c:3560-3628`, use of `dp->arg_index` and `a.arg[...]`.

### Key Entities

- **Format directive descriptor**
  A parsed formatting entity that carries at least:
  - a conversion code
  - an argument index

  Relationship:
  - drives selection of formatting behavior
  - points to the argument metadata entry used for type-sensitive decisions

  Traceability: evidenced by `dp->conversion` and `dp->arg_index` in both analyzed branches.

- **Argument metadata entry**
  A per-argument classification record that includes a type field.

  Relationship:
  - consulted by the directive logic to determine whether `%c` should use the non-wide-character path
  - indexed through the directive descriptor

  Traceability: evidenced by `a.arg[dp->arg_index].type` and comparison to `TYPE_WIDE_CHAR`.

- **Anonymous struct in `gnu/vasnprintf.c`**
  A local structural entity present in the source basis for this module analysis. The Rust port must preserve any functional role this structure has in carrying formatting-related state where needed for the above behaviors, without requiring identity with the C layout.

  Traceability: `gnu/vasnprintf.c:426-430`

## Success Criteria

- **SC-1**: For all tested `%c` cases where the referenced argument is classified as non-wide-character, the Rust module selects the same character-formatting branch behavior as the C source.
  Traceability: FR-2.

- **SC-2**: For all tested `%c` cases where the referenced argument is classified as wide-character, the Rust module does not apply the non-wide-character `%c` behavior.
  Traceability: FR-3.

- **SC-3**: For representative `%g` inputs, the Rust module produces output matching the source module's general floating-point formatting behavior.
  Traceability: FR-4.

- **SC-4**: For representative `%G` inputs, the Rust module produces output matching the source module's general floating-point formatting behavior, including any observable uppercase-specifier distinctions.
  Traceability: FR-5.

- **SC-5**: Tests that vary only the directive argument index demonstrate that the Rust module uses the indexed argument metadata entry when making `%c` type-sensitive decisions.
  Traceability: FR-6.

- **SC-6**: No tested scenario shows loss of behavioral distinction between `%c` with wide-character metadata and `%c` with non-wide-character metadata.
  Traceability: FR-2, FR-3.