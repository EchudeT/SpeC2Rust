# Implementation Plan

## Summary

Port the C test module files `test/simple.c`, `test/recursion.c`, and `test/multi.c` into a Rust module layout that preserves the existing function behavior and file-level responsibilities without adding new capabilities. The Rust implementation should translate the current free-function style directly into Rust functions, keep recursion-based logic where present, and model the existing test-style executable entry points with standard Rust test functions or small binary-facing wrappers only where required by the source layout.

The technical approach is a direct migration:
- map each C source file to a Rust module file,
- convert standalone C functions into Rust `fn` items,
- preserve integer-oriented computation semantics as closely as practical,
- avoid heap management unless required by the translated logic,
- use Rust’s ownership model and explicit return types instead of C’s implicit memory and error conventions.

Because the listed inputs show only functions and no data structures, the Rust port should remain function-centric and lightweight.

## Technical Context

- **Language/Version**: Rust 1.78+ stable
- **Primary Dependencies**:
  - Rust standard library only
- **Testing**:
  - `cargo test`
- **Performance Goals**:
  - Match the simple computational behavior of the C implementation without introducing measurable overhead beyond normal Rust debug/release differences.
  - Keep function calls, arithmetic, and recursion straightforward.
  - Avoid unnecessary allocations and dynamic dispatch.
  - Preserve stack-based execution for recursive routines unless the source logic itself requires otherwise.

## Module Mapping

Map the existing C files into a Rust module structure that mirrors the original file boundaries.

### Source File Mapping

- `test/simple.c` → `src/module_test/simple.rs`
- `test/recursion.c` → `src/module_test/recursion.rs`
- `test/multi.c` → `src/module_test/multi.rs`

### Rust Module Entry

- `src/module_test/mod.rs`
  - declares:
    - `pub mod simple;`
    - `pub mod recursion;`
    - `pub mod multi;`

### Function Mapping

Keep functions close to their source file of origin.

#### `test/simple.c`
Expected Rust targets:
- `add` → `module_test::simple::add`
- `mul` → `module_test::simple::mul`
- `compute` → `module_test::simple::compute`
- `main` → migrate into:
  - a unit test validating the original execution path, or
  - a small internal runner function if preserving call structure is necessary before test wrapping

#### `test/recursion.c`
Expected Rust targets:
- `fib` → `module_test::recursion::fib`
- `fact` → `module_test::recursion::fact`
- `main` → migrate into:
  - a unit test for the expected recursive outputs, or
  - an internal runner function used by tests

#### `test/multi.c`
Expected Rust targets:
- `helper` → `module_test::multi::helper`
- `twice` → `module_test::multi::twice`
- `run` → `module_test::multi::run`
- `orphan` → `module_test::multi::orphan`
- `main` → migrate into:
  - a unit test covering top-level behavior, or
  - an internal runner if needed for structure preservation

### Visibility Guidance

- Default to private functions inside each module.
- Mark functions `pub(crate)` only when cross-module use is required by the original C call graph.
- Avoid making items `pub` unless they need crate-external visibility.

## Data Model

No C structs or custom data structures were identified in the input.

### Data-Structure Mapping

- **C structs/enums**: none identified
- **Rust structs/enums**: none required for the initial port

### Scalar and Function-Level Type Mapping

Use direct scalar mappings consistent with the original arithmetic intent:

- `int` → `i32` by default
- `unsigned int` → `u32` if present in source during implementation
- function return values:
  - C arithmetic returns → Rust integer returns with explicit types
  - C `void` functions → Rust `()`

### Memory Management

- No manual memory management layer is required based on the available input.
- Prefer stack-only local variables.
- Do not introduce `Box`, `Rc`, `Arc`, raw pointers, or lifetimes unless the source code reveals an actual need during translation.

### Error Handling

- If the C functions assume valid inputs and return plain integers, preserve that behavior with direct returns instead of introducing `Result`.
- Only use `Result` where Rust I/O or parsing is actually needed by the translated `main` logic; otherwise keep tests and helper functions infallible.
- For recursion, preserve source-domain assumptions and avoid adding defensive error frameworks not present in the original code.

## Implementation Phases

## Phase 1: Establish module skeleton and migrate simple arithmetic file

### Goals
- Create the Rust module layout for `module_test`.
- Port the non-recursive arithmetic functions first.
- Replace the C `main` in `test/simple.c` with Rust tests or a local runner plus tests.

### Tasks
- Create:
  - `src/module_test/mod.rs`
  - `src/module_test/simple.rs`
  - `src/module_test/recursion.rs`
  - `src/module_test/multi.rs`
- Implement in `simple.rs`:
  - `add`
  - `mul`
  - `compute`
- Determine the exact C integer semantics used in `simple.c` and encode them with explicit Rust integer types.
- Convert the `main` path from `simple.c` into `#[cfg(test)]` unit tests in the same module or module-level tests in `mod.rs`.

### Technical Notes
- Keep arithmetic direct and expression-based.
- If `compute` depends only on `add` and `mul`, preserve that call composition rather than collapsing logic.
- Avoid introducing traits, generics, or utility layers.

### Exit Criteria
- `simple.rs` compiles.
- Unit tests cover the translated execution path from `test/simple.c`.

## Phase 2: Port recursive functions and preserve call behavior

### Goals
- Migrate recursive logic from `test/recursion.c`.
- Keep recursion structure intact for `fib` and `fact`.

### Tasks
- Implement in `recursion.rs`:
  - `fib`
  - `fact`
- Translate any helper execution flow associated with `main`.
- Add unit tests for:
  - base cases
  - representative recursive cases
  - the original `main` scenario from the C file

### Technical Notes
- Preserve recursive formulation rather than rewriting to iterative logic unless required to match observed behavior.
- Use explicit integer types to avoid accidental type widening.
- If the original C code allows only non-negative inputs in practice, reflect that via tests and internal assumptions rather than adding extra API surface.

### Exit Criteria
- Recursive functions match expected outputs from the C source behavior.
- `cargo test` passes for recursion-specific cases.

## Phase 3: Port multi-function file and resolve internal call graph

### Goals
- Migrate `test/multi.c` with minimal restructuring.
- Preserve the relationships among `helper`, `twice`, `run`, and `orphan`.

### Tasks
- Implement in `multi.rs`:
  - `helper`
  - `twice`
  - `run`
  - `orphan`
- Identify whether any functions are intentionally unused in the original source and keep them as such if needed.
- Translate the `main` path into Rust tests or a local runner used by tests.
- Adjust visibility only if one translated function must call another across module boundaries.

### Technical Notes
- If `orphan` is unused in the original file, keep it implemented but avoid artificial integration.
- Use `#[allow(dead_code)]` only if needed to preserve a source function that is not exercised after translation.
- Keep helper decomposition aligned with the original file; do not merge or extract extra layers.

### Exit Criteria
- `multi.rs` compiles with the full function set.
- Tests cover the original top-level flow and any significant helper interactions.

## Phase 4: Consolidate tests and finalize semantic parity

### Goals
- Verify the whole module compiles and behaves consistently with the source files.
- Clean up Rust-specific issues without changing functionality.

### Tasks
- Run `cargo test` across the full crate.
- Review integer type choices for consistency across all translated files.
- Remove unnecessary visibility and imports.
- Add narrowly scoped comments only where the C-to-Rust translation would otherwise be unclear.
- Confirm that each original C function has a direct Rust counterpart or a clearly justified test-runner replacement for `main`.

### Technical Notes
- Do not add integration infrastructure beyond standard Rust tests.
- Keep the final structure close to the source layout to simplify review against the original C code.

### Exit Criteria
- All translated module files compile cleanly.
- All tests pass under `cargo test`.
- The Rust module remains a direct port of the listed C files and functions, without added facilities.