# Implementation Plan: main_root_fpurge.c_26

## Summary

This module ports the C file `fpurge.c` and its single exported function `fpurge` into Rust with a minimal, file-for-file migration approach. The implementation should preserve the existing module boundary and behavior expectations without introducing broader abstractions or extra facilities.

Technically, the Rust work should focus on replacing the C implementation with a Rust module that exposes the equivalent internal functionality needed by the `cat` project. Because `fpurge` is tied to buffered stream handling in C, the Rust port should first determine whether the current project still requires this operation directly or whether the call sites can be expressed using standard-library I/O types and explicit buffer lifecycle control. If a direct equivalent is needed, the Rust implementation should keep the scope narrow: represent the operation in terms of Rust I/O state management local to this module, and return standard `Result`-based errors rather than C integer status codes where possible at the Rust boundary.

The migration should emphasize:
- preserving existing behavior relevant to this file,
- keeping ownership and lifetime rules explicit,
- avoiding unsafe code unless the exact stream interaction cannot be represented safely,
- limiting changes to the existing module/function footprint.

## Technical Context

### Language/Version
- Rust 1.75+ stable

### Primary Dependencies
- Rust standard library only:
  - `std::io`
  - `std::fs` if needed by surrounding file-based stream handling
- No third-party crates are recommended based on the provided input.

### Testing
- `cargo test`

### Performance Goals
- Match the practical performance characteristics of the C version for this module’s narrow responsibility.
- Avoid unnecessary allocations or buffering layers beyond what the surrounding Rust code already requires.
- Keep stream-state operations constant-time where feasible.
- Ensure no avoidable copies are introduced during buffer discard or reset behavior.

## Module Mapping

| C Source | Rust Target | Notes |
|---|---|---|
| `fpurge.c` | `src/main_root_fpurge.rs` or integrated into the existing main-cluster Rust module layout | Preserve single-purpose scope; do not split further unless the current branch structure already requires a different file path. |
| `fpurge` | `fpurge` | Keep the function name if project conventions allow; otherwise use a narrowly adapted internal Rust name with a compatibility wrapper at the module boundary. |

## Data Model

No explicit C structs are listed for this module.

The primary migration concern is not named data structures but C stream state and return/error conventions.

| C Concept | Rust Mapping | Notes |
|---|---|---|
| `FILE *`-backed stream state | Borrowed Rust I/O object or module-local stream representation | Exact type depends on existing call sites in the Rust port. Prefer safe borrowing over raw pointer-like representations. |
| Integer success/failure return | `Result<(), std::io::Error>` internally | If surrounding migrated interfaces still expect C-style status, convert only at the boundary. |
| `errno`-style failure signaling | `std::io::Error` | Preserve failure semantics as closely as practical without global error state. |

If the original C implementation depends on direct mutation of internal `FILE` buffers, and no Rust standard abstraction exposes equivalent state, use a minimal internal representation only to support this function’s existing usage. Do not generalize it into a reusable stream framework.

## Implementation Phases

### Phase 1: Source Review and Interface Locking
- Inspect `fpurge.c` and identify:
  - the exact function signature,
  - return conventions,
  - required side effects on stream/buffer state,
  - any platform-specific assumptions.
- Trace all call sites in the project to determine the actual Rust-facing interface required.
- Decide the Rust module file location consistent with the existing main-cluster layout.
- Define the narrow Rust function signature, preferring `Result` internally and preserving compatibility at the module edge if needed.

### Phase 2: Core Port of `fpurge`
- Implement the Rust equivalent of `fpurge` in the mapped module.
- Translate C buffer/state manipulation into Rust ownership and borrowing rules.
- Prefer safe standard-library operations; introduce `unsafe` only if unavoidable for parity with the original low-level behavior.
- Ensure error paths are explicit and mapped to `std::io::Error` or to the expected project-local status form.
- Keep the implementation limited to the existing function; do not introduce extra helper subsystems beyond small local helpers.

### Phase 3: Integration with Main Cluster
- Wire the Rust module into the current crate/module tree.
- Update existing references so the migrated `fpurge` path is used in place of the C module logic.
- Confirm naming, visibility, and imports match project conventions without expanding public API surface.

### Phase 4: Validation and Cleanup
- Add focused tests for:
  - successful purge/reset behavior,
  - no-op or valid handling of already-empty buffered state,
  - error propagation for invalid or unsupported stream cases, if applicable.
- Run `cargo test` and resolve integration issues.
- Remove any temporary compatibility code that is no longer needed after the module is fully migrated.
- Verify the final code does not retain C-style manual memory patterns where Rust ownership already covers them.