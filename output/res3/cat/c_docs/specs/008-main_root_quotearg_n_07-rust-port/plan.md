# Implementation Plan: main_root_quotearg_n_07

## Summary

This module ports the `quotearg.c` entry-point functions `quotearg_n`, `quotearg_n_mem`, and `quotearg_n_custom` into Rust for the `cat` project branch `008-main_root_quotearg_n_07-rust-port`.

The Rust implementation should preserve the existing call-level behavior and data flow of the C module rather than redesigning quoting behavior. The technical approach is to migrate the current per-call and per-slot quoting logic into a Rust module that uses owned buffers (`String` or `Vec<u8>` as appropriate) and explicit argument validation in place of C pointer arithmetic and implicit global-state patterns. The implementation should stay narrowly scoped to the listed functions and the data they directly depend on in `quotearg.c`.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Maintain behavior comparable to the C implementation for repeated quoting calls.
  - Avoid unnecessary reallocations where the C code reuses per-slot storage.
  - Preserve byte-oriented handling for memory-based inputs so non-UTF-8 data does not force lossy conversion unless required by the surrounding Rust API.
  - Keep implementation overhead limited to bounds checks and safe buffer management expected in idiomatic Rust.

## Module Mapping

| C File | C Functions | Rust Target |
|---|---|---|
| `quotearg.c` | `quotearg_n` | `src/quotearg.rs` -> `pub fn quotearg_n(...)` |
| `quotearg.c` | `quotearg_n_mem` | `src/quotearg.rs` -> `pub fn quotearg_n_mem(...)` |
| `quotearg.c` | `quotearg_n_custom` | `src/quotearg.rs` -> `pub fn quotearg_n_custom(...)` |

### Rust module placement

- Add or update `src/quotearg.rs` for the migrated implementation.
- Expose only the functions needed by the current `cat` port path.
- If the branch already contains partial quoting support, extend that file directly instead of introducing a parallel module.

## Data Model

The source analysis lists only anonymous C data structures. Because the target functions come from `quotearg.c`, the Rust port should map the underlying C layout patterns into a minimal set of internal Rust types that support the three listed functions without broadening the module surface.

| C Data Shape | Rust Mapping | Notes |
|---|---|---|
| Anonymous option/config struct used by quoting helpers | `struct QuotingOptions` | Rust-owned configuration object replacing mutable C structs passed by pointer. |
| Anonymous slot state for `quotearg_n` storage reuse | `struct QuoteSlot` | Holds the last rendered quoted buffer for a given slot index. |
| Anonymous aggregate for multiple slots | `struct QuoteSlotStore` | Backing storage for indexed `n` access; likely `Vec<QuoteSlot>`. |
| Anonymous custom-quote parameter pair | `struct CustomQuotingStyle` or fields inside `QuotingOptions` | Represents left/right quote delimiters for `quotearg_n_custom`. |
| Anonymous style/category constants | `enum QuotingStyle` | Replaces C integer constants/macros where present. |
| Anonymous raw buffer references | `&[u8]` / `Vec<u8>` | Used for `quotearg_n_mem` input and any byte-preserving internals. |
| Anonymous returned string buffers | `String` or `Vec<u8>` stored in slot state | Choose based on whether callers require UTF-8 or byte-preserving return form. Prefer matching existing branch conventions. |
| Anonymous nullable pointer fields | `Option<T>` / `Option<&T>` | Replaces null checks explicitly. |
| Anonymous size/length fields | `usize` | Direct replacement for C length/count fields. |
| Anonymous flag fields | `bool` / integer bitfields only if required | Use booleans unless exact bit semantics are already present in the branch. |

### Memory management notes

- Replace C static/per-slot allocated buffers with Rust-owned storage in a slot table.
- Grow slot storage lazily when `quotearg_n` is called with a larger index.
- Return values should be derived from owned slot buffers so no dangling references arise.
- For `quotearg_n_mem`, treat input as raw bytes and avoid assuming UTF-8.
- For `quotearg_n_custom`, validate custom delimiters before use and represent absent values explicitly rather than relying on null pointers.

### Error handling notes

- C implementations often assume valid inputs and may abort on invalid custom quoting arguments. In Rust, preserve observable behavior as closely as practical for this branch:
  - use explicit precondition checks for invalid slot indexes or invalid custom delimiter combinations;
  - prefer existing project conventions for internal failure paths;
  - avoid introducing new public error enums unless the surrounding Rust port already uses them for `quotearg`.

## Implementation Phases

### Phase 1: Establish Rust module skeleton and internal storage mapping

- Create or update `src/quotearg.rs`.
- Identify the C-local state directly required by `quotearg_n`, `quotearg_n_mem`, and `quotearg_n_custom`.
- Introduce minimal Rust equivalents for:
  - quoting options/configuration,
  - slot-based cached output storage,
  - custom delimiter representation.
- Map C integer sizes and pointer/null usage to `usize`, slices, and `Option`.
- Keep visibility narrow: public only for the three migrated functions and only the minimum additional internal items required.

### Phase 2: Port `quotearg_n_mem` as the byte-oriented core

- Implement `quotearg_n_mem` first, since it is the most direct length-aware entry point.
- Translate C buffer sizing and write-path logic into safe Rust buffer building.
- Preserve per-slot reuse semantics by storing the rendered result in the slot indexed by `n`.
- Ensure correct handling of:
  - arbitrary byte input,
  - zero-length input,
  - slot growth when `n` exceeds current storage.
- Add focused unit tests against representative byte cases and slot reuse behavior.

### Phase 3: Layer `quotearg_n` on top of the memory-based path

- Implement `quotearg_n` as the string/implicit-length wrapper over `quotearg_n_mem`.
- Mirror the C call chain rather than duplicating quoting logic.
- Use the existing branch’s string conventions for converting input to bytes.
- Add tests confirming wrapper equivalence with the memory-based implementation for standard string inputs.

### Phase 4: Port `quotearg_n_custom` and finalize behavior alignment

- Implement `quotearg_n_custom` using the same internal path, adding only the custom delimiter option wiring required by the C function.
- Validate delimiter inputs according to the original C expectations before invoking the quoting core.
- Reconcile any return-type or buffer-lifetime differences so all three functions share the same slot storage model.
- Add tests covering:
  - custom left/right delimiters,
  - repeated calls to the same slot,
  - interactions between default and custom quoting on different slot indexes.
- Perform final cleanup to remove C-style temporary patterns no longer needed after safe Rust ownership is in place.