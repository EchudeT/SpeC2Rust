# Implementation Plan

## Summary

Port `gnu/fcntl.c` into a Rust module that preserves the existing module scope and behavior of `dupfd` without adding new facilities. The Rust implementation should remain narrowly focused on file-descriptor duplication semantics and translate C-style integer return/error behavior into idiomatic Rust result handling internally, while still supporting the surrounding project’s expected interface shape.

The implementation approach should:
- map the single C source file to one Rust module file,
- keep logic centered on `dupfd`,
- use standard-library and platform syscall bindings only as needed for descriptor duplication,
- make ownership and lifetime rules explicit so duplicated file descriptors are not accidentally closed or leaked,
- convert OS failures into `std::io::Error` internally and expose the final project-facing API in the smallest form required by the existing codebase.

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**:
  - Rust standard library
  - `libc` crate for direct access to `fcntl`/descriptor constants if standard library APIs are insufficient for exact behavior preservation
- **Testing**: `cargo test`
- **Performance Goals**:
  - preserve system-call-level performance characteristics close to the C implementation,
  - avoid extra allocations,
  - perform only the minimum required validation around file descriptors,
  - keep the Rust wrapper thin over the underlying duplication operation.

## Module Mapping

| C Source | Rust Target | Notes |
|---|---|---|
| `gnu/fcntl.c` | `src/module_gnu_fcntl.rs` | Single-module migration matching existing file scope. |
| `dupfd` | `pub(crate) fn dupfd(...) -> ...` | Preserve function responsibility and behavior; final signature should match the consuming Rust code’s needs while maintaining C semantics. |

## Data Model

This module has no named persistent C structs; the analysis identifies only an anonymous data structure context. The Rust port should therefore avoid inventing new domain types unless required to represent ownership safely.

| C Concept | Rust Mapping | Notes |
|---|---|---|
| anonymous/local integral state | local variables with primitive integer types (`i32`, possibly `c_int`) | Use platform-compatible integer types for file descriptors and `fcntl` arguments. |
| file descriptor | `std::os::fd::RawFd` | Preferred canonical descriptor type in Rust. |
| error reporting via return code / `errno` | `std::io::Result<RawFd>` internally | Convert OS errors via `std::io::Error::last_os_error()`. |

If the surrounding port requires a C-like outward interface, keep that adaptation at the module boundary rather than introducing additional structs or wrappers.

## Implementation Phases

### Phase 1: File and API Skeleton

- Create the Rust module file for `gnu/fcntl.c` under standard project layout.
- Define the Rust equivalent of `dupfd` using `RawFd`/`c_int`-compatible parameters.
- Establish the minimal visibility required by the rest of the crate.
- Document any assumptions about valid descriptor ranges and expected call conventions.

### Phase 2: Core Logic Migration

- Translate the `dupfd` implementation directly from C control flow into Rust.
- Use `libc::fcntl` only if needed to preserve exact duplication semantics; otherwise prefer the closest standard-library-compatible approach.
- Preserve return-path behavior carefully:
  - successful duplication returns the new descriptor,
  - failed duplication returns the corresponding OS error.
- Ensure no ownership confusion:
  - input descriptor remains borrowed/non-owning,
  - returned descriptor is treated as newly acquired and independently closeable by the caller.
- Keep unsafe code narrowly scoped around the syscall boundary, with comments describing preconditions.

### Phase 3: Error Handling and Edge Validation

- Verify translation of invalid descriptor inputs, lower-bound duplication targets, and syscall failure cases.
- Confirm `errno`-derived failures are surfaced correctly through `std::io::Error`.
- Review integer conversions to avoid truncation or signedness mismatches between C and Rust.
- Keep behavior aligned with the original module rather than normalizing or redesigning edge cases.

### Phase 4: Tests and Integration Verification

- Add targeted unit tests for `dupfd` covering:
  - successful duplication of a valid descriptor,
  - failure on invalid descriptor input,
  - behavior when a minimum target descriptor constraint is involved, if present in the original function contract.
- Use `cargo test` to validate the migrated module in isolation and within crate integration.
- Confirm no descriptor leaks in tests by closing duplicated descriptors explicitly or using scoped owned-descriptor helpers only within test code.