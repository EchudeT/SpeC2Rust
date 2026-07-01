# Implementation Plan: main_root_setlocale_null_05

## Summary

This module ports the locale-query helpers from `setlocale_null.c` and `setlocale_null-unlocked.c` into Rust, preserving the existing function split between locked and unlocked code paths without adding new behavior. The Rust implementation should focus on reproducing the current call structure and return conventions used by the C module: obtaining the current locale name for a category, handling buffer-based variants, and keeping the public wrapper layering intact.

The implementation should stay close to the C file/function boundaries. Since the analyzed input shows only functions and no custom data structures, the Rust port can remain function-oriented, using standard-library string and buffer types where ownership is clear. Error handling should be explicit through `Result` or `Option` internally, with final public signatures chosen to match the surrounding Rust project conventions for this port branch.

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Keep locale lookup overhead minimal and avoid unnecessary string copies.
  - Preserve the distinction between unlocked and lock-using paths only to the extent required by the existing C implementation structure.
  - Use borrowed string data where feasible internally, allocating only when a stable owned return value or caller-provided buffer handling requires it.

## Module Mapping

### Source File Mapping

- `setlocale_null.c`
  - Migrate to a Rust module file following the crate’s existing layout, preferably `src/main_root_setlocale_null_05.rs` or the nearest equivalent module path already used by the project.
  - Contains the public wrappers and lock-aware helper flow:
    - `setlocale_null_r_with_lock`
    - `setlocale_null_r`
    - `setlocale_null`

- `setlocale_null-unlocked.c`
  - Migrate to a companion Rust module section or adjacent file only if the repository already separates translated files similarly.
  - Contains the unlocked helper flow:
    - `setlocale_null_unlocked`
    - `setlocale_null_r_unlocked`

### Function Mapping

| C Function | Rust Target | Notes |
|---|---|---|
| `setlocale_null_unlocked` | `fn setlocale_null_unlocked(...)` | Internal helper; preserve unlocked lookup behavior. |
| `setlocale_null_r_unlocked` | `fn setlocale_null_r_unlocked(...)` | Internal buffer-writing helper corresponding to unlocked path. |
| `setlocale_null_r_with_lock` | `fn setlocale_null_r_with_lock(...)` | Internal helper; keep as distinct function even if logic is thin. |
| `setlocale_null_r` | `pub(crate) fn setlocale_null_r(...)` or project-equivalent visibility | Wrapper selecting the appropriate helper path. |
| `setlocale_null` | `pub(crate) fn setlocale_null(...)` or project-equivalent visibility | Public-facing module entry point for this translated unit. |

### Notes on Duplicate Listings

The analysis lists `setlocale_null_r_with_lock` multiple times. Treat this as a duplicate extraction artifact, not as multiple separate functions. Only one Rust implementation should be created for that symbol.

## Data Model

No module-specific C structs were identified in the analysis input.

### Type Mapping

Because this module appears function-based, the main C-to-Rust mapping concerns buffers and strings:

| C Concept | Rust Mapping | Notes |
|---|---|---|
| `char *` return for locale name | `String`, `&str`, or `Option<String>` depending on surrounding crate conventions | Prefer owned `String` at public boundaries if lifetime cannot be borrowed safely. |
| Caller-provided output buffer | `&mut [u8]` or `&mut String` | Choose the form that best matches the existing Rust port style in this project. |
| Locale category integer / constants | `i32` or a small Rust enum wrapper if such an enum already exists in the project | Do not introduce a new enum unless needed by existing translated interfaces. |
| Null/error return in C | `Option<_>` or `Result<_, _>` internally | Final wrapper behavior should mirror the C failure path without hidden panics. |

### Memory Management

- Avoid exposing borrowed references tied to transient locale storage unless the lifetime is guaranteed.
- Prefer copying locale names into owned Rust strings when translating C behavior that depends on mutable or process-global locale state.
- For `_r`-style functions, validate destination capacity before writing and return an explicit failure on insufficient space instead of partial unchecked writes.

### Error Handling

- Convert C null/failure conditions into explicit Rust control flow.
- Do not use `unwrap`/`expect` in translation paths.
- Keep the locked and unlocked helper behavior consistent so wrapper functions remain thin and predictable.

## Implementation Phases

## Phase 1: Create Module Skeleton and Map Function Boundaries

- Add the Rust module file for this translation unit in the project’s standard `src/` layout.
- Declare Rust equivalents for:
  - `setlocale_null_unlocked`
  - `setlocale_null_r_unlocked`
  - `setlocale_null_r_with_lock`
  - `setlocale_null_r`
  - `setlocale_null`
- Mirror the C call hierarchy first, even if some functions are temporary stubs.
- Resolve duplicate function listings from the analysis by creating only one `setlocale_null_r_with_lock`.

### Deliverables

- Compiling module skeleton.
- Function signatures aligned with the surrounding ported codebase.
- Clear visibility boundaries between internal helpers and externally used module entry points.

## Phase 2: Port Unlocked and Buffer-Based Core Logic

- Implement `setlocale_null_unlocked` as the core locale-name retrieval helper.
- Implement `setlocale_null_r_unlocked` as the buffer-oriented variant.
- Translate C buffer handling carefully:
  - capacity checks before write
  - explicit terminator-equivalent handling where required by interoperability with existing project conventions
  - no unchecked indexing
- Keep allocations minimal and local to cases where the C code returns newly materialized string data.

### Deliverables

- Working unlocked retrieval path.
- Working `_r` unlocked path with explicit success/failure behavior.
- Unit tests covering:
  - valid locale category input
  - empty/unsupported result handling
  - insufficient output buffer behavior

## Phase 3: Port Lock-Aware Wrapper Path

- Implement `setlocale_null_r_with_lock` by translating the corresponding wrapper logic from `setlocale_null.c`.
- Implement `setlocale_null_r` as the public buffer-based wrapper, preserving the original dispatch sequence.
- Implement `setlocale_null` as the top-level convenience wrapper.
- If the original C code relies on surrounding project locking primitives, reuse only those already present in the Rust port; otherwise, keep the function layering without inventing new synchronization infrastructure.

### Deliverables

- Complete wrapper stack matching the C module structure.
- Public and internal functions connected in the same migration order as the source files.
- Tests validating wrapper equivalence to the unlocked core behavior for normal inputs.

## Phase 4: Finalize Behavioral Parity and Cleanup

- Compare the Rust functions against the original C edge paths:
  - null-like failure cases
  - category propagation
  - return-value conventions between plain and `_r` variants
- Remove any temporary translation scaffolding introduced during earlier phases.
- Keep the final module limited to the original functionality, without adding helper abstractions unrelated to the source files.

### Deliverables

- `cargo test` passing for the module.
- Final code review against C source function-by-function.
- Stable, minimal Rust translation ready for branch `005-main_root_setlocale_null_05-rust-port`.