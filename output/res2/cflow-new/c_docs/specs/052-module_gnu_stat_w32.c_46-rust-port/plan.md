# Implementation Plan: module_gnu_stat-w32.c_46

## Summary

This module ports `gnu/stat-w32.c` into Rust with a narrow scope centered on the existing `initialize` function and the file-local data it depends on. The Rust implementation should preserve the current initialization behavior and platform-specific intent without introducing broader filesystem abstractions or additional modules.

The technical approach is to translate the C file into a single Rust source file within the existing crate layout, using Rust standard library facilities first and keeping any Windows-specific interaction minimal and localized. Global or file-static C state should be represented with Rust module-private statics or small internal structs as needed, while initialization logic should be rewritten to use explicit result handling and safe ownership. Unsafe Rust should be avoided unless a direct Windows API binding becomes strictly necessary for parity.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**:
  - Rust standard library
  - No third-party crates by default
  - If Windows API access is required and cannot be expressed via `std`, use `windows-sys` with the smallest needed feature set
- **Testing**: `cargo test`
- **Performance Goals**:
  - Match the C module’s initialization cost profile closely
  - Avoid heap allocation unless required by the original logic
  - Keep one-time setup paths constant-time or near-constant-time
  - Do not add repeated initialization overhead beyond what is needed for safe Rust state management

## Module Mapping

### Source File Mapping
- `gnu/stat-w32.c` -> `src/gnu/stat_w32.rs`

### Function Mapping
- `initialize` -> `pub(crate)` or module-private `fn initialize(...)` in `src/gnu/stat_w32.rs`, depending on current crate call sites

### Scope Boundaries
- Port only the contents needed from `gnu/stat-w32.c`
- Keep related helper items in the same Rust file unless the existing Rust project already requires a different placement
- Do not introduce new public APIs unless the C function is already externally used through the module boundary

## Data Model

The analysis identifies three anonymous C data structures. Since their concrete fields are not provided, the Rust plan should preserve them as internal-only representations derived directly from actual C usage during porting.

### Data Structure Mapping Strategy
- `anonymous` -> `struct InternalStateA`
- `anonymous` -> `struct InternalStateB`
- `anonymous` -> `struct InternalStateC`

### Mapping Rules
- C anonymous `struct` used only as local grouping -> Rust private `struct`
- C anonymous flag sets or tagged choices -> Rust private `enum` if mutually exclusive semantics are present
- C scalar flags (`int`, `bool`-like fields) -> `bool`, `u32`, `i32`, or platform-sized integers based on observed use
- C pointers to static/global state -> Rust module-private static state, preferably:
  - `static` for immutable constants
  - `static mut` only as a last resort
  - `std::sync::OnceLock` for one-time initialization if lazy setup semantics are needed
- C strings:
  - borrowed C string constants -> `&'static str` when UTF-8-safe and not passed as raw C pointers
  - OS/path-facing values -> `std::ffi::OsString` / `std::path::PathBuf` when platform handling matters

### Memory Management
- Replace manual C lifetime management with Rust ownership for local state
- Convert nullable pointers into `Option<T>` or `Option<NonNull<T>>` only if raw-pointer semantics are required
- Avoid leaked allocations and preserve static-lifetime behavior only where the original C module depends on persistent process-wide state

### Error Handling
- Replace integer status propagation with:
  - `Result<T, std::io::Error>` for OS interaction
  - `Result<T, ModuleError>` only if multiple internal error classes are clearly needed
- If `initialize` in C is side-effect-only and non-failing, retain a no-return-value Rust function and encode fallible substeps internally with explicit handling consistent with current behavior

## Implementation Phases

### Phase 1: Inventory and Skeleton Port
- Create `src/gnu/stat_w32.rs`
- Identify all file-scope state, constants, macros, and helper routines in `gnu/stat-w32.c` that are required by `initialize`
- Define Rust placeholders for the three anonymous structures based on actual field usage
- Establish the Rust signature for `initialize` from existing call sites and module visibility requirements
- Add the module to the crate using the project’s current module tree, without creating extra layers

### Phase 2: State and Initialization Translation
- Port file-static and global C state into Rust module-private equivalents
- Translate `initialize` control flow directly, keeping behavior aligned with the original ordering and side effects
- Replace C-style sentinel values, null checks, and mutable shared state with idiomatic Rust representations where behavior remains unchanged
- Introduce `unsafe` only for unavoidable platform API calls or static mutation patterns that cannot be replaced with safer standard constructs

### Phase 3: Windows-Specific Behavior Finalization
- Resolve any Windows-only path, metadata, or environment assumptions used by `initialize`
- Prefer `std` APIs first; only add `windows-sys` if direct system calls are necessary for semantic parity
- Validate that process-wide initialization occurs once and that repeated calls behave consistently with the C implementation
- Ensure platform conditional compilation is minimal and localized to this module

### Phase 4: Tests and Behavioral Verification
- Add focused unit tests for:
  - one-time initialization behavior
  - repeated `initialize` invocation behavior
  - default state after initialization
- Add any narrow regression tests derived from edge cases visible in the C logic
- Run `cargo test` and fix mismatches in state transitions, error handling, and platform assumptions

## Notes and Constraints

- Keep the port limited to `gnu/stat-w32.c` and the `initialize` function’s direct dependencies
- Prefer direct translation over architectural refactoring
- Do not add cross-platform abstraction layers beyond what is needed to compile and preserve behavior
- Preserve module-private encapsulation for internal state and anonymous data representations
- Treat missing C structure detail as a port-time extraction task from the source file, not as an invitation to redesign the model