# Implementation Plan: main_root_full-write.c_27

## Summary

This module migrates the C source file `full-write.c` into a focused Rust implementation that preserves the existing write-loop behavior of `full_rw`. The Rust version should keep the same low-level responsibility: repeatedly attempt output until the requested buffer range has been handled or a terminal error occurs.

The technical approach is to implement the logic as a small Rust module with a direct translation of the control flow from C into idiomatic Rust error handling. The implementation should prefer the Rust standard library and model partial-write handling explicitly, while avoiding additional abstraction layers beyond what is needed to preserve the original function behavior. Memory ownership becomes simpler in Rust because buffer lifetimes and slice bounds are checked by the type system, but care is still required around repeated writes, interruption handling, and propagation of I/O failures.

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**: Rust standard library only (`std::io`); no third-party crates are required by the available module evidence.
- **Testing**: `cargo test`
- **Performance Goals**:
  - Match the C implementation’s operational profile for repeated partial-write handling.
  - Avoid unnecessary allocations and buffering.
  - Operate directly on borrowed byte slices.
  - Keep loop overhead minimal and rely on standard library I/O traits where possible.
  - Preserve predictable error propagation without adding extra runtime layers.

## Module Mapping

| C Source File | C Function | Rust Target | Notes |
|---|---|---|---|
| `full-write.c` | `full_rw` | `src/full_write.rs::full_rw` | Direct migration of the core loop and error-return behavior. |

### Proposed Rust File Layout

| Rust File | Purpose |
|---|---|
| `src/full_write.rs` | Contains the Rust port of `full_rw`. |
| `src/lib.rs` or existing crate root integration point | Re-exports or wires the migrated module into the current crate structure, only if needed by the existing project layout. |

## Data Model

No named C structs are listed for this module, so the migration is function-centric.

### Function-Level Type Mapping

| C Concept | Rust Mapping | Notes |
|---|---|---|
| raw output buffer pointer + length | `&[u8]` or equivalent borrowed byte slice | Replaces manual pointer/length handling with bounds-checked slices. |
| writable target / file descriptor write operation | generic writer parameter using `std::io::Write`, or a narrowly-scoped internal equivalent if project structure requires it | Choose the least expansive option that matches surrounding code. |
| byte-count return value | `usize` | Natural Rust type for completed byte counts. |
| error return / errno-style failure | `std::io::Result<usize>` or project-local `Result` alias if already established elsewhere | Preserve terminal write errors and interrupted-call retry behavior. |

### Memory Management Notes

- Replace pointer arithmetic with slice indexing and offset tracking.
- Avoid heap allocation; the function should operate on caller-provided borrowed data.
- Ensure all progress tracking is explicit and monotonic to prevent infinite loops on zero-progress conditions.

### Error Handling Notes

- Translate retryable interruption cases into loop continuation where supported by the chosen API.
- Return standard I/O errors directly rather than encoding custom recovery behavior.
- Treat unexpected zero-length progress carefully to preserve the original completion semantics and avoid livelock.

## Implementation Phases

## Phase 1: Establish Rust Module Skeleton

- Create `src/full_write.rs`.
- Add the migrated function signature for `full_rw`.
- Choose the narrowest Rust signature that fits the existing crate:
  - Prefer standard-library I/O traits and byte slices.
  - Do not introduce extra helper modules unless required by current crate organization.
- Wire the module into the crate root only as needed for compilation and existing call sites.

### Deliverables

- Compiling Rust module file with placeholder or initial implementation.
- Stable function signature agreed for the migration target.

## Phase 2: Port Core Write-Loop Logic

- Translate the C loop from `full-write.c` into Rust.
- Preserve behavior for:
  - partial writes,
  - repeated attempts until completion,
  - terminal error return,
  - interruption-aware retry handling as supported by Rust I/O APIs.
- Replace pointer arithmetic with:
  - running byte offset,
  - slicing the remaining portion of the input buffer.
- Keep the implementation local and direct; do not add capability beyond the existing function.

### Deliverables

- Working Rust implementation of `full_rw`.
- No unsafe code unless the surrounding project constraints make it unavoidable; if unavoidable, isolate it to the smallest possible expression and document why.

## Phase 3: Integrate Error Semantics and Edge Cases

- Verify behavior for:
  - empty input,
  - single complete write,
  - multiple partial writes,
  - interrupted writes,
  - immediate failure,
  - zero-progress responses if representable through the chosen API.
- Ensure return values and propagated errors align with the C module’s role.
- Confirm there are no ownership or lifetime issues in caller interactions.

### Deliverables

- Finalized error handling semantics in the migrated function.
- Clean compilation with no unnecessary abstractions.

## Phase 4: Add Focused Tests

- Add unit tests using `cargo test`.
- Use a small custom test writer to simulate:
  - full completion in one call,
  - completion across multiple partial writes,
  - interruption followed by success,
  - hard failure propagation.
- Keep tests scoped to this module’s existing responsibility only.

### Deliverables

- Targeted unit tests covering loop progress and error behavior.
- Passing `cargo test` for the migrated module.