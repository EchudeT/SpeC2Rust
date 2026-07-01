# Implementation Plan

## Summary

Port the `quotearg.c` entry-point subset for `quotearg_n`, `quotearg_n_mem`, and `quotearg_n_custom` into Rust on branch `008-main_root_quotearg_n_08-rust-port`.

This work should stay narrowly scoped to the current C module surface: provide Rust equivalents for the three exported functions and migrate the data handling they depend on from C-style global/per-slot state into explicit Rust-managed storage. The implementation should preserve current behavior and call patterns as closely as practical while replacing raw buffer management with owned Rust allocations and checked indexing.

The technical approach is:

- create one Rust module corresponding to `quotearg.c`;
- represent quote option/state records with Rust structs/enums mirroring the C layout at the behavioral level;
- replace C static/per-slot allocation with `Vec`-backed storage and `String`/`Vec<u8>` buffers as appropriate;
- expose function signatures that match project needs, with internal helpers kept local to the module rather than expanding the API;
- preserve byte-oriented behavior for `_mem` forms and custom quoting inputs;
- handle invalid slot access, allocation growth, and UTF-8/non-UTF-8 boundaries explicitly rather than relying on C undefined behavior.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - maintain amortized O(n) processing for input length `n`;
  - avoid repeated full-buffer reallocations by reusing per-slot owned buffers where the C code reused allocated memory;
  - avoid unnecessary UTF-8 conversions for byte-oriented quoting paths;
  - keep per-call overhead close to the C implementation by using direct indexing and pre-allocation where sizes are known.

## Module Mapping

| C File | Rust File | Notes |
|---|---|---|
| `quotearg.c` | `src/quotearg.rs` | Direct migration target for the three functions in scope and their required local helpers/state. |

Suggested crate integration:

| Rust Item | Location | Purpose |
|---|---|---|
| `mod quotearg;` | `src/lib.rs` or existing crate root | Register migrated module using standard Rust module layout. |
| `pub fn quotearg_n(...)` | `src/quotearg.rs` | Rust implementation of slot-based quoting entry point. |
| `pub fn quotearg_n_mem(...)` | `src/quotearg.rs` | Byte-slice/length-based slot quoting entry point. |
| `pub fn quotearg_n_custom(...)` | `src/quotearg.rs` | Custom quote delimiter entry point. |

If the existing Rust crate is binary-only, the same module can be declared from `src/main.rs`; no extra facade module is needed beyond what the current project layout requires.

## Data Model

The C analysis only identifies multiple anonymous structures. For this migration, map them to minimal named Rust types based on role in `quotearg.c`, without introducing extra capabilities.

| C Structure | Rust Mapping | Notes |
|---|---|---|
| anonymous option record(s) | `struct QuoteOptions` | Holds quoting style/flags and any character-set or delimiter configuration needed by the three functions. |
| anonymous custom delimiter record(s) | `struct CustomQuoting` | Stores left/right custom quote delimiters for `quotearg_n_custom`. Prefer borrowed inputs at API boundary, clone into owned storage only if needed for persistence. |
| anonymous slot record(s) | `struct QuoteSlot` | Stores one reusable output buffer for slot `n`. Replace raw pointer + capacity fields with owned buffer and capacity derived from container state. |
| anonymous global slot table/state | `struct QuoteSlotTable` | Wraps `Vec<QuoteSlot>` and handles growth when `quotearg_n*` references a higher slot index. |
| anonymous style discriminator(s) | `enum QuoteStyle` | Rust enum replacing integer/tag-style selection where present in C. |
| anonymous flag sets | integer newtype or plain bitfield integer in `QuoteOptions` | Keep representation simple; use constants or internal helpers instead of adding external bitflag dependencies. |
| anonymous character tables / masks | `[bool; N]`, `[u8; N]`, or `Vec<u8>` as needed | Choose fixed arrays only if C logic is table-based and fixed-width; otherwise keep direct conditionals. |
| anonymous temporary buffer views | `&[u8]` / `&str` | Use borrowed slices instead of raw pointer + length pairs internally. |
| anonymous returned string storage | `String` or `Vec<u8>` inside `QuoteSlot` | Prefer `Vec<u8>` if output may be non-UTF-8; convert to `String` only when semantics guarantee valid UTF-8. |

### Memory Management Decisions

- Replace manual `malloc`/`realloc`/`free` slot buffers with Rust-owned `Vec<u8>` or `String`.
- Grow slot storage via `Vec::resize_with` or equivalent when a requested slot index is beyond current capacity.
- Avoid returning references to temporary buffers; returned references, if any, must point to storage owned by the slot table.
- Prefer byte buffers internally for `_mem` behavior to avoid accidental UTF-8 assumptions.

### Error Handling Decisions

- Preserve infallible behavior where the C API assumes successful quoting, but eliminate unchecked pointer arithmetic and unchecked indexing.
- For conditions that are impossible in safe Rust but were implicit in C, use internal assertions only where behavior is truly invariant.
- If the surrounding crate already uses result-bearing APIs, keep conversion at the boundary; do not redesign the module around new error abstractions.

## Implementation Phases

## Phase 1: Establish Rust module and core state types

- Create `src/quotearg.rs` and wire it into the existing crate root.
- Identify the minimum state from `quotearg.c` required by:
  - `quotearg_n`
  - `quotearg_n_mem`
  - `quotearg_n_custom`
- Define Rust replacements for:
  - quote options,
  - quote style discriminator,
  - custom delimiter data,
  - per-slot reusable output storage,
  - slot table/global state.
- Convert C anonymous structs into named internal Rust structs/enums based on actual field usage, not speculative generalization.
- Decide internal buffer representation:
  - use `Vec<u8>` if any path can emit non-UTF-8 bytes;
  - otherwise use `String` only where guaranteed valid.
- Implement slot-table growth and slot lookup helpers to replace raw static array management.

**Exit criteria**:
- Module compiles with placeholder-safe internal helpers.
- All required state from the C functions is represented in Rust with no raw allocation logic.

## Phase 2: Port quoting logic for `quotearg_n_mem` as the byte-oriented base

- Port the core quoting path used for pointer+length input into a Rust function operating on `&[u8]`.
- Translate C buffer sizing logic into safe reservation/preallocation.
- Preserve custom escaping and delimiter insertion behavior required by the current option set.
- Keep implementation byte-oriented first, since `quotearg_n` can delegate to this path.
- Store generated output into the selected slot buffer and return the appropriate borrowed result according to crate conventions.
- Verify slot reuse semantics: repeated calls with the same slot should overwrite/reuse the existing owned buffer rather than allocate fresh storage each time.

**Exit criteria**:
- `quotearg_n_mem` behavior is implemented against reusable slot storage.
- Tests cover empty input, short input, embedded special bytes, and repeated use of the same and different slots.

## Phase 3: Port `quotearg_n` and `quotearg_n_custom`

- Implement `quotearg_n` as the string-oriented wrapper over `quotearg_n_mem`, preserving C argument interpretation.
- Implement `quotearg_n_custom` by constructing the equivalent custom-quote option state and delegating into the shared quoting path.
- Ensure delimiter validation and lifetime handling are explicit:
  - borrow inputs for the call;
  - clone only when slot-persistent state truly requires ownership.
- Remove any remaining C-style assumptions about null-termination where Rust lengths are explicit.

**Exit criteria**:
- All three target functions are implemented in Rust and share a common internal quoting path.
- Tests cover default quoting, custom left/right delimiters, and slot-specific output replacement.

## Phase 4: Behavioral verification and cleanup

- Add focused `cargo test` coverage comparing migrated behavior across the three public functions for representative inputs and slot indices.
- Confirm memory behavior is safe:
  - no temporary reference escapes,
  - no unchecked indexing,
  - no dependence on null-terminated storage.
- Simplify any remaining translation artifacts from C such as redundant temporary state or duplicated branches.
- Keep the final module narrowly aligned with the original file responsibilities; do not split into additional support modules unless required by the existing crate structure.

**Exit criteria**:
- `cargo test` passes.
- The migrated module is self-contained, safe, and limited to the original `quotearg.c` responsibilities for the three functions in scope.