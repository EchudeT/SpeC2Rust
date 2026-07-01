# Implementation Plan

## Summary

This module ports `gnu/xalloc-die.c` into a minimal Rust implementation centered on the single exported behavior represented by `xalloc_die`.

The C source is a small termination helper used after allocation failure. In Rust, the implementation should preserve that role: provide one module-level function that reports the allocation failure condition consistently with the surrounding port and then terminates the process with a non-returning control path.

Technical approach:

- Create a direct Rust module corresponding to `gnu/xalloc-die.c`.
- Implement `xalloc_die` as a diverging function (`-> !`) to reflect the C behavior of terminating execution.
- Prefer the Rust standard library for process termination and stderr output.
- Keep the port narrow: migrate only the existing file/function behavior, without introducing broader allocation frameworks or reusable diagnostics layers.
- Make error handling explicit by separating any message emission from the final termination step while keeping the public surface minimal.

## Technical Context

- **Language/Version**: Rust 1.76+
  A stable toolchain level suitable for standard-library-only process termination, I/O, and diverging functions.

- **Primary Dependencies**:
  - Rust standard library only (`std::io`, `std::process`)
  No third-party crates are warranted by the input.

- **Testing**:
  - `cargo test`
  - Unit tests should validate any non-terminating internal helpers, if introduced.
  - The terminating public function itself should be tested indirectly and conservatively to avoid destabilizing the test process.

- **Performance Goals**:
  - Constant-time termination path.
  - No heap allocation requirements in the failure path beyond unavoidable standard formatting behavior.
  - Maintain a simple, low-overhead implementation suitable for fatal out-of-memory handling logic.

## Module Mapping

| C File | C Function | Rust Module | Rust Item | Notes |
|---|---|---|---|---|
| `gnu/xalloc-die.c` | `xalloc_die` | `src/gnu/xalloc_die.rs` or `src/gnu/xalloc_die/mod.rs` | `pub fn xalloc_die() -> !` | Direct 1:1 migration of the module’s single responsibility. |

Recommended project placement:

- `src/gnu/mod.rs` to declare the migrated GNU compatibility modules if such a namespace already exists in the port.
- `src/gnu/xalloc_die.rs` for the concrete implementation.

If the branch already uses a flatter layout, keep that existing convention and only add the single corresponding Rust source file.

## Data Model

This module has no declared data structures in the provided input.

Data mapping summary:

| C Construct | Rust Construct | Notes |
|---|---|---|
| None | None | The module is function-only. |
| C non-returning termination routine | `fn xalloc_die() -> !` | Rust diverging return type is the correct semantic match. |

Operational mapping considerations:

- Any C global or external diagnostic usage visible during implementation should be translated to the narrowest Rust equivalent already present in the project.
- Do not introduce new stateful error objects or configuration structures for this module.

## Implementation Phases

## Phase 1: Create the Rust module skeleton

Scope:

- Add the Rust file corresponding to `gnu/xalloc-die.c`.
- Wire the module into the existing crate module tree.
- Declare the public function signature as:
  - `pub fn xalloc_die() -> !`

Technical decisions:

- Use a direct file-name normalization from C to Rust:
  - `xalloc-die.c` → `xalloc_die.rs`
- Reflect the original non-returning behavior with a diverging function type.
- Keep visibility limited to what the rest of the port requires; do not expose extra helpers publicly.

Acceptance criteria:

- The crate compiles with the new module included.
- The module exports exactly the migrated function needed by the current port stage.

## Phase 2: Port the termination behavior

Scope:

- Implement the function body for `xalloc_die`.
- Reproduce the C module’s fatal allocation-failure semantics using standard Rust facilities.

Technical decisions:

- Emit the allocation failure diagnostic to standard error using `eprintln!` or `writeln!` to `std::io::stderr()`, depending on how closely the surrounding project handles output.
- Terminate the process using `std::process::exit(...)` with the appropriate failure status consistent with the original project behavior.
- Keep the function body simple and deterministic.
- Avoid panic-based termination unless the wider port already standardizes on panic for this exact path; process exit is usually the closer match for a C fatal helper.

Memory/error-handling considerations:

- This function exists on an allocation-failure path, so implementation should avoid unnecessary intermediate state.
- If writing to stderr fails, ignore the write error and continue to termination.
- Do not attempt recovery or fallback allocation strategies.

Acceptance criteria:

- `xalloc_die` is non-returning in Rust type semantics.
- The implementation reports the fatal condition and exits consistently.

## Phase 3: Align call sites and local conventions

Scope:

- Update any Rust code in this branch that depends on the C-side `xalloc_die` behavior to call the new Rust function directly.
- Ensure naming and import paths are consistent with the crate’s existing GNU-compatibility module layout.

Technical decisions:

- Preserve a narrow migration surface: only adjust call sites needed for this module port.
- Prefer direct module-qualified calls over creating wrappers or aliases unless the existing crate style already requires them.
- If the original C function depended on project-global program name or message formatting helpers and those helpers already exist in Rust, reuse them rather than reimplementing formatting locally.

Acceptance criteria:

- All current call sites compile against the Rust implementation.
- No compatibility shims or extra abstraction layers are added without a direct migration need.

## Phase 4: Add focused tests

Scope:

- Add tests for any internal formatting/output helper extracted to support testability.
- Keep the public terminating function unmocked unless the project already has a subprocess-based test pattern.

Technical decisions:

- If needed, split the implementation into:
  - a small private/helper function that prepares the diagnostic content, and
  - the public `xalloc_die() -> !` that performs final termination.
- Test only stable, non-terminating logic in unit tests.
- If integration testing of the exit path is necessary and already customary in the project, use a subprocess invocation pattern; otherwise, do not add special harness machinery for this module alone.

Acceptance criteria:

- `cargo test` passes.
- Tests cover the stable technical behavior that can be exercised safely within the Rust test runner.