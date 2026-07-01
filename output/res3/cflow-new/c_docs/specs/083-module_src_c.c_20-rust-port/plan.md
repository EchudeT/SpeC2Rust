# Implementation Plan: module_src_c.c_20

## Summary

This module is a focused Rust port of the scanner buffer-management portion currently implemented in `src/c.c`. The identified functions indicate classic lexer runtime responsibilities: buffer switching, buffer stack push/pop, restart/flush/delete operations, NUL-transition handling, character pushback, fatal error reporting, and lightweight accessors for scanner state.

The Rust implementation should preserve the existing control flow and state layout as closely as practical, rather than redesigning the scanner runtime. The recommended approach is to migrate the relevant C file logic into a single Rust module that owns scanner state explicitly through Rust structs, replaces raw buffer ownership with `Vec<u8>` and `Option`-based handles, and converts implicit global/mutable C state into fields on a scanner context struct. The plan should prioritize behavioral equivalence for buffer lifecycle and state transitions, with minimal abstraction beyond what is needed for memory safety and compile-time correctness.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve linear-time scanner buffer operations comparable to the C implementation.
  - Avoid unnecessary buffer copies during restart, switch, push, and pop flows.
  - Keep character-buffer manipulation O(1) where the C implementation relies on direct index updates.
  - Maintain predictable memory ownership with no leaks and no use-after-free behavior.

## Module Mapping

| C Source | Rust Target | Notes |
|---|---|---|
| `src/c.c` | `src/module_src_c_c_20.rs` | Single-module port for the identified scanner runtime functions. |
| C scanner globals/state in `src/c.c` | `ScannerState` struct in `src/module_src_c_c_20.rs` | Consolidates mutable lexer runtime state that was previously implicit or file-scoped. |
| C buffer state records | `BufferState` struct in `src/module_src_c_c_20.rs` | Holds the active input buffer metadata and content. |
| C buffer stack storage | `Vec<BufferState>` or `Vec<Box<BufferState>>` within `ScannerState` | Exact choice depends on whether active/stacked ownership needs stable moves during translation. |
| C accessor functions (`yyget_*`) | inherent methods on `ScannerState` and thin free-function wrappers if needed | Keep names close to C during migration, then narrow visibility if possible. |
| C fatal path (`yy_fatal_error`) | helper returning `!` via panic or process termination wrapper | Final choice should follow existing project error conventions if already present elsewhere. |

## Data Model

Because the source analysis exposes only anonymous C data structures, the Rust model should be derived from actual field usage during migration. The mapping below keeps structure count restrained and tied to observed function needs.

| C Data Structure | Rust Representation | Mapping Notes |
|---|---|---|
| anonymous scanner runtime state | `struct ScannerState` | Central owner for current buffer, buffer stack, line tracking, input/output handles, current lexeme length, hold character, cursor indices, and flags referenced by migrated functions. |
| anonymous buffer state | `struct BufferState` | Represents one scanner buffer with owned byte storage, current position, size, line/column-related flags if referenced, fill status, interactivity flags, and status markers. |
| anonymous buffer stack container | `Vec<BufferState>` in `ScannerState` | Replaces manual dynamic arrays and stack counters from C. |
| anonymous input handle field | `Option<std::fs::File>` or generic reader placeholder decided from field usage | If the original module stores raw `FILE *` values only for restart/switch bookkeeping, use the narrowest concrete Rust type already used elsewhere in the port. Avoid introducing trait-object generalization unless required by call sites. |
| anonymous output handle field | `Option<std::fs::File>` or concrete writer type from existing project usage | Same constraint as input handle mapping. |
| anonymous status/discriminant fields | `enum BufferStatus` and/or primitive fields (`bool`, `usize`, `u8`) | Use enums only where the C code uses distinct named states; otherwise keep primitive fields for close translation. |
| anonymous temporary character state | `u8` / `Option<u8>` | Suitable for `yyunput` and hold-character restoration logic. |
| anonymous length/position counters | `usize` | Replace C integer indexes and lengths after checking for sentinel usage; use `isize` only where negative sentinel semantics exist. |

### Proposed Core Rust Types

```rust
pub struct ScannerState {
    current_buffer: Option<BufferState>,
    buffer_stack: Vec<BufferState>,
    line_number: usize,
    yy_leng: usize,
    hold_char: Option<u8>,
    cursor: usize,
    text_start: usize,
    input: Option<std::fs::File>,
    output: Option<std::fs::File>,
    // additional translated fields only as required by src/c.c usage
}

pub struct BufferState {
    buffer: Vec<u8>,
    pos: usize,
    n_chars: usize,
    is_our_buffer: bool,
    is_interactive: bool,
    at_bol: bool,
    fill_buffer: bool,
    // status/ownership fields added strictly from actual C field access
}

pub enum BufferStatus {
    New,
    Normal,
    EofPending,
}
```

### Memory Management Decisions

- Replace manual allocation/free for buffer records with owned Rust structs.
- Replace raw character arrays with `Vec<u8>`.
- Represent nullable current-buffer pointers with `Option<BufferState>` or `Option<Box<BufferState>>`.
- Ensure `yy_delete_buffer` semantics are modeled by dropping ownership and clearing active references safely.
- Preserve stack ordering exactly during `yypush_buffer_state`/`yypop_buffer_state`; no shared ownership is needed unless the C code demonstrably aliases buffer records.

### Error Handling Decisions

- Functions equivalent to accessors and buffer state changes should prefer infallible mutation where the original C code assumes valid state.
- Preconditions formerly unchecked in C should be enforced with explicit branch handling in Rust:
  - no-op on deleting `None` buffers where C tolerates null,
  - safe handling when popping an empty buffer stack,
  - explicit panic/abort path through `yy_fatal_error` for unrecoverable scanner invariants.
- Do not introduce broad `Result` propagation across the module unless existing call patterns in the port require it.

## Implementation Phases

## Phase 1: Establish Rust State and Buffer Types

- Create `src/module_src_c_c_20.rs`.
- Identify all file-scope mutable scanner variables used by:
  - `yy_try_NUL_trans`
  - `yyunput`
  - `yyrestart`
  - `yy_switch_to_buffer`
  - `yy_load_buffer_state`
  - `yy_delete_buffer`
  - `yy_flush_buffer`
  - `yypush_buffer_state`
  - `yypop_buffer_state`
  - `yy_fatal_error`
  - `yyget_lineno`
  - `yyget_in`
  - `yyget_out`
  - `yyget_leng`
- Consolidate those variables into `ScannerState`.
- Define `BufferState` with only the fields actually referenced by the migrated functions.
- Map C nullability and ownership rules to `Option` and owned `Vec<u8>` storage.
- Add minimal constructor/setup helpers strictly required to support the migrated functions.

**Deliverable**: Compiling Rust data model with placeholders or stubbed methods matching the C function set.

## Phase 2: Port Buffer Lifecycle and State Transition Functions

- Port in dependency order:
  1. `yy_load_buffer_state`
  2. `yy_flush_buffer`
  3. `yy_delete_buffer`
  4. `yy_switch_to_buffer`
  5. `yyrestart`
  6. `yypush_buffer_state`
  7. `yypop_buffer_state`
- Keep function names and internal sequencing close to the C implementation to simplify equivalence checking.
- Preserve observable semantics for:
  - active buffer replacement,
  - stack push/pop order,
  - restart behavior,
  - flushing/resetting cursor and character counts,
  - deleting current vs non-current buffers.
- Resolve ownership moves carefully when switching current buffer and stack entries; use `std::mem::replace`/`take` as needed rather than cloning.

**Deliverable**: End-to-end buffer management behavior migrated and compiling without unsafe code unless field-level translation proves it necessary.

## Phase 3: Port Character/Transition Logic and Accessors

- Port `yyunput` with exact index and hold-character semantics validated against the C code.
- Port `yy_try_NUL_trans`, preserving transition-state behavior and sentinel/NUL handling.
- Port `yy_fatal_error` as the module’s unrecoverable error path.
- Port accessors:
  - `yyget_lineno`
  - `yyget_in`
  - `yyget_out`
  - `yyget_leng`
- Keep accessor return types aligned with the Rust field types chosen in Phase 1; add narrow compatibility wrappers only if existing callers require C-like signatures.

**Deliverable**: Functional parity for scanner state mutation and state inspection routines.

## Phase 4: Validation and Cleanup

- Write `cargo test` unit tests focused on migrated behavior:
  - flush resets expected buffer fields,
  - switch/load updates active state,
  - push/pop preserves stack order,
  - delete clears ownership safely,
  - unput adjusts buffer contents/position correctly,
  - accessor methods reflect internal state,
  - fatal path is asserted with panic testing if implemented via panic.
- Compare the Rust control paths against the original `src/c.c` to remove migration-only placeholders and unused fields.
- Reduce visibility to module-private where external use is not required by the rest of the port.
- Confirm no extra module splits or helper layers were added beyond what the migrated file requires.

**Deliverable**: Tested Rust port of the `src/c.c` scanner buffer-management subset, ready for integration on branch `083-module_src_c.c_20-rust-port`.