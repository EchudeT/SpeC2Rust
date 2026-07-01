# Implementation Plan

## Summary

Port `binary-io.c` into Rust with a minimal, behavior-preserving implementation focused on `set_binary_mode`. The Rust version should mirror the original module’s role in configuring standard stream handles for binary I/O where the platform requires it, while remaining a no-op on platforms where text/binary distinction does not apply.

The implementation should stay narrow:
- migrate the single function only;
- keep platform-specific behavior isolated in one Rust source file;
- use the Rust standard library and conditional compilation for platform handling;
- preserve process-level side effects and simple success/failure reporting consistent with the C implementation’s intent.

Because this module is a small system-integration utility rather than a data-processing component, the Rust port should emphasize:
- direct mapping of the existing function;
- explicit platform branching;
- controlled unsafe usage only if required for OS interaction;
- straightforward error propagation without adding new abstraction layers.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - negligible overhead relative to the C implementation;
  - no additional heap allocation for normal execution;
  - constant-time setup work per invocation;
  - platform checks resolved at compile time where possible via `cfg`.

## Module Mapping

| C File | C Function | Rust Module/File | Rust Item |
|---|---|---|---|
| `binary-io.c` | `set_binary_mode` | `src/binary_io.rs` | `pub(crate) fn set_binary_mode(...) -> ...` |

### Integration Notes

| C Module Role | Rust Placement | Notes |
|---|---|---|
| process startup / stdio mode helper | `src/binary_io.rs` and invoked from existing main flow | Keep call sites limited to the current startup path that corresponds to the C usage. |
| platform-specific stream configuration | `cfg(windows)` and `cfg(not(windows))` sections in the same file | Avoid splitting into extra modules unless required by the existing crate layout. |

## Data Model

This module does not define persistent C structs in the provided analysis.

### Data-structure Mapping

| C Type/Structure | Rust Mapping | Notes |
|---|---|---|
| none | none required | `set_binary_mode` should remain function-based. |
| C integer status / file descriptor style values | `i32`, `std::os` platform types, or `std::io::Result<()>` as appropriate | Choose the narrowest signature that matches the existing Rust crate call pattern. |

### State and Ownership

- No long-lived owned data structures are expected.
- Any OS handle or file descriptor interaction should borrow existing standard streams rather than introduce owned wrappers.
- If low-level platform APIs are needed, keep raw-handle usage local to the function and avoid storing handles in structs.

## Implementation Phases

### Phase 1: Establish module skeleton and signature mapping

- Create `src/binary_io.rs`.
- Add the Rust equivalent of `set_binary_mode` with a signature aligned to how the surrounding Rust port invokes startup helpers.
- Add the module declaration in the crate root or main module using existing project structure conventions.
- Identify the exact C semantics to preserve:
  - whether the function acts on stdin/stdout/stdin+stdout;
  - whether it returns status or exits/report errors indirectly;
  - whether failure is ignored on non-applicable platforms.

#### Deliverables
- Compiling Rust module with placeholder platform branches.
- Confirmed function signature and call-site placement.

### Phase 2: Implement platform-specific behavior

- Implement the effective behavior under `cfg(windows)` using the minimal required OS interaction to switch the relevant stream(s) to binary mode.
- Implement `cfg(not(windows))` as a no-op if the original C behavior is platform-neutral outside Windows-like targets.
- Keep unsafe code, if any, tightly scoped and documented with:
  - why it is needed;
  - what invariants are assumed;
  - how return values are checked.

#### Error Handling Decisions
- Prefer `std::io::Result<()>` if the Rust call chain already supports fallible startup helpers.
- If the surrounding port expects a boolean or integer status, convert OS errors at the boundary rather than introducing broader error enums.
- Do not add retry or recovery logic not present in the C module.

#### Memory Management Notes
- No manual allocation should be introduced.
- Any temporary platform values remain stack-local.
- No ownership transfer of standard stream resources.

### Phase 3: Wire into main-cluster flow

- Replace or add the corresponding startup call in the Rust main flow that maps to the original use of `set_binary_mode`.
- Ensure invocation order matches the C program’s startup sequence closely enough to preserve I/O behavior.
- Keep the integration local; do not refactor unrelated startup logic.

#### Deliverables
- End-to-end compile path through the main binary.
- Binary-mode setup reachable from the same execution stage as in C.

### Phase 4: Add focused tests and validation

- Add unit tests for compile-time platform behavior where feasible:
  - no-op path on non-Windows;
  - signature and success-path behavior.
- Add narrowly scoped tests that avoid changing global process I/O state unless the crate’s existing test approach already allows it.
- Where direct runtime verification of standard stream mode is impractical, prefer validating:
  - function returns expected success shape;
  - Windows-only code compiles behind conditional compilation;
  - non-Windows path remains side-effect-free.

#### Test Scope
- `cargo test` for module/unit coverage.
- Avoid integration tests that depend on terminal state, external files, or process-wide mutable I/O configuration unless already established in the project.

## Notes on C-to-Rust Migration Decisions

- Preserve the single-function module shape rather than inventing a broader I/O subsystem.
- Use conditional compilation instead of runtime dispatch for platform distinctions.
- Favor standard-library types and module-local helpers over introducing traits or wrapper layers.
- Keep side effects explicit and localized to mirror the original system-level utility role.
- If exact C return conventions are awkward in Rust, translate them only at the API boundary nearest the original call site rather than propagating C-style integer statuses throughout the crate.