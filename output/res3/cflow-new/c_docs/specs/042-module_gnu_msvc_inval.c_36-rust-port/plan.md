# Implementation Plan

## Summary

Port `gnu/msvc-inval.c` into a single Rust module that preserves the existing responsibility of ensuring the MSVC invalid-parameter handler is installed exactly as needed by the original code path. The Rust implementation should stay narrowly scoped to the current file and function set, with no expansion beyond the existing handler-installation behavior.

The technical approach is to translate the C module into a Rust module that:
- encapsulates one-time handler initialization,
- models any C file-scope state with private Rust statics,
- uses safe Rust where possible and isolates any Windows-specific or FFI-adjacent behavior behind minimal `cfg` gates and small `unsafe` blocks only if required,
- preserves the original control-flow intent of `gl_msvc_inval_ensure_handler`.

The migration should prioritize behavioral equivalence, especially around process-global state, one-time initialization, and no-op behavior on non-MSVC or non-Windows targets.

## Technical Context

- **Language/Version**: Rust 1.76 or newer
- **Primary Dependencies**:
  - Rust standard library
  - No third-party crates by default
- **Testing**: `cargo test`
- **Performance Goals**:
  - Constant-time fast path after initialization
  - No unnecessary heap allocation
  - Minimal overhead beyond the required one-time handler check/installation
  - Preserve process-global semantics without introducing repeated setup cost

## Module Mapping

- **C source**: `gnu/msvc-inval.c`
- **Rust target**: `src/module_gnu_msvc_inval.rs`

### Function Mapping

- `gl_msvc_inval_ensure_handler`
  - Port to a Rust function with crate-private or public visibility according to current call-site needs in the Rust project
  - Preferred Rust name: `gl_msvc_inval_ensure_handler` to preserve traceability during migration

### File Organization

- Keep the port in a single Rust source file corresponding directly to the C module.
- Do not split handler logic into additional helper modules unless a tiny private helper function is necessary to isolate `unsafe` target-specific code.

## Data Model

The analysis shows only anonymous C data structures, which strongly suggests file-local or API-level opaque C declarations rather than meaningful exported domain structs. The Rust port should therefore avoid inventing new public data models and instead map only the technical state required for the function to work.

### Data-structure Mapping

- **anonymous C structs/data declarations**
  - Map to:
    - private module-level state via `static` or `static mut` only if unavoidable,
    - otherwise `std::sync::Once` / `std::sync::OnceLock` / atomic flags for one-time installation state,
    - private type aliases or zero-sized placeholders only if needed to mirror platform API signatures.

### Recommended Rust Representations

- One-time initialization state:
  - `static INIT: std::sync::Once`
  - or `static INSTALLED: std::sync::atomic::AtomicBool` if the C logic only needs a boolean guard and not infallible one-time execution semantics
- Platform handler references:
  - use private function items or platform-specific type aliases behind `#[cfg(windows)]`
- C nullability / pointer state:
  - use `Option<...>` where representable in Rust
  - use raw pointers only at platform API boundaries

### Memory Management Notes

- No heap ownership model is expected from this module.
- Any global handler state should remain process-global and static, matching the C behavior.
- `unsafe` should be confined to:
  - calls into target runtime APIs if needed,
  - manipulation of raw function pointers or platform-defined handler signatures.

### Error Handling Notes

- If the original C function is effectively best-effort and returns no status, the Rust port should keep a non-failing signature and suppress installation failures into a no-op outcome where appropriate.
- Do not introduce new error enums unless the surrounding Rust code already requires result propagation.
- If platform support is absent, provide a no-op implementation under `cfg` rather than runtime failure.

## Implementation Phases

## Phase 1: Inspect and Scaffold the Direct Port

- Create `src/module_gnu_msvc_inval.rs`.
- Add the Rust function shell for `gl_msvc_inval_ensure_handler`.
- Inspect the original C file for:
  - handler function signature,
  - file-scope static state,
  - conditional compilation paths,
  - whether the function is idempotent by design.
- Reproduce the same target gating in Rust using `#[cfg(...)]` attributes.
- Keep naming close to the C source for migration traceability.

### Deliverables

- New Rust module file
- Function stub with target-specific compilation blocks
- Initial comments documenting any remaining `unsafe` requirements

## Phase 2: Port Handler State and Initialization Logic

- Translate file-scope C state into private Rust module state.
- Implement one-time initialization using `std::sync::Once` as the default choice for process-global installation semantics.
- Port the invalid-parameter handler installation code for the applicable Windows/MSVC path.
- Add a no-op branch for unsupported targets that preserves call compatibility without expanding behavior.
- Keep all platform-specific interactions local to this module.

### Deliverables

- Complete implementation of `gl_msvc_inval_ensure_handler`
- Private static initialization state
- Minimal `unsafe` blocks with narrow scope and comments explaining invariants

## Phase 3: Integrate and Validate Behavioral Equivalence

- Wire the module into the crate using standard Rust module declarations only where the existing migration requires it.
- Update call sites to use the Rust implementation in place of the C module behavior.
- Add unit tests for:
  - repeated calls being safe and idempotent,
  - non-target builds compiling successfully,
  - target-gated code paths remaining isolated.
- Where direct runtime validation of the handler is impractical in tests, verify the observable one-time state transitions and compile-time path selection.

### Deliverables

- Module integrated into the crate
- `cargo test` coverage for idempotent initialization and cfg behavior
- Successful compilation on non-Windows targets without extra dependencies

## Phase 4: Cleanup and Migration Finalization

- Remove any temporary compatibility scaffolding introduced during the port.
- Review visibility and make all state private unless a caller requires exposure.
- Confirm no unnecessary abstractions, crates, or auxiliary modules were added.
- Ensure the final Rust file remains a direct replacement for `gnu/msvc-inval.c` in scope and responsibility.

### Deliverables

- Finalized Rust module with minimal API surface
- Cleaned comments and narrowed visibility
- Migration-ready state on branch `042-module_gnu_msvc_inval.c_36-rust-port`