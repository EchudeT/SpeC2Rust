# Implementation Plan

## Summary

Port `fadvise.c` into a focused Rust module that preserves the existing responsibility of issuing file access advice through two functions: `fdadvise` and `fadvise`. The Rust implementation should stay close to the C structure and behavior, using thin wrappers over the platform interface where needed and keeping call signatures and control flow simple.

The implementation approach is:

- migrate the C file into a single Rust module under the existing `cat` crate layout;
- represent the two C functions as Rust functions with minimal adaptation;
- use Rust’s standard library file descriptor types where possible;
- handle OS errors through `std::io::Result` and explicit errno-to-`io::Error` conversion when invoking the platform call;
- avoid introducing new abstraction layers beyond what is needed to map the existing functions safely.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**:
  - Rust standard library
  - `libc` crate for `posix_fadvise`/platform constants if direct std support is insufficient
- **Testing**: `cargo test`
- **Performance Goals**:
  - preserve the low-overhead nature of the original C implementation;
  - keep the Rust layer as a thin call-through around the OS advisory interface;
  - avoid extra allocation, buffering, or descriptor ownership changes;
  - maintain equivalent syscall count and equivalent argument passing behavior.

## Module Mapping

| C File | Rust File | Notes |
|---|---|---|
| `fadvise.c` | `src/fadvise.rs` | Direct migration target for `fdadvise` and `fadvise` |
| `fadvise.c` exported functions | `src/fadvise.rs` public or crate-visible functions | Visibility should match actual crate usage, not broadened unnecessarily |

### Function Mapping

| C Function | Rust Function | Migration Notes |
|---|---|---|
| `fdadvise` | `fdadvise(...) -> std::io::Result<()>` | Accept a raw file descriptor or borrowed fd; call platform advisory API directly |
| `fadvise` | `fadvise(...) -> std::io::Result<()>` | Resolve/adapt the Rust file handle input and delegate to `fdadvise` |

## Data Model

This module does not define custom C structs in the provided analysis.

### Data-structure Mapping

| C Type / Structure | Rust Mapping | Notes |
|---|---|---|
| none | none required | Keep implementation function-based |
| file descriptor (`int`) | `std::os::fd::RawFd` or `BorrowedFd<'_>` | Prefer borrowed descriptor where it fits surrounding call sites |
| file offset / length integral types | `libc::off_t` / `i64` / `u64` as required by syscall binding | Final choice should match the exact platform function signature used |
| advice constants | `libc` constants or local typed mapping | Use direct constant mapping; do not invent a richer enum unless required by call sites |

## Implementation Phases

### Phase 1: Establish Rust module skeleton and signature mapping

- Create `src/fadvise.rs`.
- Add Rust equivalents for `fdadvise` and `fadvise`.
- Determine the narrowest practical visibility for both functions based on current crate integration.
- Map C parameter types to Rust types with emphasis on:
  - non-owning descriptor handling;
  - exact integer width for offsets and lengths;
  - preserving current return/error behavior in a Rust-idiomatic form.
- Keep the initial implementation layout close to the source file so migration remains easy to review.

### Phase 2: Port system call logic and error handling

- Implement the advisory call using the platform interface required by the original C logic.
- Use a minimal `unsafe` block only around the syscall boundary.
- Convert the syscall return convention into `std::io::Result<()>`.
- Ensure no descriptor ownership is transferred or dropped accidentally.
- Preserve no-op/success behavior exactly where the C implementation tolerates unavailable or ineffective advisory operations, if present in the source behavior.
- Keep platform-specific constants and conversions local to this module instead of introducing wider infrastructure.

### Phase 3: Integrate callers and validate behavior

- Update the Rust crate module declarations so `fadvise.rs` is compiled and reachable from the existing main cluster code.
- Replace any remaining references to the C implementation with calls to the Rust functions.
- Confirm that argument flow from higher-level `cat` logic to `fadvise` and `fdadvise` remains unchanged.
- Verify compilation on the target Unix-like environment expected by the original module.

### Phase 4: Add tests and finish migration cleanup

- Add unit tests for:
  - successful invocation path using a temporary file;
  - error propagation for invalid file descriptor or invalid arguments where reliably testable;
  - any delegation behavior between `fadvise` and `fdadvise`.
- Prefer tests that validate return handling rather than kernel-specific caching effects.
- Remove obsolete C-side references for `fadvise.c` once Rust replacement is active on the branch.
- Run `cargo test` and fix any portability issues in type conversions or platform imports.