# Implementation Plan

## Summary

Port `progname.c` into an idiomatic Rust module that provides the `set_program_name` behavior used by the main execution path. The Rust implementation should preserve the existing role of normalizing and storing the invoked program name without adding new capability or changing call patterns outside what is required by Rust ownership and safety rules.

Technical approach:

- Migrate the single C source file into one focused Rust module.
- Represent the stored program name with standard library string/path types, choosing the narrowest internal form that matches current usage in the Rust port.
- Avoid unsafe code unless integration constraints make it strictly necessary.
- Keep the implementation centered on startup-time initialization semantics and straightforward retrieval by existing main-cluster code.
- Translate C global-state handling into Rust module-level state with explicit initialization behavior and clear ownership.

## Technical Context

- **Language/Version:** Rust 1.75+
- **Primary Dependencies:** Rust standard library only
- **Testing:** `cargo test`
- **Performance Goals:**
  - Constant-time access after initialization.
  - Negligible startup overhead relative to argument parsing.
  - No unnecessary string copying beyond the single owned representation needed to replace the C global state.

## Module Mapping

| C File | C Function | Rust Target | Notes |
|---|---|---|---|
| `progname.c` | `set_program_name` | `src/main_root_progname.rs::set_program_name` | Direct migration of initialization logic. |
| `progname.c` | module-level program-name storage | `src/main_root_progname.rs` private static/module state | Replaces C global variable pattern with Rust-managed state. |

## Data Model

This module has no declared C structs to port. The main data mapping is global state.

| C Representation | Rust Representation | Notes |
|---|---|---|
| process/program name stored in global static state | private module-level `String` or `PathBuf`-derived owned name | Final choice should follow how the value is consumed elsewhere in the Rust port; prefer `String` if only display/name comparisons are needed. |
| input `char *argv0`-style program path/name | `&str`, `String`, or `&OsStr`/`Path` at the module boundary | Prefer a borrowed Rust string/path input and convert once into owned module state. |

## Implementation Phases

### Phase 1: Create the Rust module skeleton

- Add a Rust source file for this migration, following the project’s standard `src/` layout.
- Define the Rust equivalent of `set_program_name`.
- Introduce private module state to hold the normalized program name.
- Choose standard-library-only state management appropriate for one-time initialization in the main path.

Deliverable:

- Compiling Rust module stub with function signature and private storage in place.

### Phase 2: Port the `set_program_name` logic

- Translate the C filename/program-name extraction behavior into Rust using `std::path::Path` and string handling as needed.
- Preserve the original initialization intent:
  - accept the invoked program path/name,
  - derive the stored program name,
  - store an owned value for later use.
- Handle invalid or non-UTF-8 path cases conservatively using standard library lossy conversion only if required by surrounding Rust interfaces.
- Keep failure behavior simple and aligned with existing startup assumptions; do not introduce recovery mechanisms not present in the source design.

Deliverable:

- Functional Rust implementation matching the C module’s behavior for program-name setup.

### Phase 3: Integrate with the main-cluster call site

- Replace the existing C-module dependency path with the new Rust module import/use path.
- Update the startup flow to call the Rust `set_program_name` at the same point the C code would have initialized program-name state.
- Ensure ownership and borrowing at the call site do not cause extra persistent allocations beyond the stored value itself.

Deliverable:

- End-to-end build path using the Rust version of this module in the main execution setup.

### Phase 4: Validate behavior with focused tests

- Add unit tests for:
  - plain executable names,
  - path-qualified executable names,
  - edge cases such as empty or minimal inputs, if relevant to current call patterns.
- Verify stored-name normalization behavior matches the intended C semantics.
- Run `cargo test` and confirm no regressions in startup-related integration.

Deliverable:

- Passing tests covering the migrated behavior and initialization expectations.

## Memory Management and Error Handling Notes

- Replace C global mutable storage with Rust-owned module state to eliminate manual lifetime management.
- Perform at most one owned allocation for the stored program name.
- Prefer explicit one-time initialization semantics over mutable shared state patterns.
- Avoid panics for ordinary path-shape inputs; only use infallible or clearly bounded standard library conversions where possible.
- If the surrounding Rust port requires string output, normalize path-derived values into an owned `String` at the module boundary and keep subsequent access allocation-free.