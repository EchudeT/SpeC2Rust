# Implementation Plan

## Summary

Port `fclose.c` into a Rust module that preserves the existing close-and-error-reporting behavior embodied by `fclose_nothrow` and `rpl_fclose`. The Rust implementation should stay narrowly aligned with the current C responsibilities: closing owned file handles, surfacing close failures without panicking, and preserving the distinction between a low-level non-throwing close helper and the replacement/public close path.

The preferred technical approach is to model these functions over Rust standard library file types and explicit `Result` returns rather than relying only on `Drop`, since `Drop` suppresses close errors. Ownership transfer should be explicit so the Rust port can force a close attempt at the same call site where C currently calls `fclose`. The implementation should avoid adding new abstraction layers beyond what is needed to migrate `fclose.c`.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only (`std::fs`, `std::io`, `std::mem`, `std::os::fd`/`std::os::unix::io` as needed by target codebase layout)
- **Testing**: `cargo test`
- **Performance Goals**:
  - Match C module behavior with no meaningful additional allocation overhead.
  - Keep close-path work constant-time aside from OS close cost.
  - Avoid redundant buffering, wrapping, or handle duplication during migration.
  - Preserve straightforward error propagation using `std::io::Result`.

## Module Mapping

| C File | C Function | Rust Target | Notes |
|---|---|---|---|
| `fclose.c` | `fclose_nothrow` | `src/main_cluster/fclose.rs::fclose_nothrow` | Internal helper that consumes/owns the file handle and returns close status as `io::Result<()>`. |
| `fclose.c` | `rpl_fclose` | `src/main_cluster/fclose.rs::rpl_fclose` | Main replacement close entry point; should call the helper and preserve module-specific error handling semantics. |

### Proposed Rust File Layout

| Rust File | Purpose |
|---|---|
| `src/main_cluster/fclose.rs` | Direct port of `fclose.c` functions only. |
| `src/main_cluster/mod.rs` or existing parent module file | Expose `fclose` module if the current crate structure requires it. |
| `tests/` or inline unit tests in `fclose.rs` | Focused tests for successful close and failure propagation where feasible. |

## Data Model

This module has no declared standalone C data structures to migrate.

### Handle Mapping

| C Concept | Rust Mapping | Ownership / Error Notes |
|---|---|---|
| `FILE *` | `std::fs::File` or the project’s existing owned file-handle type | Must be owned by the close function so close errors can be observed explicitly. |
| close status integer / `EOF`-style result | `std::io::Result<()>` | Prefer idiomatic Rust error propagation; convert to crate-local conventions only if required by surrounding code. |
| `errno`-based failure | `std::io::Error` | Preserve OS error information rather than flattening prematurely. |

### Memory Management Notes

- C manually closes `FILE *`; Rust should consume the owned `File` and force closure before value drop completes.
- Do not rely solely on destructor behavior, because `Drop` does not provide a caller-visible close result.
- If the original module handles already-flushed or buffered streams specially, keep that behavior local to these two functions rather than introducing a broader stream abstraction.

## Implementation Phases

### Phase 1: Module Skeleton and Signature Mapping

- Create `src/main_cluster/fclose.rs`.
- Define Rust equivalents for:
  - `fclose_nothrow`
  - `rpl_fclose`
- Choose the narrowest viable function signatures based on how the surrounding port passes file handles:
  - Prefer owned `File` parameters.
  - Return `std::io::Result<()>`.
- Wire the module into the existing crate module tree without adding new layers.

**Deliverable**: Compiling Rust module with placeholder or initial implementations and stable public/internal visibility matching current call needs.

### Phase 2: Close Semantics Port

- Implement `fclose_nothrow` as the low-level helper that performs an explicit close attempt and reports any OS error.
- Implement `rpl_fclose` as the higher-level replacement entry point that delegates to `fclose_nothrow` and preserves the intended observable behavior from the C code.
- Ensure ownership is consumed exactly once and no double-close path is introduced.
- Preserve error information with `io::Error` instead of panicking or silently swallowing failures.

**Deliverable**: Functional port of `fclose.c` behavior with explicit close/error handling.

### Phase 3: Error-Path Alignment and Integration Cleanup

- Review the original C logic for distinctions between ordinary close failure and cases where prior stream state influences the returned result.
- Mirror only those semantics that are present in `fclose.c`; do not broaden to generalized stream utilities.
- Adjust call sites, if needed, to pass owned handles into the Rust close functions.
- Keep conversions between crate-local status conventions and `io::Result` confined to module boundaries.

**Deliverable**: Integrated module with behavior aligned to original C close semantics.

### Phase 4: Tests

- Add unit tests for:
  - successful close on a temporary file,
  - single-consumption ownership behavior,
  - error propagation in any reproducible failure scenario supported by the target platform/project setup.
- Prefer deterministic tests using standard library facilities.
- Run `cargo test` and fix any module export or signature mismatches revealed by integration.

**Deliverable**: Passing tests covering the migrated functionality with no extra feature expansion.