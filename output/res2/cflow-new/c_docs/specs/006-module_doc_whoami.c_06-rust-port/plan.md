# Implementation Plan: module_doc_whoami.c_06

## Summary

Port `doc/whoami.c` into a Rust module that preserves the existing `who_am_i` behavior and keeps the implementation narrowly aligned with the original source file. The Rust work should focus on translating the current control flow, string handling, and any process- or user-identity lookup logic into safe standard-library Rust where possible.

The implementation should avoid adding new capabilities or reshaping the module beyond what is required for the migration. Any C patterns relying on implicit lifetime management, null-terminated strings, or integer status returns should be converted into explicit Rust ownership, UTF-8-aware string handling, and `Result`-based error propagation at internal boundaries. If the original logic depends on OS user information that is not exposed directly by the Rust standard library, keep the dependency choice minimal and scoped only to that lookup.

## Technical Context

### Language/Version
- Rust 1.78 or newer

### Primary Dependencies
- Rust standard library
- No third-party crates by default
- If `who_am_i` requires obtaining the current user identity in a way not supported by `std`, use:
  - `libc` for direct POSIX-compatible user lookup interop
- Do not introduce additional helper crates unless the original C behavior cannot be reproduced otherwise

### Testing
- `cargo test`

### Performance Goals
- Match the C module’s operational profile for a single identity lookup or formatted output path
- Avoid unnecessary heap allocations beyond required Rust string ownership conversions
- Keep syscall/library-call count equivalent to the C implementation’s behavior
- Preserve straightforward constant-space processing aside from output string storage

## Module Mapping

### C to Rust File Mapping
- `doc/whoami.c` -> `src/module_doc_whoami.rs`

### Function Mapping
- `who_am_i` -> `pub(crate) fn who_am_i(...) -> Result<..., ...>` or `fn who_am_i(...)` with the narrowest signature compatible with the surrounding Rust project structure

### Integration Notes
- Keep the Rust module focused on the migrated file only
- Do not split the logic into extra submodules unless required by Rust compilation boundaries
- If the original function prints directly, preserve that behavior in the nearest Rust-equivalent form rather than introducing a new abstraction layer

## Data Model

### C Structure Mapping
- `anonymous` -> eliminate if it is only a local aggregate with no cross-function role; otherwise map to a private Rust `struct`

### Type Conversion Guidance
- C strings (`char *`, fixed character buffers) -> `String`, `&str`, or `std::ffi::{CString, CStr}` where OS interop is required
- Integer status codes -> `Result<T, E>` internally; convert to process-facing exit behavior only at the module boundary if needed
- Nullable pointers -> `Option<T>` or checked raw pointers within isolated `unsafe` blocks
- Stack buffers used for identity/name retrieval -> owned Rust buffers or `Vec<u8>` only if required by a libc API

### Memory Management Notes
- Replace manual buffer management with owned Rust values
- Contain any required `unsafe` code to the smallest possible interop section
- Validate pointer results and string termination before conversion from C APIs
- Avoid exposing raw pointers outside the lookup boundary

## Implementation Phases

### Phase 1: Inspect and Define the Direct Port Boundary
- Review `doc/whoami.c` and identify the exact responsibilities of `who_am_i`
- Determine whether it:
  - returns a name,
  - prints directly,
  - reads process/user metadata,
  - uses anonymous local structs for temporary state
- Define the Rust function signature to mirror the original usage as closely as possible
- Identify any C library calls that must be replaced with `std` or `libc`

### Phase 2: Translate Core Logic and Data Handling
- Create `src/module_doc_whoami.rs`
- Port `who_am_i` in a single pass preserving original branch structure and output semantics
- Convert C string and buffer handling into safe Rust ownership patterns
- If OS identity lookup requires C APIs, isolate those calls in minimal `unsafe` blocks and immediately convert results into Rust-owned data
- Replace integer/error sentinel handling with explicit Rust error checks

### Phase 3: Wire Output and Error Behavior
- Preserve the original observable behavior for success and failure cases
- Map C-style diagnostics or return codes into idiomatic Rust internals without changing external behavior
- Ensure any direct output uses standard Rust I/O with equivalent formatting
- Keep failure paths simple and local; do not introduce recovery layers

### Phase 4: Validate with Targeted Tests
- Add unit tests for the migrated `who_am_i` logic where deterministic behavior can be checked
- Prefer tests around formatting, empty/error paths, and conversion boundaries
- If environment-dependent user lookup prevents stable assertion of exact names, test only non-empty success or controlled failure handling
- Run `cargo test` and fix any ownership, UTF-8 conversion, or OS interop edge cases