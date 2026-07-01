# Implementation Plan

## Summary

Port the `quotearg.c` portion of the `main_cluster` into Rust for branch `013-main_root_quotearg_custom_12-rust-port`, limited to the existing custom quoting entry points:

- `quotearg_custom`
- `quotearg_custom_mem`

The Rust implementation should preserve the current module boundary and migrate behavior from the existing C file rather than introducing new abstractions or additional feature surface. The technical approach is to translate the relevant quoting logic into a Rust module that operates on borrowed byte slices and returns owned Rust string/byte results as appropriate for the surrounding ported codebase. Memory ownership previously managed implicitly through C allocation patterns should be made explicit through Rust ownership and borrowing, with fallible boundaries represented using `Result` only where the surrounding API requires it.

## Technical Context

- **Language/Version:** Rust 1.75+
- **Primary Dependencies:** Rust standard library only
- **Testing:** `cargo test`
- **Performance Goals:**
  - Match the C implementation’s asymptotic behavior for custom quoting paths.
  - Avoid unnecessary intermediate allocations where input can be processed in a single pass or with predictable output growth.
  - Operate directly on `&[u8]` for `_mem`-style inputs to avoid UTF-8 assumptions and extra copying.
  - Keep allocation patterns bounded to the produced quoted output, without retaining temporary global or static buffers.

## Module Mapping

| C File | Rust File | Notes |
|---|---|---|
| `quotearg.c` | `src/quotearg.rs` | Port only the logic needed for `quotearg_custom` and `quotearg_custom_mem`. Reuse existing crate-local quoting support if already present on branch; otherwise keep implementation local to this file. |

| C Function | Rust Function | Notes |
|---|---|---|
| `quotearg_custom` | `pub(crate) fn quotearg_custom(...)` | Accept Rust string/byte-compatible inputs while preserving custom left/right quote behavior. |
| `quotearg_custom_mem` | `pub(crate) fn quotearg_custom_mem(...)` | Primary byte-slice entry point; should handle non-NUL-terminated data and explicit lengths by using `&[u8]`. |

## Data Model

The analysis reports only anonymous C data structures and does not identify named structs directly tied to these two functions. The plan should therefore keep data modeling minimal and map only the structures actually required by the migrated functions.

| C Data Structure | Rust Mapping | Notes |
|---|---|---|
| anonymous quoting-related option/state struct(s) referenced by `quotearg.c` | Existing crate-local `struct`/`enum` if already ported; otherwise introduce a private `struct` in `src/quotearg.rs` only if required by these functions | Do not recreate unrelated C layout; model only fields used by `quotearg_custom` and `quotearg_custom_mem`. |
| anonymous pointer/array-based string buffers | `Vec<u8>` / `String` | Use `Vec<u8>` internally for byte-preserving construction; convert to `String` only when the API guarantees textual output. |
| C string pointers (`char *`, `const char *`) | `&str`, `&[u8]`, or owned `String`/`Vec<u8>` | Use `&str` for custom quote delimiters if they are textual; use `&[u8]` for arbitrary input memory. |
| C length-based memory regions | `&[u8]` | Replace pointer+length pairs with slices. |
| C flags / mode values | `enum` or primitive integer type matching existing Rust port | Prefer an existing Rust enum if already available in the port; otherwise keep the narrowest private representation necessary. |

## Implementation Phases

### Phase 1: Establish Rust module and API surface

- Create or update `src/quotearg.rs` as the Rust home for the migrated code from `quotearg.c`.
- Add Rust function signatures for:
  - `quotearg_custom`
  - `quotearg_custom_mem`
- Align signatures with the surrounding Rust port conventions, keeping the mapping close to the C call sites rather than redesigning the interface.
- Identify any already-ported quoting option/state types in the branch and reuse them directly instead of introducing parallel types.
- Decide the internal output representation:
  - use `Vec<u8>` during construction for `_mem` handling
  - expose `String` only if required by existing Rust callers and valid for produced output

### Phase 2: Port core custom quoting behavior

- Translate the implementation logic for `quotearg_custom_mem` first, since it is the length-explicit primitive.
- Implement handling for:
  - explicit left/right custom quote delimiters
  - byte-slice input without assuming NUL termination
  - output assembly with deterministic ownership
- Port `quotearg_custom` as a thin wrapper over `quotearg_custom_mem`, converting the input form used by C string-based callers into the Rust slice/string form.
- Remove C-style implicit memory lifetime assumptions by returning owned output values.
- Keep error handling simple and local:
  - no recovery mechanisms
  - no global mutable buffers
  - no unsafe code unless required by surrounding existing interfaces

### Phase 3: Integrate with existing module usage and edge cases

- Update existing call sites in the Rust port that correspond to the original `quotearg.c` consumers, but only where needed for these two functions.
- Verify behavior for edge cases implied by the C API shape:
  - empty input
  - empty custom delimiters
  - non-UTF-8 input for `_mem`
  - embedded NUL bytes in memory-based input
- Ensure the wrapper and primitive maintain consistent output semantics and ownership expectations.
- Confirm no accidental expansion into unrelated quoting modes or helper APIs beyond what these two functions require.

### Phase 4: Testing and cleanup

- Add focused unit tests in the same module or crate test layout for:
  - basic custom quoting output
  - `_mem` behavior with explicit lengths
  - embedded NUL and non-UTF-8 byte inputs
  - empty delimiters and empty source input
  - wrapper equivalence between `quotearg_custom` and `quotearg_custom_mem` for textual inputs
- Run `cargo test` and fix any behavioral mismatches against the original C expectations.
- Remove dead transitional code and keep the module restricted to the migrated functionality only.