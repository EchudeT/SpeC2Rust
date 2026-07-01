# Implementation Plan: module_tilde

## Summary

Port the C tilde-expansion module into Rust by migrating the existing logic from `tilde/shell.c` and `tilde/tilde.c` into a focused Rust module that preserves current behavior and boundaries. The Rust implementation should keep the same functional surface represented by `get_home_dir`, `tilde_find_prefix`, `tilde_find_suffix`, and `memory_error_and_abort`, while replacing manual memory handling with owned Rust strings and slices.

The technical approach is a direct translation of the existing parsing and home-directory lookup flow:

- use `String`, `&str`, and standard iterator/index operations for prefix/suffix scanning;
- represent allocation failure paths as process-aborting helper behavior only where the original C code treats allocation failure as fatal;
- keep lookup and parsing logic local to the migrated module rather than introducing broader abstractions;
- preserve module scope and avoid adding capabilities beyond the C sources being ported.

## Technical Context

### Language/Version

- Rust 1.76 or newer

### Primary Dependencies

Use the Rust standard library by default:

- `std::env` for home directory environment lookup where applicable
- `std::path` for path-oriented handling when returning or composing home-directory values
- `std::process` for fatal termination behavior if a direct equivalent to `memory_error_and_abort` is required

No third-party crates are recommended because the provided inputs do not show a need beyond standard library facilities.

### Testing

- `cargo test`

Testing should cover:

- prefix detection for strings containing tilde forms
- suffix detection boundaries
- home-directory lookup behavior for current-user expansion paths supported by the original logic
- fatal-path helper behavior in the narrowest testable form, or indirect verification if implemented as a terminating function

### Performance Goals

- Maintain linear-time scanning behavior for tilde prefix/suffix parsing
- Avoid unnecessary intermediate allocations during parsing
- Limit allocations to returned expanded strings or copied home-directory values
- Match or improve the C implementation’s practical runtime for short command/path strings typical for `which`

## Module Mapping

### Source File Mapping

| C File | Rust Target | Notes |
|---|---|---|
| `tilde/shell.c` | `src/module_tilde.rs` | Migrate shell/home-directory helper logic into the same Rust module unless existing crate layout already provides a direct module file for this feature. |
| `tilde/tilde.c` | `src/module_tilde.rs` | Migrate tilde parsing functions directly, keeping related parsing logic together. |

If the project already uses directory-based modules, an equivalent mapping is acceptable:

- `tilde/shell.c` -> `src/module_tilde/shell.rs`
- `tilde/tilde.c` -> `src/module_tilde/tilde.rs`
- `src/module_tilde/mod.rs` as the re-export surface

However, prefer a single Rust module file unless the current Rust crate already requires split files.

### Function Mapping

| C Function | Rust Function | Migration Decision |
|---|---|---|
| `get_home_dir` | `fn get_home_dir(...) -> Option<String>` or `Result<String, _>` | Convert raw pointer/string ownership into explicit return ownership. Final signature should match existing call patterns in the Rust crate. |
| `tilde_find_prefix` | `fn tilde_find_prefix(input: &str) -> Option<usize>` or equivalent range return | Preserve scanning semantics while replacing pointer arithmetic with byte index logic constrained to valid UTF-8 boundaries. |
| `tilde_find_suffix` | `fn tilde_find_suffix(input: &str, start: usize) -> usize` or `Option<usize>` | Preserve delimiter detection using slice/index scanning. |
| `memory_error_and_abort` | `fn memory_error_and_abort() -> !` | Keep as a terminating helper only if still needed after removing manual allocation patterns. |

## Data Model

The analysis reports only anonymous C data structures. Since no named struct API is identified, the Rust port should avoid inventing persistent data models unless the C implementation uses internal aggregate state that must be preserved during translation.

### Data-Structure Mapping

| C Data Structure | Rust Representation | Notes |
|---|---|---|
| anonymous | Local variables / tuples | Prefer direct local-state translation where the C code used temporary aggregate values. |
| anonymous | `String` | Replace heap-allocated mutable C strings returned from helpers. |
| anonymous | `&str` slices | Replace pointer-range views used during prefix/suffix scanning. |
| anonymous | `Option<T>` | Represent nullable pointers or absent lookup results. |
| anonymous | `usize` indices | Replace pointer offsets and scan positions. |

### Memory Management Mapping

- C `char *` output buffers -> Rust `String`
- C nullable pointer returns -> `Option<String>` or `Option<usize>`
- C pointer arithmetic -> `usize` indices over `&str`/`[u8]`
- C fatal allocation path -> retained only as `-> !` abort helper if the migrated logic still needs a dedicated failure point

### Error Handling Mapping

- Absence of a home directory or unsupported tilde form should be represented explicitly through `Option`/`Result`, according to current caller expectations in the Rust crate.
- Fatal behavior should not be generalized; only keep a dedicated aborting path where the original C module treated the condition as unrecoverable.
- Avoid `unwrap()` in parsing and index manipulation; compute indices defensively to preserve correctness.

## Implementation Phases

## Phase 1: Establish Rust module skeleton and migrate parsing entry points

### Goals

- Create the Rust target module for `module_tilde`
- Port `tilde_find_prefix`
- Port `tilde_find_suffix`
- Define minimal internal signatures needed by callers

### Tasks

- Add `src/module_tilde.rs` or the equivalent existing module path in the Rust crate
- Translate prefix scanning from pointer-based logic into `&str`/byte-index scanning
- Translate suffix scanning similarly, preserving delimiter rules and stopping conditions from the C implementation
- Keep function visibility limited to the module or crate as required by current usage
- Add unit tests for scan behavior based on the C logic boundaries

### Completion Criteria

- The Rust module compiles
- Prefix and suffix detection behavior is covered by `cargo test`
- No additional parsing abstractions are introduced beyond what is needed to mirror the C functions

## Phase 2: Migrate home-directory lookup behavior

### Goals

- Port `get_home_dir`
- Replace C string allocation/ownership with Rust-owned return values
- Preserve current lookup semantics without adding new resolution features

### Tasks

- Translate the home-directory retrieval flow from `tilde/shell.c`
- Use standard library environment and path facilities where they directly match the original behavior
- Return explicit absence/failure via `Option` or `Result`
- Update internal callers to consume owned/sliced Rust strings rather than raw pointers
- Add tests covering successful lookup and no-home-directory cases that are practical to exercise

### Completion Criteria

- `get_home_dir` is fully migrated
- Caller interactions no longer depend on raw allocation semantics
- Tests validate expected lookup outcomes within standard Rust test constraints

## Phase 3: Resolve fatal-path handling and integrate module behavior

### Goals

- Port or eliminate `memory_error_and_abort` based on actual need after translation
- Finish module-level integration
- Verify behavior parity for the migrated functions

### Tasks

- Determine whether any remaining path still requires a dedicated fatal helper after adopting `String` and `Option`/`Result`
- If still required, implement `memory_error_and_abort` as a narrow `-> !` function using standard error reporting and process termination
- If not required by the translated logic, keep a minimal private helper only when needed for signature parity during migration, then remove dead usage
- Finalize internal call flow between parsing and home-directory lookup functions
- Add integration-oriented tests for representative tilde handling paths covered by this module

### Completion Criteria

- All listed C functions are either migrated directly or intentionally removed as unnecessary due to Rust ownership semantics, with no behavior expansion
- The module passes `cargo test`
- The resulting Rust implementation remains confined to the original module scope

## Final Notes

- Prefer exact behavioral migration over refactoring.
- Keep file layout and function boundaries close to the original C sources.
- Do not introduce broader path-expansion frameworks, user-database integrations, or cross-module utility layers unless they are already required by the existing Rust crate structure.