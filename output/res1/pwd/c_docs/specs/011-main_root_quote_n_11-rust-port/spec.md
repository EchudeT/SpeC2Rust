# spec.md

## Title

Functional Specification for `main_root_quote_n_11` Rust Port

## Overview

This module provides indexed quoting entry points for converting an input string into a quoted representation and returning that representation through a reusable internal slot system.

The analyzed C module surface for this port is limited to two functions in `quotearg.c`:

- `quote_n_mem`
- `quote_n`

These functions are part of a broader quoting subsystem that uses quoting configuration data (`struct quoting_options`) and per-index storage slots (`struct slotvec`). Based on the available evidence, this module’s responsibility is not to define new quoting rules, but to expose the “quote argument using slot `n`” behavior for:

- a byte sequence with explicit length, and
- a NUL-terminated string.

The Rust rewrite must preserve this functional boundary: accept an index and input, produce a quoted string using the module’s quoting subsystem behavior, and make the result available in a way equivalent to the original indexed-slot interface.

## Feature Specification

### Summary

The Rust version must implement the indexed quoting behavior represented by `quote_n_mem` and `quote_n`.

This includes:

- selecting or addressing a quoting result slot by integer index,
- quoting input data according to the module’s active/default quoting configuration,
- supporting both explicit-length input and NUL-terminated input forms,
- returning the quoted representation corresponding to the requested slot.

### In-Scope Behavior

1. **Quote explicit-length input**
   - `quote_n_mem` behavior must accept:
     - a slot index `n`,
     - a pointer/reference to input bytes/chars,
     - an explicit input length.
   - It must quote exactly the provided number of input bytes, without requiring NUL termination.

2. **Quote NUL-terminated input**
   - `quote_n` behavior must accept:
     - a NUL-terminated string.
   - It must quote the full string content up to its terminator.

3. **Use indexed result storage semantics**
   - Results are associated with the requested slot index.
   - Repeated calls with different indices must be able to produce independent quoted results as implied by the presence of `struct slotvec`.

4. **Integrate with quoting options**
   - Quoting behavior must be driven by the quoting subsystem’s option state as evidenced by repeated use of `struct quoting_options` in `quotearg.c`.
   - This module must use the applicable default/active quoting configuration rather than inventing module-specific quoting rules.

### Out-of-Scope Behavior

The Rust port specification does not require adding capabilities that are not evidenced in the analyzed surface, including:

- new public quoting styles or configuration APIs,
- thread-safety guarantees,
- serialization or persistence of quoting state,
- FFI-specific compatibility layers,
- performance targets or benchmarks.

## User Scenarios & Testing

### Scenario 1: Quote a normal C string by slot index

A caller has a normal NUL-terminated string and wants a quoted representation associated with slot `0`.

**Expected behavior**
- The caller invokes the Rust equivalent of `quote_n(0, arg)`.
- The module returns the quoted representation for that string.
- The result corresponds to slot `0`.

**Testing focus**
- Verify the returned text is the quoting subsystem’s expected output for the input.
- Verify the call succeeds for ordinary non-empty strings.

### Scenario 2: Quote multiple strings using different slot indices

A caller needs quoted representations for multiple inputs at the same time and uses different indices to keep them distinct.

**Expected behavior**
- The caller invokes the Rust equivalents of:
  - `quote_n(0, first)`
  - `quote_n(1, second)`
- Each call returns the quoted representation for its own slot.
- The second call must not overwrite the logical result for a different slot index in a way that breaks indexed-slot semantics.

**Testing focus**
- Verify different slot indices can hold distinct quoted results.
- Verify results remain attributable to the correct index during the usage sequence.

### Scenario 3: Quote a byte sequence with explicit length

A caller has input that should be quoted using a specified byte count rather than relying on NUL termination.

**Expected behavior**
- The caller invokes the Rust equivalent of `quote_n_mem(n, arg, argsize)`.
- The module quotes exactly `argsize` bytes from the provided input.
- Embedded or trailing data beyond the specified length is not included.

**Testing focus**
- Verify exact-length processing.
- Verify the output matches quoting of only the specified prefix.

### Scenario 4: Quote empty input

A caller passes an empty string or a zero-length byte sequence.

**Expected behavior**
- The module returns the quoted form of empty input according to the active/default quoting behavior.
- The operation completes without requiring special caller-side handling.

**Testing focus**
- Verify empty NUL-terminated input through the `quote_n` path.
- Verify zero-length input through the `quote_n_mem` path.

### Scenario 5: Reuse the same slot index

A caller quotes one input using slot `n`, then quotes another input using the same slot `n`.

**Expected behavior**
- The later call updates the result associated with that slot index.
- The returned quoted result reflects the most recent input for that slot.

**Testing focus**
- Verify same-index reuse returns the new quoted representation.
- Verify no stale content is returned after replacement.

## Requirements

### Functional Requirements

#### FR-1: Indexed quoting for explicit-length input
The module shall provide functionality equivalent to `quote_n_mem` from `quotearg.c`, allowing a caller to request quoting of an input buffer using a caller-specified slot index and explicit input length.

**Traceability:** `quotearg.c:1055-1059` (`quote_n_mem`)

#### FR-2: Indexed quoting for NUL-terminated input
The module shall provide functionality equivalent to `quote_n`, allowing a caller to request quoting of a NUL-terminated string using a caller-specified slot index.

**Traceability:** `quotearg.c:1067-1071` (`quote_n`)

#### FR-3: Exact-length semantics for memory input
For the explicit-length entry point, the module shall process exactly the supplied input length rather than relying on NUL termination.

**Traceability:** `quotearg.c:1055-1059` (`quote_n_mem`)

#### FR-4: Slot-associated result behavior
The module shall preserve the indexed result model implied by the slot storage structure, so that quoted results are associated with the requested integer slot.

**Traceability:** `quotearg.c:829-833` (`struct slotvec`), `quotearg.c:839`, `quotearg.c:840`, `quotearg.c:845`, `quotearg.c:1055-1071`

#### FR-5: Compatibility with quoting subsystem configuration
The module shall produce quoted output using the quoting subsystem configuration model evidenced by `struct quoting_options`, rather than introducing independent quoting behavior.

**Traceability:** `quotearg.c:57-74` (`struct quoting_options`) and related uses at `quotearg.c:108`, `113`, `114`, `117`, `125`, `133`, `144`, `160`, `171`, `184`, `187`, `782`, `784`, `795`, `808`, `810`, `874`, `952`, `960`, `979`, `1006`, `1025`, `1047`

#### FR-6: Support repeated calls across slot indices
The module shall support repeated quoting calls using different slot indices in a single execution context consistent with the indexed-slot API surface.

**Traceability:** `quotearg.c:829-845` (`struct slotvec` usage), `quotearg.c:1055-1071`

#### FR-7: Support repeated calls reusing the same slot index
The module shall support reusing a previously used slot index for a later quoting request, with the returned value corresponding to the most recent quoting operation for that slot.

**Traceability:** `quotearg.c:829-845` (`struct slotvec` usage), `quotearg.c:1055-1071`

### Key Entities

#### `quoting_options`
A quoting configuration entity used by the quoting subsystem to determine how quoted output is formed.

**Role in this module**
- Supplies the configuration context under which `quote_n_mem` and `quote_n` operate.
- Establishes that these functions are consumers of a broader quoting policy, not independent formatting routines.

**Traceability:** `quotearg.c:57-74` and related references throughout `quotearg.c`

#### `slotvec`
A slot storage entity representing per-index quoted-result storage.

**Role in this module**
- Provides the conceptual basis for the integer slot parameter `n`.
- Enables separate result association for different slot indices and replacement on reuse of the same slot.

**Traceability:** `quotearg.c:829-833`, `839`, `840`, `845`, `878`

#### Input string / input memory
The caller-provided source data to be quoted.

**Role in this module**
- For `quote_n`, input is a NUL-terminated string.
- For `quote_n_mem`, input is a byte sequence paired with an explicit size.

**Traceability:** `quotearg.c:1055-1059`, `1067-1071`

#### Quoted result
The quoted representation returned by the module for a given slot and input.

**Role in this module**
- Acts as the externally consumed product of the quoting operation.
- Is associated with a slot index under the module’s indexed result model.

**Traceability:** `quotearg.c:1055-1071`, `quotearg.c:829-845`

## Success Criteria

### SC-1: Behavioral parity for `quote_n_mem`
For representative inputs, the Rust implementation of the explicit-length entry point produces the same quoted output as the C module behavior for the same slot index, input bytes, and length.

**Traceability:** `quote_n_mem` in `quotearg.c:1055-1059`

### SC-2: Behavioral parity for `quote_n`
For representative NUL-terminated inputs, the Rust implementation of the string entry point produces the same quoted output as the C module behavior for the same slot index and string.

**Traceability:** `quote_n` in `quotearg.c:1067-1071`

### SC-3: Exact-length verification
Tests demonstrate that the explicit-length entry point quotes only the specified number of input bytes and does not depend on a terminating NUL.

**Traceability:** `quote_n_mem` in `quotearg.c:1055-1059`

### SC-4: Distinct slot behavior
Tests demonstrate that using at least two different slot indices yields independently retrievable quoted results consistent with indexed-slot behavior.

**Traceability:** `struct slotvec` in `quotearg.c:829-845`; `quote_n_mem` / `quote_n` in `quotearg.c:1055-1071`

### SC-5: Same-slot replacement behavior
Tests demonstrate that reusing the same slot index for a later quoting call causes the slot’s returned result to reflect the later input.

**Traceability:** `struct slotvec` in `quotearg.c:829-845`; `quote_n_mem` / `quote_n` in `quotearg.c:1055-1071`

### SC-6: Empty-input handling
Tests demonstrate successful quoting of:
- an empty NUL-terminated string via the `quote_n` path, and
- a zero-length input via the `quote_n_mem` path.

**Traceability:** `quote_n_mem` in `quotearg.c:1055-1059`; `quote_n` in `quotearg.c:1067-1071`