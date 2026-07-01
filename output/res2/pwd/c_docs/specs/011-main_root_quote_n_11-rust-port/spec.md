# spec.md

## Title

Functional Specification: `main_root_quote_n_11` Rust Port

## Overview

This module provides indexed quoting of input strings for use by the program’s main execution path. Its exposed behavior is centered on two entry points from `quotearg.c`:

- `quote_n_mem`
- `quote_n`

These functions return a quoted representation of an argument, selecting storage by numeric slot index. One variant accepts an explicit byte length, and the other accepts a NUL-terminated string.

The Rust rewrite must preserve the observable behavior of this module as a quoting service used by higher-level code: given a slot number and input text, it produces and returns the corresponding quoted form, with behavior grounded in the module’s quoting option state and slot-based storage model evidenced by `struct quoting_options` and `struct slotvec`.

## Feature Specification

### Summary

The Rust version must implement a slot-indexed quoting facility that:

- accepts either a byte slice with explicit length or a NUL-terminated string input equivalent,
- produces a quoted textual result,
- associates the result with the requested numeric slot,
- supports repeated calls for the same or different slot indices,
- uses the module’s quoting option model as the basis for how quoting is performed.

### In-Scope Functionality

1. **Quote input with explicit length**
   - The module must support quoting an input buffer when the caller provides:
     - a slot index `n`,
     - a pointer/string reference to input bytes,
     - an explicit byte length.
   - This corresponds to `quote_n_mem`.

2. **Quote NUL-terminated input**
   - The module must support quoting an input string when the caller provides:
     - a NUL-terminated string equivalent.
   - This corresponds to `quote_n`.

3. **Slot-based result association**
   - The module must keep quoted results associated with the caller-selected slot index.
   - Repeated use of a slot must behave as reuse of that slot’s result storage model, consistent with the presence of `struct slotvec`.

4. **Quoting governed by quoting options**
   - The produced representation must follow the module’s established quoting behavior as defined by the quoting option state evidenced by `struct quoting_options`.
   - The Rust port must preserve the externally observable quoting semantics used by these entry points.

### Out of Scope

The Rust rewrite specification does not require any new capability not evidenced by this module analysis. In particular, this spec does not add:

- new public APIs beyond the evidenced behavior,
- serialization or persistence,
- thread-safety guarantees,
- FFI surface requirements,
- alternate formatting modes not traceable to the existing module behavior.

## User Scenarios & Testing

### Scenario 1: Quote a standard C-style string for a chosen slot

A caller has a normal string argument and wants its quoted form for diagnostic or display use. The caller selects slot `0` and requests quoting through the NUL-terminated string entry point.

**Expected behavior**
- The module returns the quoted representation of that string.
- The result is associated with slot `0`.
- The returned content reflects the module’s quoting rules.

**Test focus**
- Input with ordinary printable characters.
- Verification that the returned value is stable enough for immediate caller use.
- Verification that using slot `0` produces the same quoted content as the explicit-length path with the matching byte count.

### Scenario 2: Quote a byte sequence with explicit length

A caller has an argument represented by a pointer plus length and needs quoting without relying on a terminating NUL byte.

**Expected behavior**
- The module quotes exactly the provided byte range.
- Bytes beyond the provided length do not affect the result.
- The result is returned through the selected slot.

**Test focus**
- Input containing embedded NUL bytes or trailing memory beyond the requested range.
- Equality of output against the intended byte slice only.
- No dependence on sentinel termination.

### Scenario 3: Use multiple independent slots

A caller quotes separate inputs into different slot indices so that more than one quoted result can be referenced during a larger operation.

**Expected behavior**
- Quoting into one slot does not replace the content associated with a different slot.
- Each slot returns the quoted form of the most recent input assigned to that slot.

**Test focus**
- Quote one value into slot `0` and another into slot `1`.
- Confirm slot `0` still corresponds to its own latest value.
- Confirm slot `1` corresponds to its own latest value.

### Scenario 4: Reuse the same slot

A caller repeatedly quotes different arguments using the same slot index.

**Expected behavior**
- The slot reflects the most recent quoted input for that slot.
- The module continues to return the correct current quoted result for repeated use.

**Test focus**
- Sequential calls with different inputs using the same slot.
- Validation that the latest returned content matches the latest input’s quoted form.

### Scenario 5: Consistent behavior between the two entry points

A caller may reach this module through either the explicit-length or NUL-terminated path for equivalent textual input.

**Expected behavior**
- When both paths describe the same byte content, they produce equivalent quoted content.
- Differences occur only when the explicit-length path intentionally includes bytes not representable through the NUL-terminated form.

**Test focus**
- Same ASCII input via both APIs.
- Matching quoted output for equivalent content.
- Divergence only when explicit-length input includes embedded NUL or additional bytes.

## Requirements

### Functional Requirements

#### FR-1: Indexed quoting service
The module shall provide a quoting operation addressed by integer slot index, traceable to `quote_n_mem` and `quote_n` in `quotearg.c`.

#### FR-2: Explicit-length input support
The module shall accept an input argument together with an explicit byte length and quote exactly that byte sequence, traceable to `quote_n_mem` in `quotearg.c`.

#### FR-3: NUL-terminated input support
The module shall accept a NUL-terminated string equivalent and quote the full string content up to its terminator, traceable to `quote_n` in `quotearg.c`.

#### FR-4: Slot-specific result retention
The module shall maintain result storage associated with slot indices so that different slots can hold different current quoted results, traceable to `struct slotvec` and the slot-based API shape in `quotearg.c`.

#### FR-5: Slot reuse semantics
The module shall allow repeated quoting requests for the same slot index, with the slot reflecting the most recent quoted result for that slot, traceable to the slot vector model in `quotearg.c`.

#### FR-6: Quoting behavior based on quoting options
The module shall produce quoted output consistent with the module’s quoting configuration model, traceable to `struct quoting_options` in `quotearg.c`.

#### FR-7: Functional equivalence across entry points for equivalent content
The module shall produce equivalent quoted output when `quote_n` and `quote_n_mem` are given equivalent input content, traceable to both public entry points in `quotearg.c`.

### Key Entities

#### `quoting_options`
This entity represents the quoting configuration used by the module’s quoting logic. The analysis shows it as the core options structure referenced throughout `quotearg.c`. For this module spec, it is the behavioral source of how input text is transformed into quoted output.

**Role**
- Defines the quoting rules applied by the exposed quoting functions.

**Relationship**
- Used by the quoting operations that ultimately power `quote_n_mem` and `quote_n`.

#### `slotvec`
This entity represents slot-based storage for quoted results.

**Role**
- Associates a caller-provided slot index with stored quoted output.

**Relationship**
- Serves as the storage model behind the indexed APIs `quote_n_mem` and `quote_n`.
- Works alongside quoting behavior governed by `quoting_options`.

## Success Criteria

### Behavioral Correctness

1. **Equivalent API behavior**
   - For the same textual input, the Rust implementations corresponding to `quote_n` and `quote_n_mem` produce the same quoted content.
   - Traceability: `quote_n`, `quote_n_mem`.

2. **Length-bounded quoting**
   - The explicit-length Rust implementation quotes only the specified byte range and ignores bytes beyond that range.
   - Traceability: `quote_n_mem`.

3. **Independent slot behavior**
   - Quoting into one slot does not alter the current quoted content associated with another slot.
   - Traceability: `struct slotvec`, `quote_n`, `quote_n_mem`.

4. **Repeatable slot reuse**
   - Reusing a slot updates that slot to the latest quoted content without breaking quoting for subsequent calls.

5. **Quoting-options consistency**
   - Produced output follows the same externally observable quoting rules as the C module behavior driven by `struct quoting_options`.
   - Traceability: `struct quoting_options`, `quote_n`, `quote_n_mem`.

### Test Completion Criteria

6. **Scenario coverage**
   - Automated tests cover:
     - NUL-terminated input,
     - explicit-length input,
     - multiple slot indices,
     - repeated reuse of a single slot,
     - equivalence between the two entry points for matching content.
   - Traceability: `quote_n`, `quote_n_mem`, `struct slotvec`.

7. **Port parity for module scope**
   - Within the scope of these two entry points, the Rust port matches the C module’s observable behavior for quoted result generation and slot association.
   - Traceability: `quote_n`, `quote_n_mem`, `struct quoting_options`, `struct slotvec`.