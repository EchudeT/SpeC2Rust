# Implementation Plan: module_src_parseopt_04

## Summary

This module ports the option-parsing support currently embedded in `src/main.c` into Rust, focusing only on the existing behaviors represented by:

- `parseopt_from_env`
- `fromfile_error`
- `fromfile`
- `optset_profile`
- `init_hook`

The Rust implementation should keep these routines close to the original control flow and responsibility split, rather than redesigning option handling. The technical approach is to migrate the parsing and initialization logic into a small Rust module that:

- reads option-related input from environment variables and files,
- reports parse/load errors through typed Rust errors,
- applies option profile selection using explicit state updates,
- preserves startup hook ordering in a direct, testable form.

Because the source comes from a single C file, the Rust port should also remain compact and local in structure. The main goal is behavioral equivalence with safer ownership, explicit error propagation, and elimination of manual memory management.

## Technical Context

### Language / Version
- Rust 1.78 or newer

### Primary Dependencies
- Rust standard library only:
  - `std::env` for environment access
  - `std::fs` for file loading
  - `std::path` for path handling
  - `std::io` for I/O errors
  - `std::fmt` / `std::error` for error types

No third-party crates are recommended at this stage because the input does not justify additional parsing or CLI frameworks.

### Testing
- `cargo test`

Testing should cover:
- environment-driven option sourcing,
- file-based option loading,
- profile selection updates,
- initialization hook effects,
- error formatting and propagation paths.

### Performance Goals
- Preserve the current linear parsing behavior.
- Avoid unnecessary string copying beyond what is needed to convert C-style mutable buffers into owned Rust `String`/`Vec<String>` data.
- File loading should remain proportional to input size with straightforward buffered or whole-file reads as appropriate to the original behavior.
- No performance work beyond parity with the C implementation is planned.

## Module Mapping

### Source Mapping
- C source:
  - `src/main.c`
- Rust target:
  - `src/main.rs` if these routines remain entry-point-local
  - or `src/parseopt.rs` plus `src/main.rs` wiring if the port needs separation for compilation clarity

Preferred mapping: move the parsed option support into `src/parseopt.rs` and keep `src/main.rs` limited to invocation. This still reflects the original single-file ownership while making the migrated functions individually testable.

### Function Mapping
- `parseopt_from_env`
  - Map to a Rust function that reads the relevant environment variable(s), tokenizes or forwards content according to the C logic, and updates parser/application state.
- `fromfile_error`
  - Map to a small Rust helper returning or constructing a typed error with enough context to preserve original diagnostics.
- `fromfile`
  - Map to a Rust function that opens and reads an options file, parses its contents in the same order/shape as C, and returns structured errors instead of status codes.
- `optset_profile`
  - Map to a Rust function or method that applies the selected profile to the existing options/config state.
- `init_hook`
  - Map to a Rust initialization routine invoked during startup, preserving original sequencing and side effects.

## Data Model

The analysis only exposes anonymous C data structures, so the Rust plan should avoid speculative redesign. Data model work should proceed by deriving named Rust types directly from actual field usage in `src/main.c`.

### Mapping Strategy
- C anonymous structs used only locally
  - Map to private Rust `struct`s scoped within `parseopt.rs`.
- C anonymous unions or tag-like flag combinations
  - Map to Rust `enum`s when there is clear variant behavior.
- C integer status/error codes
  - Map to `Result<T, ParseOptError>`.
- C string pointers (`char *`, `const char *`)
  - Map to `String`, `&str`, or `PathBuf` depending on ownership and semantic use.
- C arrays of strings / argv-style lists
  - Map to `Vec<String>`.
- C nullable pointers indicating optional configuration
  - Map to `Option<T>` or `Option<String>`.

### Planned Rust Types
These names are intentionally minimal and should be adjusted to match the actual field groupings found during migration:

- `ParseOptError`
  - Rust `enum` for:
    - environment read/format errors,
    - file I/O errors,
    - file content parse errors,
    - invalid profile selection,
    - initialization-time option errors.
- `OptionSource`
  - Rust `enum` identifying whether options came from environment, file, or startup/default flow if such distinction is needed by existing logic.
- `ProfileSelection`
  - Rust `enum` or `struct` only if the C code has an explicit profile concept beyond plain strings.
- `ParseOptState`
  - Rust `struct` representing the subset of program state mutated by these five functions.

### Memory Management Notes
- Replace C-owned temporary buffers with owned `String` and `Vec<String>`.
- Avoid shared mutable aliasing; pass `&mut ParseOptState` where C previously mutated global or passed-in state.
- Convert any borrowed slices only at call boundaries; internal parsing should own data when the C version relied on mutable buffers.

### Error Handling Notes
- Replace return-code-plus-diagnostic patterns with `Result`.
- Preserve user-visible error message content where practical.
- Keep `fromfile_error` as a dedicated constructor/helper if the original code centralizes diagnostics formatting.

## Implementation Phases

## Phase 1: Extract and Define Rust Interfaces
- Inspect `src/main.c` and isolate the exact data touched by:
  - `parseopt_from_env`
  - `fromfile_error`
  - `fromfile`
  - `optset_profile`
  - `init_hook`
- Create `src/parseopt.rs`.
- Define minimal Rust equivalents for:
  - parser/config state,
  - profile-related values,
  - file/env source distinctions if needed,
  - `ParseOptError`.
- Add function signatures mirroring the C call graph and side-effect boundaries.
- Wire `src/main.rs` to call the new Rust module without introducing broader architecture changes.

## Phase 2: Port File and Environment Parsing Paths
- Implement `parseopt_from_env` using `std::env`.
- Implement `fromfile` using `std::fs` and `std::io`.
- Implement `fromfile_error` as the shared error-construction path.
- Preserve the C parsing order, token treatment, and failure points as closely as possible.
- Replace mutable C buffer handling with owned Rust strings and explicit iteration over tokens/lines.

## Phase 3: Port Profile and Initialization Logic
- Implement `optset_profile` against the Rust state model.
- Implement `init_hook` and preserve startup ordering and any default/profile application sequence present in C.
- Ensure state mutation remains local and explicit rather than relying on hidden globals unless the surrounding Rust port still requires a single static configuration path.

## Phase 4: Verification and Cleanup
- Add focused unit tests for:
  - successful env parsing,
  - missing/invalid env content,
  - successful options-file parsing,
  - file error propagation,
  - invalid profile handling,
  - initialization behavior.
- Compare observable behavior against the C implementation, especially:
  - precedence between env/file/profile/defaults,
  - error text shape,
  - no-op cases when input sources are absent.
- Remove any temporary compatibility scaffolding not needed after the port is complete.