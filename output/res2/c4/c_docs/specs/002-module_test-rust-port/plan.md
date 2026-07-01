# Implementation Plan: module_test

## Summary

Port the C test module files `test/c4.c` and `test/hello.c` into a Rust module set on branch `002-module_test-rust-port`, preserving the current execution flow and function boundaries as closely as Rust allows.

The Rust implementation should focus on direct migration of the existing parsing/execution-oriented functions `next`, `expr`, `stmt`, and the two C entry functions named `main`, without introducing new subsystem boundaries beyond what is needed for Rust source organization. The preferred approach is:

- translate file-level state and procedural logic into Rust module-local structs and functions,
- replace raw pointer and integer-buffer manipulation with slice/index-based access,
- represent parse/evaluation state explicitly in Rust-owned data structures,
- use `Result` for fallible top-level operations while keeping internal control flow close to the C implementation,
- keep the implementation in standard Rust with minimal dependencies.

## Technical Context

### Language/Version
- Rust stable, edition 2021
- Minimum recommended compiler: Rust 1.75+

### Primary Dependencies
- Standard library only

No third-party crates are recommended from the available evidence. The module appears to be a direct C test/program port and does not justify parser frameworks, CLI frameworks, or error-handling crates.

### Testing
- `cargo test`

Testing should cover:
- direct behavior of translated `next`, `expr`, and `stmt` where practical,
- file-driven or string-driven execution paths corresponding to `test/c4.c` and `test/hello.c`,
- basic equivalence checks for expected success/failure behavior of the translated entry paths.

### Performance Goals
- Preserve the original module’s single-threaded procedural behavior with no significant asymptotic regressions.
- Prefer zero-copy scanning over source text where possible by using byte slices and indices.
- Avoid unnecessary heap allocation beyond what is required for Rust ownership and explicit state representation.
- Keep dispatch and state transitions straightforward so the Rust port remains close to C runtime characteristics for test-sized inputs.

## Module Mapping

### Source File Mapping
- `test/c4.c` -> `src/module_test/c4.rs`
- `test/hello.c` -> `src/module_test/hello.rs`

### Rust Module Layout
- `src/module_test/mod.rs`
  - exposes the translated submodules only as needed by the crate
- `src/module_test/c4.rs`
  - contains the Rust port of `next`, `expr`, `stmt`, and the `main` logic originating from `test/c4.c`
- `src/module_test/hello.rs`
  - contains the Rust port of the `main` logic originating from `test/hello.c`

### Function Mapping
- C `next` -> Rust `fn next(...)`
- C `expr` -> Rust `fn expr(...)`
- C `stmt` -> Rust `fn stmt(...)`
- C `main` from `test/c4.c` -> Rust `pub fn run_c4(...) -> Result<i32, ModuleError>` or equivalent crate-internal runner
- C `main` from `test/hello.c` -> Rust `pub fn run_hello(...) -> Result<i32, ModuleError>` or equivalent crate-internal runner

Because Rust cannot define two top-level functions with the same name in one module namespace serving the same role, the two translated `main` functions should be renamed according to source file origin while preserving their original responsibilities.

## Data Model

No explicit C structs were identified in the input. The port should therefore map implicit C global/file-level state into Rust structures only where required to preserve behavior.

### State Mapping
If `test/c4.c` uses shared parser/interpreter state through globals, migrate that state into a dedicated Rust struct such as:

```rust
struct C4State {
    source: Vec<u8>,
    position: usize,
    token: i32,
    // additional fields only if directly required by the original file state
}
```

This is not a feature expansion; it is the Rust replacement for C file-scope mutable state.

### Recommended C-to-Rust Representations
- `char *` / source buffer access -> `&[u8]`, `Vec<u8>`, or `&str` with byte indexing only when UTF-8 assumptions are valid
- integer token codes -> `i32` or a small Rust `enum` only if the original token set is clearly closed and already explicit in C
- global mutable parser variables -> fields on a state struct passed as `&mut C4State`
- C return-code error signaling -> `Result<T, ModuleError>` at external boundaries; internal helper functions may remain simple if the C logic assumes fatal termination
- null pointers / sentinel values -> `Option<T>` where semantically equivalent

### Error Handling Model
Use a small module-local error type for:
- input/argument validation failures,
- parse failures,
- execution/setup failures directly corresponding to C failure branches.

Prefer:
```rust
type ModuleResult<T> = Result<T, ModuleError>;
```

Avoid broad abstraction layers; keep errors close to original failure sites.

### Memory Management Notes
- Replace manual buffer ownership and pointer arithmetic with owned buffers plus indices.
- Avoid unsafe code unless the original algorithm cannot be expressed reasonably with safe indexing; if unsafe becomes necessary, isolate it narrowly and document the exact invariant being preserved from C.
- Ensure recursive functions like `expr` or `stmt`, if recursive in C, are translated without hidden shared aliasing.

## Implementation Phases

## Phase 1: Establish Rust module skeleton and port entry paths
- Create `src/module_test/mod.rs`, `src/module_test/c4.rs`, and `src/module_test/hello.rs`.
- Add Rust runner functions corresponding to both C `main` functions:
  - `run_c4`
  - `run_hello`
- Move any file-scope constants, token values, and mutable globals from `test/c4.c` into Rust module constants and a single explicit state container.
- Define the minimal `ModuleError` and `ModuleResult` types needed by the port.
- Wire the crate so tests can invoke both translated entry paths directly.

### Phase 1 Deliverable
A compiling Rust module layout with translated top-level control flow placeholders and explicit state ownership replacing C globals.

## Phase 2: Port lexical and expression/statement logic
- Translate `next` first, because it establishes tokenization/state advancement used by the rest of the file.
- Port `expr` next, keeping operator handling and precedence logic aligned with the C implementation.
- Port `stmt` after `expr`, reusing the same mutable state object.
- Preserve original control flow order and side effects rather than refactoring into new parser layers.
- Replace pointer-based traversal with index-based traversal over the source buffer.
- Keep function names and responsibilities close to the C originals for traceability.

### Phase 2 Deliverable
Working Rust equivalents of `next`, `expr`, and `stmt` with state-driven behavior matching the C source structure.

## Phase 3: Complete file-specific behavior and integrate `hello`
- Finish remaining logic in `c4.rs` required by the original `test/c4.c` `main` path.
- Port `test/hello.c` `main` into `hello.rs` with the same restrained approach, preserving its file-local behavior without introducing shared abstractions unless the code is literally duplicated.
- Normalize return codes into Rust `Result<i32, ModuleError>` while keeping the observable success/failure behavior equivalent.
- Ensure argument and input handling are explicit and memory-safe.

### Phase 3 Deliverable
Both original C test files have direct Rust counterparts with executable runner functions and no remaining C-global-style ownership issues.

## Phase 4: Verification and cleanup
- Add `cargo test` coverage for:
  - token advancement via `next` where exposed or testable through higher-level behavior,
  - representative expression and statement paths,
  - both translated entry paths from `c4` and `hello`.
- Validate that the Rust implementation preserves expected return behavior and basic output/evaluation flow from the C originals.
- Remove dead placeholders introduced during staged porting.
- Keep naming/comments sufficient to trace each Rust function back to its C origin.

### Phase 4 Deliverable
A tested Rust port of the module with stable function mapping, explicit state management, and safe memory handling.