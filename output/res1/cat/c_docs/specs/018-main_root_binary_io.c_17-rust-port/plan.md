# Implementation Plan: `main_root_binary-io.c_17`

## Summary

This module is a narrow port of `binary-io.c`, centered on migrating the `set_binary_mode` function into the Rust codebase without expanding scope beyond the existing C behavior.

The Rust implementation should preserve the original platform-specific intent:

- on platforms where binary/text mode distinctions matter, apply the equivalent mode change to the relevant standard stream handle;
- on platforms where no distinction exists, make the operation a no-op;
- keep the API surface minimal and local to the main command path.

The technical approach should favor:

- Rust standard library types for stream and platform handling where possible;
- a small, explicit platform-conditional implementation;
- fallible return handling using `std::io::Result<()>` if the underlying operation can fail on the target platform, or a no-op success path elsewhere;
- migration of only the current file/function responsibilities, without introducing broader I/O abstraction layers.

## Technical Context

### Language / Version

- Rust stable
- Minimum target: Rust 1.75 or newer

### Primary Dependencies

- Rust standard library
- No third-party crates are recommended based on the provided module scope

### Testing

- `cargo test`

### Performance Goals

- Zero or negligible overhead relative to the C implementation
- No additional buffering layers or wrapper abstractions introduced by this port
- Constant-time setup behavior for binary-mode switching
- Preserve startup-path efficiency for the main binary execution flow

## Module Mapping

### C to Rust File Mapping

- `binary-io.c` -> `src/binary_io.rs`

### Function Mapping

- `set_binary_mode` -> `pub(crate) fn set_binary_mode(...) -> std::io::Result<()>` or `pub(crate) fn set_binary_mode(...)` depending on whether the C behavior exposes actionable failure on the relevant target

### Integration Mapping

- Any current call sites in the main execution path that depend on `binary-io.c` should be updated to import and invoke `crate::binary_io::set_binary_mode`
- The module should be declared from the existing crate root or main module using standard Rust module declarations only; no extra helper modules should be added unless required by existing project layout

## Data Model

This module does not define persistent data structures in the provided analysis.

### C to Rust Type Mapping

- No C structs -> no Rust struct/enum equivalents required
- C file descriptor / stream target identifiers used by `set_binary_mode` -> Rust-compatible platform representations:
  - prefer standard library stream selection at call sites if sufficient;
  - if descriptor-level handling is required for parity, use standard OS-specific raw handle/raw fd types from `std::os::*` only

### Memory Management

- No heap-owned module state is expected
- All logic should remain stateless and stack-local
- Resource ownership should remain with existing standard streams; `set_binary_mode` must not assume ownership or recreate them

### Error Handling

- If the underlying platform operation can fail, return `std::io::Result<()>` and propagate errors to the existing caller path
- For platforms where binary mode is irrelevant, return success immediately
- Avoid panic-based handling for normal OS-level failure cases

## Implementation Phases

### Phase 1: Inspect and Place the Module Port

- Review `binary-io.c` and confirm:
  - the exact `set_binary_mode` signature,
  - which stream(s) or descriptors it targets,
  - whether the original implementation reports failure or silently ignores unsupported cases
- Add `src/binary_io.rs`
- Declare the module in the existing Rust crate entry point following the current project structure
- Port the function signature as directly as practical, keeping naming aligned with the C source responsibility

### Phase 2: Implement Platform-Specific Binary Mode Behavior

- Implement the functional core of `set_binary_mode` with `cfg`-gated branches
- On platforms where text/binary mode distinction is meaningful, map the operation to the appropriate Rust/OS-level call using standard library platform types
- On Unix-like targets, implement a no-op success path unless the C source indicates otherwise
- Keep the implementation narrow:
  - no generic I/O framework,
  - no extra stream wrapper types,
  - no configuration surface beyond what the original function requires

### Phase 3: Wire Existing Call Sites and Preserve Behavior

- Update the Rust main-path code that corresponds to the original C call sites to use `set_binary_mode`
- Ensure invocation ordering matches the C execution flow, especially if mode setting must occur before reading from or writing to standard streams
- Preserve existing caller-side behavior for ignored failures vs propagated failures according to the C semantics discovered in Phase 1

### Phase 4: Add Targeted Tests and Validation

- Add unit tests for the no-op path and any platform-conditional behavior that can be validated safely under `cargo test`
- Where direct OS mode mutation is not practical to assert in unit tests, test:
  - successful return behavior,
  - platform gating,
  - absence of unintended stateful side effects
- Confirm compilation on supported targets with conditional code paths kept minimal and well-scoped
- Verify the module remains limited to the original `binary-io.c` responsibility set