# Implementation Plan: module_gnu_msvc-inval.c_36

## Summary

This module migration covers the single C source file `gnu/msvc-inval.c` and its exported function `gl_msvc_inval_ensure_handler`. The Rust implementation should preserve the existing module boundary and behavior with a minimal porting surface: one Rust module responsible for ensuring the MSVC invalid-parameter handler is installed or initialized in the same situations as the C code.

The implementation approach should remain narrow:

- translate the file into one Rust source module;
- preserve one public entry point corresponding to `gl_msvc_inval_ensure_handler`;
- model any C file-scope state with private Rust statics;
- isolate Windows/MSVC-specific behavior behind conditional compilation;
- keep non-Windows or non-MSVC builds as no-op or stub-compatible paths only if required by the original build expectations.

Because the source appears to manage runtime handler installation and includes several anonymous C data structures, the Rust work should focus on direct representation of the required state rather than redesign. Unsafe Rust should be limited to the exact platform interop needed for runtime handler registration or low-level global state access.

## Technical Context

### Language / Version

- Rust stable, edition 2021
- Minimum recommended compiler: Rust 1.76 or newer

### Primary Dependencies

Use the Rust standard library by default.

Recommended dependencies:

- None by default

Conditional consideration only if the existing codebase already uses them for Windows type bindings:

- `windows-sys` for direct Windows/MSVC ABI items, if the port requires calling Windows/MSVC runtime APIs not available through the standard library

Do not introduce broader abstraction crates unless the migrated C code proves they are necessary.

### Testing

- `cargo test`

Testing scope should include:

- basic invocation of `gl_msvc_inval_ensure_handler`;
- idempotent repeated calls;
- conditional compilation tests to ensure the module compiles cleanly on unsupported platforms without changing exposed signatures.

### Performance Goals

This module is initialization-oriented, so performance goals should stay modest and behavioral:

- negligible steady-state overhead after first initialization;
- no avoidable heap allocation;
- constant-time repeated ensure calls;
- preserve the low-overhead nature of the original C implementation.

## Module Mapping

### C to Rust File Mapping

- `gnu/msvc-inval.c` -> `src/gnu/msvc_inval.rs`

### Public API Mapping

- `gl_msvc_inval_ensure_handler` -> `pub(crate)` or `pub` Rust function `gl_msvc_inval_ensure_handler()`

Visibility should match how the surrounding crate uses the migrated function. If the function is only consumed internally, prefer `pub(crate)`.

### Internal Layout

Keep the Rust module flat and close to the original C file:

- one Rust module for the migrated implementation;
- private helper items only when directly needed to express C file-scope logic;
- private static state for one-time initialization tracking;
- `#[cfg(...)]` gates for MSVC/Windows-specific code paths.

## Data Model

The analysis lists only anonymous C data structures, which suggests implementation-local structs, unions, or callback-related signatures rather than stable exported data models. The Rust mapping should therefore remain implementation-private and minimal.

### Data-Structure Mapping

- anonymous C structures used only for temporary local state
  - map to private Rust `struct` definitions only if they are still necessary after translation
  - otherwise replace with tuples, local bindings, or direct API argument construction

- anonymous C callback/function-pointer-related declarations
  - map to Rust type aliases such as:
    - `type InvalidParameterHandler = ...;`
  - use explicit `unsafe extern "C"` or platform ABI signatures if required by the runtime API

- anonymous C static state carriers
  - map to private Rust statics such as:
    - `static INIT: Once` or
    - `static INSTALLED: AtomicBool`
  - choose the smallest construct that preserves original one-time semantics

### Memory Management

The C module likely relies on static storage and runtime registration rather than dynamic allocation. The Rust port should preserve that model:

- prefer `static` or `static mut` replacement with safe wrappers such as `Once`/`AtomicBool`;
- avoid heap allocation unless the platform API explicitly requires ownership transfer;
- keep callback/handler values in static lifetime storage when registering with external runtime APIs.

### Error Handling

If the original function is effectively a best-effort initializer with no return value, keep the Rust function signature narrow and avoid introducing new result types unless the surrounding crate requires them.

For platform calls that may fail:

- capture failure internally;
- preserve original behavior by making the function non-panicking;
- document any intentionally ignored platform errors in comments near the unsafe block.

## Implementation Phases

## Phase 1: Create the Rust Module Skeleton

- Add `src/gnu/msvc_inval.rs` corresponding directly to `gnu/msvc-inval.c`.
- Wire the module into the existing Rust crate module tree without changing broader project structure.
- Add the Rust function `gl_msvc_inval_ensure_handler` with the target visibility matching current use.
- Establish conditional compilation boundaries for:
  - Windows + MSVC-specific implementation;
  - fallback compilation path for other targets if needed.

**Exit criteria:**

- crate compiles with the new module included;
- public/internal function name mapping is in place;
- no extra modules or helper subsystems are introduced.

## Phase 2: Port Static State and Handler Registration Logic

- Translate the C file’s file-scope state into private Rust statics.
- Implement one-time initialization semantics using standard library primitives:
  - prefer `std::sync::Once` for exactly-once setup;
  - use `AtomicBool` only if the C logic is strictly flag-based and does not require a setup closure.
- Port the invalid-parameter handler registration logic as closely as possible to the C control flow.
- Keep all unsafe operations tightly scoped around external API interaction and global mutable state access.
- Replace anonymous C data structures with:
  - private Rust structs when structurally needed;
  - type aliases or local variables when they are only incidental.

**Exit criteria:**

- repeated calls to `gl_msvc_inval_ensure_handler` are safe and idempotent;
- the Rust implementation mirrors the C initialization order and state transitions;
- unsafe code is localized and documented.

## Phase 3: Platform-Specific Cleanup and Behavioral Alignment

- Verify MSVC/Windows ABI details used by the handler function and registration call.
- Ensure non-target platforms compile without attempting unsupported runtime interactions.
- Align no-op behavior, stub behavior, or conditional exclusion with the original module’s intended portability model.
- Review for exact preservation of:
  - handler installation timing;
  - one-time semantics;
  - absence of unexpected panics or allocations.

**Exit criteria:**

- target-platform code path is behaviorally aligned with the original C file;
- non-target builds compile cleanly;
- the module remains limited to the original responsibilities.

## Phase 4: Tests and Migration Validation

- Add unit tests for the callable surface that can be exercised safely in Rust:
  - calling `gl_msvc_inval_ensure_handler` once;
  - calling it multiple times to verify idempotent behavior.
- Add compile-focused tests or cfg-gated test coverage where runtime handler behavior cannot be validated portably.
- Run `cargo test` to validate module integration.
- Perform final review for:
  - unnecessary allocations;
  - unnecessary visibility widening;
  - divergence from the original file/function scope.

**Exit criteria:**

- tests pass under `cargo test`;
- the migrated module is integrated with no scope expansion;
- the implementation remains a direct port of `gnu/msvc-inval.c`.