# Implementation Plan

## Summary

Port the C module `fflush.c` into a focused Rust module that preserves the existing file-stream flushing behavior and seek-position bookkeeping used by the `cat` project. The Rust implementation should mirror the current function boundaries closely:

- `disable_seek_optimization`
- `restore_seek_optimization`
- `update_fpos_cache`
- `rpl_fflush`

The implementation approach should stay minimal and migration-oriented:

- create one Rust module corresponding to `fflush.c`
- represent any seek-optimization state explicitly in Rust
- implement flushing and file-position cache updates with Rust standard library I/O traits where possible
- isolate any platform-specific or low-level behavior behind small internal helpers rather than widening the module surface

Because the C code operates around stream state and flushing semantics, the Rust port should prioritize correct ownership of stream-related state, explicit error propagation via `Result`, and a narrow translation of the existing control flow instead of redesigning the I/O layer.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**:
  - Rust standard library
  - No third-party crates recommended based on the provided module scope
- **Testing**:
  - `cargo test`
- **Performance Goals**:
  - preserve near-equivalent flushing cost to the C implementation
  - avoid unnecessary allocations
  - keep seek-position cache updates constant-time
  - ensure no extra buffering layers are introduced beyond what the existing Rust project already uses

## Module Mapping

### C to Rust File Mapping

- `fflush.c` → `src/main_root_fflush.rs`
  or, if this branch already follows a per-cluster layout, `src/main_cluster/main_root_fflush.rs`

The port should follow the existing Rust project’s file organization if one already exists; otherwise, use a single module file matching the migrated C unit.

### Function Mapping

- `disable_seek_optimization` → `fn disable_seek_optimization(...)`
- `restore_seek_optimization` → `fn restore_seek_optimization(...)`
- `update_fpos_cache` → `fn update_fpos_cache(...) -> Result<..., std::io::Error>`
- `rpl_fflush` → `fn rpl_fflush(...) -> Result<(), std::io::Error>`

### Visibility Guidance

Keep visibility as restricted as possible:

- internal helpers: `fn`
- exported replacement only if needed by the surrounding port: `pub(crate) fn rpl_fflush(...)`

Do not introduce extra public APIs beyond what is required to replace the C module’s call sites.

## Data Model

No explicit C structs were listed for this module, so the Rust data model should be derived from the stream state that the functions currently manipulate.

### Data-Structure Mapping

- C implicit stream/FILE state → Rust wrapper over existing stream handle/state used by the port
- C seek-optimization flag/cache values → Rust struct fields with explicit types
- C file position cache (`fpos`-like state) → `u64` or `Option<u64>` depending on whether “unknown position” is a valid state
- C integer status returns → `Result<(), std::io::Error>` or `Result<PositionType, std::io::Error>`

### Recommended Rust Representation

If the translated code needs module-local state to mimic the C logic, use a small internal struct such as:

```rust
struct SeekOptimizationState {
    enabled: bool,
    cached_pos: Option<u64>,
}
```

This is only a migration vehicle for the C logic and should be used only if the original code tracks these values separately from the stream object. If the surrounding Rust port already has an equivalent stream state type, extend that existing type instead of creating a parallel abstraction.

### Memory Management

- rely on Rust ownership and borrowing for stream-state access
- avoid heap allocation unless the existing project architecture already requires boxed trait objects
- prefer mutable borrows for in-place cache/state updates
- represent absent or invalid cached positions with `Option` rather than sentinel integers where possible

### Error Handling

- convert C-style success/failure returns into `std::io::Result`
- propagate flush and seek-related errors directly with `?`
- only preserve special-case return mapping where required for compatibility with existing translated call sites

## Implementation Phases

## Phase 1: Establish module skeleton and state mapping

- create the Rust module file for `fflush.c`
- identify the exact Rust-side stream type already used by the `cat` port
- map each C function signature into a Rust signature with minimal adaptation
- define any internal state representation needed for:
  - seek optimization enable/disable state
  - cached file position
- wire the module into the crate without adding unrelated abstractions

### Deliverables

- module file added
- function stubs with final signatures
- internal state representation decided and documented in code comments where needed

## Phase 2: Port seek-optimization control and file-position cache logic

- implement `disable_seek_optimization`
- implement `restore_seek_optimization`
- implement `update_fpos_cache`
- translate C control flow directly, preserving ordering of state changes around flush/seek-sensitive operations
- replace sentinel-based state tracking with `Option` or small enums only where this does not alter behavior

### Technical Notes

- keep cached-position updates explicit and local
- ensure mutable access patterns do not conflict with Rust borrow rules; split operations into shorter scopes if necessary
- if the original C code depends on pre- and post-operation state restoration, model that with a temporary saved value rather than broader shared mutability

### Deliverables

- seek optimization state transitions implemented
- file-position cache update logic implemented
- unit tests for normal and error-path cache updates where practical

## Phase 3: Port `rpl_fflush` and integrate error propagation

- implement `rpl_fflush` using the Rust standard library flushing APIs on the existing stream abstraction
- preserve the C ordering between:
  - disabling optimization
  - flushing
  - updating cached position if required
  - restoring optimization state
- ensure all failure paths leave state in a defined condition equivalent to the C behavior
- update call sites in the current branch to use the Rust replacement

### Deliverables

- complete `rpl_fflush` implementation
- integration with existing module call sites
- tests covering successful flush and flush failure behavior

## Phase 4: Validation and cleanup

- run `cargo test`
- add focused regression tests for:
  - repeated flush calls
  - flush after state changes affecting cached position
  - restoration of seek optimization after both success and failure
- remove any temporary translation scaffolding that is no longer needed
- confirm the final module surface remains narrow and aligned with the original C unit

### Deliverables

- passing test suite
- final module cleanup complete
- migration-ready Rust replacement for `fflush.c`