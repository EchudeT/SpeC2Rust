# spec.md

## Overview

- **Project**: `cflow-new`
- **Module**: `module_gnu_snprintf.c_45`
- **Category**: `module_cluster`
- **Source basis**: `gnu/snprintf.c`
- **Primary entry point**: `snprintf`

## Feature Specification

### Purpose

This module provides a `snprintf`-compatible formatted output function that writes formatted text into a caller-provided character buffer with an explicit size limit.

### In-scope behavior

The Rust rewrite must implement the behavior represented by the module’s single public function:

- Accept a destination character buffer, a maximum writable size, a format string, and variadic formatting arguments.
- Produce formatted output according to the supplied format and arguments.
- Write the formatted result into the destination buffer subject to the provided size bound.
- Return an integer result consistent with `snprintf` behavior as exposed by this module.

### Functional boundary

The module’s responsibility is limited to bounded formatting into a caller-owned string buffer through the `snprintf` interface. No additional public formatting APIs, buffer management facilities, or unrelated string utilities are in scope.

## User Scenarios & Testing

### Scenario 1: Format text into a sufficiently large buffer

A caller provides:

- a valid writable character buffer,
- a buffer size large enough for the formatted output,
- a valid format string,
- matching formatting arguments.

Expected module behavior:

- The formatted text is written into the buffer.
- The written content matches the requested formatting.
- The return value reports the formatting result for the produced output.

### Scenario 2: Format text into a size-limited buffer

A caller provides:

- a valid writable character buffer,
- a positive size that is smaller than the full formatted output,
- a valid format string and arguments.

Expected module behavior:

- Output writing is constrained by the provided size.
- The function still computes and returns its `snprintf` result according to the module contract.
- The call does not write beyond the caller-supplied size limit.

### Scenario 3: Request formatted length with no writable capacity

A caller provides a destination pointer and a size of zero.

Expected module behavior:

- The operation respects the zero-size limit.
- The function returns the `snprintf` result for the formatting request under this interface.

### Scenario 4: Use the function as a drop-in bounded formatting interface

A caller uses this module anywhere a standard `snprintf`-style function is expected.

Expected module behavior:

- The call shape and observable behavior remain compatible with the module’s documented `snprintf` role.
- Existing call sites that rely on bounded formatted output semantics can be migrated to the Rust version without requiring new public APIs.

### Testing focus

The Rust version must be testable for:

- correct formatted output when capacity is sufficient,
- bounded behavior when capacity is insufficient,
- correct handling of zero-size calls,
- correct integer return behavior for each of the above cases.

## Requirements

### Functional Requirements

#### FR-1: Bounded formatted output
The module shall provide a function equivalent in role to `snprintf` that formats input according to a format string and writes into a caller-provided character buffer with an explicit size bound.

**Traceability**: `gnu/snprintf.c`, `snprintf`

#### FR-2: Caller-supplied destination
The module shall operate on a destination buffer supplied by the caller rather than allocating or owning output storage itself.

**Traceability**: `gnu/snprintf.c`, `snprintf`

#### FR-3: Size-limited writes
The module shall ensure that output writing is governed by the `size` argument and does not exceed the specified bound.

**Traceability**: `gnu/snprintf.c`, `snprintf`

#### FR-4: Format-string-driven output
The module shall accept a format string and corresponding variadic arguments and use them to determine the produced textual output.

**Traceability**: `gnu/snprintf.c`, `snprintf`

#### FR-5: Integer result reporting
The module shall return an `int` result from the formatting operation, matching the observable contract of this module’s `snprintf` interface.

**Traceability**: `gnu/snprintf.c`, `snprintf`

### Key Entities

#### 1. Destination buffer
A caller-owned writable character array referenced by the `str` parameter. This is the output target for formatted text.

#### 2. Size bound
A `size_t` value provided by the caller through the `size` parameter. It constrains how much output may be written to the destination buffer.

#### 3. Format specification
A caller-provided format string referenced by the `format` parameter. It defines how variadic arguments are converted into output text.

#### 4. Variadic formatting arguments
The argument list supplied after `format`. These values are consumed according to the format specification to produce the final text.

#### Relationship of entities
The format specification and variadic arguments determine the textual result. The destination buffer receives that result, while the size bound limits how much of it may be written.

## Success Criteria

### SC-1: Functional interface parity
The Rust module exposes the same functional boundary as this module: bounded formatted output through an `snprintf`-equivalent interface.

**Traceability**: `gnu/snprintf.c`, `snprintf`

### SC-2: Correct output for non-truncating cases
For inputs where the formatted text fits within the provided size, tests demonstrate that the destination buffer contains the expected formatted output.

**Traceability**: `gnu/snprintf.c`, `snprintf`

### SC-3: Correct bounded behavior for truncating cases
For inputs where the formatted text exceeds the provided size, tests demonstrate that writes remain within the supplied bound and that observable `snprintf` behavior is preserved.

**Traceability**: `gnu/snprintf.c`, `snprintf`

### SC-4: Correct zero-size behavior
Tests demonstrate that calls with `size == 0` respect the zero-length write limit and still produce the expected integer result for this interface.

**Traceability**: `gnu/snprintf.c`, `snprintf`

### SC-5: Return-value conformance
Tests demonstrate that the Rust rewrite returns an `int`-equivalent result matching the module’s `snprintf` behavior across fitting, truncating, and zero-size scenarios.

**Traceability**: `gnu/snprintf.c`, `snprintf`