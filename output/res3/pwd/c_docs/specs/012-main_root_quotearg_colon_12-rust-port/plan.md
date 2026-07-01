# Implementation Plan

## Summary

Port the `quotearg.c` functionality for `quotearg_colon` and `quotearg_colon_mem` into Rust with a narrow, file-oriented migration that preserves current behavior and call structure rather than redesigning the quoting subsystem.

The Rust implementation should:
- replicate the colon-specific quoting behavior exposed by the two listed functions,
- keep the implementation localized to a single Rust module corresponding to the C source,
- use borrowed byte slices for `_mem`-style input and `&str`/owned `String` wrappers only where the C API implies string-oriented entry points,
- avoid introducing broader quoting abstractions unless they are strictly required to express the existing logic already present in `quotearg.c`.

Technical approach:
- migrate the two functions in dependency order, implementing `quotearg_colon_mem` as the core byte-oriented routine and `quotearg_colon` as the string/NUL-terminated convenience wrapper,
- represent output with `String` when the result is textual and valid UTF-8 is guaranteed by construction; otherwise use a byte buffer internally and convert at the boundary only if the existing behavior is textual,
- model any C internal option/state structures only if directly needed by these functions, keeping visibility private to the module,
- replace C memory ownership patterns with Rust ownership and borrowing, eliminating manual allocation/free paths.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - match the C implementation’s asymptotic behavior for input scanning and output construction,
  - avoid unnecessary intermediate allocations,
  - perform a single pass over input bytes where practical,
  - preserve efficient handling of arbitrary byte slices for the `_mem` variant.

## Module Mapping

| C File | Rust File | Notes |
|---|---|---|
| `quotearg.c` | `src/quotearg.rs` | Migrate only the logic required for `quotearg_colon` and `quotearg_colon_mem`. Keep helper routines private unless existing project usage requires broader visibility. |

| C Function | Rust Function | Notes |
|---|---|---|
| `quotearg_colon` | `pub fn quotearg_colon(input: &str) -> String` | Thin wrapper over the byte-oriented implementation, using the Rust string slice length instead of NUL-terminated scanning. |
| `quotearg_colon_mem` | `pub fn quotearg_colon_mem(input: &[u8]) -> String` | Core implementation for colon-aware quoting over explicit-length input. Internal byte-buffer construction is acceptable. |

If the surrounding Rust port already uses a different public signature convention, keep names and visibility aligned with the existing port, but do not add extra API surface beyond these two functions.

## Data Model

The analysis lists only anonymous C data structures and does not identify named structs required as stable public types for this module. The migration plan should therefore avoid inventing exported Rust data types and instead map only the minimal internal state needed by the two functions.

| C Data Structure | Rust Mapping | Usage |
|---|---|---|
| anonymous internal option/state structs in `quotearg.c` | Private `struct` or `enum` only if referenced by migrated helper logic | Introduce only when unavoidable to preserve existing control flow. |
| C string pointer + length pairs | `&[u8]` | Primary representation for `_mem` input. |
| C NUL-terminated string pointers | `&str` | Wrapper input for `quotearg_colon`. |
| C heap-allocated output buffers | `String` or `Vec<u8>` internally | Prefer `String` if output is strictly textual; otherwise build in `Vec<u8>` and convert once. |

Implementation notes for memory and error handling:
- Rust ownership replaces C allocation lifetimes; returned values own their buffers.
- No explicit free logic is needed.
- If byte processing cannot be expressed as valid UTF-8 at intermediate stages, use `Vec<u8>` internally.
- Since the C functions are not described as fallible, prefer total functions returning `String`; reserve `Result` only if the surrounding port already establishes a fallible contract.

## Implementation Phases

### Phase 1: Establish module skeleton and input/output contracts

- Create or update `src/quotearg.rs`.
- Define Rust signatures for:
  - `quotearg_colon_mem`
  - `quotearg_colon`
- Identify the exact helper logic in `quotearg.c` that these functions depend on and copy only that dependency chain into the same Rust file.
- Decide the internal output buffer type based on the migrated logic:
  - `String` directly if all emitted content is valid UTF-8 text,
  - otherwise `Vec<u8>` with a final safe conversion point.
- Keep all helper items private unless required elsewhere in the current Rust branch.

**Exit criteria**
- Module compiles with function stubs and selected internal helpers wired into the Rust file layout.
- Public API surface for this port is limited to the two required functions.

### Phase 2: Port core quoting logic for `quotearg_colon_mem`

- Implement the byte-slice-based quoting path first, since it reflects the C explicit-length entry point most directly.
- Translate character inspection and escaping logic from C into byte-oriented Rust iteration.
- Preserve behavior around colon handling and any adjacent quoting rules used by this entry point.
- Replace manual buffer growth with `String::with_capacity` or `Vec::with_capacity` based on expected output form.
- Keep logic branch structure close to the C source to reduce migration risk.

**Memory/error handling focus**
- Avoid indexing patterns that can panic unexpectedly; prefer iterator-based traversal or carefully bounded indexing.
- Ensure no assumptions of NUL termination for `_mem` input.
- Avoid lossy UTF-8 conversion during byte processing.

**Exit criteria**
- `quotearg_colon_mem` is fully implemented.
- Unit tests cover empty input, colon-containing input, and representative non-colon byte content.

### Phase 3: Implement `quotearg_colon` as wrapper and align semantics

- Implement `quotearg_colon` as the string-oriented wrapper over `quotearg_colon_mem`.
- Use `input.as_bytes()` and preserve the same quoting result as the C entry point for equivalent text input.
- Verify that wrapper behavior does not add or remove processing relative to the core function.
- Remove any temporary duplication introduced during initial porting.

**Exit criteria**
- Both functions share one core quoting path.
- Wrapper tests confirm parity with direct `_mem` invocation on equivalent byte content.

### Phase 4: Validation, cleanup, and parity review

- Add focused `cargo test` coverage for:
  - empty input,
  - strings without special handling,
  - strings containing one or multiple colons,
  - edge-length cases,
  - byte-oriented `_mem` inputs that exercise explicit-length behavior.
- Review helper visibility and remove any unused translated artifacts from `quotearg.c`.
- Compare Rust control flow and emitted output against the C implementation for the migrated functions only.
- Run formatting and standard test suite checks.

**Exit criteria**
- `cargo test` passes.
- The Rust module contains only the code necessary to support `quotearg_colon` and `quotearg_colon_mem`.
- No extra quoting facilities or generalized abstractions have been added beyond what the migrated functions require.