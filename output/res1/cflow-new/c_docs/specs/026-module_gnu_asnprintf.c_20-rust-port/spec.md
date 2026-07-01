# spec.md

## Title

Functional Specification: `module_gnu_asnprintf.c_20` Rust Port

## Metadata

- Project: `cflow-new`
- Module: `module_gnu_asnprintf.c_20`
- Category: `module_cluster`
- Source file: `gnu/asnprintf.c`
- Primary function: `asnprintf`
- Rust branch: `026-module_gnu_asnprintf.c_20-rust-port`
- Generation date: `2026-06-11`

## Overview

This module provides formatted string construction with allocation-aware behavior through the `asnprintf` entry point.

The Rust rewrite must preserve the module’s observable role: accept a printf-style format string with variadic arguments, produce the formatted character sequence, and report the resulting length through the caller-supplied length pointer. The module’s scope is limited to this formatting-and-return behavior evidenced by `gnu/asnprintf.c` and its single exported function.

## Feature Specification

### Summary

The module formats text from a format string and corresponding arguments, returning a character buffer containing the formatted result and updating the caller-visible output length.

### Required Rust Functionality

The Rust version must implement functionality equivalent to the C module’s public behavior:

- Accept a caller-provided output buffer pointer argument, a caller-provided length output pointer, and a format string with variable arguments.
- Generate the formatted byte sequence represented by the format string and argument list.
- Return a pointer/reference/owned result representing the formatted string result in a way compatible with the surrounding Rust port architecture.
- Communicate the produced string length via the `lengthp` output parameter equivalent.
- Preserve C-observable semantics for successful formatting and result delivery as required by the project’s Rust port integration.

### Functional Boundary

This module’s responsibility is restricted to formatted output creation and result-length reporting. No additional capabilities are evidenced for this module, so the Rust port must not introduce unrelated public functionality.

## User Scenarios & Testing

### Scenario 1: Format a simple string result

A caller needs a newly produced formatted string from a constant format such as plain text or a format with one substituted value.

**Expected support:**
- The module accepts the format and arguments.
- The module returns the formatted result.
- The reported length matches the number of characters in the produced output, excluding any terminating null if the surrounding port preserves C string semantics.

**Testing focus:**
- Plain literal format.
- Single `%s` or integer substitution.
- Length output matches actual produced content length.

### Scenario 2: Format into a caller-managed flow that observes result length

A caller uses the module not only for the resulting string but also to know the exact formatted length for later processing.

**Expected support:**
- The module writes the produced length into the supplied length output location.
- The value corresponds to the returned formatted content.

**Testing focus:**
- Non-empty formatted output.
- Empty formatted output.
- Multi-argument formatting.
- Length value consistency with returned content.

### Scenario 3: Use an existing result buffer argument according to module contract

A caller invokes `asnprintf` with the `resultbuf` argument as defined by the original interface.

**Expected support:**
- The Rust port preserves the original module’s supported observable behavior for this parameter rather than redefining its contract.
- Integration tests verify behavior matches the C module for representative inputs.

**Testing focus:**
- Compare Rust-port behavior with the original C module for the same `resultbuf`, `lengthp`, format, and arguments.
- Validate returned content and reported length are equivalent.

### Scenario 4: Format output requiring dynamic sizing

A caller provides input whose final formatted output size is not known before formatting.

**Expected support:**
- The module still returns the full formatted result and reports its final length.

**Testing focus:**
- Short output.
- Longer output assembled from several substituted values.
- Exact length reporting for variable-sized content.

## Requirements

## Functional Requirements

### FR-1: Formatted output generation

The module shall generate output text from a printf-style format string and corresponding variadic arguments, as evidenced by `asnprintf` in `gnu/asnprintf.c`.

### FR-2: Result return

The module shall return the formatted result through the function result of `asnprintf`, preserving the observable caller-facing behavior of the C module.

### FR-3: Output length reporting

The module shall write the length of the formatted result to the caller-supplied `lengthp` output parameter equivalent, consistent with the content returned by `asnprintf`.

### FR-4: Support for caller-supplied result buffer parameter semantics

The module shall preserve the observable semantics of the `resultbuf` parameter of `asnprintf` as defined by the original module behavior, without changing its supported usage contract.

### FR-5: Variadic formatting interface compatibility

The module shall support invocation patterns equivalent to the C variadic interface of `asnprintf`, as required for module-level behavioral equivalence in the Rust port.

### FR-6: Empty and non-empty formatted results

The module shall correctly handle both empty and non-empty formatting results, returning the corresponding content and reporting the matching length.

## Key Entities

### `asnprintf` operation

The central entity in this module is the `asnprintf` formatting operation. It relates three pieces of caller-visible state:

- `format`: the template describing the output text structure.
- variadic arguments: the values inserted according to the format.
- `lengthp`: the output location that receives the produced result length.

The operation returns the resulting formatted character buffer and updates the output length so both outputs describe the same formatted result.

### Formatted result

The formatted result is the character sequence produced from applying the arguments to the format string. It is the primary output of the module and is directly related to the reported length.

### Output length

The output length is the caller-visible size of the formatted result and must correspond to the returned formatted content.

## Success Criteria

### SC-1: Behavioral equivalence for formatting

For representative valid format strings and arguments exercised against the original module, the Rust port produces the same formatted textual result as `asnprintf`.

### SC-2: Correct length reporting

For representative valid inputs, the Rust port reports a length value equal to the produced formatted result length for every successful call.

### SC-3: Empty-output correctness

When formatting yields an empty result, the Rust port returns the empty content and reports a length of zero.

### SC-4: Result-buffer parameter compatibility

For representative calls that vary the `resultbuf` argument within the original module’s supported usage, the Rust port matches the C module’s observable behavior.

### SC-5: Integration suitability

The Rust module can replace the behavior of `gnu/asnprintf.c` within the project branch without requiring callers to rely on functionality beyond the original `asnprintf` contract.