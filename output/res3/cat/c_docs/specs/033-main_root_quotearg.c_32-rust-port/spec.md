# spec.md

## Title
Functional Specification for `main_root_quotearg.c_32` Rust Port

## Metadata
- Project: `cat`
- Module: `main_root_quotearg.c_32`
- Category: `main_cluster`
- Source file: `quotearg.c`
- Rust branch: `033-main_root_quotearg.c_32-rust-port`
- Generation date: `2026-06-09`

## Overview
This module provides argument quoting services for text intended for display or diagnostics. Its purpose is to transform arbitrary input strings or byte sequences into quoted representations according to configured quoting rules, including style selection, selective character quoting, and locale-sensitive quote delimiter choice.

The Rust rewrite must preserve the module’s observable quoting behavior exposed by the identified entry points in `quotearg.c`, including:
- producing quoted strings for NUL-terminated text and explicit-length byte sequences,
- supporting style-dependent output formatting,
- honoring option-driven quoting adjustments,
- selecting localized quotation marks where required,
- maintaining reusable storage for convenience-returning APIs,
- releasing internally retained storage when requested.

## Feature Specification

### 1. Quoted rendering of input data
The module accepts either:
- a conventional C string, or
- a memory region paired with an explicit length,

and returns a quoted representation suitable for use by callers that need a safely rendered textual form.

The Rust version must support both forms of input because both are directly represented by the module’s public wrappers:
- `quotearg`
- `quotearg_mem`
- `quotearg_char`
- `quote`
- `quote_mem`

### 2. Style-based quoting behavior
The module supports multiple quoting styles through `struct quoting_options` and the `quotearg_buffer_restyled` engine. The Rust version must preserve style-directed output differences that affect:
- which quote delimiters are used,
- whether and how special bytes are escaped,
- whether the output is wrapped in quotation marks,
- whether locale-dependent quote strings are selected.

This requirement is traceable to:
- `gettext_quote`
- `quotearg_buffer_restyled`
- `struct quoting_options`

### 3. Locale-sensitive quote delimiter selection
The module includes a dedicated helper, `gettext_quote`, that chooses quote strings based on message identifiers and quoting style. The Rust version must preserve the behavior that some quoting styles obtain their left/right quotation markers through this helper rather than treating all delimiters as fixed literals.

This requirement is limited to delimiter selection behavior evidenced by `gettext_quote`; it does not imply any additional internationalization features beyond quote-string selection.

### 4. Option-driven customization
The quoting engine operates from a `quoting_options` configuration object. The Rust version must preserve the ability, where evidenced by the current module behavior, to vary output according to:
- quoting style,
- flags,
- a set of characters that must be quoted in addition to style defaults,
- explicitly supplied left and right quote strings.

This is evidenced by the parameter set of `quotearg_buffer_restyled` and repeated local use of `struct quoting_options` in wrapper functions.

### 5. Target-buffer rendering with size reporting
The core engine function renders into a caller-supplied buffer and returns the size of the quoted result. The Rust version must preserve the behavior that the quoting operation has a form that:
- writes into a destination buffer abstraction,
- is constrained by a supplied output capacity,
- computes the resulting quoted length independent of whether the destination can hold the full result.

This requirement is traceable to `quotearg_buffer_restyled`.

### 6. Convenience APIs with retained result storage
Several wrapper functions return pointer-like access to internally retained quoted strings rather than requiring the caller to pass a destination buffer. The Rust version must preserve the observable semantics of these convenience forms:
- repeated calls can obtain quoted results without caller-managed storage,
- internal reusable storage exists for these results,
- `quotearg_free` releases that retained storage.

This requirement is traceable to:
- `quotearg`
- `quotearg_mem`
- `quotearg_char`
- `quote`
- `quote_mem`
- `quotearg_free`
- `struct slotvec`

### 7. Quoting of additional selected characters
The module provides a convenience entry point that ensures a specific character is quoted in the rendered output. The Rust version must preserve this behavior for `quotearg_char`, using the same underlying quoting rules except for the added requirement that the selected character be treated as requiring quoting.

This requirement is traceable to:
- `quotearg_char`
- `quotearg_buffer_restyled`
- `struct quoting_options`

### 8. Separate “quote” convenience behavior
The module exposes both `quotearg*` and `quote*` families. The Rust version must preserve the distinction evidenced by separate wrapper functions and options instances in the source, so that:
- `quote`
- `quote_mem`

continue to use the module’s quote-oriented default behavior rather than being collapsed into unrelated defaults.

This requirement is traceable to:
- distinct `struct quoting_options` instances referenced in the source around these wrappers.

## User Scenarios & Testing

### Scenario 1: Quote a simple command-line argument
A caller has a standard text argument and needs a quoted representation for diagnostic output.

- Input: a NUL-terminated string.
- API path: `quotearg` or `quote`.
- Expected behavior: returns a quoted textual representation according to the wrapper’s default options.
- Test focus: output is non-empty when quoting requires delimiters or escapes; plain input content is preserved except for quoting transformations.

### Scenario 2: Quote binary or substring data with explicit length
A caller needs to quote bytes that may contain embedded NUL or are not terminated.

- Input: byte region plus explicit size.
- API path: `quotearg_mem` or `quote_mem`.
- Expected behavior: quoting processes exactly the specified byte count, not stopping at the first NUL.
- Test focus: embedded NUL does not truncate processing; returned rendered size and content reflect all input bytes.

### Scenario 3: Force quoting of an otherwise ordinary character
A caller wants a specific character to be specially quoted in the output even if the default style would leave it unchanged.

- Input: text plus a designated character.
- API path: `quotearg_char`.
- Expected behavior: output reflects the normal quoting style plus the additional rule that the designated character is quoted.
- Test focus: compare output with and without the extra character requirement.

### Scenario 4: Use locale-sensitive quote marks
A caller relies on a style that chooses quote delimiters via `gettext_quote`.

- Input: text under a style that uses localized delimiters.
- API path: internal style flow through `quotearg_buffer_restyled`.
- Expected behavior: left/right quotation marks come from the style-sensitive quote-selection helper rather than from a single hard-coded delimiter pair.
- Test focus: delimiter selection varies according to the requested quote message/style combination as in the C module.

### Scenario 5: Render into bounded storage
A caller needs quoting into a fixed-size destination.

- Input: destination buffer with limited size, source text or bytes, and quoting options.
- API path: `quotearg_buffer_restyled`.
- Expected behavior: the function reports the full quoted size while respecting the provided output capacity.
- Test focus: small output buffer does not cause incorrect length computation; sufficiently large buffer contains the complete rendered output.

### Scenario 6: Reuse convenience-returned storage
A caller uses the convenience APIs multiple times and later discards internal storage.

- Input: repeated calls to convenience wrappers, then `quotearg_free`.
- API path: `quotearg`, `quotearg_mem`, `quote`, `quote_mem`, `quotearg_free`.
- Expected behavior: quoted results remain available through the wrapper contract until overwritten by later internal reuse or released by the module’s cleanup function.
- Test focus: cleanup succeeds after prior quoting operations and the module remains usable again after cleanup if called subsequently.

## Requirements

### Functional Requirements

#### FR-1: Input forms
The module shall accept both NUL-terminated strings and explicit-length memory regions for quoting operations.
- Traceability: `quotearg`, `quotearg_mem`, `quote`, `quote_mem`, `quotearg_buffer_restyled`.

#### FR-2: Core quoting transformation
The module shall convert input data into a quoted textual representation according to a selected quoting style and option set.
- Traceability: `quotearg_buffer_restyled`, `struct quoting_options`.

#### FR-3: Style-sensitive delimiters
The module shall support quoting styles that influence the left and right quotation delimiters used in output.
- Traceability: `gettext_quote`, `quotearg_buffer_restyled`, `struct quoting_options`.

#### FR-4: Locale-sensitive quote lookup
Where the source behavior uses `gettext_quote`, the module shall obtain quote delimiters through that style-aware helper behavior rather than substituting a single fixed delimiter pair for all styles.
- Traceability: `gettext_quote`.

#### FR-5: Additional forced-quote characters
The module shall support an option-controlled set of characters that are quoted in addition to characters required by the selected style.
- Traceability: `quotearg_buffer_restyled`, `quotearg_char`, `struct quoting_options`.

#### FR-6: Flag-controlled behavior
The module shall honor quoting flags carried in the quoting options / quoting engine call path when producing output.
- Traceability: `quotearg_buffer_restyled`, `struct quoting_options`.

#### FR-7: Explicit custom delimiters
The module shall support operation with explicitly supplied left and right quote strings when provided through the quoting engine interface.
- Traceability: `quotearg_buffer_restyled`.

#### FR-8: Bounded destination rendering
The module shall support rendering into caller-provided storage bounded by a supplied output size.
- Traceability: `quotearg_buffer_restyled`.

#### FR-9: Result length reporting
The module shall report the size of the quoted result from the core buffer-rendering path, including when the destination storage is not large enough to hold the complete output.
- Traceability: `quotearg_buffer_restyled`.

#### FR-10: Convenience wrapper behavior
The module shall provide convenience quoting entry points that apply predefined option sets for common quoting use.
- Traceability: `quotearg`, `quotearg_mem`, `quotearg_char`, `quote`, `quote_mem`, `struct quoting_options`.

#### FR-11: Internally retained result storage
The convenience wrappers shall use module-managed retained storage for returned quoted results.
- Traceability: wrapper functions above, `struct slotvec`.

#### FR-12: Cleanup of retained storage
The module shall provide a cleanup operation that releases the module-managed retained storage used by convenience wrappers.
- Traceability: `quotearg_free`, `struct slotvec`.

#### FR-13: Continued wrapper distinction
The module shall preserve the distinct default behaviors represented by the `quotearg*` and `quote*` wrappers where the source defines separate option objects for them.
- Traceability: `quote`, `quote_mem`, `quotearg`, `quotearg_mem`, source-local `struct quoting_options` instances.

### Key Entities

#### `quoting_options`
Configuration entity that defines how quoting is performed. Based on its use in the module, it governs:
- selected quoting style,
- flags affecting rendering behavior,
- an optional set of extra characters to quote,
- quote delimiter selection, whether fixed or style-derived.

Relationship:
- consumed by the core quoting engine,
- specialized into wrapper-specific default configurations.

Traceability:
- repeated `struct quoting_options` definitions/usages in `quotearg.c`
- `quotearg_buffer_restyled`

#### `slotvec`
Internal retained-storage entity used to keep quoted results for convenience-returning APIs.

Relationship:
- allocated and reused by wrapper functions returning stored quoted text,
- released by `quotearg_free`.

Traceability:
- `struct slotvec`
- `quotearg_free`
- convenience wrappers in `quotearg.c`

#### Quoted result buffer
Destination storage used by the core rendering path to materialize quoted output.

Relationship:
- written by `quotearg_buffer_restyled`,
- may be caller-provided or module-managed through wrapper storage.

Traceability:
- `quotearg_buffer_restyled`
- wrapper functions using retained storage

#### Quote delimiter pair
Logical left/right quote strings used to surround or format quoted output.

Relationship:
- may be selected by style through `gettext_quote`,
- may be explicitly supplied to the core engine.

Traceability:
- `gettext_quote`
- `quotearg_buffer_restyled`

## Success Criteria

### Behavioral correctness
1. For every input accepted by the C module’s listed entry points, the Rust port produces quoted output matching the C module’s observable behavior for the same style/options path.
   - Traceability: all listed functions, especially `quotearg_buffer_restyled`.

2. Explicit-length quoting in the Rust port processes exactly the provided byte count, including embedded NUL bytes.
   - Traceability: `quotearg_mem`, `quote_mem`, `quotearg_buffer_restyled`.

3. The Rust implementation preserves wrapper-specific defaults so that `quotearg*` and `quote*` continue to exhibit their distinct source-defined behaviors.
   - Traceability: `quotearg`, `quotearg_mem`, `quote`, `quote_mem`, local `quoting_options` instances.

4. The Rust implementation preserves the behavior of forcing quotation of a selected character through the `quotearg_char` path.
   - Traceability: `quotearg_char`, `quotearg_buffer_restyled`.

5. For styles using localized quote selection, the Rust port preserves `gettext_quote`-driven left/right delimiter choice.
   - Traceability: `gettext_quote`.

### Buffer and storage semantics
6. The core rendering path in Rust reports the full quoted result length even when the provided destination capacity is smaller than the full output.
   - Traceability: `quotearg_buffer_restyled`.

7. The core rendering path respects the provided output size limit and does not require unbounded caller storage.

8. Convenience APIs in Rust provide module-managed retained results consistent with the source module’s wrapper model.
   - Traceability: `quotearg`, `quotearg_mem`, `quote`, `quote_mem`, `struct slotvec`.

9. Calling the Rust equivalent of `quotearg_free` releases retained convenience-storage state without removing the module’s ability to be used again afterward.
   - Traceability: `quotearg_free`, `struct slotvec`.

### Test completeness
10. The Rust port includes tests covering:
   - simple string quoting,
   - explicit-length byte quoting with embedded NUL,
   - forced quoting of an additional character,
   - locale/style-sensitive delimiter selection behavior,
   - bounded-buffer length reporting,
   - retained-storage cleanup behavior.
   - Traceability: `gettext_quote`, `quotearg_buffer_restyled`, `quotearg_free`, wrapper functions.