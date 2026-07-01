# spec.md

## Title
Rust Port Functional Specification: `main_root_localcharset.c_20`

## Metadata
- **Project**: `pwd`
- **Module**: `main_root_localcharset.c_20`
- **Category**: `main_cluster`
- **Source file**: `localcharset.c`
- **Primary function**: `locale_charset(void) -> const char *`
- **Rust branch**: `020-main_root_localcharset.c_20-rust-port`
- **Generation date**: `2026-06-09`

## Overview
This module determines the character encoding name associated with the process locale and returns that encoding as a string.

The Rust rewrite must preserve the module’s observable role as a locale-charset resolver. Its scope is limited to identifying and returning the locale encoding name used by the surrounding program logic. The specification is based on the exported behavior evidenced by `locale_charset` and the internal table-driven mapping structures present in `localcharset.c`.

## Feature Specification

### Feature: Locale charset resolution
The module shall provide functionality equivalent to `locale_charset(void)` that resolves the current locale’s character encoding and returns it as a charset name string.

The Rust version must implement the following behavior boundaries evidenced by the source module:

- Determine the charset name associated with the current locale context.
- Return a string result representing the resolved encoding name.
- Use recognized mappings where locale-related identifiers or platform-specific encoding indicators require normalization into a charset name.
- Produce a stable string result usable by callers as a read-only charset identifier.
- Operate without requiring caller-provided input parameters.

### In-scope behavior
The Rust version shall cover the functional role of the source module only:

- locale-dependent charset identification
- mapping-based charset name normalization
- returning the resolved charset name for use by other program components

### Out-of-scope behavior
The Rust version shall not claim or introduce behavior not evidenced by this module:

- no new public APIs beyond the equivalent module functionality
- no configuration interfaces for overriding charset selection
- no promises about thread-safety, caching semantics, or reentrancy unless directly required to preserve observable behavior
- no file format, serialization, network, or recovery features

## User Scenarios & Testing

### Scenario 1: Caller needs the current locale encoding
A caller in the program needs to know which character encoding corresponds to the current locale in order to interpret text correctly.

**Expected behavior**
- The caller invokes the Rust equivalent of `locale_charset`.
- The module returns a non-empty charset name string representing the current locale encoding.

**Testing approach**
- Execute under a locale with a well-known encoding.
- Verify that the returned string names that encoding or the module’s normalized equivalent.

### Scenario 2: Locale information requires normalization
A locale or platform encoding indicator does not directly match the charset name expected by the program and must be mapped through known alias rules.

**Expected behavior**
- The module resolves the locale encoding.
- If the source representation is an alias or platform-specific form, the module returns the mapped charset name.

**Testing approach**
- Use test inputs or environment setups corresponding to known alias cases represented by the module’s mapping tables.
- Verify that the returned charset string matches the expected normalized name.

### Scenario 3: Repeated charset queries in one process
Program logic calls the locale charset resolver more than once during execution.

**Expected behavior**
- Each call returns a valid charset name consistent with the locale state visible to the module at the time of the call.
- Returned values are suitable for repeated read-only use by callers.

**Testing approach**
- Call the function multiple times under unchanged locale conditions.
- Verify that results remain valid and consistent.

### Scenario 4: Caller uses result as informational module output
Another part of the program uses the returned charset name only as an identifier and does not modify it.

**Expected behavior**
- The module provides a result compatible with read-only string use.
- The result can be compared against expected charset names in program logic or tests.

**Testing approach**
- Compare the returned string to expected charset identifiers in integration tests.
- Confirm that the result can be consumed without additional transformation by callers expecting a charset name.

## Requirements

### Functional Requirements

#### FR-1: Provide locale charset lookup
The module shall provide functionality equivalent to `locale_charset(void)` that resolves the charset associated with the current locale and returns it as a string.

**Traceability**
- Function: `locale_charset` in `localcharset.c:830-1159`

#### FR-2: Return a charset name without caller input
The module shall expose this functionality without requiring any input parameters from the caller.

**Traceability**
- Function signature: `const char * locale_charset (void);`

#### FR-3: Support table-driven charset name mapping
The module shall support mapping from locale-related or encoding-related identifiers to returned charset names using internal table-based associations where required by the source behavior.

**Traceability**
- Types: anonymous `struct table_entry` definitions and references in `localcharset.c`
- Function: `locale_charset`

#### FR-4: Normalize recognized aliases to module-defined charset names
When the locale or platform encoding indicator matches a recognized mapped entry, the module shall return the corresponding normalized charset name defined by the module’s mapping behavior.

**Traceability**
- Types: `struct table_entry` instances referenced around the mapping regions in `localcharset.c`
- Function: `locale_charset`

#### FR-5: Return a caller-usable read-only string identifier
The module shall return the resolved charset as a string suitable for read-only use by callers.

**Traceability**
- Function signature return type: `const char *`
- Function: `locale_charset`

### Key Entities

#### Entity: Locale charset resolver
The module’s central functional entity is the locale charset resolver represented by `locale_charset`.

**Role**
- Produces the charset name for the current locale context.

**Relationships**
- Consults internal mapping data to translate recognized locale/encoding forms into returned charset names.

**Traceability**
- Function: `locale_charset`

#### Entity: Table entry mapping records
The module contains internal table entry structures used to associate one identifier with another for charset resolution and normalization.

**Role**
- Represent mapping pairs used during charset determination.

**Relationships**
- Consumed by the locale charset resolver to convert locale- or platform-derived values into the returned charset string.

**Traceability**
- Anonymous `struct table_entry` definitions/references in `localcharset.c`
- Referenced type name: `table_entry`

## Success Criteria

### SC-1: Functional equivalence of primary behavior
The Rust module exposes functionality equivalent to `locale_charset` and returns a charset name string for the current locale.

**Verification**
- Unit or integration tests call the Rust equivalent under a configured locale and assert that a charset name is returned.

**Traceability**
- Function: `locale_charset`

### SC-2: Correct handling of known mapped cases
For locale or encoding forms represented by the source module’s table-driven mappings, the Rust port returns the corresponding normalized charset names.

**Verification**
- Tests cover representative mapping entries evidenced by the module tables and confirm expected outputs.

**Traceability**
- Function: `locale_charset`
- Types: `struct table_entry` mapping data

### SC-3: Read-only string usability
The Rust port returns results in a form that callers can use as a charset identifier without additional decoding or transformation.

**Verification**
- Integration tests compare the returned value directly against expected charset names.

**Traceability**
- Function signature and behavior of `locale_charset`

### SC-4: Repeatable results under unchanged locale conditions
Under the same locale conditions, repeated calls produce consistent valid charset name results.

**Verification**
- Tests invoke the function multiple times in one process without changing locale inputs and confirm consistent returned values.

**Traceability**
- Function: `locale_charset`

## Notes for the Rust Rewrite
The rewrite should preserve the source module’s functional boundary: locale charset identification and normalization via internal mapping data. The specification does not require preserving C-level implementation structure, but it does require preserving the observable behavior of the module’s single evidenced public function and its use of mapping-based charset resolution.