# spec.md

## Title

Functional Specification: `main_root_quotearg_n_07`

## Status

Draft

## Scope

This specification covers the Rust rewrite of the `main_root_quotearg_n_07` module from `quotearg.c`. The covered public behavior is limited to the slot-indexed quoting entry points:

- `quotearg_n`
- `quotearg_n_mem`
- `quotearg_n_custom`

It also covers the observable use of module state and option objects required by those entry points, as evidenced by the referenced `struct quoting_options` and `struct slotvec` usages.

## Feature Specification

### Summary

This module provides quoting services that return a quoted representation of input text through indexed result slots.

The Rust version must implement behavior equivalent to the C module for:

- quoting a NUL-terminated string using a slot number,
- quoting a byte sequence with explicit length using a slot number,
- quoting a string using caller-specified left and right quote delimiters using a slot number.

### Supported behavior

1. The module accepts a caller-provided slot index `n` and produces a quoted result associated with that slot.
2. The module supports both:
   - string input terminated in the usual C style, and
   - memory input supplied with an explicit byte length.
3. The module supports quoting with default/module-defined options as well as quoting with custom quote delimiters.
4. The module returns quoted text as character data suitable for immediate caller use.
5. Repeated calls using slot-based APIs must preserve the slot-oriented behavior implied by `slotvec`: each slot identifies a reusable storage location for returned quoted text.
6. Custom quoting must use the caller-provided `left_quote` and `right_quote` values for the produced representation.

### Out of scope

The Rust rewrite is not required by this specification to provide any behavior beyond the listed entry points and the option/slot behavior they depend on. No new APIs or capabilities are included.

## User Scenarios & Testing

### Scenario 1: Quote a normal argument with default behavior

A caller has a string argument and needs a quoted representation for diagnostics or display.

- Input: slot index `n`, string `arg`
- Operation: call equivalent of `quotearg_n(n, arg)`
- Expected result: returns quoted character data for `arg`, using the module’s default quoting behavior for this entry point.

#### Test coverage
- Quote a non-empty ASCII string.
- Quote an empty string.
- Quote the same input twice in the same slot and verify the returned content remains correct.
- Quote different inputs in the same slot sequentially and verify each returned content matches the most recent call.

### Scenario 2: Quote a byte sequence containing embedded NUL

A caller needs to quote data that cannot be represented as a simple C string because its logical length is explicit.

- Input: slot index `n`, byte buffer `arg`, byte count `argsize`
- Operation: call equivalent of `quotearg_n_mem(n, arg, argsize)`
- Expected result: returns quoted character data derived from exactly `argsize` bytes, without relying on a terminating NUL.

#### Test coverage
- Quote a buffer whose first byte is NUL with non-zero length.
- Quote a buffer containing embedded NUL bytes.
- Quote a zero-length buffer.
- Verify that changing bytes after `argsize` does not affect the result.

### Scenario 3: Quote using caller-specified delimiters

A caller needs a quoted representation wrapped with specific left and right quote strings.

- Input: slot index `n`, `left_quote`, `right_quote`, string `arg`
- Operation: call equivalent of `quotearg_n_custom(n, left_quote, right_quote, arg)`
- Expected result: returns quoted character data using the supplied quote delimiters.

#### Test coverage
- Use simple delimiters such as `[` and `]`.
- Use multi-character delimiters.
- Verify the output reflects the supplied left and right quote values.
- Quote different arguments with the same custom delimiters in the same slot.

### Scenario 4: Use multiple slots independently

A caller needs more than one quoted result to remain available at once.

- Input: two or more slot indices with distinct arguments
- Operation: quote each argument into a different slot
- Expected result: each slot yields the quoted content associated with its own most recent call, without being overwritten merely because another slot was used.

#### Test coverage
- Quote one string into slot 0 and another into slot 1.
- Verify slot 0 content still corresponds to its own argument after slot 1 is used.
- Reuse slot 1 and verify slot 0 remains unaffected.

## Requirements

### Functional Requirements

#### FR-1: Slot-indexed quoting for string input
The module shall provide a slot-indexed operation equivalent to `quotearg_n(int n, char const *arg)` that accepts a slot number and a NUL-terminated input string and returns quoted character data for that string.

**Traceability:** `quotearg.c:925-929` (`quotearg_n`), `struct slotvec` usages around `quotearg.c:829-878`.

#### FR-2: Slot-indexed quoting for explicit-length input
The module shall provide a slot-indexed operation equivalent to `quotearg_n_mem(int n, char const *arg, size_t argsize)` that quotes exactly the supplied byte range.

**Traceability:** `quotearg.c:931-935` (`quotearg_n_mem`), `struct slotvec` usages around `quotearg.c:829-878`.

#### FR-3: Slot-indexed quoting with custom delimiters
The module shall provide a slot-indexed operation equivalent to `quotearg_n_custom(int n, char const *left_quote, char const *right_quote, char const *arg)` that returns a quoted representation using the specified left and right quote strings.

**Traceability:** `quotearg.c:1012-1018` (`quotearg_n_custom`), `struct quoting_options` usages around `quotearg.c:1006-1025`.

#### FR-4: Slot-associated result lifetime across calls
The module shall maintain slot-associated returned results so that distinct slot indices identify distinct reusable result locations for quoted output.

**Traceability:** `struct slotvec` at `quotearg.c:829-833` and related uses at `839`, `840`, `845`, `878`.

#### FR-5: Default option-based quoting behavior
The module shall apply the default/module-selected quoting behavior for the non-custom entry points, as mediated by quoting option state.

**Traceability:** `struct quoting_options` declarations/usages at `quotearg.c:57-74`, `108-187`, `874`, `952`, `960`, `979`, `1047`.

#### FR-6: Custom quoting shall override quote delimiters for that operation
For the custom entry point, the module shall use caller-supplied left and right quote delimiters for the produced output rather than the default delimiters of the normal path.

**Traceability:** `quotearg.c:1012-1018`, `struct quoting_options` usage at `1006`, `1025`.

### Key Entities

#### `quoting_options`
A configuration entity that represents quoting behavior choices used by the module’s quoting operations. In this module, it is the means by which default behavior and custom-delimiter behavior are selected for output generation.

**Relationships:**
- Read or derived by the public quoting entry points.
- Customized for `quotearg_n_custom`.
- Applied when generating quoted output for a given input.

**Traceability:** `quotearg.c:57-74`, additional uses at `108-187`, `782-810`, `874`, `952`, `960`, `979`, `1006`, `1025`, `1047`.

#### `slotvec`
A slot-management entity that associates slot numbers with reusable returned storage for quoted results.

**Relationships:**
- Accessed by slot-indexed quoting entry points.
- Ensures per-slot result association across repeated calls.

**Traceability:** `quotearg.c:829-833`, uses at `839`, `840`, `845`, `878`.

## Success Criteria

1. A Rust implementation exposes behavior equivalent to all three covered entry points: default string quoting, explicit-length quoting, and custom-delimiter quoting.
   **Traceability:** `quotearg_n`, `quotearg_n_mem`, `quotearg_n_custom`.

2. For explicit-length input, tests demonstrate that embedded NUL bytes are included according to the specified byte length rather than truncating at the first NUL.
   **Traceability:** `quotearg_n_mem`.

3. For custom quoting, tests demonstrate that the produced result reflects the supplied left and right quote strings.
   **Traceability:** `quotearg_n_custom`, `quoting_options`.

4. Tests using at least two different slot indices demonstrate independent slot-associated results.
   **Traceability:** `slotvec`, `quotearg_n`, `quotearg_n_mem`, `quotearg_n_custom`.

5. Repeated use of the same slot index demonstrates that the result for that slot updates to the most recent quoted input without requiring a different API surface.
   **Traceability:** `slotvec`, slot-indexed entry points.

6. The Rust rewrite introduces no required functionality beyond the behaviors specified here for this module boundary.
   **Traceability:** module scope limited to the cited functions and dependent entities.