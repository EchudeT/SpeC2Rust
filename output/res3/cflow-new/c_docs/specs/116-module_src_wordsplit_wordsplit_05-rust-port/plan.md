# Implementation Plan: module_src_wordsplit_wordsplit_05

## Summary

This module migration covers the resource-management and result-access portion of `src/wordsplit/wordsplit.c`, specifically the functions:

- `wordsplit_free_parambuf`
- `wordsplit_clearerr`
- `wordsplit_free`
- `wordsplit_get_words`
- `wordsplit_strerror`
- `wordsplit_perror`

The Rust implementation should preserve the existing module scope and behavior boundaries without adding new capabilities. The technical approach is to translate the C ownership model into explicit Rust ownership and borrowing:

- represent the wordsplit state as a Rust struct holding buffers, parsed words, flags, and the last error state;
- replace manual cleanup functions with targeted `clear`/`drop` logic while still exposing migration-equivalent methods for the original C functions;
- model C error codes as a Rust enum plus stable message conversion for `strerror`/`perror` equivalents;
- keep the implementation concentrated in the Rust counterpart of the existing wordsplit module rather than splitting into extra abstraction layers.

The focus of this plan is a faithful port of lifecycle, buffer cleanup, error reset, word retrieval, and error-text reporting.

## Technical Context

### Language / Version

- Rust stable, edition 2021
- Minimum recommended compiler: `rustc 1.76` or newer

### Primary Dependencies

Use the Rust standard library by default.

Recommended crates:
- None required for this module migration

### Testing

- `cargo test`

Testing should cover:
- freeing/clearing owned buffers without leaks or double-free-like behavior;
- resetting error state through the clear-error path;
- retrieving parsed words through the Rust accessor corresponding to `wordsplit_get_words`;
- stable string mapping for known error codes;
- formatted error output behavior for the `perror`-equivalent path.

### Performance Goals

- Preserve linear-time cleanup relative to the number of stored words/buffers.
- Avoid unnecessary string copying when returning or exposing parsed words.
- Use `Vec`/`String` capacity reuse where it naturally replaces C buffer reuse.
- Keep error-message lookup constant-time via static matching.

## Module Mapping

### C to Rust File Mapping

| C File | Rust File | Notes |
|---|---|---|
| `src/wordsplit/wordsplit.c` | `src/wordsplit/wordsplit.rs` or the existing Rust module file corresponding to `wordsplit` | Keep all migrated functions in the same module area; do not introduce extra helper modules unless already required by the crate layout. |

### Function Mapping

| C Function | Rust Target | Migration Notes |
|---|---|---|
| `wordsplit_free_parambuf` | `Wordsplit::free_parambuf(&mut self)` or private module function on the wordsplit state | Replace manual buffer deallocation with clearing owned Rust fields that represent parameter buffers. |
| `wordsplit_clearerr` | `Wordsplit::clear_error(&mut self)` | Reset stored error code/message-related state to the no-error value. |
| `wordsplit_free` | `Wordsplit::free(self)` or `Wordsplit::reset(&mut self)` depending on call sites | Prefer `Drop` and ownership semantics; keep an explicit compatibility method only if needed by existing migration boundaries. |
| `wordsplit_get_words` | `Wordsplit::words(&self) -> &[String]` or equivalent borrowed accessor | Return borrowed word storage instead of raw pointer arrays. |
| `wordsplit_strerror` | `WordsplitError::as_str()` or module-level `wordsplit_strerror(code)` | Map integer-style error codes into static string slices. |
| `wordsplit_perror` | method or helper writing formatted error text to stderr | Use `eprintln!` or `std::io::Write` to standard error while preserving prefix + message style. |

## Data Model

Because the analysis only exposes anonymous C data structures, the Rust data model should be derived from direct field usage in `src/wordsplit/wordsplit.c` and restricted to the fields needed by the listed functions.

### Primary State Mapping

| C Shape | Rust Shape | Notes |
|---|---|---|
| main wordsplit context struct | `struct Wordsplit` | Central owner of parsed words, parameter buffers, flags, and error state. |
| `char **` word vector | `Vec<String>` | Own parsed words directly when the C side owns duplicated strings. If the C code stores slices into an owned buffer, use `Vec<String>` initially for correctness unless field usage proves `Vec<&str>`-like borrowing is safe. |
| owned character buffer for parameters/input expansions | `String` or `Vec<u8>` | Use `String` for text buffers if all content is valid text; use `Vec<u8>` only if field usage shows byte-level ownership. |
| count fields such as `argc`/word count | `usize` | Replace C integer counters with native collection lengths where possible; keep explicit counters only if externally referenced. |
| status/error integer codes | `enum WordsplitError` or `#[repr(i32)] enum` | Preserve code-to-message mapping and allow a distinguished no-error value. |
| optional allocated pointer fields | `Option<T>` | Replace nullable pointers with `Option` around owned buffers or nested structs. |
| flags bitmask | integer newtype or plain bitmask field (`u32`) | Keep representation simple unless current Rust project already uses bitflags. Standard library only is preferred. |

### Error Model

| C Concept | Rust Mapping | Notes |
|---|---|---|
| numeric error constants | `enum WordsplitError` | Include all codes referenced by `strerror`, `perror`, and `clearerr`. |
| “no error” sentinel | `WordsplitError::Ok` or `Option<WordsplitError>` | Prefer explicit enum variant if the C API uses a stable zero code. |
| error string table / switch | `match` returning `&'static str` | No heap allocation needed for standard messages. |

### Memory Management Mapping

| C Pattern | Rust Mapping | Notes |
|---|---|---|
| `free(ptr)` on owned buffers | `Option::take`, `Vec::clear`, `String::clear`, or dropping owned fields | Choose `take` when ownership must be detached; choose `clear` when storage is retained for reuse. |
| module-wide destructor function | `Drop` plus optional explicit `free` compatibility method | Avoid manual double-destruction by centralizing cleanup in owned fields. |
| resetting arrays after free | empty `Vec` / `None` | Ensure post-cleanup state remains valid for repeated method calls if C behavior allows it. |

## Implementation Phases

## Phase 1: Inspect and Define the Rust State Surface

### Goals
- Identify the exact C struct fields touched by the six target functions.
- Define the minimal Rust `Wordsplit` state and error enum needed for these paths.
- Establish file placement in the existing Rust crate matching the current `wordsplit` module.

### Tasks
- Read `src/wordsplit/wordsplit.c` and isolate:
  - fields freed by `wordsplit_free_parambuf`;
  - fields reset by `wordsplit_clearerr`;
  - all resources released by `wordsplit_free`;
  - storage referenced by `wordsplit_get_words`;
  - error codes/messages used by `wordsplit_strerror` and `wordsplit_perror`.
- Create or update the Rust `Wordsplit` struct with only the required fields.
- Create or update `WordsplitError` with explicit variants for the referenced error codes.
- Decide, per field, whether cleanup should drop ownership, clear content, or reset to `None`.

### Deliverables
- Rust struct and enum definitions compiling in the target module.
- A field-level mapping note in code comments for migrated cleanup and error fields.

## Phase 2: Port Cleanup and Accessor Logic

### Goals
- Migrate the direct state-manipulation functions first, since they define ownership and valid post-call state.

### Tasks
- Implement the Rust equivalent of `wordsplit_free_parambuf`.
- Implement the Rust equivalent of `wordsplit_clearerr`.
- Implement the Rust equivalent of `wordsplit_get_words` as a borrowed accessor over stored words.
- Implement `Drop` for `Wordsplit` if required by the final ownership shape.
- Add an explicit `free`-equivalent method only if existing migrated call sites require a named destructor-like operation.

### Technical Notes
- Avoid unsafe code unless existing crate interfaces force raw-pointer compatibility internally.
- Ensure repeated cleanup calls leave the struct in a valid empty state.
- Prefer deriving the word count from `Vec<String>` length instead of maintaining duplicate counters, unless the C layout requires both.

### Deliverables
- Compiling cleanup/accessor implementation.
- Unit tests covering:
  - empty-state cleanup;
  - cleanup after populated buffers/words;
  - repeated cleanup calls;
  - accessor output after population and after reset.

## Phase 3: Port Error String and Reporting Paths

### Goals
- Migrate the error-text conversion and reporting behavior after the error state exists in Rust form.

### Tasks
- Implement `wordsplit_strerror` as:
  - `WordsplitError::as_str()`, or
  - a module-level conversion function if existing code still uses numeric-style lookup.
- Implement `wordsplit_perror` with Rust stderr output preserving the C formatting pattern used by callers.
- Ensure unknown or unmapped codes fall back to a deterministic default message if the C code does so.

### Technical Notes
- Return `&'static str` for standard error text.
- Keep formatting minimal and stable; do not introduce richer diagnostics beyond the C behavior.
- If the original function depends on current object error state plus an optional prefix, preserve that input shape in the Rust method signature as closely as practical.

### Deliverables
- Error conversion and stderr reporting implementation.
- Unit tests covering:
  - known error code to string mapping;
  - cleared error state;
  - fallback/unknown-code handling if present in C;
  - formatted output content using testable writer indirection if needed.

## Phase 4: Integrate and Validate Against Existing Call Flow

### Goals
- Confirm the migrated functions fit the current Rust port branch without expanding module boundaries.

### Tasks
- Wire the new methods/functions into the existing Rust module exports used by adjacent migrated code.
- Remove redundant manual cleanup patterns that Rust ownership now handles automatically, but only where they correspond to the original six functions.
- Run `cargo test` and fix mismatches in state reset behavior, especially around post-free access and error clearing.
- Review for any accidental semantic drift from the C implementation in:
  - ownership lifetime,
  - empty-state representation,
  - error message text,
  - stderr formatting.

### Deliverables
- Final integrated Rust implementation for this module slice.
- Passing targeted tests and crate-wide tests relevant to `wordsplit`.