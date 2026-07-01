# spec.md

## Title

Functional Specification: `main_root_quotearg.c_32` Rust Port

## Status

Draft

## Module Overview

This module provides argument quoting services for user-visible strings. Its purpose is to transform input byte sequences into quoted representations suitable for display, diagnostics, and command-line style output, while applying selectable quoting styles, optional per-character quoting rules, locale-sensitive quote selection, and reusable storage for returned quoted strings.

The Rust rewrite must preserve the observable behavior of the C module implemented in `quotearg.c`, specifically the behavior evidenced by:

- `gettext_quote`
- `quotearg_buffer_restyled`
- `quotearg_free`
- `quotearg`
- `quotearg_mem`
- `quotearg_char`
- `quote_mem`
- `quote`

## Feature Specification

### Summary

The module shall accept an input string or arbitrary byte sequence and return a quoted form according to configured or wrapper-selected quoting behavior. It shall support:

- quoting of NUL-terminated strings and explicit-length byte sequences,
- style-dependent surrounding quotes and escaping behavior,
- locale-sensitive selection of quote glyphs where applicable,
- forcing additional characters to be quoted,
- convenience entry points for common quoting cases,
- cleanup of reusable internal quoted-string storage.

### Supported Functional Behavior

1. **Locale-sensitive quote selection**
   - The module shall provide quote delimiters chosen from a message identifier and quoting style context.
   - Where the quoting style requires translated or locale-aware quotation marks, the chosen left/right quote strings shall depend on the current translation result used by `gettext_quote`.

2. **Buffer-based quoting engine**
   - The module shall provide a core operation that writes a quoted representation into a caller-provided buffer and computes the total quoted length.
   - The quoting operation shall be driven by:
     - source bytes and source length,
     - selected quoting style,
     - flags affecting quoting behavior,
     - an optional set of extra characters that must be quoted,
     - optional explicit left/right quote strings.
   - The core operation shall support inputs containing arbitrary bytes, not just text up to the first NUL.

3. **Convenience quoting for default behavior**
   - The module shall provide a default quoting entry point for NUL-terminated strings through `quotearg`.
   - The module shall provide a default quoting entry point for explicit-length data through `quotearg_mem`.

4. **Character-forcing quoting variant**
   - The module shall provide a variant that ensures a specified character is quoted in the output through `quotearg_char`.

5. **Alternate “quote” wrappers**
   - The module shall provide `quote_mem` and `quote` wrappers that return quoted output using the module’s quote-oriented convenience behavior for explicit-length and NUL-terminated inputs respectively.

6. **Reusable returned-string storage**
   - The module shall support APIs that return pointers to internally managed quoted strings rather than requiring the caller to supply an output buffer.
   - Repeated quoting calls through these wrappers shall remain usable by allocating or reusing internal slots as needed.

7. **Explicit cleanup**
   - The module shall provide `quotearg_free` to release internal storage associated with reusable returned quoted strings.

## User Scenarios & Testing

### Scenario 1: Quote a simple argument for display

A caller has a standard C-style string and needs a quoted version for display in diagnostics.

- Input: a NUL-terminated string.
- Operation: call the Rust equivalent of `quotearg`.
- Expected result: a quoted string is returned using the module’s default quoting behavior.

**Testing focus**
- Returned output is quoted.
- Output is stable for the duration expected by the wrapper-based API.
- The produced text matches the C module’s behavior for representative inputs.

### Scenario 2: Quote binary or substring data with an explicit length

A caller needs to quote data that may include embedded NUL bytes or is not NUL-terminated.

- Input: byte sequence plus explicit length.
- Operation: call the Rust equivalent of `quotearg_mem` or `quote_mem`.
- Expected result: the full specified byte range is quoted, with no truncation at embedded NUL.

**Testing focus**
- Embedded NUL does not terminate processing early.
- Reported/returned quoted output reflects exactly the provided length.
- Buffer-based behavior and wrapper behavior agree on output content.

### Scenario 3: Force a particular character to be quoted

A caller needs a quoted rendering where one specific character receives quoting treatment even if it would not under default rules.

- Input: a string and a target character.
- Operation: call the Rust equivalent of `quotearg_char`.
- Expected result: occurrences of the specified character are quoted according to the module’s quoting rules.

**Testing focus**
- The specified character is treated as additionally quotable.
- Characters not targeted continue to follow the normal selected behavior.
- Output matches the C module for strings containing zero, one, or multiple instances of the target character.

### Scenario 4: Use the core buffer-based quoting engine

A caller manages its own output buffer and needs the quoted representation written there, while also learning the full required output length.

- Input: buffer, buffer size, source bytes, source length, style/options.
- Operation: call the Rust equivalent of the core quoting routine represented by `quotearg_buffer_restyled`.
- Expected result: the buffer receives as much of the correctly quoted output as fits, and the operation computes the full quoted size.

**Testing focus**
- Correct behavior with sufficiently large buffers.
- Correct length computation with zero-length or undersized buffers.
- Output is consistent with wrapper APIs for the same effective options.

### Scenario 5: Locale-sensitive quote delimiters

A caller runs in an environment where translated quotation marks differ from plain ASCII quotes.

- Input: a string and a style that uses translated or locale-sensitive quote delimiters.
- Operation: quote the string using behavior that depends on `gettext_quote`.
- Expected result: delimiters are selected in accordance with the translation-sensitive quote lookup behavior.

**Testing focus**
- Delimiter selection changes when translation results differ.
- Left/right quote handling is consistent with the C implementation for the tested style.
- Styles not using translated delimiters remain unaffected.

### Scenario 6: Free reusable quote storage

A caller has used wrapper APIs returning internally managed quoted strings and wants to release associated storage.

- Operation: call the Rust equivalent of `quotearg_free`.
- Expected result: reusable storage maintained by the module is released, and future quoting calls remain functional.

**Testing focus**
- Cleanup succeeds after one or many wrapper calls.
- Subsequent wrapper calls still produce correct output after cleanup.
- No stale prior output is required to remain valid after cleanup.

## Requirements

### Functional Requirements

#### FR-1: Quoted rendering of input data
The module shall convert input data into a quoted textual representation, as evidenced by `quotearg_buffer_restyled` and the wrapper functions `quotearg`, `quotearg_mem`, `quotearg_char`, `quote_mem`, and `quote`.

#### FR-2: Support both NUL-terminated and explicit-length inputs
The module shall support quoting both ordinary strings and arbitrary byte sequences with explicit lengths, as evidenced by the distinction between `quotearg`/`quote` and `quotearg_mem`/`quote_mem`, and by the `arg` plus `argsize` parameters of `quotearg_buffer_restyled`.

#### FR-3: Style-driven quoting behavior
The module shall vary its quoting behavior according to a quoting style input, as evidenced by the `enum quoting_style` parameter in `gettext_quote` and `quotearg_buffer_restyled`, and by the presence of `struct quoting_options`.

#### FR-4: Optional extra quoted characters
The module shall support marking additional characters as requiring quoting, as evidenced by the `quote_these_too` parameter of `quotearg_buffer_restyled` and the specialized wrapper `quotearg_char`.

#### FR-5: Configurable quote delimiters
The module shall support left and right quote delimiters supplied directly or selected through quote-style lookup, as evidenced by the `left_quote` and corresponding right-quote parameters in `quotearg_buffer_restyled` and the delimiter selection performed by `gettext_quote`.

#### FR-6: Locale-sensitive quote lookup
The module shall select locale-sensitive quote strings where the quoting style requires it, as evidenced by `gettext_quote`.

#### FR-7: Buffer-oriented operation with full-size computation
The module shall support writing quoted output into a caller-provided buffer while determining the total resulting size, as evidenced by the signature and role of `quotearg_buffer_restyled`.

#### FR-8: Wrapper APIs returning internally managed storage
The module shall provide convenience APIs that return pointers or references to internally managed quoted strings, as evidenced by `quotearg`, `quotearg_mem`, `quotearg_char`, `quote_mem`, and `quote`, together with the internal slot storage represented by `slotvec`.

#### FR-9: Reusable storage cleanup
The module shall provide a cleanup operation that releases internal storage associated with wrapper-returned quoted strings, as evidenced by `quotearg_free` and the `slotvec` storage management.

### Key Entities

#### `struct quoting_options`
Represents the option set controlling how quoting is performed. Based on its repeated use around style-specific and wrapper-specific operations, this entity defines the effective quoting mode used by the quoting engine and convenience wrappers.

**Relationship**
- Consumed by quoting operations to determine style and behavior.
- Specialized or copied by wrapper functions to create common quoting configurations.

#### `enum quoting_style`
Represents the quoting mode used to decide how input bytes are surrounded and escaped.

**Relationship**
- Used by `gettext_quote` to determine delimiter selection.
- Used by the core quoting engine to determine output form.
- Encapsulated within or associated with `quoting_options`.

#### `slotvec`
Represents internal reusable storage slots for APIs that return quoted strings without a caller-supplied destination buffer.

**Relationship**
- Managed by wrapper APIs that return internally stored quoted output.
- Released by `quotearg_free`.

## Success Criteria

### Behavioral Equivalence

1. For representative printable text inputs, the Rust port produces the same quoted output as the C module for:
   - `quotearg`
   - `quotearg_mem`
   - `quotearg_char`
   - `quote_mem`
   - `quote`

2. For representative explicit-length inputs containing embedded NUL and non-printable bytes, the Rust port processes the full specified length and matches the C module’s quoted result.

3. For representative styles requiring translated or locale-sensitive quote delimiters, the Rust port selects the same delimiters as the C module under the same translation environment.

4. For representative uses of extra-character quoting, including the single-character case of `quotearg_char`, the Rust port quotes the targeted character(s) in the same places as the C module.

### Buffer Semantics

5. The Rust equivalent of the core buffer-based quoting operation returns the full quoted size for a given input and option set, regardless of whether the provided output buffer is large enough to hold the complete result.

6. When the output buffer is sufficiently large, the Rust core operation writes quoted content identical to the content returned by the wrapper-based APIs for the same effective configuration.

### Storage Lifecycle

7. Repeated calls to wrapper APIs returning internally managed quoted strings remain functional across multiple invocations without requiring caller-managed output buffers.

8. After calling the Rust equivalent of `quotearg_free`, internal reusable quote storage is released, and subsequent wrapper calls still succeed and produce correct quoted output.

### Traceability

9. Every implemented behavior in the Rust module is traceable to functionality evidenced in `quotearg.c`, specifically through `gettext_quote`, `quotearg_buffer_restyled`, `quotearg_free`, the wrapper quoting functions, `struct quoting_options`, and `slotvec`, with no additional public capabilities introduced beyond those evidenced here.