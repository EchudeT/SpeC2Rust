# Implementation Plan

## Summary

Port `close-stream.c` into a Rust module that preserves the existing role of `close_stream`: closing an output stream/file handle and reporting close-time failures in a way that matches the current `cat` main-cluster behavior.

The Rust implementation should stay narrow in scope:
- migrate the single C file and its exported function,
- use Rust standard library I/O types and error propagation,
- preserve close/flush failure handling semantics at the call sites in the main execution path,
- avoid introducing new abstractions beyond what is required to represent stream ownership and close behavior safely.

Because Rust does not expose a direct equivalent of C `FILE *` with explicit `fclose` semantics in the standard library, the implementation approach should model `close_stream` around owned writable handles whose finalization can surface errors through explicit flush and ownership-consuming close paths where available. The port should prefer technical equivalence over API redesign.

## Technical Context

- **Language/Version:** Rust 1.77+
- **Primary Dependencies:** Rust standard library only (`std::fs`, `std::io`, `std::os::unix` only if required by existing project layout)
- **Testing:** `cargo test`
- **Performance Goals:**
  - Maintain negligible overhead relative to the C helper.
  - Avoid extra buffering layers unless already present in the Rust port structure.
  - Keep close-path work limited to the equivalent flush/finalization checks needed to preserve error behavior.
  - No additional allocations beyond what is needed by existing Rust I/O ownership patterns.

## Module Mapping

- **C source file:** `close-stream.c`
- **Rust target module:** `src/close_stream.rs`

Function migration:
- `close_stream` -> `pub(crate) fn close_stream(...) -> Result<(), std::io::Error>` or a project-local boolean/error-status form if the surrounding port already standardizes that interface

Integration expectations:
- Export only the migrated helper needed by the main cluster.
- Update the main-cluster caller(s) to pass owned or mutable writable handles according to the chosen Rust signature.
- Keep the migration local to the existing main execution flow; do not create extra service layers or utility modules.

## Data Model

This module does not define standalone C structs in the provided input.

Planned type mapping for function-level data handling:

- `FILE *` or equivalent C stream handle
  -> Rust owned file/stream type from `std::fs::File` or a generic writer wrapper already used in the port

- C integer/boolean status from `fclose`
  -> `Result<(), std::io::Error>` internally, with conversion to project-required exit/error status at the caller boundary if needed

- C errno-based close failure reporting
  -> `std::io::Error` using standard library error values

If the surrounding Rust port needs to support both stdout-like and file-backed outputs under one helper, use the smallest project-local enum necessary to mirror existing caller types rather than introducing trait-object abstractions.

## Implementation Phases

### Phase 1: Inspect and map current close semantics

- Review `close-stream.c` and its direct callers in the `cat` main cluster.
- Identify the exact C behavior that must be preserved:
  - whether `close_stream` performs flush checks before close,
  - whether it treats stdout specially,
  - how it reports write-versus-close failures,
  - whether it consumes ownership of the stream.
- Determine the narrowest Rust function signature compatible with current caller migration order.
- Confirm whether existing Rust-port code already uses `File`, `Stdout`, `BufWriter`, or another concrete writer type.

### Phase 2: Implement `src/close_stream.rs`

- Port `close_stream` into a dedicated Rust module.
- Encode close/finalization semantics using standard library primitives:
  - explicit `flush()` before relinquishing the handle where needed,
  - owned-handle drop/into-inner patterns only when they preserve observable error behavior,
  - direct `Result` propagation for I/O failures.
- Ensure ownership is modeled safely:
  - consume the handle if C semantics imply `fclose` ownership transfer,
  - use mutable borrowing only if the surrounding design retains ownership elsewhere and still permits equivalent finalization checks.
- Keep the implementation limited to the single migrated function and any minimal local helper required for concrete handle variants.

### Phase 3: Wire callers and preserve error paths

- Replace C-side `close_stream` usage in the Rust main-cluster path with the new module function.
- Adapt callers so error handling remains aligned with existing program flow:
  - preserve the same point where close errors affect exit status,
  - avoid swallowing late write/flush failures,
  - keep user-visible reporting logic in the existing caller layer if that is where the C program handled it.
- Remove or avoid duplicate close logic at call sites once centralized in the Rust helper.

### Phase 4: Validate with focused tests

- Add unit tests for the migrated helper using standard Rust test facilities.
- Cover the relevant close-path cases that can be reproduced in Rust:
  - successful flush/close path,
  - flush failure or write-finalization error path via a controlled failing writer abstraction if needed,
  - caller-visible status propagation.
- Add or update integration tests in the `cat` command path only where necessary to confirm that output-close failures still influence command success/failure as expected.
- Run `cargo test` and fix any ownership or error-conversion issues exposed during integration.