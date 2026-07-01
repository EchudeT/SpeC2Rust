# Implementation Plan: module_src

## Summary

Port `src/shc.c` to a Rust module on branch `001-module_src-rust-port`, preserving the existing behavior and execution flow without adding new capabilities.

The C file combines argument parsing, ARC4-style state initialization and transformation, file-based keying, script reading/evaluation support, random helper routines, byte-array printing/dumping helpers, code-generation output, orchestration, and process entry. The Rust implementation should keep this as a single focused module migration from the existing file, with functions translated in a near-structural way first, then tightened for idiomatic ownership and error propagation where that does not alter behavior.

Technical approach:
- Translate the single C compilation unit into a single Rust source module with function-level parity.
- Replace raw buffers and manual memory handling with `Vec<u8>`, slices, and owned `String`/`PathBuf` where applicable.
- Replace integer status/error signaling with `Result` for internal Rust flow, while preserving original CLI-visible outcomes in `main`.
- Keep mutable ARC4/stateful routines explicit with a dedicated state struct instead of implicit global-style array manipulation.
- Keep file I/O and argument parsing in the standard library unless a direct need appears during porting.

## Technical Context

- **Language/Version**: Rust 1.78+ stable
- **Primary Dependencies**:
  - Rust standard library
  - No third-party crates required for the initial port
- **Testing**:
  - `cargo test`
- **Performance Goals**:
  - Maintain behavior comparable to the C implementation for typical command-line use
  - Avoid unnecessary allocations in byte-processing paths
  - Keep ARC4/state mutation and byte-dump/code-generation routines linear and buffer-oriented
  - Preserve file I/O simplicity; optimize only where needed to match current behavior

## Module Mapping

### Source File Mapping
- `src/shc.c` -> `src/module_src.rs`

If the Rust binary already uses `src/main.rs`, expose the migrated logic from `src/module_src.rs` and keep `main` as the minimal entrypoint that delegates into the migrated orchestration function. If there is not yet a Rust entrypoint, place the translated entry logic in `src/main.rs` and keep the rest in `src/module_src.rs`.

### Function Mapping
- `parse_an_arg` -> `parse_an_arg`
- `parse_args` -> `parse_args`
- `stte_0` -> `stte_0`
- `key` -> `key`
- `arc4` -> `arc4`
- `key_with_file` -> `key_with_file`
- `eval_shell` -> `eval_shell`
- `read_script` -> `read_script`
- `rand_mod` -> `rand_mod`
- `rand_chr` -> `rand_chr`
- `noise` -> `noise`
- `prnt_bytes` -> `prnt_bytes`
- `prnt_array` -> `prnt_array`
- `dump_array` -> `dump_array`
- `write_C` -> `write_c`
- `make` -> `make`
- `do_all` -> `do_all`
- `main` -> `main`

### Notes on Mapping Decisions
- Preserve function boundaries from the C file to reduce migration risk.
- Use snake_case for Rust functions; only `write_C` should be normalized to `write_c`.
- Keep helper visibility private unless required by tests.
- Keep orchestration ordering (`parse_args` / `do_all` / `make` / output helpers) aligned with the C control flow.

## Data Model

The analysis reports unnamed/anonymous C data structures. During implementation, identify each concrete struct/aggregate in `src/shc.c` and map them directly to named Rust types local to `module_src`.

### Expected Mapping Rules
- Anonymous C struct used for options/configuration -> named Rust `struct` such as `Args` or `Options`
- Anonymous C struct used for ARC4/state arrays -> named Rust `struct Arc4State`
- Anonymous C struct used for generated script/code metadata -> named Rust `struct` with explicit fields
- Anonymous C aggregate used only as grouped constants or mode switches -> Rust `enum` if semantic variants exist, otherwise `struct`
- C string pointers (`char *`, `const char *`) -> `String`, `&str`, or `PathBuf` depending on ownership and filesystem use
- C byte buffers (`unsigned char *`, fixed byte arrays) -> `Vec<u8>` or `[u8; N]` when size is compile-time fixed
- C integer flags -> `bool` or narrow integer types only where exact arithmetic/bit behavior matters
- C size/count/index values -> `usize` unless exact-width compatibility is required by algorithm logic

### Core Rust Structures to Introduce

#### `Arc4State`
Use a dedicated struct for the state manipulated by `stte_0`, `key`, and `arc4`.

Planned shape:
- permutation/state array: `[u8; 256]`
- index fields corresponding to the C implementation: `u8`/`usize` as appropriate

Reason:
- Replaces implicit mutable global-style state with explicit ownership
- Preserves exact mutation order required by the algorithm

#### `Args` / `Options`
Create one struct for parsed command-line state consumed by `do_all`, `make`, and related helpers.

Planned contents:
- option flags
- input/output path fields
- script-related fields
- any mode or verbosity values present in the C code

Reason:
- Consolidates parsed state now likely spread across local variables or anonymous structures
- Makes `parse_an_arg` and `parse_args` return typed results instead of mutating many separate variables

#### Output/Generation Context Structs
Only if the C file already groups data this way, create direct Rust structs for:
- generated array metadata
- dump/print configuration
- shell-evaluation parameters

Do not invent additional abstraction layers beyond what is needed to represent the original C aggregates safely.

## Implementation Phases

## Phase 1: Skeleton Port and Type Extraction

Goals:
- Create the Rust module/file layout
- Identify and name the anonymous C data structures
- Establish type-safe signatures for all migrated functions

Tasks:
- Create `src/module_src.rs`
- Inspect `src/shc.c` and extract all anonymous structs/unions into named Rust `struct`s
- Define the main state carriers:
  - ARC4 state struct
  - parsed-arguments/options struct
  - any existing grouped metadata structs used by generation/output logic
- Translate function signatures with minimal behavioral change
- Decide per-function return types:
  - `Result<T, E>` for fallible operations such as file I/O, script reading, and output generation
  - plain return values for deterministic byte/state transforms
- Establish a small internal error type for module operations using standard library error traits or a simple enum

Completion criteria:
- All C functions have Rust counterparts declared
- All anonymous C data structures have explicit Rust representations
- The module compiles with placeholder or partial bodies where needed

## Phase 2: Core Algorithm and Parsing Migration

Goals:
- Port deterministic logic first
- Preserve state transitions and argument interpretation

Tasks:
- Implement `stte_0`, `key`, and `arc4` against `Arc4State`
- Preserve exact byte-level semantics:
  - wrapping arithmetic
  - index updates
  - in-place permutation behavior
- Implement `parse_an_arg` and `parse_args`
- Map C argv handling to `std::env::args_os` or `std::env::args` depending on whether non-UTF-8 paths/options are relevant in the source
- Keep option parsing behavior aligned with the existing C logic rather than redesigning it around a parsing crate
- Implement `rand_mod`, `rand_chr`, and `noise` using standard-library-compatible randomness already present in the C behavior model; if the C code depends only on simple process-local pseudo-random behavior, keep the logic local rather than introducing a crate

Completion criteria:
- Argument parsing works end-to-end with the migrated option state struct
- ARC4/stateful routines produce stable byte-for-byte results against C-derived expectations
- Random helper routines are ported with the same operational constraints as the source

## Phase 3: I/O, Script Handling, and Output Helpers

Goals:
- Port file and script operations safely
- Preserve byte output formatting and generated content structure

Tasks:
- Implement `key_with_file`
- Implement `read_script`
- Implement `eval_shell`
- Replace raw file descriptors and manual buffers with:
  - `std::fs::File`
  - `std::fs::read`
  - `Read`/`Write`
  - `Path`/`PathBuf`
- Ensure binary/text distinctions remain aligned with C behavior, especially for script bytes and generated C output
- Implement `prnt_bytes`, `prnt_array`, and `dump_array`
- Implement `write_c`, preserving formatting, escaping, and emitted array/content layout expected from the original generator

Memory/error handling focus:
- Avoid unchecked indexing where Rust bounds checks can preserve safety without changing output
- Convert C null/error checks into explicit `Result` propagation
- Preserve output ordering and exact textual formatting where the generated C source depends on it

Completion criteria:
- File-based keying, script reading, and shell-evaluation preparation paths are functional
- Output helpers emit stable text/byte representations matching the source behavior
- All fallible I/O paths return clear Rust errors and are translated to CLI-visible failures only at the top level

## Phase 4: Orchestration, Entry Point, and Validation

Goals:
- Complete top-level control flow
- Validate parity with the C module

Tasks:
- Implement `make`, `do_all`, and `main`
- Preserve the original call order and side-effect sequence
- Keep `main` responsible for:
  - collecting CLI args
  - invoking migrated orchestration
  - mapping Rust errors to exit status and stderr output consistent with current behavior
- Add unit tests for:
  - ARC4 state initialization and transform invariants
  - argument parsing cases derived from the C module behavior
  - byte/array print formatting
- Add fixture-style tests where practical for:
  - script reading
  - file-based keying
  - generated C output shape
- Run `cargo test` until the module compiles and passes all migrated checks

Completion criteria:
- The Rust module fully replaces `src/shc.c` behavior in the Rust branch scope
- Entry-point behavior is wired and test-covered at the function level
- The implementation remains constrained to the existing file/function responsibilities only

## Migration Notes

### Memory Management
- Replace C-owned mutable buffers with `Vec<u8>` and slices
- Replace borrowed string-pointer lifetimes with owned `String`/`PathBuf` at module boundaries where C previously relied on process-lifetime argv storage
- Keep fixed-size algorithm state in stack-allocated arrays where sizes are known and stable

### Error Handling
- Do not preserve C-style sentinel returns internally unless required by a specific algorithm contract
- Use `Result` for file reads, writes, and shell/script preparation
- Convert to exit codes only in `main` or the top orchestration layer

### Behavioral Parity Constraints
- Do not redesign command-line UX
- Do not split functionality into extra helper modules beyond the direct `src/shc.c` migration target
- Do not add concurrency, recovery layers, serialization, or FFI
- Keep generated output and byte-processing logic as close as possible to the original execution model