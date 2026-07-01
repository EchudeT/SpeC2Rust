# spec.md

## Overview

This module provides argument quoting services for the `pwd` project. It transforms input strings or byte sequences into quoted representations suitable for display, diagnostics, or shell-facing text, with behavior controlled by quoting options and quoting style selection.

The Rust rewrite must preserve the observable behavior evidenced by `quotearg.c`, especially for:

- selecting locale-sensitive quotation delimiters,
- producing quoted output for strings and memory buffers,
- supporting option-driven and character-driven quoting variations,
- managing reusable storage for returned quoted strings,
- releasing module-managed quoting storage.

## Scope

In scope for this module:

- Quoting a NUL-terminated string.
- Quoting an explicit byte buffer with caller-provided length.
- Producing quoted output according to a selected quoting style and option set.
- Applying extra quoting for selected characters.
- Choosing quotation marks based on style and locale translation hooks.
- Returning quoted strings through module-managed storage for convenience APIs.
- Releasing module-managed storage allocated for quoted results.

Out of scope for this specification:

- Any behavior not evidenced by `quotearg.c`.
- New public APIs beyond the identified functions.
- Guarantees about concurrency, persistence, serialization, or foreign-function interfaces.

## Feature Specification

### Summary

The module exposes quoting utilities centered around a configurable quoting engine. It accepts either C-style strings or explicit memory buffers, converts them into quoted text, and returns the resulting representation either in a caller-provided buffer path or through convenience wrappers that use module-managed storage.

### Supported behavior

1. **Locale-aware quote delimiter selection**
   - The module must support retrieving quotation delimiters through `gettext_quote`.
   - Delimiter choice must depend on the requested message id and quoting style.
   - The Rust version must preserve behavior where translated quotation marks may differ from plain ASCII quoting.

2. **Configurable quoting transformation**
   - The module must support a core transformation path equivalent to `quotearg_buffer_restyled`.
   - The transformation must accept:
     - an input byte sequence,
     - an explicit input size,
     - a quoting style,
     - flags,
     - an optional set of additional characters that must be quoted,
     - optional explicit left and right quote strings.
   - The transformation must report the size of the quoted result and support writing into a caller-supplied output buffer subject to buffer size.

3. **Convenience quoting for default use**
   - The module must support quoting a NUL-terminated string via `quotearg`.
   - The module must support quoting a byte sequence with explicit length via `quotearg_mem`.
   - The module must support quoting with an additional forced-quoted character via `quotearg_char`.
   - The module must support quote-oriented wrappers `quote_mem` and `quote`.

4. **Reusable internal result storage**
   - Convenience APIs returning `char *` or `char const *` in the C module must be represented in Rust with equivalent observable behavior for callers of the Rust port.
   - The module must manage internal storage for quoted results across convenience calls in a way that preserves the C module’s reuse model.

5. **Storage cleanup**
   - The module must provide behavior equivalent to `quotearg_free`, releasing storage owned by the module for convenience-API results.

## User Scenarios & Testing

### Scenario 1: Quote a simple string with default behavior
A caller needs a quoted representation of a regular NUL-terminated string for user-facing output.

- Input: a string such as `abc`.
- Action: call the default quoting entry point equivalent to `quotearg` or `quote`.
- Expected result: a non-null quoted string is returned using the module’s default quoting behavior.

### Scenario 2: Quote arbitrary bytes with explicit length
A caller needs to quote data that may include embedded NUL bytes or may not be NUL-terminated.

- Input: a byte buffer and explicit length.
- Action: call the equivalent of `quotearg_mem` or `quote_mem`.
- Expected result: the quoted output reflects exactly the provided byte range, not any implicit string terminator.

### Scenario 3: Force quoting of a specific character
A caller wants one specific character to always be treated as needing quoting in the output.

- Input: a string and a character such as `:`, `' '`, or `\`.
- Action: call the equivalent of `quotearg_char`.
- Expected result: output differs from default quoting when the specified character occurs, reflecting that this character is additionally quoted.

### Scenario 4: Use locale-sensitive quotation marks
A caller depends on translated or style-specific opening and closing quotation marks.

- Input: a style requiring localized or style-driven quote delimiters.
- Action: use the quoting path that depends on `gettext_quote`.
- Expected result: the selected left/right quote strings match the locale/style rules implemented by the source module.

### Scenario 5: Write quoted output into a bounded destination
A caller needs quoted output generation that respects a provided output buffer size while still learning the full required size.

- Input: destination buffer, destination size, source bytes, quoting style, and options.
- Action: call the Rust equivalent of the core quoting buffer transformation.
- Expected result:
  - no write occurs beyond the destination limit,
  - the function returns the size corresponding to the full quoted result.

### Scenario 6: Reuse convenience APIs and then free storage
A caller uses the convenience APIs multiple times and later releases module-owned storage.

- Action:
  1. call convenience quoting functions repeatedly,
  2. call the Rust equivalent of `quotearg_free`.
- Expected result:
  - quoted results are available during use as defined by the module,
  - internal storage held for convenience results is released when freed.

### Testing guidance

The Rust version must be tested with at least:

- simple ASCII strings,
- empty strings,
- explicit-length buffers,
- buffers containing embedded NUL,
- inputs containing characters that trigger extra quoting,
- multiple quoting styles evidenced by the source module’s option structure,
- localized-quote selection paths,
- small output buffers, including zero-length buffers,
- repeated convenience calls followed by cleanup.

## Requirements

### Functional Requirements

- **FR-1**: The module shall provide locale/style-sensitive quote delimiter selection equivalent to `gettext_quote`.
  **Traceability:** `quotearg.c`, `gettext_quote`.

- **FR-2**: The module shall provide a core quoting operation equivalent to `quotearg_buffer_restyled` that transforms an input byte sequence into a quoted representation according to quoting style, flags, additional quoted-character selection, and optional explicit left/right quotes.
  **Traceability:** `quotearg.c`, `quotearg_buffer_restyled`.

- **FR-3**: The core quoting operation shall accept explicit input length and shall not require NUL termination of the input.
  **Traceability:** `quotearg.c`, `quotearg_buffer_restyled`, `quotearg_mem`, `quote_mem`.

- **FR-4**: The core quoting operation shall support bounded output-buffer writing while returning the total quoted size needed for the full result.
  **Traceability:** `quotearg.c`, `quotearg_buffer_restyled`.

- **FR-5**: The module shall provide a default convenience API for quoting a NUL-terminated string equivalent to `quotearg`.
  **Traceability:** `quotearg.c`, `quotearg`.

- **FR-6**: The module shall provide a convenience API for quoting an explicit-length memory region equivalent to `quotearg_mem`.
  **Traceability:** `quotearg.c`, `quotearg_mem`.

- **FR-7**: The module shall provide a convenience API for quoting a string while forcing one specified character to be additionally quoted, equivalent to `quotearg_char`.
  **Traceability:** `quotearg.c`, `quotearg_char`.

- **FR-8**: The module shall provide quote-oriented wrapper APIs equivalent to `quote_mem` and `quote`.
  **Traceability:** `quotearg.c`, `quote_mem`, `quote`.

- **FR-9**: The convenience APIs shall use module-managed result storage whose lifecycle includes explicit release through behavior equivalent to `quotearg_free`.
  **Traceability:** `quotearg.c`, `quotearg_free`, `slotvec`.

- **FR-10**: The Rust rewrite shall preserve observable quoting behavior driven by quoting options represented by the source module’s `struct quoting_options`.
  **Traceability:** `quotearg.c`, `struct quoting_options`, `quotearg_buffer_restyled`.

### Key Entities

- **Quoting options**
  - Represent the active quoting configuration for a quoting operation.
  - Govern quoting style, flags, and character-specific quoting behavior.
  - Are consumed by the core quoting logic and by convenience wrappers that derive specialized behavior.
  - **Traceability:** `quotearg.c`, `struct quoting_options`, `quotearg_buffer_restyled`, wrappers creating local `quoting_options`.

- **Slot vector**
  - Represents module-managed storage slots used by convenience APIs that return quoted strings without requiring caller-owned buffers.
  - Supports reuse across calls and cleanup through the free function.
  - **Traceability:** `quotearg.c`, `struct slotvec`, `quotearg_free`.

- **Quoted result**
  - The output string produced from an input string or byte sequence according to the selected quoting configuration.
  - May be written into a caller-provided buffer or stored in module-managed storage depending on the API used.
  - **Traceability:** `quotearg.c`, `quotearg_buffer_restyled`, `quotearg`, `quotearg_mem`, `quotearg_char`, `quote_mem`, `quote`.

## Success Criteria

- **SC-1**: For inputs and styles covered by the source module, the Rust implementation produces the same quoted text as the C module for the default string API, explicit-length API, character-forcing API, and quote wrappers.
  **Traceability:** `quotearg`, `quotearg_mem`, `quotearg_char`, `quote_mem`, `quote`.

- **SC-2**: For buffer-based quoting, the Rust implementation returns the full quoted size and never writes beyond the provided output capacity.
  **Traceability:** `quotearg_buffer_restyled`.

- **SC-3**: For explicit-length inputs containing embedded NUL bytes, the Rust implementation quotes the full specified byte range rather than stopping at the first NUL.
  **Traceability:** `quotearg_buffer_restyled`, `quotearg_mem`, `quote_mem`.

- **SC-4**: Locale/style-dependent quote delimiter selection matches the source module’s behavior for the same quoting style and translation context.
  **Traceability:** `gettext_quote`.

- **SC-5**: Repeated convenience quoting calls remain usable under the module’s managed-storage model, and invoking the cleanup function releases that storage without leaving stale module-owned allocations in normal test execution.
  **Traceability:** `quotearg_free`, `slotvec`, convenience wrapper functions.

- **SC-6**: Tests cover all user scenarios in this document and pass against the Rust port.
  **Traceability:** all listed functions and entities.