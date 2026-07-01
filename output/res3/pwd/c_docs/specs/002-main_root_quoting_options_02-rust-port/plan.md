# Implementation Plan

## Summary

This module ports the `quotearg.c` portion needed for `quotearg_n_custom_mem` into Rust, with a narrow scope limited to preserving the existing behavior used by the `pwd` project branch `002-main_root_quoting_options_02-rust-port`.

The Rust implementation should:
- migrate the logic of `quotearg_n_custom_mem` only,
- keep behavior aligned with the current C semantics for custom-memory quoting,
- use standard-library string/byte handling rather than introducing new abstraction layers,
- preserve byte-oriented processing where C operates on arbitrary memory rather than UTF-8 text.

The technical approach is to implement a focused Rust module that accepts byte slices for input and returns owned quoted output, while keeping any option/state representation minimal and local to the migrated functionality. Any C patterns based on raw pointers, explicit lengths, and static storage should be translated into safe Rust ownership and borrowing where possible, with narrow internal use of low-level indexing only if required for exact behavior.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - preserve linear-time processing over the input buffer,
  - avoid unnecessary intermediate allocations beyond the final output buffer,
  - maintain byte-for-byte output compatibility for covered cases,
  - keep allocation behavior predictable using `Vec<u8>`/`String` capacity reservation where input length is known.

## Module Mapping

| C File | C Function | Rust Module | Rust Item |
|---|---|---|---|
| `quotearg.c` | `quotearg_n_custom_mem` | `src/main_root_quoting_options_02.rs` | `pub fn quotearg_n_custom_mem(...) -> ...` |

### Planned Rust file placement

- Add a single Rust source file for this migration unit:
  - `src/main_root_quoting_options_02.rs`

If the current crate already centralizes main-cluster logic elsewhere, this file should be wired into the existing module tree without adding extra layers. The port should not split this single-function migration across multiple new files.

## Data Model

The analysis only reports anonymous C data structures from `quotearg.c`, which indicates the target function likely depends on internal option/config storage rather than named public structs. Because only `quotearg_n_custom_mem` is in scope, the Rust data model should be reduced to the minimum necessary to support that function.

| C Construct | Rust Mapping | Notes |
|---|---|---|
| anonymous internal struct(s) used by quoting logic | private `struct` only if required | Introduce only when a grouped set of fields is necessary to keep the logic readable and equivalent. |
| C pointer + length memory input | `&[u8]` | Preferred mapping for arbitrary memory that may not be valid UTF-8. |
| C char pointer output / allocated buffer | `Vec<u8>` or `String` | Use `Vec<u8>` internally for byte-accurate assembly; convert to `String` only if the surrounding crate interface already requires text and conversion is guaranteed safe by escaping rules. |
| C custom quote delimiters / custom strings | `&[u8]` or small private struct holding slices | Keep borrowed inputs; avoid copying configuration unless ownership is required by the crate API. |
| C integer slot/index parameter (`n`) | `usize` | If the C API uses nonnegative indexing semantics, map directly to `usize`. |
| C nullable pointers | `Option<&[u8]>` / `Option<T>` | Represent absence explicitly rather than sentinel null values. |

### Memory management decisions

- Replace C raw-memory ownership conventions with Rust-owned output buffers.
- Prefer borrowing for input memory and custom delimiter/config data.
- Avoid global mutable storage unless the existing Rust crate already depends on slot-based persistent buffers; if such behavior is required for compatibility, keep it private to this module and model it with the smallest possible state surface.
- Keep processing byte-oriented to avoid accidental UTF-8 assumptions.

### Error handling decisions

- If the original C function is infallible under valid inputs, keep the Rust function infallible.
- If invalid custom quoting configuration must be represented, use a small local error type or `Option` only when there is clear evidence from the existing Rust crate interface.
- Do not introduce generalized error frameworks.

## Implementation Phases

## Phase 1: Establish the Rust module skeleton and signature mapping

### Goals
- Create the Rust file for this migration unit.
- Define the Rust-facing signature for `quotearg_n_custom_mem`.
- Identify the minimum internal helper data needed from `quotearg.c`.

### Tasks
- Add `src/main_root_quoting_options_02.rs`.
- Map the C function parameters to Rust types:
  - slot/index parameter to `usize`,
  - input memory plus length to `&[u8]`,
  - custom quoting operands to borrowed byte slices or a minimal private config struct.
- Wire the module into the existing crate structure without creating additional submodules.
- Document any behavior assumptions needed to reconcile C pointer/length APIs with Rust slices.

### Deliverables
- Compiling module skeleton.
- Public function stub for `quotearg_n_custom_mem`.
- Private type definitions only if needed for the migrated logic.

## Phase 2: Port the byte-wise quoting logic

### Goals
- Recreate the custom-memory quoting behavior of `quotearg_n_custom_mem`.
- Preserve C-compatible handling of arbitrary input bytes and explicit lengths.

### Tasks
- Translate the core quoting path from `quotearg.c` into byte-slice processing.
- Implement output assembly with `Vec<u8>` or `String` based on the exact output contract already used by the Rust crate.
- Preserve explicit treatment of:
  - custom left/right quote markers or equivalent custom delimiters,
  - embedded non-UTF-8 bytes,
  - zero-length input,
  - escaping/quoting rules required by this specific function.
- Keep helper routines private and limited to the directly migrated logic.

### Deliverables
- Functional Rust implementation of `quotearg_n_custom_mem`.
- No unsafe code unless exact C behavior cannot otherwise be preserved; if unsafe is unavoidable, isolate it to a minimal internal block with a clear invariant comment.

## Phase 3: Reconcile storage and ownership semantics with the original API behavior

### Goals
- Match the observable result shape expected from the original C call pattern.
- Remove C-specific lifetime/storage assumptions by converting them into explicit Rust ownership.

### Tasks
- Determine whether the C function’s slot-based `n` parameter affects persistent per-slot output storage or is only a selector into shared quoting state.
- If persistent slot behavior is required by existing call sites, implement the smallest private equivalent compatible with current Rust crate patterns.
- Otherwise, keep `n` as an accepted parameter while limiting its use to behavior actually exercised by the original function.
- Ensure there are no dangling-borrow equivalents: returned data must be clearly owned or borrowed from caller input only when safe.

### Deliverables
- Finalized API behavior for output ownership.
- Slot/index semantics implemented only to the extent required by current usage.

## Phase 4: Add focused tests and finalize compatibility checks

### Goals
- Validate parity for the migrated function.
- Lock down edge-case behavior around explicit-length memory quoting.

### Tasks
- Add unit tests covering:
  - empty input,
  - plain ASCII input,
  - input containing bytes that require quoting/escaping,
  - embedded `0u8` within the input slice,
  - custom delimiter inputs,
  - stability of output for repeated calls.
- Add targeted tests for any slot/index behavior if retained.
- Confirm all tests pass with `cargo test`.

### Deliverables
- Unit test coverage for the migrated function.
- Final cleanup of comments and internal naming to reflect the original C function mapping without adding new abstractions.