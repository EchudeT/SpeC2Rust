# Implementation Plan

## Summary

Port the C module `xalloc-die.c` into a focused Rust module that preserves the existing allocation-failure termination behavior used by the `pwd` program. The Rust implementation should keep the module narrow: provide the Rust equivalent of `xalloc_die` and wire it into the crate’s executable flow without adding new allocation abstractions or recovery paths.

Technically, this migration should rely on Rust’s standard library and model the original “fatal allocation failure” path as a small function that emits the expected diagnostic and terminates the process with the appropriate non-zero exit status. Because Rust normally aborts or panics on allocation failure before user code can recover, the implementation should mirror the existing module’s intent at the application layer rather than attempting to redesign allocation handling. The port should therefore focus on preserving call shape, process termination semantics, and message emission behavior expected by the current program structure.

## Technical Context

- **Language/Version**: Rust stable, edition 2021 (compatible with current stable toolchain; target rustc 1.75+)
- **Primary Dependencies**:
  - Rust standard library only
- **Testing**:
  - `cargo test`
- **Performance Goals**:
  - No meaningful runtime overhead relative to the C version
  - Constant-time fatal path setup
  - No additional heap allocation in the termination path where reasonably avoidable
  - Maintain process-exit behavior with minimal control-flow complexity

## Module Mapping

| C File | Rust File | Notes |
|---|---|---|
| `xalloc-die.c` | `src/xalloc_die.rs` | Direct port of the module’s single fatal-allocation function |
| `xalloc-die.c` (`xalloc_die`) | `src/xalloc_die.rs` (`pub(crate) fn xalloc_die() -> !`) | Use never-returning Rust function to express process termination semantics |
| main executable integration | `src/main.rs` or existing crate entry module | Import and use the Rust port where the original module behavior is referenced |

## Data Model

This module does not define persistent data structures.

| C Construct | Rust Construct | Notes |
|---|---|---|
| none | none | Function-only module |
| process termination via function that does not return | `fn ... -> !` | Rust never type accurately represents fatal termination |
| static diagnostic text / program name access if needed | `&'static str` and existing crate-level program context | Reuse existing application state only if already present; do not introduce new global state unless required by current port structure |

## Implementation Phases

### Phase 1: Create the Rust module skeleton

- Add `src/xalloc_die.rs`.
- Define the Rust equivalent of `xalloc_die` as a crate-visible function with signature returning `!`.
- Keep the implementation self-contained and standard-library-based.
- Decide the exact termination primitive based on current crate conventions:
  - prefer `std::process::exit(code)` if the original C behavior exits normally after reporting;
  - use `panic!` only if the surrounding port already models this path as unrecoverable panic and that is required for consistency.
- Avoid introducing helper layers, custom traits, or generic allocation APIs.

### Phase 2: Port fatal reporting semantics

- Translate the C function’s observable behavior into Rust:
  - emit the allocation failure diagnostic to standard error;
  - preserve the intended exit status;
  - ensure the function never returns.
- If the C implementation depends on existing program-name formatting or shared error helpers, reuse only the already-ported equivalent from the Rust crate.
- Keep message construction simple and avoid unnecessary owned `String` allocation in the fatal path.
- Review ownership and borrowing so the function uses only static text or existing borrowed context where possible.

### Phase 3: Integrate with the executable module layout

- Expose the new module from the crate using standard Rust module declarations.
- Replace any placeholder or missing references for this C module with calls to `xalloc_die`.
- Confirm the module remains limited to the original file’s responsibility and does not absorb unrelated error-reporting logic.
- Verify that integration does not require widening visibility beyond `pub(crate)` unless the current crate layout strictly requires it.

### Phase 4: Validate behavior with tests

- Add focused tests for any non-terminating helper logic if such factoring is necessary for testability.
- For the terminating function itself, prefer testing extracted formatting/exit-code decisions indirectly rather than invoking unconditional process termination in unit tests.
- If needed, use subprocess-style integration tests sparingly to confirm:
  - diagnostic is written to stderr;
  - process exits with the expected non-zero code.
- Run `cargo test` and confirm the module compiles cleanly without introducing warnings from unreachable code or unused exports.