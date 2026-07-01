# Implementation Plan: module_gnu_msvc-nothrow.c_37

## Summary

This module ports `gnu/msvc-nothrow.c` into Rust with a narrow scope centered on `_gl_nothrow_get_osfhandle`. The Rust implementation should preserve the existing behavior of the C function while translating its low-level handle-access logic into idiomatic Rust where possible, and using minimal platform-specific bindings where necessary.

The implementation approach is to:
- create a focused Rust module corresponding to `gnu/msvc-nothrow.c`,
- map the single exported function into a Rust function with equivalent semantics,
- preserve failure behavior and sentinel-value handling from the C implementation,
- isolate any Windows-specific file-descriptor or OS-handle access behind `cfg(windows)` so the port remains structurally aligned with the source file,
- keep memory management trivial by avoiding heap allocation and by using value-based return types.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**:
  - Rust standard library
  - No third-party crates by default
  - If direct Windows API surface is required beyond `std`, use the minimal `windows-sys` crate only for the exact handle-related bindings needed by the migrated function
- **Testing**: `cargo test`
- **Performance Goals**:
  - Match the C implementation’s constant-time behavior for descriptor-to-handle retrieval
  - Avoid additional allocations
  - Keep wrapper overhead negligible relative to the underlying OS/runtime call
  - Preserve direct error-path behavior without extra abstraction layers

## Module Mapping

### Source-to-target mapping

| C file | Rust module/file | Notes |
|---|---|---|
| `gnu/msvc-nothrow.c` | `src/gnu/msvc_nothrow.rs` | Direct port of the source file into a single Rust module |
| `_gl_nothrow_get_osfhandle` | `pub(crate) fn gl_nothrow_get_osfhandle(...)` | Keep name close to source while adapting to Rust identifier conventions |

### Project structure

| Path | Purpose |
|---|---|
| `src/gnu/mod.rs` | Declares `msvc_nothrow` module if the project already groups GNU-derived ports under `gnu` |
| `src/gnu/msvc_nothrow.rs` | Contains the migrated implementation of `_gl_nothrow_get_osfhandle` |
| `tests/` or inline `#[cfg(test)]` tests | Verifies success/failure semantics and platform gating as appropriate |

### Platform mapping

| C platform condition | Rust equivalent |
|---|---|
| MSVC/Windows-specific logic | `#[cfg(windows)]` |
| Non-Windows exclusion paths | `#[cfg(not(windows))]` stubs only if required by the crate’s public API surface |

## Data Model

This module has no custom C structs or persistent data structures to migrate.

### Scalar/type mapping

| C type/pattern | Rust type | Notes |
|---|---|---|
| file descriptor integer | `i32` | Preserve narrow integer semantics typically used for CRT file descriptors |
| OS file handle / native handle value | `isize` or platform alias | Use a signed integer-sized type if mirroring sentinel returns such as invalid handle values |
| invalid return sentinel | constant in Rust module | Represent explicitly to avoid magic numbers in call sites |

### Error representation

| C style | Rust mapping | Notes |
|---|---|---|
| sentinel return for failure | preserve sentinel-compatible return type, or wrap internally and expose a compatibility layer | Prefer behavior-preserving API for this migration |
| errno-like side effects | do not invent new error abstractions | Preserve existing observable behavior only to the extent required by the port |

## Implementation Phases

## Phase 1: Create the Rust module skeleton

- Add `src/gnu/msvc_nothrow.rs`.
- Register the module from `src/gnu/mod.rs` if that hierarchy already exists in the Rust project.
- Define the Rust equivalent of `_gl_nothrow_get_osfhandle` with a signature aligned to the expected callers.
- Add compile-time platform gating so Windows-specific code is isolated cleanly.

### Deliverables
- Module file exists and compiles in isolation.
- Function signature decided and documented in code comments for compatibility.

## Phase 2: Port `_gl_nothrow_get_osfhandle` behavior

- Translate the C logic directly into Rust without broad redesign.
- Use `std` APIs first; if the C behavior depends on CRT/Windows details not exposed by `std`, add the smallest possible platform binding surface.
- Preserve invalid-descriptor handling and failure sentinel behavior.
- Keep the implementation allocation-free and side-effect minimal.

### Technical notes
- Avoid introducing ownership-heavy wrappers around native handles; this function appears to retrieve, not assume ownership.
- Ensure the returned value is treated as borrowed/observational, matching the original semantics.
- Keep unsafe code tightly scoped and documented if native calls are required.

### Deliverables
- Functional Rust implementation of the migrated function.
- Unsafe blocks, if any, reduced to the exact native interaction points.

## Phase 3: Add tests for compatibility behavior

- Add unit tests covering expected outcomes for invalid file descriptors.
- Add positive-path tests only where they can be expressed reliably using standard library file creation and descriptor extraction on Windows.
- Ensure tests are platform-gated rather than forcing cross-platform emulation.
- Verify the module builds cleanly under `cargo test`.

### Deliverables
- `cargo test` passes.
- Tests confirm core compatibility expectations for success/failure paths.

## Phase 4: Final integration and cleanup

- Review naming and visibility so the port matches the crate’s existing internal module conventions.
- Remove any unnecessary helper code added during migration.
- Confirm the module remains narrowly scoped to the original C file and function.
- Perform a final check that no extra facilities or abstractions were introduced beyond what the source module requires.

### Deliverables
- Integrated Rust port on branch `043-module_gnu_msvc_nothrow.c_37-rust-port`
- Minimal, source-aligned implementation ready for follow-on module ports