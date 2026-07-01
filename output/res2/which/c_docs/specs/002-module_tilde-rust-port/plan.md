# Implementation Plan

## Summary

Port the existing `module_tilde` C module into Rust by preserving the current file-level responsibilities and function behavior, with a narrow migration focused on tilde-related path expansion support. The Rust implementation should map the logic from `tilde/shell.c` and `tilde/tilde.c` into a small Rust module that keeps the same operational boundaries: locating a home directory, scanning a string for tilde expansion boundaries, and handling allocation-style fatal errors through Rust-native failure paths.

The technical approach should favor:
- direct translation of the existing parsing flow into safe Rust string/byte scanning,
- use of `std::env` and platform-appropriate path/string handling for home directory lookup,
- elimination of manual memory ownership from C in favor of owned `String`/`PathBuf` values,
- replacement of abort-on-allocation helpers with explicit panic or infallible standard allocation assumptions only where behavior must remain fatal.

The migration should avoid introducing new subsystems or generalized expansion frameworks. The goal is a Rust module that mirrors the existing C behavior closely enough to replace the current implementation incrementally.

## Technical Context

### Language/Version
- Rust 1.75+
  This is sufficient for stable standard-library string handling, path APIs, and test support.

### Primary Dependencies
- Rust standard library only:
  - `std::env`
  - `std::path`
  - `std::ffi` only if needed for platform string conversions
- No third-party crates are recommended based on the provided input.

### Testing
- `cargo test`

### Performance Goals
- Preserve the current module’s lightweight parsing behavior with linear scans over input strings.
- Avoid unnecessary intermediate allocations during prefix/suffix detection.
- Restrict allocations primarily to final owned return values where the C code would have produced allocated strings.
- Maintain behavior appropriate for command-line utility usage; no additional optimization work beyond parity is planned.

## Module Mapping

### C to Rust File Mapping
- `tilde/shell.c`
  - migrate into `src/module_tilde/shell.rs` or fold into `src/module_tilde.rs` if the project prefers a single-module file
  - responsibility retained: home-directory lookup logic used by tilde expansion
- `tilde/tilde.c`
  - migrate into `src/module_tilde/tilde.rs` or fold into `src/module_tilde.rs`
  - responsibility retained: tilde prefix/suffix scanning and associated expansion helpers

### Recommended Rust Module Layout
Prefer the smallest structure that still reflects the original split:

```text
src/
  module_tilde/
    mod.rs
    shell.rs
    tilde.rs
```

If the surrounding Rust project already uses flat module files, an acceptable alternative is:

```text
src/
  module_tilde.rs
```

with internal sections corresponding to the original C files. Do not create extra support modules beyond this mapping.

### Function Mapping
- `get_home_dir`
  - Rust function in `shell.rs`
  - return an owned Rust type such as `Option<String>` or `Option<PathBuf>` depending on call-site needs
  - prefer `Option<String>` if the original logic is string-oriented and consumed directly by tilde expansion
- `tilde_find_prefix`
  - Rust function in `tilde.rs`
  - implement as index/substring boundary detection over `&str` or byte slices
- `tilde_find_suffix`
  - implement as complementary boundary detection over `&str` or byte slices
- `memory_error_and_abort`
  - do not reproduce manual allocation handling
  - replace with Rust-native fatal behavior only where exact fail-fast semantics are required, likely via `panic!`
  - if not externally observable, remove the helper and let standard allocation failure behavior stand

## Data Model

The analysis reports only anonymous C data structures, so no named public struct migration is indicated. The plan should therefore keep data modeling minimal and function-oriented.

### C Structure Mapping
- anonymous C structs/unions used only internally
  - map to:
    - local Rust variables,
    - tuples,
    - small private structs only if required to keep logic readable during translation

### Rust Data Choices
- C string pointers / heap buffers
  - `String` for owned text results
  - `&str` for borrowed input scanning
- filesystem/home-directory values
  - `PathBuf` internally if path operations are needed
  - convert to `String` only at text-processing boundaries
- C integer indices/pointer arithmetic
  - `usize` indices into validated string/byte slices
- sentinel/null return patterns
  - `Option<T>`
- fatal internal failure paths
  - `panic!` only for preserved abort semantics
  - otherwise explicit `Option`/`Result` where the current function contract naturally allows absence/failure

### Memory Management Notes
- Eliminate manual allocation/free flows from the C implementation.
- Avoid returning borrowed references derived from temporary environment/path conversions.
- Prefer byte-slice scanning for delimiter detection to avoid repeated substring allocation.
- Validate any string indexing carefully; use byte indices only when scanning ASCII delimiters and convert back to `&str` on known-valid boundaries.

## Implementation Phases

## Phase 1: Establish module skeleton and migrate home-directory lookup
- Create the Rust `module_tilde` module using the minimal file structure needed to reflect `shell.c` and `tilde.c`.
- Port `get_home_dir` first.
- Use the standard library environment/path APIs to replace C environment and allocation handling.
- Decide the exact return type based on current consumers:
  - prefer `Option<String>` if expansion logic is text-based,
  - otherwise use `Option<PathBuf>` internally and convert at the module boundary.
- Document any platform assumptions present in the C code and keep behavior as close as practical without adding compatibility layers.
- Add focused unit tests for:
  - environment-present home lookup,
  - missing-home behavior,
  - empty or unusable values if the original code distinguishes them.

## Phase 2: Port tilde boundary scanning functions
- Port `tilde_find_prefix` and `tilde_find_suffix` into Rust with a direct translation of the original scan rules.
- Implement scanning using `&str` plus byte iteration where delimiters are ASCII and index safety is clear.
- Keep parsing behavior close to C rather than redesigning the API.
- If the C functions returned pointers into strings, represent the same outcome in Rust as:
  - `Option<usize>` for positions, or
  - borrowed substrings if clearer and safe for the calling pattern.
- Add unit tests covering:
  - no-tilde input,
  - tilde at string start,
  - tilde after delimiters recognized by the original code,
  - suffix termination on expected separators,
  - edge cases such as empty strings and standalone `~`.

## Phase 3: Integrate expansion flow and remove C-style memory failure handling
- Connect home-directory lookup and prefix/suffix scanning in the Rust module so the translated logic matches the current call sequence from the C implementation.
- Remove the need for `memory_error_and_abort` by using Rust-owned strings and standard allocation semantics.
- Where the original helper represented an unconditional fatal path that must remain visible, replace it with a narrow `panic!` at the corresponding invariant boundary rather than preserving a separate helper unless required by structure.
- Review all translated functions for:
  - owned vs borrowed string lifetimes,
  - avoidance of unnecessary clones,
  - exact handling of absent home-directory data.

## Phase 4: Validation and cleanup
- Run `cargo test` and complete parity-oriented unit coverage for the migrated functions.
- Compare translated behavior against the C implementation for representative inputs, especially around delimiter scanning and missing-home cases.
- Simplify any temporary structs or helper functions introduced during translation if they are not required for clarity.
- Ensure final module organization reflects the original source split without adding new public surface area beyond what the port requires.