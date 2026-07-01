# Implementation Plan

## Summary

Port `progname.c` into a small Rust module that preserves the current responsibility of recording the executable's program name for later use by the `cat` binary. The Rust implementation should stay minimal and align with the existing main-program cluster rather than introducing broader infrastructure.

Technical approach:

- Migrate `set_program_name` into a Rust module dedicated to program-name initialization.
- Derive the program name from process startup arguments using the Rust standard library.
- Store the resulting value in a process-global location only if the surrounding Rust port still requires shared access beyond `main`; otherwise keep the API narrow and initialization-oriented.
- Avoid adding new capabilities such as path normalization policies beyond what is needed to match the original module's role.
- Handle non-UTF-8 executable names conservatively using owned path/string forms from the standard library, converting only at the boundary required by the rest of the port.

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Constant-time initialization apart from extracting the executable name from the invocation path.
  - No repeated allocations after initial program-name capture if a global storage path is used.
  - Negligible startup overhead relative to the overall `cat` process lifecycle.

## Module Mapping

| C File | C Function | Rust Module/File | Rust Item | Notes |
|---|---|---|---|---|
| `progname.c` | `set_program_name` | `src/main_root_progname.rs` or `src/progname.rs` | `pub(crate) fn set_program_name(...)` | Keep mapping narrow and centered on startup initialization. |

Recommended placement:

- If the current Rust port already groups translated main-cluster files under a dedicated namespace, place this at `src/main_root_progname.rs`.
- If the crate is still flat, use `src/progname.rs`.

Recommended public surface:

- `pub(crate) fn set_program_name(argv0: &std::ffi::OsStr)` if the caller already has the raw executable argument.
- If startup code structure makes that awkward, `pub(crate) fn set_program_name_from_env()` may be used internally, but only as a direct replacement path for this module's current duty.

## Data Model

This module does not define dedicated C structs in the provided input. The key migration concern is the C global program-name state.

| C Concept | Rust Mapping | Notes |
|---|---|---|
| Global stored program name (`char *`-style process-global state) | `static` global using `std::sync::OnceLock<...>` | Use one-time initialization to model startup assignment safely without manual memory management. |
| Program name text | `std::ffi::OsString` preferred for storage | Preserves non-UTF-8 platform values better than `String`. |
| Borrowed C string input (`char const *argv0`) | `&std::ffi::OsStr` or `&std::path::Path` | Prefer non-lossy argument handling at the API boundary. |
| Basename extraction from invocation path | `std::path::Path::file_name()` | Matches the technical need without manual pointer arithmetic. |

Recommended storage choice:

```text
static PROGRAM_NAME: OnceLock<OsString>
```

Access pattern, if needed by adjacent migrated code:

- Store the basename as owned `OsString`.
- Expose a crate-private getter returning `Option<&OsStr>` only if another already-planned ported module needs it.
- Do not introduce mutable global state beyond single initialization.

Memory-management decisions:

- Replace C lifetime/manual ownership behavior with owned Rust storage in `OnceLock`.
- Avoid leaking boxed strings or using unsafe static mut state.
- Keep conversions explicit when a UTF-8 `&str` is required by downstream formatting APIs; use lossy conversion only at display sites if necessary.

Error-handling decisions:

- `set_program_name` should remain effectively infallible for normal startup use.
- If basename extraction yields no terminal component, fall back to the original input value rather than failing.
- If the initialization function is called more than once, either:
  - ignore subsequent calls after first initialization, or
  - return a simple crate-private status/result if the surrounding port already prefers explicit initialization outcomes.
  Prefer the first option unless existing Rust main-cluster conventions require otherwise.

## Implementation Phases

### Phase 1: Establish module file and API shape

- Create the Rust module corresponding to `progname.c`.
- Add the crate-private function signature for `set_program_name`.
- Choose the exact input type based on the current Rust `main` translation:
  - `&OsStr` if passing `argv[0]` directly,
  - or a narrow internal no-argument initializer if startup code is already env-based.
- Add a single-assignment global storage using `OnceLock<OsString>` if shared retrieval is required elsewhere in the current port branch.

Deliverable:

- Compiling module skeleton with placeholder logic replaced by standard-library-based initialization.

### Phase 2: Port basename extraction and storage behavior

- Implement extraction of the executable leaf name from the startup path using `Path`/`file_name`.
- Preserve fallback behavior when the path has no filename component.
- Store the owned result in the global one-time cell.
- Keep the implementation fully safe Rust; do not use raw pointers or unsafe globals.

Key checks:

- Handles plain executable names and path-qualified invocation names.
- Does not assume UTF-8.
- Does not panic during ordinary startup.

Deliverable:

- Functional `set_program_name` equivalent integrated with the current main path.

### Phase 3: Wire into the main-cluster call site

- Update the translated main startup flow to call the new Rust `set_program_name` at the same logical point as the C original.
- Remove any temporary duplicate startup-name handling introduced during earlier migration work.
- Keep module boundaries tight: this phase should only connect the existing main path to the new module.

Deliverable:

- End-to-end initialization path for program name in the Rust `cat` binary.

### Phase 4: Add focused tests and finalize behavior

- Add unit tests for:
  - simple program name input,
  - path input with basename extraction,
  - non-UTF-8-tolerant handling as far as platform-safe tests allow,
  - repeated initialization behavior if observable within the module API.
- Run `cargo test`.
- Confirm no unnecessary allocations or conversions remain beyond initial capture.

Deliverable:

- Tested, minimal Rust replacement for `progname.c`.