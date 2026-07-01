# Implementation Plan

## Summary
Port the `fflush.c` logic for `clear_ungetc_buffer_preserving_position` and `clear_ungetc_buffer` into a small Rust module within the main execution cluster for `cat`. The Rust implementation should preserve the existing behavior and scope of these helper routines rather than redesigning I/O flow.

The technical approach is to translate the current C file into a focused Rust module that:
- keeps the two existing function boundaries,
- uses safe Rust where possible,
- isolates any low-level stream-position handling behind narrow helper logic,
- represents failures with standard Rust result types rather than implicit C error signaling.

Because these functions are tied to stream state manipulation, the migration should favor direct behavioral parity with the current file over abstraction. The implementation should avoid introducing new module layers or broader buffering facilities beyond what is needed to replace the existing C routines.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**:
  - Rust standard library
  - No third-party crates recommended based on the current module evidence
- **Testing**: `cargo test`
- **Performance Goals**:
  - Maintain effectively negligible overhead relative to the C helpers
  - Avoid unnecessary buffer copies or allocations during ungetc-buffer clearing
  - Preserve stream position with minimal extra seeks when using the position-preserving path
  - Keep implementation suitable for the hot path expectations of a command-line utility

## Module Mapping

| C File | C Function | Rust Target |
|---|---|---|
| `fflush.c` | `clear_ungetc_buffer_preserving_position` | `src/main_root_clear_ungetc_08.rs` -> `fn clear_ungetc_buffer_preserving_position(...) -> Result<..., ...>` |
| `fflush.c` | `clear_ungetc_buffer` | `src/main_root_clear_ungetc_08.rs` -> `fn clear_ungetc_buffer(...) -> Result<..., ...>` |

### Rust Module Placement
Use a single Rust source file corresponding to this migration unit:
- `src/main_root_clear_ungetc_08.rs`

If the project’s existing binary layout requires inclusion from `main.rs` or another root module, expose only the minimal internal functions needed for the current call sites. Do not split this file further unless the surrounding project structure already requires it.

## Data Model

This module analysis does not list standalone C data structures. The main migration concern is stateful C stream handling.

| C Concept | Rust Mapping |
|---|---|
| `FILE *` stream state | Borrowed stream parameter over the narrowest applicable standard trait(s), likely combinations of `Read`, `Write`, and `Seek` depending on actual call behavior |
| C integer status / error return | `Result<(), std::io::Error>` or similarly narrow `Result` type |
| Saved file position (e.g. `off_t`, `fpos_t`, or equivalent usage) | `u64` or `std::io::SeekFrom`-compatible saved position, chosen to match the actual translated logic |

### Memory Management
- No manual heap ownership model is expected from this file.
- Stream objects should remain externally owned; functions should operate on mutable borrows.
- Avoid unsafe code unless the existing project architecture forces direct libc stream interop and there is no standard-library path for preserving semantics.

### Error Handling
- Convert C-style success/failure paths into explicit `Result`.
- Preserve operational failure points such as inability to query or restore position.
- Do not add retry or recovery behavior beyond the original control flow.

## Implementation Phases

## Phase 1: Inspect and map the C stream operations
- Review `fflush.c` and identify the exact mechanics used by:
  - `clear_ungetc_buffer_preserving_position`
  - `clear_ungetc_buffer`
- Determine whether the C implementation depends on:
  - file-position save/restore,
  - flushing semantics,
  - read/write mode distinctions,
  - EOF/error flag interactions.
- Map each required C stream operation to an equivalent Rust standard-library operation where available.
- Confirm the minimal trait bounds needed for the Rust function signatures.

## Phase 2: Port the functions into a single Rust module
- Create `src/main_root_clear_ungetc_08.rs`.
- Implement `clear_ungetc_buffer_preserving_position` first, since it likely contains the stricter positioning logic.
- Implement `clear_ungetc_buffer` as the direct counterpart to the simpler C routine, reusing only the translated internal steps already present in the original file’s structure.
- Keep function naming close to the C source for migration traceability.
- Use explicit `Result` returns and preserve original branching behavior.
- Keep stream mutation localized through `&mut` borrows and avoid introducing global state or broader buffering abstractions.

## Phase 3: Integrate with existing main-cluster call sites
- Replace references to the C-backed implementation with the Rust module functions on branch `009-main_root_clear_ungetc_08-rust-port`.
- Adjust call sites only as needed to satisfy Rust ownership, mutability, and error propagation requirements.
- Ensure the migrated functions remain internal to the main-cluster area unless wider visibility is already required by the current codebase.

## Phase 4: Validate behavior with targeted tests
- Add focused unit tests for:
  - clearing buffered pushed-back input without losing the underlying position,
  - position-preserving behavior before and after the operation,
  - failure propagation when seek/position restoration is not possible,
  - no-op behavior when no pushed-back state needs clearing, if applicable from the C logic.
- Prefer deterministic in-memory or temporary-file tests using the standard library.
- Run `cargo test` and confirm parity with expected behavior from the original helper routines.