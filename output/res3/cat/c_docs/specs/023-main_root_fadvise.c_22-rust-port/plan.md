# Implementation Plan

## Summary

Port the C module `fadvise.c` into a Rust module that preserves the current role of issuing file-access advisory calls through the two existing functions `fdadvise` and `fadvise`. The Rust implementation should stay narrowly aligned with the source file and function boundaries, replacing C integer/file-descriptor handling and errno-style results with idiomatic Rust signatures while keeping behavior equivalent.

The implementation approach is:

- migrate the single C file into a single Rust source module;
- keep the same two-function structure as the migration anchor;
- use the Rust standard library for file-descriptor ownership and error propagation where possible;
- perform the advisory system call through a minimal OS-facing layer appropriate for Unix targets;
- map C return/error handling to `std::io::Result`;
- avoid adding new abstractions or utility layers beyond what is needed to preserve the existing functionality.

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**:
  - Rust standard library
  - `libc` crate for `posix_fadvise`/platform constants if direct std support is insufficient for the required advisory call
- **Testing**: `cargo test`
- **Performance Goals**:
  - no material overhead beyond the original C implementation for advisory calls;
  - preserve direct file-descriptor-based execution path;
  - avoid unnecessary allocations, buffering, or descriptor wrapping during the call path;
  - maintain constant-time parameter translation and direct OS error propagation.

## Module Mapping

| C File | Rust File | Notes |
|---|---|---|
| `fadvise.c` | `src/fadvise.rs` | Direct migration target for both exported functions |
| `fadvise.c` | `src/main.rs` or existing caller module | Only for updating call sites/imports if this module is invoked from the main binary path |

### Function Mapping

| C Function | Rust Function | Notes |
|---|---|---|
| `fdadvise` | `fdadvise(...) -> std::io::Result<()>` | Primary low-level advisory function operating on a file descriptor or borrowed fd |
| `fadvise` | `fadvise(...) -> std::io::Result<()>` | Wrapper-level function preserving original call layering and argument adaptation |

## Data Model

No user-defined C structs are listed for this module, so the Rust port should not introduce new persistent data structures unless strictly required by signature migration.

### C to Rust Type Mapping

| C Concept | Rust Mapping | Notes |
|---|---|---|
| file descriptor (`int`) | `std::os::fd::BorrowedFd<'_>` or `RawFd` | Prefer `BorrowedFd<'_>` for non-owning descriptor access at function boundaries where practical |
| file offset / length (`off_t`, size-like integers) | `i64`/`u64` as required by syscall interface | Final choice should match target API expectations and preserve signedness where the C API requires it |
| advisory mode constants | integer constants / small Rust enum if needed internally | Do not expose extra abstraction unless needed to map C constants cleanly |
| errno-style failure | `std::io::Error` / `std::io::Result<()>` | Convert OS return codes directly |

## Implementation Phases

### Phase 1: Establish module skeleton and signatures

- Create `src/fadvise.rs` as the direct Rust counterpart to `fadvise.c`.
- Add Rust function stubs for:
  - `fdadvise`
  - `fadvise`
- Choose signatures based on current call patterns in the project:
  - prefer borrowed descriptor input for the low-level path;
  - keep parameter ordering aligned with the C implementation to reduce migration risk.
- Define only the minimal constant/type mappings needed for advisory mode values.
- Wire module visibility/imports so existing callers can be migrated without introducing new modules.

### Phase 2: Port syscall logic and error translation

- Implement the advisory call path in `fdadvise`.
- Use `libc::posix_fadvise` or equivalent minimal Unix binding if standard library APIs do not expose the required operation.
- Translate:
  - successful C return paths to `Ok(())`;
  - nonzero/negative OS outcomes to `Err(std::io::Error::from_raw_os_error(...))`.
- Port `fadvise` as the wrapper that performs any file-to-fd extraction or argument normalization present in the C source.
- Preserve edge-case behavior from the C implementation, especially:
  - invalid descriptor handling;
  - offset/length conversion boundaries;
  - advisory mode pass-through.

### Phase 3: Update integration points and compile the branch

- Replace C-module references with Rust-module imports in the existing main/binary code path.
- Update call sites to handle `std::io::Result<()>` explicitly.
- Keep the migration local to the existing file/function responsibility; do not generalize advisory handling into shared infrastructure.
- Verify Unix-specific imports and cfg usage if the project targets multiple platforms.
- Ensure the crate builds cleanly on the target branch with this module replacing its C counterpart.

### Phase 4: Add focused tests and finalize behavioral parity

- Add unit tests for parameter/error behavior that can be validated without expanding scope.
- Add targeted tests for:
  - invalid fd error propagation;
  - successful invocation path on a temporary file where supported;
  - wrapper-to-low-level delegation consistency.
- Run `cargo test`.
- Compare migrated behavior against the original C expectations at the function boundary level and adjust only mismatches directly tied to the original module semantics.

## Memory Management and Error Handling Notes

- This module should remain allocation-free in normal operation.
- Prefer borrowed file-descriptor access to avoid accidental ownership transfer or descriptor closure.
- Keep all unsafe usage tightly scoped to the OS call boundary if `libc` is used.
- Validate integer conversions explicitly when crossing from Rust numeric types to syscall parameter types.
- Do not suppress OS errors; return them directly as `std::io::Error` so caller behavior remains transparent.