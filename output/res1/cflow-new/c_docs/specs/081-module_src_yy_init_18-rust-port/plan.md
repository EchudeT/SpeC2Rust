# Implementation Plan

## Summary

Port the C module logic in `src/c.c` for `yy_init_buffer` and `yy_init_globals` into a Rust module that preserves current initialization behavior, mutable state transitions, and buffer setup semantics without adding new capabilities.

The Rust implementation should follow the existing C control flow closely:
- translate global and buffer initialization routines directly,
- represent C state explicitly in Rust structs,
- keep initialization order and default values aligned with the source behavior,
- use `Option`, owned storage, and explicit mutable references to replace nullable pointers and implicit global mutations where possible.

Because the analyzed surface is limited to two initialization functions and unnamed C data structures, the plan should focus on a narrow migration of the involved state containers and helper logic required by these functions only. The port should avoid architectural expansion and keep file/module boundaries minimal.

## Technical Context

- **Language/Version**: Rust 1.78+ edition 2021
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Maintain initialization overhead comparable to the C implementation.
  - Avoid unnecessary heap allocations beyond what is required to represent buffer/state ownership safely in Rust.
  - Preserve constant-time initialization steps for global state and buffer metadata.
  - Keep data layout simple and mutation direct to avoid avoidable abstraction costs.

## Module Mapping

### C to Rust File Mapping

- `src/c.c`
  - migrate relevant `yy_init_buffer` and `yy_init_globals` logic into:
    - `src/module_src_yy_init_18.rs` if integrating as a focused module file, or
    - `src/lib.rs` / existing module tree entry if the project already centralizes translated lexer/runtime state there

Preferred mapping is a single Rust module file named after the migration unit to keep scope narrow:
- `src/module_src_yy_init_18.rs`

### Function Mapping

- `yy_init_buffer`
  - Rust: `fn yy_init_buffer(buffer: &mut YyBufferState, input: InputHandle, globals: &mut YyGlobals)` or equivalent narrow signature based on actual surrounding state ownership
- `yy_init_globals`
  - Rust: `fn yy_init_globals(globals: &mut YyGlobals)`

If the original C implementation relies on process-wide globals, consolidate those fields into a single explicit Rust state struct and pass mutable references rather than recreating implicit global access.

## Data Model

The analysis reports only anonymous C data structures, so the Rust data model should be derived strictly from fields actually touched by `yy_init_buffer` and `yy_init_globals`.

### Mapping Strategy

- **Anonymous C structs used for lexer/global state**
  - Rust: named structs with focused field sets, for example:
    - `YyGlobals`
    - `YyBufferState`
    - additional small support structs only if required by accessed fields

- **C pointers**
  - `T *` -> `Option<Box<T>>`, `Option<&mut T>`, or owned struct fields depending on lifetime/ownership in the original code
  - non-owning nullable references -> `Option<usize>` or `Option<NonNull<T>>` only if necessary; prefer safe references and ownership restructuring inside the translated module

- **C character/input buffers**
  - `char *` / byte storage -> `Vec<u8>` or borrowed byte slice, depending on whether the buffer is owned and mutated by initialization
  - sentinel/end markers -> explicit bytes appended only if required by the source logic

- **C integral flags and counters**
  - `int`, `size_t`, similar scalar state -> `i32`, `usize`, or narrower integer types matching actual usage
  - boolean flags -> `bool` where semantics are binary

- **C FILE/input handles**
  - If only stored as opaque handles by these functions, represent with a narrow Rust abstraction already used by the project.
  - If no existing abstraction is available and behavior is limited to initialization-time storage, define a minimal `InputHandle` placeholder type tied only to these functions' needs.

### Proposed Rust Structures

Exact fields should be limited to those accessed by the two migrated functions.

```rust
struct YyGlobals {
    // translated former global lexer state fields touched by yy_init_globals
}

struct YyBufferState {
    // translated buffer metadata touched by yy_init_buffer
}
```

If the C code initializes nested anonymous records, flatten them into the nearest owning Rust struct unless preserving a nested shape is necessary to keep field mapping readable.

### Memory Management Notes

- Replace null-initialized pointers with `Option`.
- Prefer owned Rust storage for buffer contents and state containers.
- Keep borrowing rules explicit: `yy_init_buffer` should mutate the buffer and any shared lexer/global state through `&mut` references.
- Avoid unsafe code unless the surrounding project interface forces raw-pointer interop. If unsafe is unavoidable, isolate it to the thinnest conversion boundary and keep initialization logic safe.

### Error Handling Notes

If the original initialization functions are infallible, keep them infallible in Rust.
If initialization depends on allocatable storage or external handles and the C code signals failure through return codes/null checks, map this to:
- `Result<(), InitError>` for recoverable setup failure, or
- internal assertions only where the original code assumes valid preconditions and project conventions support that assumption.

Do not introduce broader error frameworks; use standard library error types or a small local enum.

## Implementation Phases

## Phase 1: Extract and Map C State

- Inspect `src/c.c` and isolate all fields read or written by:
  - `yy_init_buffer`
  - `yy_init_globals`
- Identify which anonymous C structs correspond to:
  - buffer state,
  - global lexer/runtime state,
  - any nested metadata directly referenced by these functions
- Define minimal named Rust structs for those fields only.
- Decide final ownership model for:
  - buffer storage,
  - input handle storage,
  - mutable global state access

### Deliverables
- Rust module file created
- Named Rust state structs declared
- Field-by-field mapping notes embedded as code comments during migration

## Phase 2: Port Initialization Logic

- Implement `yy_init_globals` first, because it establishes baseline state and default values.
- Port `yy_init_buffer` second, preserving:
  - initialization order,
  - flag defaults,
  - buffer position/state reset behavior,
  - any end-of-buffer marker setup,
  - attachment of input source references/handles
- Keep translated logic structurally similar to the C implementation to reduce semantic drift.
- Replace C null checks and direct pointer writes with `Option` and mutable field assignment.

### Deliverables
- Working Rust implementations of both functions
- No new public behavior beyond the original initialization routines
- Minimal helper functions only if needed to keep direct translation readable

## Phase 3: Integrate With Surrounding Module State

- Connect the new Rust functions to the existing crate module tree on branch `081-module_src_yy_init_18-rust-port`.
- Update any call sites involved in this migration unit so they pass explicit mutable state instead of relying on C-style globals.
- Keep integration limited to the functions and state required by this module.
- Remove or avoid duplicate transitional representations once the Rust path is wired.

### Deliverables
- Functions callable through the crate's normal module structure
- Call-site compilation complete for the migrated surface
- No redundant parallel initialization path inside the Rust module

## Phase 4: Verification and Cleanup

- Add focused unit tests covering:
  - default global state after `yy_init_globals`
  - buffer field reset/default values after `yy_init_buffer`
  - repeated initialization on an existing buffer state
  - null/optional input-handle scenarios only if present in the C logic
- Compare behavior against the C source expectations for initialized field values and state transitions.
- Remove migration comments that are no longer needed, keeping only concise notes where field naming differs materially from C.

### Deliverables
- `cargo test` passing
- Initialization behavior validated against source semantics
- Final Rust module kept narrow and aligned with original C responsibilities