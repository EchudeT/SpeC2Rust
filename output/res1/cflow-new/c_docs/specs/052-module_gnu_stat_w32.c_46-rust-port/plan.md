# Implementation Plan

## Summary

Port `gnu/stat-w32.c` into an idiomatic Rust module that preserves the existing module scope and behavior, with implementation centered on the `initialize` function and the small set of internal anonymous C data structures it depends on.

The Rust implementation should:
- mirror the current file-level responsibility of Windows-specific GNU stat initialization logic,
- keep the migration narrowly scoped to the existing source file and function set,
- replace C memory and state handling with explicit Rust ownership and initialization patterns,
- use `Result`-based error propagation where the C code relied on status checks or implicit initialization failure paths.

The preferred approach is a direct translation into a single Rust module file with minimal restructuring. Internal anonymous C structs should become private Rust structs with descriptive names derived from their usage in `initialize`. Any global or static initialization state in the C file should be represented with Rust `static` items only if required by the original logic; otherwise, confine state to function-local values.

## Technical Context

### Language / Version
- Rust 1.78+ stable

### Primary Dependencies
- Rust standard library
- No third-party crates should be introduced unless the existing C implementation clearly depends on functionality not reasonably expressible with the standard library

### Testing
- `cargo test`

### Performance Goals
- Preserve the current constant-time initialization characteristics of the C module
- Avoid unnecessary heap allocation unless the original C structures require owned dynamic storage
- Keep Windows-specific initialization overhead minimal and limited to the existing call path of `initialize`
- Match existing behavior closely rather than introducing new abstraction layers

## Module Mapping

### Source File Mapping
- `gnu/stat-w32.c` -> `src/module_gnu_stat_w32.rs`

### Function Mapping
- `initialize` -> `pub(crate) fn initialize(...) -> Result<..., ...>` or `pub(crate) fn initialize(...)` depending on the original observable failure mode

If the C function currently reports failure through return codes:
- translate integer/status returns into a small Rust error type local to this module

If the C function is effectively infallible:
- keep the Rust function infallible and model any optional setup using local checks and explicit defaults

### Visibility and Scope
- Keep the Rust module internal to the crate unless the surrounding project already exposes this module publicly
- Keep helper functions and mapped anonymous structures private to the module
- Do not split this port into additional submodules unless required by existing project layout conventions

## Data Model

Because the C analysis exposes only anonymous structures, the Rust plan should assign private names based on each structure’s role in `initialize`.

### Structure Mapping
- `anonymous` -> `struct InitState`
- `anonymous` -> `struct StatConfig`
- `anonymous` -> `struct WindowsStatContext`

These names are placeholders for implementation planning and should be finalized after inspecting field usage in `gnu/stat-w32.c`.

### Mapping Rules
- C anonymous structs used only within `initialize`
  - map to private Rust structs with narrowly typed fields
- C integer flags
  - map to `bool` when binary
  - otherwise map to fixed-width integer types (`i32`, `u32`, etc.) matching semantic range
- C pointers
  - map to references when borrowing is clear
  - map to `Option<T>` or `Option<NonNull<T>>` for nullable non-owning pointers
  - map to owned Rust values (`Box<T>`, `Vec<T>`, `String`) only when the original code owns the allocation
- C string data
  - map to `String` for owned text
  - map to `&str` for borrowed string literals or validated borrowed text
  - use `OsString` / `OsStr` only if Windows path or platform string handling is directly required by the existing code
- C enums or symbolic constants embedded in integers
  - map to Rust enums only where the value set is closed and directly visible from the file
  - otherwise keep as integer constants to avoid widening scope

### Memory Management Notes
- Replace manual zero-initialization and lifetime management with Rust value initialization
- Eliminate raw pointer ownership transfer where possible
- If unsafe code is necessary for Windows-specific interop already implied by the C source, isolate it in the smallest possible block and document invariants locally
- Avoid recreating C-style mutable global state unless file-level behavior requires it

### Error Handling Notes
- Convert sentinel values and integer error returns into `Result`
- Preserve original initialization semantics and ordering
- Use standard error representations local to the module; avoid introducing broad shared error frameworks for a single-file port

## Implementation Phases

## Phase 1: Inspect and Skeleton-Port the File
- Create `src/module_gnu_stat_w32.rs`
- Copy the file-level responsibility of `gnu/stat-w32.c` into one Rust module without expanding scope
- Identify the exact signature and side effects of `initialize`
- Define provisional private Rust structs for the three anonymous C data structures
- Establish constants, local state, and any required Windows-specific imports from the standard library

### Deliverables
- Compiling Rust module skeleton
- Stubbed or partially translated `initialize`
- Initial struct definitions with field placeholders replaced by concrete Rust types as identified

## Phase 2: Translate Data and Initialization Logic
- Port the body of `initialize` directly, preserving execution order
- Replace C initialization patterns with explicit Rust constructors or local variable initialization
- Map C condition checks and status propagation into Rust control flow
- Resolve pointer/null handling into `Option`, references, or minimal `unsafe` blocks where unavoidable
- Keep all helper logic in the same module unless a pre-existing crate layout requires another destination

### Deliverables
- Fully translated `initialize`
- Finalized private Rust data structures
- Clear ownership model for all state manipulated by initialization

## Phase 3: Integrate Error Semantics and Windows-Specific Behavior
- Align return behavior with the original C function’s observable outcomes
- Introduce a module-local error type only if the C function can fail in meaningful ways
- Validate any Windows path, metadata, or stat-related assumptions present in the original code
- Minimize and document any `unsafe` usage required for platform interaction

### Deliverables
- Behaviorally aligned Rust implementation
- Module-local error handling finalized
- Unsafe boundaries reviewed and reduced

## Phase 4: Add Focused Tests and Final Cleanup
- Add unit tests covering the reachable behaviors of `initialize`
- Reproduce edge cases implied by the original C logic, especially default-state and failure-path handling
- Run `cargo test` and fix compile or semantic mismatches
- Remove any temporary translation scaffolding that is no longer needed

### Deliverables
- Passing `cargo test`
- Clean final Rust module with restrained scope
- Confirmed one-to-one migration coverage for `gnu/stat-w32.c` and `initialize`