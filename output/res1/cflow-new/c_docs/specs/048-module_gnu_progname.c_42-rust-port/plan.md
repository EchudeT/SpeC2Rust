# Implementation Plan

## Summary

Port `gnu/progname.c` into an idiomatic Rust module that preserves the existing module scope and behavior of `set_program_name` without adding new responsibilities. The Rust implementation should focus on extracting and storing the executable/program name from process arguments using standard library path handling, while matching the C module’s role as a small process-name utility.

The implementation should remain minimal:
- migrate the single C source file into one Rust source module;
- translate the function-level behavior of `set_program_name`;
- use owned Rust string storage to avoid C-style global pointer lifetime risks;
- expose only the module-level API needed by the existing project migration.

## Technical Context

- **Language/Version**: Rust 1.76+
- **Primary Dependencies**: Rust standard library only (`std::env`, `std::path`, `std::ffi` only if required by surrounding code)
- **Testing**: `cargo test`
- **Performance Goals**:
  - Constant-time global name access after initialization
  - Negligible startup overhead
  - No unnecessary string copies beyond initial normalization/storage
  - Preserve lightweight utility-module behavior comparable to the C implementation

## Module Mapping

| C Source | Rust Target | Notes |
|---|---|---|
| `gnu/progname.c` | `src/module_gnu_progname.rs` | Single-file migration for the module |
| `set_program_name` | `set_program_name` | Direct function migration with Rust-safe string/path handling |

If the crate already uses a module tree for clustered ports, the file should be declared from the existing root module without introducing extra abstraction layers.

## Data Model

This module has no dedicated C structs to translate. The relevant C-level state should be represented as Rust-owned global/module state.

| C Concept | Rust Representation | Notes |
|---|---|---|
| global program-name pointer/string state | `static` global initialized via `OnceLock<String>` or equivalent standard-library one-time storage | Replaces raw global char pointer semantics with owned storage |
| input program path (`char *`) | `&str`, `String`, or `&OsStr`/`Path` as required by call boundary | Prefer standard UTF-8 `String` where project API allows; use `Path` parsing for basename extraction |

### Memory Management Notes

- Avoid borrowed global string references derived from transient inputs.
- Store the computed program name as owned Rust data.
- Use one-time initialization or controlled replacement semantics only if required by the original calling pattern.
- Do not model C mutable global pointers directly.

### Error Handling Notes

- Prefer a narrow API surface.
- If invalid or empty input is possible, handle it explicitly through:
  - a no-op/default fallback if required by existing semantics, or
  - a `Result` return only if the surrounding Rust port already uses fallible initialization.
- Do not introduce richer error taxonomies unless demanded by the current module interface.

## Implementation Phases

### Phase 1: File and API Migration

- Create the Rust module file corresponding to `gnu/progname.c`.
- Add the module to the existing crate/module tree.
- Port the `set_program_name` function signature into Rust in the narrowest form compatible with current callers.
- Identify the minimal module-global state needed to replace the C file’s global storage pattern.

### Phase 2: Core Behavior Translation

- Implement program-name extraction using `std::path::Path` filename handling rather than manual pointer scanning.
- Normalize the selected name into owned Rust storage.
- Replace raw C global state with `OnceLock<String>` or similarly minimal standard-library storage.
- Ensure behavior for edge cases from path-like inputs is defined and consistent:
  - full path input;
  - basename-only input;
  - empty or non-final path component cases as applicable.

### Phase 3: Safety and Semantics Validation

- Review initialization semantics against the original C usage pattern:
  - whether initialization is intended once;
  - whether subsequent calls should overwrite or be ignored.
- Confirm there are no dangling-reference risks or hidden dependence on C string mutability.
- Keep visibility restricted to what existing migrated code requires.

### Phase 4: Tests

- Add unit tests for:
  - plain executable name input;
  - full path input producing basename;
  - empty or unusual path input handling according to chosen semantics;
  - repeated initialization behavior if applicable.
- Run `cargo test` and verify the module builds cleanly within the branch integration target.