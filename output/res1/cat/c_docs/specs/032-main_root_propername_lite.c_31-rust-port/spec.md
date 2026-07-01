# spec.md

## Title

Functional Specification: `main_root_propername-lite.c_31`

## Metadata

- Project: `cat`
- Module: `main_root_propername-lite.c_31`
- Category: `main_cluster`
- Source file: `propername-lite.c`
- Primary function: `proper_name_lite`
- Rust branch: `032-main_root_propername_lite.c_31-rust-port`
- Generation date: 2026-06-06

## Overview

This module provides a lightweight proper-name selection function. It accepts two caller-provided name strings:

- an ASCII-form name
- a UTF-8-form name

The module returns a string to be used as the displayed proper name. The Rust rewrite must preserve the same observable behavior and role: selecting the appropriate proper-name representation from the two provided inputs, without introducing additional responsibilities or public capabilities.

## Feature Specification

### Feature: Lightweight proper-name selection

The module exposes one functional responsibility: given an ASCII name and a UTF-8 name, return the proper-name string that should be used by callers.

The Rust version must implement equivalent behavior for this selection operation:

- accept two name inputs corresponding to the ASCII and UTF-8 representations
- produce one selected proper-name result
- preserve the module’s lightweight scope as a name-selection utility rather than a formatting, storage, or policy framework

### Functional boundary

The module’s boundary is limited to proper-name selection from the two supplied representations. The Rust rewrite must not expand this into unrelated features such as persistent storage, localization management, alternate encodings beyond the provided inputs, or additional public APIs not evidenced by the source module interface.

## User Scenarios & Testing

### Scenario 1: Caller provides both an ASCII and a UTF-8 proper name

A caller has two representations of the same proper name and asks the module for the proper display string.

Expected support in Rust:

- the function accepts both inputs
- the function returns one proper-name string for caller use
- repeated calls with the same inputs produce consistent results

### Scenario 2: Caller uses the module as a lightweight helper in program output paths

A higher-level command or message-construction path uses this module only to obtain a proper name string, then incorporates that result into output.

Expected support in Rust:

- the module can be used as a pure selection helper
- the returned result is suitable for direct use as a name string by the caller
- no extra setup or module-managed state is required

### Scenario 3: Caller distinguishes between fallback-safe and richer name representations

A caller provides an ASCII-safe version together with a UTF-8 version and relies on the module to choose the proper output representation according to the original module behavior.

Expected support in Rust:

- both representations are accepted as inputs to the same operation
- the selected result follows the source module’s behavior
- the module does not alter the caller contract by requiring only one representation or by returning a compound object instead of a single selected name string

### Testing guidance

The Rust rewrite should be tested with cases that verify:

- two provided name inputs are accepted
- one selected name result is produced
- the result is stable for identical input pairs
- integration-style use is possible where the returned name is directly consumed by caller output logic
- observable behavior matches the source module for representative ASCII/UTF-8 input pairs

## Requirements

### Functional Requirements

#### FR-1: Dual-input proper-name operation

The module shall provide an operation equivalent to `proper_name_lite` that accepts two caller-supplied name inputs representing an ASCII form and a UTF-8 form.

**Traceability:** `propername-lite.c`, `proper_name_lite`

#### FR-2: Single selected proper-name result

The module shall return a single proper-name string result chosen from the supplied inputs, matching the role of the source function.

**Traceability:** `propername-lite.c`, `proper_name_lite`

#### FR-3: Lightweight utility scope

The module shall remain limited to proper-name selection behavior and shall not require module-owned mutable runtime state, configuration setup, or additional processing responsibilities beyond this selection role.

**Traceability:** `propername-lite.c`, `proper_name_lite`

#### FR-4: Behavior preservation

For the same effective input values, the Rust rewrite shall preserve the observable selection behavior of the source module.

**Traceability:** `propername-lite.c`, `proper_name_lite`

### Key Entities

#### Entity: ASCII proper-name input

A caller-provided string representing a proper name in ASCII form.

**Relationship:** This is one of the two inputs to the module’s sole operation.

**Traceability:** `propername-lite.c`, `proper_name_lite(name_ascii, ...)`

#### Entity: UTF-8 proper-name input

A caller-provided string representing a proper name in UTF-8 form.

**Relationship:** This is the second input to the module’s sole operation.

**Traceability:** `propername-lite.c`, `proper_name_lite(..., name_utf8)`

#### Entity: Selected proper-name result

A returned string representing the chosen proper-name form for caller use.

**Relationship:** This result is derived from the two input name representations by the module’s selection logic.

**Traceability:** `propername-lite.c`, `proper_name_lite`

## Success Criteria

### SC-1: Interface parity

The Rust module provides one proper-name selection operation corresponding to the source module’s exposed function and accepts both ASCII-form and UTF-8-form name inputs.

**Traceability:** `propername-lite.c`, `proper_name_lite`

### SC-2: Output contract parity

For each invocation, the Rust module returns one selected proper-name string result usable by callers in place of the C module’s returned name.

**Traceability:** `propername-lite.c`, `proper_name_lite`

### SC-3: Observable behavior match

For a representative test set of input pairs drawn from the original module’s intended usage, the Rust rewrite produces the same selected result as the source module.

**Traceability:** `propername-lite.c`, `proper_name_lite`

### SC-4: Stateless helper usage

The Rust rewrite can be invoked directly by callers as a lightweight helper without requiring initialization, configuration, or persistent module-managed state.

**Traceability:** `propername-lite.c`, `proper_name_lite`

## Out of Scope

The Rust rewrite specification does not require or authorize adding:

- new public APIs beyond the source module’s evidenced functional surface
- storage or management of name catalogs
- localization frameworks or generalized character-encoding conversion services
- thread-safety guarantees not evidenced by the source module
- serialization, FFI layers, or recovery mechanisms not evidenced by the source module