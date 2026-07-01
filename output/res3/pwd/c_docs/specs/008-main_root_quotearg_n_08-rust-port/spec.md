# spec.md

## Overview

This specification defines the Rust rewrite scope for module **main_root_quotearg_n_08** from project **pwd**, based on functionality evidenced in `quotearg.c`.

The module provides slot-based argument quoting helpers centered on these entry points:

- `quotearg_n`
- `quotearg_n_mem`
- `quotearg_n_custom`

Its role is to return quoted representations of input arguments, with support for:

- selecting a caller-visible slot by numeric index,
- quoting either NUL-terminated text or byte sequences with explicit length,
- applying default quoting behavior or caller-supplied custom quote delimiters.

The Rust version must preserve the functional behavior exposed by these entry points and the data relationships implied by `struct quoting_options` and `struct slotvec`.

---

## Feature Specification

### Summary

The module formats input arguments into quoted output strings. It supports repeated calls using numbered slots so that multiple independently retained quoted results can coexist at once. It also supports quoting byte slices with explicit length and quoting with custom left/right delimiters.

### In Scope

The Rust rewrite must implement:

1. **Default slot-based quoting for string arguments**
   - Accept a slot number and a NUL-terminated argument equivalent.
   - Return the quoted representation associated with that slot.

2. **Slot-based quoting for explicit-length input**
   - Accept a slot number, an input pointer/reference, and an explicit byte length.
   - Quote exactly the specified number of bytes, without relying on a terminating NUL.

3. **Custom-delimiter quoting**
   - Accept a slot number, custom left quote text, custom right quote text, and an argument.
   - Return a quoted representation using those provided delimiters.

4. **Use of quoting options**
   - Behavior is driven by quoting options data represented by `struct quoting_options`.
   - The custom-delimiter entry point must apply custom quoting behavior through such options.

5. **Per-slot result management**
   - Results are managed by slot index via `struct slotvec`.
   - Reusing the same slot updates that slot’s stored quoted result.
   - Different slot numbers must allow distinct results to remain independently accessible across calls.

### Out of Scope

The Rust rewrite specification does not require any capabilities not evidenced by the listed functions and data structures, including any new public APIs, thread-safety guarantees, serialization behavior, or foreign-function interfaces.

---

## User Scenarios & Testing

### Scenario 1: Quote a standard argument using the default behavior

A caller has a text argument and needs a quoted printable form using the module’s default quoting behavior.

- Call the Rust equivalent of `quotearg_n` with slot `0` and the argument.
- Receive a quoted string result for that slot.

**Expected outcome**
- The result is a quoted form of the provided argument.
- The same slot can be used again later for another argument.

### Scenario 2: Keep multiple quoted results alive by using different slots

A caller needs two quoted arguments at the same time for later formatting or reporting.

- Call the Rust equivalent of `quotearg_n` with slot `0` for the first argument.
- Call it again with slot `1` for the second argument.

**Expected outcome**
- Slot `0` and slot `1` each produce and retain their own quoted result.
- The second call does not overwrite the stored result associated with the other slot.

### Scenario 3: Reuse a slot for a new argument

A caller no longer needs the earlier quoted content in a given slot and wants to quote a new argument into that same slot.

- Call the Rust equivalent of `quotearg_n` or `quotearg_n_mem` with a slot previously used.
- Read back the returned quoted result for the new input.

**Expected outcome**
- The slot now represents the new quoted argument.
- The previous content for that same slot is replaced.

### Scenario 4: Quote a byte sequence containing embedded NUL or non-text data

A caller has input that cannot be represented safely as a NUL-terminated string and must quote exactly a specified number of bytes.

- Call the Rust equivalent of `quotearg_n_mem` with a slot, a byte sequence, and its exact length.

**Expected outcome**
- The quoted result is derived from exactly the provided byte count.
- Embedded NUL bytes do not truncate processing.

### Scenario 5: Quote using custom left and right delimiters

A caller needs argument text surrounded by caller-specified quote markers instead of the default style.

- Call the Rust equivalent of `quotearg_n_custom` with:
  - a slot,
  - `left_quote`,
  - `right_quote`,
  - the argument text.

**Expected outcome**
- The returned result uses the provided left and right quote strings.
- The customization applies to that quoting operation as driven by quoting options.

### Scenario 6: Distinguish custom quoting from default quoting

A caller compares output from default quoting and custom quoting for the same argument.

- Quote the argument with the Rust equivalent of `quotearg_n`.
- Quote the same argument with the Rust equivalent of `quotearg_n_custom`.

**Expected outcome**
- The custom call reflects the provided delimiters.
- The default call reflects the module’s normal quoting behavior.
- The two outputs differ when the custom delimiters differ from the defaults.

### Testing Notes

The Rust version must be testable with cases that cover:

- slot `0` and at least one additional slot,
- repeated calls to the same slot,
- distinct results in different slots,
- explicit-length input containing embedded NUL,
- empty input,
- custom left/right quote strings,
- comparison between default and custom quoting paths.

---

## Requirements

### Functional Requirements

#### FR-1: Default quoted output by slot
The module shall provide a functionally equivalent operation to `quotearg_n` that accepts a numeric slot and an argument string and returns that argument’s quoted representation for the specified slot.

**Traceability:** `quotearg.c`, `quotearg_n`

#### FR-2: Explicit-length quoted output by slot
The module shall provide a functionally equivalent operation to `quotearg_n_mem` that accepts a numeric slot, input data, and an explicit byte length, and shall quote exactly that byte range.

**Traceability:** `quotearg.c`, `quotearg_n_mem`

#### FR-3: Custom-delimiter quoted output by slot
The module shall provide a functionally equivalent operation to `quotearg_n_custom` that accepts a numeric slot, a left quote string, a right quote string, and an argument string, and returns a quoted representation using those delimiters.

**Traceability:** `quotearg.c`, `quotearg_n_custom`

#### FR-4: Slot isolation
The module shall maintain quoted results by slot such that quoting into one slot does not replace the currently stored result for a different slot.

**Traceability:** `quotearg.c`, `struct slotvec`; use by `quotearg_n`, `quotearg_n_mem`, `quotearg_n_custom`

#### FR-5: Slot replacement on reuse
When a quoting operation is performed for a slot that has already been used, the module shall update that slot’s stored quoted result to correspond to the most recent input for that slot.

**Traceability:** `quotearg.c`, `struct slotvec`; use by `quotearg_n`, `quotearg_n_mem`, `quotearg_n_custom`

#### FR-6: Quoting behavior driven by quoting options
The module shall represent quoting behavior using a quoting-options entity equivalent in purpose to `struct quoting_options`, and custom quoting shall be expressed through that options model.

**Traceability:** `quotearg.c`, `struct quoting_options`; `quotearg_n_custom`

#### FR-7: Default and custom quoting paths remain distinct
The module shall preserve the behavioral distinction between default quoting operations and custom-delimiter quoting operations.

**Traceability:** `quotearg.c`, `quotearg_n`, `quotearg_n_custom`, `struct quoting_options`

### Key Entities

#### `quoting_options`
A configuration entity that represents how quoting should be performed.

**Role**
- Encapsulates quoting behavior used by quoting operations.
- Supports the custom quoting path used by the custom-delimiter entry point.

**Relationships**
- Used by quoting functions to determine output form.
- Custom left/right quote handling is expressed through this entity.

**Traceability:** `quotearg.c`, `struct quoting_options`; `quotearg_n_custom`

#### `slotvec`
A slot-management entity that associates slot numbers with stored quoted results.

**Role**
- Tracks per-slot storage for quoted output.
- Enables independent retention and replacement of quoted results by slot index.

**Relationships**
- Referenced by slot-based quoting operations.
- Works with quoting output generated under selected quoting options.

**Traceability:** `quotearg.c`, `struct slotvec`; `quotearg_n`, `quotearg_n_mem`, `quotearg_n_custom`

---

## Success Criteria

### Functional Correctness

1. **Default quoting support**
   - The Rust module provides a callable equivalent to `quotearg_n` that returns quoted output for string input by slot.
   - Verified by tests that quote at least one non-empty input string.

2. **Explicit-length input support**
   - The Rust module provides a callable equivalent to `quotearg_n_mem` that processes exactly the specified byte length.
   - Verified by tests with embedded NUL bytes showing that processing is not truncated at the first NUL.

3. **Custom delimiter support**
   - The Rust module provides a callable equivalent to `quotearg_n_custom` that applies caller-provided left and right quote strings.
   - Verified by tests where output includes the specified delimiters.

4. **Independent slot retention**
   - Quoting into slot `n` does not alter the stored result previously produced for a different slot `m`.
   - Verified by tests using at least two distinct slots and comparing both outputs after both calls.

5. **Slot overwrite behavior**
   - Reusing the same slot replaces that slot’s prior quoted result with one derived from the new input.
   - Verified by tests that quote two different inputs into the same slot sequentially and confirm the second result supersedes the first for that slot.

6. **Distinct default vs custom behavior**
   - For the same input, default quoting and custom-delimiter quoting produce distinguishable results when custom delimiters differ from default behavior.
   - Verified by comparative tests using one default call and one custom-delimiter call.

### Scope Compliance

7. **No unevidenced API expansion**
   - The Rust rewrite exposes only functionality necessary to preserve the evidenced behavior of `quotearg_n`, `quotearg_n_mem`, `quotearg_n_custom`, and the supporting roles of `quoting_options` and `slotvec`.

8. **Traceable implementation scope**
   - Every implemented externally relevant behavior in the Rust rewrite can be traced to the listed functions or data structures from `quotearg.c`.