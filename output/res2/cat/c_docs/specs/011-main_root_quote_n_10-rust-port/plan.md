# Implementation Plan

## Summary

Port the `quotearg.c` slice for `main_root_quote_n_10` into Rust by migrating the two public entry points `quote_n_mem` and `quote_n` into a single Rust module that preserves the existing call shape and index-based quoting behavior.

The Rust implementation should stay narrow:
- move only the logic required by these two functions,
- keep quoting state and slot-based output management local to the module,
- use borrowed byte/string inputs where possible,
- allocate owned output only for returned quoted results,
- replace C pointer/length handling with Rust slices and `String`/`Vec<u8>` as appropriate.

The technical approach is to translate the relevant quoting path from `quotearg.c` into safe Rust first, introducing small internal helpers only when needed to preserve the original function boundaries and behavior. Any C static storage or per-slot cached buffers used by `quote_n_mem`/`quote_n` should be represented with module-private Rust storage structures, without widening the API or adding extra features.

## Technical Context

### Language/Version
- Rust stable, edition 2021
- Minimum recommended compiler: `rustc 1.74` or newer

### Primary Dependencies
- Rust standard library only

No third-party crate is recommended from the available input. The required work is centered on string/byte transformation, index-based storage, and ownership management, all of which are covered by `std`.

### Testing
- `cargo test`

Testing should focus on:
- parity of `quote_n_mem` and `quote_n` behavior for representative inputs,
- slot/index isolation across repeated calls,
- byte-length handling for `quote_n_mem`,
- handling of empty input, embedded non-printable bytes, and repeated quoting calls.

### Performance Goals
- Preserve linear-time processing with respect to input length.
- Avoid unnecessary intermediate allocations beyond the final quoted buffer.
- Reuse per-index storage if the C implementation does so for returned buffers.
- Keep hot-path logic on byte slices where feasible, converting to UTF-8 strings only when required by the Rust-facing API.

## Module Mapping

### C to Rust File Mapping
- `quotearg.c` -> `src/main_root_quote_n_10.rs`

If the project already has a central module tree, expose this file from the existing `mod` structure only as needed for the current branch. Do not split this port into additional files unless existing project layout requires it.

### Function Mapping
- `quote_n_mem` -> `pub(crate)` or `pub` Rust function `quote_n_mem`
- `quote_n` -> `pub(crate)` or `pub` Rust function `quote_n`

Recommended Rust-facing shapes:
- `quote_n_mem(n, input: &[u8]) -> ...`
- `quote_n(n, input: &str) -> ...`

Final return types should be chosen based on how the surrounding port expects ownership:
- prefer owned `String` for text-returning behavior,
- use `Vec<u8>` only if the original path must preserve arbitrary bytes that are not valid UTF-8.

Internal helpers may be added only to support direct migration of shared quoting logic used by these two functions.

## Data Model

The analysis only reports anonymous C data structures. For this module, map structures by responsibility rather than by original anonymous names.

### Data-structure Mapping
- anonymous static slot array/state -> module-private Rust struct such as `QuoteSlots`
- anonymous per-slot buffer record -> module-private Rust struct such as `QuoteSlot`
- anonymous quoting options/state used by these functions -> module-private Rust struct or enum only if required by the migrated code path
- anonymous temporary buffer views -> Rust slices (`&[u8]`) or owned buffers (`Vec<u8>`)
- anonymous C string/pointer pairs -> `&str`, `&[u8]`, `String`, or `Vec<u8>` depending on call boundary

### Recommended Rust Representations
- C `char *` input with explicit length -> `&[u8]`
- C NUL-terminated string input -> `&str` at API boundary, internally `as_bytes()`
- C returned pointer to cached quoted string -> owned Rust return value, or module-managed slot storage plus borrowed return only if the wider port requires matching aliasing semantics
- C global/static mutable storage -> module-private state with explicit ownership and controlled mutation

### Memory Management Decisions
- Eliminate raw pointer arithmetic in favor of slice indexing.
- Replace manual buffer growth with `String::with_capacity` or `Vec::with_capacity`.
- If original functions rely on slot-numbered retained buffers, represent them with a growable vector of per-slot owned buffers.
- Avoid exposing references tied to mutable global state unless required by the surrounding API; prefer returning owned values from the Rust port.

### Error Handling Decisions
- If the original C code assumes infallible quoting under normal allocation semantics, keep the Rust API non-`Result` and rely on standard allocation failure behavior.
- If invalid UTF-8 can arise from `quote_n_mem`, keep processing at the byte level internally and convert only in a controlled way.
- Do not introduce custom error hierarchies unless a direct migration dependency requires them.

## Implementation Phases

### Phase 1: Establish module skeleton and direct function boundaries
- Create `src/main_root_quote_n_10.rs`.
- Add Rust equivalents for `quote_n_mem` and `quote_n` with signatures aligned to the surrounding project.
- Identify the minimal internal state required by these two functions from `quotearg.c`.
- Define module-private Rust types for slot storage and any required quoting context.
- Wire the module into the existing crate layout without adding unrelated modules.

### Phase 2: Port core quoting logic and slot management
- Translate the shared quoting path used by `quote_n_mem` and `quote_n`.
- Replace C pointer/length traversal with slice-based iteration.
- Implement index-based slot lookup/growth matching the C behavior for `n`.
- Port only the escaping/quoting branches reachable from these two entry points.
- Keep buffer ownership explicit so repeated calls do not alias mutable memory unsafely.

### Phase 3: Align returned value behavior and edge cases
- Confirm how the Rust port should represent outputs for both byte-length and string-based entry points.
- Handle empty input, embedded special bytes, and boundary cases around slot indices.
- Ensure `quote_n` delegates through the same core path as `quote_n_mem` where appropriate.
- Remove any remaining C-style assumptions about sentinel terminators or writable shared buffers.

### Phase 4: Verification and cleanup
- Add focused unit tests for:
  - basic quoting results,
  - `quote_n_mem` length-sensitive behavior,
  - repeated calls with the same slot,
  - calls across different slots,
  - empty and special-character inputs.
- Compare results against the C behavior for representative cases from the migrated path.
- Simplify internal helpers only after parity is established.
- Keep the final module limited to migrated functionality from `quotearg.c`.