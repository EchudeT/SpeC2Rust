# Implementation Plan

## Summary

Port `fcntl.c` into a Rust module that preserves the existing module boundary and migrates the `dupfd` logic with behavior aligned to the current C implementation. The Rust work should stay narrowly scoped to translating the file-descriptor duplication path, using Rust’s standard library where possible and direct OS interaction only where standard APIs do not expose equivalent behavior.

The implementation should favor:
- a single Rust module corresponding to `fcntl.c`
- explicit integer/file-descriptor handling compatible with Unix targets
- idiomatic `Result`-based error propagation mapped from C error returns
- no expansion of functionality beyond the existing `dupfd` behavior

Because this module operates on raw file descriptors, the Rust version should carefully separate owned and borrowed descriptor handling to avoid accidental double-close or descriptor leaks. Where ownership transfer is required, use standard Unix descriptor types from the standard library.

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**:
  - Rust standard library
  - No third-party crates by default
  - If direct `fcntl`/`dup`-family access is required and cannot be expressed through stable standard APIs alone, use `libc` minimally for Unix system-call bindings
- **Testing**: `cargo test`
- **Performance Goals**:
  - Match the C implementation’s syscall-level cost profile
  - Avoid extra heap allocation
  - Preserve constant-time descriptor handling aside from the underlying OS call
  - Do not introduce wrapper layers that add measurable overhead around `dupfd`

## Module Mapping

| C File | Rust File | Notes |
|---|---|---|
| `fcntl.c` | `src/fcntl.rs` | Direct migration target for `dupfd`; keep scope limited to this file’s existing responsibility |

If the crate already uses a different root layout, place the translated file in the closest equivalent existing module location rather than introducing new architectural layers.

## Data Model

This module has no named public C struct to migrate; the only identified data structure is anonymous/internal state.

| C Construct | Rust Mapping | Notes |
|---|---|---|
| anonymous/internal local data | local variables / private helper types if strictly necessary | Prefer direct translation into local Rust bindings rather than introducing new structs |
| file descriptor (`int`) | `std::os::fd::RawFd` | Preserve raw Unix descriptor semantics |
| C error return (`-1` + `errno`) | `std::io::Result<_>` or internal `Result<_, std::io::Error>` | Convert OS failure into idiomatic Rust error propagation |

Descriptor ownership rules should be explicit:
- use `RawFd` for borrowed/non-owning descriptor inputs
- use `OwnedFd` only when the duplicated descriptor is owned by Rust after success
- avoid constructing owning descriptor types from borrowed descriptors unless ownership transfer is certain

## Implementation Phases

### Phase 1: Module Skeleton and API Mapping

- Create the Rust file corresponding to `fcntl.c`.
- Identify the exact `dupfd` signature and usage expectations in the surrounding codebase.
- Define the Rust function signature to preserve call semantics as closely as possible while using idiomatic error handling internally.
- Decide whether the external-facing API should:
  - return `RawFd` on success, or
  - return `OwnedFd` internally and convert only at call boundaries if required by existing project conventions.
- Add minimal module-level documentation describing the Unix-only descriptor behavior.

**Exit criteria**:
- Rust module exists in the target branch.
- `dupfd` has a defined Rust signature and compiles as a stub.
- No extra helper modules or abstractions have been introduced.

### Phase 2: Core `dupfd` Port

- Translate the body of `dupfd` from C into Rust with a direct control-flow mapping.
- Use stable standard-library descriptor types where they fit the C semantics.
- If the C logic depends on `fcntl` duplication commands or minimum-target-fd behavior not exposed by the standard library, call the underlying Unix API with minimal `libc` usage.
- Preserve all return-path distinctions from the C implementation:
  - successful duplicated descriptor
  - syscall failure propagation
  - invalid input handling consistent with the source behavior
- Ensure unsafe code, if needed for syscall boundaries, is tightly scoped and documented with the required invariants.

**Memory and resource handling requirements**:
- Never assume ownership of an input raw descriptor.
- Wrap only newly created duplicated descriptors as owned values when safe.
- Prevent leaks on intermediate failure paths.
- Avoid closing descriptors implicitly unless the original C code does so.

**Exit criteria**:
- `dupfd` is functionally implemented.
- Error mapping is complete and follows OS error behavior.
- Unsafe blocks, if any, are minimal and justified.

### Phase 3: Validation and Behavior Lock-In

- Add unit tests covering the migrated `dupfd` behavior with temporary files or pipes.
- Verify expected outcomes for:
  - successful duplication
  - duplication with target/minimum descriptor constraints if applicable
  - failure on invalid descriptors
- Confirm duplicated descriptors are independently valid from the original descriptor.
- Validate no accidental ownership bugs occur, especially around descriptor closure after test completion.

Test design should remain narrow and directly tied to the migrated function; do not add broad infrastructure.

**Exit criteria**:
- `cargo test` passes.
- Tests demonstrate parity for the implemented `dupfd` behavior.
- No additional unsupported capabilities were added during validation.

### Phase 4: Integration Cleanup

- Wire the Rust `fcntl` module into the existing crate/module tree.
- Replace or connect any call sites expected to use the migrated `dupfd`.
- Remove or retire only the migrated C-side dependency path if the project’s migration process requires it.
- Perform a final review for:
  - signature stability
  - Unix import correctness
  - error-path consistency
  - absence of unnecessary abstractions

**Exit criteria**:
- The Rust module is integrated into the project structure.
- The migrated path builds cleanly on the target Unix environment.
- Scope remains limited to `fcntl.c` and `dupfd`.