# Implementation Plan

## Summary

Port the `src/c.c` logic for `yy_init_buffer` and `yy_init_globals` into a Rust module that preserves the existing initialization flow and state layout as closely as practical. The Rust implementation should focus on translating the current C-style scanner/global initialization behavior into explicit Rust data structures and functions without introducing new abstractions beyond what is needed for safe ownership and predictable mutation.

The technical approach is:

- create a single Rust module corresponding to this C module cluster
- model the C anonymous data structures as named Rust structs with fields migrated from the original layout
- translate global and buffer initialization routines into Rust functions/methods with explicit state passing instead of implicit mutable globals where possible
- keep allocation and defaulting behavior close to the C implementation, using `Option`, `Vec`, and standard initialization patterns to represent nullable pointers, owned buffers, and zero-initialized state
- preserve execution order and field initialization semantics so downstream scanner behavior remains compatible with the original implementation

## Technical Context

### Language/Version

- Rust 1.78+ edition 2021

### Primary Dependencies

- Rust standard library only

No third-party crates are recommended based on the available module evidence. The functionality described is limited to state and buffer initialization and can be implemented with standard library types.

### Testing

- `cargo test`

Testing should cover:

- global state initialization produces expected default values
- buffer initialization sets all required buffer fields consistently
- repeated initialization does not leave partially initialized state
- null/absent optional buffer state from the C version is represented safely in Rust

### Performance Goals

- match the original C module’s initialization cost closely
- avoid unnecessary heap allocations beyond those already implied by buffer ownership
- keep field initialization O(1) except where buffer sizing or zero-filling is required
- avoid cloning large buffers during setup

## Module Mapping

### C to Rust File Mapping

- `src/c.c` -> `src/module_src_yy_init_18.rs`

If the target crate already centralizes scanner logic in another file, this module should instead be added adjacent to that code and re-exported minimally, but the port should remain a single Rust source file for this C cluster.

### Function Mapping

- `yy_init_buffer` -> `pub(crate) fn yy_init_buffer(...)` or `impl ScannerState { fn init_buffer(...) }`
- `yy_init_globals` -> `pub(crate) fn yy_init_globals(...)` or `impl ScannerState { fn init_globals(...) }`

Preferred approach:

- place both functions on the scanner/state-holding struct when they primarily mutate shared lexer state
- use free functions only if surrounding ported modules already follow that pattern

### State Mapping Strategy

Because the original C module likely relies on mutable global scanner state and buffer records:

- gather all related mutable global fields into one Rust state struct
- pass `&mut` references explicitly between initialization functions
- represent C nullable pointers as `Option<T>` or `Option<Box<T>>`
- represent owned byte buffers with `Vec<u8>` when Rust owns memory, or slices/references if ownership clearly belongs elsewhere

## Data Model

The input only identifies multiple anonymous C data structures, so the Rust plan should assign stable names based on usage rather than preserving anonymity.

### Struct Mapping Principles

- each anonymous C struct used by `yy_init_buffer` or `yy_init_globals` becomes a named Rust struct
- if several anonymous structs are variants of a single state concept, consolidate only when field usage clearly matches
- avoid speculative redesign; keep a close one-to-one field mapping where possible

### Expected Rust Data Structures

#### 1. Global scanner state

- C: anonymous global/state struct(s)
- Rust: `ScannerState`

Purpose:

- holds the fields initialized by `yy_init_globals`
- centralizes former C global mutable state

Likely Rust field styles:

- scalar counters/flags -> `usize`, `i32`, `bool`
- nullable pointers -> `Option<...>`
- current buffer pointer/reference -> `Option<BufferState>`
- file/input handles -> only migrate as-is if directly present in the C fields visible to this module; otherwise keep as placeholders to be filled during detailed port

#### 2. Buffer state

- C: anonymous buffer-related struct(s)
- Rust: `BufferState`

Purpose:

- stores the fields initialized by `yy_init_buffer`
- tracks buffer content metadata and buffer status flags

Likely Rust field styles:

- raw character buffer -> `Vec<u8>` or borrowed mutable slice depending on original ownership
- buffer size -> `usize`
- position indices -> `usize`
- status flags -> `bool` or small enums
- input source references -> `Option<...>`

#### 3. Auxiliary status/flag groups

- C: remaining anonymous structs, if they are nested records
- Rust: small named structs or enums such as:
  - `BufferFlags`
  - `InputSourceState`
  - `LexerPositions`

These should only be introduced when the C layout clearly groups fields. Otherwise, keep fields flattened into `ScannerState` or `BufferState` to reduce migration risk.

### C-to-Rust Type Conversion Rules

- `char *` / byte buffer pointers -> `Vec<u8>`, `&[u8]`, `&mut [u8]`, or `Option<NonNull<u8>>` only if direct raw-pointer semantics are unavoidable
- `int` -> `i32` unless used for sizes or indexing, then convert to `usize` with care
- unsigned sizes/counts -> `usize`
- sentinel integer flags -> `bool` or small enum when values are known and limited
- nested anonymous structs -> named `struct`
- null object references -> `Option<T>`

### Memory Management Decisions

- replace manual zeroing with `Default` implementations where default values are semantically correct
- use `Option` instead of null pointers
- use owned Rust buffers for any memory allocated by initialization functions
- if the original function only initializes externally owned memory, accept `&mut BufferState` and mutate in place rather than reallocating
- isolate any unavoidable `unsafe` to narrow field-translation boundaries; avoid `unsafe` if standard types can express the same layout/behavior

### Error Handling Decisions

The C functions may have relied on implicit success/failure through state mutation. In Rust:

- return `Result<(), InitError>` only when initialization can actually fail due to allocation or invalid input assumptions
- if the C logic is infallible once valid state objects exist, keep signatures infallible and encode optional state explicitly
- avoid broad custom error frameworks; a small module-local error enum is sufficient if needed

## Implementation Phases

## Phase 1: Establish Rust module and state skeleton

Scope:

- create the Rust file for this module cluster
- identify all fields touched by `yy_init_globals` and `yy_init_buffer`
- define named Rust structs for the relevant anonymous C structures
- add `Default` implementations or constructors mirroring C zero/default initialization

Deliverables:

- `src/module_src_yy_init_18.rs`
- initial `ScannerState` and `BufferState` definitions
- placeholder mapping comments tying Rust fields back to C usage during migration

Acceptance criteria:

- module compiles with struct definitions in place
- all state previously implied by C globals involved in these two functions has an explicit Rust home
- no extra helper subsystems are introduced

## Phase 2: Port `yy_init_globals`

Scope:

- translate `yy_init_globals` field-by-field into Rust
- replace implicit global resets with explicit mutation on `ScannerState`
- preserve initialization ordering and sentinel/default values

Implementation notes:

- use `ScannerState::init_globals(&mut self)` if state-centric
- ensure nullable/resettable members become `None`
- ensure counters, flags, and current-buffer references match C defaults

Acceptance criteria:

- unit tests verify all fields affected by global initialization are reset correctly
- repeated calls produce the same stable initialized state
- no leaked ownership or lingering stale references remain after reset

## Phase 3: Port `yy_init_buffer`

Scope:

- translate buffer initialization logic into Rust against `BufferState`
- preserve buffer metadata setup, position resets, and status flags
- connect buffer initialization to the global scanner state only where the C function actually does so

Implementation notes:

- prefer `fn init_buffer(buffer: &mut BufferState, ...)` or `ScannerState::init_buffer(...)`
- if the original code depends on external buffer storage, represent that explicitly rather than copying unnecessarily
- validate index and size conversions when moving from C integers to Rust `usize`

Acceptance criteria:

- unit tests verify initialized buffer state matches expected defaults
- buffer reinitialization behaves deterministically
- optional or absent backing state is handled safely without null dereference patterns

## Phase 4: Integration cleanup and verification

Scope:

- connect this module to the surrounding Rust port with minimal exports
- remove temporary translation comments that are no longer needed
- finalize tests for interaction between global and buffer initialization

Testing focus:

- initializing globals before buffer setup
- reinitializing globals after buffer state exists
- initializing multiple buffers sequentially if supported by the original state model

Acceptance criteria:

- `cargo test` passes
- module API surface is limited to what the existing port needs
- implementation remains constrained to the behavior of the original two C functions