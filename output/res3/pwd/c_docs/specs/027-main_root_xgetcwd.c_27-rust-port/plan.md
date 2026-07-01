# Implementation Plan: main_root_xgetcwd.c_27

## Summary

This module ports the C file `xgetcwd.c` and its single function `xgetcwd` into Rust, preserving its narrow responsibility: obtaining the current working directory and returning it through an owned allocation model.

The Rust implementation should rely primarily on `std::env::current_dir()` and convert the resulting `PathBuf` into an owned path/string form appropriate for the surrounding `pwd` port. The migration should keep behavior aligned with the C implementation’s intent: encapsulate current-directory retrieval, centralize allocation ownership in Rust, and expose errors explicitly through `Result` rather than C-style null/error signaling.

The technical approach is to:
- replace manual heap allocation and buffer resizing with Rust-owned types,
- map OS errors through `std::io::Error`,
- keep path handling platform-correct by using `PathBuf`/`OsString` internally,
- only perform UTF-8 conversion at the API boundary if required by the existing Rust port structure.

## Technical Context

### Language/Version
- Rust stable, edition 2021
- Minimum practical compiler target: Rust 1.70+

### Primary Dependencies
- Rust standard library only:
  - `std::env::current_dir`
  - `std::path::PathBuf`
  - `std::ffi::OsString`
  - `std::io`

No third-party crates are recommended because the input provides no evidence that additional portability or path-encoding facilities are required beyond the standard library.

### Testing
- `cargo test`

### Performance Goals
- Match or improve the C implementation’s practical performance for current-directory retrieval.
- Avoid repeated allocation/reallocation logic previously needed in C.
- Keep the implementation to a single system-facing retrieval via `std::env::current_dir()`.
- Do not introduce unnecessary path cloning or UTF-8 conversions beyond what the Rust module interface requires.

## Module Mapping

| C File | C Function | Rust Module/File | Rust Item |
|---|---|---|---|
| `xgetcwd.c` | `xgetcwd` | `src/xgetcwd.rs` | `pub fn xgetcwd(...) -> Result<..., std::io::Error>` |

### Mapping Notes
- The C module is a direct single-file/single-function migration.
- The Rust file should remain narrowly scoped to this function and any minimal helper logic needed for path conversion.
- The exact return type should follow the existing branch/module integration needs:
  - prefer `PathBuf` if callers can consume native paths,
  - use `String` only if the wider `pwd` Rust port already standardizes on UTF-8 path strings.

## Data Model

This module has no named C structs to migrate.

### Data-Structure Mapping

| C Concept | Rust Mapping | Notes |
|---|---|---|
| heap-allocated current working directory buffer (`char *`) | `PathBuf` or `OsString` | Rust owns allocation and frees automatically via RAII. |
| null pointer on failure | `Result<T, std::io::Error>` | Explicit error propagation replaces sentinel return values. |
| dynamically resized byte buffer | standard-library internal allocation | No manual capacity management should be reimplemented unless required by an existing branch API. |

### Ownership and Memory Management
- C manual allocation/free behavior is replaced by owned Rust values.
- No raw pointers should be exposed.
- Returned data should be fully owned by the caller through the returned Rust type.
- Temporary conversions should avoid lossy behavior unless the surrounding API already requires string conversion.

### Error Handling
- Preserve OS error reporting through `std::io::Error`.
- Do not emulate C null-return patterns internally except at a compatibility boundary if the broader port requires it.
- If UTF-8 conversion is necessary, conversion failure must be handled explicitly rather than silently discarded.

## Implementation Phases

## Phase 1: Establish Rust Module and API Shape
- Create `src/xgetcwd.rs`.
- Define the Rust `xgetcwd` function with a return type aligned to the port’s existing call sites:
  - preferred: `Result<PathBuf, std::io::Error>`
  - alternative only if required by integration: `Result<String, std::io::Error>`
- Add the module declaration/export in the crate root or relevant parent module.
- Keep the public surface limited to the migrated function.

### Deliverables
- Rust module file exists.
- Function signature is fixed.
- Build integration is in place.

## Phase 2: Port Retrieval Logic and Error Mapping
- Implement `xgetcwd` using `std::env::current_dir()`.
- Return the owned path directly if using `PathBuf`.
- If the surrounding module requires strings, convert from `PathBuf` carefully:
  - prefer non-lossy conversion,
  - surface conversion failure as an error if needed by the chosen interface.
- Remove all C-style allocation assumptions; rely on Rust ownership and drop behavior.

### Deliverables
- Functional Rust replacement for `xgetcwd`.
- Explicit `Result`-based error propagation.
- No manual memory management or resizing logic retained.

## Phase 3: Add Focused Tests for Behavioral Equivalence
- Add unit tests covering successful retrieval of the current working directory.
- Verify returned data is non-empty and represents an existing path.
- Add tests for any conversion boundary behavior if the API returns `String`.
- Keep tests local to this module and runnable through `cargo test`.

### Deliverables
- Passing unit tests for success-path behavior.
- Validation of the chosen path/string return model.

## Phase 4: Final Integration Cleanup
- Align call sites, if any, to consume the new Rust return type.
- Remove or avoid any compatibility scaffolding no longer needed after direct migration.
- Confirm the module remains a minimal direct port of `xgetcwd.c` without adding unrelated abstractions.

### Deliverables
- Module integrated into the branch cleanly.
- No extra facilities beyond the migrated function.
- Final compile/test pass for the branch.