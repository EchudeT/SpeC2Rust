# Implementation Plan

## Summary
This module ports the `fflush.c` logic related to clearing an `ungetc` buffer into Rust for the `pwd` project branch `009-main_root_clear_ungetc_09-rust-port`.

The Rust implementation should preserve the narrow scope of the original C module: migrate the behavior of:

- `clear_ungetc_buffer_preserving_position`
- `clear_ungetc_buffer`

The technical approach should favor a direct translation of the existing control flow into a small Rust module under the main cluster, using standard library I/O abstractions where possible and isolating any platform- or stream-position-specific behavior behind internal helper functions. Since the analyzed C file centers on stream state handling, the Rust port should represent stream mutation explicitly via `&mut` access, return `Result` for fallible operations, and avoid introducing additional subsystem layers beyond what is required to preserve existing behavior.

## Technical Context

- **Language/Version**: Rust 1.77+
  Chosen to align with current stable Rust and standard library I/O traits needed for stream-position management.

- **Primary Dependencies**:
  - Rust standard library
    - `std::io::{self, Read, Seek, SeekFrom, BufRead}` as applicable to the migrated implementation
  - No third-party crates recommended based on the available module evidence

- **Testing**:
  - `cargo test`
  - Unit tests focused on:
    - buffer-clearing behavior
    - preservation of logical stream position where required
    - no-op handling when no pushed-back data is present
    - propagation of I/O errors from underlying stream operations

- **Performance Goals**:
  - Maintain behavior comparable to the C implementation for single-stream buffer clearing
  - Avoid unnecessary allocations during buffer reset
  - Keep operations bounded to stream state updates and minimal seeking/reading needed to preserve position semantics
  - Do not introduce extra buffering layers beyond those required by the migrated logic

## Module Mapping

| C File | C Function | Rust Module | Rust Item |
|---|---|---|---|
| `fflush.c` | `clear_ungetc_buffer_preserving_position` | `src/main_root_clear_ungetc_09.rs` | `fn clear_ungetc_buffer_preserving_position(...) -> io::Result<()>` |
| `fflush.c` | `clear_ungetc_buffer` | `src/main_root_clear_ungetc_09.rs` | `fn clear_ungetc_buffer(...) -> io::Result<()>` |

### Notes
- Keep both migrated functions in a single Rust source file corresponding to this module migration.
- If crate layout already uses a central `main_cluster` mod tree, expose this file through the existing `mod` declarations only; do not create extra abstraction modules.
- Internal helper functions are acceptable only when needed to map C stream-state steps into safe Rust operations.

## Data Model

No explicit C structs were identified in the analysis input for this module.

The migration should therefore use Rust data mappings only as needed by the target codebase’s existing stream representation.

| C Representation | Rust Representation | Notes |
|---|---|---|
| `FILE *` or internal stream handle usage implied by `fflush.c` | Existing project stream abstraction, or a generic `&mut T` where `T: Seek` and related I/O traits as required | Final signature should match the surrounding Rust port’s established stream type, with the smallest trait surface necessary |
| return status / error sentinel | `io::Result<()>` | Replace integer status returns with explicit error propagation |
| internal pushed-back byte state / ungetc buffer state | Fields on the existing Rust stream wrapper, if present | If the project already models ungetc state, clear it in place without adding unrelated metadata |

### Memory Management and Error Handling
- Replace implicit C ownership and mutable global stream state access with explicit mutable borrowing.
- Avoid raw pointers in the Rust port unless already required by the surrounding ported code.
- Represent all fallible seek/read/state-reset operations with `io::Result`.
- Ensure temporary state captured for position preservation is restored or reported as an error without partial silent failure.

## Implementation Phases

## Phase 1: Establish module skeleton and function signatures
- Create `src/main_root_clear_ungetc_09.rs`.
- Add the two public/internal functions mapped from `fflush.c`.
- Align function visibility and signatures with the existing Rust crate structure.
- Determine the minimal stream trait bounds or concrete stream wrapper type needed from surrounding migrated modules.
- Add the module to the crate’s existing `mod` tree without introducing new package structure.

### Deliverables
- Compiling module file with placeholder or initial translated bodies
- Finalized Rust signatures for:
  - `clear_ungetc_buffer_preserving_position`
  - `clear_ungetc_buffer`

## Phase 2: Port buffer-clearing logic from `fflush.c`
- Translate the C control flow for clearing pushed-back input state directly into Rust.
- Implement the preserving-position variant first, since it defines the stricter stream-state behavior.
- Implement the plain clearing variant by either:
  - translating its dedicated logic directly, or
  - delegating to the preserving-position function only if that exactly matches the C behavior
- Keep all stream-state mutation localized to the module and avoid introducing broader buffering APIs.

### Technical focus
- Model stream position capture/restore through `Seek` where applicable.
- Clear any ungetc-related in-memory state without leaking or retaining stale bytes.
- Preserve exact error paths where stream repositioning or state reset can fail.
- Avoid hidden state duplication; mutate existing stream representation in place.

### Deliverables
- Functional Rust implementation of both migrated functions
- Direct comments only where needed to document C-to-Rust behavior mapping

## Phase 3: Integrate with existing stream representation and validate edge cases
- Connect the module functions to the project’s actual stream abstraction if generic prototypes were used initially.
- Verify behavior for:
  - empty ungetc state
  - one or more pushed-back bytes
  - streams requiring seek-based position preservation
  - seek failure or underlying I/O failure
- Remove any temporary assumptions made during translation that are not consistent with neighboring ported modules.

### Deliverables
- Integrated implementation matching crate interfaces
- Clean compile across the branch with no unused compatibility scaffolding

## Phase 4: Add focused tests and finalize migration
- Add unit tests in the module or existing test location following the crate’s conventions.
- Cover the migrated functions with minimal, behavior-focused cases.
- Confirm `cargo test` passes.
- Review for:
  - unnecessary allocations
  - unnecessary trait bounds
  - incomplete error propagation
  - deviations from original mutation order

### Deliverables
- Passing test coverage for the migrated module
- Finalized Rust port of `fflush.c` scope only