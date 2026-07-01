# Implementation Plan: module_src_yy_get_17

## Summary

This module ports the `src/c.c` scanner-buffer logic centered on:

- `yy_get_next_buffer`
- `yy_get_previous_state`

The Rust implementation should preserve the existing control flow and state transitions of the C scanner runtime while translating pointer-based buffer access into bounded slice/index operations and explicit state-bearing structs. The work should remain narrowly scoped to these existing functions and the data they directly manipulate.

The technical approach is:

- migrate the relevant scanner state from C anonymous structs into named Rust structs defined within the same Rust module or a tightly paired local module;
- represent mutable scanner buffers with `Vec<u8>` plus cursor/index fields rather than raw pointer arithmetic;
- encode function result categories with small Rust enums where the C code currently uses integer status values;
- keep the original algorithm order intact to reduce behavioral drift;
- use `Result` only where the C code can actually fail due to representational constraints in Rust; otherwise keep internal helper functions infallible and model scanner outcomes explicitly.

## Technical Context

- **Language/Version:** Rust 1.78+
- **Primary Dependencies:** Rust standard library only
- **Testing:** `cargo test`
- **Performance Goals:**
  - preserve linear-time scanning behavior of the original routines;
  - avoid per-call heap allocation in `yy_get_next_buffer` and `yy_get_previous_state` beyond existing buffer growth/reload needs;
  - keep buffer movement and state reconstruction bounded to the same asymptotic costs as the C implementation;
  - minimize copies by using in-place buffer compaction where the C code shifts unread data.

## Module Mapping

### C to Rust File Mapping

- `src/c.c`
  - migrate the logic for `yy_get_next_buffer` and `yy_get_previous_state` into a Rust scanner runtime module, preferably:
    - `src/module_src_yy_get_17.rs`
  - if the project already has a scanner-related Rust file, place these functions there instead of creating extra layers, and keep this module limited to the migrated functions and their directly required types/constants.

### Function Mapping

- `yy_get_next_buffer`
  - Rust function with mutable access to scanner state and buffer state
  - returns a Rust enum representing the original C outcome categories rather than bare integer constants

- `yy_get_previous_state`
  - Rust function that reconstructs the prior DFA/state-machine state from current buffer contents and scanner state
  - returns the scanner state type directly, or a narrow integer alias if the state machine is numeric in the existing code

### Constant and Macro Mapping

Any C macros used only by these functions should become:

- `const` items for numeric values
- small private helper functions for macro logic with side effects
- type aliases only where they preserve clarity of the translated scanner state

## Data Model

Because the analysis exposes only anonymous C structures, the Rust plan should introduce named internal types based on usage, not invent broader abstractions.

### Data-Structure Mapping

- **C anonymous scanner/global state**
  - **Rust:** `struct ScannerState`
  - Holds fields directly used by `yy_get_next_buffer` and `yy_get_previous_state`, such as:
    - current buffer handle/index if present
    - current state value
    - hold character / restoration character
    - text start/end positions
    - last accepting state and position if used by the DFA logic

- **C anonymous buffer state**
  - **Rust:** `struct BufferState`
  - Holds:
    - backing buffer: `Vec<u8>`
    - current position index
    - number of valid characters
    - buffer capacity
    - ownership/fill flags if present in the C code
    - end-of-buffer marker handling fields as required by the original logic

- **C anonymous DFA state representation**
  - **Rust:** `type DfaState = i32` or `usize`
  - Final choice should match the semantics of the original generated code:
    - use integer alias if state values are plain numeric labels;
    - use enum only if the C code clearly distinguishes a small fixed set of semantic states rather than generated numeric DFA states.

- **C anonymous accept-state tracking**
  - **Rust:** fields inside `ScannerState`
  - Avoid introducing separate structs unless the C code already groups them that way.

- **C anonymous result/status values from `yy_get_next_buffer`**
  - **Rust:** `enum NextBufferResult`
  - Typical variants should map directly to the original outcome categories used by callers, for example:
    - `ContinueScan`
    - `EndOfFile`
    - `LastMatch`
  - Variant names should follow the observed C meanings when implementing.

### Memory Management Decisions

- Replace raw character pointers with byte indices into `Vec<u8>`.
- Preserve sentinel/end-of-buffer behavior explicitly by reserving the required trailing marker space in the buffer.
- Use `copy_within` for unread-data compaction instead of manual pointer copying.
- Avoid references into the buffer that would outlive mutations; compute indices first, then mutate.

### Error Handling Decisions

- Internal scanner transitions should remain explicit and non-panicking in normal operation.
- Use debug assertions for invariants that mirror generated-code assumptions.
- Use `Result` only for operations that can genuinely fail in Rust representation terms, such as impossible index conversions or buffer growth failures if those are surfaced by the surrounding project conventions.
- Do not introduce new recovery behavior; invalid states should remain programmer/internal errors unless the original C code handled them.

## Implementation Phases

## Phase 1: Extract and Define Local Scanner Types

- Identify all fields in `src/c.c` that are directly read or written by:
  - `yy_get_next_buffer`
  - `yy_get_previous_state`
- Create named Rust equivalents for the anonymous C structs, limited to those fields.
- Introduce:
  - `ScannerState`
  - `BufferState`
  - `DfaState` alias
  - `NextBufferResult`
- Translate relevant C macros/constants into private Rust constants and helpers.
- Establish buffer invariants in comments and assertions:
  - valid data range
  - current cursor location
  - reserved end-of-buffer markers

## Phase 2: Port `yy_get_previous_state`

- Implement `yy_get_previous_state` first because it is state-reconstruction logic with fewer buffer mutation concerns.
- Translate the C transition loop as directly as possible:
  - preserve state update order;
  - preserve accepting-state tracking;
  - preserve character-class/index calculations.
- Replace pointer iteration with indexed traversal over the active buffer contents.
- Verify that end-of-buffer markers are handled exactly as in the original logic.
- Add focused unit tests for:
  - reconstruction across ordinary input
  - last-accepting-state updates
  - behavior at end-of-buffer boundaries

## Phase 3: Port `yy_get_next_buffer`

- Implement buffer compaction and refill logic using `Vec<u8>` and index arithmetic.
- Preserve the original branch structure for:
  - remaining text movement
  - refill/EOF decision
  - end-of-buffer status selection
- Encode returned statuses with `NextBufferResult`.
- Ensure hold-character restoration and cursor updates occur in the same sequence as the C implementation.
- Add tests for:
  - unread-data compaction
  - refill when space is available
  - EOF/no-refill paths
  - status returned for final-match vs continue conditions

## Phase 4: Integrate and Validate Module Behavior

- Wire both functions into the existing Rust scanner call path for this module only.
- Remove temporary translation scaffolding that is no longer needed after both functions compile together.
- Run `cargo test` and add regression cases covering the interaction between:
  - previous-state reconstruction
  - next-buffer refill transitions
  - end-of-buffer marker preservation
- Perform a final pass to eliminate accidental panics from unchecked indexing where simple bounds-safe restructuring is possible without altering control flow.