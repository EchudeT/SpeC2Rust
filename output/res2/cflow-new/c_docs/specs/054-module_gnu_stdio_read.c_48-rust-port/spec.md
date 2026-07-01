# spec.md

## Title

Functional Specification: Rust Port of `gnu/stdio-read.c` for `cflow-new`

## Metadata

- **Project**: `cflow-new`
- **Module**: `module_gnu_stdio-read.c_48`
- **Category**: `module_cluster`
- **Source file**: `gnu/stdio-read.c`
- **Rust branch**: `054-module_gnu_stdio_read.c_48-rust-port`
- **Generation date**: `2026-06-17`

## Overview

This module provides read-oriented standard I/O entry points corresponding to common C stdio input operations. Its role is to expose functional interfaces for:

- formatted input from standard input and from a specified stream,
- variadic and `va_list`-based formatted input forms,
- single-character input from standard input and from a specified stream,
- line-oriented string input from a specified stream,
- block/buffered byte input from a specified stream.

The Rust rewrite must preserve the observable behavior boundary of this module as a stdio-read interface layer. The specification is limited to behavior evidenced by the available source functions and does not assume additional capabilities beyond those entry points.

## Scope

### In Scope

The Rust version must implement the functional behavior represented by these source entry points:

- formatted input wrappers for standard input and arbitrary streams,
- `va_list`-style formatted input equivalent behavior where applicable in Rust design,
- character input operations,
- line input into caller-provided storage,
- block input into caller-provided storage.

### Out of Scope

The following are not required unless directly needed to preserve the evidenced behavior of the listed functions:

- write-oriented stdio functionality,
- file opening/closing lifecycle APIs,
- buffering policy configuration,
- thread-safety guarantees,
- serialization or persistence features,
- error recovery extensions beyond the underlying input operation result,
- new public APIs not required to represent the source module’s behavior.

## Feature Specification

### Feature Summary

The module acts as a read-function façade for standard C-style stdio input behavior. It must support callers that read data from either the implicit standard input stream or an explicit input stream, using one of four styles:

1. **Formatted input**
   - Read and parse input according to a format string.
   - Support both standard-input and stream-specific forms.
   - Support direct variadic-call style and argument-list style behavior.

2. **Single character input**
   - Read one character from standard input or a specific stream.
   - Return the read character when available.
   - Signal end-of-input or read failure using the same outcome model expected by the source interface.

3. **Line input**
   - Read up to a caller-specified limit into caller-provided character storage from a stream.
   - Preserve the source function’s success/failure boundary: return the destination on success, otherwise indicate no line was produced.

4. **Block input**
   - Read byte data into caller-provided storage from a stream in element-count form.
   - Report how many elements were read.

### Required Rust Functional Behavior

The Rust port must provide module behavior equivalent to the source module for the following function families:

- `scanf` / `vscanf`: formatted input from standard input.
- `fscanf` / `vfscanf`: formatted input from an explicitly supplied stream.
- `getchar` / `fgetc`: single-character input from standard input or stream.
- `fgets`: bounded line input from stream into caller-provided buffer semantics.
- `fread`: block input from stream into caller-provided buffer semantics.

Where Rust cannot natively express C variadic interfaces, the implementation must still preserve the functional distinction between:

- using standard input vs a specified stream, and
- formatted input driven by a direct argument set vs a preassembled argument-list equivalent internal path.

This requirement is about behavior, not identical Rust surface syntax.

## User Scenarios & Testing

### Scenario 1: Read formatted values from standard input

A caller needs to parse structured text from standard input using a format string and receive a count of successfully matched and assigned inputs.

**Expected support**
- The Rust port accepts standard-input formatted reads.
- It returns an integer result consistent with successful assignment count or input failure/end-of-input signaling.

**Testing focus**
- Input containing values matching the format.
- Input that stops matching after some assignments.
- Empty or exhausted standard input.

### Scenario 2: Read formatted values from a specific stream

A caller has an existing input stream and needs to parse formatted content from that stream rather than standard input.

**Expected support**
- The Rust port accepts an explicit stream/source handle.
- Behavior is equivalent to the standard-input form except that the source is caller-selected.

**Testing focus**
- Parsing from a non-stdin stream with valid formatted content.
- Partial match cases.
- End-of-stream before any successful assignment.

### Scenario 3: Use argument-list-based formatted input path

A caller or internal adapter needs the formatted-input logic to operate through an argument-list-driven path rather than a direct variadic call.

**Expected support**
- The Rust port preserves a distinct internal or exposed pathway corresponding to `vscanf`/`vfscanf` behavior.
- Results match the equivalent formatted read using the same source and format.

**Testing focus**
- Compare direct formatted-input results with argument-list-path results for the same input.
- Verify same success count and failure/end-of-input outcomes.

### Scenario 4: Read a single character from standard input

A caller wants exactly one character from standard input and needs a distinct result for success versus end-of-input/failure.

**Expected support**
- The Rust port returns one character value when available.
- It signals no character available using the same outcome class as the source behavior.

**Testing focus**
- Standard input containing at least one byte/character.
- Exhausted input.
- Repeated reads consuming successive characters.

### Scenario 5: Read a single character from a specified stream

A caller wants one character from a stream other than standard input.

**Expected support**
- The Rust port performs single-character reading from the supplied stream.
- Success and no-data/failure outcomes follow the source behavior boundary.

**Testing focus**
- Character reads from a prepared stream.
- Sequential reads until end-of-stream.

### Scenario 6: Read a bounded line into caller-provided storage

A caller needs to read text from a stream into an existing buffer with a maximum length.

**Expected support**
- The Rust port reads at most the caller-specified bound according to `fgets`-style semantics.
- On success, the destination buffer content represents the line fragment read.
- On failure with no data produced, the operation signals failure rather than fabricated content.

**Testing focus**
- A line shorter than the bound.
- A line exactly filling the permitted read window.
- A line longer than the bound, requiring truncation to the operation’s bound.
- End-of-stream before any characters are read.

### Scenario 7: Read binary or raw data blocks into caller-provided storage

A caller needs to fill a buffer from a stream and know how many complete elements were read.

**Expected support**
- The Rust port reads into caller-provided storage using element size and element count semantics matching `fread`.
- The return value reflects completed elements read, including partial completion due to end-of-stream.

**Testing focus**
- Full read where enough data is available.
- Short read due to end-of-stream.
- Zero elements requested.
- Different element sizes and counts.

## Requirements

### Functional Requirements

#### FR-1: Formatted input from standard input
The module shall support formatted input operations from standard input corresponding to `scanf` and `vscanf`, producing an integer result representing the formatted-read outcome evidenced by those functions in `gnu/stdio-read.c`.

**Traceability**: `scanf`, `vscanf`

#### FR-2: Formatted input from an explicit stream
The module shall support formatted input operations from a caller-specified input stream corresponding to `fscanf` and `vfscanf`, producing an integer result representing the formatted-read outcome evidenced by those functions in `gnu/stdio-read.c`.

**Traceability**: `fscanf`, `vfscanf`

#### FR-3: Standard-input and explicit-stream formatted paths must remain behaviorally aligned
For equivalent input content, format string, and destination arguments, the standard-input and explicit-stream formatted input operations shall differ only by input source selection, not by parsing intent or result model.

**Traceability**: `scanf`, `fscanf`, `vscanf`, `vfscanf`

#### FR-4: Single-character input from standard input
The module shall support reading one character from standard input corresponding to `getchar`, returning either the next character value or an end-of-input/failure indication consistent with the source function boundary.

**Traceability**: `getchar`

#### FR-5: Single-character input from an explicit stream
The module shall support reading one character from a caller-specified stream corresponding to `fgetc`, returning either the next character value or an end-of-input/failure indication consistent with the source function boundary.

**Traceability**: `fgetc`

#### FR-6: Bounded line input into caller-provided character storage
The module shall support reading text from a caller-specified stream into caller-provided storage with an explicit maximum length, corresponding to `fgets`. It shall indicate success by returning the destination-equivalent result and indicate failure when no line data is produced.

**Traceability**: `fgets`

#### FR-7: Block input into caller-provided storage
The module shall support reading from a caller-specified stream into caller-provided storage using element size and element count semantics corresponding to `fread`, and shall return the number of elements read.

**Traceability**: `fread`

#### FR-8: Caller-supplied destination storage is required for line and block reads
For operations corresponding to `fgets` and `fread`, the module shall write into storage supplied by the caller rather than allocating an unrelated replacement result as the primary data destination.

**Traceability**: `fgets`, `fread`

#### FR-9: Input source distinction must be preserved
The module shall preserve the distinction between operations that implicitly use standard input and operations that consume from an explicit stream/source handle.

**Traceability**: `scanf`, `fscanf`, `vscanf`, `vfscanf`, `getchar`, `fgetc`, `fgets`, `fread`

### Key Entities

Because no module-specific structs are identified in the source analysis, the key entities are functional interface entities evidenced by the function signatures.

#### Entity 1: Input stream
A stream represents the source of readable data. Some operations receive it explicitly, while others implicitly use standard input.

**Relationships**
- Used directly by stream-specific formatted input.
- Used directly by `fgetc`, `fgets`, and `fread`.
- Standard input is the implicit stream for `scanf`, `vscanf`, and `getchar`.

**Traceability**: `fscanf`, `vfscanf`, `fgetc`, `fgets`, `fread`, `scanf`, `vscanf`, `getchar`

#### Entity 2: Format string
A format string describes how formatted input should be interpreted and assigned.

**Relationships**
- Consumed by standard-input formatted input functions.
- Consumed by explicit-stream formatted input functions.
- Shared behavioral concept across direct and argument-list-driven formatted paths.

**Traceability**: `scanf`, `fscanf`, `vscanf`, `vfscanf`

#### Entity 3: Argument set for formatted assignment
Formatted input writes parsed results into caller-designated destinations supplied either through a direct variadic call pattern or an argument-list pattern.

**Relationships**
- Paired with a format string.
- Used by both standard-input and explicit-stream formatted functions.

**Traceability**: `scanf`, `fscanf`, `vscanf`, `vfscanf`

#### Entity 4: Caller-provided character buffer
A writable character buffer receives line-oriented input for the `fgets`-equivalent operation.

**Relationships**
- Filled from a specified input stream.
- Constrained by a caller-provided maximum length.

**Traceability**: `fgets`

#### Entity 5: Caller-provided raw byte buffer
A writable memory region receives block input for the `fread`-equivalent operation.

**Relationships**
- Filled from a specified input stream.
- Interpreted according to caller-provided element size and count.

**Traceability**: `fread`

## Success Criteria

### SC-1: Formatted standard-input operation coverage
The Rust module provides a working equivalent for standard-input formatted reading and returns integer outcomes for successful parsing, partial matching, and input exhaustion/failure cases.

**Measured by**
- Tests covering successful match, partial match, and no-input cases for the `scanf`/`vscanf` behavior family.

**Traceability**: `scanf`, `vscanf`

### SC-2: Formatted explicit-stream operation coverage
The Rust module provides a working equivalent for stream-based formatted reading and returns integer outcomes for successful parsing, partial matching, and input exhaustion/failure cases.

**Measured by**
- Tests covering successful match, partial match, and no-input cases for the `fscanf`/`vfscanf` behavior family.

**Traceability**: `fscanf`, `vfscanf`

### SC-3: Behavioral parity between direct and argument-list formatted paths
For the same source content and format, the direct formatted-input path and the argument-list-based path yield the same observable result classification and assignment count.

**Measured by**
- Paired tests comparing `scanf` vs `vscanf`-equivalent behavior.
- Paired tests comparing `fscanf` vs `vfscanf`-equivalent behavior.

**Traceability**: `scanf`, `fscanf`, `vscanf`, `vfscanf`

### SC-4: Character input correctness
The Rust module correctly reads successive single characters from standard input and explicit streams, and signals end-of-input/failure when no further character is available.

**Measured by**
- Tests reading known character sequences through `getchar`-equivalent and `fgetc`-equivalent operations until exhaustion.

**Traceability**: `getchar`, `fgetc`

### SC-5: Line input correctness
The Rust module correctly performs bounded line reads into caller-provided storage, succeeds when line data is available, respects the provided maximum length, and signals failure when no data is read.

**Measured by**
- Tests for short lines, exact-bound reads, over-bound lines, and end-of-stream-without-data cases.

**Traceability**: `fgets`

### SC-6: Block input correctness
The Rust module correctly reads raw data into caller-provided storage using element size/count semantics and reports the number of complete elements read.

**Measured by**
- Tests for full reads, short reads, and zero-count requests.

**Traceability**: `fread`

### SC-7: Source-selection correctness
Operations that are standard-input-based and operations that are explicit-stream-based read from the intended source and do not conflate those source modes.

**Measured by**
- Tests using distinct data in standard input and an explicit stream, verifying each function family consumes from the proper source.

**Traceability**: `scanf`, `fscanf`, `vscanf`, `vfscanf`, `getchar`, `fgetc`