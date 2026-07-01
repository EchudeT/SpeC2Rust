# Implementation Plan: module_src_yy_init_18

## Summary

This module migration covers the C initialization logic currently located in `src/c.c` for the functions `yy_init_buffer` and `yy_init_globals`. The Rust implementation should preserve the existing initialization order, state defaults, and buffer/global setup behavior without introducing new runtime features or broader refactoring outside this module boundary.

The technical approach is to translate the relevant C state and initialization routines into Rust-owned data structures and explicit initializer functions. Where the C code relies on zeroed memory, mutable globals, or implicit lifetime rules, the Rust version should replace these with typed structs, `Default` implementations where appropriate, and explicit mutable state passed through functions. The implementation should stay close to the original control flow so that behavior remains easy to validate against the C source.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve the constant-time nature of initialization paths.
  - Avoid unnecessary heap allocation beyond what is required by the original buffer setup behavior.
  - Keep state initialization predictable and close to C runtime cost.
  - Do not introduce additional abstraction layers that materially change startup or buffer initialization overhead.

## Module Mapping

### Source Mapping

- **C source file**
  - `src/c.c`

- **Rust target module**
  - `src/module_src_yy_init_18.rs`

### Function Mapping

- `yy_init_buffer`
  - Migrate to a Rust function in `src/module_src_yy_init_18.rs`
  - Keep responsibilities limited to buffer-state initialization and reset of fields managed by the C function
  - Prefer taking explicit mutable references to buffer/global state rather than relying on process-global mutable storage

- `yy_init_globals`
  - Centralize initialization of scanner/global fields that are reset by the C implementation
  - Use typed defaults and explicit assignment for fields whose initialization values matter semantically

### Scope Boundaries

- Only migrate logic needed for:
  - buffer initialization
  - scanner/global initialization
- Do not expand into unrelated lexer behavior, tokenization logic, I/O abstraction, or extra helper subsystems unless directly required to express these two functions safely in Rust.

## Data Model

The input analysis identifies multiple anonymous C data structures. Because they are unnamed in the analysis, the Rust plan should first assign stable internal names based on their usage in `yy_init_buffer` and `yy_init_globals`, then migrate only the structures required by these functions.

### Data-Structure Mapping Strategy

- **anonymous -> Rust named struct**
  - Each anonymous C struct referenced by these functions should receive a purpose-based Rust name.
  - Suggested naming pattern:
    - `YyBufferState` for the buffer record used by `yy_init_buffer`
    - `YyGlobals` or `YyScannerState` for global/scanner-level mutable state used by `yy_init_globals`
    - Additional anonymous records should be named only if they are directly accessed by these two functions

### Expected Rust Representations

#### Buffer-related C struct
- **C**: anonymous buffer state struct
- **Rust**: `struct YyBufferState`
- **Mapping notes**:
  - Convert scalar flags and counters to Rust integer/bool types matching the C width where behavior depends on size.
  - Convert raw character buffer pointers into safer representations where ownership is clear:
    - borrowed slice or mutable slice if storage is externally owned
    - `Vec<u8>` or `Box<[u8]>` only if the original function owns allocation for the buffer contents
  - Preserve sentinel/end-of-buffer conventions explicitly if the C code depends on them.

#### Global/scanner C struct
- **C**: anonymous scanner/global state struct
- **Rust**: `struct YyGlobals` or `struct YyScannerState`
- **Mapping notes**:
  - Replace mutable global variables with fields on a single state struct.
  - Group all fields reset by `yy_init_globals` into this struct even if the original C code used separate globals.
  - Use `Option<T>` for nullable pointers/references from C.
  - Use `Default` only where zero/default semantics match the C initialization behavior.

#### Auxiliary anonymous structs
- **C**: additional anonymous structs listed by analysis
- **Rust**: named case-by-case only if touched by these functions
- **Mapping notes**:
  - Avoid creating Rust types for anonymous C structs not involved in this module slice.
  - If a nested record is only stored or forwarded, preserve layout semantically rather than recreating all unused fields immediately.
  - For fields that are opaque to this module, represent them minimally and expand only if compilation requires it.

### Type Conversion Rules

- `char *` / buffer pointers
  - Map to `*mut u8`, `&mut [u8]`, or owned byte storage depending on actual ownership in the C code
  - Prefer slices/references where safe borrowing is expressible

- nullable pointers
  - Map to `Option<&mut T>`, `Option<&T>`, or `Option<NonNull<T>>` depending on aliasing/lifetime constraints
  - Use raw pointers only where the original structure cannot yet be safely expressed otherwise

- integer flags
  - Map to `bool` when values are strictly logical
  - Otherwise preserve as `i32`, `u32`, `usize`, or other matching integer types if exact sentinel values are used

- zero-initialized storage
  - Replace with `Default` or explicit constructor functions
  - Avoid `mem::zeroed()` except where the final type is plain data and no safer constructor is viable

## Implementation Phases

## Phase 1: Isolate State and Define Rust Data Types

- Create `src/module_src_yy_init_18.rs`.
- Identify all fields read or written by `yy_init_buffer` and `yy_init_globals` in `src/c.c`.
- Define the minimum Rust structs required to represent:
  - buffer state
  - scanner/global state
  - any directly referenced nested records
- Assign stable Rust names to the anonymous C structs based on usage.
- Introduce explicit constructors or `Default` implementations only for structures whose initialization semantics are fully known from the C code.
- Decide ownership for buffer memory:
  - borrowed if managed elsewhere
  - owned only if the original initialization logic implies module ownership

## Phase 2: Port `yy_init_globals`

- Implement the Rust equivalent of `yy_init_globals` using the new global/scanner state struct.
- Translate each C global reset into explicit field initialization.
- Replace C null-state handling with `Option` or explicit default field values.
- Preserve any required initialization ordering if later fields depend on earlier resets.
- Add focused unit tests covering:
  - fresh global state initialization
  - re-initialization/reset behavior
  - nullable/empty state cases reflected in the C logic

## Phase 3: Port `yy_init_buffer`

- Implement the Rust equivalent of `yy_init_buffer` using `&mut YyBufferState` and any required mutable scanner/global context.
- Translate buffer flags, position markers, counters, and end-of-buffer setup directly from the C control flow.
- Preserve sentinel placement and buffer status rules exactly where they affect later scanner behavior.
- Handle memory safely:
  - validate borrowed buffer length before writing sentinels or indices
  - avoid unchecked indexing unless invariants are established locally
- Add focused unit tests covering:
  - initialization of a new buffer state
  - reinitialization of an existing buffer
  - minimal valid buffer sizes and sentinel handling
  - interaction with global/scanner state if required by the C logic

## Phase 4: Integrate and Verify Module Equivalence

- Wire the new Rust module into the crate using standard Rust module declarations only as needed.
- Remove or avoid duplicate initialization logic elsewhere in the Rust port.
- Run `cargo test` and resolve any mismatches in default values, field widths, or buffer boundary behavior.
- Perform a final review to ensure:
  - no extra API surface was added beyond what these functions require
  - memory ownership is explicit
  - nullability and reset semantics match the original C behavior
  - the migration remains limited to the existing file/function responsibilities