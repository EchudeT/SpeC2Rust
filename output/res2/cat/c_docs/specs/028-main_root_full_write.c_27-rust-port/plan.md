# Implementation Plan: main_root_full-write.c_27

## Summary

This module ports the C file `full-write.c` into a Rust module that preserves the existing low-level write behavior concentrated in `full_rw`. The Rust implementation should keep the same narrow responsibility: repeatedly perform I/O until the requested transfer is complete or a terminal error/EOF-like condition is reached, while correctly handling partial progress and retryable interruption cases.

The implementation approach should stay close to the original control flow rather than redesigning the behavior. In Rust, the core logic should be expressed with safe slice handling and `std::io` result types where possible, while still preserving the C function’s semantics around byte counts, retry conditions, and error propagation. Ownership and bounds safety will be handled by Rust slices and integer conversions, replacing manual pointer arithmetic.

## Technical Context

- **Language/Version**: Rust 1.77+
- **Primary Dependencies**: Rust standard library only (`std::io`, `std::cmp`, basic numeric conversions)
- **Testing**: `cargo test`
- **Performance Goals**:
  - Match the C implementation’s linear, low-overhead loop structure.
  - Avoid unnecessary allocations or buffering.
  - Preserve partial-write progress behavior.
  - Use direct slice advancement instead of copied intermediate buffers.

## Module Mapping

| C File | C Function | Rust Target |
|---|---|---|
| `full-write.c` | `full_rw` | `src/full_write.rs::full_rw` |

### Rust module placement

- Add a dedicated Rust source file: `src/full_write.rs`
- Expose the migrated function through the existing crate module tree only as needed by the current port branch.
- Do not split the logic into extra helper modules unless required by an existing project structure.

## Data Model

This module has no standalone C struct definitions to migrate.

### Function-level type mapping

| C Concept | Rust Mapping |
|---|---|
| raw buffer pointer + length | `&[u8]` / `&mut [u8]` as appropriate to the migrated call pattern |
| byte count return value (`size_t`/`ssize_t`-style behavior) | `usize` for successful transferred-byte counts |
| error signaling via negative return / `errno` | `std::io::Result<usize>` or equivalent crate-local error return expected by surrounding code |
| function pointer or operation selector, if present in original implementation | explicit Rust generic/function parameter only if required by the original function signature; otherwise keep implementation local and direct |

### Memory and error-handling decisions

- Replace pointer arithmetic with slice slicing (`buf = &buf[n..]`) to preserve progress safely.
- Keep transfer counters in `usize`; use checked or explicit conversions only where Rust APIs require them.
- Map interrupted system call behavior to retry on `ErrorKind::Interrupted`.
- Preserve partial-progress semantics: once bytes have been transferred, return that progress unless the original module contract clearly requires a hard error.
- Avoid unsafe code unless the surrounding crate already exposes only raw file-descriptor APIs and there is no standard-library alternative compatible with the existing interface.

## Implementation Phases

### Phase 1: Inspect and define the Rust surface

- Review the exact C signature and call pattern of `full_rw` in `full-write.c`.
- Identify whether the function is specialized for writes only or shared between read/write-style callbacks.
- Determine the narrowest Rust signature that matches existing crate usage without introducing new abstractions.
- Create `src/full_write.rs` and declare the migrated function with crate-visible visibility matching current usage.

### Phase 2: Port the transfer loop

- Translate the core loop from `full_rw` directly into Rust.
- Preserve:
  - repeated operation until requested length is satisfied,
  - handling of partial transfers,
  - retry on interruption,
  - stop conditions for zero-length progress or terminal error.
- Use slices and offset tracking instead of manual pointer updates.
- Keep the implementation allocation-free and focused on the original function only.

### Phase 3: Integrate error and return semantics

- Align the Rust return type with the surrounding crate’s conventions while preserving C behavior as closely as possible.
- Ensure terminal errors are propagated correctly.
- Ensure partial successful progress is not lost when an error occurs after some bytes were transferred, if that matches the original contract.
- Verify integer handling for large buffer lengths and transferred-byte accumulation.

### Phase 4: Add focused tests and complete module wiring

- Add unit tests covering:
  - complete transfer in one operation,
  - multiple partial transfers,
  - interruption followed by success,
  - zero-progress termination behavior if applicable,
  - error before any progress,
  - error after partial progress.
- Run `cargo test`.
- Wire the module into the crate with the minimal necessary `mod`/`use` changes, without introducing additional support layers.