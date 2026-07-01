# Implementation Plan: `main_root_fadvise.c_22`

## Summary

Port the C file `fadvise.c` into a focused Rust module that preserves the existing advisory file-access behavior used by the `cat` program. The Rust implementation should migrate the two existing functions, `fdadvise` and `fadvise`, without adding broader abstractions or new features.

The technical approach is to keep the logic close to the C structure:

- represent the module as a small Rust source file under the existing binary crate layout,
- preserve the split between descriptor-based advisory handling and higher-level file-path/file-handle invocation if both are present in current call sites,
- use safe Rust where possible,
- isolate any required OS-specific advisory call behind a minimal internal function boundary,
- map C integer/file-descriptor error signaling into Rust `Result`-based handling while preserving caller-visible behavior.

Because file access advice is platform-sensitive and often optional, the port should prefer compile-time conditional support and no-op behavior only where that matches current project behavior. The implementation should avoid introducing new policy and should only migrate what the current C module already does.

## Technical Context

### Language / Version

- Rust stable, edition 2021
- Minimum target: Rust 1.74 or newer

### Primary Dependencies

Use the Rust standard library by default.

Recommended crates only if already used elsewhere in the project for low-level Unix bindings:

- `libc`: for direct access to `posix_fadvise`/related constants if standard library support is insufficient for the target behavior

No additional crates should be introduced unless required by existing project conventions for Unix syscall bindings.

### Testing

- `cargo test`

Testing focus:

- unit tests for argument translation and return-value/error mapping where feasible,
- platform-gated tests for advisory calls if the project already permits such tests,
- regression validation that unsupported or ignored advice paths match current behavior.

### Performance Goals

- Preserve the negligible overhead profile of the original C module.
- Avoid heap allocation in the advisory path.
- Keep the Rust implementation as a thin wrapper around OS advisory facilities or equivalent existing project utilities.
- Maintain identical or near-identical call frequency and control flow relative to the C version.

## Module Mapping

### C to Rust File Mapping

- `fadvise.c` → `src/.../fadvise.rs` or integrated into the existing `src/main.rs`-adjacent module structure used by this `cat` port

The exact destination should follow the current Rust project layout for this branch, but should remain a single Rust module corresponding directly to the original C file.

### Function Mapping

- `fdadvise` → `fn fdadvise(...) -> Result<(), std::io::Error>` or internal equivalent returning a lightweight status
- `fadvise` → `fn fadvise(...) -> Result<(), std::io::Error>` or caller-shape equivalent matching existing Rust-side invocation needs

If the surrounding Rust port expects C-like behavior instead of `Result`, these functions may return simple status values internally, but error handling should still be expressed using Rust types within the implementation and converted only at the module boundary.

## Data Model

This module analysis reports no dedicated C structs.

### Data-structure Mapping

- No C struct mappings required.
- C scalar types should map as follows, depending on actual function signatures in the source:
  - file descriptor `int` → `std::os::fd::RawFd`
  - offsets/lengths such as `off_t` → `i64` or platform-correct alias via `libc::off_t` when syscall ABI compatibility is required
  - advisory mode constants `int`/macros → Rust `const` values or direct use of `libc` constants

### Memory Management

- No owned heap-backed data structures are expected for this module.
- Borrow existing file handles or pass raw file descriptors without taking ownership unless the surrounding Rust code already owns the descriptor object.
- Avoid descriptor lifecycle changes; this module should not open, duplicate, or close file descriptors unless the original C function already does so.

### Error Handling

- Convert OS error returns into `std::io::Error`.
- Preserve any C behavior where advice failures are intentionally ignored or downgraded, if that is what the original module does.
- Keep unsupported-platform handling explicit with `cfg` gates rather than hidden fallback abstractions.

## Implementation Phases

### Phase 1: Source Signature and Behavior Extraction

- Inspect `fadvise.c` and identify the exact signatures, constants, and call relationships for `fdadvise` and `fadvise`.
- Confirm whether `fadvise` is a wrapper over `fdadvise` or whether both expose distinct entry points.
- Identify all external dependencies from the C file:
  - system headers,
  - advisory constants,
  - file descriptor types,
  - any project-local helpers.
- Determine current error semantics:
  - whether failures are ignored,
  - whether diagnostics are emitted,
  - whether return codes propagate to callers.

Deliverable:

- a Rust module skeleton with matching function names/signatures adapted to existing Rust project conventions, but no expanded functionality.

### Phase 2: Core Rust Port of Advisory Logic

- Implement the low-level descriptor-based advisory path first (`fdadvise`).
- Use standard-library Unix descriptor types where possible.
- If direct advisory syscalls are needed, use the narrowest possible `libc` call surface.
- Port the higher-level `fadvise` function as a direct wrapper or companion function following the original control flow.
- Preserve constant mappings and any conditional execution logic from the C source.
- Keep unsafe code, if required, minimal and localized around the syscall boundary with clear invariants:
  - valid descriptor passed by caller,
  - offset/length types mapped correctly,
  - no ownership transfer.

Deliverable:

- working Rust implementation of `fdadvise` and `fadvise` with behavior aligned to the C file.

### Phase 3: Integration into the `cat` Rust Branch

- Replace or wire up the existing C-derived call sites to use the new Rust module.
- Ensure module visibility is limited to what current call sites require.
- Remove any temporary stubs used during migration.
- Validate that return handling at call sites matches previous C behavior, especially if advisory failures are non-fatal.

Deliverable:

- the `cat` branch builds with the Rust module in place of the C implementation for this area.

### Phase 4: Tests and Behavioral Verification

- Add unit tests for any pure decision logic or parameter mapping present in the port.
- Add platform-gated tests for successful invocation on valid file descriptors where practical.
- Verify failure-path behavior for invalid descriptors or unsupported advice modes only to the extent observable in the original C semantics.
- Run `cargo test` and fix any portability issues caused by platform-specific advisory APIs.

Deliverable:

- passing tests and documented platform gating consistent with the original module scope.

## Notes and Constraints

- Keep the implementation limited to migrating `fadvise.c` and its two functions.
- Do not introduce broader I/O utility layers, descriptor wrapper frameworks, or cross-module abstractions beyond what is strictly needed for this file.
- Prefer standard Rust module organization and existing project patterns over creating new infrastructure.
- Match original behavior closely, especially around non-fatal advisory calls and OS-dependent support.