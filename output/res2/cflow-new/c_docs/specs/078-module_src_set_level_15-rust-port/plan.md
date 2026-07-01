# Implementation Plan: module_src_set_level_15

## Summary

This module cluster covers the migration of the C functions `set_level_indent` and `set_level_mark` from `src/main.c` and `src/output.c` into Rust, preserving their current role in output-level formatting and marker selection logic. The Rust implementation should keep behavior aligned with the existing C code path, with focus on direct translation of control flow, state updates, and string/character handling rather than redesign.

The technical approach is to port the relevant formatting state and helper logic into a small Rust module that operates on existing program state using explicit mutable references. C global or file-local state used by these functions should be mapped into Rust structs or fields on an existing context type, avoiding new abstraction layers unless required by ownership rules. String and byte-oriented formatting decisions should use standard library types such as `String`, `&str`, and `Vec<u8>` only where the original C usage requires owned or mutable buffers.

## Technical Context

- **Language/Version**: Rust 1.78 or newer
- **Primary Dependencies**:
  - Rust standard library only
- **Testing**:
  - `cargo test`
- **Performance Goals**:
  - Maintain output-generation performance comparable to the C implementation for level-formatting paths
  - Avoid unnecessary heap allocation in repeated indentation/mark-setting operations
  - Preserve simple constant-time state updates where the C code uses direct assignment or pointer replacement

## Module Mapping

### C Source to Rust Source

- `src/main.c`
  - Migrate `set_level_indent`
  - Place in the Rust module responsible for retained program state and top-level formatting configuration, preferably within the existing main translation target such as `src/main.rs` or a closely scoped sibling module if already present in the Rust project
- `src/output.c`
  - Migrate `set_level_mark`
  - Place in the Rust module responsible for output formatting behavior, preferably `src/output.rs` if the Rust port already mirrors the C file split

### Function Mapping

- `set_level_indent` (C)
  - Rust function with the same narrow responsibility
  - Prefer signature using explicit mutable access to the formatting/context state instead of hidden globals
- `set_level_mark` (C)
  - Rust function with equivalent state mutation and validation behavior
  - Keep return behavior aligned with the C function’s observable contract, using `Result` only if the C path has actual failure signaling that must be preserved

### Migration Scope

Only migrate the two identified functions and the minimum state they require from the two listed C files. Do not broaden the plan into unrelated output or parsing facilities.

## Data Model

The analysis lists only anonymous C data structures, so the Rust data model should be derived strictly from actual field usage reached by `set_level_indent` and `set_level_mark`.

### Mapping Strategy

- **Anonymous C structs used only for module-local formatting state**
  - Map to named Rust `struct`s with fields matching the accessed members only
- **Anonymous C unions or flag-style variants, if present**
  - Map to Rust `enum` only when the original usage is truly variant-based
  - Otherwise keep a plain `struct` with primitive fields for a direct port
- **C string pointers (`char *`, `const char *`)**
  - Map to `String` for owned mutable textual state
  - Map to `&str` for borrowed static or caller-owned string data where lifetime usage is straightforward
  - Map to `Option<String>` or `Option<&str>` if the C logic distinguishes null from non-null
- **C character buffers**
  - Map to `String` if treated as text
  - Map to `Vec<u8>` only if byte-wise mutation is required by the original implementation
- **C integral flags and counters**
  - Map to fixed Rust integer types matching the original range assumptions where visible (`i32`, `usize`, etc.)
- **C global mutable state**
  - Consolidate into a Rust context/config struct passed as `&mut` where possible

### Expected Rust Structures

Because the C structs are unnamed in the analysis, define only the minimum required named Rust types during implementation, such as:

- `OutputState`
  - Holds current level-related indentation and mark configuration used by output routines
- `LevelFormat`
  - Holds per-level formatting text or marker data if the C functions operate on indexed level state
- `Context`
  - Only if these functions currently depend on broader shared program state and an existing Rust port already uses a central state carrier

### Memory Management and Ownership

- Replace C pointer ownership conventions with clear Rust ownership:
  - copied configuration text becomes owned `String`
  - borrowed constants remain `&'static str` where appropriate
- Remove manual allocation/free patterns by storing owned strings directly in state structs
- For any array-of-level state formerly allocated in C, use `Vec<T>` with bounds-checked indexing; preserve exact indexing semantics carefully if C uses sentinel or one-based conventions

### Error Handling

- If the C functions cannot fail in practice and only update state, keep Rust APIs infallible
- If the C code validates inputs and reports failure through return codes, map that to:
  - `Result<(), LevelFormatError>` for meaningful failures, or
  - `bool` only if the surrounding Rust port already uses boolean status and no richer error information exists
- Do not introduce broad error frameworks; use module-local error enums if needed

## Implementation Phases

### Phase 1: Identify and Model Required State

- Inspect `set_level_indent` and `set_level_mark` in `src/main.c` and `src/output.c`
- Trace all directly accessed globals, static locals, structs, and helper functions required by these two functions
- Define the minimum Rust state structures needed to hold:
  - indentation text or width state
  - level marker text or symbol state
  - any per-level arrays or counters touched by the functions
- Decide exact ownership for migrated string data (`String`, `&str`, `Option<_>`) based on C pointer lifetime behavior

**Exit criteria**:
- All fields read or written by the two target functions are accounted for in Rust types
- Any dependent helper signatures needed for compilation are identified, but not expanded beyond necessity

### Phase 2: Port `set_level_indent`

- Implement the Rust equivalent of `set_level_indent`
- Translate C branching and indexing directly, preserving observable behavior
- Replace pointer/null checks with `Option` handling where applicable
- Preserve exact mutation order if later formatting logic depends on side effects
- Add focused unit tests covering:
  - normal indentation updates
  - empty or null-equivalent input behavior
  - repeated calls replacing prior state
  - boundary behavior for level indexing if present

**Exit criteria**:
- `set_level_indent` compiles and passes direct behavior tests
- No manual memory lifecycle remains for its migrated state

### Phase 3: Port `set_level_mark`

- Implement the Rust equivalent of `set_level_mark`
- Translate any mark parsing, assignment, or per-level update logic without redesign
- Ensure character/text handling matches the original C semantics, especially for:
  - single-character vs string markers
  - null termination assumptions
  - overwrite vs append behavior
- Add focused unit tests covering:
  - valid mark updates
  - empty/null-equivalent mark handling
  - replacement of existing mark state
  - invalid input handling if the C function signals errors

**Exit criteria**:
- `set_level_mark` compiles and passes direct behavior tests
- Mark state integrates cleanly with the Rust output state model

### Phase 4: Integrate and Verify Against Existing Output Paths

- Wire both migrated functions into the Rust project branch at the same call sites or equivalent configuration paths used by the C implementation
- Remove any temporary duplication created during the port
- Add integration-level tests for combined indentation and mark configuration where both functions affect output formatting
- Confirm that state borrowing remains simple and does not force unnecessary cloning or structural changes outside this module cluster

**Exit criteria**:
- Both functions are used from their intended Rust call paths
- `cargo test` passes
- The migrated module behavior matches the C formatting state transitions for covered cases