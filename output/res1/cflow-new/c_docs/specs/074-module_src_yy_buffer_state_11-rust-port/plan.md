# Implementation Plan: module_src_yy_buffer_state_11

## Summary

This module covers the buffer lifecycle and buffer-stack preparation logic currently implemented in `src/c.c` through the functions `yy_create_buffer`, `yyensure_buffer_stack`, and `yy_scan_buffer`.

The Rust implementation should port these routines as a focused buffer-management module without adding new subsystem boundaries or extra runtime features. The technical approach is to translate the existing C buffer state handling into Rust-owned data structures using `Vec<u8>` and explicit state structs, preserving the current control flow and invariants around:

- buffer allocation and initialization,
- ensuring stack capacity for active buffers,
- scanning over an existing caller-provided buffer with boundary/sentinel validation.

Because the source originates from C-style lexer buffer management, the Rust design should keep state transitions explicit and local, avoiding unnecessary abstraction. Ownership and borrowing should replace manual allocation where Rust owns memory; when the original logic relies on externally supplied memory, the Rust port should represent that distinction directly so destruction and reuse rules remain correct.

## Technical Context

### Language/Version

- Rust 1.78+ edition 2021

### Primary Dependencies

- Rust standard library only:
  - `Vec`
  - `Option`
  - `Box` only if needed for stable ownership of nested state
  - `Result` for fallible creation and validation paths

No third-party crates are recommended because the input does not show requirements beyond standard memory and state management.

### Testing

- `cargo test`

Testing should cover:
- creation of a new buffer with expected initial metadata,
- repeated stack growth through `yyensure_buffer_stack`,
- scanning a valid external buffer,
- rejection or error signaling for invalid scan-buffer inputs,
- preservation of sentinel/end-marker assumptions required by the original C logic.

### Performance Goals

- Preserve the current asymptotic behavior of the C implementation.
- Keep buffer creation and stack extension amortized `O(1)` per push/growth step.
- Avoid unnecessary copying for scan operations over caller-supplied buffers.
- Use contiguous byte storage and direct indexing where safe and clear.
- Keep allocations limited to the points where the C code also allocates or grows storage.

## Module Mapping

### Source Mapping

- C source file:
  - `src/c.c`

- Rust target module:
  - `src/module_src_yy_buffer_state_11.rs`

If the project already has a central lexer/runtime module, this file should be integrated there only by minimal declaration and use statements. The implementation should remain concentrated in a single Rust module corresponding to this migration unit.

### Function Mapping

- `yy_create_buffer`
  - Port to a Rust constructor-style function that allocates and initializes a buffer-state struct.
  - Prefer a signature returning `Result<YyBufferState, BufferError>` if allocation/validation failures need representation.
  - If the surrounding port already uses infallible allocation assumptions, keep the result shape aligned with that existing project pattern.

- `yyensure_buffer_stack`
  - Port to a mutating function on the owning lexer/global state struct.
  - Responsibility remains limited to ensuring stack storage exists and has capacity for buffer entries.
  - Do not combine with unrelated push/pop behavior.

- `yy_scan_buffer`
  - Port to a function that validates and wraps an externally supplied byte buffer as a scanning buffer state.
  - Preserve the distinction between owned buffers created internally and externally supplied buffers that must not be freed/reallocated by the module.

## Data Model

The analysis lists multiple anonymous C data structures. For this migration, they should not be expanded into speculative Rust types beyond what is required by the three target functions. The port should derive the minimal concrete Rust structures from the actual fields touched by these routines.

### Data-Structure Mapping

Because the C source uses anonymous/internal structures, the Rust mapping should be organized by usage rather than by preserving anonymous naming:

- C anonymous buffer-state struct used by `yy_create_buffer` / `yy_scan_buffer`
  - Rust: `struct YyBufferState`
  - Expected fields to port only if referenced by these functions:
    - underlying byte storage or external buffer reference
    - current buffer size
    - number of characters / effective length if present
    - ownership flag for externally supplied vs internally allocated memory
    - interactive/beginning-of-line/fill flags only if these functions initialize them
    - position fields and status fields only if they are written during creation/scan setup

- C anonymous scanner/global state struct used by `yyensure_buffer_stack`
  - Rust: `struct YyGlobalState` or the existing owning scanner struct in the port
  - Expected fields:
    - buffer stack storage
    - current stack length/capacity indicators if not implied by `Vec`
    - current buffer index/pointer only if touched by stack initialization logic

- C anonymous stack-entry pointer arrays
  - Rust: `Vec<YyBufferState>` or `Vec<Box<YyBufferState>>`
  - Choose `Vec<YyBufferState>` unless self-referential or stable-address requirements in surrounding migrated code force boxed elements.

### Ownership Model

- Internally created buffers:
  - Rust-owned allocation, typically `Vec<u8>`.
  - The buffer state owns its memory and drops it normally.

- Externally scanned buffers:
  - Represent as non-owning or specially tagged storage inside `YyBufferState`.
  - Recommended shape:
    - `enum BufferStorage { Owned(Vec<u8>), External(Vec<u8>) }` only if the external buffer is transferred into Rust ownership by API design, or
    - `enum BufferStorage<'a> { Owned(Vec<u8>), Borrowed(&'a mut [u8]) }` if the surrounding architecture already carries lifetimes cleanly.
  - If lifetime propagation would disrupt the existing port structure, use a project-consistent representation that keeps the "do not free/reallocate external memory" rule explicit.

### Error Mapping

C routines that return null pointers or signal invalid inputs should map to explicit Rust errors where practical:

- invalid external buffer layout for `yy_scan_buffer`
- impossible size assumptions or overflow during allocation/growth
- allocation failure is normally handled by Rust panic/abort semantics unless the existing project design wraps such failures

A small local error enum is sufficient if needed:

- `enum BufferError`
  - `InvalidScanBuffer`
  - `SizeOverflow`
  - `StackGrowthFailed` only if the project already models such failures explicitly

## Implementation Phases

## Phase 1: Establish Rust buffer-state types and direct constructor port

### Goal

Create the Rust data structures and port `yy_create_buffer` with the same initialization responsibilities as the C code.

### Tasks

- Inspect `src/c.c` and extract only the fields read or written by:
  - `yy_create_buffer`
  - `yy_scan_buffer`
  - `yyensure_buffer_stack`
- Define `YyBufferState` in `src/module_src_yy_buffer_state_11.rs`.
- Define or extend the owning scanner/global state struct only with the buffer-stack fields required by `yyensure_buffer_stack`.
- Port `yy_create_buffer`:
  - allocate buffer storage with the required extra sentinel/end-marker space from the C logic,
  - initialize all status and cursor fields touched by the C function,
  - mark the buffer as owned by the Rust state,
  - preserve any default flags set by the original implementation.

### Deliverables

- New Rust module file with core structs.
- Compiling implementation of `yy_create_buffer`.
- Unit tests for basic buffer creation and initialization invariants.

### Notes

- Mirror C initialization order where practical to reduce migration risk.
- Do not generalize into a reusable allocator abstraction.

## Phase 2: Port buffer stack preparation logic

### Goal

Migrate `yyensure_buffer_stack` so the owning scanner/global state can lazily initialize and grow its buffer stack.

### Tasks

- Port stack initialization behavior exactly:
  - create stack storage when absent,
  - reserve or grow capacity when full or near full according to the C routine,
  - preserve any initial slack space assumptions used by the original code.
- Replace manual C array reallocation with `Vec` growth while preserving visible semantics.
- Port any current-index initialization done during first stack setup.

### Deliverables

- Compiling implementation of `yyensure_buffer_stack`.
- Tests covering:
  - first-time stack creation,
  - repeated ensure calls without unnecessary state changes,
  - growth after capacity pressure.

### Notes

- Keep this phase limited to capacity management.
- Do not add stack APIs beyond what existing migrated callers require.

## Phase 3: Port external-buffer scanning setup

### Goal

Migrate `yy_scan_buffer` with explicit validation and ownership handling for caller-provided buffers.

### Tasks

- Identify the exact C preconditions for a scan buffer to be accepted.
- Implement equivalent Rust validation, especially around required trailing sentinel/end-marker bytes.
- Construct `YyBufferState` for an external buffer without converting it into normal owned storage unless the surrounding port already requires ownership transfer.
- Preserve initialization of cursor/state fields performed during scan setup.
- Ensure the buffer state's ownership flag or storage enum prevents accidental deallocation/reallocation of external memory.

### Deliverables

- Compiling implementation of `yy_scan_buffer`.
- Tests for:
  - valid external buffer acceptance,
  - invalid sentinel layout rejection,
  - ownership-mode distinction between created and scanned buffers.

### Notes

- Prefer exact compatibility with current C assumptions over API elegance.
- Keep the external-buffer representation as narrow as possible.

## Phase 4: Integration cleanup and migration verification

### Goal

Connect the new module into the branch with minimal edits and verify behavioral parity for the migrated functions.

### Tasks

- Replace or gate the corresponding C-path usage with the Rust implementations in this branch.
- Align names/signatures with surrounding migrated code conventions where necessary, without changing function responsibilities.
- Remove temporary stubs introduced during earlier phases.
- Add focused regression tests covering the interaction sequence:
  - ensure stack,
  - create buffer,
  - scan external buffer.

### Deliverables

- Integrated Rust module used by the branch.
- Passing `cargo test`.
- Final code review pass for:
  - ownership correctness,
  - absence of out-of-bounds indexing,
  - correct handling of external versus owned buffers.

## Migration Order

1. Define the buffer-state and scanner-state fields required by the three functions.
2. Port `yy_create_buffer`.
3. Port `yyensure_buffer_stack`.
4. Port `yy_scan_buffer`.
5. Integrate and run module-level tests.

## Acceptance Criteria

- All three target functions are implemented in Rust and compiled on branch `074-module_src_yy_buffer_state_11-rust-port`.
- The Rust module remains limited to the responsibilities present in `src/c.c` for this migration unit.
- Buffer ownership semantics are explicit and safe.
- Stack growth and scan-buffer validation preserve the original behavior closely enough for existing callers.
- `cargo test` passes with focused coverage for creation, stack ensure, and scan-buffer cases.