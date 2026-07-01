# Implementation Plan

## Summary

Port `progname.c` into a focused Rust module that provides the `set_program_name` behavior needed by the `pwd` main-program cluster. The Rust implementation should preserve the existing responsibility of extracting and storing the executable/program name from process startup inputs without adding new capabilities.

The technical approach is to map the C logic into a small Rust module using standard library path and string facilities. Since C implementations of program-name setup often rely on mutable global state, the Rust port should minimize unsafe patterns and prefer a controlled module-level storage approach using standard library synchronization primitives only if required by the surrounding call pattern. If the wider port only needs one-time initialization during startup, the implementation should reflect that narrow usage rather than generalizing beyond the original behavior.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Constant-time or near-constant-time initialization relative to argument length, excluding basename extraction.
  - No unnecessary allocations beyond the owned storage required for the program name.
  - Startup-only overhead should remain negligible for a command-line utility.

## Module Mapping

| C File | C Function | Rust Module/File | Rust Item |
|---|---|---|---|
| `progname.c` | `set_program_name` | `src/progname.rs` | `pub(crate) fn set_program_name(...)` |

If the crate currently keeps startup wiring in `src/main.rs`, the migrated function should be invoked there at the same point the C code performs initialization, without creating extra abstraction layers.

## Data Model

This module does not define dedicated C structs in the provided input.

| C Construct | Rust Mapping |
|---|---|
| process/program name held in static/global storage | module-local owned string storage, preferably `String` in a one-time initialization container |
| C string input (`char *` / `argv[0]`-style source) | `&str`, `String`, or `&OsStr`/`&Path` depending on the caller’s existing startup argument representation |

### Data Handling Notes

- If the original C function derives a basename from a path-like `argv[0]`, Rust should use `std::path::Path` for path component extraction.
- If non-UTF-8 executable names must be tolerated based on the surrounding port, prefer accepting `&OsStr` internally and converting only where needed. If the existing Rust entry path already normalizes arguments to UTF-8, keep the interface narrow and use `&str`.
- Ownership should be explicit: once initialized, the stored program name should be owned by the module and not borrow from `argv`.

## Implementation Phases

### Phase 1: Establish Rust module and function mapping

- Create `src/progname.rs`.
- Add the Rust equivalent of `set_program_name` with a signature matching how startup arguments are already represented in the ported `main` path.
- Port only the logic necessary to:
  - accept the incoming executable/program identifier,
  - reduce it to the intended program name form,
  - store it for later access if the original module does so.
- Keep the implementation scoped to the single migrated function and its direct storage needs.

### Phase 2: Port C string/path behavior safely

- Translate basename/path trimming behavior from the C implementation using `std::path::Path` methods where appropriate.
- Replace C mutable global storage with a narrow Rust module-level storage pattern:
  - prefer one-time initialization semantics if the function is only called during startup,
  - avoid unsafe global mutation unless unavoidable for exact compatibility.
- Ensure memory safety by storing an owned value instead of retaining references into argument buffers.
- Preserve original error-handling intent:
  - if the C code assumes valid startup input and does not return errors, keep the Rust function similarly simple,
  - handle empty or malformed input only to the extent necessary to avoid panics.

### Phase 3: Integrate with main-program startup flow

- Update `src/main.rs` or the corresponding startup module to call `set_program_name` at the same initialization stage as the C code.
- Verify that any later consumers of the stored program name use the Rust module instead of relying on C-style extern globals.
- Do not introduce additional helper modules or cross-cutting infrastructure beyond what is needed for this migration.

### Phase 4: Add focused tests and finalize migration

- Add unit tests in `src/progname.rs` or adjacent test modules covering:
  - plain executable name input,
  - path-qualified input where basename extraction is expected,
  - empty or minimal input if relevant to the original logic.
- Run `cargo test` and confirm no behavioral drift in startup naming logic.
- Remove or avoid any temporary compatibility code once the Rust module is wired into the main path.