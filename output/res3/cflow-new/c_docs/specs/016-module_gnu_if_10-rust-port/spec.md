# spec.md

## Title

Functional Specification: `module_gnu_if_10` Rust Port

## Document Metadata

- Project: `cflow-new`
- Module: `module_gnu_if_10`
- Category: `module_cluster`
- Source basis: `gnu/vasnprintf.c`
- Rust branch: `016-module_gnu_if_10-rust-port`
- Generation date: 2026-06-17

## 1. Feature Specification

### 1.1 Overview

This module covers the conditional formatting behavior in `gnu/vasnprintf.c` for format items whose conversion kind is:

- string conversion (`'s'`) with behavior dependent on whether the selected argument is a wide string or not, and
- character conversion (`'c'`) when the selected argument is a wide character.

The Rust rewrite must preserve the same functional decision boundaries evidenced by the source analysis: it must correctly distinguish between narrow and wide argument kinds for these conversion cases and route formatting behavior accordingly.

### 1.2 In-Scope Functionality

The Rust version must implement the following module functionality:

1. **String conversion classification for `%s`-style formatting**
   - When processing a format directive whose conversion is `'s'`, the module must inspect the referenced argument type and distinguish between wide-string and non-wide-string cases.
   - The distinction must follow the source conditional split that depends on build mode (`WIDE_CHAR_VERSION`) and the argument type classification.

2. **Wide-character classification for `%c`-style formatting**
   - When processing a format directive whose conversion is `'c'`, the module must detect the case where the referenced argument is a wide character.
   - This case must be handled as a distinct formatting path from ordinary narrow-character formatting.

3. **Argument-type-sensitive formatting dispatch**
   - The module must use the format directive’s conversion code together with the selected argument’s classified type to choose the correct formatting behavior.
   - Dispatch decisions must be based on the directive metadata and indexed argument metadata, not on guessed or implicit conversions.

### 1.3 Out of Scope

The following are not specified here because they are not evidenced by the provided module analysis:

- Defining new formatting syntax
- Adding public APIs beyond what is required to preserve module behavior
- Guarantees about thread safety, async behavior, serialization, FFI, or recovery behavior
- Performance targets or benchmarking requirements
- Formatting behaviors for conversions other than the evidenced `'s'` and `'c'` conditional branches

## 2. User Scenarios & Testing

### 2.1 Scenario: Formatting a string directive with a non-wide string argument

A caller processes a format directive with conversion `'s'` and the selected argument is classified as a non-wide string for the active build mode.

**Expected behavior:**
- The module recognizes that this directive belongs to the non-wide string path.
- It does not misclassify the argument as a wide string.
- The resulting dispatch follows the ordinary string-formatting branch for that mode.

**Testing guidance:**
- Provide a parsed directive with conversion `'s'`.
- Provide indexed argument metadata indicating the non-wide string case for the active mode.
- Verify the Rust module selects the same branch as the C logic.

### 2.2 Scenario: Formatting a string directive with a wide string argument

A caller processes a format directive with conversion `'s'` and the selected argument is classified as a wide string for the active build mode.

**Expected behavior:**
- The module recognizes that this directive belongs to the wide-string-related path as defined by the source condition.
- The branch decision changes appropriately when the argument classification changes.

**Testing guidance:**
- Use the same directive shape as above, but switch the argument type classification to wide string.
- Verify the Rust module’s branch selection matches the source conditional semantics.

### 2.3 Scenario: Formatting a character directive with a wide character argument

A caller processes a format directive with conversion `'c'` and the selected argument is classified as a wide character.

**Expected behavior:**
- The module identifies this as the wide-character case.
- It dispatches to the distinct path for wide character handling rather than treating the argument as an ordinary narrow character.

**Testing guidance:**
- Provide directive metadata with conversion `'c'`.
- Provide indexed argument metadata indicating wide character type.
- Verify the module selects the wide-character branch.

### 2.4 Scenario: Indexed argument lookup drives conversion behavior

A caller uses a format directive whose argument is selected by index from parsed argument metadata.

**Expected behavior:**
- The module uses the directive’s `arg_index` to inspect the corresponding argument classification.
- Branch selection depends on the indexed argument entry, not on unrelated arguments.

**Testing guidance:**
- Construct multiple argument metadata entries with differing types.
- Point the directive at one specific entry.
- Verify only that referenced entry affects the branch decision.

## 3. Requirements

### 3.1 Functional Requirements

- **FR-1:** The module shall evaluate format directives based on a conversion discriminator and referenced argument type classification.
  **Traceability:** `gnu/vasnprintf.c`, conditional branches at lines 2914-3366 and 3369-3557.

- **FR-2:** For conversion `'s'`, the module shall distinguish between wide-string and non-wide-string argument cases according to the source conditional logic, including the build-mode-sensitive interpretation tied to `WIDE_CHAR_VERSION`.
  **Traceability:** `gnu/vasnprintf.c`, branch at lines 2914-3366.

- **FR-3:** For conversion `'c'`, the module shall recognize the case where the referenced argument type is `TYPE_WIDE_CHAR` and treat it as a distinct formatting branch.
  **Traceability:** `gnu/vasnprintf.c`, branch at lines 3369-3557.

- **FR-4:** The module shall determine argument type by consulting the argument array entry referenced by the directive’s argument index.
  **Traceability:** `gnu/vasnprintf.c`, both evidenced branches reference `a.arg[dp->arg_index].type`.

- **FR-5:** The module shall preserve the behavioral distinction between `TYPE_WIDE_STRING` and `TYPE_WIDE_CHAR` classifications where they affect conversion handling.
  **Traceability:** `gnu/vasnprintf.c`, branches at lines 2914-3366 and 3369-3557.

### 3.2 Key Entities

#### Format Directive Descriptor

A directive descriptor entity is evidenced by use of:

- `dp->conversion`
- `dp->arg_index`

**Role:**
- Identifies which conversion is being processed.
- Identifies which argument metadata entry controls behavior.

**Relationship:**
- References one argument entry through `arg_index`.

#### Argument Metadata Collection

An argument collection entity is evidenced by use of:

- `a.arg[...]`
- `a.arg[...].type`

**Role:**
- Stores per-argument type classification used by formatting decisions.

**Relationship:**
- Supplies the type information consulted by a directive descriptor.

#### Argument Type Classification

The type classification domain is evidenced by:

- `TYPE_WIDE_STRING`
- `TYPE_WIDE_CHAR`

**Role:**
- Distinguishes wide-string and wide-character cases from other argument kinds.

**Relationship:**
- Each argument metadata entry has one type classification.
- Directive behavior depends on the combination of conversion code and this classification.

#### Local Anonymous Struct

The source analysis evidences an anonymous `struct` at `gnu/vasnprintf.c:426-430`.

**Role in specification:**
- The Rust port must preserve any behaviorally relevant state represented by this local structured data if it participates in directive or argument classification for the covered paths.
- No additional fields or semantics are specified here beyond what is evidenced.

## 4. Success Criteria

- **SC-1:** For every tested directive with conversion `'s'`, changing only the referenced argument type between wide-string and non-wide-string classifications changes branch selection in the same way as the source logic.
  **Traceability:** branch at `gnu/vasnprintf.c:2914-3366`.

- **SC-2:** For every tested directive with conversion `'c'`, when the referenced argument type is `TYPE_WIDE_CHAR`, the Rust port selects the wide-character handling path.
  **Traceability:** branch at `gnu/vasnprintf.c:3369-3557`.

- **SC-3:** Tests demonstrate that argument lookup is index-driven: changing non-referenced argument entries does not change the selected branch, while changing the referenced entry does.
  **Traceability:** both evidenced branches use `dp->arg_index`.

- **SC-4:** The Rust implementation preserves the distinction between wide-string-sensitive `%s` behavior and wide-character-sensitive `%c` behavior without collapsing them into a single undifferentiated path.
  **Traceability:** separate source branches at `gnu/vasnprintf.c:2914-3366` and `3369-3557`.

- **SC-5:** The Rust port introduces no unsupported behavior claims beyond these evidenced formatting decisions.
  **Traceability:** constrained by analyzed module evidence from `gnu/vasnprintf.c`.