# spec.md

## Title

Rust Functional Specification for `module_gnu_stdio-write.c_49`

## Metadata

- Project: `cflow-new`
- Module: `module_gnu_stdio-write.c_49`
- Category: `module_cluster`
- Source file: `gnu/stdio-write.c`
- Rust branch: `055-module_gnu_stdio_write.c_49-rust-port`
- Generation date: `2026-06-11`

## Overview

This module provides write-oriented stdio wrapper behavior for formatted output, character output, string output, and binary/block output.

The source evidence shows a compact API surface that exposes the following C stdio-style operations:

- formatted writing to standard output and arbitrary streams
- variadic and `va_list`-based formatted writing
- single-character writing to standard output and arbitrary streams
- C-string writing to arbitrary streams and to standard output with newline behavior for `puts`
- block writing of raw bytes to arbitrary streams

The Rust rewrite must preserve this functional boundary: it must provide equivalent write-facing behavior for the operations represented by the source module, without expanding the module into unrelated stdio features.

## Scope

### In Scope

The Rust version must implement the behavior corresponding to the module’s write APIs evidenced in `gnu/stdio-write.c`:

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

The following are not evidenced by this module and must not be introduced as required behavior in the specification:

- input/parsing stdio features
- file opening, closing, seeking, or buffering policy APIs
- extended formatting features beyond the underlying formatted output behavior already implied by the named functions
- thread-safety guarantees
- recovery, retry, persistence, or transactional behavior
- serialization protocols
- new public APIs beyond those needed to represent the evidenced module behavior

## Feature Specification

### Feature 1: Formatted output dispatch

The module provides formatted output functions for writing formatted text either to standard output or to a specified stream.

#### Required behavior

- The Rust version must support formatted output directed to the standard output target, corresponding to `printf`.
- The Rust version must support formatted output directed to a caller-specified stream target, corresponding to `fprintf`.
- The Rust version must support formatting driven by an explicit variadic-argument-list equivalent, corresponding to `vprintf` and `vfprintf`.
- Standard-output formatted writing and explicit-stream formatted writing must behave consistently with each other except for the destination target.
- The result of a formatted write operation must indicate success or failure in a way equivalent to the source module’s return-bearing contract.

### Feature 2: Single-character output

The module provides character-oriented output to standard output and to arbitrary streams.

#### Required behavior

- The Rust version must support writing a single character/unit corresponding to `putchar` to standard output.
- The Rust version must support writing a single character/unit corresponding to `fputc` to a specified stream.
- The operation must report success or failure in a return value equivalent to the source behavior.

### Feature 3: String output

The module provides writing of NUL-terminated strings to a stream and to standard output.

#### Required behavior

- The Rust version must support writing a caller-provided string to a specified stream, corresponding to `fputs`.
- The Rust version must support writing a caller-provided string to standard output and appending the newline behavior associated with `puts`.
- `fputs` and `puts` must remain behaviorally distinct where the source API distinguishes newline handling.
- The operation must report success or failure in a return value equivalent to the source behavior.

### Feature 4: Raw block output

The module provides block writing of raw memory to a specified stream.

#### Required behavior

- The Rust version must support writing `n` items of size `s` from a caller-provided memory region to a specified stream, corresponding to `fwrite`.
- The return value must indicate how much data was successfully written, equivalent to the count-oriented contract of the source API.
- Partial completion must be representable through the return value when the full request is not written.

## User Scenarios & Testing

### Scenario 1: Write formatted text to standard output

A caller needs to emit formatted text to the process standard output destination.

#### Expected support

- A call equivalent to `printf` writes formatted content to standard output.
- The call returns a success/failure indicator equivalent to the source contract.

#### Test focus

- formatting path to standard output is reachable
- output destination is standard output, not an arbitrary stream
- return contract is preserved

### Scenario 2: Write formatted text to a chosen stream

A caller has a stream handle and needs to emit formatted text to that stream.

#### Expected support

- A call equivalent to `fprintf` writes formatted content to the provided stream.
- A call equivalent to `vfprintf` performs the same behavior when the arguments are supplied in list form.

#### Test focus

- destination routing to the provided stream
- consistency between direct variadic formatting and argument-list formatting
- return contract is preserved

### Scenario 3: Forward an existing argument list to output

A caller already has a prepared variadic argument list and needs to forward it to output without reconstructing arguments.

#### Expected support

- A call equivalent to `vprintf` writes to standard output using the supplied argument list.
- A call equivalent to `vfprintf` writes to the given stream using the supplied argument list.

#### Test focus

- `v*` variants accept and consume argument-list input
- destination behavior matches corresponding non-`v*` forms
- result signaling remains equivalent

### Scenario 4: Emit a single character

A caller needs lightweight output of one character either to standard output or a stream.

#### Expected support

- A call equivalent to `putchar` writes one character to standard output.
- A call equivalent to `fputc` writes one character to the provided stream.

#### Test focus

- exactly one character is emitted
- destination differs correctly between stdout and explicit stream
- return value reflects success/failure

### Scenario 5: Emit a string with or without automatic newline

A caller wants to write a string either exactly as provided or with the conventional trailing newline of `puts`.

#### Expected support

- A call equivalent to `fputs` writes the given string to the provided stream without introducing the `puts` newline behavior.
- A call equivalent to `puts` writes the given string to standard output and appends a newline.

#### Test focus

- `fputs` does not add the `puts` newline behavior
- `puts` appends one newline after the supplied string
- return contract is preserved

### Scenario 6: Write raw bytes in blocks

A caller has binary or non-text data in memory and needs to write it to a stream in item-sized blocks.

#### Expected support

- A call equivalent to `fwrite` writes from a memory buffer to the provided stream.
- The return value reports the number of items written, not merely a boolean success flag.

#### Test focus

- item count semantics are preserved
- zero, partial, and full-write cases are representable through the return value
- destination routing to the provided stream is correct

## Requirements

### Functional Requirements

#### FR-1 Formatted write to standard output
The module shall provide behavior equivalent to `printf`, writing formatted output to the standard output destination and returning a result equivalent to the source contract.

**Traceability:** `gnu/stdio-write.c`, `printf`

#### FR-2 Formatted write to a specified stream
The module shall provide behavior equivalent to `fprintf`, writing formatted output to a caller-specified stream and returning a result equivalent to the source contract.

**Traceability:** `gnu/stdio-write.c`, `fprintf`

#### FR-3 Formatted write using an argument-list form to standard output
The module shall provide behavior equivalent to `vprintf`, writing formatted output to standard output using a supplied argument-list representation.

**Traceability:** `gnu/stdio-write.c`, `vprintf`

#### FR-4 Formatted write using an argument-list form to a specified stream
The module shall provide behavior equivalent to `vfprintf`, writing formatted output to a caller-specified stream using a supplied argument-list representation.

**Traceability:** `gnu/stdio-write.c`, `vfprintf`

#### FR-5 Single-character write to standard output
The module shall provide behavior equivalent to `putchar`, writing a single character/unit to standard output and returning a result equivalent to the source contract.

**Traceability:** `gnu/stdio-write.c`, `putchar`

#### FR-6 Single-character write to a specified stream
The module shall provide behavior equivalent to `fputc`, writing a single character/unit to a caller-specified stream and returning a result equivalent to the source contract.

**Traceability:** `gnu/stdio-write.c`, `fputc`

#### FR-7 String write to a specified stream
The module shall provide behavior equivalent to `fputs`, writing a caller-provided C-style string to a specified stream and returning a result equivalent to the source contract.

**Traceability:** `gnu/stdio-write.c`, `fputs`

#### FR-8 String write to standard output with newline behavior
The module shall provide behavior equivalent to `puts`, writing a caller-provided C-style string to standard output and appending newline behavior distinct from `fputs`.

**Traceability:** `gnu/stdio-write.c`, `puts`

#### FR-9 Raw block write to a specified stream
The module shall provide behavior equivalent to `fwrite`, writing block data from caller-provided memory to a specified stream and returning the number of items successfully written.

**Traceability:** `gnu/stdio-write.c`, `fwrite`

#### FR-10 Destination consistency across paired APIs
The module shall preserve the destination distinction evidenced by the API pairs: standard-output forms target standard output; `f*` forms target the supplied stream.

**Traceability:** `gnu/stdio-write.c`, `printf`, `fprintf`, `vprintf`, `vfprintf`, `putchar`, `fputc`, `fputs`, `puts`, `fwrite`

#### FR-11 Behavioral distinction between plain string write and newline-appending string write
The module shall preserve the distinct behavior between stream string output and standard-output string output with trailing newline, as evidenced by `fputs` and `puts`.

**Traceability:** `gnu/stdio-write.c`, `fputs`, `puts`

#### FR-12 Count-oriented completion reporting for block writes
The module shall preserve count-oriented completion reporting for raw block output, including the ability to represent fewer items written than requested.

**Traceability:** `gnu/stdio-write.c`, `fwrite`

### Key Entities

This module does not define its own data structures in the provided source evidence. Its behavior is organized around established stdio entities and call inputs.

#### Entity 1: Output stream
A stream destination corresponding to `FILE *` is the target for stream-directed operations.

**Relationships:**
- used by `fprintf`, `vfprintf`, `fputc`, `fputs`, and `fwrite`
- distinguishes explicit-stream functions from standard-output functions

#### Entity 2: Standard output destination
The process standard output destination is the implicit target for standard-output operations.

**Relationships:**
- used by `printf`, `vprintf`, `putchar`, and `puts`
- parallels explicit-stream operations with an implicit destination

#### Entity 3: Format string
A caller-provided format specification drives formatted output operations.

**Relationships:**
- consumed by `printf`, `fprintf`, `vprintf`, and `vfprintf`
- paired with either direct variadic arguments or an argument-list form

#### Entity 4: Variadic argument list
An argument-list representation corresponding to `va_list` supplies formatted output arguments for `v*` functions.

**Relationships:**
- consumed by `vprintf` and `vfprintf`
- serves as the argument source alternative to direct variadic calls

#### Entity 5: C-style string input
A NUL-terminated string input is written by string-output functions.

**Relationships:**
- consumed by `fputs` and `puts`
- destination and newline behavior depend on the selected function

#### Entity 6: Raw memory buffer
A caller-provided memory region supplies bytes/items for block output.

**Relationships:**
- consumed by `fwrite`
- interpreted together with item size and item count to determine the write request

## Success Criteria

### SC-1 API-equivalent functional coverage
The Rust rewrite implements behavior covering all nine evidenced write operations from `gnu/stdio-write.c`.

**Traceability:** `printf`, `fprintf`, `vprintf`, `vfprintf`, `putchar`, `fputc`, `fputs`, `puts`, `fwrite`

### SC-2 Correct destination routing
Tests demonstrate that standard-output operations target standard output and stream-based operations target the caller-specified stream.

**Traceability:** `printf`, `fprintf`, `vprintf`, `vfprintf`, `putchar`, `fputc`, `fputs`, `puts`, `fwrite`

### SC-3 Preserved newline distinction for string-output calls
Tests demonstrate that the `puts`-equivalent appends newline behavior while the `fputs`-equivalent does not introduce that behavior.

**Traceability:** `fputs`, `puts`

### SC-4 Preserved argument-list support for formatted output
Tests demonstrate that `vprintf`-equivalent and `vfprintf`-equivalent behavior accept argument-list-based formatted output and produce output consistent with their paired non-`v*` forms for the same destination.

**Traceability:** `vprintf`, `vfprintf`, `printf`, `fprintf`

### SC-5 Single-character output behavior is preserved
Tests demonstrate that character-output operations emit one character to the correct destination and return a result equivalent to the source contract.

**Traceability:** `putchar`, `fputc`

### SC-6 Block-write completion reporting is preserved
Tests demonstrate that the `fwrite`-equivalent returns the number of items written and can represent less-than-requested completion.

**Traceability:** `fwrite`

### SC-7 No unsupported feature expansion
The Rust rewrite does not require or expose additional module functionality outside the write-oriented stdio boundary evidenced by `gnu/stdio-write.c`.

**Traceability:** `gnu/stdio-write.c`