# Implementation Plan

## Summary

This plan ports the `src/c.c` scanner-buffer logic for `yy_get_next_buffer` and `yy_get_previous_state` into a focused Rust module on branch `080-module_src_yy_get_17-rust-port`.

The Rust implementation should preserve the existing control flow and state transitions of the C code rather than redesigning the scanner. The technical approach is a direct migration of the involved buffer/state handling into Rust-owned data structures, using slices, indices, and enums to replace pointer arithmetic and integer status codes where possible. Any anonymous C data shapes referenced by these functions should be named in Rust according to their actual role in the scanner state, but only to the extent required to port these functions and their immediate dependencies.

Key implementation priorities:

- keep behavior aligned with the original scanner logic
- translate pointer-based buffer access into bounds-checked index operations
- represent scanner/buffer status codes with Rust enums when they are local to this module
- minimize allocation changes beyond what is necessary to express ownership safely
- preserve performance characteristics expected from lexer buffer movement/state recomputation

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Maintain near-equivalent runtime behavior for buffer refill and previous-state reconstruction paths
  - Avoid unnecessary heap allocations during repeated scanner advancement
  - Keep hot-path operations on buffer contents index-based and contiguous
  - Preserve amortized linear behavior of buffer consumption/refill logic

## Module Mapping

### C to Rust File Mapping

- `src/c.c`
  - migrate relevant logic into `src/module_src_yy_get_17.rs`
  - if the crate already centralizes scanner code elsewhere, place the ported functions into the existing scanner module instead of introducing extra abstraction layers

### Function Mapping

- `yy_get_next_buffer`
  - port as a Rust function with mutable access to scanner state and buffer state
  - return a Rust enum or integer-compatible status type matching the original branch behavior
- `yy_get_previous_state`
  - port as a Rust function that recomputes scanner state from buffered characters and current DFA state data
  - operate on explicit scanner context rather than implicit globals/macros where possible

### Macro/Global-State Handling

C scanner macros and implicit globals used by these functions should be converted into:

- fields on a scanner state struct
- small private helper methods only where needed to preserve readability of the migrated code
- module-local constants/enums for buffer action/status values

No additional module decomposition should be introduced beyond what is required to host these two functions and the data they directly manipulate.

## Data Model

The input lists only anonymous C data structures. For the Rust port, map them by usage rather than by preserving anonymous layout names.

### Data-structure Mapping Principles

- anonymous C scanner-global state
  - -> `ScannerState`
  - holds mutable scanning position, current buffer reference/state, hold character, and DFA-related working fields used by the two functions
- anonymous C buffer-state structure
  - -> `BufferState`
  - holds character buffer, current fill length, buffer position markers, ownership flags if needed, and end-of-buffer metadata
- anonymous C transition/state tables
  - -> borrowed slices or `Vec`s inside a table container such as `DfaTables`
  - use integer index types (`usize` for indexing, fixed-width integer types only if required by source layout)
- anonymous C status/action constants
  - -> Rust `enum` with explicit discriminants if compatibility with existing numeric logic matters
- anonymous C pointer-based cursor fields
  - -> `usize` indices into `Vec<u8>`/`&[u8]`
- anonymous C character storage
  - -> `u8`
  - use byte-oriented handling unless the original code clearly depends on signed `char`
- anonymous C booleans/flags
  - -> `bool` or small enums where the flag encodes more than two states

### Proposed Rust Types

```rust
pub(crate) struct ScannerState {
    buffer: BufferState,
    hold_char: u8,
    buf_pos: usize,
    text_start: usize,
    n_chars: usize,
    // DFA working state required by yy_get_previous_state / yy_get_next_buffer
    current_state: i32,
    // references or owned tables as required by existing scanner design
}

pub(crate) struct BufferState {
    data: Vec<u8>,
    n_chars: usize,
    buf_size: usize,
    at_bol: bool,
    fill_buffer: bool,
    buffer_status: BufferStatus,
}

pub(crate) enum BufferStatus {
    New,
    Normal,
    EofPending,
}

pub(crate) enum NextBufferResult {
    ContinueScan,
    LastMatch,
    EndOfFile,
}
```

### Memory Management Decisions

- Replace raw buffer pointers with `Vec<u8>` plus explicit indices.
- Preserve the C layout concept of end-of-buffer sentinel bytes by reserving space in the vector if the original logic depends on sentinel placement.
- Avoid copying active lexeme content unless the original algorithm requires buffer compaction before refill.
- Keep ownership local to `ScannerState`/`BufferState`; do not introduce shared ownership unless already required by existing crate structure.

### Error Handling Decisions

These functions are typically internal scanner mechanics and generally encode outcomes as status values rather than recoverable errors. The Rust port should therefore:

- preserve internal status-return behavior with enums
- use `debug_assert!` for invariant checks that reflect assumptions from the C code
- avoid introducing `Result` unless a true external failure path exists in the migrated logic (for example, a read/fill operation already modeled as fallible elsewhere in the crate)

## Implementation Phases

## Phase 1: Extract and Model Required Scanner State

- Identify all fields, macros, constants, and table accesses referenced by `yy_get_next_buffer` and `yy_get_previous_state` in `src/c.c`.
- Define minimal Rust structs/enums for:
  - scanner state
  - buffer state
  - buffer status / next-buffer return status
  - DFA table access required by previous-state computation
- Replace anonymous C structures with named Rust types based strictly on usage in these two functions.
- Decide the exact byte-buffer representation, including how end-of-buffer markers are stored.

**Exit criteria**:
- All data required by the two functions is represented in Rust types.
- No unresolved anonymous-struct usage remains for the targeted functions.

## Phase 2: Port `yy_get_previous_state`

- Translate DFA/state reconstruction logic directly from C into Rust.
- Convert pointer walks over buffered characters into index-based iteration.
- Preserve transition ordering and acceptance-state handling exactly.
- Keep helper extraction minimal; only factor tiny private helpers if required to mirror repeated table lookups or character classification access.

**Exit criteria**:
- `yy_get_previous_state` compiles in Rust.
- State transitions and acceptance tracking are represented without unsafe pointer arithmetic.
- Unit tests cover representative state recomputation paths from buffered input.

## Phase 3: Port `yy_get_next_buffer`

- Translate buffer compaction, refill-decision, and end-of-buffer handling into Rust.
- Preserve the original relation among:
  - current text position
  - number of valid characters
  - hold character restoration
  - sentinel/end-of-buffer behavior
  - returned action/status
- Implement any required movement of unconsumed bytes using safe slice operations.
- If external input refill is performed outside these functions in the current crate, integrate with that existing mechanism rather than designing a new reader abstraction.

**Exit criteria**:
- `yy_get_next_buffer` compiles and updates scanner state correctly.
- Buffer movement/refill logic does not rely on raw pointers.
- Returned status values match original control-flow branches.

## Phase 4: Validation and Behavioral Tightening

- Add focused `cargo test` coverage for:
  - previous-state recomputation over normal buffered text
  - buffer advance when unread bytes remain
  - end-of-buffer transitions
  - EOF-pending/final-buffer branch behavior, if present in the source logic
- Compare key state mutations against the C behavior for the targeted paths.
- Remove any temporary scaffolding introduced during migration and keep the final API limited to the required module surface.

**Exit criteria**:
- Tests pass under `cargo test`.
- The Rust module is limited to the migrated scanner functionality without added facilities.
- Buffer/state invariants are documented inline where the C code previously relied on implicit assumptions.