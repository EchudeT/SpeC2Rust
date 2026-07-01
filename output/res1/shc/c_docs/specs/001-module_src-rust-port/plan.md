# Implementation Plan: module_src

## Summary

Port `src/shc.c` into a Rust module while preserving the current module boundary and function-oriented behavior. The Rust implementation should remain centered on the existing responsibilities visible from the C file: argument parsing, internal state initialization and mutation, script/file reading, byte/array output helpers, code generation/writing steps, orchestration, and process entry handling.

The technical approach is a direct migration of the current C logic into a single Rust source module first, with narrow internal type extraction only where needed to replace anonymous C structures and pointer-based state. Unsafe code should be avoided unless a specific C behavior cannot be represented with standard Rust ownership and slice handling. Error-prone C patterns such as raw buffers, unchecked indexing, null pointers, and implicit integer conversions should be converted into explicit Rust types, bounds-checked collection access, and `Result`-based propagation.

## Technical Context

- **Language/Version**: Rust stable, edition 2021, targeting Rust 1.76+.
- **Primary Dependencies**:
  - Rust standard library.
  - No third-party crates are recommended by default because the provided input does not require external parsing, crypto replacement, or CLI frameworks.
- **Testing**:
  - `cargo test`
  - Unit tests for migrated helper functions where behavior can be isolated.
  - Module-level tests for argument parsing and byte/array formatting output.
- **Performance Goals**:
  - Maintain behavior and practical runtime characteristics comparable to the C implementation for the current module scope.
  - Avoid unnecessary allocations during byte processing and file reading beyond what safe Rust requires.
  - Keep state mutation and array traversal linear and allocation-light.
  - Prefer in-place mutation of buffers and state arrays where the C code currently does so.

## Module Mapping

### Source File Mapping

| C File | Rust File | Notes |
|---|---|---|
| `src/shc.c` | `src/module_src.rs` or `src/shc.rs` | Implement as one Rust module matching the existing single-file C scope. |
| `src/shc.c` entry logic | `src/main.rs` | Keep process entry in Rust main and delegate to migrated module functions. |

### Function Mapping

| C Function | Rust Target | Migration Notes |
|---|---|---|
| `parse_an_arg` | `fn parse_an_arg(...) -> Result<..., ...>` | Convert string/pointer parsing into `&str` / `String` processing. |
| `parse_args` | `fn parse_args(...) -> Result<..., ...>` | Use `std::env::args` or passed iterator; return explicit parsed state. |
| `stte_0` | `fn stte_0(state: &mut ...)` | Keep as internal state reset/init function. |
| `key` | `fn key(state: &mut ..., data: &[u8])` | Replace raw pointer/length pairs with slices. |
| `arc4` | `fn arc4(state: &mut ..., data: &mut [u8])` | Preserve in-place buffer transformation. |
| `key_with_file` | `fn key_with_file(state: &mut ..., path: &Path) -> Result<(), ...>` | Use filesystem APIs and explicit IO errors. |
| `eval_shell` | `fn eval_shell(...) -> Result<..., ...>` | Keep process/shell execution logic scoped to current behavior only. |
| `read_script` | `fn read_script(path: &Path) -> Result<Vec<u8>, ...>` | Replace manual file buffer management with `std::fs::read`. |
| `rand_mod` | `fn rand_mod(...) -> ...` | Preserve algorithm; use explicit integer types. |
| `rand_chr` | `fn rand_chr(...) -> ...` | Keep deterministic behavior relative to ported random routine. |
| `noise` | `fn noise(...) -> ...` | Migrate directly; keep local to module if not externally needed. |
| `prnt_bytes` | `fn prnt_bytes(...) -> String` or writer-based function | Prefer formatting into writer/string instead of stdout side effects unless current flow requires direct output. |
| `prnt_array` | `fn prnt_array(...) -> String` or writer-based function | Same approach as above. |
| `dump_array` | `fn dump_array(...) -> Result<(), ...>` | Map file/output interactions to `Write` or filesystem APIs. |
| `write_C` | `fn write_c(...) -> Result<(), ...>` | Preserve generation/writing order; rename to snake_case. |
| `make` | `fn make(...) -> Result<(), ...>` | Keep orchestration semantics. |
| `do_all` | `fn do_all(...) -> Result<(), ...>` | Preserve top-level module workflow. |
| `main` | `fn main()` | Thin entry point that maps errors to exit codes/messages. |

## Data Model

The input only identifies anonymous C structures. The Rust port should replace them with named internal structs only when required to hold migrated state cleanly.

### C-to-Rust Structure Mapping

| C Data Structure | Rust Data Structure | Notes |
|---|---|---|
| anonymous state struct used by initialization/key/arc4 routines | `struct Arc4State` | Holds byte state array and index/counter fields with explicit integer types. |
| anonymous parsed-argument/config struct | `struct ParsedArgs` | Collect parsed flags, file paths, and mode selections now passed around explicitly. |
| anonymous script/input metadata struct | `struct ScriptInput` | Holds script bytes and related path/name fields if present in current C flow. |
| anonymous output/codegen options struct | `struct OutputSpec` | Represents output names/targets currently derived from arguments. |
| anonymous transient generation/work struct | `struct WorkState` | Only introduce if several C locals currently travel across functions; otherwise keep locals. |

### Primitive and Memory Mapping

| C Pattern | Rust Mapping | Notes |
|---|---|---|
| `char *` string input | `String`, `&str`, or `PathBuf` | Choose `PathBuf` for file paths and `String` for owned textual values. |
| `(ptr, len)` byte buffers | `&[u8]` / `&mut [u8]` / `Vec<u8>` | Prefer slices for borrowed data and `Vec<u8>` for owned buffers. |
| fixed-size byte arrays | `[u8; N]` | Suitable for internal state tables. |
| mutable global/static-style state in function scope | owned struct passed as `&mut` | Avoid hidden shared mutable state. |
| C integer flags | `bool` or small enums | Use enums when mutually exclusive modes exist. |
| sentinel/error return codes | `Result<T, ModuleError>` | Preserve failure points but make them explicit. |

### Error Model

Introduce a module-local error type, likely:

```rust
enum ModuleError {
    Io(std::io::Error),
    InvalidArgs(String),
    InvalidData(String),
    ProcessFailure(String),
}
```

This should stay minimal and reflect only currently observed categories from the migrated functions. Avoid adding generalized infrastructure beyond what this module needs.

## Implementation Phases

### Phase 1: Skeleton Port and Type Extraction

- Create the Rust module file corresponding to `src/shc.c`.
- Move process entry handling into `src/main.rs` and delegate to the Rust module.
- Identify all anonymous C structs and replace them with the minimum required named Rust structs.
- Define core aliases, enums, and `ModuleError`.
- Port signatures for all listed functions into snake_case Rust forms.
- Convert raw buffers and pointer-length pairs to slices, arrays, and `Vec<u8>`.
- Keep function boundaries close to the C original to reduce migration risk.

### Phase 2: Core Logic Migration

- Port argument parsing logic from `parse_an_arg` and `parse_args`.
- Port stateful byte-processing functions: `stte_0`, `key`, and `arc4`.
- Port file-dependent helpers: `key_with_file` and `read_script`.
- Port deterministic/randomized helper routines: `rand_mod`, `rand_chr`, and `noise`.
- Replace manual memory handling with owned buffers and scoped borrows.
- Replace C-style error returns with `Result` propagation while preserving failure points and control flow.

### Phase 3: Output and Generation Flow

- Port formatting/output helpers: `prnt_bytes`, `prnt_array`, and `dump_array`.
- Port generation/writing routines: `write_c`, `make`, and `do_all`.
- Use `std::fs` and `std::io::Write` in place of direct C file APIs.
- Preserve output order, textual layout, and file-write sequencing expected from the existing implementation.
- Keep naming and orchestration close to the original flow; avoid introducing extra layers.

### Phase 4: Entry Integration and Verification

- Port `eval_shell` and complete `main` integration.
- Map module errors to user-visible messages and non-zero exit behavior in a minimal way.
- Add focused unit tests for:
  - argument parsing,
  - state initialization and mutation,
  - byte/array formatting helpers,
  - file reading behavior with temporary test inputs.
- Add regression-style tests for migrated workflows where deterministic outputs can be asserted.
- Run `cargo test` and resolve any behavior drift caused by integer conversion, indexing, or newline/formatting differences.