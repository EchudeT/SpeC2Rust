# Implementation Plan: main_root_fcntl.c_24

## Summary
Port `fcntl.c` functionality for `dupfd` into an idiomatic Rust module that preserves the current behavior and call boundaries needed by the `cat` project. The implementation should stay narrowly scoped to migrating the existing file and function, using Rust’s standard library and minimal OS bindings where required for file-descriptor duplication semantics.

The Rust approach should:
- map the single C source file into one Rust module under the existing crate layout,
- represent raw file descriptors with standard Unix types,
- preserve return-value and error-path behavior through `Result`-based internal handling with explicit conversion at the module boundary if needed by surrounding code,
- avoid adding new abstractions beyond what is necessary to replace the current C logic.

## Technical Context

### Language / Version
- Rust stable, edition 2021
- Minimum recommended compiler: `rustc 1.74+`

### Primary Dependencies
- Rust standard library
- `std::os::unix::io` for Unix file descriptor types and conversions
- No third-party crates are recommended based on the provided module scope

### Testing
- `cargo test`

### Performance Goals
- Maintain behavior and runtime characteristics close to the C implementation
- Keep file-descriptor operations as direct OS calls with negligible abstraction overhead
- Avoid unnecessary allocation, copying, or wrapper layers in the `dupfd` path

## Module Mapping

### C to Rust File Mapping
- `fcntl.c` -> `src/main_root_fcntl_c_24.rs` or the closest existing Rust module file matching project conventions

### Function Mapping
- `dupfd` -> `dupfd` Rust function in the mapped module

### Integration Notes
- Keep the migration localized to the Rust equivalent of `fcntl.c`
- Do not split the function into extra helper modules unless required by borrow-checking or error conversion
- If the project already has a main-cluster module layout, place this function within that existing structure rather than introducing a new subsystem

## Data Model

### C Data Structure Mapping
- `anonymous` -> no named Rust struct required unless the C implementation uses a local aggregate that must be made explicit during translation

### Primitive and OS Type Mapping
- C file descriptor integers -> `std::os::unix::io::RawFd`
- C integer return codes -> `i32` or `RawFd`, depending on the original function contract
- C errno-based failures -> `std::io::Error` internally, converted to the expected outward return form as required by surrounding code

### Memory Management Notes
- No heap-backed data model is expected for this module
- File descriptors must not be accidentally closed by temporary ownership wrappers unless the original semantics explicitly transfer ownership
- Prefer raw descriptor handling or carefully bounded use of owned descriptor types only if ownership is unambiguous in the translated function

## Implementation Phases

### Phase 1: Module Skeleton and Signature Port
- Create the Rust module corresponding to `fcntl.c`
- Add the `dupfd` function with a signature aligned to the current call sites and expected return contract
- Map C primitive types to Rust Unix descriptor types
- Document any assumptions about descriptor ownership and return semantics directly in code comments

### Phase 2: Core Logic Translation
- Translate the `dupfd` logic into Rust using direct OS-facing facilities
- Preserve boundary behavior for valid descriptors, target descriptor constraints, and failure returns
- Implement explicit error handling around the underlying duplication call
- Ensure no unintended ownership transfer or double-close behavior is introduced

### Phase 3: Error Semantics and Call-Site Alignment
- Verify that Rust error paths match the C module’s observable behavior
- Convert internal `std::io::Error` handling into the exact return format required by the surrounding `cat` code
- Adjust imports, visibility, and module references so the Rust module replaces the C implementation cleanly

### Phase 4: Tests and Validation
- Add focused unit tests for successful duplication and failure cases that can be exercised safely under `cargo test`
- Validate descriptor lifecycle behavior to ensure duplicated descriptors remain usable and independently closable
- Confirm the module builds cleanly on the target Unix environment with no extra dependencies