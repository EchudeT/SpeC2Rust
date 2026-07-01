# Implementation Plan

## Summary

Port `progname.c` into a Rust module that preserves the existing responsibility of `set_program_name`: deriving and storing the executable name used by the program entry path. The Rust implementation should stay minimal and local to the current main-cluster behavior, using standard-library path handling and process argument access rather than introducing broader infrastructure.

The technical approach is to migrate the C logic into a small Rust module that:
- extracts the invoked program path from process arguments,
- derives the display/basename form expected by the original logic,
- stores it in a process-local form appropriate for the Rust port,
- exposes a narrow function matching the original module role.

The migration should avoid adding new capabilities beyond the current file/function scope. Error handling should remain simple and explicit, with invalid or unavailable argument data handled through conservative fallbacks rather than panics where practical.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only (`std::env`, `std::path`, and standard string/path types)
- **Testing**: `cargo test`
- **Performance Goals**:
  - Constant-time or near-constant overhead relative to startup work already required for argument inspection
  - No repeated path parsing beyond what is needed to initialize program name state
  - No unnecessary heap allocation beyond owned storage for the final program name value

## Module Mapping

| C File | C Function | Rust Module/File | Rust Item | Notes |
|---|---|---|---|---|
| `progname.c` | `set_program_name` | `src/progname.rs` | `set_program_name` | Direct migration target; keep scope limited to program-name derivation and storage |

If the current Rust binary layout already centralizes startup in `src/main.rs`, that file should call into `src/progname.rs` during existing argument/setup flow rather than expanding startup architecture.

## Data Model

This module has no declared C structs to port.

Data/state mapping for the function-level behavior:

| C Concept | Rust Mapping | Notes |
|---|---|---|
| Program name stored as process-global string state | Module-local owned string state | Use a simple module-local storage approach consistent with current port architecture |
| `char *` / C string input path | `OsString` / `PathBuf` / `String` as needed | Prefer `OsStr`/`Path` while deriving basename; convert to owned `String` only for retained state |
| Basename extraction from argv path | `std::path::Path::file_name` with fallback | Keep behavior narrow and startup-oriented |

Memory management decisions:
- Replace borrowed/global C string handling with owned Rust storage to avoid lifetime hazards.
- Avoid leaking or pinning memory unless required by already-established project conventions.
- Keep conversions from OS-native strings explicit; where a lossy string conversion is necessary for stored display name, do it once at initialization.

Error handling decisions:
- `set_program_name` should not rely on unchecked indexing or UTF-8 assumptions.
- Missing argument 0, empty path values, or non-UTF-8 paths should resolve to a conservative fallback string or retained lossy representation, depending on the surrounding project convention.
- Avoid panic-based control flow for normal startup conditions.

## Implementation Phases

### Phase 1: Inspect and map current startup usage

- Identify how the Rust port’s main entry currently obtains process arguments and where `set_program_name` must be invoked.
- Confirm whether any existing global/program-name accessor already exists in the branch to avoid duplicating state.
- Define the exact Rust module file placement as `src/progname.rs`, keeping the migration aligned with the single C source file.

### Phase 2: Port program-name derivation logic

- Implement `set_program_name` in Rust with standard-library path handling.
- Migrate basename extraction from the invoked executable path using `Path` operations.
- Store the resulting program name in a minimal module-local owned representation.
- Ensure the implementation handles:
  - absent `argv[0]` equivalent,
  - empty executable name,
  - non-UTF-8 executable paths,
  without introducing extra facilities beyond this module.

### Phase 3: Integrate with main-cluster flow

- Update the existing Rust startup path to call `set_program_name` at the corresponding initialization point.
- Keep call ordering aligned with the original C startup expectations.
- Limit integration changes to the files already involved in main-cluster initialization; do not introduce new layers or helper subsystems.

### Phase 4: Validate behavior with focused tests

- Add unit tests for the extracted derivation logic using representative path inputs:
  - simple executable name,
  - absolute/relative path input,
  - trailing component extraction,
  - empty or missing-like fallback behavior.
- If direct testing of process-global state is awkward, structure tests around a small internal helper for path-to-name derivation while keeping the public surface unchanged.
- Run `cargo test` and verify the module compiles cleanly under the target branch with no added nonstandard dependencies.