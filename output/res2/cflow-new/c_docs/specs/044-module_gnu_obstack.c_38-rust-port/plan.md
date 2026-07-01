# Implementation Plan: module_gnu_obstack.c_38

## Summary

This module migration covers `gnu/obstack.c`, with current scope limited to the `print_and_abort` function and the directly related internal data representations identified during analysis. The Rust implementation should preserve the C module’s operational behavior closely, especially around fatal error reporting and process termination semantics, while avoiding any expansion beyond the existing file and function scope.

The implementation approach is a narrow port into a single Rust module that mirrors the original file layout and responsibility. Because the known exported behavior is centered on abort-style failure handling, the Rust version should explicitly separate:
- message formatting and emission,
- terminal failure path,
- any internal state or placeholder structure definitions needed to keep the module shape aligned with the original C source.

Memory management should rely on Rust ownership and stack-based values wherever possible. Since the C function name strongly implies non-returning failure behavior, the Rust entry point should use a diverging function return type (`-> !`) if the migrated behavior confirms unconditional termination. Error handling should not introduce recovery paths that do not exist in the C source.

## Technical Context

### Language/Version
- Rust 1.78+ stable

### Primary Dependencies
- Rust standard library only
- No third-party crates are recommended based on the available module evidence

### Testing
- `cargo test`

### Performance Goals
- Preserve negligible overhead for fatal-path execution
- Avoid unnecessary heap allocation in message emission where practical
- Maintain behavior close to the C implementation without adding abstraction layers that change control flow cost

## Module Mapping

### C to Rust File Mapping
- `gnu/obstack.c` -> `src/module_gnu_obstack.rs`

### Function Mapping
- `print_and_abort` -> `pub(crate) fn print_and_abort(...) -> !` or `fn print_and_abort(...) -> !`
  - Final visibility and exact signature should be determined by existing call sites in the Rust branch
  - If formatting inputs are variable in the C source, map them to explicit Rust parameters rather than recreating C varargs unless strictly required by surrounding migrated code

### Module Placement
- Declare the Rust module from the crate root using standard Rust module layout only
- Keep all logic for this C file inside one Rust source file unless an already-existing project layout requires a different placement

## Data Model

The analysis reports multiple anonymous C data structures without names or field details. Since the current functional scope only identifies `print_and_abort`, the migration plan should treat these carefully and only materialize Rust types when required for compilation or direct behavioral preservation.

### Mapping Strategy
- Anonymous C structs/unions used only locally -> private Rust `struct` definitions with descriptive placeholder names based on usage context
- Anonymous integral/flag groupings -> Rust primitive integers or private `enum`/newtype only if semantic distinctions are visible in the source
- Anonymous pointer-carried state -> private Rust structs with owned fields or borrowed references according to actual lifetime usage

### Initial Data Structure Plan
Because no field layouts are provided, define only the minimum needed after inspecting `gnu/obstack.c`:
- `anonymous` -> `struct ObstackStateLike { ... }` if a state carrier is needed
- `anonymous` -> `struct AbortContextLike { ... }` if formatting or diagnostic context is needed
- Remaining anonymous items -> defer until source inspection confirms they are required by `print_and_abort` or unavoidable file-level compilation dependencies

### Memory and Layout Notes
- Do not preserve C memory layout with `#[repr(C)]` unless layout compatibility is required by actual cross-module usage
- Replace nullable pointers with `Option<&T>`, `Option<&mut T>`, or `Option<NonNull<T>>` only where the original semantics require pointer-like optionality
- Replace manual buffer ownership with `String`, `Vec<u8>`, or borrowed string slices as justified by actual source behavior

## Implementation Phases

### Phase 1: Source Inspection and Rust Module Skeleton
- Inspect `gnu/obstack.c` to confirm the exact signature, side effects, and termination behavior of `print_and_abort`
- Identify whether the anonymous structures are directly referenced by this function or only present elsewhere in the file
- Create `src/module_gnu_obstack.rs`
- Add the module declaration in the crate root following existing project conventions
- Stub `print_and_abort` with the intended non-returning shape if confirmed by the C source

### Phase 2: Function Port and Minimal Type Migration
- Port `print_and_abort` into Rust with behavior-preserving control flow
- Map C output/error reporting calls to `eprintln!`, `std::io::stderr`, or equivalent standard-library APIs based on required formatting fidelity
- Map termination behavior to `std::process::abort()` or another standard-library terminal path only if it matches the original C behavior
- Introduce only those private Rust structs/enums that are necessary for this function or unavoidable file-level compilation
- Eliminate raw memory management where not semantically required

### Phase 3: Behavioral Validation and Cleanup
- Add focused unit tests for observable behavior that can be validated without intercepting process abort in-process
- Where direct abort behavior prevents ordinary unit testing, isolate format construction or pre-abort message generation into internal helpers only if needed for testability and only within this module
- Verify that no extra capabilities or alternate recovery paths were introduced
- Remove unused placeholder types and keep the final Rust module limited to the migrated C file scope

### Phase 4: Integration Verification
- Run `cargo test`
- Validate crate integration and call-site compatibility on branch `044-module_gnu_obstack.c_38-rust-port`
- Confirm naming, visibility, and return-type choices match the surrounding migrated codebase
- Perform a final review for ownership correctness, absence of unnecessary `unsafe`, and close behavioral alignment with the original C implementation