# Implementation Plan

## Summary

Port the C module `fclose.c` into a focused Rust module that preserves its existing responsibility: close/flush-backed file handling with the same observable success and failure behavior for the two exported functions `fclose_nothrow` and `rpl_fclose`. The Rust implementation should stay narrow and map directly to the existing file and function boundaries rather than introducing broader I/O abstractions.

The technical approach is to model the C logic with Rust standard library file types and explicit result propagation. Because C `fclose` combines buffered flush and descriptor close semantics, the Rust port should make close behavior explicit by consuming owned file handles and treating flush failures and final close/drop outcomes consistently with the original intent. Any special "nothrow" behavior should be represented as non-panicking Rust code returning status through `Result` or an internal status code that can be translated at the module boundary.

## Technical Context

- **Language/Version**: Rust 1.77+ stable
- **Primary Dependencies**:
  - Rust standard library only
  - No third-party crates are recommended from the provided evidence
- **Testing**:
  - `cargo test`
  - Unit tests for success/failure paths of file close behavior
  - Integration-style tests using temporary files/directories via `std::env::temp_dir`
- **Performance Goals**:
  - Match C module behavior without adding avoidable allocations
  - Keep close-path overhead negligible relative to OS file operations
  - Preserve straightforward ownership-based resource cleanup with no extra buffering layers

## Module Mapping

| C File | Rust Module/File | Notes |
|---|---|---|
| `fclose.c` | `src/main_root_fclose.rs` or `src/fclose.rs` | Single-module port of the existing file; keep naming aligned with project conventions |
| `fclose_nothrow` | `pub(crate) fn fclose_nothrow(...)` | Internal helper preserving non-panicking close/error semantics |
| `rpl_fclose` | `pub(crate) fn rpl_fclose(...)` | Main replacement close routine, migrated directly from C control flow |

If this project already groups `main_cluster` code under an existing module tree, place the Rust file within that tree and expose only the same scope needed by current callers.

## Data Model

This module analysis does not list custom C data structures. The migration therefore centers on standard C I/O handle usage.

| C Type / Concept | Rust Mapping | Notes |
|---|---|---|
| `FILE *` | `std::fs::File` or owned wrapper around `File` | Prefer owned `File` so close is represented by consumption/drop |
| integer status return (`int`) | `std::io::Result<()>` internally, translated to `i32` if required by surrounding module API | Use Rust errors internally for clarity; preserve external status shape only if needed |
| `errno`-style failure | `std::io::Error` | Carry OS error information through `Result` and convert at the boundary if the project expects integer codes |

Because Rust does not expose a direct `fclose` equivalent on `std::fs::File`, the implementation should rely on:
- explicit `flush()` where applicable before finalization
- ownership consumption so the handle cannot be reused after close
- careful treatment of drop/into_raw_fd patterns only if required by existing project interfaces

Avoid introducing custom structs unless a minimal wrapper is required to preserve the exact calling convention already used elsewhere in the port.

## Implementation Phases

### Phase 1: Establish module skeleton and signatures

- Create the Rust file corresponding to `fclose.c`.
- Add direct Rust equivalents for:
  - `fclose_nothrow`
  - `rpl_fclose`
- Decide the exact signature based on the surrounding port:
  - prefer owned file-handle input
  - preserve integer return status only if other migrated code depends on it
- Keep visibility restricted to the current cluster/module requirements.
- Document any unavoidable semantic differences between C `FILE *` closing and Rust `File` finalization in code comments only where needed.

### Phase 2: Port close/error logic faithfully

- Translate the original control flow in `fclose_nothrow` first, since it likely encapsulates the special error-handling path.
- Implement `rpl_fclose` on top of that helper or inline equivalent logic, matching the original order of operations.
- Ensure the Rust code:
  - does not panic on normal I/O failure paths
  - preserves explicit success/failure returns
  - avoids double-close by consuming ownership once
- Handle flush-before-close behavior explicitly where the C code depended on `fclose` to report buffered write errors.
- Keep error mapping local to the module; do not introduce generalized error frameworks.

### Phase 3: Reconcile C/Rust semantic edges

- Review places where C relied on `FILE *` state that is not directly modeled by Rust `File`.
- Resolve any remaining ownership and lifetime issues with the narrowest possible adaptation.
- If the wider port still uses raw descriptors or libc-like conventions, isolate conversion logic inside this module rather than expanding shared infrastructure.
- Verify that the module does not leak file handles and that failure paths leave no reusable invalid handle behind.

### Phase 4: Add tests and finalize integration

- Add `cargo test` coverage for:
  - successful close of a writable file
  - failure propagation from flush/close-related operations where reproducible
  - idempotency assumptions are not relied upon after ownership consumption
- Add integration checks against the immediate callers in the `main_cluster` path if already ported.
- Confirm final file/module naming and exports match the branch’s migration conventions.
- Remove any temporary compatibility code not required by existing callers.