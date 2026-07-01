# Implementation Plan: module_src_yy_buffer_state_11

## Summary

This module migrates the buffer-management portion of the lexer runtime from `src/c.c` into Rust, specifically the logic currently centered on `yy_create_buffer`, `yyensure_buffer_stack`, and `yy_scan_buffer`.

The Rust implementation should preserve the existing control flow and ownership model as closely as possible while replacing manual allocation and pointer arithmetic with standard-library containers and explicit state types. The main technical approach is:

- move the relevant `yy_buffer_state`-related state into a dedicated Rust module;
- represent buffer storage with owned byte containers where Rust owns memory, and with bounded borrowed or transferred storage when the C code expects scanning over caller-provided bytes;
- convert implicit global/buffer-stack mutations into explicit mutable state updates on a lexer/buffer manager struct;
- keep behavior aligned with the current C implementation rather than redesigning lexer architecture.

The implementation should focus narrowly on migrating the existing file-local functionality and the immediate state it requires, without introducing additional abstractions beyond what is necessary to replace C memory management safely.

## Technical Context

### Language/Version
- Rust stable, edition 2021
- Minimum recommended compiler: Rust 1.76 or newer

### Primary Dependencies
- Rust standard library only
- No third-party crates are required based on the current module evidence

### Testing
- `cargo test`

### Performance Goals
- Match the current C module’s asymptotic behavior for buffer creation, stack growth, and buffer scanning
- Avoid unnecessary copying except where required to establish Rust ownership safely
- Preserve amortized growth behavior for the buffer stack using `Vec`
- Keep per-call overhead minimal and suitable for lexer hot paths

## Module Mapping

### Source Mapping
- C source: `src/c.c`
- Rust target: `src/module_src_yy_buffer_state_11.rs`

### Function Mapping
- `yy_create_buffer`
  - Migrate to a Rust constructor/helper that creates and initializes a buffer-state instance
- `yyensure_buffer_stack`
  - Migrate to a Rust method/helper that ensures stack capacity using `Vec`
- `yy_scan_buffer`
  - Migrate to a Rust method/helper that validates and installs a scan buffer over provided bytes/state

### Ownership and State Placement
Because these functions are tightly coupled through shared lexer state in the C file, they should be grouped in one Rust module and implemented against a single mutable state holder rather than as unrelated free functions. The Rust module should contain:

- buffer-state type(s)
- lexer-local buffer stack state
- the migrated implementations of the three functions

This keeps migration close to the original file organization and avoids spreading the port across multiple new modules.

## Data Model

The input analysis only exposes anonymous C data structures, so the Rust plan should map them by role as discovered in `src/c.c` during implementation, keeping names aligned to their actual C usage.

### Core Mappings

#### C anonymous struct used as `yy_buffer_state`
- Rust: `struct YyBufferState`
- Expected fields to migrate by role:
  - underlying buffer bytes
  - current size/capacity markers
  - position/index markers used by scan/create logic
  - ownership flags if present in C
  - status booleans currently encoded as integer fields in C

#### C anonymous struct holding lexer/global scanner state
- Rust: `struct LexerState`
- Expected fields to migrate by role:
  - current buffer reference/index
  - buffer stack
  - stack top/current position
  - stack capacity bookkeeping
  - any adjacent fields directly read or mutated by the three target functions

### Field-Level Mapping Rules

#### Raw buffer memory
- C: `char *`, `unsigned char *`, or equivalent byte pointers
- Rust:
  - `Vec<u8>` when Rust owns allocated buffer contents
  - `Box<[u8]>` if fixed ownership after creation is simpler
  - `&[u8]` / `&mut [u8]` only if the original function semantics clearly operate on caller-owned memory without transferring ownership and lifetimes remain local
- Decision rule:
  - use owned storage by default unless `yy_scan_buffer` must model externally supplied memory exactly

#### Stack storage
- C: manually grown array of buffer pointers/states
- Rust: `Vec<...>`
- Stack top/capacity integers should be replaced with `Vec` length/capacity where possible, keeping explicit index fields only when needed to preserve behavior

#### Optional/current buffer references
- C: nullable pointers
- Rust:
  - `Option<usize>` for current-stack index when stack-owned
  - `Option<YyBufferState>` only if no stack indirection is needed
- Prefer index-based access if the C logic depends on a stable current buffer slot within a stack

#### Integer flags and booleans
- C: `int` flag fields
- Rust: `bool` where binary semantics are clear
- Keep integer types only when values are not boolean or participate in sentinel arithmetic

#### Sizes and indices
- C: `int`, `size_t`
- Rust:
  - `usize` for lengths, capacities, and indexes
  - signed integer types only if negative sentinel values are required by preserved logic

### Error Handling Mapping

- C allocation failure / null-return patterns
  - Rust: `Result<_, BufferError>` for fallible constructors/helpers when failure is meaningful to callers
- C validity checks in `yy_scan_buffer`
  - Rust: explicit validation returning `Result` or `Option`, depending on how the original caller distinguishes invalid buffers
- Internal invariants that must hold if the surrounding port is correct
  - Rust: `debug_assert!` for developer checks, not new recovery logic

### Memory Management Decisions

- Replace manual heap allocation/free with RAII-owned Rust structures
- Avoid exposing raw pointers internally unless unavoidable for compatibility with adjacent unmigrated code
- If temporary raw-pointer interop is necessary during staged migration, isolate it at module boundaries and keep the internal representation safe
- Preserve any sentinel-byte requirements from the C scanner buffer format explicitly in validation logic rather than relying on unchecked memory layout assumptions

## Implementation Phases

### Phase 1: Extract and model the relevant state
- Inspect `src/c.c` and identify the exact anonymous structs and fields touched by:
  - `yy_create_buffer`
  - `yyensure_buffer_stack`
  - `yy_scan_buffer`
- Define `YyBufferState` and `LexerState` in `src/module_src_yy_buffer_state_11.rs`
- Map C scalar fields to Rust scalar types conservatively
- Introduce buffer stack representation with `Vec`
- Keep names and field grouping close to original roles to simplify verification against the C source

### Phase 2: Port buffer creation and stack growth logic
- Implement the Rust equivalent of `yyensure_buffer_stack`
  - replace manual capacity checks/reallocation with `Vec` growth
  - preserve observable stack initialization behavior
- Implement the Rust equivalent of `yy_create_buffer`
  - create buffer-state instances
  - initialize size markers, sentinel-related space, and any ownership/status fields
  - return explicit errors instead of null on allocation/validation failure
- Add unit tests covering:
  - empty-state stack initialization
  - repeated stack growth
  - buffer creation with expected sizes and default field values

### Phase 3: Port scan-buffer installation logic
- Implement the Rust equivalent of `yy_scan_buffer`
  - validate input buffer layout and sentinel requirements from the C logic
  - construct or register the corresponding `YyBufferState`
  - update lexer state to reference the scanned buffer as required by the original implementation
- Ensure ownership semantics are explicit:
  - whether the scanned bytes are copied into owned storage or wrapped with constrained borrowing/owned boxed storage based on actual C behavior
- Add tests covering:
  - valid scan-buffer setup
  - invalid input buffer rejection
  - interaction with existing stack state and current-buffer selection

### Phase 4: Integrate and verify against surrounding ported code
- Replace the relevant logic in the Rust port branch with calls to the new module functions/methods
- Remove any temporary unsafe or placeholder representations used during migration
- Run `cargo test` and adjust signatures/types to match neighboring migrated lexer code
- Perform final review for:
  - no leaked C allocation patterns
  - no unchecked null-style state transitions
  - behavior preserved for buffer-stack and scan-buffer paths