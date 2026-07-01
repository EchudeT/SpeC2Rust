# Implementation Plan

## Summary

This module ports `gnu/getprogname.c` into Rust with a minimal, behavior-preserving implementation focused on exposing the current program name through a Rust function corresponding to `getprogname`.

The Rust implementation should stay narrowly scoped to the existing C file and function, avoiding expansion into unrelated process-inspection utilities. The preferred technical approach is to derive the executable or invocation name from the standard library (`std::env`) and return it in a form suitable for internal project use. If the surrounding Rust port requires C-like nullability semantics, the implementation should represent absence explicitly with `Option`.

Key technical goals:

- Migrate only the behavior represented by `getprogname`.
- Use the Rust standard library by default.
- Avoid global mutable state unless the original project architecture already requires cached initialization.
- Handle platform string conversion safely, especially for non-UTF-8 executable names.
- Keep ownership and lifetime rules explicit so no borrowed value outlives its source buffer.

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Constant-time access after any optional one-time initialization.
  - Negligible allocation overhead; at most one owned string/path extraction per call or one cached allocation if project conventions require caching.
  - No unnecessary copying beyond what is needed to convert OS strings safely.

## Module Mapping

| C Source File | C Function | Rust Module/File | Rust Item |
|---|---|---|---|
| `gnu/getprogname.c` | `getprogname` | `src/module_gnu_getprogname.rs` | `pub fn getprogname(...) -> ...` |

### Mapping Notes

- Keep the Rust implementation in a single module corresponding directly to the source C file.
- Do not split helper logic into additional modules unless required by the crate’s existing layout.
- If the broader port already groups translated GNU helpers under a common module tree, place this file there without introducing new abstraction layers.

## Data Model

The C analysis reports only anonymous data structures and no named persistent module state. The Rust port should therefore avoid inventing new public data types unless required by signature adaptation.

| C Data Structure | Rust Mapping | Notes |
|---|---|---|
| anonymous | no dedicated Rust type | Treat as implementation-local temporary values only |
| anonymous | no dedicated Rust type | Use standard library path/string types instead |

### Value Representation

Because `getprogname` in C typically exposes a process name string pointer, the Rust port should choose one of the following representations based on surrounding crate conventions:

- `Option<String>` if the port is expressed idiomatically in Rust and absence must be modeled safely.
- `Option<std::ffi::OsString>` if preserving non-UTF-8 platform names matters.
- `&'static str` or similar only if the project already establishes a cached static program name and can justify the lifetime safely.

Preferred internal types:

- `std::path::PathBuf` for executable path retrieval.
- `std::ffi::OsString` for platform-correct filename extraction.
- `Option<T>` for nullable C outcomes.

## Implementation Phases

### Phase 1: Inspect and Define the Rust Signature

- Review how the surrounding Rust port expresses translated C string-returning helpers.
- Determine the narrowest compatible Rust signature for `getprogname`.
- Create `src/module_gnu_getprogname.rs` and add the direct function stub.
- Document any unavoidable semantic difference from C pointer-based returns, especially around nullability and ownership.

**Exit criteria**:
- Rust module file exists.
- Function signature is fixed and consistent with crate conventions.
- No extra public API beyond the translated function.

### Phase 2: Implement Program Name Resolution

- Implement program name lookup using `std::env::args_os().next()` as the primary source when invocation name semantics are desired.
- If crate requirements instead align with executable filename semantics, use `std::env::current_exe()` and extract `file_name()`.
- Normalize the result to the selected return type without assuming UTF-8.
- Ensure empty or unavailable names map cleanly to `None` or the crate’s equivalent absence value.
- Keep memory ownership fully within Rust; do not expose references tied to temporary buffers.

**Exit criteria**:
- Function returns the program name in the chosen Rust representation.
- Missing data and non-UTF-8 cases are handled safely.
- No mutable global state is introduced unless strictly required by existing architecture.

### Phase 3: Integrate Error Handling and Lifetime Safety

- Confirm that all fallible standard-library calls are handled without panics in normal operation.
- If conversion to `String` is required, use lossy conversion only if the existing port accepts that behavior; otherwise retain `OsString`.
- If a borrowed/static return is mandated by crate design, introduce a minimal one-time initialization approach using standard library primitives and clearly bound ownership.
- Verify that no dangling references or invalid lifetime assumptions remain.

**Exit criteria**:
- All fallible paths are explicit.
- Ownership model is documented in code comments where needed.
- Implementation compiles cleanly with no unsafe code unless existing project constraints make it unavoidable.

### Phase 4: Add Focused Tests

- Add unit tests covering:
  - normal retrieval of a non-empty program name,
  - behavior when only a path is present and basename extraction is required,
  - absence/failure path if it can be simulated through internal helpers,
  - non-UTF-8 tolerant handling where practical for the target platform.
- Keep tests scoped to this module and its direct semantics.
- Run `cargo test` and fix any platform-specific assumptions.

**Exit criteria**:
- Module tests pass under `cargo test`.
- Tests validate the selected nullability and string-type behavior.
- No unrelated fixtures or integration harnesses are added.

## Memory Management and Error Handling Notes

- Prefer owned Rust values (`String`, `OsString`, `PathBuf`) over borrowed pointers from temporary environment/query results.
- Use `Option` to represent null-like outcomes from the C API.
- Avoid `unwrap`/`expect` in runtime logic.
- If caching is required to emulate stable repeated returns, use a standard-library one-time initialization primitive and store an owned value whose lifetime is intentionally extended by the cache.
- Do not introduce manual memory management or unsafe pointer handling unless the crate’s existing translation boundary explicitly requires it.

## Completion Criteria

The port is complete when:

- `gnu/getprogname.c` is mapped to one Rust source file and one Rust function.
- The Rust implementation preserves the original module’s narrow responsibility.
- Program name retrieval is safe with respect to ownership, nullability, and platform string handling.
- `cargo test` passes for the translated module.