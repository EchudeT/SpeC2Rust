# Implementation Plan: module_test

## Summary

This module is a small test-oriented C code set spread across `test/simple.c`, `test/recursion.c`, and `test/multi.c`. The Rust port should preserve the existing function-level behavior and keep the implementation close to the original control flow and arithmetic logic.

The Rust approach should be a direct source migration:
- translate each C source file into a corresponding Rust module file,
- keep function names and boundaries aligned with the original where practical,
- use plain Rust functions and primitive integer types,
- avoid introducing new abstraction layers beyond what is needed for idiomatic and safe Rust.

Because the analyzed inputs show only standalone functions and no declared shared data structures, the port should remain lightweight and focus on deterministic function translation, straightforward ownership, and test coverage for each migrated function path.

## Technical Context

### Language / Version
- Rust stable, edition 2021
- Minimum recommended toolchain: Rust 1.76 or newer

### Primary Dependencies
- Rust standard library only

No third-party crates are recommended because the input shows only simple computation and test-style entry points, with no evidence requiring external parsing, async runtime, CLI frameworks, or numerical libraries.

### Testing
- `cargo test`

Testing should cover:
- arithmetic helpers such as `add`, `mul`, `helper`, and `twice`,
- recursive functions such as `fib` and `fact`,
- integration of per-file execution functions such as `run`, `compute`, and translated `main`-equivalent behavior where applicable.

### Performance Goals
- Preserve the same asymptotic behavior as the C originals.
- Avoid unnecessary heap allocation; use stack-based values and plain function calls.
- Keep recursive functions semantically equivalent unless a direct Rust translation would be unsound or untestable.
- Maintain negligible overhead relative to the C logic for these small test functions.

## Module Mapping

The C sources should map directly to Rust source files under standard Rust layout.

### Proposed Rust Layout
- `src/lib.rs`
- `src/simple.rs`
- `src/recursion.rs`
- `src/multi.rs`

### File Mapping
- `test/simple.c` → `src/simple.rs`
- `test/recursion.c` → `src/recursion.rs`
- `test/multi.c` → `src/multi.rs`

### Function Mapping

#### From `test/simple.c`
Functions identified:
- `add`
- `mul`
- `orphan`
- `compute`
- `main`

Rust mapping:
- `src/simple.rs`
  - `pub fn add(...) -> ...`
  - `pub fn mul(...) -> ...`
  - `pub fn orphan(...) -> ...`
  - `pub fn compute(...) -> ...`
- The C `main` should not be preserved as a Rust crate entry point inside the library module. Instead:
  - translate its internal logic into a regular function such as `pub fn run_main_like(...) -> ...` only if needed to preserve testable behavior,
  - or fold its logic into targeted unit tests if it is only a thin wrapper around `compute` and related functions.

#### From `test/recursion.c`
Functions identified:
- `fib`
- `fact`
- `main`

Rust mapping:
- `src/recursion.rs`
  - `pub fn fib(...) -> ...`
  - `pub fn fact(...) -> ...`
- The C `main` should be converted the same way as above:
  - not as the crate's executable entry point,
  - but as a normal helper function only if its behavior is meaningful and should be asserted directly.

#### From `test/multi.c`
Functions identified:
- `helper`
- `twice`
- `run`

Rust mapping:
- `src/multi.rs`
  - `pub fn helper(...) -> ...`
  - `pub fn twice(...) -> ...`
  - `pub fn run(...) -> ...`

### Crate Root Responsibilities
`src/lib.rs` should:
- declare only the migrated modules:
  - `pub mod simple;`
  - `pub mod recursion;`
  - `pub mod multi;`
- avoid adding unrelated support layers or utility modules.

## Data Model

No C structs, unions, or enums were identified in the analysis results.

### Data-Structure Mapping
- No struct or enum migration is required.
- C primitive numeric values should map to Rust primitive integer types chosen per original usage.

### Primitive Type Mapping Guidance
Because exact C signatures are not included, type selection should be conservative and consistent:
- C `int` → Rust `i32` by default
- C boolean-style return values or branch predicates → Rust `bool` only when semantically clear
- If original functions return status-like integers, keep them as `i32` rather than converting to `Result` without evidence

### Memory Management
- No manual memory management is expected because no heap-managed structures or raw pointer APIs are present in the analyzed module.
- Prefer owned values and plain parameter passing.
- Avoid unsafe code unless the original source reveals a construct that cannot be represented safely, which is not indicated here.

### Error Handling
- For pure arithmetic and recursive helper functions, preserve direct return-value behavior rather than introducing new error enums.
- If the C code relies on integer-domain assumptions, document them in tests.
- Do not add recovery frameworks or generalized error layers that are not present in the source behavior.

## Implementation Phases

## Phase 1: Set Up Crate Structure and Port Independent Arithmetic Functions

### Goals
- Establish the Rust module layout.
- Migrate the simplest non-recursive functions first.
- Confirm baseline type choices and naming.

### Tasks
- Create `src/lib.rs` with module declarations for `simple`, `recursion`, and `multi`.
- Port from `test/simple.c`:
  - `add`
  - `mul`
  - `orphan`
  - `compute`
- Port from `test/multi.c`:
  - `helper`
  - `twice`
- Choose direct Rust signatures using primitive integers, preferring `i32` unless the source indicates otherwise.
- Add unit tests for each migrated function with representative inputs derived from the C behavior.

### Notes
- Keep function bodies structurally similar to the C originals.
- Do not refactor arithmetic composition beyond what is required for valid Rust syntax.

## Phase 2: Port Recursive Logic and Validate Behavior Boundaries

### Goals
- Migrate recursion-focused code with minimal behavioral change.
- Verify base cases and recursive cases explicitly.

### Tasks
- Port from `test/recursion.c`:
  - `fib`
  - `fact`
- Preserve recursive structure unless the original implementation requires a small Rust-specific adjustment for correctness.
- Add unit tests covering:
  - base cases,
  - one-step recursive progression,
  - several ordinary positive inputs.

### Notes
- Keep return types aligned with the C originals to avoid changing arithmetic behavior.
- If recursion depth is inherently unbounded in the original, document that the Rust port preserves that characteristic rather than redesigning it.

## Phase 3: Port Per-File Driver Logic and Resolve `main` Translation

### Goals
- Capture remaining orchestration functions.
- Replace C `main` functions with testable Rust equivalents without turning the library crate into multiple binaries.

### Tasks
- Port from `test/multi.c`:
  - `run`
- Review each C `main` occurrence from:
  - `test/simple.c`
  - `test/recursion.c`
- For each `main`:
  - translate its internal logic into a regular Rust function only when it contains meaningful behavior not already covered by lower-level functions,
  - otherwise express its behavior through unit tests calling the already migrated functions.
- Ensure no naming conflicts remain from multiple C `main` definitions.

### Notes
- This phase should keep the port restrained: no binary targets are needed unless the original project structure explicitly requires them.
- Prefer private helper functions for `main`-specific sequencing if exposure is unnecessary.

## Phase 4: Consolidate Tests and Finalize Migration Parity

### Goals
- Confirm that all listed functions are accounted for.
- Ensure the Rust module organization is complete and minimal.

### Tasks
- Verify coverage of the full function list:
  - `helper`
  - `twice`
  - `run`
  - `fib`
  - `fact`
  - `add`
  - `mul`
  - `orphan`
  - `compute`
  - translated `main` logic from both C files where needed
- Run `cargo test` and resolve any mismatches caused by integer typing or recursive edge behavior.
- Remove any unnecessary scaffolding introduced during migration.

### Exit Criteria
- Each original C source file has a direct Rust module counterpart.
- Each meaningful non-`main` function has been ported.
- Any `main` behavior from the C files has either been translated into ordinary functions or covered by tests.
- The crate builds and passes `cargo test`.