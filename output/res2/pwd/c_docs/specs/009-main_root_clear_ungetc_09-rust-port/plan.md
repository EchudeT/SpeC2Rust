# Implementation Plan

## Summary

Port the C module `fflush.c` into a focused Rust module that preserves the existing scope: clearing any pushed-back input state represented by the original `ungetc`-related logic, including the variant that preserves the current stream position. The Rust implementation should stay close to the original control flow and file/function boundaries, translating the two C functions into a small Rust module with explicit result-based error handling.

The technical approach is to migrate the behavior into safe Rust where possible, using the standard library for I/O abstractions and position management. Because C `FILE*` plus `ungetc` semantics do not map directly to Rust’s standard stream types, the Rust side should model the required behavior as utility functions operating on seekable/readable stream state rather than trying to reproduce C runtime internals. Any logic that depends on stream position should be expressed through `Seek`/`stream_position`-style APIs, with ownership and borrowing making state transitions explicit.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**:
  - Rust standard library
  - No third-party crates recommended based on the provided input
- **Testing**:
  - `cargo test`
- **Performance Goals**:
  - Preserve the original module’s low overhead characteristics
  - Avoid unnecessary allocations
  - Keep stream-position operations bounded to the minimum needed to clear pushed-back state
  - Match C behavior closely enough for command-line utility use without adding extra abstraction layers

## Module Mapping

- **C source file**: `fflush.c`
- **Rust module/file**: `src/main_root_clear_ungetc_09.rs` or `src/main_cluster/main_root_clear_ungetc_09.rs` depending on the existing crate layout
- **Function mapping**:
  - `clear_ungetc_buffer_preserving_position` → `clear_ungetc_buffer_preserving_position`
  - `clear_ungetc_buffer` → `clear_ungetc_buffer`

If the current Rust port keeps a flat binary-oriented layout, prefer a single module file under `src/` and wire it into the existing `main`-cluster module tree without introducing extra helper modules.

## Data Model

No explicit C structs were identified in the input for this module.

### C to Rust mapping

- **C stream state (`FILE *`, implicit libc-managed unread buffer)**
  → Rust generic stream parameters constrained by standard I/O traits as needed:
  - `Read`
  - `Seek`
  - possibly `BufRead` only if directly required by the migrated logic

Because Rust does not expose a direct equivalent of C’s internal `ungetc` buffer, the Rust implementation should represent the needed capability through stream operations and documented assumptions about the caller-provided stream type. If the ported code already maintains its own reader abstraction elsewhere in the project, reuse that existing type instead of introducing a new wrapper.

- **C integer/status return codes**
  → `std::io::Result<()>` or `Result<(), std::io::Error>`

This keeps error propagation explicit and avoids sentinel return values.

## Implementation Phases

### Phase 1: Module skeleton and function signature migration

- Create the Rust module corresponding to `fflush.c`.
- Add Rust function signatures for:
  - `clear_ungetc_buffer_preserving_position`
  - `clear_ungetc_buffer`
- Choose the narrowest trait bounds required by the original logic, preferring standard-library traits over custom abstractions.
- Convert C-style status returns into `io::Result<()>`.
- Keep names close to the original C names for traceability during review.

### Phase 2: Core logic port

- Port the body of `clear_ungetc_buffer_preserving_position` first, since it carries the stricter behavioral requirement around stream position.
- Express position preservation via `Seek` operations and ensure any temporary cursor movement is restored before returning success.
- Port `clear_ungetc_buffer` next, reusing the preserved-position routine only if that matches the original dependency direction and does not alter behavior.
- Replace any implicit C memory/state handling with explicit Rust borrows and scoped mutable access.
- Avoid adding new buffering layers or generalized stream adapters beyond what is necessary to represent the existing behavior.

### Phase 3: Error handling and edge-case alignment

- Audit all I/O operations for failure paths and propagate errors directly with `Result`.
- Ensure partial state changes do not leave the caller’s stream position unexpectedly modified when the preserving-position function reports success.
- Review EOF, non-seekable stream incompatibility, and invalid-position cases according to what the Rust signatures permit.
- If non-seekable streams cannot support the preserved-position variant, reflect that through trait bounds rather than runtime feature expansion.

### Phase 4: Tests and integration verification

- Add unit tests covering:
  - clearing unread/pushed-back state in the normal case
  - preserving position after clearing
  - no-op behavior when there is no unread state to clear
  - error propagation from failing seek/read operations where testable
- Use in-memory streams such as `Cursor` where they can model the required behavior.
- Integrate the module into the existing branch layout and confirm it builds and passes `cargo test`.
- Keep tests targeted to the two migrated functions and avoid introducing broader harnesses not required by this module.