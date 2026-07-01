# Implementation Plan: main_root_stat_03

## Summary

This module ports the `cat` entry path represented by `cat.c` and the small file-control helper in `fcntl.c` into Rust, preserving the existing control flow and system-call-facing behavior without adding new features. The Rust implementation should keep the module boundary tight: one executable entry module for `main`, and one helper module for the `klibc_fcntl` logic.

The technical approach is a direct migration of existing file and function responsibilities into idiomatic but restrained Rust:
- translate `main` into a Rust `fn main()` with explicit argument handling and exit-status control,
- migrate `klibc_fcntl` into a narrow Rust helper function,
- use the Rust standard library for file descriptor and I/O handling where possible,
- use `std::os::unix` facilities for Unix-specific descriptor access,
- represent C anonymous structures only if they correspond to actual persisted state needed by the translated functions; otherwise keep them as local Rust values rather than inventing extra types.

Particular care should be taken around ownership of file descriptors, preserving non-owning vs owning behavior, and mapping C integer/error-return conventions into `Result`-based Rust code with explicit process exit codes in `main`.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**:
  - Rust standard library
  - No third-party crates recommended based on the provided input
- **Testing**: `cargo test`
- **Performance Goals**:
  - Maintain behavior and runtime characteristics comparable to the C implementation for normal command execution
  - Avoid unnecessary allocations in argument and descriptor handling
  - Keep file-descriptor operations as thin wrappers over standard-library/Unix OS interfaces
  - Preserve streaming-style I/O behavior appropriate for a small command-line utility

## Module Mapping

### C to Rust File Mapping

- `cat.c`
  - Rust target: `src/main.rs`
  - Responsibility:
    - executable entry point
    - argument processing
    - dispatch into existing translated logic
    - exit-code handling

- `fcntl.c`
  - Rust target: `src/fcntl.rs`
    - migration of `klibc_fcntl`
    - narrow file-descriptor control helper used by the entry path

### Function Mapping

- `main`
  - C: `main`
  - Rust: `fn main()`
  - Notes:
    - convert C argc/argv handling into `std::env::args_os()`
    - keep top-level orchestration in `main`
    - move any reusable internal parsing only if required by direct translation, without introducing extra abstraction layers

- `klibc_fcntl`
  - C: `klibc_fcntl`
  - Rust: `fn klibc_fcntl(...) -> std::io::Result<...>` or minimal equivalent
    - preserve command-specific behavior and integer flag semantics
    - use Unix-specific descriptor APIs from the standard library where sufficient
    - if exact `fcntl` command coverage cannot be expressed via safe std APIs alone, keep the implementation minimal and scoped only to the cases already exercised by this module

## Data Model

The input identifies only anonymous C structures. Because no named persistent data model is evident, the Rust port should avoid inventing broad replacement types. Map structures only where the translated code requires retained state.

### Data-structure Mapping

- `anonymous`
  - Rust mapping: local tuple / local binding / small private struct only if necessary
  - Guidance:
    - if the C anonymous struct is used only as a temporary grouping of fields inside a function, represent it with local variables in Rust
    - if multiple fields must be passed together between translated functions, introduce a private `struct` in the corresponding Rust file with the smallest exact field set

  - Rust mapping: `enum` only if the C usage represents tagged mode/command selection
    - otherwise prefer primitive integers or booleans matching original semantics

  - Rust mapping: no dedicated type unless required for correctness/readability of the direct port
    - do not create public data types for implementation-only C layout artifacts

### C-to-Rust Representation Rules

- C integer return codes used for success/failure:
  - Rust: `Result<T, std::io::Error>` internally, converted to process exit code in `main`
- C file descriptors:
  - Rust: `RawFd`, `BorrowedFd<'_>`, or `OwnedFd` depending on ownership
  - Use borrowed descriptors by default when the Rust code must not assume close responsibility
- C flags/commands:
  - Rust: primitive integer types matching platform expectations (`libc`-style values only if unavoidable, but avoid adding crate dependencies unless exact translation requires it)
- C strings/argv:
  - Rust: `OsString`/`OsStr` from `std::env::args_os()` to preserve non-UTF-8 arguments
- C nullability:
  - Rust: `Option<T>` only where the original code has meaningful absence semantics

## Implementation Phases

## Phase 1: Establish crate layout and translate the entry path

### Goals
- Create the minimal Rust executable structure
- Port `main` from `cat.c` into `src/main.rs`
- Preserve argument flow and exit behavior

### Tasks
- Create `src/main.rs`
- Translate C `main` signature and argc/argv access into Rust argument collection via `std::env::args_os()`
- Preserve existing branching and status-code behavior as directly as possible
- Keep any helper logic inside `main.rs` unless it is the direct translation target of `fcntl.c`
- Define a small internal error-to-exit mapping strategy so `main` remains the only place that terminates the process

### Notes
- Avoid introducing a library crate unless required by the existing project layout
- Keep all logic single-threaded and process-local
- Prefer explicit control flow over abstraction during the initial migration

## Phase 2: Port `klibc_fcntl` and integrate Unix descriptor handling

### Goals
- Translate `fcntl.c` into a narrowly scoped Rust helper module
- Preserve descriptor manipulation semantics used by the module

### Tasks
- Create `src/fcntl.rs`
- Port `klibc_fcntl` into a Rust function with a minimal signature aligned to the translated call sites
- Use `std::os::unix::io` / `std::os::fd` types for descriptor access
- Ensure ownership is explicit:
  - borrowed when acting on externally managed descriptors
  - owned only if the translated logic takes responsibility for descriptor lifetime
- Return `std::io::Result<_>` from the helper and convert errors at the caller boundary

### Notes
- Limit implementation strictly to the `fcntl` behaviors actually needed by `main_root_stat_03`
- Do not generalize into a broader file-control utility layer
- If platform-specific raw calls are unavoidable, keep them isolated inside `src/fcntl.rs`

## Phase 3: Resolve data mappings and error semantics

### Goals
- Replace anonymous C data usages with minimal Rust equivalents
- Finish correctness work around ownership, lifetimes, and error propagation

### Tasks
- Review each anonymous C structure usage and decide:
  - local variables only,
  - small private struct,
  - or simple enum for command/mode selection
- Replace integer sentinel patterns with `Option` or `Result` where doing so does not change behavior
- Verify all descriptor paths avoid double-close and do not create accidental ownership transfer
- Ensure all C-style error returns are mapped consistently into `std::io::Error` or explicit exit codes
- Keep module interfaces private unless cross-file access is required

### Notes
- This phase should reduce unsafe/system-facing surface area rather than expand it
- Preserve exact operational behavior over stylistic refactoring

## Phase 4: Add focused tests and finalize migration

### Goals
- Confirm the ported module compiles and preserves expected behavior
- Add restrained tests aligned to the migrated functions

### Tasks
- Add unit tests for `klibc_fcntl` only where deterministic behavior can be exercised safely
- Add argument/exit-path tests for translated `main` logic by extracting only the minimal testable internal function if needed
- Run `cargo test`
- Validate formatting and lint cleanliness insofar as they do not force structural rewrites

### Notes
- Keep tests centered on migrated behavior from existing files
- Do not add benchmarking, integration harness expansion, or speculative edge-case facilities