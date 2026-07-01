# Implementation Plan: `module_src_main.c_26`

## Summary

This module is the Rust port of `src/main.c`, centered on process startup, option interpretation, initialization, configuration parsing, symbol filtering, and program entry orchestration. The implementation should preserve the existing execution flow and migrate the current file-local behavior into a single Rust module aligned with the original scope.

The Rust approach should keep the port narrow:

- translate the existing top-level control flow in `main`
- migrate helper routines for option classification, numeric parsing, level parsing, tilde expansion, rc/config parsing, and symbol selection
- replace C global state and anonymous aggregates with explicit Rust structs owned by the module
- use `Result`-based error propagation internally, with a single process-exit boundary in `main`
- avoid introducing additional architectural layers beyond what is required to represent existing state safely

The preferred shape is a single Rust source module corresponding to the C file, with internal helper functions and private state containers that mirror the original responsibilities.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only by default
  - `std::env` for argument handling
  - `std::fs` for rc/config file reading
  - `std::path` for path and tilde-expansion support
  - `std::process` for exit handling
  - `std::num` and string parsing APIs for numeric conversion
- **Testing**: `cargo test`
- **Performance Goals**:
  - maintain command-line startup performance comparable to the C implementation
  - avoid unnecessary heap allocation during option scanning and parsing
  - keep file parsing single-pass where the C logic is single-pass
  - use borrowed string slices where possible, materializing owned `String` values only when state retention is required

## Module Mapping

### C to Rust File Mapping

- `src/main.c` -> `src/main.rs` or `src/bin/cflow-new.rs` depending on current crate layout

If the Rust project is a single binary crate, prefer keeping the port in `src/main.rs` to match the source role directly. If the repository already uses a library-plus-binary arrangement, place migrated logic in the existing binary entry file without creating new layers not required by the C source.

### Function Mapping

| C Function | Rust Mapping | Notes |
|---|---|---|
| `CHAR_TO_SM` | private helper function or small inline conversion | Preserve exact character-to-mode/category mapping semantics. |
| `find_option_type` | private function | Return an enum instead of integer/string discriminator where possible. |
| `symbol_override` | private function mutating module state | Convert pointer/state mutation into `&mut` updates on explicit structs. |
| `number` | private parsing helper | Return `Result<integer_type, ParseError>`. |
| `parse_level_string` | private parsing helper | Represent parsed level data with enum/bitflag-style Rust type as needed by existing logic. |
| `tildexpand` | private path helper | Return `PathBuf` or `String` depending on downstream use. |
| `parse_rc` | private function | Read config/rc input and update mutable runtime state. |
| `globals_only` | private predicate/helper | Preserve current filtering logic as a pure function where possible. |
| `include_symbol` | private predicate/helper | Return `bool`; consume explicit symbol/config state instead of globals. |
| `xalloc_die` | error-to-exit helper or eliminated | Replace allocation-failure fatal path with standard Rust allocation behavior; keep explicit fatal helper only if needed for message compatibility. |
| `init` | private initialization function | Build runtime state explicitly and return `Result<State, Error>` or mutate passed state. |
| `main` | crate entry point | Parse args, call init/config helpers, execute main flow, map errors to stderr + exit code. |

## Data Model

The source analysis exposes multiple anonymous C data structures. Since names are unavailable, the Rust plan should map them by role discovered during migration rather than preserving anonymous layout literally.

### Data Structure Mapping Strategy

| C Construct | Rust Mapping | Migration Rule |
|---|---|---|
| anonymous struct used for runtime options | named `struct RuntimeOptions` | Collect command-line and rc-derived option state in one explicit owner. |
| anonymous struct used for initialization/program state | named `struct AppState` | Hold mutable state previously carried via globals/statics. |
| anonymous struct used for symbol selection/filtering | named `struct SymbolFilter` | Model include/exclude/global-only behavior. |
| anonymous struct used for option descriptors | named `enum OptionType` plus optional descriptor struct | Prefer enums over integer tags. |
| anonymous struct used for parsed level representation | named `struct LevelSpec` or `enum LevelSpec` | Choose based on whether the C logic stores flags/ranges versus a single mode. |
| anonymous list/node-like aggregates | `Vec<T>` or small named structs | Replace manual linked or array management with owned collections only where already implied by the C logic. |
| C strings (`char *`) | `String`, `&str`, `PathBuf` | Borrow during parsing, own when retained in state. |
| C integer status/error codes | `Result<T, ModuleError>` and small enums | Preserve external exit behavior while making internal failures explicit. |
| C globals/statics | fields on `AppState` / `RuntimeOptions` | Eliminate unsafe global mutation in favor of owned module state. |

### Expected Rust Types

The exact field list should be derived during source migration, but the implementation should converge on a restrained set such as:

- `struct AppState`
- `struct RuntimeOptions`
- `struct SymbolFilter`
- `enum OptionType`
- `enum ModuleError`
- `struct LevelSpec` or `enum LevelSpec`

### Memory Management Notes

- Replace all manual allocation paths with owned Rust containers (`String`, `Vec`, `PathBuf`, `Box` only if structurally necessary).
- Remove explicit free/error-on-null allocation patterns from normal flow.
- Any C routines that relied on mutable shared buffers should become local variables or owned fields with clear borrowing boundaries.
- Avoid `unsafe` unless the existing repository interface forces it; none is implied by the module description alone.

### Error Handling Notes

- Parsing helpers such as `number`, `parse_level_string`, and `parse_rc` should return `Result`.
- Fatal user-facing failures should be reported once in `main`.
- If the original `xalloc_die` emits a specific fatal message, preserve message behavior through a normal Rust error path rather than re-creating allocator hooks.
- Functions that were boolean/int status returns in C may remain `bool` only if they are true predicates; otherwise convert to `Result` for parse and I/O operations.

## Implementation Phases

## Phase 1: Skeleton Port and State Extraction

- Create the Rust destination file for the `src/main.c` port in the existing crate layout.
- Introduce the minimum named Rust state types required to replace file-scope globals and anonymous structs:
  - `AppState`
  - `RuntimeOptions`
  - `SymbolFilter`
  - `OptionType`
  - `ModuleError`
- Port `main` and `init` first as the top-level control skeleton.
- Convert C global initialization into explicit Rust constructors/default initialization.
- Establish the single error-reporting boundary in `main`.

**Exit criteria**:
- Rust binary compiles.
- Program entry and initialization flow exist with placeholder-equivalent logic wired through explicit state.
- No remaining dependence on implicit global mutable state for migrated portions.

## Phase 2: Parsing Helpers and Option Logic

- Port helper routines in the order they support startup behavior:
  - `CHAR_TO_SM`
  - `find_option_type`
  - `number`
  - `parse_level_string`
  - `tildexpand`
- Replace integer/tag-based branching with Rust enums where behavior is unchanged.
- Keep parser behavior close to C, especially around invalid input acceptance/rejection and boundary conditions.
- Add focused unit tests for:
  - numeric parsing edge cases
  - level-string parsing cases
  - option classification behavior
  - tilde expansion for supported input forms

**Exit criteria**:
- Core helper routines are fully migrated and covered by unit tests.
- Parsing behavior is deterministic and represented without raw pointer manipulation.

## Phase 3: Configuration and Symbol Selection Migration

- Port `parse_rc` using `std::fs` and standard string processing.
- Port `globals_only`, `include_symbol`, and `symbol_override`.
- Move all symbol-selection decisions onto explicit Rust state passed by reference.
- Preserve current precedence between command-line state, rc-derived state, and symbol override behavior.
- Add unit tests for rc parsing and symbol filter decisions based on representative inputs.

**Exit criteria**:
- Rc/config parsing updates runtime state correctly.
- Symbol inclusion and override logic run without globals or pointer-based mutation.
- File I/O and parse failures are surfaced as structured errors.

## Phase 4: Integration Cleanup and Behavioral Verification

- Remove or fold `xalloc_die` into normal Rust fatal error handling unless a dedicated compatibility message path is required.
- Align return codes and stderr output with existing observable behavior from `main`.
- Review all migrated functions for unnecessary allocation and tighten borrowed/owned string usage.
- Add integration-style tests for end-to-end startup scenarios that exercise:
  - argument handling
  - rc loading path
  - symbol filter setup
  - invalid input failure paths

**Exit criteria**:
- All listed functions from `src/main.c` are migrated or intentionally eliminated through direct Rust equivalents.
- `cargo test` passes.
- Module behavior is consolidated into the Rust entry module with no extra unsupported abstractions added.