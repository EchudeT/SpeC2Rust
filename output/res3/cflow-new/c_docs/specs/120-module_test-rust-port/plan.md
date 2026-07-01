# Implementation Plan

## Summary

Port the C `module_test` module into a Rust module that preserves the existing function-level behavior found in:

- `test/simple.c`
- `test/recursion.c`
- `test/multi.c`

The Rust implementation should stay narrowly aligned with the current C sources and function set:

- `helper`
- `twice`
- `run`
- `fib`
- `fact`
- `main`
- `add`
- `mul`
- `orphan`
- `compute`

Technical approach:

- Translate each C source file into a corresponding Rust module or test-oriented source file with the same behavioral boundaries.
- Prefer plain free functions over introducing new traits, abstractions, or service layers.
- Use Rust’s ownership and type system to eliminate manual memory concerns present in C, while keeping logic structurally close to the original code.
- Preserve recursive behavior for `fib` and `fact` unless the C code requires different control flow.
- Keep error handling minimal and explicit; if the C code assumes valid integer inputs and direct execution flow, mirror that with simple return values rather than adding new error frameworks.
- Consolidate duplicate `main`-style entry behavior into Rust tests or small binary entry points only where needed by the existing file layout.

## Technical Context

- **Language/Version**: Rust 1.78+ edition 2021
- **Primary Dependencies**:
  - Rust standard library only
- **Testing**:
  - `cargo test`
- **Performance Goals**:
  - Match C behavior closely for small integer arithmetic and recursive examples.
  - Avoid unnecessary heap allocation.
  - Keep call structure simple and predictable.
  - Maintain acceptable performance for the current sample/test-style workload rather than introducing optimizations that alter structure.

## Module Mapping

C-to-Rust mapping should remain file-oriented and minimal.

| C File | Rust Target | Notes |
|---|---|---|
| `test/simple.c` | `src/module_test/simple.rs` | Port arithmetic/helper-style functions found in this source. |
| `test/recursion.c` | `src/module_test/recursion.rs` | Port recursive functions such as `fib` and `fact` with direct recursive implementations unless source behavior requires otherwise. |
| `test/multi.c` | `src/module_test/multi.rs` | Port remaining orchestration or cross-function calls such as `run`, `compute`, or related helpers. |

Top-level Rust module layout:

| Rust File | Purpose |
|---|---|
| `src/module_test/mod.rs` | Re-export or declare `simple`, `recursion`, and `multi` submodules. |
| `src/module_test/simple.rs` | Functions migrated from `test/simple.c`. |
| `src/module_test/recursion.rs` | Functions migrated from `test/recursion.c`. |
| `src/module_test/multi.rs` | Functions migrated from `test/multi.c`. |
| `tests/module_test.rs` or inline unit tests | Behavioral verification for migrated functions and former `main`-driven flows. |

Function mapping should follow original ownership by source file as discovered during porting. If duplicate symbol names such as `main` exist across C files, do not force a single Rust `main`; instead:

- convert each C `main` scenario into a dedicated test case, or
- create narrowly scoped helper functions that represent the executable path previously exercised by each C file.

## Data Model

No C data structures were identified in the analysis input.

Data-model mapping:

| C Construct | Rust Construct | Notes |
|---|---|---|
| None identified | None required | Keep functions operating on primitive numeric types. |

Primitive mapping guidelines for the port:

| C Pattern | Rust Type | Notes |
|---|---|---|
| integer parameters/returns | `i32` by default | Use only if consistent with the C source signatures and arithmetic range. |
| non-negative recursive counts | `u32` only if clearly warranted | Prefer preserving signedness if the C code uses plain `int`. |
| no dynamic allocation | stack values | No heap-backed structures should be introduced without source evidence. |

Memory management and error handling decisions:

- Use Rust value semantics for all primitive operations.
- No manual allocation or deallocation is expected.
- If the C code does not report errors, keep Rust signatures simple and avoid introducing `Result` unnecessarily.
- If any function depends on input domains like `n >= 0` for recursion, encode the same assumptions in tests and document edge behavior in code comments rather than redesigning APIs.

## Implementation Phases

### Phase 1: Establish module skeleton and port leaf arithmetic functions

Create the Rust module structure under `src/module_test/` and wire `mod.rs` to expose only the migrated functions needed by current callers/tests.

Migration order in this phase:

- Port simple arithmetic and helper-style functions first:
  - `helper`
  - `twice`
  - `add`
  - `mul`
  - `orphan`

Technical tasks:

- Map C `int`-style signatures to Rust integer types consistently.
- Keep functions as free functions.
- Add unit tests for direct input/output behavior derived from the C code paths.
- Resolve naming collisions only by module qualification, not by renaming unless required by Rust compilation rules.

### Phase 2: Port recursive logic and validate base-case behavior

Migrate recursive functions from `test/recursion.c`:

- `fib`
- `fact`

Technical tasks:

- Preserve recursive structure and base cases exactly.
- Confirm integer type choice does not change observable behavior for current test inputs.
- Add targeted tests for:
  - base cases
  - one-step recursive cases
  - representative normal cases

Constraints:

- Do not introduce memoization, iterator rewrites, or algorithmic redesign unless the C source already does so.
- Keep stack usage characteristics broadly comparable to the original sample code.

### Phase 3: Port orchestration functions and replace C `main` usage with Rust tests

Migrate higher-level execution flow from `test/multi.c` and any remaining source-level entry paths:

- `run`
- `compute`
- `main` cases from each C source

Technical tasks:

- Convert each C `main` into:
  - a test case, if it exists only to demonstrate behavior, or
  - a small internal function invoked by tests, if that keeps logic clearer without adding new public API.
- Preserve call ordering between helper functions.
- Keep module boundaries aligned with original source ownership.

Special handling:

- Because multiple C files may define `main`, Rust should not mirror them as crate entry points.
- Instead, represent each former executable path as a test scenario validating expected outputs or return values.

### Phase 4: Final integration review and cleanup

Complete the port by verifying consistency across modules and removing any accidental expansion beyond the C behavior.

Technical tasks:

- Ensure exported functions match only the needed migrated surface.
- Review for integer conversions, recursion termination, and duplicate-name handling.
- Run `cargo test` and fix behavioral mismatches.
- Keep documentation limited to migration notes and any essential assumptions discovered from the C implementation.

Exit criteria:

- All listed C functions are migrated or intentionally represented through equivalent Rust tests where `main` was only an entry wrapper.
- The Rust module compiles cleanly.
- `cargo test` passes for the migrated behavior.