# spec.md

## Title

`main_root_clear_ungetc_09` functional specification

## Metadata

- Project: `pwd`
- Module: `main_root_clear_ungetc_09`
- Category: `main_cluster`
- Rust branch: `009-main_root_clear_ungetc_09-rust-port`
- Source basis: `fflush.c`
- Generation date: 2026-06-07

## Overview

This module provides internal stream-state handling related to clearing buffered `ungetc` data from a `FILE` stream. Its purpose is to remove pushed-back input state while preserving correct file-position behavior as required by the two observed internal functions.

The Rust rewrite must implement equivalent behavior for:

- clearing pushed-back input while preserving the stream position when required
- clearing pushed-back input for a stream in the broader sense used by the module

This specification is limited to behavior evidenced by the analyzed module content and does not define any new public API beyond what is required to preserve the original module’s function.

## Feature Specification

### Feature: clear pushed-back input state on a stream

The module manages the removal of `ungetc`-buffered state from a stream object.

Observed behavior is split into two internal responsibilities:

1. **Position-preserving clear**
   - Clear pushed-back input state associated with a stream.
   - Preserve the stream’s logical file position while doing so.

2. **General clear of ungetc state**
   - Clear pushed-back input state from a stream.
   - Ensure the stream is left without residual `ungetc` content after the operation.

The Rust version must preserve the same functional boundary: it must operate on stream state, remove pushed-back input state, and maintain position preservation where the source module distinguishes that requirement.

### In scope

- Internal handling of stream state associated with `ungetc`
- Behavior differences between general clearing and position-preserving clearing
- Operation on an existing stream entity analogous to `FILE *`

### Out of scope

- Defining new public user-facing stream APIs
- General flushing behavior not evidenced by the listed functions
- Threading guarantees
- Cross-platform abstraction promises beyond source-equivalent behavior
- Any persistence, serialization, recovery, or FFI layer

## User Scenarios & Testing

### Scenario 1: Stream has pushed-back characters that must be discarded

A caller operating on an input stream reaches a point where any pending `ungetc` characters must no longer be visible to future reads.

**Expected result:**
- The stream no longer exposes previously pushed-back characters.
- Subsequent stream behavior reflects that the pushed-back buffer has been cleared.

**Testing focus:**
- Construct a stream state with pushed-back input.
- Invoke the Rust equivalent of the module behavior.
- Verify that previously pushed-back bytes/chars are not returned by subsequent reads.

### Scenario 2: Clearing pushed-back state must not disturb file position

A caller needs to discard `ungetc` state but must preserve the stream’s logical position.

**Expected result:**
- The pushed-back state is removed.
- The stream position after the operation is the same logical position required by the original module behavior.

**Testing focus:**
- Create a seekable stream.
- Read data, push back data, record expected logical position.
- Invoke the position-preserving clear behavior.
- Verify that the resulting position matches expected preserved position.
- Verify that pushed-back content is no longer present.

### Scenario 3: Stream has no pushed-back state

A caller clears `ungetc` state on a stream that currently has none.

**Expected result:**
- The operation completes without introducing new stream data or changing semantics beyond the absence of `ungetc` state.
- The stream remains usable.

**Testing focus:**
- Use a stream with no prior `ungetc`.
- Invoke each relevant clearing path.
- Verify no pushed-back content appears and stream remains valid for continued use.

### Scenario 4: Clearing is part of internal stream maintenance

The module is used as an internal helper in larger stream-management logic.

**Expected result:**
- The Rust rewrite exposes equivalent internal behavior that can be called from surrounding stream code.
- The helper leaves the stream in a consistent state for subsequent operations.

**Testing focus:**
- Integrate the Rust implementation into a higher-level stream workflow.
- Verify downstream reads or position queries behave consistently after clearing.

## Requirements

### Functional Requirements

- **FR-1:** The module shall provide internal functionality to clear pushed-back `ungetc` state from a stream entity corresponding to `FILE *`.
  **Traceability:** `clear_ungetc_buffer` in `fflush.c:49-70`

- **FR-2:** The module shall provide internal functionality to clear pushed-back `ungetc` state while preserving the stream position semantics required by the original source.
  **Traceability:** `clear_ungetc_buffer_preserving_position` in `fflush.c:38-44`

- **FR-3:** After a clear operation completes, previously pushed-back input shall no longer be available as pending input from the stream.
  **Traceability:** implied by `clear_ungetc_buffer` and `clear_ungetc_buffer_preserving_position` in `fflush.c`

- **FR-4:** The position-preserving clear path shall maintain the stream’s logical position rather than leaving it shifted by the presence or removal of pushed-back input.
  **Traceability:** `clear_ungetc_buffer_preserving_position` in `fflush.c:38-44`

- **FR-5:** The general clear path shall leave the stream in a consistent state suitable for subsequent stream operations after `ungetc` state removal.
  **Traceability:** `clear_ungetc_buffer` in `fflush.c:49-70`

### Key Entities

- **Stream object**
  - The central entity is a C `FILE *` stream, which the Rust rewrite must model with an internal stream representation sufficient to support equivalent `ungetc`-state clearing behavior.
  - Relationship: both observed functions operate on this stream entity.

- **Pushed-back input state**
  - Internal state representing characters or bytes previously returned to the stream via `ungetc`-like behavior.
  - Relationship: this is the state both functions remove.

- **Logical stream position**
  - The stream position relevant to continued reading and seeking semantics.
  - Relationship: the position-preserving clear operation must maintain this correctly while clearing pushed-back state.

## Success Criteria

- **SC-1:** For a stream containing pushed-back input state, the Rust implementation removes that state such that subsequent reads do not return the discarded pushed-back data.
  **Traceability:** `clear_ungetc_buffer`, `clear_ungetc_buffer_preserving_position`

- **SC-2:** For a seekable stream where position can be observed, invoking the position-preserving clear behavior leaves the logical stream position unchanged relative to the expected source-equivalent semantics.
  **Traceability:** `clear_ungetc_buffer_preserving_position`

- **SC-3:** Invoking the general clear behavior on a stream with no pushed-back state completes without creating incorrect pending input and leaves the stream usable for continued operations.
  **Traceability:** `clear_ungetc_buffer`

- **SC-4:** The Rust rewrite keeps the module limited to the evidenced responsibility of clearing `ungetc`-related stream state and does not require invention of unrelated public features.
  **Traceability:** module scope evidenced solely by `fflush.c` functions listed above