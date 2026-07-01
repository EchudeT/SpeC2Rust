# Implementation Plan

## Summary

Port `binary-io.c` into a focused Rust module that preserves the existing responsibility of `set_binary_mode` without introducing additional abstractions or new behavior. The Rust implementation should map the current platform-dependent binary/text stream handling into a minimal, idiomatic interface located in the main binary crate, using conditional compilation for platforms where binary mode matters and no-op behavior where it does not.

The technical approach is to migrate the single C function into a small Rust module with the narrowest public surface needed by the existing main-cluster code. Error handling should be explicit through `Result` only if the original call path can observe failure; otherwise, preserve current semantics with a side-effecting helper and platform-gated implementation details. Ownership and memory concerns are minimal because the C module does not define persistent data structures; the main migration concern is safe handling of standard stream descriptors/handles and avoiding unsafe code unless required by platform APIs.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**:
  - Rust standard library
  - No third-party crates recommended based on current module evidence
- **Testing**: `cargo test`
- **Performance Goals**:
  - Negligible overhead versus the C implementation
  - No additional buffering layers or dynamic allocation in the binary-mode setup path
  - Preserve startup-time characteristics for the main binary path
  - Keep platform branching compile-time via `cfg` where possible

## Module Mapping

- **C source file**
  - `binary-io.c`

- **Rust target module**
  - `src/binary_io.rs`

- **Function mapping**
  - `set_binary_mode` -> `set_binary_mode`

- **Integration location**
  - Expose the Rust module from the main crate with a standard `mod binary_io;`
  - Update the current main-cluster call sites to invoke `crate::binary_io::set_binary_mode(...)` or equivalent narrow path matching existing usage

## Data Model

This module has no declared C structs or owned data models to migrate.

### C to Rust mapping

- **C module-level procedural logic**
  - Free function(s) -> Rust free function(s) in `src/binary_io.rs`

- **C stream/file descriptor inputs**
  - If the original function operates on standard streams or raw descriptors, map to:
    - `std::os::fd::RawFd` on Unix
    - `std::os::windows::io::RawHandle` or platform-specific internal handling on Windows
  - Prefer keeping the function signature aligned with current Rust call sites rather than introducing new wrapper types

- **Error representation**
  - C integer/status result -> Rust `Result<(), std::io::Error>` if failure is observable and propagated
  - C no-op / ignored status semantics -> Rust `fn set_binary_mode(...)` with internal best-effort handling only if that matches current behavior

## Implementation Phases

### Phase 1: Module skeleton and signature alignment

- Create `src/binary_io.rs`
- Add the Rust equivalent of `set_binary_mode` with a signature matching the existing main-cluster migration needs
- Wire the module into the crate with the minimum required `mod` declaration and imports
- Decide the return type based strictly on how the original C function reports errors and how callers use the result
- Avoid introducing helper submodules or broader I/O abstractions

### Phase 2: Platform-specific behavior migration

- Implement `set_binary_mode` using conditional compilation
- On platforms where binary/text mode distinction is irrelevant, implement a no-op path
- On Windows, implement the binary-mode switch with the smallest necessary platform interaction
- Keep any required `unsafe` blocks tightly scoped and documented around the exact OS call boundary
- Ensure no heap allocation or long-lived state is introduced

### Phase 3: Call-site migration and behavior preservation

- Replace references to the C implementation in the main execution path with the Rust module function
- Preserve invocation order relative to existing stdin/stdout setup logic
- Verify that any previous status checking, ignored failures, or fatal handling remains semantically aligned
- Remove only the migrated dependency path for `binary-io.c` from the Rust build path without broad project restructuring

### Phase 4: Tests and validation

- Add unit tests for compile-time platform behavior where feasible
- Add focused tests for:
  - no-op behavior on non-Windows targets
  - stable function invocation without panics
  - error-path behavior only if the function exposes `Result`
- Keep tests lightweight and limited to this module’s observable behavior
- Run `cargo test` to confirm integration with the main crate

## Notes on Memory Management and Error Handling

- This module does not require heap-managed data structures; prefer stack-only values and direct calls
- Avoid storing global mutable state
- If platform APIs require raw handles/descriptors, borrow them transiently and do not assume ownership
- Use `Result` only when the original semantics require propagating setup failures; otherwise keep behavior minimal and caller-compatible
- Any `unsafe` use must be limited to unavoidable OS interop and isolated within the platform-specific implementation