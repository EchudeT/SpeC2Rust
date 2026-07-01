# spec.md

## Title
Rust Functional Specification for `main_root_localcharset.c_29`

## Metadata
- Project: `cat`
- Module: `main_root_localcharset.c_29`
- Category: `main_cluster`
- Source file: `localcharset.c`
- Primary function: `locale_charset(void) -> const char *`
- Rust branch: `030-main_root_localcharset.c_29-rust-port`
- Generation date: 2026-06-09

## Overview
This module determines the character encoding associated with the current locale and returns that encoding name as a string.

The Rust rewrite must preserve the module’s externally visible behavior as evidenced by `locale_charset` in `localcharset.c`. Its responsibility is limited to resolving a locale-dependent charset name, including the use of built-in alias/mapping data represented by `table_entry` records where needed to normalize or translate platform- or locale-specific encoding identifiers into the module’s returned charset name.

## Feature Specification

### Summary
The Rust module must provide the functional equivalent of `locale_charset()`:
- inspect the process locale context relevant to character encoding,
- derive the charset name associated with that locale,
- apply built-in mapping or alias resolution represented by the module’s charset tables,
- return a stable string result representing the detected charset.

### In-scope behavior
The Rust version must implement the following observed functional boundary:
1. Resolve the locale charset for the current runtime environment.
2. Use module-owned charset mapping data to interpret or normalize encoding identifiers when direct locale-provided values are not already in the desired form.
3. Return the resolved charset name as a string value suitable for callers that need to know the active locale encoding.

### Out-of-scope behavior
The Rust version must not introduce unevidenced behavior, including:
- new public APIs beyond the Rust equivalent of `locale_charset`,
- configuration systems,
- persistence or serialization,
- explicit thread-safety guarantees,
- recovery workflows beyond ordinary function return behavior,
- charset conversion or transcoding,
- locale mutation.

## User Scenarios & Testing

### Scenario 1: Caller needs the current locale encoding
A caller running under a configured locale invokes the module to obtain the charset name associated with that locale.

Expected result:
- The module returns a non-empty charset name reflecting the current locale context.

### Scenario 2: Locale encoding name requires alias normalization
A caller runs in an environment where the locale or platform exposes an encoding identifier that is represented in the module’s internal mapping tables.

Expected result:
- The module resolves that identifier through the built-in mapping data and returns the corresponding charset name defined by the module behavior.

### Scenario 3: Platform-specific locale encoding discovery path
A caller runs on a platform or locale configuration where charset detection requires platform-dependent interpretation rather than a single universal source.

Expected result:
- The module still returns a charset name through the supported locale-detection logic embodied by `locale_charset`.

### Scenario 4: Repeated queries in the same locale context
A caller invokes the charset query multiple times without changing locale context.

Expected result:
- Each call returns the same effective charset result for that unchanged locale context.

### Test coverage expectations
The Rust rewrite must be testable against the following behavior classes:
- locale-derived charset resolution returns a string result;
- known alias or mapping-table inputs produce the expected normalized charset output;
- repeated calls under unchanged locale conditions remain behaviorally consistent;
- unsupported or indirect locale naming forms are handled through the same resolution rules as the C module, without inventing fallback semantics not evidenced by the source.

## Requirements

### Functional Requirements

#### FR-1: Locale charset query
The module shall expose a Rust equivalent of `locale_charset` that returns the charset name associated with the current locale context.

Traceability:
- Function: `locale_charset` (`localcharset.c:830-1159`)

#### FR-2: Locale-dependent resolution
The module shall determine its result from runtime locale information rather than from a caller-supplied charset parameter.

Traceability:
- Function: `locale_charset` (`localcharset.c:830-1159`)

#### FR-3: Mapping-table-based normalization
The module shall support normalization or translation of charset identifiers using the module’s built-in table data where required by the resolution logic.

Traceability:
- Function: `locale_charset` (`localcharset.c:830-1159`)
- Types: anonymous `struct table_entry` definitions and references in `localcharset.c`

#### FR-4: String result representing charset identity
The module shall return the resolved charset as a string value naming the detected encoding.

Traceability:
- Function signature: `const char * locale_charset (void);`
- Function: `locale_charset` (`localcharset.c:830-1159`)

#### FR-5: Deterministic result for unchanged locale state
For repeated invocations under the same effective locale conditions, the module shall produce the same effective charset name.

Traceability:
- Function: `locale_charset` (`localcharset.c:830-1159`)

### Key Entities

#### `table_entry`
A table record used by the module’s built-in charset mapping data.

Role:
- represents one mapping relation used during charset name resolution or normalization.

Relationships:
- `locale_charset` consults collections of `table_entry` records to map locale/platform encoding identifiers to returned charset names.

Traceability:
- Types: anonymous `struct table_entry` occurrences in `localcharset.c` and referenced `table_entry`

#### Locale charset result string
The resolved charset name returned by the module.

Role:
- communicates the encoding identity associated with the current locale to callers.

Relationships:
- produced by `locale_charset`;
- may be derived directly from locale information or via `table_entry`-backed mapping.

Traceability:
- Function signature and body: `locale_charset` (`localcharset.c:830-1159`)

## Success Criteria

### SC-1: Functional equivalence of primary behavior
The Rust module provides a callable equivalent of `locale_charset` that returns a charset name for the current locale context.

Traceability:
- Function: `locale_charset` (`localcharset.c:830-1159`)

### SC-2: Mapping behavior preserved
For locale or encoding identifiers covered by the module’s built-in mapping tables, the Rust implementation returns the same normalized or translated charset names as the C module.

Traceability:
- Function: `locale_charset` (`localcharset.c:830-1159`)
- Types: `table_entry` table definitions/references

### SC-3: Stable behavior across repeated calls
Under unchanged locale conditions, repeated calls to the Rust implementation yield the same effective charset result.

Traceability:
- Function: `locale_charset` (`localcharset.c:830-1159`)

### SC-4: No unevidenced scope expansion
The Rust rewrite remains limited to locale charset detection and mapping behavior present in this module and does not add unrelated public functionality.

Traceability:
- Module file: `localcharset.c`
- Function: `locale_charset` (`localcharset.c:830-1159`)