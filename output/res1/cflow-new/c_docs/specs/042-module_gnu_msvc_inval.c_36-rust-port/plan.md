# Implementation Plan: module_gnu_msvc-inval.c_36

## Summary

Port `gnu/msvc-inval.c` to a single Rust module that preserves the existing module boundary and behavior centered on `gl_msvc_inval_ensure_handler`.

The Rust implementation should focus on:

- representing the one-time installation/ensuring logic in idiomatic Rust,
- preserving the original side-effect-oriented behavior rather than redesigning the API,
- containing any platform-specific behavior behind conditional compilation,
- using standard-library synchronization primitives for one-time initialization where possible,
- keeping unsafe code narrowly scoped to the platform interaction points required by the original C logic.

The implementation should migrate the existing C file into one Rust source file with a direct function mapping, avoiding expansion into additional helper subsystems beyond what is required to mirror the current behavior.

## Technical Context

### Language / Version

- Rust stable, edition 2021
- Minimum recommended compiler: `rustc 1.74+`

### Primary Dependencies

Use the Rust standard library by default.

Recommended dependencies:

- No third-party crates required for the planned port.
- Use `std::sync::Once` or `std::sync::OnceLock` for one-time handler installation state.
- Use `core::ffi` / `std::ffi` C-compatible primitive types if platform bindings are needed in the translated implementation.

### Testing

- `cargo test`

Testing scope should include:

- compile-time validation for supported target configurations,
- unit tests for one-time initialization behavior where this can be exercised without relying on unavailable platform hooks,
- platform-gated tests for Windows/MSVC-specific paths,
- regression-oriented tests that confirm repeated calls to `gl_msvc_inval_ensure_handler` do not re-run installation logic.

### Performance Goals

This module is initialization-oriented rather than throughput-oriented. Performance goals are therefore minimal and specific:

- constant-time fast path after the handler has already been ensured,
- no avoidable heap allocation,
- no repeated installation work on subsequent calls,
- negligible overhead relative to the original C implementation.

## Module Mapping

### C to Rust File Mapping

- `gnu/msvc-inval.c` -> `src/gnu/msvc_inval.rs`

If the existing Rust crate already exposes a matching namespace, keep the module nested under the current `gnu` module tree rather than creating a new top-level organization.

### Function Mapping

- `gl_msvc_inval_ensure_handler` -> `pub(crate)` or `pub` function `gl_msvc_inval_ensure_handler()`

Visibility should match actual crate usage. Prefer the narrowest visibility that still supports current callers.

### Behavioral Mapping

The Rust function should preserve the C module’s role as an idempotent “ensure installed” entry point:

- one-time initialization semantics,
- platform-conditional behavior for MSVC-specific invalid parameter handling,
- no new return protocol unless required by the existing crate conventions,
- side effects limited to the equivalent handler registration/initialization operation.

## Data Model

The analysis only identifies multiple anonymous C data structures and does not provide named layouts. For planning purposes, the Rust port should avoid inventing persistent public data types unless the C implementation requires them for callback signatures or platform interop.

### Data-Structure Mapping

- Anonymous C structs/unions used only locally -> local Rust bindings or type aliases scoped inside `src/gnu/msvc_inval.rs`
- C function-pointer callback state -> Rust `type` alias for the callback signature, using `unsafe extern "C"` only if required by the platform ABI
- Static initialization flags / internal state -> `static Once`, `OnceLock`, or `AtomicBool` only if a simple flag is sufficient

### Mapping Guidance

Because the C analysis does not expose stable named structures, use the following restrained mapping rules:

1. **Do not create public Rust structs** solely to mirror anonymous C layouts if they are only implementation details.
2. **Prefer primitive and standard-library representations** for state:
   - C static flag -> `static Once` or `static AtomicBool`
   - C null/non-null callback pointer tracking -> `Option<CallbackType>` only if needed internally
3. **Use raw pointers only at the boundary** where platform APIs require them.
4. **Keep unsafe blocks minimal and documented** around:
   - callback registration,
   - global mutable state interop,
   - ABI-specific function signatures.

### Memory Management

- Preserve static lifetime behavior with Rust statics rather than heap allocation.
- Avoid manual allocation unless forced by the underlying platform API.
- Ensure any callback or handler references used by the OS/runtime have `'static` lifetime and are not moved after registration.

### Error Handling

Since `gl_msvc_inval_ensure_handler` appears to be an ensure/setup routine rather than a rich fallible API:

- preserve the original observable behavior if the C version does not report errors,
- if platform calls can fail but the C code suppresses or ignores failure, keep equivalent best-effort semantics,
- avoid introducing `Result` into the public signature unless required by existing Rust-side integration.

## Implementation Phases

## Phase 1: Module Skeleton and Direct Translation Boundary

Goals:

- Create the Rust destination file corresponding to `gnu/msvc-inval.c`.
- Establish the module export path and function signature for `gl_msvc_inval_ensure_handler`.
- Identify all platform-specific symbols and anonymous C data uses from the source file and translate them into the smallest possible Rust-local representations.

Tasks:

- Add `src/gnu/msvc_inval.rs`.
- Wire the module into the existing crate module tree.
- Port preprocessor conditionals to `#[cfg(...)]` attributes.
- Translate file-scope static state into Rust statics.
- Define any required callback type aliases and local constants.

Exit criteria:

- The crate compiles with the new Rust module stub in place.
- The Rust function exists with the intended visibility and target gating.
- No extra modules or abstractions have been introduced beyond direct migration needs.

## Phase 2: One-Time Handler Installation Logic

Goals:

- Implement the actual ensure/install behavior of `gl_msvc_inval_ensure_handler`.
- Preserve idempotence and global-state semantics from the C source.

Tasks:

- Translate the handler registration sequence directly from C into Rust.
- Replace ad hoc C one-time checks with `std::sync::Once` or an equally close standard-library primitive.
- Encapsulate unsafe platform interaction in a minimal internal block or helper function within the same file.
- Verify that repeated calls produce the same final state without duplicate registration work.

Exit criteria:

- The Rust implementation mirrors the C module’s setup order and side effects.
- Repeated invocation is safe and does not reinstall the handler.
- Unsafe code is localized and justified with comments.

## Phase 3: Platform Gating, Behavioral Parity Checks, and Tests

Goals:

- Finalize target-specific compilation behavior.
- Add restrained tests covering the migrated behavior that can be validated in Rust.

Tasks:

- Ensure non-Windows or non-MSVC builds either compile to a no-op equivalent or follow the original conditional behavior exactly.
- Add unit tests for one-time execution semantics where testable without external runtime dependence.
- Add `#[cfg]`-gated tests for Windows/MSVC builds if the handler path is test-observable.
- Confirm there are no ownership or lifetime issues in registered callback state.

Exit criteria:

- `cargo test` passes for applicable targets.
- The module compiles cleanly under target combinations relevant to the original C file.
- The Rust port remains a narrow migration of the original file, with no expanded capability surface.

## Notes and Constraints

- Keep the implementation confined to the migrated file and necessary module declarations.
- Avoid introducing broader runtime abstraction layers for invalid-parameter handling.
- Prefer standard-library primitives over external crates.
- Preserve C-like semantics for global initialization and side effects, while making ownership and unsafe boundaries explicit in Rust.
- Do not add new public data models unless the original module externally exposes them.