# Implementation Plan

## Summary

Port `fflush.c` into a Rust module that preserves the existing behavior of seek-state handling around stream flushing. The Rust implementation should focus on migrating the current function set only:

- `disable_seek_optimization`
- `restore_seek_optimization`
- `update_fpos_cache`
- `rpl_fflush`

The technical approach should stay close to the C control flow and system interaction patterns. Because the source module operates on C `FILE*` stream state and flushing semantics that are not modeled by Rust's standard I/O types, the Rust port should use low-level libc-compatible calls and opaque stream pointers where needed, while keeping unsafe code narrowly scoped inside the module. The module should centralize state transitions related to temporary seek optimization disablement, flush execution, and file-position cache updates, with explicit error propagation based on OS/libc return values.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**:
  - Rust standard library
  - `libc` crate for `FILE`, `fflush`, `fileno`, `lseek`, and related C/POSIX interop needed to mirror the source module
- **Testing**: `cargo test`
- **Performance Goals**:
  - Match the C module's operational cost profile for flush paths
  - Avoid extra buffering layers or stream abstraction overhead
  - Keep additional allocations at zero in normal execution
  - Restrict state bookkeeping to lightweight stack/local values where possible

## Module Mapping

| C File | Rust File | Notes |
|---|---|---|
| `fflush.c` | `src/main_root_fflush.rs` | Direct port of the module's functions with minimal reshaping |
| `fflush.c` | `src/lib.rs` or existing module root export | Re-export only if required by current project layout |

### Function Mapping

| C Function | Rust Function | Notes |
|---|---|---|
| `disable_seek_optimization` | `disable_seek_optimization` | Port as internal helper unless existing call graph requires wider visibility |
| `restore_seek_optimization` | `restore_seek_optimization` | Port as paired internal helper restoring prior stream/descriptor behavior |
| `update_fpos_cache` | `update_fpos_cache` | Port as helper for synchronizing position-related cached state after flush |
| `rpl_fflush` | `rpl_fflush` | Primary entry point; preserve return conventions and error mapping |

## Data Model

No explicit standalone data structures were identified in the input module. The port should therefore avoid inventing new persistent model types unless required to represent temporary saved state from the C implementation.

### C-to-Rust Type Mapping

| C Type / Concept | Rust Mapping | Notes |
|---|---|---|
| `FILE *` | `*mut libc::FILE` | Opaque libc stream handle; all direct use contained in unsafe blocks |
| file descriptor (`int`) | `libc::c_int` / `i32` | Use exact libc-compatible integer type in syscall-facing code |
| file position / offset | `libc::off_t` | Preserve platform-correct width |
| status return (`int`) | `libc::c_int` or `Result<(), io::Error>` internally | Internal helpers may use `Result`; exported compatibility function should preserve C-style return behavior |
| errno-driven failure | `std::io::Error` internally, converted back to C return codes | Read immediately after failing libc calls |

### Temporary State Representation

If the C code stores intermediate seek-optimization state during `disable_seek_optimization` / `restore_seek_optimization`, represent it as a small private Rust struct local to this module, for example:

- prior descriptor flags
- prior file position validity marker
- any cached offset needed for restoration

This struct should remain private and only include fields directly required by the original logic. Do not generalize it into a broader stream wrapper.

## Implementation Phases

### Phase 1: Module Skeleton and API Port

- Create `src/main_root_fflush.rs` for the direct port of `fflush.c`.
- Define Rust signatures for:
  - `disable_seek_optimization`
  - `restore_seek_optimization`
  - `update_fpos_cache`
  - `rpl_fflush`
- Establish the minimal imports from `std` and `libc`.
- Decide visibility based on actual use:
  - keep helpers private by default
  - expose only `rpl_fflush` if that matches the current project interface
- Preserve C-compatible return semantics for the main replacement function.

### Phase 2: Unsafe Boundary and Helper Migration

- Port the helper functions first, keeping each unsafe interaction localized:
  - descriptor extraction
  - seek/flush libc calls
  - cache-related stream state reads/writes required by the original logic
- Convert raw libc failures into internal `io::Error` values immediately, then translate back to integer status at the public boundary.
- Preserve call ordering from the C implementation, especially where optimization disablement must bracket flush execution.
- Ensure no borrowed Rust I/O abstractions are layered over `FILE*`; operate directly on the underlying C handles.

### Phase 3: `rpl_fflush` Integration

- Port `rpl_fflush` using the migrated helpers.
- Keep the branch structure aligned with the original implementation:
  - identify stream conditions
  - disable seek optimization when required
  - call flush
  - update cached file-position state
  - restore temporary state on all relevant exit paths
- Audit all early returns so restoration logic is not skipped where the C code would restore state.
- Preserve exact success/failure behavior expected by callers, including handling of null or special stream cases if present in the source logic.

### Phase 4: Validation and Cleanup

- Add unit/integration tests under `cargo test` covering:
  - successful flush on writable stream
  - failure propagation from libc flush path
  - correct handling of descriptor/seek-related branches exercised by the helper functions
  - file-position cache updates for representative stream states where testable
- Compare Rust behavior against the C module's expected return values and error conditions.
- Minimize and document each unsafe block with the specific invariant it depends on.
- Remove any helper abstraction not justified by the source migration.