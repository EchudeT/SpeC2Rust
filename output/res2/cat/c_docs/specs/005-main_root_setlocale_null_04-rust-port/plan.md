# Implementation Plan

## Summary

This module ports the `setlocale_null` helper logic from the C sources `setlocale_null.c` and `setlocale_null-unlocked.c` into Rust for the `cat` project branch `005-main_root_setlocale_null_04-rust-port`.

The Rust implementation should preserve the existing behavior and call structure of the C code as closely as practical, with a narrow migration scope:

- migrate the unlocked and locking-aware helper functions,
- keep the distinction between lower-level helper routines and the public entry point,
- represent locale-query results using owned Rust strings rather than borrowed raw pointers,
- model failure with `Option` or `Result` at internal boundaries, while exposing the exact needs of the surrounding Rust port.

Because the analyzed module contains only functions and no standalone data structures, the Rust work is primarily a function-level translation with careful treatment of:

- null-handling from the C API,
- string ownership and lifetime conversion,
- lock-sensitive helper separation where present in the C layout,
- avoiding unnecessary feature expansion beyond the existing module responsibilities.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**:
  - Rust standard library
  - No third-party crates are recommended based on the provided module analysis
- **Testing**:
  - `cargo test`
  - unit tests for function-level behavior
  - integration through the existing `cat` Rust port where this helper is consumed
- **Performance Goals**:
  - preserve the lightweight helper nature of the original C code,
  - avoid unnecessary allocations beyond the string materialization required to replace C pointer returns,
  - keep helper layering shallow and close to the original control flow,
  - maintain behavior appropriate for repeated locale queries without introducing additional synchronization layers not present in the migration target.

## Module Mapping

### Source File Mapping

- `setlocale_null-unlocked.c` -> `src/main_root_setlocale_null_04/setlocale_null_unlocked.rs`
- `setlocale_null.c` -> `src/main_root_setlocale_null_04/setlocale_null.rs`

If the surrounding port already consolidates small C modules into a single Rust module file, these may instead be placed under:

- `src/main_root_setlocale_null_04.rs`

The preferred approach is to keep the file split aligned with the original C sources unless the current Rust branch structure already dictates a single-file module.

### Function Mapping

- `setlocale_null_unlocked` -> `setlocale_null_unlocked`
- `setlocale_null_r_unlocked` -> `setlocale_null_r_unlocked`
- `setlocale_null_r_with_lock` -> `setlocale_null_r_with_lock`
- `setlocale_null_r` -> `setlocale_null_r`
- `setlocale_null` -> `setlocale_null`

Notes:

- The repeated appearances of `setlocale_null_r_with_lock` in the analysis should be treated as one Rust function unless the actual C sources reveal conditional variants or macro-expanded forms that must be collapsed into a single implementation.
- Function visibility should be minimized:
  - public only where required by callers in the Rust port,
  - helper routines kept `pub(crate)` or private.

## Data Model

No explicit C structs were identified for this module.

### C-to-Rust Type Mapping

The main migration work is conversion of C idioms to Rust value types:

- `char *` / `const char *` locale result -> `String`
- nullable string result -> `Option<String>`
- status-returning helper with output buffer semantics -> `Result<String, LocaleQueryError>` or internal `Option<String>` plus caller-side adaptation
- locale category integer parameters -> `i32` or a small Rust type alias matching surrounding libc usage in the port

### Error Model

Because the C implementation likely uses null pointers and return-status conventions, the Rust port should adopt a minimal internal error representation only if needed to preserve call layering:

```rust
enum LocaleQueryError {
    QueryFailed,
    InvalidLocaleData,
}
```

This enum should be introduced only if the translated control flow needs to distinguish failure modes. If the C code only distinguishes success from null/failure, prefer `Option<String>`.

### Memory Management Mapping

- C-owned or static locale strings returned from `setlocale` should be copied into owned Rust `String` values before leaving unsafe boundaries.
- No Rust-side manual memory management should be introduced.
- Temporary buffers from the C version, if any, should map to `String` or `Vec<u8>` only where directly required by the original implementation.

## Implementation Phases

## Phase 1: Establish module skeleton and translate unlocked helpers

- Create the Rust module file(s) corresponding to:
  - `setlocale_null-unlocked.c`
  - `setlocale_null.c`
- Port:
  - `setlocale_null_unlocked`
  - `setlocale_null_r_unlocked`
- Keep the translated signatures close to the original call graph, adapting pointer-based outputs to owned Rust return values.
- Isolate any unavoidable unsafe locale calls into the smallest possible internal blocks.
- Add initial unit tests covering:
  - successful locale string retrieval path,
  - null/failure propagation,
  - empty or invalid string handling if present in the C logic.

## Phase 2: Port locking-aware helper path and unify internal behavior

- Port:
  - `setlocale_null_r_with_lock`
  - `setlocale_null_r`
- Preserve the original distinction between unlocked and lock-using paths without inventing new concurrency abstractions.
- Collapse duplicate analyzed entries for `setlocale_null_r_with_lock` into one implementation after verifying the actual C source.
- Ensure the Rust helper layering mirrors the C responsibilities:
  - low-level query helper,
  - lock-aware wrapper if present,
  - common result normalization.
- Add tests focused on equivalence between unlocked and wrapper paths where both should produce the same observable result.

## Phase 3: Port public entry point and integrate into crate layout

- Port the public function:
  - `setlocale_null`
- Connect it to the already translated helper functions with the same decision flow as in C.
- Align visibility and module exports with the rest of the Rust `cat` port.
- Confirm that return types fit existing callers without adding compatibility layers beyond what the branch already uses.
- Add crate-level tests or integration coverage for the final public API behavior.

## Phase 4: Cleanup, validation, and migration completion

- Review translated code for:
  - unnecessary allocations,
  - excess cloning,
  - unsafe scope minimization,
  - exact handling of null and error cases.
- Remove any temporary translation scaffolding introduced during earlier phases.
- Run:
  - `cargo test`
  - `cargo fmt`
  - `cargo clippy` if already standard for the branch
- Verify the final module remains limited to the original C file responsibilities and does not add unrelated locale facilities.