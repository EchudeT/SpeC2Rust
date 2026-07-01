# Implementation Plan: module_src

## Summary

Port `src/shc.c` into an idiomatic Rust module while preserving the existing behavior and execution flow of the C implementation. The Rust work should stay narrowly scoped to the current file and function set: argument parsing, ARC4-related state handling, file-based keying, script reading/evaluation support, random/noise helpers, byte/array printing and dumping, C output generation, build orchestration, and program entry.

The implementation approach is a direct migration of the existing functions into a single Rust source module, using Rust ownership and slice-based APIs to replace raw pointer manipulation. Mutable cryptographic/stateful routines should use explicit state structs rather than implicit global memory patterns. Fallible file, process, and argument operations should return `Result` and be converted to exit codes only at the top-level entry path. Unsafe Rust should be avoided unless a specific low-level translation point requires it.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**:
  - Rust standard library
  - No third-party crates recommended by default; the analyzed input does not require external dependencies
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve the practical runtime characteristics of the original single-file utility
  - Avoid unnecessary heap allocation in byte-processing paths
  - Keep ARC4/state mutation operations linear and in-place
  - Maintain file and process handling suitable for command-line usage without introducing extra abstraction overhead

## Module Mapping

### C to Rust File Mapping

- `src/shc.c` -> `src/module_src.rs`

### Function Mapping

All existing C functions should be migrated with minimal behavioral expansion and kept close to the original call graph.

- `parse_an_arg` -> `fn parse_an_arg(...) -> Result<..., ...>`
- `parse_args` -> `fn parse_args(...) -> Result<..., ...>`
- `stte_0` -> `fn stte_0(state: &mut Arc4State)`
- `key` -> `fn key(state: &mut Arc4State, data: &[u8])`
- `arc4` -> `fn arc4(state: &mut Arc4State, data: &mut [u8])`
- `key_with_file` -> `fn key_with_file(state: &mut Arc4State, path: &Path) -> Result<(), ...>`
- `eval_shell` -> `fn eval_shell(...) -> Result<..., ...>`
- `read_script` -> `fn read_script(path: &Path) -> Result<Vec<u8>, ...>`
- `rand_mod` -> `fn rand_mod(...) -> ...`
- `rand_chr` -> `fn rand_chr(...) -> ...`
- `noise` -> `fn noise(...) -> ...`
- `prnt_bytes` -> `fn prnt_bytes(...) -> String` or writer-based function
- `prnt_array` -> `fn prnt_array(...) -> String` or writer-based function
- `dump_array` -> `fn dump_array(...) -> Result<..., ...>`
- `write_C` -> `fn write_c(...) -> Result<..., ...>`
- `make` -> `fn make(...) -> Result<..., ...>`
- `do_all` -> `fn do_all(...) -> Result<..., ...>`
- `main` -> `fn main()`

### Rust Module Placement

Use a restrained structure:

- `src/module_src.rs`: migrated implementation from `src/shc.c`
- `src/main.rs`: minimal CLI entry that calls into `module_src` if the crate is binary-based

If the existing Rust crate is already a binary crate, `src/main.rs` may contain the migrated code directly only if that best matches the original single-file shape. Otherwise, prefer `src/module_src.rs` plus a thin `main`.

## Data Model

Because the analysis only reports anonymous C data structures, the Rust mapping should be derived from actual usage during migration rather than inventing new abstractions.

### C Struct to Rust Mapping

- `anonymous` used for ARC4 or mutable cipher state
  -> `struct Arc4State { ... }`
  - Expected fields should be translated from the original state array/counters exactly
  - Use fixed-size arrays where the C code uses fixed-size buffers
  - Use `u8`, `usize`, or narrower integer types as dictated by indexing semantics

- `anonymous` used for parsed command-line configuration
  -> `struct ArgsConfig { ... }`
  - Store flags, paths, optional values, and mode switches inferred from `parse_an_arg` / `parse_args`
  - Prefer `Option<String>` / `Option<PathBuf>` for optional owned values
  - Preserve original defaulting behavior

- `anonymous` used for output-generation context
  -> `struct OutputConfig { ... }`
  - Holds formatting/output-related settings if the C code groups them
  - Keep only fields already present in the C code

- `anonymous` used for script or buffer metadata
  -> `struct ScriptBuffer { data: Vec<u8>, ... }`
  - Add metadata fields only if directly present in the C struct or required to preserve semantics

- `anonymous` used for build/execution parameters
  -> `struct BuildConfig { ... }`
  - Maps directly from `write_C`, `make`, `do_all`, or `eval_shell` usage

### Primitive and Memory Mappings

- `char *` / byte buffers -> `Vec<u8>` for owned mutable data, `&[u8]` for borrowed input
- C strings used as paths or command arguments -> `String` / `PathBuf`
- fixed C arrays -> fixed Rust arrays `[T; N]` where size is compile-time constant
- nullable pointers -> `Option<T>` / `Option<&T>` / `Option<PathBuf>` as applicable
- integer flags -> `bool` or explicit integer type if exact arithmetic/bit behavior matters

### Error Handling Mapping

- C sentinel returns / exit-style propagation -> `Result<T, ModuleError>`
- Define a single local error enum such as `ModuleError` to represent:
  - invalid arguments
  - file I/O failures
  - process execution failures
  - encoding/formatting failures
  - internal translation mismatches where the C code relied on assumptions

Top-level `main` should translate errors into messages and process exit status, preserving the CLI-oriented behavior.

## Implementation Phases

## Phase 1: Establish Rust module skeleton and migrate state/data definitions

- Create `src/module_src.rs`
- Identify all anonymous C structs in `src/shc.c` and define direct Rust equivalents with field-for-field intent
- Introduce a restrained local error type for fallible operations
- Port constant data, static tables, and fixed-size buffers exactly
- Decide whether `src/main.rs` will be a thin wrapper or host the migrated code directly
- Add compilation scaffolding with placeholder function signatures for all listed functions to preserve migration order

### Deliverables

- Compiling Rust module skeleton
- Direct Rust struct definitions for all C data structures used by the file
- Clear ownership boundaries for buffers, paths, and mutable state

## Phase 2: Port core stateful and utility routines

- Migrate `stte_0`, `key`, and `arc4` first because other logic depends on them
- Port `rand_mod`, `rand_chr`, and `noise` with attention to integer width, modulo behavior, and reproducibility expectations from the original code
- Port `prnt_bytes`, `prnt_array`, and `dump_array`
  - Prefer writer- or `String`-based formatting rather than C-style buffer writes
  - Preserve output format exactly where these functions generate code or byte literals
- Add focused unit tests for the deterministic routines where expected outputs can be captured from the C implementation or inferred from stable logic

### Deliverables

- Functional byte/state helpers in Rust
- Deterministic tests for ARC4/state initialization and formatting helpers
- No raw-pointer-style memory handling remaining in these areas

## Phase 3: Port argument parsing and file-based input paths

- Migrate `parse_an_arg` and `parse_args`
- Replace pointer/index walking over `argv` with `std::env::args_os` or passed-in argument slices while preserving original parsing order and failure behavior
- Port `read_script` using `std::fs::read`
- Port `key_with_file` using standard file APIs and exact byte handling
- Ensure all path-bearing operations use `Path`/`PathBuf` and do not assume UTF-8 unless the original behavior requires string interpretation
- Add unit tests for representative valid/invalid argument combinations and file-reading edge cases

### Deliverables

- End-to-end argument decoding into Rust config structs
- File-backed keying and script reading implemented with `Result`-based error propagation
- Tests for parse behavior and file I/O translation

## Phase 4: Port execution/output orchestration and wire program entry

- Migrate `eval_shell`, `write_C`, `make`, `do_all`, and `main`
- Use `std::process::Command` for shell/build invocation only to the extent the C code already performs external execution
- Preserve command construction and exit/error behavior without adding new execution features
- Ensure generated output files and intermediate artifacts follow the original path and naming logic
- Make `main` responsible only for invoking the migrated workflow and converting errors to CLI-visible output and exit status
- Add integration-style tests where practical for non-destructive paths; keep destructive/external-command behavior isolated behind small functions for testability

### Deliverables

- Full migrated control flow from CLI entry through generation/build orchestration
- Program-level error handling aligned with Rust conventions and original utility behavior
- `cargo test` coverage for deterministic logic and selected orchestration paths