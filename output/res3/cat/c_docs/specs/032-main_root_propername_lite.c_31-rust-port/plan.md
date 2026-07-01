# Implementation Plan

## Summary

Port `propername-lite.c` into an idiomatic Rust module that preserves the existing behavior of `proper_name_lite` without widening scope. The Rust implementation should stay close to the original control flow and string-handling semantics, using standard-library string types and borrowed data where possible to minimize allocations.

The technical approach is a direct function-level migration:
- translate `proper_name_lite` into a Rust function in a single corresponding module,
- represent C string inputs with `&str` or byte/string references as appropriate to the observed call pattern during migration,
- return owned or borrowed Rust string data based on the original function’s lifetime expectations,
- replace manual C memory handling with Rust ownership and borrowing,
- express any failure cases through explicit return types only if the original function has observable error outcomes; otherwise keep the API simple and deterministic.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Match the C implementation’s asymptotic behavior.
  - Avoid unnecessary heap allocation when the result can be borrowed or constructed once.
  - Preserve lightweight execution suitable for command-line utility startup paths.
  - Keep string processing linear in input length.

## Module Mapping

| C File | C Function | Rust Module | Rust Item |
|---|---|---|---|
| `propername-lite.c` | `proper_name_lite` | `src/propername_lite.rs` | `pub(crate) fn proper_name_lite(...)` |

### Rust project placement

- Add a single Rust source file for this migration unit:
  - `src/propername_lite.rs`
- Expose it from the existing crate root only as needed by current call sites:
  - `mod propername_lite;`
  - or `pub(crate) mod propername_lite;`

This plan keeps the port limited to the original file/function boundary and does not introduce extra abstraction layers.

## Data Model

No named C structs are listed for this module, so no struct-to-struct migration is required.

### Function-level data mapping

| C Concept | Rust Mapping | Notes |
|---|---|---|
| C string input (`char *` / `const char *`) | `&str` when valid UTF-8 is already guaranteed by surrounding code; otherwise `&[u8]` or `&CStr` at the boundary, converting internally as needed | Choose the narrowest safe mapping based on actual caller data during implementation |
| Returned string data | `String` or `Cow<'a, str>` | Prefer `Cow<'a, str>` if the function sometimes returns input unchanged and sometimes constructs output; otherwise use `String` for a simpler direct port |
| Nullability from C | `Option<&str>` / `Option<&CStr>` only if null is part of existing behavior | Do not introduce `Option` unless null is a real input case in the original function |

### Memory management considerations

- Eliminate manual allocation/free behavior from C and rely on Rust ownership.
- If the C function may return either an existing string or a synthesized representation, model this with `Cow<'a, str>` to preserve efficiency without unsafe memory handling.
- Avoid `unsafe` unless the surrounding port requires direct interaction with raw C-compatible inputs.

### Error handling considerations

- If the original function does not report errors separately, keep the Rust function non-fallible.
- If invalid text decoding must be handled at the boundary, isolate that at the caller/module edge rather than spreading decoding logic through the implementation.
- Do not add custom error hierarchies for this single-function migration.

## Implementation Phases

### Phase 1: Establish module skeleton and API boundary

- Create `src/propername_lite.rs`.
- Add the Rust signature for `proper_name_lite` based on current caller expectations.
- Decide the input/return string types from actual usage in the existing port branch:
  - prefer `&str` for inputs already represented as Rust strings,
  - use `Cow<'_, str>` only if it clearly avoids extra allocation while preserving behavior,
  - otherwise use `String` for a straightforward port.
- Wire the module into the crate with the minimum visibility needed.

### Phase 2: Port `proper_name_lite` logic directly

- Translate the C control flow into Rust with behavior-preserving branching.
- Replace C string comparisons and concatenation with standard-library operations.
- Preserve original formatting and selection behavior exactly, especially around:
  - input inspection,
  - fallback naming behavior,
  - output composition.
- Remove any manual lifetime assumptions present in C by returning owned data where borrowing would be ambiguous.

### Phase 3: Validate edge cases and memory semantics

- Add unit tests covering the function’s observed behavior from the original implementation and known edge cases:
  - unchanged input path,
  - transformed/output-composed path,
  - empty-string handling if applicable,
  - null-equivalent handling only if present in C callers.
- Confirm there are no hidden allocation/lifetime issues from the translated return type.
- Check that the final API remains internal and limited to the original migration scope.

### Phase 4: Integrate and clean up call sites

- Update current Rust call sites in this branch to use the new module/function.
- Remove any temporary compatibility code created during the port.
- Run `cargo test` and fix any signature mismatches or borrow/ownership issues without expanding module responsibilities.