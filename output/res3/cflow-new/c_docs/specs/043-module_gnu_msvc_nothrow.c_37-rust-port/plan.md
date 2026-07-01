# Implementation Plan: module_gnu_msvc-nothrow.c_37

## Summary

This module is a narrow portability layer around `_gl_nothrow_get_osfhandle`, currently sourced from `gnu/msvc-nothrow.c`. The Rust port should preserve the existing behavior and scope: provide a Windows-oriented helper that obtains an OS file handle from a C-runtime file descriptor while keeping failure handling explicit and non-panicking.

The implementation approach should stay minimal and migration-focused:

- map the single C function into a single Rust module function,
- keep the interface close to the original operational contract,
- use conditional compilation for Windows-specific behavior,
- represent fallible results with Rust error-returning types rather than C sentinel patterns where practical,
- avoid introducing broader abstractions or extra portability layers beyond what is required by this source file.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**:
  - Rust standard library
  - No third-party crates recommended unless the surrounding project already depends on a Windows bindings crate
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve constant-time handle lookup behavior comparable to the C implementation
  - Avoid heap allocation
  - Keep call overhead minimal and limited to the underlying platform/API interaction
  - Match the original module’s lightweight role without adding wrapper layers that affect hot-path use

## Module Mapping

### Source File Mapping

- `gnu/msvc-nothrow.c`
  → `src/gnu/msvc_nothrow.rs`

### Function Mapping

- `_gl_nothrow_get_osfhandle`
  → `pub(crate) fn gl_nothrow_get_osfhandle(...) -> ...`

### Rust Module Placement

Use a direct module layout aligned with the original file location:

- `src/gnu/mod.rs`
- `src/gnu/msvc_nothrow.rs`

If this module is only consumed internally, keep visibility restricted to `pub(crate)` and avoid creating a public API surface broader than the original usage requires.

## Data Model

This module has no dedicated C structs or persistent data structures to migrate.

### Data Mapping

- C integer file descriptor parameters
  → Rust signed integer type matching the original ABI intent, typically `i32`
- C raw OS handle return value
  → Rust raw handle representation appropriate to the implementation path:
  - preferably a platform raw handle-compatible primitive/pointer type
  - or a direct integer-sized type if required to mirror the original return contract closely

### Error Representation

Because the original C code likely signals failure using sentinel values rather than structured errors, the Rust port should choose the narrowest representation that preserves semantics:

- Preferred internal API:
  - `Result<RawHandleLike, std::io::Error>` if the callers can consume structured errors
- If strict compatibility with existing translated call sites is needed:
  - `Option<RawHandleLike>` or a sentinel-preserving raw return type, with error extraction handled locally

The final choice should be based on the immediate caller migration needs, not on designing a new abstraction.

## Implementation Phases

## Phase 1: Create the Rust Module Skeleton

- Add `src/gnu/msvc_nothrow.rs`
- Register it from `src/gnu/mod.rs`
- Define the Rust function corresponding to `_gl_nothrow_get_osfhandle`
- Add `#[cfg(windows)]` guards around the real implementation
- Add a restrained non-Windows stub only if required for cross-platform compilation of the crate

### Phase 1 Deliverables

- Module file created
- Function signature established
- Buildable module wiring in place
- No extra helper subsystems introduced

## Phase 2: Port the Handle Lookup Logic

- Translate the C function logic into Rust with behavior kept as close as practical
- Use the smallest unsafe boundary necessary if the implementation must call CRT or platform APIs
- Preserve failure behavior, especially invalid descriptor handling
- Ensure no panics are introduced for ordinary runtime failure cases
- Keep memory ownership trivial: no allocations, no borrowed-state retention, no handle ownership transfer unless the C source clearly did so

### Phase 2 Technical Notes

- If an external function call is required, isolate it inside a small unsafe block
- Document:
  - parameter expectations,
  - invalid input behavior,
  - returned handle ownership semantics
- Do not wrap the result in ownership-managing handle types unless the original function created ownership, which this module does not indicate

## Phase 3: Align Error Handling and Call-Site Expectations

- Confirm the return type expected by migrated callers
- Normalize sentinel/error conversion at this boundary rather than spreading compatibility logic outward
- Ensure errno-style or OS-error-style information is preserved only as far as needed by the translated module interactions
- Keep the function contract stable and internal

### Phase 3 Deliverables

- Finalized return type
- Error path behavior reviewed against the C source intent
- Internal API ready for dependent module migration

## Phase 4: Add Focused Tests

- Add unit tests for the valid and invalid file descriptor cases that can be exercised safely in Rust
- On Windows, verify behavior using a real file descriptor obtained from standard Rust file APIs where feasible
- Ensure tests do not assume ownership transfer of the returned handle
- Gate platform-specific tests with `#[cfg(windows)]`

### Test Cases

- invalid descriptor returns failure as expected
- valid descriptor returns a non-invalid OS handle
- repeated calls do not mutate ownership or close the underlying resource

## Notes and Constraints

- Keep the port limited to `gnu/msvc-nothrow.c`; do not introduce generalized handle utilities
- Prefer standard library types and conditional compilation over external crates
- Keep unsafe code localized and justified
- Preserve original resource-lifetime expectations: this function should observe or retrieve a handle, not manage it
- Avoid expanding behavior beyond the single migrated function