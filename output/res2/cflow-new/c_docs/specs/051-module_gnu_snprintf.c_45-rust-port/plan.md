# Implementation Plan: module_gnu_snprintf.c_45

## Summary

Port `gnu/snprintf.c` into a focused Rust module that preserves the existing module boundary and behavior scope around `snprintf`. The Rust implementation should center on bounded string formatting into a caller-provided byte buffer, matching C-style truncation and termination expectations as closely as possible within safe Rust.

The implementation should avoid expanding formatting capabilities beyond what is necessary for the migrated function. The preferred technical approach is:

- represent the destination as `&mut [u8]`
- produce formatted content through Rust’s formatting machinery where applicable
- explicitly enforce output length limits and trailing NUL handling
- return an integer result consistent with the C contract expected by the surrounding project
- isolate any unavoidable low-level byte handling in a small internal function

This module has no standalone data model of its own, so the migration effort is primarily behavioral and API-focused.

## Technical Context

- **Language/Version**: Rust 1.78 or newer
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - linear-time formatting and copy behavior relative to produced output length
  - no heap allocation in the core bounded-write path unless required by the chosen formatting bridge
  - predictable truncation behavior for small destination buffers
  - minimal copying beyond the final bounded write into the output buffer

## Module Mapping

| C Source File | Rust Target |
| --- | --- |
| `gnu/snprintf.c` | `src/module_gnu_snprintf_c_45.rs` |

| C Function | Rust Item |
| --- | --- |
| `snprintf` | `pub fn snprintf(...) -> i32` or the closest project-required Rust signature preserving bounded formatting semantics |

### Notes on Mapping

- Keep the port contained to a single Rust source file for this module.
- Do not split formatting helpers into additional modules unless required by compilation boundaries.
- If the wider project needs a C-like API shape, keep that shape localized in this file and implement the core logic in a private helper using slices and explicit length checks.

## Data Model

This module defines no named C structs in the analyzed input.

### Data-Structure Mapping

| C Type / Concept | Rust Mapping |
| --- | --- |
| `char *` destination buffer | `&mut [u8]` internally |
| destination size parameter | `usize` |
| formatted byte count return | `i32` |
| NUL-terminated string output | explicit final `0u8` write when buffer length permits |

### Memory and Error Handling Decisions

- Treat the destination buffer as an owned mutable slice at the Rust API boundary wherever possible.
- Avoid raw pointer manipulation in the core implementation.
- If an external compatibility layer requires pointer-style inputs, convert once at the boundary and validate size assumptions before writing.
- Prevent buffer overruns by truncating writes to `buf.len().saturating_sub(1)` when NUL termination is required.
- Use checked or saturating conversions when translating lengths to `i32`.
- Distinguish formatting failure from truncation:
  - truncation should follow `snprintf`-style success semantics with a full-length return value where feasible
  - unrecoverable internal formatting errors should map to a negative return or the project’s existing error convention

## Implementation Phases

## Phase 1: Establish module file and API boundary

- Create `src/module_gnu_snprintf_c_45.rs`.
- Define the Rust entry point for `snprintf` using the closest signature required by the rest of the port.
- Document the intended C-semantic points at the function boundary:
  - bounded output
  - truncation without overflow
  - NUL termination when buffer size is nonzero
  - integer return semantics
- Add a minimal private helper for bounded buffer writes so unsafe handling, if any, is not spread across the module.

### Deliverables

- module file created
- public function stubbed with final intended signature
- internal helper skeleton for bounded write and termination

## Phase 2: Port bounded formatting behavior

- Implement the formatting path in Rust using standard formatting support and explicit byte copying into the destination buffer.
- Ensure the function computes:
  - produced output length
  - actual copied length
  - trailing NUL placement
- Preserve edge-case behavior for:
  - zero-sized destination buffer
  - one-byte destination buffer
  - exact-fit output
  - truncated output
- Keep conversions between `usize` and `i32` explicit and guarded.

### Deliverables

- working bounded formatting implementation
- explicit truncation and termination logic
- no out-of-bounds writes in any size case

## Phase 3: Align return-value and compatibility semantics

- Verify that the Rust function’s return behavior matches the expected semantics of the original module as used by the project.
- If the surrounding port requires a more C-shaped wrapper, add it in the same file without changing the core helper’s logic.
- Keep compatibility adaptations narrow:
  - argument normalization
  - pointer/slice conversion
  - final return-value mapping

### Deliverables

- stable public API for project integration
- compatibility wrapper only if required by callers
- core logic separated from boundary adaptation

## Phase 4: Add focused tests and finalize migration

- Add unit tests covering:
  - empty buffer handling
  - single-byte buffer with only NUL output
  - non-truncated formatting
  - truncation with correct termination
  - return length independent of truncation
  - large output length conversion behavior
- Run `cargo test` and fix any semantic mismatches found during integration.
- Keep tests local to the module unless the project already uses a centralized test layout.

### Deliverables

- module-level unit tests
- passing `cargo test`
- completed migration of `gnu/snprintf.c` into the Rust module