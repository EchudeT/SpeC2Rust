# spec.md

## Title

Rust Functional Specification for `module_gnu_stdio-write.c_49`

## Metadata

- Project: `cflow-new`
- Module: `module_gnu_stdio-write.c_49`
- Category: `module_cluster`
- Source file: `gnu/stdio-write.c`
- Rust branch: `055-module_gnu_stdio_write.c_49-rust-port`
- Generation date: `2026-06-17`

## Overview

This module provides write-side stdio-style output entry points for formatted text output, character output, string output, and raw block output. The source module exposes public functions corresponding to common C stdio write operations and routes output either to the standard output stream or to a caller-supplied stream.

The Rust rewrite must preserve the functional behavior boundary evidenced by the source file: it must provide the same write-oriented capabilities covered by the module’s public surface, including:

- formatted output to standard output and arbitrary output streams
- variadic-formatted output equivalents where applicable to the Rust design
- single-character output
- NUL-terminated string output
- line output to standard output with newline termination
- raw byte/block output to a stream

This specification defines required behavior only within that evidenced boundary and does not introduce additional public capabilities.

## Scope

### In Scope

The Rust version must implement the behavior represented by these source entry points:

- `printf`
- `fprintf`
- `vprintf`
- `vfprintf`
- `putchar`
- `fputc`
- `fputs`
- `puts`
- `fwrite`

### Out of Scope

The following are not required unless already necessary to preserve the above behavior:

- read-side stdio behavior
- creation or ownership management of stream objects beyond accepting or using existing streams
- new formatting languages or extensions not evidenced by the source module
- buffering policy guarantees beyond what is required for observable write behavior
- thread-safety guarantees
- recovery, retry, or durability features
- additional public APIs beyond the module boundary above

## Feature Specification

### Feature 1: Formatted output to standard output

The module supports writing formatted output to the standard output stream.

This includes:
- accepting a format string with arguments
- producing output on standard output
- returning an integer result representing the outcome of the write operation

Traceability:
- `printf`
- `vprintf`

### Feature 2: Formatted output to a specified stream

The module supports writing formatted output to a caller-specified output stream.

This includes:
- accepting a stream handle/reference
- accepting a format string with arguments
- producing output on the provided stream
- returning an integer result representing the outcome of the write operation

Traceability:
- `fprintf`
- `vfprintf`

### Feature 3: Single-character output

The module supports writing a single character either to standard output or to a specified stream.

This includes:
- writing one character value to standard output
- writing one character value to a provided stream
- returning an integer result representing the outcome of the write operation

Traceability:
- `putchar`
- `fputc`

### Feature 4: String output

The module supports writing a NUL-terminated string either to a specified stream or, in line-oriented form, to standard output.

This includes:
- writing the bytes of a string to a provided stream
- writing a string to standard output followed by a newline for the line-oriented operation
- returning an integer result representing the outcome of the write operation

Traceability:
- `fputs`
- `puts`

### Feature 5: Raw block output

The module supports writing a sequence of raw bytes from memory to a specified stream.

This includes:
- accepting a memory region pointer/reference
- accepting element size and element count
- attempting to write data corresponding to the requested byte count
- returning a `size_t`-compatible count result representing write progress/outcome

Traceability:
- `fwrite`

## User Scenarios & Testing

### Scenario 1: Print formatted status text to standard output

A caller needs to display formatted text on standard output, such as a message containing numeric or textual values.

Expected support:
- the module accepts the format string and values
- output appears on standard output in formatted form
- the call returns a result value consistent with successful or failed output

Traceability:
- `printf`
- `vprintf`

Suggested tests:
- write a plain literal format string
- write a format string with substituted values
- verify output destination is standard output
- verify the result indicates success when output completes

### Scenario 2: Print formatted text to a chosen stream

A caller needs to send formatted output to a non-stdout stream, such as a file-backed stream or test-controlled output sink.

Expected support:
- the module writes to the specified stream rather than standard output
- formatting behavior is applied to the provided arguments
- the call returns a result value reflecting the write outcome

Traceability:
- `fprintf`
- `vfprintf`

Suggested tests:
- write formatted output to an in-memory or temporary stream
- verify bytes are written to the provided stream only
- verify the returned status reflects successful completion

### Scenario 3: Emit a single character

A caller needs to write one character at a time, either to standard output or to a specific stream.

Expected support:
- one character is written per call
- destination selection follows the API used
- the return value reflects the write outcome

Traceability:
- `putchar`
- `fputc`

Suggested tests:
- write a printable character to standard output
- write a printable character to a supplied stream
- verify exactly one character is appended to the destination

### Scenario 4: Write a string without formatting

A caller already has a complete string and needs to write it directly to a stream without format substitution.

Expected support:
- the module writes the string contents to the specified stream
- for the line-oriented standard-output operation, a newline is added after the string
- return values indicate the outcome

Traceability:
- `fputs`
- `puts`

Suggested tests:
- write a non-empty string to a supplied stream and verify exact byte content
- call the line-oriented standard-output operation and verify newline termination
- write an empty string and verify only the newline is added for the line-oriented operation

### Scenario 5: Write binary or arbitrary memory data

A caller needs to write raw bytes that may include non-text content.

Expected support:
- the module accepts a source memory region, element size, element count, and destination stream
- data is written according to the requested total size
- the returned count communicates how much was written

Traceability:
- `fwrite`

Suggested tests:
- write a known byte buffer with `size=1` and `count=len`
- write multiple fixed-size elements and verify concatenated output bytes
- verify the returned count matches the number of elements successfully written

## Requirements

### Functional Requirements

#### FR-1: Standard output formatted writing
The Rust module shall provide functionality equivalent to formatted writing to the standard output stream.

Traceability:
- `printf`
- `vprintf`

#### FR-2: Stream-targeted formatted writing
The Rust module shall provide functionality equivalent to formatted writing to a caller-supplied stream.

Traceability:
- `fprintf`
- `vfprintf`

#### FR-3: Standard output single-character writing
The Rust module shall provide functionality equivalent to writing a single character to standard output.

Traceability:
- `putchar`

#### FR-4: Stream-targeted single-character writing
The Rust module shall provide functionality equivalent to writing a single character to a caller-supplied stream.

Traceability:
- `fputc`

#### FR-5: Stream-targeted string writing
The Rust module shall provide functionality equivalent to writing a NUL-terminated string’s contents to a caller-supplied stream.

Traceability:
- `fputs`

#### FR-6: Line-oriented standard output string writing
The Rust module shall provide functionality equivalent to writing a string to standard output with newline termination.

Traceability:
- `puts`

#### FR-7: Raw block writing
The Rust module shall provide functionality equivalent to writing raw data from memory to a caller-supplied stream using element size and element count inputs.

Traceability:
- `fwrite`

#### FR-8: Destination fidelity
For operations that accept a stream parameter, the Rust module shall direct output to the supplied stream; for operations without a stream parameter, it shall direct output to standard output.

Traceability:
- `fprintf`
- `vfprintf`
- `fputc`
- `fputs`
- `fwrite`
- `printf`
- `vprintf`
- `putchar`
- `puts`

#### FR-9: Result reporting
The Rust module shall return operation results in forms corresponding to the source API categories: integer status/count results for formatted, character, and string output operations, and `size_t`-compatible element-count results for raw block writing.

Traceability:
- `printf`
- `fprintf`
- `vprintf`
- `vfprintf`
- `putchar`
- `fputc`
- `fputs`
- `puts`
- `fwrite`

### Key Entities

#### 1. Output stream
A write destination representing either:
- the process standard output stream, or
- a caller-supplied stream

Relationship to module behavior:
- all operations write to an output stream
- some operations implicitly select standard output
- others require the caller to supply the stream

Traceability:
- `fprintf`
- `vfprintf`
- `fputc`
- `fputs`
- `fwrite`
- implicit standard output use in `printf`, `vprintf`, `putchar`, `puts`

#### 2. Format string
A textual formatting specification used by formatted-output operations.

Relationship to module behavior:
- consumed together with arguments
- determines the formatted bytes emitted to the destination stream

Traceability:
- `printf`
- `fprintf`
- `vprintf`
- `vfprintf`

#### 3. Variadic argument list
An argument collection paired with a format string for the `v*` formatted-output forms.

Relationship to module behavior:
- provides the values consumed by the formatting operation
- is used only by the variadic-list-based formatted entry points

Traceability:
- `vprintf`
- `vfprintf`

#### 4. Character value
A single output character value written by character-output operations.

Relationship to module behavior:
- forms the entire payload of `putchar` and `fputc`

Traceability:
- `putchar`
- `fputc`

#### 5. String input
A NUL-terminated byte string supplied for direct text output.

Relationship to module behavior:
- written directly by string-output operations
- written with newline termination by the line-oriented standard-output operation

Traceability:
- `fputs`
- `puts`

#### 6. Raw memory buffer
A contiguous input memory region interpreted for block writing together with element size and count.

Relationship to module behavior:
- provides the source bytes for raw output
- is paired with size/count metadata to determine intended write volume

Traceability:
- `fwrite`

## Success Criteria

### SC-1: Formatted standard output behavior
For representative formatted input cases, the Rust module produces formatted output on standard output and returns a success-compatible integer result when the write completes successfully.

Traceability:
- `printf`
- `vprintf`

### SC-2: Formatted stream output behavior
For representative formatted input cases, the Rust module produces formatted output on the supplied stream and does not redirect that output to standard output.

Traceability:
- `fprintf`
- `vfprintf`

### SC-3: Character output correctness
When given a single character, the Rust module writes exactly one character to the selected destination and returns a result consistent with successful completion.

Traceability:
- `putchar`
- `fputc`

### SC-4: String output correctness
When given a string:
- the stream-targeted string operation writes the string contents as provided, and
- the line-oriented standard-output operation writes the string contents followed by a newline.

Traceability:
- `fputs`
- `puts`

### SC-5: Raw block output correctness
When given a byte buffer, element size, and element count, the Rust module writes the requested data sequence to the supplied stream and returns an element-count result consistent with completed writes.

Traceability:
- `fwrite`

### SC-6: Destination selection correctness
In tests covering all public operations, functions with explicit stream parameters write to the provided stream, and functions without explicit stream parameters write to standard output.

Traceability:
- `printf`
- `fprintf`
- `vprintf`
- `vfprintf`
- `putchar`
- `fputc`
- `fputs`
- `puts`
- `fwrite`

## Acceptance Notes

- The Rust rewrite may adapt language-level API shape as needed, but it must preserve the functional boundary and externally observable write behavior defined in this specification.
- Any Rust-side adaptation for variadic formatted forms must still preserve the evidenced distinction between formatted output to standard output and formatted output to a specified stream.
- No additional module responsibilities are required beyond the behaviors traceable to `gnu/stdio-write.c`.