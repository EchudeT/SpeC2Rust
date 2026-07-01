# Implementation Plan

## Summary

Port `gnu/fd-hook.c` into a focused Rust module that preserves the existing hook-execution behavior for two call paths:

- `execute_all_close_hooks`
- `execute_all_ioctl_hooks`

The Rust implementation should stay narrow: migrate the hook iteration and invocation logic from the C file into a single Rust module within the existing crate layout, without introducing broader abstractions or new subsystem boundaries.

Technical approach:

- Represent the C hook records and their sequencing using Rust structs and collections from the standard library.
- Preserve execution order and side-effect behavior as closely as possible to the C implementation.
- Model nullable function pointers and optional hook payloads with `Option`.
- Keep memory ownership explicit and simple, preferring owned Rust data where possible and avoiding unsafe code unless the original C layout or callback representation makes it strictly necessary.
- Express C-style status propagation with Rust `Result` or direct return values, depending on the original function contract in the surrounding port.

## Technical Context

### Language / Version

- Rust stable, edition 2021
- Minimum recommended toolchain: Rust 1.75+

### Primary Dependencies

Use the Rust standard library by default.

Recommended crates:

- None required based on the current module evidence.

### Testing

- `cargo test`

Testing focus:

- hook iteration order
- execution of all registered close hooks
- execution of all registered ioctl hooks
- behavior with empty hook lists
- behavior when hook entries contain absent/optional callbacks
- return-value propagation consistent with the chosen Rust signature

### Performance Goals

- Match the C module’s linear traversal characteristics.
- Avoid unnecessary heap allocations during hook execution.
- Keep per-hook dispatch overhead minimal.
- Preserve predictable memory usage with straightforward container choices.

## Module Mapping

### C to Rust File Mapping

- `gnu/fd-hook.c` → `src/module_gnu_execute_all_15.rs`

If the crate already uses a module-cluster directory structure, keep the same structure and place the port at the closest existing path, for example:

- `gnu/fd-hook.c` → `src/module_cluster/module_gnu_execute_all_15.rs`

The Rust file should contain only the migrated logic for:

- `execute_all_close_hooks`
- `execute_all_ioctl_hooks`

### Function Mapping

- `execute_all_close_hooks` → `pub(crate) fn execute_all_close_hooks(...)`
- `execute_all_ioctl_hooks` → `pub(crate) fn execute_all_ioctl_hooks(...)`

Signature shaping rules:

- Convert raw integer file descriptor arguments to `std::os::fd::RawFd` where applicable.
- Convert C integer status returns to:
  - `i32` if surrounding ported modules still rely on C-style values, or
  - `Result<(), HookError>` only if adjacent migrated code already uses Rust error types.
- Callback parameters should be passed by mutable or shared reference only when mutation is required by the original logic.

## Data Model

The source analysis reports only anonymous C data structures. Because no field layout is provided, the Rust plan should map them conservatively around observed usage in `gnu/fd-hook.c`.

### Data-Structure Mapping Strategy

For each anonymous C struct used by this module:

- anonymous hook node / record → named Rust struct with a module-local name derived from purpose
- anonymous list/container state → named Rust struct or `Vec<...>` depending on actual usage
- anonymous callback-bearing entry → Rust struct containing callback fields as `Option<fn(...) -> ...>` or boxed closure equivalents only if necessary

### Recommended Rust Types

Use purpose-based names after inspecting actual field use in the C source, likely along these lines:

- close-hook entry struct → `CloseHook`
- ioctl-hook entry struct → `IoctlHook`
- hook registry/list struct → `HookRegistry` or separate per-hook-list structs if the C code keeps them distinct

### C-to-Rust Field Mapping Rules

- C function pointer → `Option<fn(...) -> ...>`
- C raw pointer to context/user data:
  - if non-owning and lifetime-bounded in module usage, use references with explicit lifetimes
  - if opaque and identity-only, use `*mut c_void` / `*const c_void` only when unavoidable
- C linked-list next pointer:
  - prefer `Vec<T>` if the Rust port can own the sequence outright
  - otherwise use index-based traversal rather than self-referential structures
- C integer flags / request codes → `i32`, `u32`, `u64`, or libc-compatible integer type based on actual call signatures
- C nullability → `Option<T>`

### Memory Management

- Replace manual lifetime and null checks with Rust ownership and `Option`.
- Avoid recreating C linked-list allocation patterns unless they are required by surrounding code.
- If callback context must remain opaque, keep it borrowed or raw but do not add ownership layers not present in the source behavior.
- Any unavoidable unsafe block should be confined to callback invocation or opaque pointer handling and documented inline.

### Error Handling

- Preserve the original control-flow semantics:
  - if the C functions always run all hooks and ignore intermediate callback failures, the Rust version should do the same
  - if the C functions return early on error, preserve that exact traversal behavior
- Prefer simple error representation local to this module only if needed by the migrated signature.
- Do not introduce global error frameworks.

## Implementation Phases

### Phase 1: Source Inspection and Rust Skeleton

- Inspect `gnu/fd-hook.c` to identify:
  - actual anonymous struct roles
  - callback signatures
  - storage model for close and ioctl hooks
  - return-value conventions
- Create the Rust module file at the selected `src/...` path.
- Define minimal named Rust structs and type aliases corresponding only to the data touched by:
  - `execute_all_close_hooks`
  - `execute_all_ioctl_hooks`
- Add module-private helper types only where required to express callback signatures cleanly.

### Phase 2: Port Hook Data Traversal and Execution

- Implement Rust equivalents of the hook containers/records.
- Port `execute_all_close_hooks` first:
  - translate iteration order exactly
  - preserve null-callback handling semantics
  - preserve return behavior
- Port `execute_all_ioctl_hooks` second using the same structural approach.
- Keep the implementation local to this module; do not generalize into a reusable hook framework.

### Phase 3: Memory and Signature Tightening

- Replace any initial placeholder raw-pointer usage with safer references where the call graph allows.
- Minimize unsafe code to the smallest callback boundary, if any remains necessary.
- Align integer and descriptor types with the rest of the Rust port.
- Verify that the Rust data model does not allocate or clone during hook execution unless the C logic inherently requires it.

### Phase 4: Tests and Behavioral Validation

- Add unit tests covering:
  - no hooks registered
  - one hook registered
  - multiple hooks with verified execution order
  - absent optional callback entries
  - ioctl path argument forwarding
  - close path argument forwarding
  - return propagation behavior
- Run `cargo test`.
- Adjust signatures or internal representation only where tests reveal mismatch with the original C behavior.