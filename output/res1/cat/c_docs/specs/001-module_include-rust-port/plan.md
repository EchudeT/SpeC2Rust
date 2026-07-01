# Implementation Plan

## Summary

Port `include/safe-read.c` into an idiomatic Rust module that preserves the existing low-level read/write retry behavior of `safe_rw` without adding new capabilities. The Rust implementation should keep the same operational role: performing file-descriptor I/O with careful handling of partial transfers and interrupted system calls, while expressing ownership, buffer access, and error propagation through Rust types.

The implementation should prefer the Rust standard library where possible, but because `safe_rw` is centered on Unix file descriptors and syscall-style semantics, the port will likely require direct Unix bindings for `read`/`write` behavior that matches the C logic. The Rust module should therefore focus on a narrow translation of the existing function into a small internal module, with explicit handling for retryable errors, byte counts, and buffer slicing. Memory safety should come from Rust slice APIs rather than manual pointer arithmetic.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**:
  - Rust standard library
  - `libc` crate for direct Unix `read`/`write` access and errno-based error inspection, if standard-library abstractions cannot preserve the original `safe_rw` semantics closely enough
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve syscall-level behavior comparable to the C implementation
  - Avoid unnecessary allocations and data copies
  - Use borrowed buffers/slices and direct file-descriptor operations
  - Maintain efficient handling of partial reads/writes and interrupted calls

## Module Mapping

### C to Rust File Mapping

- `include/safe-read.c` → `src/module_include.rs`

### Function Mapping

- `safe_rw` → `pub(crate)` or `pub` Rust function in `src/module_include.rs`, depending on call-site needs within the ported project

### Rust Module Shape

Use a single Rust source file for this module to match the current scope of the C input and avoid introducing extra structure:

```text
src/
  module_include.rs
```

If the broader crate already uses `mod` declarations from `src/lib.rs` or `src/main.rs`, expose it there with a direct module declaration only.

## Data Model

No named C structs were identified in this module.

### Function-Level Type Mapping

Because this module is function-oriented rather than struct-oriented, the key data-model work is parameter and return-type translation:

- C file descriptor (`int`) → `std::os::fd::RawFd`
- C byte buffer pointer (`void *`, `char *`, `unsigned char *`) → `&mut [u8]` for reads or `&[u8]` for writes
- C byte count (`size_t`) → `usize`
- C transferred byte count / syscall result (`ssize_t`) → `isize` internally when matching syscall results, then converted to `usize` or error type safely
- C errno-based failure reporting → `std::io::Result<usize>` or a small internal result shape if the original function must distinguish retry/EOF/error states exactly

### Error Handling Mapping

- Interrupted syscall handling (`EINTR`) should be implemented explicitly in the retry loop
- Other syscall failures should map to `std::io::Error::last_os_error()`
- Partial transfer handling should use slice offsetting rather than pointer arithmetic
- Any sentinel-style C return conventions should only be preserved where required by existing callers; otherwise use `std::io::Result`

## Implementation Phases

## Phase 1: Establish module skeleton and function signature

- Create `src/module_include.rs`
- Add the Rust equivalent of `safe_rw` with a signature derived from actual call requirements in the port
- Choose the narrowest visibility needed by existing migrated callers
- Decide whether the function should expose:
  - separate read/write operation selection via enum/flag translation, or
  - a direct one-to-one signature mirroring the C function shape as closely as practical
- Add the module declaration in the crate root (`src/lib.rs` or `src/main.rs`) without introducing extra helper modules unless they are required for this function to compile cleanly

## Phase 2: Port syscall loop and buffer handling

- Translate the core `safe_rw` loop into Rust
- Replace manual pointer arithmetic with slice splitting/offset tracking
- Implement retry behavior for interrupted syscalls
- Preserve partial-transfer semantics exactly as needed by callers
- Use `libc::read`/`libc::write` only if `std` abstractions cannot represent the original file-descriptor behavior closely enough
- Keep all unsafe code tightly scoped around the syscall boundary, with safe buffer-length accounting outside the unsafe block
- Validate conversions between `usize`, `isize`, and syscall return values to avoid truncation or sign errors

## Phase 3: Integrate error semantics and caller compatibility

- Align the Rust return behavior with the expectations of the existing project logic
- Ensure EOF, zero-byte transfers, and hard errors are represented consistently
- Confirm that any original C assumptions about retry termination or total bytes processed are preserved
- Adjust call sites, if already ported, to use the Rust result type and buffer borrowing model without adding compatibility layers beyond what is necessary

## Phase 4: Add focused tests and finalize migration

- Add unit tests for:
  - successful transfer
  - partial transfer accumulation behavior, if applicable
  - interrupted syscall retry behavior where testable
  - error propagation for invalid file descriptors or closed pipe ends
  - zero-length buffer handling
- Prefer standard Unix primitives available through the standard library for tests (for example, pipes or temporary files), keeping tests close to the original behavior under `cargo test`
- Review the final implementation for:
  - minimal unsafe scope
  - no unnecessary allocation
  - correct lifetime and slice usage
  - parity with the original C function’s operational semantics