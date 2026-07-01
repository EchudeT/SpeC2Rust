# spec.md

## Title
Rust Port Functional Specification for `module_gnu_stdio-read.c_48`

## Metadata
- Project: `cflow-new`
- Module: `module_gnu_stdio-read.c_48`
- Category: `module_cluster`
- Source file: `gnu/stdio-read.c`
- Rust branch: `054-module_gnu_stdio_read.c_48-rust-port`
- Generation date: `2026-06-11`

## Overview
This module provides stdio input-reading entry points corresponding to formatted input and basic stream-reading operations. Its role is to expose callable interfaces for reading from standard input or a supplied `FILE` stream, including:

- formatted scanning from `stdin` and from an explicit stream
- variadic-list based formatted scanning
- single-character input
- line input into a caller-provided buffer
- block input into a caller-provided buffer

The Rust rewrite must preserve this module’s externally observable reading behavior and scope as evidenced by the source file’s exported functions. The specification covers only these read-oriented stdio capabilities and their relationship to `FILE`, `stdin`, caller-provided buffers, format strings, and `va_list`-style argument forwarding.

## Feature Specification

### Feature Summary
The Rust version must implement the module’s read-side stdio interface surface represented by the following source-backed operations:

- formatted input from standard input
- formatted input from a specific stream
- formatted input using an existing variadic argument list
- reading one character from standard input
- reading one character from a specific stream
- reading a line into a provided character buffer
- reading a block of elements into a provided memory buffer

### Supported Functional Behavior
1. **Formatted input from standard input**
   - Provide behavior equivalent in role to `scanf`.
   - Accept a format string and write parsed input values into caller-supplied destinations.
   - Read from standard input.

2. **Formatted input from a specific stream**
   - Provide behavior equivalent in role to `fscanf`.
   - Accept a `FILE` stream and a format string, and write parsed input values into caller-supplied destinations.
   - Read from the supplied stream.

3. **Formatted input with forwarded variadic arguments**
   - Provide behavior equivalent in role to `vscanf` and `vfscanf`.
   - Accept an existing variadic argument list and perform the same formatted parsing behavior as the corresponding non-`v` forms.
   - Read from standard input for `vscanf`, and from the supplied stream for `vfscanf`.

4. **Single-character input**
   - Provide behavior equivalent in role to `getchar` and `fgetc`.
   - Return the next character from standard input or a supplied stream, respectively.
   - Preserve end-of-input and read-failure signaling consistent with stdio-style character reads.

5. **Line input into caller buffer**
   - Provide behavior equivalent in role to `fgets`.
   - Read from the supplied stream into a caller-provided character buffer with a caller-provided size bound.
   - Preserve bounded writing behavior implied by the buffer length parameter.

6. **Block input into caller buffer**
   - Provide behavior equivalent in role to `fread`.
   - Read element data from the supplied stream into caller-provided memory.
   - Respect caller-provided element size and element count in determining the requested transfer.

### Out of Scope
The Rust port specification does not require capabilities not evidenced in this module, including:
- write-side stdio operations
- stream creation, opening, closing, or seeking APIs
- new public APIs beyond the read entry points represented here
- guarantees beyond the standard observable behavior of these read calls

## User Scenarios & Testing

### Scenario 1: Parse values from standard input
A caller needs to read formatted data from standard input using a format string and destination arguments.

**Expected support**
- The Rust module provides standard-input formatted scanning behavior corresponding to `scanf`.
- A caller using an existing variadic argument list can invoke equivalent behavior through the `v` form.

**Test focus**
- Reading formatted values from standard input populates caller destinations as directed by the format string.
- Return values indicate successful assignment count or input termination/failure according to stdio-style scanning behavior.

### Scenario 2: Parse values from a specific stream
A caller has a `FILE` stream and needs to extract structured data according to a format string.

**Expected support**
- The Rust module provides stream-based formatted scanning behavior corresponding to `fscanf`.
- The Rust module also supports the forwarded-argument-list form corresponding to `vfscanf`.

**Test focus**
- Input is read from the supplied stream rather than standard input.
- The formatted parse result and return value match the stream contents and requested format.

### Scenario 3: Read one character from input
A caller needs the next byte/character from input, either from standard input or a specific stream.

**Expected support**
- The Rust module provides `getchar`-equivalent behavior for standard input.
- The Rust module provides `fgetc`-equivalent behavior for a supplied stream.

**Test focus**
- Successive calls return successive input characters.
- End-of-file or read failure is reported using the stdio-style integer result convention.

### Scenario 4: Read one line into a fixed buffer
A caller needs to read text from a stream into a preallocated character buffer with a maximum capacity.

**Expected support**
- The Rust module provides `fgets`-equivalent behavior.

**Test focus**
- At most the provided bounded capacity is written in accordance with stdio line-read behavior.
- The returned pointer/null-style success indication matches whether input was obtained.
- Buffer contents reflect line-oriented input from the specified stream.

### Scenario 5: Read a block of binary or raw data
A caller needs to fill a memory buffer from a stream using element size and count parameters.

**Expected support**
- The Rust module provides `fread`-equivalent behavior.

**Test focus**
- The number of elements reported as read reflects the actual completed transfer.
- Data copied into the buffer matches the bytes available from the stream up to the requested amount.
- Partial reads at end of input are correctly reflected in the returned element count.

## Requirements

### Functional Requirements

#### FR-1: Standard-input formatted scanning
The module shall provide formatted input reading from standard input corresponding to `scanf`, using a format string and caller-supplied destinations.
**Traceability:** `gnu/stdio-read.c`, `scanf`

#### FR-2: Stream-based formatted scanning
The module shall provide formatted input reading from a supplied `FILE` stream corresponding to `fscanf`, using a format string and caller-supplied destinations.
**Traceability:** `gnu/stdio-read.c`, `fscanf`

#### FR-3: Variadic-list formatted scanning from standard input
The module shall provide formatted input reading from standard input using an existing variadic argument list corresponding to `vscanf`.
**Traceability:** `gnu/stdio-read.c`, `vscanf`

#### FR-4: Variadic-list formatted scanning from a supplied stream
The module shall provide formatted input reading from a supplied `FILE` stream using an existing variadic argument list corresponding to `vfscanf`.
**Traceability:** `gnu/stdio-read.c`, `vfscanf`

#### FR-5: Standard-input character read
The module shall provide single-character input from standard input corresponding to `getchar`.
**Traceability:** `gnu/stdio-read.c`, `getchar`

#### FR-6: Stream character read
The module shall provide single-character input from a supplied `FILE` stream corresponding to `fgetc`.
**Traceability:** `gnu/stdio-read.c`, `fgetc`

#### FR-7: Bounded line read into caller buffer
The module shall provide line-oriented input from a supplied `FILE` stream into a caller-provided character buffer with caller-provided length bound, corresponding to `fgets`.
**Traceability:** `gnu/stdio-read.c`, `fgets`

#### FR-8: Block read into caller buffer
The module shall provide block input from a supplied `FILE` stream into caller-provided memory based on requested element size and element count, corresponding to `fread`.
**Traceability:** `gnu/stdio-read.c`, `fread`

#### FR-9: Use the correct input source for each entry point
The module shall read from standard input only for standard-input forms, and from the caller-supplied `FILE` stream only for stream-based forms.
**Traceability:** `gnu/stdio-read.c`, `scanf`, `vscanf`, `getchar`, `fscanf`, `vfscanf`, `fgetc`, `fgets`, `fread`

#### FR-10: Preserve stdio-style result signaling
The module shall preserve the observable result conventions associated with each operation class:
- scanning functions return an integer scan result
- character-reading functions return an integer character/error-or-EOF result
- line-reading returns success/failure via pointer-style result
- block-reading returns the number of whole elements read
**Traceability:** `gnu/stdio-read.c`, function signatures for `scanf`, `fscanf`, `vscanf`, `vfscanf`, `getchar`, `fgetc`, `fgets`, `fread`

### Key Entities

#### `FILE`
Represents an input stream used by stream-based operations. It is the required stream handle for `fscanf`, `vfscanf`, `fgetc`, `fgets`, and `fread`.

**Relationships**
- Consumed by stream-reading functions as the source of input.
- Distinguished from standard input used implicitly by `scanf`, `vscanf`, and `getchar`.

#### Standard input
Represents the implicit input source for standard-input entry points.

**Relationships**
- Used by `scanf`, `vscanf`, and `getchar`.
- Not passed explicitly by the caller.

#### Format string
Represents the caller-supplied description of how formatted scanning should parse incoming input.

**Relationships**
- Consumed by `scanf`, `fscanf`, `vscanf`, and `vfscanf`.
- Directs how input is interpreted and where parsed values are assigned.

#### Variadic argument list
Represents a forwarded collection of argument destinations for formatted scanning.

**Relationships**
- Consumed by `vscanf` and `vfscanf`.
- Serves the same role as direct variadic arguments in the non-`v` formatted scanning functions.

#### Caller-provided character buffer
Represents writable storage supplied by the caller for line input.

**Relationships**
- Used by `fgets`.
- Constrained by the caller-provided length parameter.

#### Caller-provided memory buffer
Represents writable storage supplied by the caller for block input.

**Relationships**
- Used by `fread`.
- Filled according to requested element size and element count.

## Success Criteria

### SC-1: API coverage
The Rust port exposes behaviorally corresponding implementations for all source-backed operations in this module: standard-input scanning, stream scanning, `v`-form scanning, character reads, bounded line read, and block read.
**Traceability:** `gnu/stdio-read.c`, all listed functions

### SC-2: Correct input-source selection
Tests demonstrate that standard-input forms consume input from standard input, while stream-based forms consume input from the explicitly supplied `FILE` stream.
**Traceability:** `gnu/stdio-read.c`, `scanf`, `vscanf`, `getchar`, `fscanf`, `vfscanf`, `fgetc`, `fgets`, `fread`

### SC-3: Formatted scan result compatibility
For representative successful, partial, and terminated-input cases, formatted scanning operations return stdio-compatible scan result values and assign outputs according to the format string.
**Traceability:** `gnu/stdio-read.c`, `scanf`, `fscanf`, `vscanf`, `vfscanf`

### SC-4: Character-read result compatibility
For representative stream contents and end-of-input cases, character-reading operations return the next available character when present and stdio-compatible non-character result signaling at end of input or read failure.
**Traceability:** `gnu/stdio-read.c`, `getchar`, `fgetc`

### SC-5: Bounded line-read behavior
Tests demonstrate that the line-reading operation writes into the caller buffer subject to the provided size bound and reports success/failure using the expected pointer-style convention.
**Traceability:** `gnu/stdio-read.c`, `fgets`

### SC-6: Block-read count behavior
Tests demonstrate that block reads return the number of whole elements successfully read and store the corresponding bytes into the caller-provided buffer, including partial completion at end of input.
**Traceability:** `gnu/stdio-read.c`, `fread`

### SC-7: No unsupported functional expansion
The Rust module implementation remains limited to the read-oriented stdio functionality evidenced by this source module and does not require unrelated write-side or stream-management features to satisfy this specification.
**Traceability:** `gnu/stdio-read.c`, module function set