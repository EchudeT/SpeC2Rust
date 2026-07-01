# Implementation Plan

## Summary

This module ports the `fflush.c` logic into Rust for the `cat` project branch `026-main_root_fflush.c_25-rust-port`. The scope is limited to migrating the existing behaviors represented by:

- `disable_seek_optimization`
- `restore_seek_optimization`
- `update_fpos_cache`
- `rpl_fflush`

The Rust implementation should preserve the original control flow around stream flushing and file-position bookkeeping, while adapting C stream/state manipulation to Rust’s ownership and error model. Since the source module is centered on `FILE *`-style state, the Rust port should use narrowly scoped internal stream state representations and standard-library I/O traits where possible, with Unix-specific descriptor access only where required to mirror the original low-level behavior.

The technical approach should favor:

- direct file/function migration rather than redesign,
- explicit state transitions for seek-optimization enable/disable behavior,
- `Result`-based error propagation instead of implicit C error flags,
- minimal unsafe code, used only if unavoidable for descriptor-level interaction.

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**:
  - Rust standard library
  - No third-party crates recommended from the provided evidence
- **Testing**: `cargo test`
- **Performance Goals**:
  - Maintain behavior comparable to the C implementation for flush operations
  - Avoid unnecessary allocations during flush/state updates
  - Keep file-position cache updates constant-time
  - Preserve low-overhead handling for streams where seek optimization is temporarily disabled/restored

## Module Mapping

### C to Rust File Mapping

- `fflush.c` → `src/main_root_fflush.rs`

If the existing Rust project already centralizes main-cluster code in another file layout, place this module there without introducing extra abstraction layers. The migration should remain a single Rust module corresponding to the original C file.

### Function Mapping

- `disable_seek_optimization` → `disable_seek_optimization`
- `restore_seek_optimization` → `restore_seek_optimization`
- `update_fpos_cache` → `update_fpos_cache`
- `rpl_fflush` → `rpl_fflush`

Function names may remain close to the C originals to simplify review and traceability during the port.

## Data Model

No explicit C structs were listed in the analysis input. The plan should therefore avoid inventing broad new data models and instead introduce only the minimal Rust-side state needed to replace implicit C stream metadata access.

### Data Mapping Strategy

| C concept | Rust representation |
|---|---|
| `FILE *` stream handle | Borrowed generic stream state wrapper over standard I/O types, or a narrow internal struct representing flush/seek-related state |
| cached file position | `Option<u64>` or equivalent integer position cache |
| seek optimization enabled/disabled flag | `bool` or small enum |
| C integer error return (`0` / `EOF` or `-1`) | `std::io::Result<()>` internally; convert to project-required return type at boundary if needed |

### Minimal Internal Types

If the implementation needs to model C stream-local state explicitly, use a small internal struct such as:

- `SeekOptimizationState`
  - enabled/disabled status
  - cached file position, if known

This should remain private to the module unless another already-existing Rust module requires direct access.

## Implementation Phases

## Phase 1: Establish module skeleton and state mapping

- Create the Rust module file corresponding to `fflush.c`.
- Add the four migrated function stubs with signatures aligned to the surrounding Rust project conventions.
- Identify the exact stream abstraction already used in the Rust port of `cat`:
  - if standard library `File`/`Stdout`-style types are used, adapt this module to them;
  - if there is already a project-local stream wrapper, integrate directly with it rather than introducing a parallel abstraction.
- Define the minimum private state needed for:
  - seek optimization enable/disable tracking,
  - file-position cache maintenance,
  - flush outcome propagation.
- Decide return-type normalization:
  - internal logic in `io::Result`
  - boundary conversion only if the rest of the project expects C-like status codes.

## Phase 2: Port seek-optimization and file-position cache logic

- Implement `disable_seek_optimization` as a direct state transition with no extra policy added.
- Implement `restore_seek_optimization` to reverse only the temporary state change introduced by the disable path.
- Implement `update_fpos_cache` using the smallest faithful Rust equivalent of the C position-cache update:
  - update cached position only when the stream state makes it valid to do so;
  - invalidate cache when flush/seek behavior makes the position uncertain.
- Keep ownership simple:
  - pass mutable references to stream/state objects,
  - avoid heap allocation unless the existing Rust project structure already requires it.
- Ensure error cases do not leave partially updated cache state without explicit invalidation.

## Phase 3: Port `rpl_fflush` and connect module behavior

- Implement `rpl_fflush` around Rust flushing primitives and any required descriptor/position handling.
- Preserve the original ordering of operations:
  - temporary seek-optimization changes,
  - flush execution,
  - file-position cache update/restoration,
  - error propagation.
- If low-level descriptor interaction is required for parity with the C logic, confine platform-specific code to the smallest possible section and keep it internal to this module.
- Avoid broad API expansion: only expose what the original C module provided or what the current Rust project integration strictly requires.

## Phase 4: Validate behavior and finalize migration

- Add focused unit tests for:
  - disabling and restoring seek optimization state,
  - valid cache update after successful flush,
  - cache invalidation on flush failure or uncertain position state,
  - `rpl_fflush` success/error paths.
- Run `cargo test` and fix mismatches against expected module behavior.
- Perform a final review for:
  - unnecessary unsafe code,
  - accidental API growth,
  - ownership/borrowing issues around mutable stream access,
  - consistency with neighboring migrated main-cluster modules.