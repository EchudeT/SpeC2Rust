# spec.md

## Title

Rust Functional Specification for `module_gnu_stdio-read.c_48`

## Document Metadata

- Project: `cflow-new`
- Module: `module_gnu_stdio-read.c_48`
- Category: `module_cluster`
- Source file: `gnu/stdio-read.c`
- Target Rust branch: `054-module_gnu_stdio_read.c_48-rust-port`
- Generation date: `2026-06-17`

## Overview

This module provides read-oriented stdio entry points corresponding to common C input functions. Its role is to expose the observable behavior of formatted and unformatted input operations for standard input and explicit `FILE` streams.

The Rust rewrite must implement the same functional surface evidenced by the source module:

- formatted input from standard input and from a specified stream
- variadic-list based formatted input from standard input and from a specified stream
- single-character input from standard input and from a specified stream
- line-oriented buffered input from a specified stream
- block-oriented binary/text byte input from a specified stream

The module does not define its own persistent data structures. Its behavior is centered on operating on C stdio concepts already provided by the surrounding runtime, especially `FILE`, character buffers, raw memory buffers, format strings, and `va_list`.

## Scope

### In Scope

The Rust version must cover the behavior represented by these public entry points in `gnu/stdio-read.c`:

- `scanf`
- `fscanf`
- `vscanf`
- `vfscanf`
- `getchar`
- `fgetc`
- `fgets`
- `fread`

### Out of Scope

No additional capabilities are evidenced and therefore must not be introduced as part of this module specification, including:

- new public input APIs
- write/output behavior
- stream creation, closing, seeking, or buffering policy management
- custom file abstractions beyond the `FILE`-based behavior already implied by the source interface
- thread-safety guarantees
- serialization or persistence features
- recovery, retry, or fault-tolerance mechanisms beyond normal stdio return behavior

## Feature Specification

### Feature 1: Formatted input from standard input

The module provides formatted input using a format string against the standard input stream.

This behavior is represented by:

- `scanf(const char *format, ...)`
- `vscanf(const char *format, va_list args)`

The Rust version must support invoking formatted parsing against standard input, using either direct variadic-style arguments or an already prepared variable-argument list representation where applicable to the Rust design.

Observable behavior that must be preserved:

- the format string determines how input is interpreted
- data is consumed from standard input
- the function returns an integer result in the same role as the C interface, indicating the outcome of the input/conversion operation

### Feature 2: Formatted input from a specified stream

The module provides formatted input using a format string against an explicit `FILE` stream.

This behavior is represented by:

- `fscanf(FILE *stream, const char *format, ...)`
- `vfscanf(FILE *stream, const char *format, va_list args)`

The Rust version must support the same formatted parsing behavior when the caller specifies the source stream explicitly.

Observable behavior that must be preserved:

- the provided stream is the source of consumed input
- the format string drives parsing and conversion
- the function returns an integer result corresponding to formatted input outcome

### Feature 3: Single-character input

The module provides byte/character-oriented single-item input from either standard input or a specified stream.

This behavior is represented by:

- `getchar(void)`
- `fgetc(FILE *stream)`

The Rust version must support reading the next input unit from the relevant stream and returning an integer result in the same role as the C interface.

Observable behavior that must be preserved:

- `getchar` reads from standard input
- `fgetc` reads from the supplied stream
- the return value distinguishes successful character retrieval from end-of-input or error according to stdio semantics

### Feature 4: Line-buffered string input

The module provides bounded line-oriented input into a caller-supplied character buffer.

This behavior is represented by:

- `fgets(char *s, int n, FILE *stream)`

The Rust version must support reading from a supplied stream into a caller-provided destination buffer with an explicit maximum count.

Observable behavior that must be preserved:

- input is read from the given stream
- at most the specified bound is used for the destination content
- the returned result indicates whether a string buffer was produced or input could not be obtained, matching the role of the C interface

### Feature 5: Block input into caller memory

The module provides bulk input into caller-supplied memory.

This behavior is represented by:

- `fread(void *ptr, size_t s, size_t n, FILE *stream)`

The Rust version must support reading up to a requested number of elements of a requested size from the provided stream into the destination memory region.

Observable behavior that must be preserved:

- the destination buffer is caller-supplied
- the requested transfer is defined by element size and element count
- the function returns the number of elements actually read, in the same role as the C interface

## User Scenarios & Testing

### Scenario 1: Read structured values from standard input

A caller needs to parse structured text from standard input using a format string and receive the conversion count/result through the formatted input API.

Supported by:

- `scanf`
- `vscanf`

Tests should verify:

- input is consumed from standard input rather than an arbitrary stream
- the same format string produces the expected parsing outcome
- the function returns the expected integer result for successful and unsuccessful conversion situations

### Scenario 2: Read structured values from a specific open stream

A caller has an existing `FILE` stream and uses formatted input to extract values from that stream, without using standard input.

Supported by:

- `fscanf`
- `vfscanf`

Tests should verify:

- only the supplied stream is consumed
- format-directed parsing works through both direct and `va_list`-based entry points
- return values reflect formatted input outcomes consistently between the stream-specific and standard-input variants

### Scenario 3: Read one character at a time

A caller performs incremental input processing and needs the next input character or needs to detect end-of-file/error.

Supported by:

- `getchar`
- `fgetc`

Tests should verify:

- `getchar` reads from standard input
- `fgetc` reads from the given stream
- successful reads return the expected next character code
- end-of-input behavior is distinguishable through the returned integer result

### Scenario 4: Read a bounded line into a caller buffer

A caller needs to fetch a line or partial line from a stream into an existing mutable character buffer with a size limit.

Supported by:

- `fgets`

Tests should verify:

- no more than the requested bound is written according to the function contract
- data comes from the given stream
- the result indicates success by returning the destination string pointer role, or failure/end condition by a null-equivalent result role

### Scenario 5: Read bulk data into memory

A caller needs to load bytes or records from a stream into an existing destination region.

Supported by:

- `fread`

Tests should verify:

- the requested transfer uses the supplied element size and count
- the destination receives the bytes actually read
- the return value equals the number of complete elements read
- partial completion is reported through the count rather than requiring a separate API

## Requirements

### Functional Requirements

#### FR-1: Standard-input formatted reading

The Rust module shall provide behavior equivalent to `scanf` and `vscanf` for performing format-driven input from standard input.

Traceability:

- `gnu/stdio-read.c:88-99`
- `gnu/stdio-read.c:122-126`

#### FR-2: Stream-specific formatted reading

The Rust module shall provide behavior equivalent to `fscanf` and `vfscanf` for performing format-driven input from a caller-specified stream.

Traceability:

- `gnu/stdio-read.c:105-116`
- `gnu/stdio-read.c:132-137`

#### FR-3: Formatted-input result reporting

For formatted input entry points, the Rust module shall return an integer result serving the same functional purpose as the corresponding C interfaces' return value.

Traceability:

- `scanf`
- `fscanf`
- `vscanf`
- `vfscanf`

#### FR-4: Standard-input single-character reading

The Rust module shall provide behavior equivalent to `getchar` for reading one character/input unit from standard input.

Traceability:

- `gnu/stdio-read.c:140-144`

#### FR-5: Stream-specific single-character reading

The Rust module shall provide behavior equivalent to `fgetc` for reading one character/input unit from a caller-specified stream.

Traceability:

- `gnu/stdio-read.c:146-151`

#### FR-6: Single-character result signaling

For single-character input entry points, the Rust module shall return an integer result serving the same functional purpose as the corresponding C interfaces, including distinguishing successful read from non-success terminal conditions.

Traceability:

- `getchar`
- `fgetc`

#### FR-7: Bounded line input

The Rust module shall provide behavior equivalent to `fgets` for reading from a caller-specified stream into a caller-provided character buffer under an explicit size bound.

Traceability:

- `gnu/stdio-read.c:153-158`

#### FR-8: Line-input result signaling

For line-oriented input, the Rust module shall return a result serving the same functional purpose as the C `fgets` return value, indicating whether the destination buffer was produced.

Traceability:

- `fgets`

#### FR-9: Block input to caller memory

The Rust module shall provide behavior equivalent to `fread` for reading data from a caller-specified stream into caller-provided memory using element size and element count parameters.

Traceability:

- `gnu/stdio-read.c:162-167`

#### FR-10: Block-input completion reporting

For bulk input, the Rust module shall return a count serving the same functional purpose as the C `fread` return value: the number of elements successfully read.

Traceability:

- `fread`

### Key Entities

This module does not define module-owned structs or persistent records in the provided source file. The key entities it operates on are external interface entities required by the function signatures.

#### Entity 1: `FILE` stream

Represents the source of input for stream-specific functions.

Used by:

- `fscanf`
- `vfscanf`
- `fgetc`
- `fgets`
- `fread`

Relationship:
- functions consume input from a supplied `FILE` instance
- standard-input variants are equivalent in role but use the process standard input stream implicitly

#### Entity 2: Format string

Represents the parsing/conversion specification for formatted input.

Used by:

- `scanf`
- `fscanf`
- `vscanf`
- `vfscanf`

Relationship:
- determines how bytes from the selected input stream are interpreted and assigned

#### Entity 3: Variable argument list / argument targets

Represents the caller-provided destinations for formatted conversions.

Used by:

- `scanf`
- `fscanf`
- `vscanf`
- `vfscanf`

Relationship:
- receives parsed values according to the format string
- `v*` forms accept a preassembled argument list rather than direct variadic arguments

#### Entity 4: Character buffer

Represents caller-owned mutable storage for line-based input.

Used by:

- `fgets`

Relationship:
- receives text read from the selected stream subject to the provided bound

#### Entity 5: Raw memory buffer and transfer sizing

Represents caller-owned destination storage and the element-size/element-count request for block input.

Used by:

- `fread`

Relationship:
- receives data from the selected stream
- transfer request is defined by `(size, count)`

## Success Criteria

### SC-1: API coverage

The Rust rewrite exposes functional equivalents for all eight evidenced entry points in `gnu/stdio-read.c`:

- formatted input from standard input
- formatted input from a specified stream
- `va_list`-style formatted variants
- single-character standard-input read
- single-character stream read
- bounded line read
- block read

Traceability:
- all listed functions in the module analysis

### SC-2: Correct input-source selection

Tests demonstrate that standard-input variants consume from standard input and stream-specific variants consume from the provided stream.

Traceability:
- `scanf`, `vscanf`, `getchar`
- `fscanf`, `vfscanf`, `fgetc`, `fgets`, `fread`

### SC-3: Return-value role preservation

For each function category, tests demonstrate that the Rust implementation reports outcomes in the same functional form as the C interface:

- integer outcome for formatted input
- integer character/non-success signaling for single-character input
- success/failure pointer-role outcome for line input
- element-count outcome for block input

Traceability:
- all evidenced functions

### SC-4: Bounded line-read behavior

Tests demonstrate that the `fgets` equivalent respects the caller-provided maximum size and writes into caller-provided buffer storage only within that contract.

Traceability:
- `gnu/stdio-read.c:153-158`

### SC-5: Sized bulk-read behavior

Tests demonstrate that the `fread` equivalent uses the supplied element size and count to determine the requested transfer and reports the number of complete elements actually read.

Traceability:
- `gnu/stdio-read.c:162-167`

### SC-6: Consistency across formatted-input entry points

Tests demonstrate that standard-input and stream-specific formatted input, including direct and `v*` forms, behave consistently with the same format-driven parsing expectations when pointed at equivalent input data.

Traceability:
- `scanf`
- `fscanf`
- `vscanf`
- `vfscanf`