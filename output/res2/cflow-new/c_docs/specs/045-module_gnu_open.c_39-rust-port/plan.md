# Implementation Plan: module_gnu_open.c_39

## Summary
Port `gnu/open.c` into a focused Rust module that preserves the existing `orig_open` behavior and call shape as closely as Rust permits. The implementation should center on Rust’s standard library file-opening facilities, with small, explicit translations for C-style flags, path handling, and error propagation. The port should avoid introducing new abstractions beyond what is required to represent the current function and any directly related helper logic.

The technical approach is to migrate the file-opening logic into a single Rust module with a narrow API, map C integer/flag-driven behavior to `std::fs::OpenOptions` and platform-specific open options where needed, and convert C error signaling into `Result`-based Rust errors while preserving observable failure conditions as much as practical.

## Technical Context

### Language / Version
- Rust 1.78 or newer

### Primary Dependencies
- Rust standard library
- No third-party crates recommended based on the provided module scope

### Testing
- `cargo test`

### Performance Goals
- Maintain behavior with no meaningful regression for single file-open operations
- Keep allocation overhead minimal, especially around path conversion
- Preserve direct system-call-oriented behavior where Rust standard facilities allow
- Avoid unnecessary wrappers or indirection in the migrated function path

## Module Mapping

### C to Rust File Mapping
- `gnu/open.c` → `src/module_gnu_open.rs`

### Function Mapping
- `orig_open` → `orig_open` in `src/module_gnu_open.rs`

### Rust Module Placement
- Expose the module from the crate using standard Rust module declarations only as needed for existing project integration
- Keep implementation concentrated in one Rust source file unless tests require adjacent `mod tests`

## Data Model

The analysis only reports anonymous C data structures and one functional entry point. Since no named struct contract is provided, the Rust port should avoid inventing broad data models and instead keep representations local and minimal.

### Data Structure Mapping
- `anonymous` → no standalone Rust type unless the C source reveals a required local aggregate

### Expected Type Conversions
- C path input (`char *` / `const char *`) → Rust path representation such as `&Path` or `PathBuf`, depending on ownership needs
- C flag bitmask (`int`) → Rust integer type preserving bit operations, then translated into `OpenOptions`
- C mode value (`int` / `mode_t`) → Rust integer type, applied only when create-mode semantics are needed on Unix
- C file descriptor result (`int`) → either:
  - `std::fs::File` internally, if callers can be updated accordingly, or
  - raw file descriptor form using standard OS descriptor extraction if the migrated API must remain descriptor-oriented

### Memory Management Notes
- Replace manual C resource ownership with Rust RAII for opened files
- If a raw descriptor must be returned, make ownership transfer explicit to avoid double-close or premature close
- Avoid heap allocation unless needed for path normalization or owned path conversion

### Error Handling Notes
- Replace C sentinel returns and `errno`-style handling with `std::io::Result`
- If compatibility requires integer-style return codes at a boundary, keep conversion isolated at the boundary layer
- Preserve invalid flag/path handling explicitly rather than silently normalizing unsupported combinations

## Implementation Phases

### Phase 1: Source Analysis and API Boundary Definition
- Inspect `gnu/open.c` and identify the exact signature and behavioral dependencies of `orig_open`
- Determine whether `orig_open` is descriptor-returning, `FILE`-related, or already close to a filesystem abstraction
- Identify all C open flags and mode interactions used by the function
- Define the narrow Rust function signature required for the port, preferring a direct migration over redesign
- Record any platform-specific behavior that must be preserved, especially Unix open-mode details

### Phase 2: Core Function Port
- Create `src/module_gnu_open.rs`
- Implement `orig_open` with direct translation of:
  - path conversion
  - open flag decoding
  - optional mode handling
  - return/error mapping
- Use `std::fs::OpenOptions` as the primary implementation path
- On Unix, use standard-library platform extensions only if needed to carry C-style flags or mode bits
- Keep helper functions private and limited to decoding existing C semantics

### Phase 3: Error and Ownership Validation
- Verify that all resource ownership transitions are correct, especially if raw descriptors are exposed
- Confirm that invalid inputs and failing opens produce stable Rust error results
- Audit for any places where C relied on mutable buffers, null pointers, or unchecked integers and convert them into explicit Rust validation
- Remove any temporary compatibility scaffolding not required by the final module boundary

### Phase 4: Tests and Integration
- Add focused unit tests covering:
  - successful open of an existing file
  - create/truncate or append behavior if used by the C function
  - invalid path or permission failure
  - flag combinations that are meaningful to this module
- Integrate the module into the crate with only the required `mod` and visibility declarations
- Run `cargo test` and confirm the migrated module builds cleanly on the target branch