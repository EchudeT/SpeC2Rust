# spec.md

## Title
Functional Specification for `main_root_localcharset.c_20` Rust Port

## Document Control
- Project: `pwd`
- Module: `main_root_localcharset.c_20`
- Category: `main_cluster`
- Source file: `localcharset.c`
- Primary function: `locale_charset(void) -> const char *`
- Rust branch: `020-main_root_localcharset.c_20-rust-port`
- Generation date: `2026-06-07`

## Overview
This module determines the character encoding associated with the process locale and returns it as a charset name string.

The Rust rewrite must preserve the module’s observable behavior as a locale-to-charset resolver. Its scope is limited to:
- obtaining the active locale context relevant to character encoding,
- mapping locale information to a charset name,
- returning a stable charset name result for callers of the module’s main function.

The specification is limited to behavior evidenced by `localcharset.c` and its `locale_charset` function, plus the table-driven mapping structures referenced in that file.

## Feature Specification

### Feature: Resolve locale charset name
The module shall provide functionality equivalent to `locale_charset(void)` that returns the charset name corresponding to the current locale environment.

Behavior covered by this feature:
- inspect locale-related process state sufficient to determine the active locale encoding,
- recognize locale encodings when directly specified in locale naming,
- normalize or translate recognized locale encoding identifiers through built-in mapping tables where applicable,
- return a charset name string suitable for use by the rest of the program.

### Functional boundary
The Rust port shall implement only the charset-resolution behavior evidenced by this module. It shall not introduce:
- additional public APIs beyond the module-equivalent entry point needed by the port,
- new configuration surfaces,
- persistence, serialization, recovery, or benchmarking behavior,
- guarantees not evidenced by the source module.

## User Scenarios & Testing

### Scenario 1: Locale explicitly names an encoding
A caller invokes the module while the current locale name contains an explicit codeset component.

Expected behavior:
- the module extracts or recognizes the encoding information from the locale context,
- the returned value corresponds to that encoding, after any module-defined canonicalization or table lookup.

Test focus:
- locale names that include a codeset suffix,
- returned charset string is non-empty and matches expected mapped value.

### Scenario 2: Locale requires table-based normalization
A caller invokes the module in an environment where the locale’s encoding identifier is recognized only through one of the module’s built-in mapping tables.

Expected behavior:
- the module consults the relevant mapping data,
- the module returns the mapped charset name rather than an unrecognized raw token.

Test focus:
- inputs known to require translation through `table_entry`-based mappings,
- output equals the module’s canonical mapped name.

### Scenario 3: Locale does not provide a directly usable encoding token
A caller invokes the module when the locale information does not expose an immediately usable codeset name.

Expected behavior:
- the module still resolves the charset using the fallback behavior evidenced by the source logic,
- the function returns a usable charset name string.

Test focus:
- locale contexts lacking explicit encoding suffixes,
- output remains non-null and non-empty.

### Scenario 4: Repeated calls under unchanged locale state
A caller invokes the module multiple times without changing locale-related process state.

Expected behavior:
- each call returns the same charset result for the same locale state.

Test focus:
- consecutive calls,
- stable returned value equality by string content.

## Requirements

### Functional Requirements

#### FR-1: Locale charset query
The Rust module shall expose behavior equivalent to `locale_charset(void)` from `localcharset.c`, returning the charset name for the current locale context.

Traceability:
- `localcharset.c`
- `locale_charset`

#### FR-2: Locale-derived encoding recognition
The module shall derive charset information from locale-related naming/state when such information is present in the active locale.

Traceability:
- `localcharset.c`
- `locale_charset`

#### FR-3: Table-driven charset mapping
The module shall support table-driven translation from locale- or platform-specific encoding identifiers to returned charset names where such mappings are represented by the module’s `table_entry` data.

Traceability:
- `localcharset.c`
- `locale_charset`
- `struct table_entry` occurrences at the referenced source locations

#### FR-4: Canonical return as string name
The module shall return the resolved charset as a string name consumable by callers, matching the module’s mapping and normalization behavior.

Traceability:
- `localcharset.c`
- `locale_charset`

#### FR-5: Fallback resolution within module scope
When a directly recognizable encoding name is not available from the locale context, the module shall apply the fallback resolution paths implemented in the source module before producing the return value.

Traceability:
- `localcharset.c`
- `locale_charset`

#### FR-6: Deterministic result for unchanged locale state
For repeated invocations under unchanged locale-related process state, the module shall produce the same charset name result.

Traceability:
- `localcharset.c`
- `locale_charset`

### Key Entities

#### `table_entry`
A mapping record used by the module’s built-in lookup tables.

Role:
- associates an input locale-related identifier or platform-specific token with an output charset name.

Relationships:
- `locale_charset` consults one or more collections of `table_entry` records to convert recognized locale/platform identifiers into the returned charset string.

Traceability:
- `localcharset.c`
- anonymous `struct table_entry` definitions and references listed in the module analysis

#### Locale charset result string
The resolved charset name returned to callers.

Role:
- represents the module’s final output after locale inspection and any applicable mapping.

Relationships:
- produced by `locale_charset`,
- may be derived directly from locale information or indirectly through `table_entry`-based translation.

Traceability:
- `localcharset.c`
- `locale_charset`

## Success Criteria

### SC-1: Correct locale-based resolution
For test locales whose expected charset result is known from the source module behavior, the Rust port returns the same charset name as the C module.

Traceability:
- `localcharset.c`
- `locale_charset`

### SC-2: Mapping-table parity
For representative inputs covered by the module’s `table_entry` mappings, the Rust port returns the same mapped charset names as the source module.

Traceability:
- `localcharset.c`
- `locale_charset`
- `table_entry`

### SC-3: Non-empty successful result
In supported locale scenarios handled by the source module, the Rust port returns a non-empty charset string.

Traceability:
- `localcharset.c`
- `locale_charset`

### SC-4: Stable repeated-call behavior
When invoked multiple times without locale changes, the Rust port returns identical charset string content on each call.

Traceability:
- `localcharset.c`
- `locale_charset`

### SC-5: Scope fidelity
The Rust implementation remains limited to charset-resolution behavior evidenced by this module and does not add unrelated externally visible functionality.

Traceability:
- `localcharset.c`
- `locale_charset`

## Out of Scope
The Rust rewrite specification does not require:
- additional locale-management APIs,
- mutation of process locale settings,
- thread-safety guarantees,
- serialization or persistence,
- performance benchmarking targets,
- public interfaces beyond those needed to preserve the module-equivalent behavior.