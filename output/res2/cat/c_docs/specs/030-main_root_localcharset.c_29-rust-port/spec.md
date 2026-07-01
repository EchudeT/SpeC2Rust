# spec.md

## Title

Functional Specification for `main_root_localcharset.c_29` Rust Port

## Document Information

- **Project**: `cat`
- **Module**: `main_root_localcharset.c_29`
- **Category**: `main_cluster`
- **Source file**: `localcharset.c`
- **Primary function**: `locale_charset(void) -> const char *`
- **Rust branch**: `030-main_root_localcharset.c_29-rust-port`
- **Generation date**: `2026-06-07`

## Overview

This module provides locale character set detection for the program at runtime. Its functional role is to determine the character encoding associated with the current locale environment and return that encoding as a charset name string.

The Rust rewrite must preserve this behavior: given the process locale configuration visible at call time, it must resolve and return an appropriate charset name for the active locale, using the same functional decision space represented by `localcharset.c`, including table-driven charset name normalization or lookup where applicable.

This module is a query module. It exposes behavior centered on a single operation: obtaining the locale charset name. No additional public capabilities are required by this specification.

## Feature Specification

### Feature: Determine locale charset name

The module shall provide functionality equivalent to `locale_charset(void)`.

Behaviorally, this feature must:

- inspect the active locale-related runtime configuration as needed to determine the locale's encoding;
- derive a charset name representing that encoding;
- use built-in mapping data where the source module uses table-driven mappings to translate locale-specific identifiers into returned charset names;
- return a stable string result representing the detected charset for the current call;
- produce a charset result suitable for downstream program logic that needs to know the locale encoding.

The Rust version must implement the same functional boundary: locale charset resolution and mapping, not broader locale management.

### Feature boundaries

Included:

- locale encoding determination;
- charset name lookup and normalization implied by mapping tables in `localcharset.c`;
- returning the resolved charset name to callers.

Excluded because not evidenced in the module input:

- setting or modifying process locale;
- converting text between encodings;
- exposing additional public locale-inspection APIs;
- persistence, serialization, remote lookup, or configuration management;
- guarantees beyond the observable return behavior of `locale_charset`.

## User Scenarios & Testing

### Scenario 1: Caller requests the charset for the current locale

A caller invokes the module's locale charset function while the process is running under a configured locale.

Expected result:

- the module returns a non-empty charset name string corresponding to the current locale encoding;
- the result is derived from the locale information visible to the process at the time of the call.

### Scenario 2: Locale identifier requires table-based mapping

A caller runs the program in an environment where the locale naming form is not itself the final charset name and requires lookup through one of the module's internal mapping tables.

Expected result:

- the module resolves the locale-related identifier through the applicable mapping table;
- the returned string is the mapped charset name represented by the source module's data.

### Scenario 3: Different locale environments produce different charset results when appropriate

The function is called in two executions or test setups with different locale configurations.

Expected result:

- when the source module's behavior distinguishes these locale configurations, the Rust version returns correspondingly different charset names;
- when the source module normalizes multiple locale representations to the same charset, the Rust version returns the same normalized charset name.

### Scenario 4: Caller uses result as a read-only charset identifier

Program logic obtains the charset string and uses it for conditional behavior, comparison, or reporting.

Expected result:

- the returned value is directly usable as a charset identifier;
- it matches the naming behavior of the source module closely enough for code depending on known charset names to continue working.

### Testing expectations

The Rust port must be testable with scenario-based validation that covers:

- successful charset resolution for a locale with explicit encoding;
- successful resolution through table-based mapping entries represented by `table_entry` data;
- normalization behavior where multiple locale forms map to one charset result;
- consistency with the source module for representative locale inputs supported by `localcharset.c`.

## Requirements

### Functional Requirements

#### FR-1: Locale charset query
The module shall provide a functionally equivalent locale charset query operation corresponding to `locale_charset` in `localcharset.c`.

**Traceability**: `localcharset.c`, `locale_charset` [830-1159]

#### FR-2: Runtime locale-based resolution
The locale charset query shall determine its result from the active locale context available at runtime rather than from a fixed compile-time constant.

**Traceability**: `localcharset.c`, `locale_charset` [830-1159]

#### FR-3: Table-driven mapping support
The module shall support charset resolution using built-in mapping entries where locale identifiers or intermediate names require translation to returned charset names.

**Traceability**: `localcharset.c`, `locale_charset` [830-1159]; `struct table_entry` occurrences [86-90, 93, 577-581, 584, 1030, 1032, 1104, 1106]

#### FR-4: Charset name return value
The query operation shall return a charset name string as its observable output.

**Traceability**: `localcharset.c`, `locale_charset` signature and body [830-1159]

#### FR-5: Mapping-consistent normalization
Where the source module's mapping data causes distinct locale-related inputs to resolve to the same charset name, the Rust version shall preserve that normalization behavior.

**Traceability**: `localcharset.c`, `locale_charset` [830-1159]; `struct table_entry` mapping tables

#### FR-6: Source-compatible decision scope
The Rust version shall implement only the locale charset detection and mapping behavior evidenced by this module and shall not require callers to use additional module-specific public operations.

**Traceability**: `localcharset.c`, `locale_charset` [830-1159]

### Key Entities

#### Entity: Locale charset query result
A charset name string returned by the module to identify the encoding associated with the current locale.

Relationship:

- produced by the locale charset query operation;
- may be derived directly from locale information or via mapping-table translation.

**Traceability**: `locale_charset` [830-1159]

#### Entity: `table_entry`
A mapping record type used by the module's internal tables for lookup/translation related to locale charset resolution.

Relationship:

- groups source identifiers with corresponding charset-oriented outputs;
- consumed by the locale charset query logic to convert locale/platform-specific names into returned charset names.

**Traceability**: anonymous `struct table_entry` definitions/references in `localcharset.c` [86-90, 93, 577-581, 584, 1030, 1032, 1104, 1106]

#### Entity: Mapping tables
Collections of `table_entry` records embedded in the module.

Relationship:

- provide the known translation set used by locale charset resolution;
- are consulted by `locale_charset` when direct locale-derived naming is insufficient or requires normalization.

**Traceability**: `localcharset.c`; `struct table_entry` occurrences and `locale_charset` [830-1159]

## Success Criteria

### SC-1: Functional parity for primary operation
For representative locale configurations covered by the source module, the Rust implementation returns the same charset name as the C module's `locale_charset`.

**Traceability**: `localcharset.c`, `locale_charset` [830-1159]

### SC-2: Mapping-table behavior preserved
Tests exercising locale cases that depend on internal `table_entry` mappings shall produce the same mapped charset names in Rust as in the source module.

**Traceability**: `struct table_entry` occurrences; `locale_charset` [830-1159]

### SC-3: Normalization behavior preserved
Where multiple source locale forms are normalized by the C module to one charset name, the Rust port shall return that same normalized charset name for the same inputs.

**Traceability**: `locale_charset` [830-1159]; mapping tables in `localcharset.c`

### SC-4: Direct usability of returned identifier
The Rust module's returned value shall be a usable charset name string for caller-side comparison and selection logic, matching the source module's observable output form.

**Traceability**: `locale_charset` [830-1159]

### SC-5: No unsupported feature expansion
The Rust rewrite shall remain within the functional scope of locale charset detection and mapping and shall not introduce required new public functionality beyond the evidenced query behavior.

**Traceability**: module file/function scope in `localcharset.c`

## Out of Scope

The following are not required by this specification because they are not evidenced by the analyzed module:

- text transcoding;
- locale mutation APIs;
- user configuration interfaces;
- persistence or caching guarantees beyond observable function behavior;
- additional public APIs unrelated to `locale_charset`.