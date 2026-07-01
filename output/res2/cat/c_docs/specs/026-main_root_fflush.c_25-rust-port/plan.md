# Implementation Plan

## Summary

Port `fflush.c` into a Rust module that preserves the existing responsibility: managing `fflush`-related behavior together with the seek/fpos bookkeeping helpers used by the C implementation. The Rust work should stay narrowly scoped to migrating the current file and its functions:

- `disable_seek_optimization`
- `restore_seek_optimization`
- `update_fpos_cache`
- `rpl_fflush`

The Rust implementation should prefer `std::io` abstractions where they can express the same behavior, and use small internal state representations to replace C-side mutable stream bookkeeping. Because the original C code is centered on stream state and flushing semantics, the Rust port should model these operations explicitly rather than broadening the module into a generic I/O layer.

The main technical approach is:

- create one Rust source module corresponding to `fflush.c`
- translate the helper functions into internal Rust functions with narrow visibility
- represent C stream-position/seek-optimization state with Rust structs/enums as needed
- return `std::io::Result` or equivalent error-aware results instead of C integer status codes
- keep ownership and mutability explicit through borrowing, avoiding raw memory manipulation unless unavoidable for parity with the source logic

## Technical Context

- **Language/Version**: Rust 1.77+
- **Primary Dependencies**: Rust standard library only (`std::io`, `std::fs` if needed, `std::os::unix` only if the surrounding project already depends on platform-specific file-descriptor behavior)
- **Testing**: `cargo test`
- **Performance Goals**:
  - preserve the original module’s low overhead around flush and seek-state tracking
  - avoid unnecessary allocations
  - keep flush-path logic constant-overhead aside from underlying I/O calls
  - preserve equivalent behavior for stream-position cache updates and optimization toggling without adding synchronization or abstraction layers not present in the C module

## Module Mapping

| C File | Rust Module | Notes |
|---|---|---|
| `fflush.c` | `src/fflush.rs` | Direct port target for all functions in this module |

| C Function | Rust Item | Visibility | Notes |
|---|---|---|---|
| `disable_seek_optimization` | `fn disable_seek_optimization(...)` | private or `pub(crate)` | Internal helper for stream-state mutation |
| `restore_seek_optimization` | `fn restore_seek_optimization(...)` | private or `pub(crate)` | Counterpart to temporary optimization disabling |
| `update_fpos_cache` | `fn update_fpos_cache(...)` | private or `pub(crate)` | Updates cached file-position state after flush-related operations |
| `rpl_fflush` | `pub(crate) fn rpl_fflush(...) -> io::Result<()>` | `pub(crate)` | Main exported replacement behavior for this migrated module |

If the crate already organizes coreutils-style replacements under another file path, place this implementation in the existing equivalent location instead of introducing a new architectural layer. The mapping should remain one C file to one Rust module.

## Data Model

No concrete C structs were listed in the analysis results, so the Rust data model should be introduced only to the extent required by the translated function signatures and state transitions.

| C Concept | Rust Representation | Notes |
|---|---|---|
| `FILE *`-associated mutable state | `&mut` reference to an internal stream wrapper or state holder | Exact type should follow existing project I/O abstractions if already present |
| seek optimization enabled/disabled flag | `bool` or small enum | Use enum if the C logic distinguishes more than two states |
| cached file position (`fpos`/offset-like state) | `Option<u64>` or project-specific offset type | `Option` covers unknown/invalid cache state cleanly |
| C integer status return | `std::io::Result<()>` or `Result<T, io::Error>` | Convert failure paths into Rust errors instead of sentinel integers |

Recommended minimal internal types, only if the port needs explicit state containers:

```rust
enum SeekOptimizationState {
    Enabled,
    Disabled,
}

struct FposCache {
    position: Option<u64>,
}
```

These should only be added if the translated logic cannot be expressed directly through existing crate types. Do not invent broader stream frameworks.

## Implementation Phases

### Phase 1: Module Skeleton and Signature Translation

- Create the Rust module for `fflush.c` in the standard crate layout.
- Identify the exact Rust-facing type that corresponds to the stream handled by `rpl_fflush`.
- Translate C function signatures into Rust function signatures, initially preserving call relationships and control flow structure.
- Define the minimal internal state representation required for:
  - temporary seek optimization disabling/restoration
  - cached file-position tracking
- Replace C return conventions with `io::Result`-style results.

**Exit criteria**:
- All four functions exist in Rust with compile-valid signatures.
- The module builds with placeholder or partial internals wired correctly.

### Phase 2: Core Logic Port

- Implement `disable_seek_optimization` and `restore_seek_optimization` as direct state transitions mirroring the C logic.
- Implement `update_fpos_cache` with explicit handling for valid versus invalid/unknown position state.
- Port `rpl_fflush` control flow in the same order as the C source:
  - pre-flush state handling
  - flush invocation
  - post-flush cache/state updates
  - error propagation
- Remove any C-style implicit mutation assumptions by making borrows and mutable state updates explicit.
- Keep platform-specific branching only if it exists in the source behavior being migrated.

**Exit criteria**:
- Rust logic covers all original function paths.
- Error paths and state restoration paths are represented explicitly and compile cleanly.

### Phase 3: Error Handling and Semantic Alignment

- Verify that all source failure cases map to Rust errors without losing observable behavior needed by callers.
- Ensure temporary state changes are always restored on both success and failure paths.
- Confirm cached position handling matches source semantics for:
  - valid known position
  - invalidated cache after flush/error
  - unchanged cache when appropriate
- Review ownership/borrowing to ensure there are no hidden aliasing assumptions from the C code.

**Exit criteria**:
- Function behavior is stable under success and failure cases.
- No unsafe code is introduced unless strictly required by surrounding crate interfaces.

### Phase 4: Focused Tests and Integration Validation

- Add unit tests for the helper functions where the translated state model makes them testable in isolation.
- Add `rpl_fflush` tests covering:
  - successful flush path
  - failure propagation
  - seek optimization state restoration
  - file-position cache update/invalidation behavior
- Run `cargo test` and adjust the implementation to match expected module semantics in the existing crate.

**Exit criteria**:
- Tests cover the migrated logic paths without adding new functionality.
- The module integrates into the branch `026-main_root_fflush.c_25-rust-port` and passes `cargo test`.