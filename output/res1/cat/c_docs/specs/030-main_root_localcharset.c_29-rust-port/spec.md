# spec.md

## Overview

This module provides charset detection for the current process locale through a single public function, `locale_charset()`, implemented in `localcharset.c`. Its purpose is to return a charset name describing the encoding associated with the active locale environment, suitable for use by higher-level text-processing logic in the `cat` project.

The Rust rewrite must preserve the functional role of this module: determine the locale-dependent character encoding name and return it as a stable string result to callers, including handling platform- or locale-specific naming differences through internal mapping tables represented by `table_entry` records.

## Scope

In scope:

- Determining the character set name for the current locale.
- Applying built-in alias or normalization mappings needed to translate locale-derived encoding identifiers into the module’s returned charset name.
- Returning a string result for caller consumption without requiring caller-provided buffers.

Out of scope:

- General locale management.
- Text conversion between encodings.
- Validation of arbitrary external charset names unrelated to the current locale lookup.
- Any public API beyond the behavior represented by `locale_charset()`.

## Feature Specification

### Feature: Locale charset resolution

The module resolves the charset associated with the current locale and returns it as a string.

#### Required behavior

- The Rust version must provide behavior equivalent to `locale_charset()` from `localcharset.c`.
- The function must inspect the current locale context and derive a charset name from it.
- The function must account for locale naming forms where the encoding is explicit and for cases where a platform-specific lookup or alias mapping is needed.
- The function must use embedded mapping data equivalent in role to the C module’s `table_entry` tables to normalize or translate locale/encoding identifiers to the returned charset name.
- The function must return a string naming the detected charset, not a structured object.
- The returned charset name must be suitable for repeated use by callers during program execution.

#### Observable outcomes

- When the locale clearly specifies an encoding, the module returns the corresponding charset name.
- When locale-derived names require alias resolution, the module returns the mapped canonical or expected charset name defined by the module’s built-in tables.
- When platform or locale conventions differ, the module still returns the module-defined charset result for the active locale context.

## User Scenarios & Testing

### Scenario 1: Caller asks for the current locale encoding

A caller in the main program needs to know what encoding the current locale implies before processing text.

**Expected result:**
Calling the Rust equivalent of `locale_charset()` returns a non-empty charset name string corresponding to the active locale.

**Test approach:**
- Run under a locale with an explicit encoding in its name.
- Verify that the returned string matches the encoding expected from that locale form.

### Scenario 2: Locale uses an alias or non-canonical encoding name

A caller runs in an environment where the locale or system reports an encoding name that is not the final name the module returns directly.

**Expected result:**
The module applies its internal mapping rules and returns the mapped charset name.

**Test approach:**
- Use inputs or environments corresponding to entries covered by the C module’s mapping tables.
- Verify that the Rust result matches the same mapped name as the C module.

### Scenario 3: Platform-specific locale naming differences

A caller runs on a system where locale encoding information is represented using platform-specific conventions.

**Expected result:**
The module still resolves and returns the charset name produced by the original C behavior for that environment.

**Test approach:**
- Compare Rust and C outputs on the same supported target/environment.
- Confirm behavioral equivalence of the returned charset string.

### Scenario 4: Repeated queries during one process run

A caller invokes locale charset detection more than once.

**Expected result:**
The module continues to return a usable charset name string on each call.

**Test approach:**
- Call the function repeatedly under an unchanged locale environment.
- Verify that each call returns the same charset name as the original module for that environment.

## Requirements

### Functional Requirements

- **FR-1**: The module shall provide locale charset detection behavior equivalent to `locale_charset()` in `localcharset.c`.
  **Traceability:** `localcharset.c`, `locale_charset`.

- **FR-2**: The module shall return the detected charset as a string result.
  **Traceability:** `localcharset.c`, `locale_charset`.

- **FR-3**: The module shall derive the charset from the current locale context rather than from caller-supplied charset input.
  **Traceability:** `localcharset.c`, `locale_charset`.

- **FR-4**: The module shall apply internal lookup or alias-mapping rules represented by `table_entry` data to translate locale- or system-derived names into the returned charset name where needed.
  **Traceability:** `localcharset.c`, `table_entry`, `locale_charset`.

- **FR-5**: The module shall support the locale/encoding naming variations handled by the original module’s built-in mapping tables.
  **Traceability:** `localcharset.c`, `table_entry`, `locale_charset`.

- **FR-6**: The module shall produce results consistent with the original module for the same locale environment on supported targets.
  **Traceability:** `localcharset.c`, `locale_charset`, `table_entry`.

### Key Entities

- **Locale charset result**: The string value returned to callers naming the charset associated with the current locale.
  **Traceability:** `localcharset.c`, `locale_charset`.

- **`table_entry`**: Internal mapping record type used by the module’s built-in tables to associate one locale- or encoding-related identifier with another returned charset name or intermediate normalized form.
  **Traceability:** `localcharset.c`, anonymous `struct table_entry` definitions and references.

- **Mapping tables**: Collections of `table_entry` records used to resolve aliases, normalize names, or handle platform-specific locale encoding conventions before producing the final charset result.
  **Traceability:** `localcharset.c`, `table_entry`, `locale_charset`.

## Success Criteria

- **SC-1**: For locale environments exercised by the original C module, the Rust module returns the same charset string as `locale_charset()` for the same environment.
  **Traceability:** `localcharset.c`, `locale_charset`.

- **SC-2**: In tests covering locale or encoding aliases represented by the C module’s mapping tables, the Rust module returns the same mapped charset names as the C module.
  **Traceability:** `localcharset.c`, `table_entry`, `locale_charset`.

- **SC-3**: Repeated calls under an unchanged locale environment produce a usable charset string and match the C module’s observed result.
  **Traceability:** `localcharset.c`, `locale_charset`.

- **SC-4**: The Rust rewrite does not require any additional caller input beyond invoking the locale charset query behavior represented by the original module.
  **Traceability:** `localcharset.c`, `locale_charset`.