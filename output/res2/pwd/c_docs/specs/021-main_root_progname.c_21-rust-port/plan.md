# Implementation Plan

## Summary

Port `progname.c` into a Rust module that provides the `set_program_name` behavior used by the program startup path. The Rust implementation should stay narrowly aligned with the existing C responsibility: derive and retain the executable/program name from startup input without introducing broader process-management facilities.

The preferred technical approach is to:
- implement a small Rust module dedicated to program-name extraction and storage,
- accept startup-provided arguments in Rust-native forms,
- normalize the incoming executable path down to the program name component,
- store the result in process-local state with explicit ownership,
- expose a minimal API matching the existing call pattern required by the main cluster.

This migration should favor `std` types such as `Path`, `PathBuf`, `OsStr`, `OsString`, and owned strings/OS strings over raw pointer-style handling. Error handling should remain simple and local: if the startup value is missing or not representable as UTF-8 where a `String` is needed, the implementation should preserve behavior with a conservative fallback rather than adding new error propagation layers unless the surrounding Rust port already requires a `Result`.

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - constant-time retained access after initialization,
  - negligible startup overhead,
  - no unnecessary heap churn beyond the single owned program-name value,
  - behavior equivalent to the C path-processing responsibility for normal command-line inputs.

## Module Mapping

| C File | C Function | Rust Module | Rust Item |
|---|---|---|---|
| `progname.c` | `set_program_name` | `src/main_root_progname.rs` or `src/progname.rs` | `pub(crate) fn set_program_name(...)` |

### Notes
- Keep the Rust file count minimal: one Rust source file for this migrated C file.
- Place the module where the existing main-cluster Rust port expects startup utilities; do not split into helper submodules unless already required by the current crate layout.
- If the main binary already has a root module for startup concerns, register this module there and migrate call sites directly.

## Data Model

This module has no declared C structs to port.

### Data Mapping

| C Concept | Rust Representation |
|---|---|
| `char *` / program path input | `&OsStr`, `OsString`, or `&str` depending on call-site constraints |
| global retained program name | owned process-local state such as `OnceLock<OsString>` or `OnceLock<String>` |
| basename-style path extraction | `std::path::Path` operations |

### Ownership and Lifetime Decisions
- Replace borrowed C-string/global-pointer style retention with owned Rust storage.
- Prefer `OsString` for retained state if the original executable name may not be valid UTF-8.
- Convert to `String` only if the surrounding Rust code already assumes UTF-8 and all current call sites require it.
- Use one-time initialization storage to mirror the C module’s write-on-startup usage pattern and avoid mutable global aliasing.

### Error Handling Decisions
- Avoid panics for ordinary malformed or empty input.
- If the input path has no final component, retain the original input when possible.
- If no usable input is available, use a small deterministic fallback consistent with the binary context rather than introducing a new failure mode.
- If one-time global storage is used, repeated initialization attempts should be handled explicitly and quietly according to current startup flow expectations.

## Implementation Phases

## Phase 1: Create the Rust module and migrate core logic
- Add a single Rust module corresponding to `progname.c`.
- Implement `set_program_name` with Rust-native path handling:
  - receive the startup executable argument from the existing main path,
  - derive the final path component,
  - retain an owned copy in module-local state.
- Choose the retained type based on existing crate expectations:
  - `OsString` if no UTF-8 guarantee exists,
  - `String` only if all current uses are text-based and UTF-8-safe.
- Keep the public surface minimal and limited to what current call sites need.

## Phase 2: Integrate with the main-cluster startup flow
- Replace the C-module usage with the Rust module in the startup sequence.
- Update the relevant main-cluster file(s) to call the Rust `set_program_name` at the same initialization point as the C version.
- Ensure no raw pointer assumptions remain in the migrated call path.
- Preserve the current initialization order so downstream logic sees the program name set when expected.

## Phase 3: Add focused tests for path and storage behavior
- Add unit tests covering:
  - simple executable names,
  - full paths with directory components,
  - trailing separator edge cases as applicable to current intended behavior,
  - empty or missing-like inputs handled by the chosen fallback rule,
  - repeated initialization behavior if one-time storage is used.
- Keep tests local to the module and run them with `cargo test`.
- Validate that ownership is self-contained and that no borrowed startup data escapes its lifetime.

## Phase 4: Final cleanup and equivalence review
- Remove or stop referencing the old C implementation in this migration branch.
- Review the Rust module for narrow parity with the original responsibility only.
- Confirm memory safety properties:
  - no dangling references,
  - no unsafe code unless strictly required by an existing interface,
  - no mutable global state without controlled initialization.
- Confirm that error handling remains minimal and does not expand the original module’s behavior surface.