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

This module provides write-oriented stdio entry points corresponding to formatted output, character output, string output, line output, and block output operations.

The source module defines public functions that route output requests to either the standard output stream or a caller-supplied stream. The Rust rewrite must preserve the same functional scope and observable behavior category:

- formatted output to standard output and to a specified stream,
- variadic-form formatted output via `va_list`-style entry points,
- single-character output,
- string output with and without an appended newline,
- binary/block output of a counted byte range.

This specification is limited to the functionality evidenced by `gnu/stdio-write.c` and its listed exported functions.

## Scope

### In Scope

The Rust version must implement the functional behavior represented by these operations from `gnu/stdio-write.c`:

- formatted write to standard output,
- formatted write to a specified stream,
- formatted write from a variadic argument list to standard output,
- formatted write from a variadic argument list to a specified stream,
- writing one character to standard output,
- writing one character to a specified stream,
- writing a NUL-terminated string to a specified stream,
- writing a NUL-terminated string followed by a newline to standard output,
- writing `n` elements of size `s` from a caller-provided memory region to a specified stream.

### Out of Scope

The Rust version must not claim or introduce functionality not evidenced by this module, including:

- new public APIs beyond the module’s evidenced write-oriented stdio surface,
- read operations,
- file opening, closing, seeking, buffering policy control, or stream creation APIs,
- concurrency guarantees,
- serialization protocols,
- recovery or retry mechanisms,
- performance targets beyond functional equivalence.

## Feature Specification

### Feature: Formatted output dispatch

The module exposes formatted-output functions for standard output and arbitrary streams. The Rust rewrite must support:

- accepting a format string and arguments for output to standard output,
- accepting a format string and arguments for output to a specified stream,
- accepting a format string with a prebuilt variadic argument list for standard output,
- accepting a format string with a prebuilt variadic argument list for a specified stream.

These operations must produce formatted textual output on the selected destination and report completion using integer return values consistent with the role of stdio write functions.

Traceability:
- `printf`
- `fprintf`
- `vprintf`
- `vfprintf`

### Feature: Character and string output

The module provides direct text emission helpers. The Rust rewrite must support:

- writing a single character to standard output,
- writing a single character to a specified stream,
- writing a NUL-terminated string to a specified stream,
- writing a NUL-terminated string followed by a newline to standard output.

These operations must target the expected destination and preserve the distinction between plain string output and line output with newline termination.

Traceability:
- `putchar`
- `fputc`
- `fputs`
- `puts`

### Feature: Counted block output

The module provides a counted raw-write operation over a caller-supplied memory region. The Rust rewrite must support:

- writing a sequence consisting of `n` elements of size `s` from a memory buffer to a specified stream,
- reporting completion using a count-based return value appropriate to block output.

The operation must be defined in terms of caller-provided byte data and explicit element count/size inputs.

Traceability:
- `fwrite`

## User Scenarios & Testing

### Scenario 1: Write formatted text to standard output

A caller emits user-facing text using a format string and runtime values, targeting standard output.

The Rust version must support:
- invocation through the standard-output formatted write entry point,
- visible formatted text emission to standard output,
- integer status/result reporting.

Representative tests:
- write a fixed literal format string with no substitutions,
- write a format string with one or more value substitutions,
- verify output destination is standard output rather than an arbitrary stream.

Traceability:
- `printf`
- `vprintf`

### Scenario 2: Write formatted text to a chosen stream

A caller emits formatted text to a specific open stream such as a file-backed or memory-backed stdio stream supplied externally.

The Rust version must support:
- invocation with an explicit stream parameter,
- formatted emission to that stream,
- result reporting through the function return value.

Representative tests:
- write formatted output to a non-stdout stream,
- compare stream contents against expected formatted text,
- verify the standard-output helper and stream-specific helper target different destinations when given different streams.

Traceability:
- `fprintf`
- `vfprintf`

### Scenario 3: Emit one character at a time

A caller writes individual characters either to standard output or to a chosen stream.

The Rust version must support:
- single-character output to standard output,
- single-character output to a supplied stream,
- preservation of the written character value in output behavior.

Representative tests:
- write one printable character to standard output,
- write a sequence of characters by repeated calls and verify accumulated output,
- write to a non-stdout stream and verify stream content.

Traceability:
- `putchar`
- `fputc`

### Scenario 4: Emit strings and newline-terminated lines

A caller writes complete text strings to a stream, or writes a line to standard output with automatic newline addition.

The Rust version must support:
- writing a provided string as-is to a chosen stream,
- writing a provided string to standard output followed by a newline,
- preserving the difference between no-newline and newline-appending behaviors.

Representative tests:
- call the stream string function and verify no extra newline is appended,
- call the line-output function and verify exactly one newline is appended after the string,
- compare outputs between the two entry points using the same input string.

Traceability:
- `fputs`
- `puts`

### Scenario 5: Write binary or arbitrary byte blocks

A caller writes counted data from memory to a stream, including data that may contain zero bytes or may not represent text.

The Rust version must support:
- block writes controlled by element size and element count,
- writing arbitrary byte sequences to a stream,
- count-based result reporting.

Representative tests:
- write a byte slice with `s = 1` and `n = len`,
- write multiple fixed-size elements and verify the total bytes written correspond to `s * n`,
- write data containing embedded zero bytes and verify content is preserved in the destination stream.

Traceability:
- `fwrite`

## Requirements

### Functional Requirements

#### FR-1: Standard-output formatted write

The module shall provide a formatted-output operation targeting the standard output stream.

Expected behavior:
- accepts a format string and arguments,
- writes the formatted result to standard output,
- returns an integer result representing write completion status.

Traceability:
- `printf` in `gnu/stdio-write.c`

#### FR-2: Stream-targeted formatted write

The module shall provide a formatted-output operation targeting a caller-supplied stream.

Expected behavior:
- accepts a stream, a format string, and arguments,
- writes the formatted result to the specified stream,
- returns an integer result representing write completion status.

Traceability:
- `fprintf` in `gnu/stdio-write.c`

#### FR-3: Variadic-list formatted write to standard output

The module shall provide a formatted-output operation targeting standard output using a variadic argument-list input.

Expected behavior:
- accepts a format string and argument-list object,
- writes the formatted result to standard output,
- returns an integer result representing write completion status.

Traceability:
- `vprintf` in `gnu/stdio-write.c`

#### FR-4: Variadic-list formatted write to a supplied stream

The module shall provide a formatted-output operation targeting a supplied stream using a variadic argument-list input.

Expected behavior:
- accepts a stream, a format string, and argument-list object,
- writes the formatted result to the specified stream,
- returns an integer result representing write completion status.

Traceability:
- `vfprintf` in `gnu/stdio-write.c`

#### FR-5: Single-character output to standard output

The module shall provide an operation that writes one character to standard output.

Expected behavior:
- accepts a character value,
- emits one output character to standard output,
- returns an integer result appropriate to character output.

Traceability:
- `putchar` in `gnu/stdio-write.c`

#### FR-6: Single-character output to a supplied stream

The module shall provide an operation that writes one character to a caller-supplied stream.

Expected behavior:
- accepts a character value and a stream,
- emits one output character to that stream,
- returns an integer result appropriate to character output.

Traceability:
- `fputc` in `gnu/stdio-write.c`

#### FR-7: String output to a supplied stream

The module shall provide an operation that writes a NUL-terminated string to a caller-supplied stream without adding a newline.

Expected behavior:
- accepts a string and a stream,
- writes the string bytes up to the terminating NUL,
- does not append a newline as part of this operation,
- returns an integer result appropriate to string output.

Traceability:
- `fputs` in `gnu/stdio-write.c`

#### FR-8: Line output to standard output

The module shall provide an operation that writes a NUL-terminated string to standard output and appends a newline.

Expected behavior:
- accepts a string,
- writes the string to standard output,
- appends one newline after the string,
- returns an integer result appropriate to line output.

Traceability:
- `puts` in `gnu/stdio-write.c`

#### FR-9: Counted block output to a supplied stream

The module shall provide an operation that writes counted data from a caller-provided memory region to a caller-supplied stream.

Expected behavior:
- accepts a source memory pointer, element size `s`, element count `n`, and stream,
- attempts to write `n` elements of `s` bytes each,
- returns a `size_t`-style count result for the block-write operation.

Traceability:
- `fwrite` in `gnu/stdio-write.c`

### Key Entities

#### Entity: Output stream handle

A stream handle represents the destination for stream-targeted write operations.

Relationships:
- used by stream-targeted formatted output,
- used by stream-targeted character output,
- used by stream-targeted string output,
- used by block output,
- omitted by standard-output convenience operations, which target the standard output stream implicitly.

Traceability:
- parameter `FILE *stream` in `fprintf`, `vfprintf`, `fputc`, `fputs`, `fwrite`

#### Entity: Format string

A format string represents the template used by formatted-output operations.

Relationships:
- consumed by standard-output formatted write,
- consumed by stream-targeted formatted write,
- paired either with direct variadic arguments or with an argument-list object.

Traceability:
- parameter `const char *format` in `printf`, `fprintf`, `vprintf`, `vfprintf`

#### Entity: Variadic argument list

A variadic argument-list object represents pre-collected formatting arguments for `v*` output operations.

Relationships:
- combined with a format string,
- used only by the `vprintf` and `vfprintf` forms.

Traceability:
- parameter `va_list args` in `vprintf`, `vfprintf`

#### Entity: Character value

A character value represents a single output unit for character-write operations.

Relationships:
- passed to standard-output character output,
- passed to stream-targeted character output.

Traceability:
- parameter `int c` in `putchar`, `fputc`

#### Entity: NUL-terminated string

A NUL-terminated string represents textual data for string and line output operations.

Relationships:
- written directly to a supplied stream without newline addition,
- written to standard output with newline addition in the line-output variant.

Traceability:
- parameter `const char *string` in `fputs`, `puts`

#### Entity: Counted memory buffer

A counted memory buffer represents raw source data for block output.

Relationships:
- consists of a base pointer plus element size and element count,
- written to a supplied stream by the block output operation.

Traceability:
- parameters `const void *ptr`, `size_t s`, `size_t n` in `fwrite`

## Success Criteria

### SC-1: Functional surface completeness

The Rust module exposes behavior covering all write-oriented functional entry points evidenced in `gnu/stdio-write.c`:
- formatted output to standard output,
- formatted output to a specified stream,
- variadic-list formatted output to standard output,
- variadic-list formatted output to a specified stream,
- character output to standard output,
- character output to a specified stream,
- string output to a specified stream,
- line output to standard output,
- counted block output to a specified stream.

Verification:
- feature-to-source trace review against the listed functions.

### SC-2: Destination correctness

Each operation writes to the destination implied by its interface:
- standard-output variants target standard output,
- stream variants target the supplied stream.

Verification:
- tests using distinct output captures for standard output and non-stdout streams show output appears only at the expected destination.

Traceability:
- `printf`, `vprintf`, `putchar`, `puts`
- `fprintf`, `vfprintf`, `fputc`, `fputs`, `fwrite`

### SC-3: Newline behavior correctness

String and line output remain behaviorally distinct:
- stream string output appends no newline,
- line output appends one newline after the provided string.

Verification:
- tests compare captured output bytes for the same input string across the two operations.

Traceability:
- `fputs`
- `puts`

### SC-4: Block write count semantics preserved

The block output operation accepts explicit element size and count inputs and returns a count-style result for the write attempt.

Verification:
- tests exercise combinations of `s` and `n`,
- result values are validated as count-based rather than character-based or status-only.

Traceability:
- `fwrite`

### SC-5: Arbitrary byte preservation for block output

The block output operation can write non-text byte sequences, including embedded zero bytes, without truncating at zero.

Verification:
- tests write buffers containing zero bytes and compare resulting stream contents byte-for-byte.

Traceability:
- `fwrite`

### SC-6: Formatted output path support

Both direct variadic-style formatted entry points and argument-list-based formatted entry points are supported for standard output and stream destinations.

Verification:
- tests cover both formatted call styles and confirm equivalent output for equivalent inputs.

Traceability:
- `printf`, `fprintf`, `vprintf`, `vfprintf`