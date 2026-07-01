# Implementation Plan: module_src_wordsplit_wordsplit_05

## Summary

This plan ports the selected `src/wordsplit/wordsplit.c` responsibilities into Rust on branch `116-module_src_wordsplit_wordsplit_05-rust-port`, limited to the existing function surface:

- `wordsplit_free_parambuf`
- `wordsplit_clearerr`
- `wordsplit_free`
- `wordsplit_get_words`
- `wordsplit_strerror`
- `wordsplit_perror`

The Rust implementation should preserve the current module role: lifecycle cleanup, error-state reset/reporting, and word vector access for an existing word-splitting state object. The port should migrate the current C ownership and state transitions into explicit Rust ownership, borrowing, and `Option`-based resource release, without adding new parsing capabilities or auxiliary subsystems.

The technical approach is to translate the relevant C state container(s) into a Rust state struct that owns its internal buffers and word storage. Cleanup functions become methods or free functions that clear owned allocations deterministically. Error handling should be represented with a narrow internal error enum plus stored error state on the main wordsplit object so that `clearerr`, `strerror`, and `perror` retain their existing operational role.

## Technical Context

### Language / Version

- Rust stable
- Recommended edition: **Rust 2021**
- Minimum practical toolchain target: **rustc 1.74+**

### Primary Dependencies

Use the Rust standard library by default.

Recommended dependencies:
- **None required** for this migration.

Rationale:
- Memory management can be handled with `Vec`, `String`, `Box`, and `Option`.
- Error text formatting can use `std::fmt` and `std::io`.
- stderr reporting can use `eprintln!` or `std::io::stderr()`.

### Testing

- `cargo test`

Testing focus:
- cleanup functions leave state empty and reusable where applicable
- repeated cleanup is harmless
- `get_words` exposes the stored words consistently
- `clearerr` removes prior error state
- `strerror` returns stable text for known error states
- `perror` writes expected prefixed/unprefixed output behavior

### Performance Goals

- Preserve linear-time behavior for word list extraction/access.
- Avoid unnecessary copies when exposing stored words internally; if API shape requires returned ownership, keep copying limited to current C-equivalent behavior.
- Cleanup operations should remain proportional to currently owned allocations.
- No additional heap structures beyond what is required to represent the existing C state safely.

## Module Mapping

### C to Rust File Mapping

Source being migrated:
- `src/wordsplit/wordsplit.c`

Rust destination:
- `src/wordsplit.rs`

If the repository already uses a nested module layout for `wordsplit`, keep the implementation in the existing corresponding file instead of introducing extra modules. The goal is direct migration of the current file responsibilities, not a broader reorganization.

### Function Mapping

| C Function | Rust Mapping | Notes |
|---|---|---|
| `wordsplit_free_parambuf` | `fn free_parambuf(state: &mut WordSplit)` or `impl WordSplit { fn free_parambuf(&mut self) }` | Clears parameter-buffer-owned fields only. |
| `wordsplit_clearerr` | `fn clearerr(state: &mut WordSplit)` or method | Resets stored error code/message context. |
| `wordsplit_free` | `fn free(state: &mut WordSplit)` or `impl Drop` plus explicit clear method | Explicitly releases all owned buffers and word storage; retain explicit callable cleanup to mirror C behavior. |
| `wordsplit_get_words` | `fn get_words(state: &WordSplit) -> &[String]` or equivalent borrowed view | Prefer borrowed slice if compatible with surrounding Rust port; if external compatibility demands owned form, keep conversion local. |
| `wordsplit_strerror` | `fn strerror(err: WordSplitError) -> &'static str` and/or `impl Display` | Backed by internal error enum or stored error state mapping. |
| `wordsplit_perror` | `fn perror(prefix: Option<&str>, state: &WordSplit)` | Emits formatted error text to stderr. |

## Data Model

The input analysis reports only anonymous C data structures, so the plan should infer a restrained Rust mapping centered on the state required by the listed functions rather than inventing additional abstractions.

### Core Mapping Strategy

| C Construct | Rust Mapping | Migration Notes |
|---|---|---|
| main wordsplit state struct | `struct WordSplit` | Central owner of words, parameter buffer(s), and error state. |
| `char **` word vector | `Vec<String>` | Safe owned storage for split words. |
| `char *` mutable text buffer | `String` or `Vec<u8>` | Use `String` if content is textual and UTF-8 validated elsewhere in the port; use `Vec<u8>` only if exact byte preservation is required by surrounding code. |
| optional owned pointer field | `Option<T>` | Represents nullable ownership from C. |
| numeric error code / enum-like constants | `enum WordSplitError` | Include only codes needed by this module’s existing behavior. |
| stderr print path | `std::io::stderr()` or `eprintln!` | No logging framework needed. |
| manual free/reset patterns | `clear()`, `take()`, field reassignment | Replaces `free()` and nulling in C. |

### Proposed Rust Structures

The exact fields should be derived from the current `wordsplit.c` usage, but this module should converge on a restrained shape similar to:

```rust
pub struct WordSplit {
    words: Vec<String>,
    parambuf: Option<String>,
    error: Option<WordSplitError>,
    error_detail: Option<String>,
}
```

And:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WordSplitError {
    // Variants added only as required by the migrated C error mapping
}
```

### Memory Management Decisions

- Replace explicit deallocation with ownership-driven cleanup.
- Keep explicit cleanup entry points because the C API surface includes them and other migrated code may call them directly.
- Use `Option::take()` to model “free then null”.
- Ensure `wordsplit_free` clears all subordinate buffers before resetting top-level collections, matching C observable postconditions as closely as needed by callers.
- Avoid unsafe code unless the wider port already requires raw-pointer compatibility at this boundary; for this module alone, safe Rust should be sufficient.

### Error Handling Decisions

- Store the current error state on `WordSplit` rather than relying on global state.
- `wordsplit_clearerr` should reset both code and any attached detail string.
- `wordsplit_strerror` should map known error values to fixed strings.
- `wordsplit_perror` should combine optional prefix + error string + optional detail in a deterministic format mirroring current C behavior as closely as practical.

## Implementation Phases

## Phase 1: State and Type Migration

Scope:
- Inspect `src/wordsplit/wordsplit.c` for the exact fields touched by the six target functions.
- Define the Rust `WordSplit` state with only the fields necessary for these functions.
- Define the internal `WordSplitError` representation and string mapping table.

Deliverables:
- Rust struct/enum definitions in the destination module.
- Initial constructors/defaults as required by existing call sites.
- Unit tests for default empty state and error-string mapping.

Completion criteria:
- All data owned by the targeted C functions has a direct Rust field mapping.
- No placeholder heap ownership remains expressed as raw pointers unless forced by existing surrounding code.

## Phase 2: Cleanup and Error-State Port

Scope:
- Port `wordsplit_free_parambuf`
- Port `wordsplit_clearerr`
- Port `wordsplit_free`

Implementation notes:
- Preserve call ordering semantics where visible in C.
- Make repeated calls idempotent by using `clear`, `truncate(0)`, and `Option::take()`.
- If the C function distinguishes partial cleanup from full cleanup, keep that distinction explicit in separate Rust methods/functions.

Deliverables:
- Working cleanup functions/methods.
- Tests covering:
  - partial buffer cleanup
  - full object cleanup
  - repeated cleanup calls
  - cleanup after prior error state

Completion criteria:
- Cleanup functions remove owned data without panics.
- Post-cleanup state is empty/reset in a way consistent with expected downstream reuse or disposal.

## Phase 3: Word Access and Error Reporting Port

Scope:
- Port `wordsplit_get_words`
- Port `wordsplit_strerror`
- Port `wordsplit_perror`

Implementation notes:
- Prefer returning a borrowed slice for internal Rust use.
- If existing migrated interfaces require count-plus-pointer style behavior, adapt at the boundary but keep internal storage idiomatic.
- Keep `perror` formatting minimal and deterministic, using stderr only.

Deliverables:
- Accessor and reporting functions.
- Tests covering:
  - non-empty and empty word access
  - all supported error-to-string mappings
  - stderr output shape for `perror` with and without prefix/detail

Completion criteria:
- Accessor behavior is stable and consistent with stored state.
- Error reporting reflects current error state accurately.

## Phase 4: Integration Verification

Scope:
- Align names, visibility, and signatures with the surrounding Rust port conventions already present on the branch.
- Replace any remaining C-style null/reset assumptions in call sites touching this module.
- Run full module tests with `cargo test`.

Deliverables:
- Finalized Rust module wired into the existing project structure.
- Adjusted tests for any dependent call sites already ported.

Completion criteria:
- The targeted C functions are fully represented in Rust.
- No extra module layers or unevidenced facilities were introduced.
- `cargo test` passes for the migrated module and its immediate integrations.