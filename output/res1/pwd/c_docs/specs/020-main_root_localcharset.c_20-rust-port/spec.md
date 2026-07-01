# spec.md

## Title

Rust Functional Specification for `main_root_localcharset.c_20`

## Metadata

- Project: `pwd`
- Module: `main_root_localcharset.c_20`
- Category: `main_cluster`
- Source file: `localcharset.c`
- Primary function: `locale_charset(void) -> const char *`
- Rust branch: `020-main_root_localcharset.c_20-rust-port`
- Generation date: `2026-06-07`

## Overview

This module determines the character encoding associated with the current locale and returns that encoding name as a string.

The Rust rewrite must preserve the observable behavior of the C module’s exported functionality: given the process locale environment and platform locale configuration available at call time, it identifies the locale charset name and returns a stable encoding label representing that result. Where the source module uses internal charset mapping tables, the Rust version must preserve the same functional role: normalize platform- or locale-specific charset identifiers into the module’s returned charset name.

This specification covers only the functionality evidenced by `localcharset.c`, its `locale_charset` function, and the module’s internal table-based charset mapping data.

## Scope

### In Scope

- Determining the charset name for the current locale.
- Using locale-derived information to select an encoding label.
- Applying table-driven mapping from locale/platform-specific identifiers to returned charset names.
- Returning a string result suitable for callers that need to know the current locale encoding.

### Out of Scope

- Defining new public APIs beyond the Rust equivalent of `locale_charset`.
- Introducing configuration formats, persistence, serialization, or recovery behavior.
- Promising thread-safety, async behavior, or FFI interfaces not evidenced by the source module.
- Expanding charset detection beyond locale-based behavior present in the source module.

## Feature Specification

### Feature: Locale Charset Resolution

The module shall provide functionality equivalent to `locale_charset`, which resolves the character encoding associated with the current locale.

Behaviorally, the Rust version must:

1. Inspect locale-related runtime state as needed to determine the current locale’s encoding.
2. Interpret locale naming conventions sufficiently to extract or infer the charset component where present.
3. Use internal mapping tables to convert recognized locale/platform charset identifiers into the module’s returned charset label.
4. Return a charset name string for use by callers.
5. Provide deterministic results for the same effective locale input and mapping data.

### Feature: Charset Name Mapping

The source module contains internal `table_entry`-based mapping tables referenced from multiple points in `localcharset.c`. The Rust version must preserve this table-driven mapping behavior.

Behaviorally, mapping support must:

1. Match recognized input charset identifiers against internal mapping entries.
2. Return the mapped canonical or expected output label used by the module.
3. Fall back to the module’s remaining locale resolution behavior when a direct mapping is not applicable.

### Feature: Integration with Main Program Locale Handling

As part of the main cluster, this module supports higher-level program logic that depends on accurate locale encoding identification. The Rust rewrite must continue to serve as the charset-resolution component used by the rest of the program, with no requirement for callers to supply additional parameters.

## User Scenarios & Testing

### Scenario 1: Program needs current locale encoding

A caller in the main program needs to know the encoding associated with the process locale in order to interpret or emit text correctly.

**Expected behavior**
- Calling the Rust equivalent of `locale_charset` returns a non-empty charset label derived from the current locale context.
- The same locale context yields the same returned charset label.

**Testing approach**
- Set locale-related environment/runtime state to a known locale.
- Invoke the function.
- Verify that a charset string is returned and is consistent across repeated calls under unchanged locale settings.

### Scenario 2: Locale name includes an explicit codeset

The current locale naming convention explicitly carries a codeset component.

**Expected behavior**
- The module extracts or otherwise recognizes that codeset information.
- If the codeset identifier is covered by internal mapping data, the returned value reflects the mapped result.
- If no remapping is needed, the returned value reflects the identified charset name.

**Testing approach**
- Use test locales whose names include explicit encoding identifiers.
- Verify that returned values match expected mapped or direct charset labels according to the source module behavior.

### Scenario 3: Platform- or locale-specific charset aliases require normalization

A locale or platform reports a charset identifier that is not the final label returned by the module and must be normalized through mapping tables.

**Expected behavior**
- The module recognizes the alias or platform-specific identifier.
- It returns the mapped charset label defined by the module’s table data.

**Testing approach**
- Exercise inputs corresponding to known table-mapped identifiers from `localcharset.c`.
- Verify exact string equality with the expected mapped output.

### Scenario 4: Caller relies on stable string result semantics

Program logic calls the function and uses the returned charset label without additional ownership negotiation.

**Expected behavior**
- The Rust rewrite exposes equivalent practical behavior: callers can obtain and use the returned charset name directly as a string result.
- The returned value remains valid for ordinary caller use consistent with the C module’s API role.

**Testing approach**
- Call the function from consuming code paths that immediately compare, log, or branch on the returned charset string.
- Verify compatibility of those operations with the Rust API chosen for the port.

## Requirements

### Functional Requirements

#### FR-1: Resolve current locale charset
The module shall provide functionality equivalent to `locale_charset(void)` that determines the charset associated with the current locale context.

**Traceability:** `localcharset.c`, `locale_charset`

#### FR-2: Return charset as string
The module shall return the resolved charset as a string value representing the locale encoding name.

**Traceability:** `localcharset.c`, `locale_charset`

#### FR-3: Use locale-derived input
Charset resolution shall be based on locale-derived information available to the process at runtime, rather than requiring caller-supplied charset input.

**Traceability:** `localcharset.c`, `locale_charset`

#### FR-4: Support table-driven charset normalization
The module shall apply internal table-driven mapping when locale/platform charset identifiers require conversion to the module’s returned charset label.

**Traceability:** `localcharset.c`, `locale_charset`, `struct table_entry` references

#### FR-5: Preserve recognized mapping outcomes
For charset identifiers recognized by the source module’s mapping tables, the Rust rewrite shall produce the same mapped output labels as the source behavior.

**Traceability:** `localcharset.c`, `locale_charset`, `struct table_entry` references

#### FR-6: Behave deterministically for fixed locale state
For unchanged locale runtime state, repeated invocations shall produce the same charset result.

**Traceability:** `localcharset.c`, `locale_charset`

### Key Entities

#### `table_entry`
A table record used for internal charset mapping. The source analysis shows multiple anonymous struct definitions/usages associated with `table_entry` within `localcharset.c`, indicating a repeated internal role for mapping one charset-related identifier to another.

**Functional role**
- Represents one mapping relationship used during charset resolution.
- Participates in internal lookup logic used by `locale_charset`.

**Relationship to module behavior**
- `locale_charset` consults collections of `table_entry` data to normalize or translate locale/platform-specific charset names into the module’s returned charset label.

#### Locale charset result
The resolved charset name returned by `locale_charset`.

**Functional role**
- Final output consumed by the rest of the program.
- Derived from locale state, optionally transformed by `table_entry`-based mapping.

**Relationship to other entities**
- Produced by `locale_charset`.
- May be a direct locale-derived charset identifier or a mapped value selected through `table_entry` data.

## Success Criteria

### SC-1: Functional equivalence of primary API
The Rust module provides a function equivalent in purpose to `locale_charset` and returns a charset name for the active locale context.

**Traceability:** `localcharset.c`, `locale_charset`

### SC-2: Correct mapping preservation
For test cases derived from source-supported charset mapping entries, the Rust implementation returns the same mapped charset labels as the source module.

**Traceability:** `localcharset.c`, `locale_charset`, `struct table_entry` references

### SC-3: Deterministic repeated calls
With locale settings unchanged, repeated calls return identical charset strings.

**Traceability:** `localcharset.c`, `locale_charset`

### SC-4: Scenario coverage
The Rust implementation passes tests covering:
- locale contexts with explicit codeset identifiers,
- locale/platform charset aliases requiring mapping,
- ordinary caller retrieval and use of the returned charset string.

**Traceability:** `localcharset.c`, `locale_charset`, `struct table_entry` references

### SC-5: No unsupported feature expansion
The Rust rewrite remains limited to the evidenced module role of locale charset resolution and internal mapping, without adding unrelated public capabilities.

**Traceability:** `localcharset.c`, `locale_charset`, `struct table_entry` references