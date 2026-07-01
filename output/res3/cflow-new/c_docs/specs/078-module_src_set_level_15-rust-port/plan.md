# Implementation Plan: module_src_set_level_15

## Summary

This module port covers the C logic associated with `set_level_indent` and `set_level_mark`, currently located across `src/main.c` and `src/output.c`. The Rust implementation should preserve the existing behavior for computing or applying output level formatting state, with emphasis on direct migration of the current control flow and data access patterns rather than redesign.

The technical approach is to:
- translate the relevant file-local and shared state used by these functions into Rust data structures with explicit ownership,
- move string and buffer handling from raw C pointers into `String`, `&str`, and byte slices where appropriate,
- preserve the original function boundaries as much as practical so the Rust port remains traceable to the C sources,
- keep formatting/state mutation logic localized in a small Rust module cluster corresponding to the original C source split.

Because the available analysis shows only anonymous C data structures, the Rust plan should initially mirror the minimum needed state used by these two functions and refine types during migration as the exact field usage is extracted from the C code. The port should avoid adding new abstractions unless required by Rust’s ownership and borrowing rules.

## Technical Context

- **Language/Version**: Rust 1.78 or newer
- **Primary Dependencies**:
  - Rust standard library only
- **Testing**:
  - `cargo test`
- **Performance Goals**:
  - Maintain behaviorally equivalent per-call cost to the C implementation
  - Avoid unnecessary heap allocation in hot formatting paths
  - Prefer borrowed string slices and in-place mutation where the C code currently mutates existing state
  - Keep output formatting logic linear in the length of processed text or level markers

## Module Mapping

### C to Rust File Mapping

| C File | Rust File | Notes |
|---|---|---|
| `src/main.c` | `src/main.rs` or `src/lib.rs` + `src/main.rs` | Keep only the migrated entry/use sites for `set_level_indent` if this file currently owns global/module state or invocation flow. |
| `src/output.c` | `src/output.rs` | Primary target for formatting-related logic, including `set_level_mark` and any shared helpers directly used by these functions. |

### Function Mapping

| C Function | Rust Function | Notes |
|---|---|---|
| `set_level_indent` | `set_level_indent` | Preserve parameter order/meaning where practical; convert pointer-based outputs into mutable references or return values. |
| `set_level_mark` | `set_level_mark` | Preserve mutation semantics; use `&mut` access to formatting/output state instead of raw pointer updates. |

### Suggested Rust Module Layout

A restrained layout matching the existing C split:

```text
src/
  main.rs
  output.rs
```

If the project already uses a library crate structure, the equivalent restrained form is:

```text
src/
  lib.rs
  output.rs
  main.rs
```

No additional modules should be introduced unless required to break a compile-time cycle caused by shared state.

## Data Model

The input analysis lists only anonymous C structures. The Rust port should therefore derive concrete types from actual field usage in `set_level_indent` and `set_level_mark`, keeping the model narrow and migration-focused.

### Mapping Principles

| C Pattern | Rust Mapping | Notes |
|---|---|---|
| Anonymous `struct` used as persistent module state | Named `struct` | Name from role observed in code, such as output state, formatting state, or level state. |
| Anonymous `struct` used only within one function | Local named `struct` or direct tuple/local variables | Prefer locals if the state is short-lived. |
| `char *` mutable string buffer | `String` or `Vec<u8>` | Use `String` if text semantics are required; `Vec<u8>` only if the C logic is byte-oriented. |
| `const char *` input | `&str` or `&[u8]` | Choose `&str` when valid text is assumed by the original formatting logic. |
| Integer flags | `bool` or small integer types | Convert to `bool` when clearly binary; otherwise preserve width conservatively. |
| Output parameters via pointers | Return value and/or `&mut` parameters | Match original mutation behavior without raw pointers. |
| Nullable pointers | `Option<T>` / `Option<&T>` / `Option<&mut T>` | Use explicit optionality instead of sentinel nulls. |

### Initial Rust Structures

Because the exact C fields are not provided, start with minimal named structures extracted from actual access sites during porting. The expected shape is:

```rust
pub struct OutputState {
    // fields migrated from shared output-related anonymous structs
}

pub struct LevelFormatState {
    // fields used by set_level_indent / set_level_mark
}
```

If one or both functions operate directly on broader program state from `main.c`, fold only the required fields into a dedicated borrowed view or a narrower state struct rather than exposing unrelated program-wide data.

### Memory Management Decisions

- Replace raw ownership of dynamically allocated strings with owned `String`.
- Replace transient pointer arithmetic with slice indexing or iterator-based traversal.
- Use `&mut` borrows for in-place state updates instead of aliasable mutable pointers.
- Avoid `unsafe` unless the existing code depends on representation or buffer operations that cannot be expressed safely; if needed, isolate it to the smallest possible block and document the C equivalence.

### Error Handling Decisions

If the original C functions are side-effect-only and do not report failure, keep Rust signatures infallible.

If migration reveals failure cases such as:
- invalid level values,
- malformed input text,
- impossible buffer/state assumptions previously unchecked in C,

then use a small module-local error enum or `Result<(), Error>` only for the affected functions. Do not introduce a broader error framework.

## Implementation Phases

### Phase 1: Extract and Map Existing State

- Inspect `set_level_indent` and `set_level_mark` in `src/main.c` and `src/output.c`.
- Identify all anonymous structs, globals, and helper functions directly touched by these two functions.
- Define the minimum Rust named structs required to represent:
  - level-related formatting state,
  - output-related mutable state,
  - any configuration flags directly read by these functions.
- Decide per field whether it maps to:
  - `String`,
  - `&str`,
  - integer scalar,
  - `bool`,
  - `Option<_>`.
- Establish the Rust file placement matching the C source ownership.

### Phase 2: Port Core Logic Function-for-Function

- Implement `set_level_indent` in Rust with control flow closely matching the C version.
- Implement `set_level_mark` in Rust with the same mutation order and formatting behavior as the C code.
- Port only the helper logic directly required by these functions; do not generalize shared formatting utilities beyond what the C code already uses.
- Replace pointer writes and buffer mutations with:
  - returned values where the C function computes text/state,
  - `&mut` references where the C function updates caller-owned state.
- Keep naming close to the C source to simplify review against the original implementation.

### Phase 3: Integrate With Calling Code

- Update the Rust equivalents of the call sites from `src/main.c` and `src/output.c`.
- Ensure shared state lifetimes and mutable borrowing align with original usage order.
- Resolve any global-state patterns by passing explicit mutable references into `set_level_indent` and `set_level_mark`.
- Remove any remaining placeholder C-style assumptions such as null-based branching where Rust types can express the state directly.

### Phase 4: Validate Behavior With Focused Tests

- Add unit tests covering the observed behavior of:
  - indentation level updates,
  - level mark selection or formatting,
  - edge cases around empty strings, zero/default levels, and repeated updates if present in the C logic.
- Where practical, derive expected outputs from the current C behavior for representative inputs.
- Run `cargo test` and confirm no unnecessary allocations or ownership workarounds were introduced in the formatting path.