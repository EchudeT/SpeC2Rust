# Implementation Plan

## Summary

Port the C module `close-stream.c` into a Rust module that preserves the existing behavior of `close_stream` without adding new capabilities. The Rust implementation should focus on translating the stream-closing logic into idiomatic, minimal Rust while keeping compatibility with the surrounding `pwd` main-cluster behavior.

The preferred technical approach is to implement a small Rust module that operates on standard-library I/O abstractions where possible and keeps the original function’s decision flow explicit. Since the source module exposes a single function and no standalone data structures, the migration should remain narrow: reproduce close/flush/error-propagation behavior, map C-style return/error handling into Rust `Result` or a small internal status mapping as required by the existing call sites, and ensure resources are released through ownership and drop semantics rather than manual memory management.

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**: Rust standard library only (`std::io`, `std::fs` if needed by call sites)
- **Testing**: `cargo test`
- **Performance Goals**:
  - Match the C implementation’s constant-time close-path behavior.
  - Avoid additional heap allocation in the ported function.
  - Preserve low-overhead stream finalization and error reporting.
  - Rely on Rust ownership to eliminate manual cleanup costs and memory hazards.

## Module Mapping

- **C source file**: `close-stream.c`
- **Rust module target**: `src/main_root_close_stream.rs` or the nearest existing main-cluster module file consistent with the current crate layout
- **C function -> Rust function**:
  - `close_stream` -> `close_stream`

If the current Rust port keeps a single main aggregation file instead of one file per migrated C unit, place `close_stream` into that existing file rather than introducing extra structure.

## Data Model

This module does not define dedicated C data structures.

Expected technical mappings for function-level concepts:

- **`FILE *` / C stream handle** -> Rust stream owner or mutable reference, using standard-library types already used by the surrounding port
- **C integer status code** -> `Result<(), std::io::Error>` internally, with conversion to the project’s required outward return form if the surrounding main module expects integer exit/status semantics
- **`errno`-style failure signaling** -> `std::io::Error` propagation or explicit mapping at the module boundary

Because no explicit structs are declared in this module, no standalone Rust struct or enum should be introduced unless needed strictly to match an existing project-wide error/status type.

## Implementation Phases

### Phase 1: Inspect and place the module in the Rust main cluster

- Identify how the current Rust branch organizes previously ported main-cluster functions.
- Create or update the Rust file that corresponds to `close-stream.c` using the project’s existing naming/layout convention.
- Define the Rust signature for `close_stream` based on the actual surrounding call pattern:
  - use a standard-library writer/stream abstraction if already established in the port,
  - otherwise keep the function narrowly scoped to the concrete stream type used by callers.
- Document the exact C return-path behavior that must be preserved, especially around close failure versus prior write failure.

### Phase 2: Port `close_stream` logic with explicit error mapping

- Translate the C control flow directly into Rust, keeping behavior-focused parity rather than redesign.
- Replace manual close/error checks with the nearest Rust equivalent:
  - flush/finish if the original function distinguishes buffered output finalization,
  - allow ownership/drop to release the stream,
  - surface close/finalization errors explicitly where Rust exposes them.
- Preserve the original distinction between success and failure states.
- Avoid introducing generic wrappers or broader I/O abstractions beyond what the existing caller set requires.

### Phase 3: Integrate with callers and status conventions

- Update the immediate Rust call sites in the main cluster to use the new `close_stream` function.
- Map Rust `Result` values to the project’s established top-level error/status handling if needed.
- Verify that ownership transfer and lifetimes make the stream unusable after close, replacing C’s manual invalidation with Rust move semantics where appropriate.
- Ensure no duplicated cleanup remains at the call sites after migration.

### Phase 4: Add focused tests for close-path behavior

- Add unit tests covering the migrated `close_stream` behavior through `cargo test`.
- Prefer narrowly scoped tests that verify:
  - successful close/finalization,
  - propagation of flush/close errors where a failing writer can be simulated,
  - no double-close pattern at migrated call sites.
- Use only standard-library testing facilities unless the repository already contains a local test helper for failing writers.