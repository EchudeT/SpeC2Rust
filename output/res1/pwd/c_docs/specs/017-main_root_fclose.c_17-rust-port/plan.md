# Implementation Plan

## Summary

Port the `fclose.c` module into Rust with a narrow scope that preserves the existing close-and-status behavior represented by `fclose_nothrow` and `rpl_fclose`. The Rust implementation should focus on mapping C file-closing semantics onto Rust’s standard I/O and ownership model without adding broader stream abstractions.

The technical approach is to:
- represent the module as a small Rust source file in the main crate,
- implement explicit close helpers around owned file handles and standard output/error flushing where applicable,
- translate C-style integer status reporting into `std::io::Result<()>` internally, with any compatibility-facing layer returning simple status codes only if the surrounding crate requires it,
- ensure resource finalization is explicit rather than relying only on `Drop`, so behavior remains close to the original C control flow.

Particular care is needed around:
- preserving close error propagation,
- handling flush-before-close behavior when required by the migrated logic,
- avoiding double-close or use-after-close patterns by consuming ownership of file objects in close functions.

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**: Rust standard library only (`std::fs`, `std::io`, `std::os::*` only if platform-specific descriptor access is required by the existing crate structure)
- **Testing**: `cargo test`
- **Performance Goals**:
  - no meaningful regression versus C for ordinary file close paths,
  - constant-overhead wrapper logic only,
  - no unnecessary allocations,
  - preserve direct ownership-based resource release using standard library primitives.

## Module Mapping

- **C source**: `fclose.c`
- **Rust target**: `src/fclose.rs` or integrated into the crate’s existing main-cluster module file layout if already established in the port branch
- **C functions to Rust functions**:
  - `fclose_nothrow` -> `fclose_nothrow(...)`
  - `rpl_fclose` -> `rpl_fclose(...)`

Recommended mapping approach:
- keep function names close to the C originals for migration traceability,
- keep the module private or crate-visible unless the current crate organization requires public exposure,
- migrate only the functions present in this module and wire call sites to the Rust equivalents.

## Data Model

This module does not define standalone C structs. The migration should therefore use direct Rust standard-library ownership types rather than inventing new data structures.

### Data-structure mapping

- `FILE *` -> Rust owned file/stream handle appropriate to the call site:
  - `std::fs::File` for ordinary files,
  - if the existing port uses a crate-local stream wrapper elsewhere, use that existing wrapper rather than introducing a new abstraction here.

### Error/status mapping

- C `int` close status (`0` / `EOF` or error-style return) -> `std::io::Result<()>` internally
- if a compatibility boundary requires C-like status values, convert from `Result` at the outermost layer only

### Ownership model

- C raw stream pointer with manual lifetime management -> Rust owned parameter consumption where possible
- explicit close operation should take ownership of the handle so that:
  - the resource cannot be used after close,
  - double-close is structurally prevented,
  - drop remains a fallback, not the primary semantic mechanism

## Implementation Phases

### Phase 1: Module skeleton and signature mapping

- Create the Rust module for `fclose.c`.
- Add Rust equivalents for:
  - `fclose_nothrow`
  - `rpl_fclose`
- Preserve naming correspondence with the C source for auditability.
- Decide the narrowest workable Rust function signatures based on existing call sites in the branch:
  - prefer owned handle parameters,
  - prefer `std::io::Result<()>` internally,
  - add status-code adapters only if required by surrounding migrated code.

Deliverables:
- compilable module file,
- placeholder-compatible function bodies if call-site-first migration is needed,
- documented assumptions on handle types.

### Phase 2: Close semantics and error handling migration

- Implement the actual close path using Rust standard library behavior.
- Ensure flush-before-close behavior is preserved where the C logic distinguishes buffered output handling.
- Make error handling explicit:
  - propagate close/flush failures through `Result`,
  - avoid suppressing errors unless that is specifically the role of `fclose_nothrow`.
- For `fclose_nothrow`, preserve the intended non-throwing/non-aborting behavior by returning status rather than panicking.
- For `rpl_fclose`, implement the replacement close logic that matches the original module’s error-reporting intent, without introducing broader stream management.

Deliverables:
- functional close helpers,
- explicit mapping notes for any C errno-style behavior that must become `io::Error`,
- no unsafe code unless an already-migrated surrounding interface makes it unavoidable.

### Phase 3: Call-site integration and type alignment

- Update module imports and call sites to use the Rust implementations.
- Align any surrounding code that still assumes C pointer-style stream handling:
  - replace manual null/error checks with `Result` handling,
  - consume handles at the close point.
- Keep the integration limited to the files directly affected by this module migration.

Deliverables:
- crate builds with the migrated close module wired in,
- no remaining references to the C implementation for these functions,
- minimal API surface consistent with current crate organization.

### Phase 4: Tests for close outcomes and edge cases

- Add focused unit tests under `cargo test` covering:
  - successful close of a writable file,
  - flush-then-close behavior on buffered output if represented in the Rust call pattern,
  - repeated-close prevention via ownership semantics,
  - error propagation for failing flush/close-adjacent scenarios that can be simulated safely.
- Prefer filesystem-backed temporary files using the standard library.
- Do not add benchmark or concurrency tests.

Deliverables:
- deterministic unit tests for normal and error paths,
- verification that the module preserves expected return/error behavior across migrated callers.