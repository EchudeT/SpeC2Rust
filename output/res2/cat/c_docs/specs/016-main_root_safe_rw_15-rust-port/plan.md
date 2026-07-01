# Implementation Plan

## Summary

Port the `safe-read.c` module into a focused Rust implementation that preserves the existing low-level safe read/write retry behavior of `safe_rw` without adding new capabilities. The Rust version should provide the same operational role within the main executable path: perform repeated read/write style I/O against a file descriptor-like object, handle short transfers, and propagate terminal errors cleanly.

The implementation should prefer Rust standard library facilities first, while acknowledging that this module is fundamentally rooted in Unix file-descriptor I/O semantics. The technical approach is to translate the existing control flow of `safe_rw` into a small Rust module with explicit loop-based retry logic, careful treatment of partial progress, and narrow error mapping through `std::io::Result`. The port should keep ownership and allocation minimal, avoid introducing abstraction layers not present in the C code, and preserve behavior around interrupted system calls and incomplete transfers as closely as practical.

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**:
  - `std` only by default
  - If direct file-descriptor syscalls are required to match the C behavior exactly, use `libc` minimally and only for the read/write call boundary and errno-aware handling
- **Testing**: `cargo test`
- **Performance Goals**:
  - Maintain equivalent asymptotic behavior to the C implementation
  - Avoid heap allocation in the core transfer path
  - Preserve loop-based partial-transfer handling without unnecessary buffering or copying
  - Keep syscall count behavior aligned with the original retry logic

## Module Mapping

### C to Rust File Mapping

- `safe-read.c` -> `src/main_root_safe_rw_15.rs`
- `include/safe-read.c` -> migrated into Rust module-level documentation/comments and any internal function signature notes; no separate header equivalent beyond Rust item visibility

### Function Mapping

- `safe_rw` -> `pub(crate) fn safe_rw(...) -> std::io::Result<usize>` or equivalent narrow internal signature matching the surrounding Rust port
- duplicate `safe_rw` listing -> treat as the same migrated function, confirming whether one declaration is from include context and one from implementation context

### Integration Placement

Because this is a `main_cluster` module, keep the Rust file under the main crate source tree and expose it only as needed by the existing main-path port. Do not create extra helper modules unless the translated logic requires one small private helper to separate syscall invocation from retry policy.

## Data Model

No dedicated C structs are listed for this module.

### Data-structure Mapping

- No C struct/union/enum migration required
- C pointer-and-length buffer handling -> Rust slices:
  - mutable buffer parameters -> `&mut [u8]`
  - immutable buffer parameters -> `&[u8]`
- C file descriptor integer -> `std::os::fd::RawFd` if staying close to syscall-level behavior
- C integer return/error conventions -> `std::io::Result<usize>`

### Memory Management Notes

- Replace raw pointer arithmetic with slice indexing and explicit offset tracking
- Ensure partial-transfer state is tracked using `usize`
- Avoid unsafe code unless direct syscall interop is necessary; if used, confine unsafe blocks to the syscall boundary only
- Do not allocate new buffers in the port unless the original function semantics require caller-owned storage only

### Error Handling Notes

- Translate errno-driven failure paths into `std::io::Error`
- Preserve retry behavior for interrupt-style transient errors where the C function does so
- Preserve early exit on EOF or non-retryable error according to the original loop logic
- Distinguish between “made progress before error” and “no progress” only if the C function does so; otherwise keep the Rust behavior aligned strictly to the source logic

## Implementation Phases

### Phase 1: Inspect and Fix the Rust Surface for `safe_rw`

- Read both `include/safe-read.c` and `safe-read.c` to determine the exact `safe_rw` signature, especially:
  - whether it abstracts both read and write through a function pointer or mode flag
  - whether it returns signed counts, booleans, or errno-style status
  - how it handles zero-length operations and EOF
- Create `src/main_root_safe_rw_15.rs`
- Define the Rust function signature to match the original caller expectations as closely as possible within idiomatic Rust return types
- Add the module to the main crate with `pub(crate)` visibility only as required

### Phase 2: Port the Retry Loop and Partial-Transfer Logic

- Translate the core `safe_rw` loop directly from C into Rust
- Preserve:
  - retry-on-interruption behavior
  - accumulation of transferred bytes
  - handling of short reads/writes
  - termination conditions
- Use standard-library types first; if exact fd-level behavior is needed, isolate `libc::read`/`libc::write` usage in a minimal private call site
- Keep all buffer advancement logic slice-based and bounds-safe

### Phase 3: Error Semantics and Call-Site Alignment

- Verify the Rust return values and error propagation match the expectations of the surrounding `cat` main-path code
- Adjust any signed/unsigned count conversions carefully
- Confirm no ownership or lifetime issues exist at call sites that previously passed raw buffers
- Keep the module API narrow and avoid introducing wrappers beyond what migration requires

### Phase 4: Tests and Behavioral Verification

- Add unit tests for the translated loop behavior, focusing on:
  - full transfer in one operation
  - partial transfer followed by completion
  - interrupted call retry behavior
  - terminal error propagation
  - zero-byte/EOF behavior if applicable
- Prefer tests using standard file primitives where possible
- If syscall-level mocking is necessary, keep it local to this module and minimal rather than introducing a general framework
- Run `cargo test` and verify the module integrates cleanly with the branch `016-main_root_safe_rw_15-rust-port`