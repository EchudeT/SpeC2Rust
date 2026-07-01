# Implementation Plan

## Summary

Port `src/c.c` into a single Rust module that preserves the existing lexer-buffer management behavior and public function surface represented by the analyzed symbols. The Rust implementation should focus on translating the current stateful scanner support logic around buffer switching, buffer stack operations, restart/flush/delete behavior, NUL transition handling, pushback (`yyunput`), fatal error reporting, and simple accessors for scanner state.

The implementation approach is a direct migration of the existing C file into Rust-standard ownership and borrowing patterns without adding new capabilities. The port should replace implicit global/mutable C state with an explicit Rust scanner state structure held within the module, while keeping function responsibilities aligned with the original C functions. Memory management should move from manual allocation/free patterns to owned Rust structs, `Vec`, and `Option`, with index-based or boxed storage where stable internal references are needed.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve linear-time buffer scanning behavior and constant-time access for current buffer state.
  - Avoid unnecessary copying during buffer switches, flushes, and stack push/pop operations.
  - Keep pushback and NUL-transition handling close to the C control flow to minimize behavioral drift.
  - Maintain predictable memory usage by using owned buffers and bounded stack growth consistent with existing behavior.

## Module Mapping

### C to Rust File Mapping

- `src/c.c` -> `src/module_src_c_c_20.rs`

### Function Mapping

All analyzed functions should be migrated into the Rust module with responsibility-preserving equivalents:

- `if` -> no direct function mapping; treat as recovered parser artifact / control-flow only, not an exported Rust item
- `yy_try_NUL_trans` -> `fn yy_try_nul_trans(...)`
- `yyunput` -> `fn yyunput(...)`
- `yyrestart` -> `fn yyrestart(...)`
- `yy_switch_to_buffer` -> `fn yy_switch_to_buffer(...)`
- `yy_load_buffer_state` -> `fn yy_load_buffer_state(...)`
- `yy_delete_buffer` -> `fn yy_delete_buffer(...)`
- `yy_flush_buffer` -> `fn yy_flush_buffer(...)`
- `yypush_buffer_state` -> `fn yypush_buffer_state(...)`
- `yypop_buffer_state` -> `fn yypop_buffer_state(...)`
- `yy_fatal_error` -> `fn yy_fatal_error(...) -> !`
- `yyget_lineno` -> `fn yyget_lineno(...) -> i32` or `usize` based on effective C field type
- `yyget_in` -> `fn yyget_in(...) -> ...`
- `yyget_out` -> `fn yyget_out(...) -> ...`
- `yyget_leng` -> `fn yyget_leng(...) -> usize`

### Rust Module Scope

Keep the migrated logic contained in one Rust source file for this port. Do not split buffer logic into additional helper modules unless required by compilation. Internal helpers may be added inside the same file to replace C macros or repeated state updates.

## Data Model

Because the analysis only exposes anonymous C data structures, define Rust types according to their observed usage in `src/c.c` during migration.

### Primary Structure Mapping

- **C anonymous scanner/buffer state structs** -> **named Rust structs in `src/module_src_c_c_20.rs`**
  - Introduce explicit names derived from role, such as:
    - `ScannerState`
    - `BufferState`
    - `BufferStack`
  - Name selection should be driven by the actual C field groups present in `src/c.c`.

### Expected Structural Mapping

#### Buffer state
Typical lexer buffer state in C should map to an owned Rust struct:

```rust
struct BufferState {
    data: Vec<u8>,
    pos: usize,
    n_chars: usize,
    is_our_buffer: bool,
    is_interactive: bool,
    at_bol: bool,
    fill_buffer: bool,
    buffer_status: BufferStatus,
}
```

If the C code stores sentinel NUL bytes, preserve them explicitly in `Vec<u8>` rather than relying on string types.

#### Buffer status / flags
Integer or macro-based status fields in C should become Rust enums when the value set is closed and evident:

```rust
enum BufferStatus {
    New,
    Normal,
    EofPending,
}
```

If exact status constants are unclear, begin with integer-compatible representation during initial migration and tighten to an enum once validated by tests.

#### Scanner state
Implicit globals and related mutable fields in C should be consolidated:

```rust
struct ScannerState {
    current_buffer: Option<usize>,
    buffer_stack: Vec<BufferState>,
    hold_char: u8,
    c_buf_p: usize,
    n_chars: usize,
    lineno: usize,
    leng: usize,
    input_state: InputState,
    output_state: OutputState,
}
```

Use indices into `buffer_stack` instead of self-referential borrows.

#### Input/output handles
If `yyget_in` / `yyget_out` expose C `FILE *`-like state, represent only the minimum needed by this module:
- If the file handles are not actively manipulated here, store opaque placeholders or module-local abstractions.
- Prefer standard-library types only if actual reads/writes occur in this file.
- If accessors merely expose stored pointers in C, Rust equivalents should expose references or lightweight state handles without inventing new I/O layers.

### Scalar Mapping Rules

- `char *` buffer memory -> `Vec<u8>` / `&[u8]`
- mutable cursor pointers -> `usize` indices
- integer lengths/counts -> `usize`
- line numbers -> `usize` unless exact signed semantics must be preserved
- boolean flags -> `bool`
- nullable pointers -> `Option<T>` / `Option<usize>`

### Memory Management Decisions

- Replace manual allocation/free in buffer lifecycle functions with ownership-based creation and `Option`/`Vec` removal.
- `yy_delete_buffer` should clear owned storage safely and update current-buffer selection without leaving dangling references.
- `yypush_buffer_state` / `yypop_buffer_state` should operate on `Vec`-backed stack state.
- `yyunput` must guard against underflow when moving the cursor backward; preserve original behavior while expressing failures through internal panic/fatal path only where the C code does so.

### Error Handling Decisions

- Preserve fatal, non-recoverable paths with `yy_fatal_error(...) -> !`, implemented via `panic!` or process termination semantics consistent with current project conventions.
- Do not introduce `Result` return types to functions that are fatal-only in the C implementation unless required by surrounding Rust call sites.
- Accessor functions should remain infallible where the C code assumes initialized scanner state; if initialization may be absent, use clearly bounded internal checks.

## Implementation Phases

## Phase 1: Establish Rust module and state structures

- Create `src/module_src_c_c_20.rs`.
- Inspect `src/c.c` and identify all anonymous struct layouts actually used by the listed functions.
- Define named Rust structs/enums for scanner state, buffer state, and any status flags.
- Convert C globals/static mutable state used by this file into fields on `ScannerState`.
- Add minimal internal constructors/default initialization needed to support buffer lifecycle functions.
- Keep naming close to the original `yy*` conventions for traceability.

## Phase 2: Port buffer lifecycle and stack management functions

- Implement:
  - `yyrestart`
  - `yy_switch_to_buffer`
  - `yy_load_buffer_state`
  - `yy_delete_buffer`
  - `yy_flush_buffer`
  - `yypush_buffer_state`
  - `yypop_buffer_state`
- Translate pointer-based buffer operations into index-based updates on owned Rust data.
- Preserve ordering-sensitive side effects, especially current-buffer replacement and stack updates.
- Validate buffer deletion and pop behavior for empty-stack and current-buffer edge cases.
- Keep helper routines local to this module rather than introducing new support layers.

## Phase 3: Port cursor-sensitive scanner operations and fatal path

- Implement:
  - `yy_try_nul_trans`
  - `yyunput`
  - `yy_fatal_error`
- Translate C cursor arithmetic carefully into checked index movement.
- Preserve sentinel/NUL handling semantics from the original buffer layout.
- Use targeted internal assertions or fatal error calls where the C code relies on impossible-state assumptions.
- Confirm that no Rust borrow conflicts remain in state mutation paths.

## Phase 4: Port accessors and verify behavioral parity

- Implement:
  - `yyget_lineno`
  - `yyget_in`
  - `yyget_out`
  - `yyget_leng`
- Align return types with the actual migrated state fields.
- Add unit tests covering:
  - buffer switch/load/flush sequencing
  - push/pop stack behavior
  - delete of current and non-current buffer
  - unput cursor movement and boundary handling
  - accessor values after state changes
- Run `cargo test` and fix any state-lifetime or ownership regressions without expanding module scope.