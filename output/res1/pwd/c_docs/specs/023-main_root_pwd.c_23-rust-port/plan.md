# Implementation Plan: main_root_pwd.c_23

## Summary

This module ports the `pwd.c` main-cluster logic for the `usage` and `nth_parent` functions into Rust on branch `023-main_root_pwd.c_23-rust-port`.

The Rust implementation should stay narrowly aligned with the existing C file structure and behavior:
- migrate the path-processing logic of `nth_parent` into safe Rust string/path handling,
- migrate `usage` into a Rust function that emits the same command-help text through standard output/error paths used by the binary,
- preserve current control flow and return/error behavior expected by the surrounding `pwd` program.

Technical approach:
- implement the migrated logic in the Rust binary source corresponding to `pwd.c`,
- prefer `std::path`, `std::ffi`, and standard string slicing/iteration for parent-path computation,
- avoid introducing extra abstraction layers beyond what is needed to replace the C functions safely,
- replace C pointer arithmetic and manual buffer reasoning with bounds-checked Rust operations,
- represent failure through `Result` where it affects internal helpers, converting to process-facing exit behavior only at the binary boundary.

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**:
  - Rust standard library
  - No third-party crates are required based on the provided module analysis
- **Testing**: `cargo test`
- **Performance Goals**:
  - Maintain behavior comparable to the C implementation for normal command-line use
  - Avoid unnecessary path allocations where straightforward borrowing or in-place derivation is possible
  - Keep startup and path-processing overhead minimal and appropriate for a small CLI utility

## Module Mapping

### C to Rust File Mapping

- `pwd.c` -> `src/bin/pwd.rs`
  If the project already uses a different binary entry layout, place the migrated functions in the existing Rust file that corresponds to the `pwd` executable rather than creating additional modules.

### Function Mapping

- `usage` -> `fn usage(...)`
  - Rust function local to the `pwd` binary implementation
  - Responsible for emitting help/usage text and returning or exiting according to existing binary conventions
- `nth_parent` -> `fn nth_parent(...) -> ...`
  - Rust helper in the same file/module as the binary logic
  - Reimplemented using safe Rust path/string traversal rather than mutable C buffers and pointer stepping

## Data Model

The analysis reports only anonymous C data structures and does not identify named structs directly tied to the target functions. For this module, the expected migration focus is function-level logic rather than struct-heavy translation.

### Data-structure Mapping

- Anonymous C internal data/temporary state -> Rust local variables and standard-library types
  - C string pointers / character buffers -> `String`, `&str`, `PathBuf`, or `&Path`
  - Pointer/index traversal state -> `usize` indices, iterators, or `Components`
  - Integer counters for parent depth -> `usize` or other unsigned integer type matching usage
  - Status/return codes -> `Result<T, E>` internally, with conversion to process exit semantics at the outer layer

### Ownership and Memory Notes

- Replace all manual lifetime and buffer management with Rust-owned strings or borrowed string/path slices.
- Avoid exposing raw pointers.
- Ensure parent-path derivation does not rely on unchecked indexing into UTF-8 strings unless operating on byte slices with explicit boundary care.
- Prefer `Path`/`PathBuf` when preserving filesystem semantics matters; use `String`/`&str` only if the original logic is purely textual and must remain so.

## Implementation Phases

## Phase 1: Establish Rust File Placement and Port `usage`

- Identify the Rust binary file corresponding to `pwd.c`.
- Add the `usage` function in the same file/module as the current CLI entry logic.
- Map C output behavior to Rust standard I/O calls without changing visible command behavior.
- Keep signatures and call sites minimal and aligned with the existing Rust binary structure.
- Add unit coverage for usage-path invocation only if the current Rust project structure already supports testing emitted text locally without extra scaffolding.

## Phase 2: Port `nth_parent` Logic Safely

- Translate the `nth_parent` algorithm directly from C into a Rust helper in the same module.
- Replace pointer arithmetic and mutable buffer scanning with:
  - `Path` parent traversal when semantics are path-oriented, or
  - explicit bounded string/byte traversal when the C logic depends on textual slash processing.
- Preserve edge-case behavior:
  - root paths,
  - shallow paths with requested depth beyond available parents,
  - trailing separators if relevant to the original logic,
  - empty or minimal path forms as handled in C.
- Return a Rust type that makes invalid states explicit instead of relying on null pointers or sentinel values.

## Phase 3: Integrate with Existing `pwd` Control Flow

- Wire `usage` and `nth_parent` into the current Rust port of the `pwd` main logic.
- Ensure all former C call sites are mapped without adding new feature flags or alternate execution paths.
- Convert helper errors into the same user-visible failure behavior expected by the CLI.
- Verify that path output remains stable for the code paths that depend on parent computation.

## Phase 4: Validation and Cleanup

- Add focused tests for `nth_parent` covering representative path-depth combinations.
- Run `cargo test` and fix behavioral mismatches against the C implementation intent.
- Remove any temporary compatibility code introduced during translation.
- Confirm the final Rust code uses only safe abstractions unless a clearly unavoidable low-level operation is required; if so, isolate it and document why.