# Implementation Plan

## Summary

Port the `src/wordsplit/wordsplit.c` subset for `module_src_wordsplit_wordsplit_05` into a single Rust module that preserves the existing responsibilities of allocation cleanup, error-state reset/reporting, and word-vector access. The Rust implementation should migrate the behavior of:

- `wordsplit_free_parambuf`
- `wordsplit_clearerr`
- `wordsplit_free`
- `wordsplit_get_words`
- `wordsplit_strerror`
- `wordsplit_perror`

The implementation approach should stay close to the C layout and lifecycle:

- represent the owning wordsplit state as a Rust struct,
- replace manual buffer ownership and free paths with `Vec`, `String`, and `Option`,
- model error codes with a Rust enum plus stable message mapping,
- preserve explicit cleanup entry points where the original API had distinct free/reset functions,
- keep the module scope limited to the existing file/function set without introducing new subsystem boundaries.

This module is primarily a state-management and resource-release port, so the technical focus is deterministic ownership transfer, non-panicking cleanup behavior, and stable translation of C error/status handling into idiomatic Rust return/reference types.

## Technical Context

- **Language/Version**: Rust 1.76 or newer
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - preserve linear-time behavior for word-list access and cleanup paths,
  - avoid unnecessary string or vector cloning during `get_words` access,
  - ensure cleanup/reset operations are proportional only to currently owned buffers,
  - maintain negligible overhead versus the C implementation for normal state access and error reporting.

## Module Mapping

### C to Rust File Mapping

- `src/wordsplit/wordsplit.c`
  - migrate into a Rust source file following standard project layout, preferably:
    - `src/wordsplit.rs` if the project keeps a flat module layout, or
    - `src/wordsplit/mod.rs` if the crate already uses a directory-backed module.

Use one Rust module for this migration unit. Do not split these functions into extra helper modules unless required by the existing crate structure.

### Function Mapping

- `wordsplit_free_parambuf`
  - map to a Rust method on the owning state, e.g. `fn free_parambuf(&mut self)`
  - responsibility: clear/release parameter-buffer-related owned storage without dropping the entire wordsplit state

- `wordsplit_clearerr`
  - map to `fn clearerr(&mut self)`
  - responsibility: reset stored error code/message-related fields to the non-error state

- `wordsplit_free`
  - map to an owning cleanup method, e.g. `fn free(&mut self)` or to `Drop` plus a compatibility method
  - recommendation: keep an explicit `free`-named method for migration clarity, even if `Drop` also performs full cleanup

- `wordsplit_get_words`
  - map to a borrowing accessor, e.g. `fn get_words(&self) -> &[String]` or `Option<&[String]>`
  - choose the return shape based on whether the C function can represent “no words available” distinctly from an empty list

- `wordsplit_strerror`
  - map to `fn strerror(&self) -> &'static str` or `fn strerror(code: WordSplitError) -> &'static str`
  - use stable static message strings rather than heap allocation

- `wordsplit_perror`
  - map to `fn perror(&self, prefix: Option<&str>)`
  - emit to standard error via `eprintln!` or a `std::io::stderr` write path if formatting control is needed

## Data Model

Because the input exposes only anonymous C structures and not their field definitions, the Rust data model should be reconstructed strictly from the needs of the listed functions and from adjacent existing state in the same module during implementation. Do not invent new semantic containers beyond what is required to host the migrated fields.

### Core State Mapping

| C construct | Rust mapping | Notes |
|---|---|---|
| owning wordsplit state struct | `struct WordSplit` | Central state object containing words, parameter buffers, and error state |
| word array (`char **`-style storage) | `Vec<String>` | Own parsed words directly; if internal representation still needs borrowed slices during migration, confine that internally |
| optional allocated buffer | `Option<String>` or `Option<Vec<u8>>` | Choose `String` for text buffers, `Vec<u8>` only if byte-oriented ownership is required by the original field usage |
| manually tracked count/capacity | implicit `Vec` metadata | Avoid separate count fields unless required for compatibility with surrounding code |
| error code integer/macros | `enum WordSplitError` | Include a zero/non-error variant and explicit variants for known migrated cases |
| nullable message pointer | derived message via match | Prefer static message lookup over storing transient error strings unless the C code requires dynamic text |
| flags/bitmasks | `u32` or dedicated newtype | Keep raw integer flags if only carried through existing logic; do not over-model unless needed |

### Error Representation

Prefer a small internal enum:

```rust
enum WordSplitError {
    None,
    // migrated concrete variants from C error constants
    Unknown(i32),
}
```

Implementation notes:

- If the C file uses fixed symbolic error constants, translate each used constant into an enum variant.
- If some call paths can surface unmapped legacy codes, retain an `Unknown(i32)` variant or equivalent fallback.
- `clearerr` should set the state back to `WordSplitError::None`.
- `strerror` should map every variant to a static string.

### Cleanup and Ownership Mapping

- Any C heap buffers freed by `wordsplit_free_parambuf` should become owned fields wrapped in `Option<_>` or clearable `Vec<_>`.
- Any C heap buffers freed by `wordsplit_free` should be owned directly by `WordSplit` so Rust drop semantics naturally reclaim them.
- If the C implementation performs partial cleanup while leaving the object reusable, mirror that by clearing fields rather than replacing the struct wholesale.
- Avoid raw pointers in the Rust port unless required by surrounding unmigrated code in the same crate.

## Implementation Phases

## Phase 1: Reconstruct Minimal State and Error Types

- Inspect `src/wordsplit/wordsplit.c` and identify the exact fields touched by:
  - `wordsplit_free_parambuf`
  - `wordsplit_clearerr`
  - `wordsplit_free`
  - `wordsplit_get_words`
  - `wordsplit_strerror`
  - `wordsplit_perror`
- Define the Rust `WordSplit` struct with only the fields required by these functions and their immediate invariants.
- Define the Rust error representation for the migrated error codes.
- Establish the target Rust file location and wire the module into the crate without creating extra abstraction layers.

**Exit criteria**:
- `WordSplit` compiles with placeholder methods.
- All fields required for cleanup, word access, and error reporting are identified and typed.

## Phase 2: Port Cleanup and Access Logic

- Implement `free_parambuf` by converting each manual free/reset step into:
  - `Option::take()`,
  - `Vec::clear()`,
  - assignment to `None`,
  - or state reset as appropriate.
- Implement `clearerr` as a pure error-state reset.
- Implement `free` as the full-object cleanup path:
  - clear words,
  - clear parameter buffers,
  - reset error state,
  - release any remaining owned dynamic storage.
- Implement `get_words` as a borrowing accessor with no cloning.
- Preserve any C-visible distinction between “uninitialized”, “cleared”, and “empty result” only if the original function logic depends on it.

**Exit criteria**:
- Cleanup methods are idempotent where the C code allowed repeated free/reset calls.
- Word access returns stable borrowed data without allocation.

## Phase 3: Port Error Message and Printing Behavior

- Implement `strerror` using a fixed match table from migrated error variants/codes to static messages.
- Implement `perror` to print:
  - optional prefix,
  - separator formatting matching the C behavior closely,
  - current error text from `strerror`.
- Ensure unmapped/unknown codes still produce a deterministic fallback message.
- Keep output behavior simple and standard-library based.

**Exit criteria**:
- Error reset, lookup, and printing behave consistently across no-error and error cases.
- No heap allocation is required for standard error string retrieval.

## Phase 4: Validate Migration with Focused Tests

- Add unit tests covering:
  - `clearerr` resets error state,
  - `free_parambuf` clears only parameter-buffer-owned fields,
  - `free` clears all owned state and remains safe on repeated invocation,
  - `get_words` returns expected slices for empty and populated states,
  - `strerror` returns the expected message for each migrated error variant,
  - `perror` formatting logic where testable.
- Prefer direct state-based assertions for cleanup behavior rather than introducing new test harness layers.
- Run `cargo test` and resolve borrow/ownership issues by tightening field mutability and return types rather than adding indirection.

**Exit criteria**:
- All migrated functions are covered by focused tests.
- The Rust module compiles cleanly and passes `cargo test`.