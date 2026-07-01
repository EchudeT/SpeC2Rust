# Implementation Plan

## Summary

Port the `quotearg.c` functionality needed by `main_root_quote_n_10` into Rust by migrating only the `quote_n_mem` and `quote_n` call paths and the minimum supporting data definitions they require. The Rust implementation should preserve the original indexed-quote behavior and string/memory handling semantics while replacing manual buffer management with owned Rust types.

The implementation approach is to:
- map the relevant quoting state from the C file into a small Rust module,
- implement `quote_n_mem` as the core routine over byte slices,
- layer `quote_n` on top for NUL-terminated string-style input via `&str`/`&[u8]`,
- keep behavior aligned with the existing C logic without broadening the quoting API surface beyond what these two functions need.

The migration should stay narrowly scoped to the existing file and functions, using standard-library byte/string facilities and explicit error-free ownership transfer instead of C allocation patterns.

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve linear-time processing relative to input length.
  - Avoid unnecessary intermediate allocations beyond the returned quoted result.
  - Use borrowed byte slices as inputs where possible.
  - Keep per-call state localized and avoid global mutable buffers when migrating indexed quote storage.

## Module Mapping

| C File / Function | Rust Location | Notes |
|---|---|---|
| `quotearg.c` | `src/main_root_quote_n_10.rs` | Restrict migration to logic needed by this module. |
| `quote_n_mem` | `pub(crate) fn quote_n_mem(...)` | Core implementation operating on byte slices and quote index. |
| `quote_n` | `pub(crate) fn quote_n(...)` | Thin wrapper delegating to `quote_n_mem`. |

If the current Rust crate already centralizes ported quoting code in an existing file, place these functions there instead of creating a broader new subsystem; otherwise use a single module file dedicated to this migration unit.

## Data Model

The C analysis lists only anonymous structures, so the Rust mapping should be derived from actual usage within `quote_n_mem` and `quote_n`, not by reproducing every unnamed C layout. Only data structures directly referenced by these functions should be migrated.

| C Data Shape | Rust Mapping | Migration Guidance |
|---|---|---|
| Anonymous quoting option/state structs used by `quotearg.c` | Private `struct` with named fields only if required by `quote_n_mem` / `quote_n` | Introduce only the fields actually read by the migrated functions. |
| Anonymous slot/buffer bookkeeping for quote index `n` | `Vec<Option<String>>` or equivalent private storage | Replace manual allocation/reallocation with owned Rust storage. |
| C string pointer + length pairs | `&[u8]` for raw input, `&str` only where valid UTF-8 is guaranteed | Prefer byte-oriented processing to preserve C semantics. |
| Output buffers allocated in C | `String` if output is textual, otherwise `Vec<u8>` with final conversion when valid | Choose one representation based on actual escaping logic used by these functions. |
| Integer quote index / sizes | `usize` | Use checked indexing and growth. |

### Memory Management

- Replace any static or heap-managed C slot arrays with owned Rust containers.
- Avoid raw pointers in the public internal API for this module.
- Grow quote-slot storage through `Vec` capacity management rather than manual `realloc`.
- Return owned output rather than references into mutable shared buffers unless existing crate architecture already defines a safe equivalent.

### Error Handling

These C functions typically do not expose rich errors. The Rust port should therefore:
- keep infallible APIs where practical,
- avoid `unwrap` on input-dependent operations,
- use byte-oriented logic to avoid UTF-8 conversion failures during core processing,
- convert to `String` only when guaranteed safe by implementation choice.

## Implementation Phases

## Phase 1: Extract and map the minimal C logic

- Inspect `quotearg.c` and isolate the exact code paths and helper state touched by:
  - `quote_n_mem`
  - `quote_n`
- Identify which anonymous structs, constants, and helper routines are actually required.
- Create the Rust module file for this migration unit.
- Define the minimal private Rust data structures needed to represent:
  - quote slot storage by index,
  - any quoting options directly consumed by these functions,
  - input/output buffer handling.

### Deliverables
- Rust module skeleton with function signatures.
- Private type definitions limited to the required C state.
- Notes in code comments documenting which C fields were intentionally omitted because the target functions do not use them.

## Phase 2: Implement core quoting and indexed storage

- Port `quote_n_mem` first as the canonical implementation.
- Use `&[u8]` input and `usize` index to mirror C pointer-plus-length behavior.
- Reproduce the original slot-selection and slot-growth semantics using safe `Vec` operations.
- Replace C buffer writes with Rust-owned output construction.
- Ensure repeated calls with different `n` values update the correct stored slot result.

Then implement `quote_n` as a wrapper:
- accept the Rust equivalent of a C string input,
- determine length in the wrapper rather than duplicating core logic,
- delegate directly to `quote_n_mem`.

### Deliverables
- Working `quote_n_mem`.
- Working `quote_n`.
- Internal storage management replacing C allocation and reuse behavior.

## Phase 3: Verify behavior and edge conditions

- Add unit tests covering:
  - empty input,
  - simple unquoted/quoted content cases from the migrated behavior,
  - embedded bytes if supported by the C path under `quote_n_mem`,
  - multiple quote indices to verify slot isolation,
  - repeated calls to the same index to verify replacement semantics.
- Compare outputs against the C implementation for representative cases taken directly from the original logic around these functions.
- Review for allocation and indexing safety:
  - no out-of-bounds slot access,
  - no dependence on dangling references,
  - no accidental UTF-8 assumptions in byte-processing paths.

### Deliverables
- `cargo test` coverage for the migrated functions.
- Final pass removing unused placeholder fields or helpers not needed by this module.

## Phase 4: Integrate with the branch module boundary

- Connect the Rust module into the `011-main_root_quote_n_10-rust-port` branch’s existing crate structure.
- Update call sites within the ported codebase only as needed to use the new Rust functions.
- Keep naming close to the C originals for traceability during review.
- Confirm the final module remains limited to the functionality represented by `quote_n_mem` and `quote_n` and does not introduce broader quoting abstractions.

### Deliverables
- Integrated module buildable in the target branch.
- Stable function mapping from original C names to Rust implementations.