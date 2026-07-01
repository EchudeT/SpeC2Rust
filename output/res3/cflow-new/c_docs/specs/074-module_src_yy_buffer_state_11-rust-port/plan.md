# Implementation Plan: module_src_yy_buffer_state_11

## Summary

Port the buffer-management portion of `src/c.c` covering `yy_create_buffer`, `yyensure_buffer_stack`, and `yy_scan_buffer` into a focused Rust module that preserves the existing control flow and ownership model as closely as practical.

The Rust implementation should:
- migrate the existing buffer-state logic without introducing new abstraction layers beyond what is needed for safety,
- represent the scanner buffer state explicitly with Rust structs,
- replace manual heap allocation and raw buffer bookkeeping with `Vec<u8>` and owned state where possible,
- keep API boundaries narrow and aligned to the original function grouping.

The implementation approach is to isolate the buffer-state operations into a single Rust module, translate the participating C data structures into concrete Rust types, and preserve the original ordering and side effects of buffer creation, stack growth, and in-memory buffer scanning. Memory ownership and sentinel handling for scanned buffers must be made explicit so that the Rust port remains behaviorally equivalent while avoiding unsafe pointer arithmetic except where unavoidable.

## Technical Context

### Language / Version
- Rust 1.78+ stable

### Primary Dependencies
- Rust standard library only
- No third-party crates are recommended based on the current module evidence

### Testing
- `cargo test`

### Performance Goals
- Maintain linear-time behavior for buffer allocation and initialization operations
- Preserve amortized growth behavior for buffer stack expansion
- Avoid unnecessary byte copying beyond what is required by `yy_scan_buffer`
- Keep per-buffer metadata compact and owned by Rust structures
- Match C module behavior closely enough that scanner-facing overhead stays negligible relative to input processing

## Module Mapping

### Source Mapping
- C source file: `src/c.c`
- Rust target file: `src/module_src_yy_buffer_state_11.rs`

### Function Mapping
- `yy_create_buffer` -> `yy_create_buffer` in `src/module_src_yy_buffer_state_11.rs`
- `yyensure_buffer_stack` -> `yyensure_buffer_stack` in `src/module_src_yy_buffer_state_11.rs`
- `yy_scan_buffer` -> `yy_scan_buffer` in `src/module_src_yy_buffer_state_11.rs`

### Integration Mapping
- Expose only the migrated buffer-state functions and the minimum required buffer-state types
- Keep scanner-global or scanner-instance state in the same Rust module if these functions directly mutate it
- If the branch already contains surrounding scanner code, wire this module into the existing crate module tree using a single `mod module_src_yy_buffer_state_11;` entry and direct imports only where needed

## Data Model

Because the analysis reports only anonymous C structures, the Rust port should derive concrete names from usage within these three functions rather than inventing broader domain types.

### Struct Mapping
- C anonymous buffer state struct -> Rust `struct YyBufferState`
- C anonymous scanner/global state struct used for current buffer stack management -> Rust `struct ScannerState` or the existing crate-local scanner state type if already present
- C anonymous stack container fields for buffer pointers/count/capacity -> Rust fields on `ScannerState`
- C anonymous input buffer memory region -> Rust `Vec<u8>` for owned buffers, or a dedicated enum for owned vs externally supplied memory if `yy_scan_buffer` must reference caller-provided storage without copying

### Recommended Rust Shapes

#### `YyBufferState`
Expected fields should be migrated according to actual usage in the C functions, likely including:
- underlying byte storage or reference to scanned storage,
- buffer size,
- number of valid characters,
- current position/index if touched by these functions,
- ownership flag indicating whether storage should be freed/dropped,
- flags for interactivity, beginning-of-line status, or fill behavior only if referenced by the migrated code.

Suggested shape:
```rust
struct YyBufferState {
    buf: BufferStorage,
    size: usize,
    n_chars: usize,
    is_our_buffer: bool,
    // additional migrated flags/indices only if used by these functions
}
```

#### `BufferStorage`
Use an enum only if needed by `yy_scan_buffer` semantics:
```rust
enum BufferStorage {
    Owned(Vec<u8>),
    External { ptr_like_len_checked: Vec<u8> },
}
```

If the wider port can safely normalize all scanned buffers into owned storage without changing required semantics, prefer:
```rust
type BufferStorage = Vec<u8>;
```

#### `ScannerState`
Suggested fields:
```rust
struct ScannerState {
    buffer_stack: Vec<YyBufferState>,
    current_buffer: Option<usize>,
}
```

If the original C code requires pointer-stable references to buffer states, use:
```rust
struct ScannerState {
    buffer_stack: Vec<Box<YyBufferState>>,
    current_buffer: Option<usize>,
}
```

### C-to-Rust Type Conversions
- `char *` / buffer pointer -> `Vec<u8>` or borrowed byte slice representation depending on ownership
- `int` sizes/counts -> `usize` where values are non-negative and index-related
- allocation flags -> `bool`
- nullable buffer pointers -> `Option<...>`

### Memory Management Decisions
- Replace `malloc`/`realloc`/`free` for stack storage with `Vec`
- Replace per-buffer heap allocation with owned Rust values
- For `yy_scan_buffer`, explicitly validate sentinel bytes and minimum size before constructing the Rust buffer state
- If caller-provided buffer ownership must remain external, represent that state explicitly and avoid freeing caller memory
- Avoid raw pointers unless required by existing crate interfaces; if unavoidable, confine unsafe usage to small conversion boundaries with documented invariants

### Error Handling Decisions
- Functions that can fail due to invalid buffer layout or allocation-related state construction should return `Result<_, BufferError>` if surrounding migrated code permits
- If the existing scanner API requires null-style failure signaling, use `Option` or `Result` internally and translate at the narrow API edge
- Define only minimal error variants needed by these functions, such as:
  - invalid scan buffer sentinel layout,
  - insufficient buffer length,
  - inconsistent scanner stack state

## Implementation Phases

## Phase 1: Extract and Define Core Buffer Types
- Inspect `src/c.c` usage for the anonymous structs touched by the three target functions
- Introduce `src/module_src_yy_buffer_state_11.rs`
- Define `YyBufferState` and the minimal scanner state fields required for buffer stack management
- Translate constant values, sentinel requirements, and field defaults used during buffer creation
- Decide whether `buffer_stack` stores values directly or boxed values based solely on pointer-stability requirements from the migrated code

### Deliverables
- Rust module file with struct and enum definitions
- Minimal type aliases/constants needed by the three functions
- Unit-test skeletons for buffer state construction defaults

## Phase 2: Port Buffer Creation and Stack Growth
- Port `yyensure_buffer_stack` first so stack allocation/growth behavior exists before buffer insertion
- Implement stack initialization and expansion using `Vec`, preserving original semantics for empty-stack setup and capacity increases
- Port `yy_create_buffer` next, mapping buffer allocation and initialization into owned Rust state
- Preserve any original extra sentinel space allocation and default field initialization
- Keep failure behavior aligned with the original API shape expected by callers

### Deliverables
- Working Rust implementations of:
  - `yyensure_buffer_stack`
  - `yy_create_buffer`
- Tests covering:
  - initial empty stack creation,
  - repeated stack growth,
  - created buffer size and sentinel capacity,
  - ownership/default flag initialization

## Phase 3: Port In-Memory Scan Buffer Handling
- Implement `yy_scan_buffer` using the Rust buffer model selected in Phase 1
- Validate input buffer length and required end-of-buffer sentinel bytes before accepting the buffer
- Construct a `YyBufferState` with correct ownership semantics and character count
- Ensure the returned buffer state integrates correctly with the scanner stack/current-buffer conventions already ported
- Keep any unsafe code limited to interface boundaries if existing callers still use raw memory

### Deliverables
- Working Rust implementation of `yy_scan_buffer`
- Tests covering:
  - valid scanned buffer acceptance,
  - rejection of undersized buffers,
  - rejection of missing sentinel bytes,
  - correct non-owning vs owning behavior according to migrated semantics

## Phase 4: Integrate and Verify Module Behavior
- Wire the new module into the crate using standard Rust module declarations
- Update existing call sites in the branch to use the Rust implementations for these three functions
- Remove or isolate the corresponding C-side logic from the active Rust path
- Run `cargo test` and add focused regression tests for interactions between create/ensure/scan flows
- Confirm that no extra support layers or unrelated scanner features were introduced during the migration

### Deliverables
- Integrated Rust module in the crate build
- Passing unit tests for the migrated functions
- Regression coverage for stack state transitions and scanned-buffer setup