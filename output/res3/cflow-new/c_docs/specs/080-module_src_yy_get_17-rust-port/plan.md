# Implementation Plan: module_src_yy_get_17

## Summary

This module migrates the buffer-transition and scanner-state logic currently implemented in `src/c.c` for the functions `yy_get_next_buffer` and `yy_get_previous_state` into Rust. The Rust implementation should preserve the existing control flow and state semantics closely, with minimal redesign, because these functions are part of a scanner runtime path where behavioral compatibility matters more than abstraction.

The implementation approach is a direct port of the involved logic into a Rust module that operates over explicit scanner state structures instead of implicit C globals/macros where possible. Pointer-based buffer manipulation from C should be translated into slice indexing and bounded position tracking in Rust. Any C sentinel or status codes used by these functions should be represented with small Rust enums or constants to keep the translated control flow readable while preserving the original branching structure.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve near-C execution characteristics for scanner hot paths.
  - Avoid unnecessary allocation during buffer inspection and state transitions.
  - Use contiguous byte storage (`Vec<u8>` or borrowed byte slices) and index-based traversal.
  - Keep translated logic single-pass where the C implementation is single-pass.

## Module Mapping

### C to Rust File Mapping

- `src/c.c`
  - Relevant C functions:
    - `yy_get_next_buffer`
    - `yy_get_previous_state`
  - Rust target:
    - `src/module_src_yy_get_17.rs`

### Rust Module Responsibilities

- Host the direct Rust ports of:
  - `yy_get_next_buffer`
  - `yy_get_previous_state`
- Define only the scanner-state and buffer-state types required by these functions.
- Keep helper constants/types local to this module unless already shared elsewhere in the Rust port.

## Data Model

Because the source analysis exposes only anonymous C data structures, the Rust plan should introduce narrowly scoped named types based on usage within these two functions rather than inventing broader models.

### Data-structure Mapping Strategy

- **C anonymous structs used by scanner runtime state**
  - **Rust mapping**: named `struct` types with fields inferred from actual access patterns in `yy_get_next_buffer` and `yy_get_previous_state`
  - **Notes**:
    - Consolidate only fields referenced by these functions.
    - Use explicit field names derived from the original C variable/member names where available.

### Expected Rust Types

#### Scanner State Holder
- **C**: anonymous scanner/global runtime state
- **Rust**: `struct ScannerState`
- **Purpose**:
  - Hold current buffer reference or index
  - Track current position in buffer
  - Track hold character / end-of-buffer handling state
  - Store lexer automaton state needed by `yy_get_previous_state`

#### Buffer State
- **C**: anonymous buffer-state struct
- **Rust**: `struct BufferState`
- **Purpose**:
  - Own or reference buffer bytes
  - Track valid character count
  - Track buffer positioning fields used during refill logic
  - Represent end-of-buffer markers explicitly

#### DFA / Transition State
- **C**: anonymous state values and transition table related records
- **Rust**:
  - `type StateId = usize` or `u32` depending on table indexing needs
  - small supporting `struct`/`enum` only if required by the translated code
- **Purpose**:
  - Represent scanner automaton state during backtracking/state reconstruction

#### Buffer Action Result
- **C**: integer return codes from `yy_get_next_buffer`
- **Rust**: `enum NextBufferResult`
- **Variants**:
  - `ContinueScan`
  - `LastMatch`
  - `EndOfFile`
- **Purpose**:
  - Replace magic integer status codes with a local enum while preserving exact branching behavior

### C-to-Rust Type Conventions

- `char *` / byte buffer pointers -> `Vec<u8>` plus indices, or `&[u8]`/`&mut [u8]` where ownership is external
- pointer offsets -> `usize`
- C integer state codes -> `usize`, `u32`, or local enums based on table usage
- mutable global-like scanner variables -> fields on `ScannerState`
- sentinel characters -> `u8`

### Memory Management Notes

- Replace manual pointer movement and overlapping character shifting with bounded slice copies.
- Prefer `copy_within` for in-buffer compaction when matching C memmove-style behavior.
- Keep ownership of scanner buffers explicit to prevent aliasing issues.
- Avoid introducing heap allocation in `yy_get_previous_state` unless the C logic already depends on external tables/buffers.

### Error Handling Notes

- If the original C code assumes valid internal state, represent the same assumptions with internal invariants rather than broad recoverable error APIs.
- Use `debug_assert!` for index/state invariants in hot paths.
- Reserve `Result` return types only for cases where Rust file/module integration requires handling invalid setup; do not add new error surfaces to the two migrated functions without source evidence.

## Implementation Phases

## Phase 1: Extract and Model Required Runtime State

- Inspect `src/c.c` and isolate all fields, constants, and tables touched by:
  - `yy_get_next_buffer`
  - `yy_get_previous_state`
- Define `src/module_src_yy_get_17.rs`.
- Introduce minimal Rust structs/enums for:
  - scanner state
  - buffer state
  - next-buffer result codes
  - automaton state identifiers
- Map C macros used by these functions into Rust constants or small inline helpers.
- Keep all definitions limited to what is necessary for these two ports.

### Deliverables
- Rust module file created
- Minimal type definitions compiled
- Constant/table access strategy fixed

## Phase 2: Port `yy_get_previous_state`

- Translate the DFA/state reconstruction logic first, because it is logically narrower and provides the state semantics used by buffer handling.
- Port transition traversal using explicit indexing into Rust arrays/slices.
- Preserve the original iteration order and state updates.
- Replace pointer walking over matched text with byte-slice/index traversal.
- Add focused unit tests for:
  - empty/initial traversal cases
  - normal state accumulation across bytes
  - end-of-buffer-sensitive state reconstruction if present in the C logic

### Deliverables
- `yy_get_previous_state` implemented
- Local tests covering state reconstruction paths
- Verified index safety and type correctness

## Phase 3: Port `yy_get_next_buffer`

- Translate buffer refill/end-of-buffer logic using the state model from earlier phases.
- Replace C buffer shifting with Rust slice operations such as `copy_within`.
- Preserve the original meaning of:
  - remaining text movement
  - hold character restoration
  - end-of-buffer marker placement
  - return status selection
- Ensure all index arithmetic is bounds-checked by construction.
- Keep allocation behavior minimal and aligned with the original logic.

### Deliverables
- `yy_get_next_buffer` implemented
- Tests for:
  - buffer continuation path
  - last-match path
  - end-of-file path
  - text preservation during buffer movement

## Phase 4: Integration and Behavioral Verification

- Wire the module into the Rust project at the exact call sites replacing the C implementation path for this module scope.
- Align naming and visibility with the rest of the port branch without broad refactoring.
- Run `cargo test` and fix mismatches caused by state layout or return-code interpretation.
- Review for:
  - no unnecessary allocations
  - no unsafe code unless table/layout constraints make it unavoidable
  - close correspondence between C branches and Rust branches

### Deliverables
- Module integrated into branch `080-module_src_yy_get_17-rust-port`
- Tests passing with the migrated implementation
- Final review of memory/index invariants