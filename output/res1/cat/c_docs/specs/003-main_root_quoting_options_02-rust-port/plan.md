# Implementation Plan

## Summary

Port the `quotearg.c` functionality used by `quotearg_n_custom_mem` into a Rust module that preserves the current quoting behavior and call patterns required by the `cat` main-cluster work. The implementation should stay narrow: migrate only the logic and data representations needed for this function and the supporting quoting option state it touches, without introducing broader quoting abstractions beyond what is required for compatibility.

The Rust approach should:
- translate the C quoting-option handling into explicit Rust structs/enums,
- replace raw memory and pointer arithmetic with slice-based and `Vec<u8>`/`String`-backed processing where possible,
- preserve byte-oriented behavior for non-UTF-8 input,
- expose a Rust function with behavior aligned to `quotearg_n_custom_mem`,
- keep ownership and lifetime rules local to the module rather than reproducing C-style global mutable storage unless the original call pattern requires per-slot state.

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Maintain linear-time processing over the input buffer.
  - Avoid unnecessary intermediate allocations beyond what is needed to build the quoted result.
  - Preserve byte-level handling for arbitrary input data.
  - Keep per-call overhead comparable to the C implementation for typical command-line sized inputs.

## Module Mapping

- **C source file**: `quotearg.c`
- **Rust module**: `src/quotearg.rs`

Planned mapping:
- `quotearg_n_custom_mem` -> `pub(crate) fn quotearg_n_custom_mem(...)`
- required internal helpers from `quotearg.c` -> private functions inside `src/quotearg.rs`
- any C static state used specifically to support slot-based quoting for this function -> module-private Rust state with minimal equivalent scope

If the current Rust branch already has a main module layout, this module should be added directly to that existing structure and referenced from the current `cat` main-cluster code without creating extra intermediate modules.

## Data Model

The C analysis lists multiple anonymous data structures. For this port, they should not be reproduced anonymously; instead, map them into narrowly scoped named Rust types based on actual usage reached from `quotearg_n_custom_mem`.

### Planned C-to-Rust structure mapping

Because only `quotearg_n_custom_mem` is in scope, the data model should be reduced to the minimum set required by that path:

- **C anonymous quoting options struct(s)**
  -> `struct QuotingOptions`
  - Holds the option fields actually consulted by `quotearg_n_custom_mem` and its helpers.
  - Represent flag fields with Rust integer/bitflag-compatible storage or small enums if the set is closed and clear from the migrated code.
  - Character-set or quoting-style tables should become owned fixed-size arrays or slices, depending on original semantics.

- **C anonymous slot/result holder struct(s)**
  -> `struct QuotedSlot`
  - Stores the reusable output buffer for `n`-indexed quoting if the C function relies on persistent slot allocation.
  - Fields likely map to:
    - output bytes/string buffer,
    - cached capacity/length if needed for faithful behavior.

- **C anonymous custom quoting descriptor struct(s)**
  -> `struct CustomQuoting`
  - Stores the custom left/right quoting delimiters or equivalent custom markers used by `quotearg_n_custom_mem`.
  - Use byte slices or owned `Vec<u8>` depending on whether call sites require borrowing or persistence.

- **C anonymous flag/style representations**
  -> `enum QuotingStyle` and/or module constants
  - Only introduce variants/constants that are exercised by the implementation path for `quotearg_n_custom_mem`.

- **C raw character buffers / memory regions**
  -> `&[u8]`, `Vec<u8>`, and `String` only when UTF-8 validity is guaranteed
  - Prefer `&[u8]` for inputs because the C function accepts arbitrary memory.
  - Prefer `Vec<u8>` for internal quoted output if escaping is byte-oriented.

### Memory management decisions

- Replace C-managed reallocating slot buffers with Rust-owned buffers.
- If slot-indexed reuse is required, store `Vec<u8>` per slot and grow as needed.
- Avoid exposing raw pointers in the Rust-facing API unless the surrounding port layer forces it.
- Convert all implicit null-termination assumptions into explicit length-aware handling.

### Error handling decisions

- The original C function likely does not use rich error returns. Preserve that shape where practical.
- Internal allocation failures remain Rust panic/abort territory as normal for standard allocation behavior.
- For invalid assumptions that become explicit in Rust (for example, invalid slot index conversion), use internal checked conversions and keep behavior deterministic.

## Implementation Phases

## Phase 1: Isolate and map the `quotearg_n_custom_mem` dependency surface

- Inspect `quotearg.c` and identify the exact helper functions, constants, and state touched by `quotearg_n_custom_mem`.
- Determine whether the function depends on:
  - quoting style enums/constants,
  - custom delimiter storage,
  - slot-indexed reusable buffers,
  - default/global quoting options.
- Create `src/quotearg.rs`.
- Define the minimum Rust type set needed for this call path:
  - `QuotingOptions`
  - `CustomQuoting`
  - `QuotedSlot`
  - `QuotingStyle` or equivalent constants, only if actually referenced.
- Decide the Rust function signature based on current branch usage, keeping it byte-oriented and length-aware.

**Exit criteria**:
- All required C-side dependencies for `quotearg_n_custom_mem` are enumerated.
- Rust module file exists with type skeletons and function stubs.

## Phase 2: Port the core quoting logic and state handling

- Implement the quoting transformation used by `quotearg_n_custom_mem` over `&[u8]`.
- Port only the helper routines directly necessary for:
  - applying custom opening/closing quote markers,
  - escaping embedded bytes as required by the C logic,
  - building the final output buffer.
- Implement slot handling if the function’s `n` parameter selects persistent storage:
  - map C slot arrays to a Rust `Vec<QuotedSlot>` or equivalent module-private storage,
  - ensure growth is checked and localized,
  - avoid broader global state redesign.
- Preserve observable behavior around empty input, embedded special bytes, and custom delimiter insertion.

**Exit criteria**:
- `quotearg_n_custom_mem` is fully implemented in Rust.
- Required helper functions compile and are private to `src/quotearg.rs`.
- The implementation is free of unsafe code unless a surrounding existing interface absolutely requires it.

## Phase 3: Integrate with the existing main-cluster code

- Wire `src/quotearg.rs` into the current crate module tree.
- Replace or route the existing usage points in the `cat` main-cluster branch to the Rust implementation.
- Ensure argument and return types match the expectations of the calling code with minimal adaptation.
- Remove or avoid duplicate temporary implementations if present in the branch.

**Exit criteria**:
- Main-cluster code calls the Rust ported function successfully.
- Build passes with the module included in normal compilation.

## Phase 4: Verify compatibility with focused tests

- Add unit tests in the Rust module for:
  - empty input,
  - plain ASCII input,
  - custom left/right quote delimiters,
  - inputs containing bytes that require escaping,
  - repeated calls with different slot indices if slot behavior is part of the port,
  - non-UTF-8 byte input.
- Add integration coverage only if the current branch already has a direct path exercising this function from the `cat` command flow.
- Use expected outputs derived from the C behavior for the migrated path only.

**Exit criteria**:
- `cargo test` passes.
- The ported behavior for `quotearg_n_custom_mem` matches the C implementation for covered cases.