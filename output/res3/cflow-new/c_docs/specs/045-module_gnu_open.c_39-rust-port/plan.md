# Implementation Plan: module_gnu_open.c_39

## Summary

Port `gnu/open.c` into an idiomatic Rust module that preserves the behavior of `orig_open` while narrowing the implementation scope to the existing file and function only. The Rust port should map the C open-wrapper logic to standard-library file opening primitives where possible, and use a minimal Unix-specific path only if exact flag-oriented behavior from the C implementation requires lower-level access.

The implementation should prioritize:
- direct migration of the existing `orig_open` behavior,
- explicit ownership and lifetime handling for path and file resources,
- precise propagation of OS-level open failures through Rust error types,
- no expansion beyond the current module boundary.

## Technical Context

### Language/Version
- Rust stable, edition 2021
- Minimum recommended compiler: Rust 1.76+

### Primary Dependencies
- Rust standard library
- No third-party crates by default
- If exact C-style flag handling is required by the migrated logic, use `std::os::unix` facilities before considering any external crate

### Testing
- `cargo test`

### Performance Goals
- Preserve the current module’s operational complexity and system-call profile as closely as practical
- Avoid unnecessary heap allocation during path and option preparation
- Keep wrapper overhead negligible relative to the underlying `open`/file-open system call
- Maintain equivalent error-path cost to the C implementation

## Module Mapping

### C to Rust File Mapping
- `gnu/open.c` -> `src/module_gnu_open.rs`

### Function Mapping
- `orig_open` -> `orig_open`

### Rust Module Shape
The Rust module should remain narrowly scoped around the migrated function:
- one module file for the ported logic,
- one public or crate-visible function matching the project’s existing internal call pattern,
- helper functions only if needed to isolate varargs/flag translation behavior from the main open path.

## Data Model

The analysis reports two anonymous C data structures. Because no named fields or usage details are provided, the Rust plan should avoid inventing new public models. Data representation should be derived only from actual use in `gnu/open.c`.

### Mapping Strategy
- anonymous C struct/union used only as temporary local state -> Rust local variables or a private struct with named fields based on observed usage
- anonymous bit/flag carrier -> Rust integer flag variables or a private enum if the C code expresses a closed set of modes

### Expected Rust Representations
- C path parameters (`char *`, `const char *`) -> `&Path`, `&OsStr`, or `PathBuf` only when ownership is required
- C file descriptor return values (`int`) -> `RawFd` or `std::fs::File`, depending on the surrounding project API needs
- C mode/flag integers -> `i32`/platform flag type, kept private to the module
- nullable/optional C values -> `Option<T>`

### Memory Management Notes
- Eliminate manual resource tracking where ownership can be expressed through `File`
- If the function must return or operate on raw file descriptors, ensure descriptor ownership transfer is explicit and does not cause double-close conditions
- Avoid temporary C-string ownership patterns unless exact syscall interop requires them

### Error Handling Notes
- C error signaling via negative return codes / `errno` -> `std::io::Result<_>` internally
- If the surrounding Rust project requires C-like signatures, convert at the boundary only, while keeping internal logic result-based
- Preserve OS error identity by forwarding `std::io::Error::last_os_error()` or equivalent standard-library errors

## Implementation Phases

### Phase 1: Source Analysis and Signature Freeze
- Inspect `gnu/open.c` and determine the exact `orig_open` signature, including:
  - path parameter type,
  - flag parameters,
  - whether an optional mode argument is consumed,
  - return type and error convention.
- Identify whether the anonymous data structures are materially used or are compiler artifacts not requiring direct translation.
- Decide the Rust function signature based on the existing call sites in the port branch, keeping the interface as close as possible to current project needs.
- Record any Unix-specific requirements that prevent a pure `std::fs::OpenOptions` implementation.

### Phase 2: Core Port of `orig_open`
- Create `src/module_gnu_open.rs`.
- Implement the `orig_open` logic with a narrow translation of:
  - path handling,
  - open flag interpretation,
  - optional mode handling if present in the C version,
  - direct error propagation.
- Prefer standard library file-opening APIs if they can express the required behavior.
- If exact behavior depends on low-level open flags, isolate that translation in a small private helper rather than broadening the module surface.
- Keep the implementation limited to the single migrated function and only the private helpers strictly required to support it.

### Phase 3: Integration and Type/Ownership Validation
- Wire the new module into the crate using standard Rust module declarations only.
- Adjust the return type boundary as needed:
  - use `File` if ownership should stay managed in Rust,
  - use `RawFd` only if existing migrated callers require descriptor-level interoperation.
- Verify that no descriptor leaks occur on success or failure paths.
- Validate that path conversion does not introduce lossy UTF-8 assumptions.

### Phase 4: Tests and Behavioral Verification
- Add focused unit tests for the migrated behavior using `cargo test`.
- Cover:
  - successful open of an existing file,
  - failure on missing path,
  - permission-denied or invalid-flag-related failure where reproducible,
  - mode-sensitive creation behavior only if the original function supports creation flags.
- Confirm that error mapping preserves OS-level failure semantics rather than replacing them with custom abstractions.
- Keep tests local to the migrated module and avoid adding unrelated infrastructure.