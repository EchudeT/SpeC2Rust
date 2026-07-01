# Implementation Plan: module_gnu_stat-w32.c_46

## Summary

Port `gnu/stat-w32.c` into a focused Rust module that preserves the existing initialization behavior exposed by `initialize`. The Rust implementation should translate the current file-local C setup logic into a small, self-contained Rust module on branch `052-module_gnu_stat_w32.c_46-rust-port`, using standard library facilities first and keeping the migration limited to the observed module surface.

The implementation approach is:

- create one Rust source module corresponding to `gnu/stat-w32.c`
- migrate the `initialize` function behavior directly, preserving call ordering and side effects
- replace anonymous C data layouts with named Rust-private structs or enums only where needed to represent current state
- express resource ownership explicitly through Rust lifetimes and ownership rather than manual memory handling
- convert C-style status/error signaling into `Result` where the surrounding Rust code permits, or into the narrowest equivalent internal representation if a direct signature match is required

The plan intentionally avoids adding new abstraction layers or extra facilities beyond what is required to migrate this file and function.

## Technical Context

- **Language/Version**: Rust 1.78 or newer
- **Primary Dependencies**:
  - Rust standard library
  - No third-party crates recommended from current evidence
- **Testing**:
  - `cargo test`
- **Performance Goals**:
  - Maintain behaviorally equivalent initialization cost
  - Avoid unnecessary heap allocation during initialization unless the C logic inherently requires retained state
  - Keep data layout and control flow simple enough to match the original module’s startup-time characteristics
  - Preserve constant-time access for any migrated static/module state

## Module Mapping

### C to Rust File Mapping

- `gnu/stat-w32.c` → `src/gnu/stat_w32.rs`

### Function Mapping

- `initialize` → `pub(crate)` or `pub(super)` Rust function `initialize(...)`
  - Final visibility should be the minimum required by current call sites
  - Signature should be adapted to Rust idioms only as far as surrounding integration allows

### Integration Boundary

- If `gnu/stat-w32.c` is currently included from a broader GNU compatibility area, place the Rust file under the equivalent existing namespace rather than introducing a new subsystem.
- Update the nearest existing `mod.rs` or parent module declaration only to register `stat_w32`.

## Data Model

The analysis reports three anonymous data structures. Since they are unnamed in C, the Rust plan should introduce stable internal names based on actual usage during migration.

### Data-Structure Mapping

- `anonymous` → `StatW32State`
  - Use when the C anonymous structure represents retained module state or initialization state
  - Prefer a private `struct`
- `anonymous` → `StatW32Config`
  - Use when the C anonymous structure groups configuration/constants consumed by `initialize`
- `anonymous` → `StatW32Flags` or `StatW32Kind`
  - Use `struct` if the original form is a grouped field record
  - Use `enum` if the original form is a tagged mode/category concept

### Mapping Rules

- C integer fields:
  - `int`, `unsigned int`, `long`, similar → Rust fixed-width or platform-width integers chosen from actual semantic use
  - Prefer `i32`, `u32`, `isize`, `usize` only when the C role is clear
- C pointers:
  - borrowed input pointers → Rust references where validity is guaranteed by call structure
  - nullable/non-owning pointers → `Option<&T>` / `Option<&mut T>` where feasible
  - owning buffers/resources → owned Rust fields such as `Vec<T>`, `String`, or dedicated structs only if ownership truly exists in the C code
- C booleans/flags:
  - numeric flag fields → `bool` or compact integer bitfields only if bitwise behavior is required
- Static/module storage:
  - translate to `const`, `static`, or function-local values according to original lifetime and mutability
  - avoid introducing synchronization primitives unless mutable global state is unquestionably required by the existing logic

### Memory Management Notes

- Eliminate manual lifetime tracking and cleanup where Rust ownership can represent the original behavior directly.
- If `initialize` mutates module-level state, keep the state representation minimal and explicit.
- Avoid unsafe code unless direct low-level interoperability or layout fidelity is strictly necessary for this file’s behavior.

### Error Handling Notes

- Convert sentinel return values and manual error propagation into `Result` internally.
- Where an external signature must remain close to existing project conventions, isolate conversion at the module boundary.
- Preserve initialization failure conditions and ordering from the C implementation.

## Implementation Phases

## Phase 1: Source Analysis and Rust Module Skeleton

- Inspect `gnu/stat-w32.c` and identify:
  - exact `initialize` signature
  - all file-local statics, macros, helper routines, and anonymous structures used by `initialize`
  - whether initialization is pure setup, cached state creation, or platform-conditional logic
- Create `src/gnu/stat_w32.rs`.
- Add the module declaration in the nearest existing Rust parent module.
- Introduce placeholder named Rust types for the three anonymous C structures, scoped privately to this module.
- Define the Rust `initialize` function signature with the narrowest visibility compatible with current callers.

**Exit criteria**:
- Rust module is wired into the crate
- all required C-local entities used by `initialize` are identified and mapped

## Phase 2: Data and Control-Flow Migration

- Port the anonymous C data structures into private Rust structs/enums.
- Translate file-local constants/macros used by `initialize` into:
  - `const`
  - small helper functions
  - local pattern matches
- Implement the body of `initialize` in Rust, preserving:
  - initialization order
  - condition checks
  - state updates
  - failure paths
- Replace C null/state checks with explicit `Option`/`Result` handling.
- Keep any mutable retained state local to the module and avoid widening its visibility.

**Exit criteria**:
- `initialize` compiles in Rust
- all state used by the original C function is represented without raw manual memory management unless unavoidable

## Phase 3: Integration and Behavior Preservation

- Connect existing callers to the Rust `initialize` implementation.
- Remove or bypass the original C module from the active build for this migrated path.
- Verify any return-value conventions expected by upstream code and apply boundary conversions only where required.
- Review platform-conditional portions of the original file and preserve only the conditions needed for this module’s current build target expectations.

**Exit criteria**:
- crate builds successfully with the Rust module in use
- no unresolved dependency remains on `gnu/stat-w32.c` for `initialize`

## Phase 4: Targeted Testing and Cleanup

- Add unit tests covering:
  - successful initialization path
  - invalid/precondition failure path if present in C logic
  - repeated initialization behavior if the original function permits or depends on it
- Run `cargo test`.
- Perform a final pass to simplify:
  - unnecessary mutable state
  - unnecessary allocations
  - any temporary unsafe blocks introduced during initial porting

**Exit criteria**:
- tests pass under `cargo test`
- implementation remains limited to the migrated file/function behavior
- no extra facilities or unrelated refactors are included