# Implementation Plan: module_tilde Rust Port

## Summary

Port the existing tilde-expansion module from `tilde/shell.c` and `tilde/tilde.c` into a Rust module that preserves current behavior and migration boundaries. The Rust implementation should focus on translating the existing function set—`get_home_dir`, `tilde_find_prefix`, `tilde_find_suffix`, and `memory_error_and_abort`—into idiomatic Rust equivalents while keeping the same operational scope.

The technical approach is a direct source migration:
- move string scanning logic from the C implementation into Rust `&str` / byte-slice processing,
- replace manual allocation and abort-oriented memory handling with Rust ownership and explicit failure paths,
- keep the module self-contained under standard Rust project layout,
- avoid introducing new abstraction layers or extra helper subsystems beyond what is needed to preserve the current module behavior.

Because the source module appears centered on identifying and expanding tilde-related path segments, the Rust port should prioritize exact string-boundary handling, safe path/home-directory lookup, and minimal behavioral drift from the current C implementation.

## Technical Context

### Language / Version
- Rust stable, edition 2021
- Minimum recommended compiler: Rust 1.74+

### Primary Dependencies
Use the Rust standard library by default.

Recommended dependencies:
- None required initially

If home-directory lookup cannot be implemented acceptably with only the standard library across the project’s supported platforms, add:
- `home` — only for stable, cross-platform home directory resolution corresponding to `get_home_dir`

No other third-party crates are planned.

### Testing
- `cargo test`

Testing approach:
- unit tests in the Rust module for prefix/suffix detection and home-directory lookup behavior,
- focused migration tests for edge cases derived from the C logic,
- no additional benchmark or integration framework unless already present in the repository.

### Performance Goals
- Preserve near-linear scanning behavior for prefix/suffix detection.
- Avoid unnecessary heap allocations during string boundary detection.
- Limit allocations to returned owned strings where required by the migrated API.
- Maintain performance at least comparable to the original C logic for typical command/path-sized inputs.

## Module Mapping

### C to Rust File Mapping
- `tilde/shell.c` -> `src/module_tilde.rs`
- `tilde/tilde.c` -> `src/module_tilde.rs`

If the repository already uses a module directory layout, use:
- `tilde/shell.c` -> `src/module_tilde/mod.rs`
- `tilde/tilde.c` -> `src/module_tilde/mod.rs`

The preferred migration is to keep both C source files consolidated into one Rust module unless the existing Rust project structure already requires a split. This keeps the port aligned with the small existing function surface and avoids introducing extra structure.

### Function Mapping
- `get_home_dir` -> `fn get_home_dir(...) -> ...`
  - Return a Rust-owned path/string form rather than raw allocated memory.
  - Use `Option` or `Result` depending on whether the original behavior distinguishes lookup failure from fatal failure.

- `tilde_find_prefix` -> `fn tilde_find_prefix(input: &str, ...) -> Option<usize>` or equivalent slice-returning helper
  - Preserve exact scanning semantics from C.
  - Prefer returning indices or borrowed slices instead of pointer arithmetic.

- `tilde_find_suffix` -> `fn tilde_find_suffix(input: &str, ...) -> Option<usize>` or equivalent slice-returning helper
  - Mirror original delimiter detection and boundary semantics.

- `memory_error_and_abort` -> removed as a direct memory-allocation facility; replaced by Rust allocation failure behavior and explicit fatal error helper only if the module still requires a dedicated abort path
  - If retained for behavioral parity: `fn memory_error_and_abort() -> !`
  - Otherwise fold call sites into `panic!`/process termination only where the original C code treated the condition as unrecoverable.

## Data Model

The analysis only reports anonymous C data structures. Since no named structs are identified in the module interface, the Rust port should avoid inventing persistent data models unless the source code requires local representation during migration.

### C Struct to Rust Mapping
- anonymous -> local Rust tuple/struct only if needed to preserve parsing state

### Preferred Representation Strategy
- Replace pointer-plus-length or delimiter state with:
  - `&str` for valid textual input,
  - `&[u8]` for byte-precise scanning if the C code operates on ASCII delimiters and byte offsets,
  - `usize` indices instead of raw pointers,
  - `String` or `PathBuf` for owned return values from home-directory expansion paths.

### Memory Management Mapping
- C manual allocation/free -> Rust ownership (`String`, `PathBuf`, `Vec<u8>`)
- C null pointer signaling -> `Option<T>`
- C status/error return plus output buffer -> `Result<T, E>` when failure is semantically meaningful
- C abort on allocation failure -> default Rust allocation behavior or explicit `panic!`/fatal helper only where needed for migration parity

### Error Handling Mapping
- Home-directory lookup failure:
  - use `Option` if missing-home is a normal condition in existing behavior,
  - use `Result` if callers need to distinguish environment/system lookup failure.

- Parsing/scanning functions:
  - return `Option<usize>` / `Option<Range<usize>>` for “not found”
  - do not model ordinary scan misses as exceptional errors

## Implementation Phases

### Phase 1: Source Audit and API Skeleton
- Inspect `tilde/shell.c` and `tilde/tilde.c` to confirm exact signatures, call relationships, and delimiter rules.
- Create the Rust destination module in the existing crate layout.
- Define Rust function signatures for:
  - `get_home_dir`
  - `tilde_find_prefix`
  - `tilde_find_suffix`
  - `memory_error_and_abort` (only if still required after call-site review)
- Decide the minimal shared internal types, preferring indices and borrowed string slices over new structs.
- Record any behavior that depends on environment variables, platform path handling, or byte-level scanning.

### Phase 2: Port Core String and Home-Directory Logic
- Port `tilde_find_prefix` from C scanning logic to Rust, preserving boundary detection order and exact delimiter checks.
- Port `tilde_find_suffix` using the same direct scanning strategy, validating byte/character assumptions before choosing `&str` or byte-slice iteration.
- Port `get_home_dir` using the standard library first:
  - read relevant environment variables as needed,
  - convert to owned Rust path/string values,
  - preserve original fallback behavior where present.
- Replace manual allocation paths with owned Rust values and borrowed references.
- Remove raw pointer arithmetic in favor of index-based slicing.

### Phase 3: Resolve Fatal/Error Paths and Integrate
- Review all C call sites for `memory_error_and_abort`.
- Eliminate it where Rust ownership makes it unnecessary.
- If a dedicated fatal helper is still needed for behavior parity, implement it as a diverging Rust function (`-> !`) with a clear panic/abort message.
- Integrate the migrated functions into the crate’s existing public/internal module visibility rules.
- Ensure no extra API surface is introduced beyond what existing callers need.

### Phase 4: Validation and Behavior Lock-In
- Add unit tests covering:
  - tilde prefix detection,
  - tilde suffix detection,
  - strings without tilde markers,
  - delimiter edge cases,
  - home-directory resolution success/failure cases.
- Add regression tests for any C-specific edge behavior discovered during the port.
- Run `cargo test` and resolve mismatches by tightening the Rust implementation to existing module behavior rather than redesigning semantics.
- Perform final cleanup to remove migration-only scaffolding and confirm the module remains narrowly scoped to the original C functionality.