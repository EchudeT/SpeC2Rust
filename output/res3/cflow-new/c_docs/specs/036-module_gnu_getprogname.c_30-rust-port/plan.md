# Implementation Plan: module_gnu_getprogname.c_30

## Summary

Port `gnu/getprogname.c` into an idiomatic Rust module that preserves the existing module scope: exposing the equivalent of `getprogname` without adding broader process-management features. The Rust implementation should prefer the standard library and derive the program name from `std::env::args_os()` / current executable path as needed, returning a borrowed or owned Rust string type appropriate to the surrounding project API.

The implementation should stay narrowly aligned with the original C file’s responsibility: retrieving the current program name. The Rust port should explicitly handle platform string conversion boundaries (`OsStr`/`OsString` vs UTF-8 `String`) and avoid unsafe code unless an existing project interface makes it unavoidable. Memory management will rely on Rust ownership instead of static mutable state from C.

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Constant or near-constant overhead per call beyond process argument access
  - No unnecessary heap allocations if the selected API can return a borrowed value
  - Behavior should remain lightweight and suitable for repeated calls
  - No regression relative to the original C helper’s narrow utility role

## Module Mapping

- **C source**: `gnu/getprogname.c`
- **Rust target**: `src/module_gnu_getprogname.rs`
  - Contains the Rust implementation of `getprogname`
  - Keeps logic local to this module rather than introducing extra helper modules
- **Public API mapping**:
  - `getprogname` -> `pub fn getprogname(...) -> ...`
- **Internal scope**:
  - Any path-basename extraction helper should remain private inside `src/module_gnu_getprogname.rs`
  - No expansion into unrelated process-introspection APIs

## Data Model

The C analysis lists only anonymous data structures and a single function. For this module, no durable domain model appears necessary.

- **C anonymous structures** -> **No direct Rust struct required**
  - If the original C file uses file-local temporary aggregates or unnamed storage, represent them with:
    - local variables, or
    - a private tuple / small private helper type only if required by the translated control flow
- **Program name representation**
  - C string pointer / static storage semantics -> Rust `String`, `OsString`, `PathBuf`, or `&str`-like view depending on the project-facing API
  - Prefer `OsString`/`OsStr` internally when reading process arguments or executable paths
  - Convert to `String` only at the API boundary if the surrounding crate requires UTF-8 text

## Implementation Phases

### Phase 1: Inspect API shape and create module skeleton
- Confirm how the Rust crate expects this functionality to be exposed:
  - standalone function,
  - crate-visible helper, or
  - public module function matching existing naming conventions
- Add `src/module_gnu_getprogname.rs`
- Define the minimal Rust function signature for `getprogname` based on the project’s existing call sites or planned module integration
- Decide the return type before coding:
  - prefer a non-panicking type,
  - avoid global mutable state,
  - keep compatibility with expected caller usage

### Phase 2: Port retrieval and basename logic
- Implement program-name retrieval using the Rust standard library:
  - first choice: `std::env::args_os().next()`
  - fallback if needed: `std::env::current_exe()`
- Extract the basename equivalent of the original C logic using:
  - `std::path::Path`
  - `.file_name()`
- Handle non-UTF-8 process names deliberately:
  - preserve as `OsString` internally,
  - convert lossy only if a `String` return type is required
- Ensure behavior for missing or empty argv-like data is explicit:
  - return empty/optional/result form according to the chosen crate API
- Keep memory handling fully owned/borrowed through Rust types; do not reproduce C-style static buffers

### Phase 3: Integrate and align error handling
- Wire the new module into the crate using standard Rust module declarations only
- Match project error-handling conventions:
  - use `Option` for absent program name if callers can tolerate absence, or
  - use `Result` only if the project already expects fallible propagation here
- Remove any need for manual lifetime or pointer management
- Verify there is no hidden dependence on process-global mutable state from the C implementation

### Phase 4: Add focused tests and finalize migration
- Add unit tests for:
  - basename extraction from a simple executable name
  - path-form executable name reducing to filename
  - empty or unavailable input path handling via helper-level tests where direct process-state control is impractical
  - non-UTF-8 tolerant path handling if the chosen API exposes `OsStr`/`OsString`
- Run `cargo test`
- Confirm the port remains limited to `gnu/getprogname.c` behavior and does not introduce additional facilities
- Final review for:
  - no unnecessary dependencies,
  - no unsafe code unless strictly required,
  - no module expansion beyond the original file scope