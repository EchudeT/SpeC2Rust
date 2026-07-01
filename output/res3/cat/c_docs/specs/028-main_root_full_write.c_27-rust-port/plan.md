# Implementation Plan

## Summary

Port `full-write.c` into an idiomatic Rust module that preserves the existing low-level write loop behavior of `full_rw` without expanding scope. The Rust implementation should provide the same core responsibility: repeatedly attempting I/O until the requested transfer is completed or an error/terminal condition is reached.

The implementation should stay close to the C control flow and syscall-oriented semantics, while expressing ownership and failure paths through Rust’s standard library types. The main technical approach is:

- map the single C function into a focused Rust function in the main cluster area of the crate,
- use standard `std::io::{Read, Write}` interfaces where feasible,
- preserve partial-progress handling explicitly,
- represent failures with `std::io::Result`,
- avoid introducing extra abstraction layers beyond what is needed to mirror the original file and function.

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**: Rust standard library only (`std::io`)
- **Testing**: `cargo test`
- **Performance Goals**:
  - maintain linear buffered transfer behavior with no unnecessary allocations,
  - preserve efficient handling of partial writes/reads,
  - avoid copying beyond what slice-based standard library APIs require,
  - stay operationally comparable to the original C routine for normal file/stream workloads.

## Module Mapping

| C File | C Function | Rust File | Rust Item |
|---|---|---|---|
| `full-write.c` | `full_rw` | `src/main_root_full_write.rs` | `full_rw` |

### Mapping Notes

- Keep the Rust module narrowly scoped to the migrated logic from `full-write.c`.
- If the surrounding crate already has a module layout for the main cluster, register `src/main_root_full_write.rs` there without creating additional helper modules unless strictly required by compilation.
- Preserve the original function role and naming as closely as Rust conventions and project consistency allow.

## Data Model

This module has no named C structs in the provided analysis.

### Data-structure Mapping

| C Construct | Rust Construct | Notes |
|---|---|---|
| raw buffer pointer + length parameters | `&[u8]`, `&mut [u8]`, or explicit slice windows | Use slice types to encode bounds and borrowing safely. |
| integer byte counters | `usize` | Natural Rust type for buffer lengths and offsets. |
| integer return status / negative error convention | `std::io::Result<usize>` or equivalent result form matching call sites | Use Rust error propagation instead of sentinel values. |
| file descriptor oriented I/O state | generic `Read` / `Write` implementor references, or the smallest crate-consistent equivalent | Prefer standard traits unless the surrounding port already requires direct descriptor handling. |

### Memory and Error Handling Decisions

- Replace manual pointer arithmetic with slice indexing and offset tracking.
- Ensure partial-progress state is tracked explicitly through an index/remaining-length loop.
- Use `std::io::Error` for error propagation.
- Treat zero-length progress carefully to preserve the original termination semantics and avoid infinite loops.
- Do not introduce heap allocation if the C routine operates only on caller-provided buffers.

## Implementation Phases

## Phase 1: Inspect and Define Rust Function Surface

- Examine the original `full_rw` signature and its direct callers in the port branch.
- Select the narrowest Rust signature that matches current crate usage:
  - prefer `Read`/`Write` trait-based parameters if callers are already Rust I/O objects,
  - otherwise mirror existing low-level descriptor-oriented interfaces already present in the crate.
- Create `src/main_root_full_write.rs`.
- Add the module to the crate’s existing main-cluster module declarations.
- Document the intended return semantics in code comments so the migrated behavior stays aligned with the C routine.

### Deliverables

- Compiling Rust module stub with the chosen `full_rw` signature.
- Module wired into the crate without adding unrelated infrastructure.

## Phase 2: Port Core Transfer Loop

- Translate the C loop structure directly into Rust:
  - maintain current offset,
  - retry until the requested amount has been processed or termination condition occurs,
  - update total transferred after each successful partial operation.
- Replace pointer math with safe slicing based on the current offset.
- Preserve the original distinction between:
  - successful full completion,
  - partial completion before terminal condition,
  - immediate failure.
- Handle standard I/O edge cases explicitly:
  - interrupted operations,
  - zero-byte progress,
  - propagated write/read errors.
- Keep implementation local to the single function unless a tiny private helper is required by Rust borrow rules.

### Deliverables

- Working Rust implementation of `full_rw`.
- No unsafe code unless direct descriptor interaction in the surrounding crate makes it unavoidable.

## Phase 3: Validate Behavioral Equivalence with Unit Tests

- Add focused tests covering the migrated function behavior using in-memory readers/writers where possible.
- Cover:
  - full completion in one operation,
  - completion across multiple partial operations,
  - interruption/retry behavior if representable in test doubles,
  - zero-progress terminal case,
  - error propagation after partial or zero progress according to the original semantics.
- Use lightweight custom test doubles only if standard library types cannot model partial-transfer behavior adequately.
- Confirm the module passes `cargo test`.

### Deliverables

- Unit tests for normal, partial, and failure paths.
- Verified build and test success.

## Phase 4: Final Integration Cleanup

- Compare the Rust behavior against the original C implementation one final time for return values, loop exits, and error handling.
- Remove any temporary migration comments or scaffolding that are not needed for maintenance.
- Ensure naming, visibility, and file placement match the project’s existing Rust port conventions.
- Confirm no extra capabilities were added beyond the original module responsibility.

### Deliverables

- Finalized Rust port of `full-write.c`.
- Clean integration on branch `028-main_root_full_write.c_27-rust-port`.