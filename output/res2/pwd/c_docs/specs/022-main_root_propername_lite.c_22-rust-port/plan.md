# Implementation Plan

## Summary
This module ports the C file `propername-lite.c` and its single function `proper_name_lite` into a small Rust module within the `pwd` project branch `022-main_root_propername_lite.c_22-rust-port`.

The Rust implementation should stay narrowly aligned with the existing C behavior: migrate the function logic into idiomatic Rust while preserving observable output and control flow relevant to the caller. Since the input analysis shows no module-local data structures and only one function, the implementation should remain compact and avoid introducing additional abstraction layers.

The main technical approach is:
- translate `proper_name_lite` directly into a Rust function,
- use standard-library string types and borrowed slices in place of C string pointers,
- make ownership and lifetime boundaries explicit,
- represent failure conditions through Rust return types only if the original call pattern requires it; otherwise keep the function total and side-effect compatible.

Memory safety will come from replacing manual C string handling with `&str`, `String`, and explicit conversions at module boundaries. Error handling should be minimal and driven only by the original function’s real behavior, not by speculative redesign.

## Technical Context

- **Language/Version**: Rust stable, edition 2021
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Maintain behavior with no meaningful regression relative to the C implementation for typical short text processing.
  - Avoid unnecessary heap allocation beyond what is required by the original function semantics.
  - Keep the function implementation single-pass where practical, matching the lightweight nature of the source module.

## Module Mapping

| C Source File | C Function | Rust Target |
|---|---|---|
| `propername-lite.c` | `proper_name_lite` | `src/main_root_propername_lite.rs` with `pub(crate)` or private function visibility based on actual caller usage |

### Notes
- Keep the Rust module focused on this file’s contents only.
- If the surrounding port already has a central module file such as `src/lib.rs` or `src/main.rs`, expose the Rust function there only as needed for existing call sites.
- Do not split this migration into submodules unless the current project layout already requires a specific placement.

## Data Model

No explicit C structs or module-local composite data structures were identified for this module.

### C to Rust Type Mapping
The migration is expected to rely on direct scalar and string mappings used by `proper_name_lite`:

| C Concept | Rust Mapping |
|---|---|
| `char *` / `const char *` input text | `&str` when valid UTF-8 is already guaranteed by the Rust caller; otherwise `&[u8]` or `&OsStr` only if required by existing surrounding interfaces |
| newly produced textual result | `String` if owned output is required |
| nullable pointer semantics | `Option<&str>` or `Option<String>` only if null is part of the original contract |

### Memory Management
- Eliminate manual allocation and pointer ownership from the C implementation.
- Prefer borrowed input parameters over owned values unless the original usage requires transfer or construction.
- If the C function conditionally returns allocated text, encode that as an owned Rust `String`.
- If the C code depends on null checks, model them explicitly with `Option` rather than sentinel pointer values.

### Error Handling
- If the original function cannot fail in a meaningful way, use a plain return type.
- If conversion from external byte-oriented data is required at call boundaries, isolate validation there rather than embedding generalized error machinery into this module.
- Avoid introducing custom error enums unless a concrete failure mode from the C behavior must be represented.

## Implementation Phases

### Phase 1: Inspect and map the C function contract
- Review `propername-lite.c` and identify the exact parameter list, return type, and any caller-visible edge cases of `proper_name_lite`.
- Determine whether the C function:
  - returns borrowed versus allocated text,
  - accepts nullable inputs,
  - performs formatting, selection, or fallback logic,
  - depends on locale-sensitive or byte-level operations.
- Choose the narrowest Rust signature that preserves existing semantics for current callers.

### Phase 2: Port `proper_name_lite` into a Rust module
- Create the Rust file corresponding to this module, following the project’s current source layout.
- Translate the function logic directly, preserving decision order and text handling behavior.
- Replace C string manipulation with standard-library string/slice operations.
- Resolve ownership explicitly:
  - borrowed inputs where possible,
  - owned output only where necessary,
  - no raw pointers or manual memory management in normal implementation code.

### Phase 3: Integrate with existing callers
- Wire the Rust function into the surrounding crate structure.
- Update imports/module declarations so the ported function is reachable from the current call path.
- Keep naming close to the source function to simplify traceability during the migration.
- Avoid adding helper layers unless they are necessary to bridge an existing interface mismatch.

### Phase 4: Validate behavior with focused tests
- Add unit tests covering the actual behavior branches of `proper_name_lite`.
- Include edge cases derived from the C implementation, such as:
  - empty input,
  - identical or fallback names if applicable,
  - null-equivalent cases if the original interface permits them,
  - non-ASCII text only if the surrounding Rust interface accepts UTF-8 text.
- Run `cargo test` and confirm the Rust results match the expected outcomes from the C logic.