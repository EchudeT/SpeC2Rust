# Implementation Plan: module_gnu_gl_msvc_inval_per_thread_07

## Summary

This module ports `gnu/msvc-inval.c` into a focused Rust implementation that preserves the existing per-thread invalid-parameter handling behavior used on MSVC-oriented code paths. The Rust work should mirror the current file’s responsibilities only: represent the current thread-local invalid-parameter state, provide the equivalent handler routine, and expose the current-state accessor logic corresponding to the C functions already present.

The implementation approach should favor a small Rust module using the standard library only, especially `thread_local!`, interior mutability primitives suited to per-thread state, and narrow internal APIs that directly correspond to the C entry points. The migration should avoid adding broader runtime facilities or cross-module abstractions beyond what is required to replace the original file.

## Technical Context

### Language / Version
- Rust stable
- Minimum recommended version: **1.74** or newer

### Primary Dependencies
- Rust standard library only
- No third-party crates recommended, since the provided input does not justify external dependencies

### Testing
- `cargo test`

### Performance Goals
- Preserve constant-time access to current per-thread invalid-parameter state
- Avoid heap allocation in normal handler/state-access paths
- Keep the Rust port operationally lightweight and comparable to the original C implementation
- Maintain minimal synchronization overhead by using thread-local storage rather than shared global locking

## Module Mapping

### Source Mapping
- C source: `gnu/msvc-inval.c`
- Rust target: `src/module_gnu_gl_msvc_inval_per_thread_07.rs`

### Responsibility Mapping
- `gl_msvc_invalid_parameter_handler`
  - Map to a Rust internal function with the same semantic role
  - Implement as the module’s state-updating handler routine
  - Keep signature Rust-native unless an existing surrounding codebase requires a specific calling convention
- `gl_msvc_inval_current`
  - Map to a Rust function returning the current thread-local invalid-parameter state
  - Preserve direct accessor behavior without introducing unrelated result layers

### Integration Scope
- Keep all migrated logic in one Rust module corresponding to the single C file
- Do not split into extra submodules unless required by existing crate layout
- Expose only the functions needed by the rest of the Rust port

## Data Model

The analysis only reports multiple anonymous C data structures and does not provide field definitions. The Rust plan should therefore preserve semantics while deferring exact type shape until the C source is inspected line-by-line.

### Mapping Strategy
- Anonymous C structs used only for transient callback parameters
  - Map to either:
    - no dedicated Rust struct, if fields are unused in effective logic, or
    - a private Rust struct with named fields if the data is materially referenced
- Anonymous C state container holding per-thread invalid-parameter status
  - Map to a private Rust struct, for example `InvalidParameterState`
- Anonymous flag-like or small-status storage
  - Map to primitive Rust types such as `bool`, `i32`, `usize`, or `Option<T>` based on actual C usage
- Anonymous pointer-bearing structures
  - Replace raw ownership patterns with borrowed references or owned Rust values where possible
  - Use raw pointers only if the translated logic truly depends on pointer identity or nullable callback inputs

### Expected Core Rust State
A likely minimal internal representation is:
- a private per-thread state struct
- stored in `thread_local!`
- wrapped with `Cell` or `RefCell` depending on whether the state is copy-sized or requires structured mutation

Example planning shape:
- `struct InvalidParameterState { ... }`
- `thread_local! { static CURRENT: Cell<...> / RefCell<InvalidParameterState> = ... }`

### Memory Management Decisions
- Thread-local state should be owned entirely by Rust and automatically cleaned up with thread teardown
- Avoid manual allocation/free patterns from C
- Replace nullable state checks with `Option<T>` where they reflect real absence/presence semantics
- Keep lifetimes internal to the module where possible to prevent leaking raw state references

### Error Handling Decisions
- Preserve the C module’s behavioral model rather than introducing new error types
- If the C logic records an invalid-parameter event instead of returning errors, keep that as internal state mutation
- Use Rust return values only where the original accessor semantics clearly require them
- Avoid panics in normal invalid-parameter handling paths

## Implementation Phases

### Phase 1: Source Inspection and Rust Module Skeleton
- Inspect `gnu/msvc-inval.c` and identify:
  - exact per-thread state fields
  - whether handler parameters are used or ignored
  - how `gl_msvc_inval_current` exposes current status
- Create `src/module_gnu_gl_msvc_inval_per_thread_07.rs`
- Define the minimal private state type(s) required to mirror the C file
- Establish thread-local storage using standard library facilities
- Add function stubs matching the C file’s migrated responsibilities

### Phase 2: Function Porting and State Semantics
- Port `gl_msvc_invalid_parameter_handler` into Rust
- Port `gl_msvc_inval_current` into Rust
- Replace C thread-local/per-thread mechanisms with Rust `thread_local!`
- Translate any sentinel values, flags, or counters into Rust-native representations
- Preserve original control flow and state transitions closely, without adding new features or broader abstractions

### Phase 3: Type Tightening and Safety Review
- Refine anonymous-structure mappings into the narrowest private Rust types that still preserve behavior
- Remove unnecessary raw pointers if the translated code permits safe Rust references or owned values
- Confirm no accidental shared mutable global state was introduced
- Review any callback-like logic for correct lifetime and mutability handling
- Ensure the accessor function does not expose invalid borrows from thread-local storage

### Phase 4: Tests and Integration Validation
- Add unit tests for:
  - default per-thread state
  - state changes performed by the handler
  - thread-local isolation across multiple threads
  - accessor behavior after one or more handler invocations
- Run `cargo test`
- Verify the Rust module remains limited to the original C file’s scope and does not introduce unrelated facilities

## Notes and Constraints

- The function list includes repeated occurrences of `gl_msvc_invalid_parameter_handler`; treat these as a single migrated function unless source inspection proves otherwise.
- The anonymous data structures must be resolved from the actual C source during implementation; no speculative public Rust API should be created around them.
- Keep visibility minimal: private state, narrowly exposed functions, and no additional compatibility layers unless already required by the crate.
- The migration should prioritize behavioral equivalence, thread-local correctness, and simple ownership over architectural expansion.