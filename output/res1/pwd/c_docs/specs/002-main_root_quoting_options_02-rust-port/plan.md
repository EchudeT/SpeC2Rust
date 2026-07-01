# Implementation Plan

## Summary

This module ports the `quotearg.c` functionality needed by `quotearg_n_custom_mem` into Rust for the `pwd` project branch `002-main_root_quoting_options_02-rust-port`.

The Rust implementation should focus narrowly on reproducing the existing quoting behavior required by this function, without broadening the surrounding quoting subsystem. The preferred approach is to translate the relevant logic from `quotearg.c` into a Rust module that operates on byte slices (`&[u8]`) and returns owned output buffers (`String` or `Vec<u8>` depending on the exact escaping needs). Since the source function name indicates custom quoting over arbitrary memory, the implementation should preserve byte-oriented handling first and only convert to UTF-8 text where the original behavior guarantees textual output.

Memory ownership should be made explicit through Rust values instead of C-style shared/static buffers. Any C implicit global or reusable state used by this path should be migrated into local function state or a small Rust options type limited to what `quotearg_n_custom_mem` actually consumes.

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Match the C path’s asymptotic behavior for scanning and escaping input buffers.
  - Avoid unnecessary intermediate allocations where a single output buffer can be sized and extended incrementally.
  - Preserve byte-exact output for covered inputs.
  - Keep per-call allocation bounded to the produced quoted result and minimal temporary state.

## Module Mapping

| C Source File | C Function | Rust Module/File | Rust Item |
|---|---|---|---|
| `quotearg.c` | `quotearg_n_custom_mem` | `src/main_root_quoting_options_02.rs` or the nearest existing quoting-focused module in `src/` | `pub(crate) fn quotearg_n_custom_mem(...)` |
| `quotearg.c` | internal helper logic used only by this function | same Rust file | private helper functions only as needed to mirror the C control flow |

### Mapping Notes

- Keep the port scoped to the code path required by `quotearg_n_custom_mem`.
- Do not create a general-purpose quoting framework unless the existing Rust project already has the exact target location for this code.
- If the current Rust branch already contains partial `quotearg` migration, place this function beside the existing ported quoting code rather than creating a new abstraction layer.

## Data Model

The analysis only exposes anonymous C data structures, so the Rust plan should avoid inventing broad replacements. Data structures should be introduced only when directly required by the translated function.

| C Data Structure | Rust Mapping | Notes |
|---|---|---|
| anonymous struct/union used by quoting options/state | private Rust `struct` with named fields | Create only if `quotearg_n_custom_mem` reads multiple related fields repeatedly. |
| anonymous flag/config container | private Rust `struct` or small `enum` set | Replace bitfields/macros with explicit Rust fields or enums. |
| raw memory input (`char *` + length) | `&[u8]` | Preferred representation for arbitrary memory. |
| returned quoted buffer | `String` or `Vec<u8>` | Use `String` only if output is guaranteed valid UTF-8; otherwise produce bytes and convert at API boundary if necessary. |
| nullable custom quote pointers | `Option<&[u8]>` or `Option<u8>`/`Option<Vec<u8>>` depending on actual C usage | Preserve optionality explicitly. |
| integer option/index parameters | `usize` / `u32` / `i32` as appropriate | Use checked conversions when translating from C integer widths. |

### Memory Management Decisions

- Replace C-managed output buffers and reused static storage with Rust-owned return values.
- Avoid borrowing output from temporary state.
- If the C function uses an index `n` to select storage slots, do not reproduce static slot storage unless the wider Rust port already depends on it; instead, return the computed quoted result directly and adapt the caller in the existing migration path.
- Any required temporary accumulation should use `Vec<u8>` with `reserve` when output growth can be estimated.

### Error Handling Decisions

- Prefer infallible behavior if the C function always returns a quoted representation for any byte input.
- If conversion to `String` is needed and output may contain non-UTF-8 bytes, keep the internal representation as `Vec<u8>` and defer UTF-8 assumptions.
- Use `Result` only where the surrounding Rust project API already requires fallible interfaces; do not introduce new error taxonomies for this module alone.

## Implementation Phases

## Phase 1: Isolate the C Scope and Define Rust Signatures

- Identify the exact `quotearg_n_custom_mem` inputs, return behavior, and helper dependencies inside `quotearg.c`.
- Determine whether the function depends on:
  - custom left/right quote delimiters,
  - option/state structs,
  - slot-based storage semantics,
  - locale or character classification logic,
  - helper escape routines.
- Define the minimal Rust function signature that matches current project usage.
- Introduce only the smallest supporting private types needed to represent the C state read by this function.
- Decide the concrete return type (`String` vs `Vec<u8>`) based on whether the generated output is strictly textual in this code path.

**Deliverable**:
- Rust function stub and any minimal private option/state structs in the target source file.

## Phase 2: Port Core Quoting Logic

- Translate the byte-walking and quoting logic from `quotearg_n_custom_mem` into Rust.
- Preserve:
  - input length semantics,
  - handling of arbitrary memory contents,
  - custom quote delimiter behavior,
  - escaping rules and ordering.
- Replace pointer arithmetic with indexed or iterator-based traversal over `&[u8]`.
- Replace mutable C output buffer writes with `Vec<u8>` or `String` accumulation.
- Convert C conditional branches and flag checks into explicit Rust matches/ifs.
- Keep helper functions private and directly traceable to corresponding C logic.

**Deliverable**:
- Working Rust implementation for the main function and any required internal helpers.

## Phase 3: Reconcile State, Ownership, and Call-Site Behavior

- Remove any remaining C assumptions about static buffers, aliasing, or mutable global state from this code path.
- If the original function selected among reusable buffers by index, adapt the Rust call path to use returned owned values instead of persistent slot storage, unless an existing partial port already established equivalent storage.
- Ensure all integer conversions are explicit and safe.
- Confirm that optional custom quoting parameters are represented without null-pointer semantics.
- Keep the implementation contained to the existing module boundary.

**Deliverable**:
- Integrated Rust module with ownership-safe behavior matching the expected caller contract.

## Phase 4: Validate Behavior with Targeted Tests

- Add unit tests under the Rust module or the project’s existing test layout covering:
  - empty input,
  - plain ASCII input,
  - embedded bytes requiring escaping,
  - custom delimiter usage,
  - length-bounded input handling,
  - edge cases around zero bytes or non-printable bytes if the C path supports them.
- Add comparison-style tests derived from observed C behavior for `quotearg_n_custom_mem`.
- Verify no accidental UTF-8 assumptions corrupt byte-exact results.
- Run `cargo test` and fix any output mismatches against the source semantics.

**Deliverable**:
- Passing tests for the migrated function and stable behavior at the Rust call site.