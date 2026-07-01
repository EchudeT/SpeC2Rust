# Implementation Plan

## Summary

This plan covers the Rust port of the `quotearg.c` portion used by `main_root_quotearg_custom_13`, limited to migrating the existing `quotearg_custom` and `quotearg_custom_mem` functionality into the Rust branch `013-main_root_quotearg_custom_13-rust-port`.

The Rust implementation should preserve the current module boundary and behavior shape rather than redesigning quoting facilities. The technical approach is to translate the relevant custom-quoting logic into a Rust module that operates on borrowed byte/string inputs, validates the custom quote delimiters required by the original API contract, and returns owned Rust output instead of relying on C-style static or caller-managed buffers. Any behavior that depends on invalid inputs should be represented explicitly through Rust result types or internal assertions, depending on whether the C path is operationally recoverable.

The port should stay narrowly scoped to:
- the custom quoting entry points,
- their immediate supporting data representations already implied by `quotearg.c`,
- tests that confirm parity for normal and edge-case quoting behavior.

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve linear-time processing relative to input length.
  - Avoid unnecessary intermediate allocations beyond the final quoted output.
  - Keep memory ownership explicit and local to each call, with no global mutable buffers.
  - Match C behavior closely enough that command-line observable output remains unchanged for covered cases.

## Module Mapping

| C File | C Functions | Rust Target |
|---|---|---|
| `quotearg.c` | `quotearg_custom` | `src/quotearg.rs` -> `pub fn quotearg_custom(...)` |
| `quotearg.c` | `quotearg_custom_mem` | `src/quotearg.rs` -> `pub fn quotearg_custom_mem(...)` |

### Rust module placement

The migration should remain concentrated in a single Rust source file corresponding to the original responsibility:

- `src/quotearg.rs`

If the crate already has an existing module for quoting, these functions should be added there instead of introducing a parallel abstraction.

## Data Model

The analysis lists only anonymous C data structures, with no named struct contract exposed for this module slice. The plan therefore keeps data-structure migration minimal and only introduces Rust types where needed to support the two target functions.

| C Representation | Rust Mapping | Notes |
|---|---|---|
| anonymous internal quoting-related structs | private Rust `struct` only if already required by existing `quotearg` ported code | Do not invent new public model types unless necessary for compiling migrated functions. |
| C string pointer inputs (`char *`, `const char *`) | `&str` when UTF-8 text is required by surrounding Rust code, otherwise `&[u8]` | Prefer `&[u8]` for `_mem` variants because the C API is length-based and may not assume UTF-8. |
| C length argument (`size_t`) | `usize` | Direct mapping. |
| returned quoted C string buffer | `String` or `Vec<u8>` internally, exposed as `String` if semantics are text-only | Choose the narrowest type consistent with existing Rust port conventions. |
| nullable/invalid delimiter pointers | `Result<_, QuoteArgError>` or explicit panic/assert for impossible internal misuse | Use explicit handling instead of undefined behavior. |

### Expected Rust type direction

To preserve the distinction between the two original entry points:

- `quotearg_custom_mem`: should accept byte-oriented input plus explicit length semantics.
- `quotearg_custom`: should delegate to the `_mem` form using full input length.

If custom opening and closing quote markers are represented independently in C, mirror that with borrowed parameters rather than introducing configuration builders or broader option objects.

## Implementation Phases

### Phase 1: Isolate and map existing quoting behavior

- Inspect `quotearg.c` and identify the exact logic path exercised by `quotearg_custom` and `quotearg_custom_mem`.
- Determine which anonymous internal structures are actually touched by these functions.
- Establish the smallest Rust signatures that preserve the original call semantics.
- Decide per parameter whether the Rust form should be `&str` or `&[u8]`, with bias toward `&[u8]` for memory-length-based processing.
- Record any C preconditions on custom left/right quote arguments so they can be enforced explicitly in Rust.

**Exit criteria**:
- Final Rust signatures selected.
- Required helper state from `quotearg.c` identified.
- No extra quoting modes or unrelated `quotearg` functionality included in scope.

### Phase 2: Port core implementation into `src/quotearg.rs`

- Implement `quotearg_custom_mem` first as the core routine.
- Translate the C logic directly, preserving escaping and delimiter placement behavior.
- Replace pointer arithmetic with slice indexing/iteration.
- Replace C-managed output buffers with owned Rust output assembly.
- Implement `quotearg_custom` as a thin wrapper over the `_mem` implementation.
- Keep helper functions private and local to the module.
- Use explicit error handling for invalid custom quote delimiters if the C logic relies on non-null/non-empty inputs.

**Memory and error-handling rules**:
- No global mutable storage.
- No unsafe code unless a surrounding existing port already requires it; if unavoidable, confine it to the smallest possible boundary.
- Avoid lossy conversion from bytes to UTF-8 unless the original behavior is text-only and validated by the caller.
- If output must remain textual but input may be arbitrary bytes, preserve escaping logic rather than forcing UTF-8 interpretation.

**Exit criteria**:
- Both target functions compile in Rust.
- Implementation stays within the existing quoting module boundary.
- Behaviorally relevant helper logic is migrated without adding new APIs.

### Phase 3: Add focused parity tests

- Add unit tests covering:
  - basic custom left/right quoting,
  - empty input,
  - embedded characters requiring quoting/escaping under the original logic,
  - `_mem` handling of explicit lengths,
  - wrapper equivalence between `quotearg_custom` and `quotearg_custom_mem`.
- Include byte-oriented tests if non-UTF-8 or raw byte handling is part of the C behavior.
- Verify that invalid delimiter cases follow the chosen Rust contract consistently.

**Exit criteria**:
- `cargo test` passes.
- Tests cover normal path and key edge conditions for the two migrated functions only.

### Phase 4: Integrate and trim

- Replace any remaining call sites in the Rust branch that still depend on placeholder or unported custom quoting behavior.
- Remove temporary translation scaffolding introduced during the port.
- Confirm the final module uses standard Rust ownership and does not retain C-style lifetime assumptions.
- Review for unnecessary abstractions and reduce to the smallest maintainable translation.

**Exit criteria**:
- The branch uses the Rust implementation for this module slice.
- No extra compatibility layers remain beyond what the existing crate structure requires.
- Scope remains restricted to the migrated file/functions.

## Notes and Constraints

- Keep the port aligned with the original `quotearg.c` behavior rather than introducing a generalized quoting framework.
- Do not create additional modules unless the current crate layout already requires one.
- Anonymous C structures should only be modeled in Rust if directly needed by these functions; otherwise keep their influence implicit in local logic.
- Favor standard library facilities for string/byte handling and allocation.
- The migration target is behavioral parity for the specified functions, not broader cleanup of the entire quoting subsystem.