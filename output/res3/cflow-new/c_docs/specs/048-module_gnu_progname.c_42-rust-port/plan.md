# Implementation Plan

## Summary

Port `gnu/progname.c` into a focused Rust module that preserves the existing responsibility of setting and exposing the process program name derived from argv-style input. The Rust implementation should stay minimal and align with the current module boundary: migrate `set_program_name` and the related internal state only, without introducing broader process or CLI abstractions.

The technical approach is to replace the C module’s mutable global string handling with a small Rust module that stores the computed program name in process-global state using standard library synchronization primitives. Path parsing should use `std::path` where possible, while keeping behavior close to the C implementation’s basename-style extraction. The migration should explicitly handle ownership of incoming string data, avoid borrowed global state, and keep initialization/update semantics simple and testable.

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Constant-time global state replacement apart from input string parsing/allocation.
  - Single allocation for stored program-name content per update path where feasible.
  - No meaningful regression relative to the C module, since the operation is initialization-oriented and not performance-critical.

## Module Mapping

| C File | Rust File | Notes |
|---|---|---|
| `gnu/progname.c` | `src/module_gnu_progname.rs` | Direct port of module logic for program-name extraction and storage. |
| `gnu/progname.c` (`set_program_name`) | `src/module_gnu_progname.rs` (`set_program_name`) | Preserve function role and narrow scope; adapt signature to Rust string/path input conventions as required by the surrounding crate. |

## Data Model

This module does not define named C structs in the provided input, but it does rely on module-level mutable state.

| C Concept | Rust Mapping | Notes |
|---|---|---|
| Global mutable program name storage | `static` global state using `std::sync::OnceLock` with interior mutability, or `std::sync::Mutex<String>` behind a `static` initializer | Use owned `String` storage to avoid lifetime issues from borrowed argv memory. |
| C string input (`char *` / `const char *`) | `&str`, `String`, or `&std::ffi::OsStr` depending on crate call sites | Prefer the narrowest signature that matches existing Rust-side callers; if path fidelity matters, use `&OsStr` internally before converting for storage. |
| Basename-style derived pointer into input buffer | Newly owned Rust `String` containing the derived program name | Rust should not emulate pointer aliasing into the original input buffer. |

## Implementation Phases

### Phase 1: Create the Rust module skeleton and state mapping

- Add `src/module_gnu_progname.rs` following standard Rust project layout.
- Introduce the minimal module-global storage needed for the program name.
- Choose a standard-library-only state primitive:
  - Prefer `OnceLock<Mutex<String>>` or equivalent to support initialization and subsequent updates safely within process scope.
- Define the Rust `set_program_name` entry point with a signature consistent with the crate’s existing call path.
- Keep the public API limited to the migrated function and only the state accessors required by existing code integration.

### Phase 2: Port basename extraction and update semantics

- Recreate the C logic that derives the effective program name from the provided argument/path.
- Use `std::path::Path`/`file_name` when input semantics are path-like, while ensuring fallback handling for edge cases such as:
  - empty input
  - trailing separators
  - paths without a filename component
- Store the derived value as owned Rust data in the module-global state.
- Ensure memory behavior is explicit:
  - no retained borrows into caller-owned buffers
  - no unsafe code unless required by surrounding crate constraints
- Express invalid or lossy string cases through the narrowest practical behavior for the crate:
  - if UTF-8 is already guaranteed by callers, accept `&str`
  - otherwise normalize at the boundary and keep internal storage consistent

### Phase 3: Integrate with the crate and preserve module boundaries

- Wire the new Rust module into the branch’s crate module tree.
- Replace references to the C implementation with calls to the Rust `set_program_name`.
- Keep migration scope limited to `gnu/progname.c`; do not introduce unrelated utility modules.
- Verify that any existing program-name reads in the Rust port use the same stored state expected from this module.

### Phase 4: Add focused tests and finalize parity checks

- Add unit tests covering:
  - plain executable names
  - absolute and relative path inputs
  - empty or separator-ended inputs, according to chosen parity behavior
  - repeated updates to program name state
- Where global state affects test isolation, serialize access within tests and reset only if the chosen storage model permits it; otherwise structure tests around deterministic update order.
- Run `cargo test` and confirm that behavior matches the expected basename extraction and storage semantics of the original module without adding new capabilities.