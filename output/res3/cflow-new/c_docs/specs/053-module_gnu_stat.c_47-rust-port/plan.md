# plan.md

## Summary

This module ports the C source file `gnu/stat.c` function `is_unc_root` into a Rust module with equivalent path-classification behavior. The Rust implementation should stay narrowly scoped to the existing function and preserve its current responsibility: inspecting a path value and determining whether it represents a UNC root form.

The technical approach is to translate the current string-scanning logic into safe Rust using standard-library path and string/byte handling where possible, while avoiding changes in observable behavior. Because UNC handling is platform-sensitive and often depends on exact separator parsing, the implementation should prefer explicit inspection of path bytes or characters over higher-level normalization that could alter semantics. Ownership should remain simple: borrowed string/path inputs, pure return values, and no heap-managed shared state.

## Technical Context

- **Language/Version**: Rust 1.78 or newer
- **Primary Dependencies**:
  - Rust standard library only
- **Testing**:
  - `cargo test`
- **Performance Goals**:
  - Match the C implementation’s constant-space behavior
  - Keep path inspection linear in input length
  - Avoid unnecessary allocations and path normalization copies
  - Preserve low-overhead checks suitable for frequent filesystem-path evaluation

## Module Mapping

- **C source**: `gnu/stat.c`
- **Rust target**: `src/module_cluster/module_gnu_stat.rs`

### Function mapping

- `is_unc_root` -> `pub(crate) fn is_unc_root(...) -> bool`

### Suggested crate module exposure

- Declare the Rust file from the existing cluster area only as needed by current project structure:
  - `src/module_cluster/mod.rs` -> `pub(crate) mod module_gnu_stat;`

The Rust module should contain only the migrated function and any minimal private helper logic required to preserve the C behavior.

## Data Model

The analysis lists only anonymous C data structures and a single function. Since no named struct appears to be part of the migrated surface for `is_unc_root`, the Rust port should avoid inventing replacement data types unless the source file requires a local helper representation during translation.

### Mapping

- `anonymous` -> no direct Rust type unless required by surrounding translated code

### Input/representation guidance

Because the function is path-oriented, choose the narrowest representation that preserves behavior from the C code:

- If the original C function operates on `char *` / `const char *`, prefer:
  - `&str` when UTF-8 assumptions are already valid in the Rust caller context
  - `&std::ffi::OsStr` or `&std::path::Path` only if surrounding migrated code already uses them and exact separator access remains possible without semantic drift
- If exact byte-level separator scanning is needed to mirror C behavior, implement the logic over `&[u8]` internally from a borrowed source, but keep the external signature aligned with the rest of the Rust port

### Memory management and errors

- No manual memory management is needed; use borrowed inputs and return `bool`
- No error object is expected for this function; malformed or non-matching input should map to `false` unless the C logic indicates otherwise
- Avoid panics from indexing by using iterator-based or bounds-checked scanning

## Implementation Phases

### Phase 1: Source analysis and signature selection

- Inspect `gnu/stat.c` and isolate the exact `is_unc_root` implementation, including:
  - accepted input type
  - separator rules
  - treatment of repeated separators
  - minimum component count for a UNC root match
  - any platform-conditional logic embedded in the function
- Determine the closest Rust function signature based on the actual call sites in the ported codebase
- Create `src/module_cluster/module_gnu_stat.rs`
- Add the module declaration in `src/module_cluster/mod.rs` if not already present

### Phase 2: Direct function translation

- Port `is_unc_root` as a focused, side-effect-free Rust function
- Translate pointer/index scanning into safe Rust iteration or bounded indexing
- Preserve:
  - exact root-detection rules
  - separator handling
  - return behavior for empty, short, or malformed paths
- Keep helper logic private and local to this module
- Do not add generalized path utilities beyond what is necessary for this function

### Phase 3: Unit tests from observed C behavior

- Add focused unit tests in the same module or the standard Rust test layout
- Cover:
  - empty input
  - non-UNC paths
  - partial UNC-like prefixes
  - valid UNC root forms
  - paths extending beyond the root portion
  - separator edge cases seen in the C logic
- Use test cases derived from the original implementation’s behavior rather than platform-normalized expectations

### Phase 4: Integration verification and cleanup

- Confirm call sites compile cleanly with the chosen Rust signature
- Remove any temporary translation scaffolding not needed after the port
- Run `cargo test`
- Verify the implementation remains allocation-free in normal operation and does not introduce behavior beyond the original C function