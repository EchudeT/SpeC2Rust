# Implementation Plan: main_root_quotearg.c_32

## Summary

This module ports the quoting logic currently implemented in `quotearg.c` into a Rust module that preserves existing call patterns and output behavior used by the `cat` project. The implementation should focus on migrating the existing functions as closely as practical:

- `gettext_quote`
- `quotearg_buffer_restyled`
- `quotearg_free`
- `quotearg`
- `quotearg_mem`
- `quotearg_char`
- `quote_mem`
- `quote`

The Rust approach should keep the implementation concentrated in a single module corresponding to the C source file, with function-level migration rather than redesign. The main technical work is translating C string and buffer handling into safe Rust while preserving byte-oriented behavior, especially for routines that operate on arbitrary memory rather than validated UTF-8 text.

The preferred implementation strategy is:

- represent quoting inputs primarily as `&[u8]` where C code handled pointer-plus-length data;
- build outputs with `Vec<u8>` or `String` depending on whether the function contract is byte-oriented or text-oriented;
- replace global/manual allocation cleanup patterns with Rust ownership, while keeping a minimal compatibility layer for functions that in C depended on reusable internal buffers;
- encode C enum/flag-style behavior as Rust enums and small option structs only where required by the existing logic in `quotearg.c`.

## Technical Context

### Language/Version

- Rust stable, edition 2021
- Minimum recommended compiler: `rustc 1.74` or newer

### Primary Dependencies

Use the Rust standard library by default.

Recommended dependencies:
- None required initially

If tests need byte-for-byte behavioral comparison fixtures, keep them within the standard test framework unless an existing project dependency already covers this.

### Testing

- `cargo test`

Testing focus:
- byte-accurate output for representative quoting cases;
- behavior for empty input, ASCII input, embedded special characters, and arbitrary byte slices;
- parity across wrapper functions (`quotearg`, `quotearg_mem`, `quotearg_char`, `quote_mem`, `quote`);
- cleanup behavior equivalent to `quotearg_free` where applicable in the Rust design.

### Performance Goals

- Preserve linear-time processing over the input buffer.
- Avoid unnecessary intermediate allocations during quoting.
- Use preallocated output buffers where the C code used caller-provided buffers or where output growth is predictable.
- Keep wrapper functions thin so they do not duplicate quoting work.

## Module Mapping

### C to Rust File Mapping

- `quotearg.c` → `src/main_root_quotearg.rs`

If the repository already uses a different file naming convention for migrated modules, place the implementation in the corresponding existing module file and avoid splitting this C file across multiple new Rust modules.

### Function Mapping

- `gettext_quote` → `pub(crate) fn gettext_quote(...) -> ...`
- `quotearg_buffer_restyled` → `pub(crate) fn quotearg_buffer_restyled(...) -> ...`
- `quotearg_free` → `pub(crate) fn quotearg_free()`
- `quotearg` → `pub(crate) fn quotearg(...) -> ...`
- `quotearg_mem` → `pub(crate) fn quotearg_mem(input: &[u8]) -> ...`
- `quotearg_char` → `pub(crate) fn quotearg_char(...) -> ...`
- `quote_mem` → `pub(crate) fn quote_mem(input: &[u8]) -> ...`
- `quote` → `pub(crate) fn quote(...) -> ...`

### Mapping Notes

- Preserve the wrapper hierarchy from C rather than flattening all entry points into a new API.
- Functions that in C accepted `char *` and `size_t` should become byte-slice based APIs in Rust where possible.
- Functions that implicitly returned pointers to managed static/internal storage in C should be translated into owned Rust return values unless another existing module interface requires mutable shared storage.

## Data Model

The analysis lists only anonymous C data structures. For this migration, map them conservatively based on actual use in `quotearg.c` rather than inventing broader abstractions.

### Data-structure Mapping Strategy

- C anonymous struct used for quoting options/state → Rust `struct QuotingOptions`
- C anonymous enum/flag groups used to select quoting style → Rust `enum QuotingStyle`
- C bitfield/flag storage → Rust integer flags (`u32`/`usize`) or explicit boolean fields, whichever matches the original logic more directly
- C static slot/buffer records used for reusable quoted arguments → Rust `struct Slot` or `Vec<u8>`-backed storage owned by a module-local container
- C pointer-plus-length string references → Rust `&[u8]` or `Option<&[u8]>`
- C writable output buffer arguments → Rust `&mut Vec<u8>` or `&mut [u8]` plus returned written length, depending on how closely `quotearg_buffer_restyled` needs to mirror the original behavior

### Expected Rust Types

Because the source analysis does not provide named structs, define only the minimum required types after inspecting `quotearg.c`:

```rust
pub(crate) enum QuotingStyle {
    // variants derived directly from C style selectors
}

pub(crate) struct QuotingOptions {
    style: QuotingStyle,
    // direct fields only for flags and characters used in quotearg.c
}

struct Slot {
    buf: Vec<u8>,
}
```

### Memory Management Decisions

- Replace manual allocation and release with owned Rust buffers.
- If the C implementation maintains global reusable storage for `quotearg`-family wrappers, model it with module-local owned state only as far as needed for compatibility.
- `quotearg_free` should clear any such internal reusable storage; if the final Rust implementation uses only owned return values and no retained global buffers, keep `quotearg_free` as a no-op compatibility function.
- Avoid exposing raw pointers internally except at existing module boundaries that require them.

### Error Handling Decisions

- Preserve infallible behavior where the C code assumes allocation failure is unrecoverable.
- Internal helper functions should return lengths or owned buffers directly rather than introducing broad `Result` usage without evidence from the original interfaces.
- If exact interface compatibility requires reporting truncation or written size, mirror that with `usize`.

## Implementation Phases

## Phase 1: Inspect and Scaffold the Direct Port

Goals:
- create the Rust module file corresponding to `quotearg.c`;
- identify all local constants, static state, option fields, and style selectors used by the listed functions;
- define minimal Rust enums/structs needed to represent the C state.

Tasks:
- Read `quotearg.c` and enumerate anonymous structs/enums by actual field usage.
- Create `src/main_root_quotearg.rs` with placeholder signatures for all migrated functions.
- Translate C constants, quoting style tags, and flag values into Rust constants or enums.
- Decide the exact return types for each public wrapper based on existing Rust project call sites.

Exit criteria:
- the Rust module compiles with stubs;
- all C functions listed in scope have direct Rust counterparts;
- all required state representations are identified from the source file.

## Phase 2: Port Core Buffer Quoting Logic

Goals:
- implement `gettext_quote` and `quotearg_buffer_restyled` first, since the remaining functions are wrappers or storage helpers around this logic.

Tasks:
- Port quote selection logic from `gettext_quote` using Rust string/byte handling without changing behavior.
- Implement the byte-by-byte quoting transform in `quotearg_buffer_restyled`.
- Preserve C semantics for:
  - explicit input length handling;
  - quoting of arbitrary bytes;
  - special treatment of selected quote characters and styles;
  - output length calculation and buffer growth behavior.
- Use focused helper functions only when they correspond directly to repeated local logic in `quotearg.c`.

Testing:
- Add unit tests for representative input/output pairs derived from the C behavior.
- Cover empty slices, normal text, quote characters, and non-UTF-8 bytes.

Exit criteria:
- core quoting logic is implemented and tested;
- wrappers can rely on a stable core function.

## Phase 3: Port Wrappers and Internal Storage Behavior

Goals:
- implement the remaining entry points with minimal deviation from the C layering.

Tasks:
- Implement `quotearg_mem`, `quotearg`, `quotearg_char`, `quote_mem`, and `quote` as thin adapters over `quotearg_buffer_restyled`.
- Reproduce any internal slot reuse only if required by existing behavior or callers.
- Implement `quotearg_free` to clear module-local retained buffers if such storage is used; otherwise keep it as a compatibility no-op.

Testing:
- Add wrapper-level tests to confirm equivalent output paths.
- Verify repeated calls do not produce stale-buffer behavior regressions.

Exit criteria:
- all listed functions are fully implemented;
- no unresolved placeholder state remains;
- wrapper APIs compile at all call sites in the branch.

## Phase 4: Integration Cleanup and Parity Review

Goals:
- finish integration into the Rust branch with behavior preserved and implementation narrowed to the original file scope.

Tasks:
- Replace any temporary scaffolding types with direct representations matching the final port.
- Remove unused helpers introduced during translation.
- Review all interfaces for unnecessary allocation or UTF-8 assumptions.
- Confirm `cargo test` passes for the module and affected callers.

Exit criteria:
- module is fully migrated within the intended file scope;
- tests pass;
- implementation remains limited to the original `quotearg.c` responsibilities.