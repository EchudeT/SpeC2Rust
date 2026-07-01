# spec.md

## Title

Functional Specification for `module_gnu_if_10` Rust Port

## Document Metadata

- Project: `cflow-new`
- Module: `module_gnu_if_10`
- Category: `module_cluster`
- Source file: `gnu/vasnprintf.c`
- Rust branch: `016-module_gnu_if_10-rust-port`
- Generation date: `2026-06-17`

## Overview

This module covers the conditional formatting behavior in `vasnprintf` for character and string conversions when the selected argument type is wide-character based. The analyzed behavior is limited to the decision paths for:

- string conversion (`%s`) when the active argument type and build mode determine whether a wide string path is applicable, and
- character conversion (`%c`) when the selected argument is a wide character.

The Rust rewrite must preserve the same functional selection behavior evidenced by these conditions. This specification is intentionally limited to those conditional responsibilities and does not define unrelated formatting features.

## Scope

### In Scope

- Selection of the formatting path for string conversions based on:
  - conversion kind being string,
  - whether wide-character mode is active, and
  - whether the referenced argument is typed as a wide string.
- Selection of the formatting path for character conversions when the referenced argument is typed as a wide character.
- Use of parsed directive data and indexed argument type information to make those selections.

### Out of Scope

- A complete specification of all `vasnprintf` formatting behavior.
- Numeric formatting, flags, width, precision, locale behavior, memory growth policy, or output buffering behavior not evidenced by the analyzed conditions.
- New public APIs or behavior beyond the source module’s evidenced conditional responsibilities.

## Feature Specification

The Rust module must implement the conditional decision logic used during format processing to recognize when a formatting directive should be handled as a wide-string or wide-character case.

### Feature 1: String conversion classification with wide-string awareness

When processing a formatting directive whose conversion kind is string (`s`), the module must determine whether the directive matches the wide-string-specific branch by combining:

- the directive conversion value,
- the argument type referenced by the directive’s argument index, and
- the active wide-character build variant.

The source evidence shows that this classification differs by build mode:

- In wide-character builds, the relevant branch is taken when the conversion is `s` and the referenced argument is **not** a wide string.
- In non-wide-character builds, the relevant branch is taken when the conversion is `s` and the referenced argument **is** a wide string.

The Rust port must preserve this mode-dependent classification exactly.

### Feature 2: Character conversion classification for wide characters

When processing a formatting directive whose conversion kind is character (`c`), the module must identify the wide-character-specific case when the referenced argument type is a wide character.

The Rust port must preserve this classification exactly.

### Feature 3: Indexed argument inspection

The above decisions must be made using:

- the current directive’s conversion field,
- the current directive’s argument index, and
- the argument-type table for the referenced argument.

The Rust port must therefore support looking up the type of the argument selected by the directive and applying the corresponding conditional logic.

## User Scenarios & Testing

### Scenario 1: Non-wide build receives `%s` with a wide-string argument

A caller processes a format directive for string conversion. The directive references an argument whose type metadata marks it as a wide string. In the non-wide build variant, the module must classify this directive into the wide-string-related conditional branch.

**Test expectation:**
- Given conversion `s`
- Given argument type `TYPE_WIDE_STRING`
- Given non-wide build mode
- The module selects the branch corresponding to the source condition.

### Scenario 2: Wide-character build receives `%s` with a non-wide-string argument

A caller processes a format directive for string conversion in the wide-character build variant. The directive references an argument whose type metadata is not `TYPE_WIDE_STRING`. The module must classify this directive into the corresponding conditional branch for that build mode.

**Test expectation:**
- Given conversion `s`
- Given argument type other than `TYPE_WIDE_STRING`
- Given wide-character build mode
- The module selects the branch corresponding to the source condition.

### Scenario 3: `%s` does not match the branch when the type/build combination does not satisfy the source condition

A caller processes a string conversion directive, but the argument type and build mode do not satisfy the analyzed condition. The module must not classify the directive as matching this specific branch.

**Test expectation:**
- For each build mode, provide the opposite argument-type case
- Verify the branch is not selected

### Scenario 4: `%c` with a wide-character argument

A caller processes a character conversion directive. The directive references an argument whose type metadata marks it as a wide character. The module must classify the directive into the wide-character-related conditional branch.

**Test expectation:**
- Given conversion `c`
- Given argument type `TYPE_WIDE_CHAR`
- The module selects the branch corresponding to the source condition

### Scenario 5: `%c` with a non-wide-character argument

A caller processes a character conversion directive whose referenced argument is not typed as a wide character. The module must not classify it as the wide-character-specific case.

**Test expectation:**
- Given conversion `c`
- Given argument type other than `TYPE_WIDE_CHAR`
- The branch is not selected

### Scenario 6: Directive uses indexed argument metadata

A caller processes multiple directives with different argument indices. The module must determine branch selection using the type of the argument referenced by each directive’s own index, not by position-independent assumptions.

**Test expectation:**
- Prepare multiple directives with distinct argument indices
- Assign different types to those indexed arguments
- Verify classification follows the referenced index for each directive

## Requirements

### Functional Requirements

#### FR-1: String conversion branch recognition
The Rust module shall recognize the analyzed string-conversion conditional only when the directive conversion is `s` and the referenced argument type/build-mode combination matches the source condition evidenced in `gnu/vasnprintf.c`.

**Traceability:** `gnu/vasnprintf.c`, analyzed `if` at lines 2914-3366.

#### FR-2: Build-mode-dependent wide-string logic
The Rust module shall preserve the build-mode-dependent distinction for string conversion classification:
- in wide-character mode, match when the referenced argument is not `TYPE_WIDE_STRING`;
- otherwise, match when the referenced argument is `TYPE_WIDE_STRING`.

**Traceability:** `gnu/vasnprintf.c`, analyzed `if` at lines 2914-3366.

#### FR-3: Wide-character conversion branch recognition
The Rust module shall recognize the analyzed character-conversion conditional only when the directive conversion is `c` and the referenced argument type is `TYPE_WIDE_CHAR`.

**Traceability:** `gnu/vasnprintf.c`, analyzed `if` at lines 3369-3557.

#### FR-4: Argument-index-based type lookup
The Rust module shall evaluate the above conditions using the current directive’s argument index to access the referenced argument type metadata.

**Traceability:** `gnu/vasnprintf.c`, analyzed `if` at lines 2914-3366 and 3369-3557.

#### FR-5: Non-matching cases remain excluded
The Rust module shall not classify directives into these branches when either the conversion code or the referenced argument type does not satisfy the corresponding analyzed condition.

**Traceability:** implied by both analyzed conditionals in `gnu/vasnprintf.c`.

### Key Entities

#### Formatting directive record
An internal directive data record stores at least the conversion specifier and the argument index used to inspect argument metadata during classification.

**Relationship:**
- Each directive refers to one argument entry through its argument index.

**Traceability:** usage of `dp->conversion` and `dp->arg_index` in both analyzed conditionals.

#### Argument metadata collection
An indexed argument metadata collection stores the type of each available argument. The analyzed logic accesses the selected argument entry and reads its type to determine whether it is a wide string or wide character.

**Relationship:**
- The directive record indexes into this collection.
- The collection contains entries with a `type` field.

**Traceability:** usage of `a.arg[dp->arg_index].type` in both analyzed conditionals.

#### Core supporting struct
The source analysis identifies an anonymous struct in `gnu/vasnprintf.c` that participates in the module’s data model. The Rust port must preserve the functional role of source-side structured state needed to evaluate directive conversion and argument typing, without requiring identical layout.

**Traceability:** anonymous struct at lines 426-430.

## Success Criteria

### SC-1: Correct `%s` classification in non-wide mode
For test inputs representing non-wide mode, 100% of `%s` directives referencing `TYPE_WIDE_STRING` arguments are classified as matching the analyzed branch, and 100% of `%s` directives referencing non-wide-string arguments are not.

**Traceability:** `gnu/vasnprintf.c`, analyzed `if` at lines 2914-3366.

### SC-2: Correct `%s` classification in wide-character mode
For test inputs representing wide-character mode, 100% of `%s` directives referencing non-`TYPE_WIDE_STRING` arguments are classified as matching the analyzed branch, and 100% of `%s` directives referencing `TYPE_WIDE_STRING` arguments are not.

**Traceability:** `gnu/vasnprintf.c`, analyzed `if` at lines 2914-3366.

### SC-3: Correct `%c` wide-character classification
For test inputs with conversion `c`, 100% of directives referencing `TYPE_WIDE_CHAR` arguments are classified as matching the analyzed branch, and 100% of directives referencing other argument types are not.

**Traceability:** `gnu/vasnprintf.c`, analyzed `if` at lines 3369-3557.

### SC-4: Correct use of directive argument indices
In tests with multiple directives and multiple argument entries, branch-selection results always correspond to the type at the directive’s referenced argument index.

**Traceability:** `gnu/vasnprintf.c`, use of `dp->arg_index` in both analyzed conditionals.

### SC-5: No unsupported behavior claims
The Rust module specification and implementation remain limited to the evidenced conditional formatting responsibilities and do not introduce required behavior beyond the analyzed source conditions.

**Traceability:** bounded by analyzed source coverage in `gnu/vasnprintf.c`.

## Acceptance Notes

- This specification defines only the evidenced conditional behavior for wide-string and wide-character classification inside `vasnprintf`.
- If the Rust rewrite reorganizes the source into different functions or data types, the observable branch-selection behavior must still satisfy all functional requirements and success criteria above.