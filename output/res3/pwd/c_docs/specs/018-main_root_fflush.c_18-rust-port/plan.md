# Implementation Plan

## Summary
Port `fflush.c` into a focused Rust module that preserves the existing responsibility of stream flush handling and related seek-position bookkeeping. The Rust implementation should keep the scope narrow: migrate the behavior represented by `disable_seek_optimization`, `restore_seek_optimization`, `update_fpos_cache`, and `rpl_fflush` into a single module aligned with the current main-cluster layout.

The technical approach should prefer the Rust standard library and isolate any unavoidable low-level stream interaction behind small internal helpers. Because the source functions are centered on file-stream state and flush behavior, the Rust port should model the necessary stream state explicitly rather than relying on implicit mutable global C state. Error handling should use `std::io::Result` and preserve observable failure behavior at call sites. Memory ownership should be expressed through Rust borrowing and scoped mutation so there is no manual lifetime or buffer management.

## Technical Context

- **Language/Version:** Rust 1.78+
- **Primary Dependencies:**
  - Rust standard library (`std::io`, `std::fs`, `std::os::*` as needed by platform-specific file descriptor access)
  - No third-party crates are recommended based on the provided module scope.
- **Testing:** `cargo test`
- **Performance Goals:**
  - Preserve the current operational cost profile of flush-related paths.
  - Avoid unnecessary allocations in flush and position-cache update paths.
  - Keep seek-state updates constant-time and limited to mutable state already associated with the stream abstraction.
  - Do not introduce extra synchronization or background processing.

## Module Mapping

| C Source File | Rust Module/File | Notes |
|---|---|---|
| `fflush.c` | `src/main_root_fflush.rs` | Direct migration target for the module-level logic. Keep all four migrated functions together unless an existing project file layout requires placement in `src/main.rs`-adjacent support code. |

| C Function | Rust Function/Method | Migration Notes |
|---|---|---|
| `disable_seek_optimization` | `disable_seek_optimization(...)` | Convert to an internal helper that updates explicit Rust stream state flags. |
| `restore_seek_optimization` | `restore_seek_optimization(...)` | Pair with the disable helper; restore prior optimization state without expanding semantics. |
| `update_fpos_cache` | `update_fpos_cache(...) -> io::Result<()>` | Represent cached file-position updates explicitly; return `Result` for any underlying seek/query failure. |
| `rpl_fflush` | `rpl_fflush(...) -> io::Result<()>` | Main replacement flush entry point; preserve ordering between flush behavior and seek-cache maintenance. |

## Data Model

No C data structures were provided for this module, so the Rust plan should introduce only the minimum internal state required to represent behavior already implied by the functions.

| C Representation | Rust Representation | Notes |
|---|---|---|
| Implicit `FILE *` state and associated seek/position flags | Internal stream wrapper or borrowed mutable stream state struct | Use a narrowly scoped Rust struct only if needed to hold cached position and optimization flags. Do not generalize beyond this module. |
| Cached file position state | `Option<u64>` or equivalent field in internal state | Use `Option` to represent known vs unknown cached position. |
| Temporary optimization disable/restore state | `bool` fields or a small internal enum | Prefer simple flags unless the C logic clearly distinguishes more than enabled/disabled/unknown states. |
| C error returns / errno-style failure | `std::io::Result<()>` / `std::io::Error` | Convert low-level failures directly into Rust I/O errors. |

### Proposed Minimal Internal Rust Types
If the port requires explicit state tracking, keep it minimal:

```rust
struct StreamState {
    seek_optimization_enabled: bool,
    cached_position: Option<u64>,
}
```

If the surrounding Rust code already has a stream abstraction, extend that existing type instead of introducing a second wrapper.

## Implementation Phases

### Phase 1: Establish module skeleton and function surfaces
- Create `src/main_root_fflush.rs` for the migrated logic.
- Add Rust equivalents for:
  - `disable_seek_optimization`
  - `restore_seek_optimization`
  - `update_fpos_cache`
  - `rpl_fflush`
- Define the minimum internal state representation needed for seek optimization flags and cached file position.
- Choose function signatures that use `&mut` borrowing and `std::io::Result` rather than raw pointer mutation and integer status codes.
- Keep visibility restricted (`pub(crate)` or private) according to actual call usage in the existing Rust port structure.

### Phase 2: Port state transitions and flush behavior
- Translate the C control flow for seek optimization disable/restore directly into Rust helper logic.
- Port cached file-position update behavior, including invalidation when position is unknown or cannot be safely retained.
- Implement `rpl_fflush` so that:
  - flush execution remains the central operation,
  - state updates happen in the same relative order as the C implementation,
  - failures short-circuit through `io::Result`,
  - no extra fallback behavior is introduced.
- Ensure mutable borrowing boundaries clearly separate stream operations from cache/flag mutation.

### Phase 3: Integrate with existing main-cluster call sites
- Replace references to the C-side module behavior with the Rust module entry points on branch `018-main_root_fflush.c_18-rust-port`.
- Keep the integration local to the current module’s callers; do not refactor unrelated stream or main-cluster code.
- Verify any existing stream abstractions can supply the information needed for flush and position-cache updates without widening module scope.
- Confirm that resource ownership remains with the existing caller and that this module does not assume responsibility for opening, closing, or duplicating streams.

### Phase 4: Validate behavior with targeted tests
- Add `cargo test` coverage for:
  - successful flush path,
  - flush path with cached-position update,
  - disable/restore optimization state transitions,
  - error propagation when flush or position retrieval fails.
- Prefer unit tests in the module unless existing project conventions require integration tests.
- Use deterministic temporary-file scenarios from the standard library for file-backed stream behavior.
- Confirm no unintended state retention occurs across repeated flush calls.