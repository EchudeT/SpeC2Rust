# Implementation Plan: module_gnu_execute_all_15

## Summary

This module ports the logic from `gnu/fd-hook.c` that executes registered file-descriptor-related hooks across the current hook lists. The Rust implementation should preserve the existing behavior of iterating through the maintained hook collections and invoking each registered callback for the two existing entry points:

- `execute_all_close_hooks`
- `execute_all_ioctl_hooks`

The technical approach should stay minimal and migration-focused:

- move the behavior of `gnu/fd-hook.c` into a single Rust module file;
- represent the C hook records and hook lists with Rust structs using owned storage where possible;
- model optional callback pointers with `Option<fn(...) -> ...>` or a narrow internal callable representation matching the original call sites;
- keep execution order consistent with the C implementation;
- translate C-style nullability and manual traversal into explicit Rust iteration over stored entries;
- avoid adding new facilities beyond what is required to preserve the current module behavior.

Memory safety is improved by replacing raw list traversal and nullable callback handling with typed Rust containers and `Option`, while error behavior should remain aligned with the original functions rather than introducing new recovery paths.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - preserve the current linear traversal cost of hook execution;
  - avoid unnecessary allocations during hook execution;
  - keep callback dispatch overhead minimal and comparable to direct iteration in C;
  - maintain deterministic iteration order equivalent to the source module.

## Module Mapping

### C to Rust File Mapping

- `gnu/fd-hook.c` -> `src/gnu/fd_hook.rs`

### Function Mapping

- `execute_all_close_hooks` -> `pub(crate) fn execute_all_close_hooks(...)`
- `execute_all_ioctl_hooks` -> `pub(crate) fn execute_all_ioctl_hooks(...)`

The Rust module should contain only the migrated state and helper definitions required by these two functions. Any C-local helper logic currently embedded in `gnu/fd-hook.c` should remain local to `src/gnu/fd_hook.rs` unless an existing Rust crate layout already requires a different visibility boundary.

## Data Model

The input analysis identifies only anonymous C data structures. Since the source file is `gnu/fd-hook.c` and the functions execute hook lists, the Rust data model should map these anonymous records into named internal Rust types based on their role in the file.

### Data Structure Mapping

- `anonymous` -> `struct CloseHookEntry`
  - Represents one registered close-hook record.
  - Fields should correspond directly to the C record fields used during close-hook execution.
  - Callback fields should be represented as `Option<fn(...) -> ...>` when plain function pointers are sufficient.

- `anonymous` -> `struct IoctlHookEntry`
  - Represents one registered ioctl-hook record.
  - Fields should mirror the C record fields consumed by `execute_all_ioctl_hooks`.

- `anonymous` -> `struct CloseHookList`
  - Represents the collection or head state for registered close hooks.
  - Prefer `Vec<CloseHookEntry>` if the C code only needs ordered traversal.
  - If the C code depends on linked layout semantics, model only the needed traversal shape, but do not preserve raw pointers unnecessarily.

- `anonymous` -> `struct IoctlHookList`
  - Represents the collection or head state for registered ioctl hooks.
  - Prefer the same storage strategy used for close hooks for consistency.

- `anonymous` -> `enum HookResult` or direct primitive return type alias
  - Use only if the C code has integer status values that benefit from a constrained internal Rust representation.
  - Otherwise keep the original primitive type mapping.

- `anonymous` -> `type RawFdLike = i32`
  - Use a local alias only if it clarifies signatures migrated from C integer file descriptors.

- `anonymous` -> `struct ModuleHookState`
  - Optional internal grouping type if the C file maintains multiple related hook lists in shared state.
  - Introduce this only when it maps directly to existing file-level state from `gnu/fd-hook.c`.

### C-to-Rust Type Guidance

- C function pointer -> `Option<fn(...) -> ...>` where captures are not needed
- C `int` -> `i32`
- C pointer used only for presence/absence -> `Option<T>` or `Option<NonNull<T>>`
- C linked-node traversal -> `Vec<T>` if insertion/removal semantics from this file do not require node identity
- C nullable callback/context fields -> `Option<...>`

### Memory Management and Error Handling

- Replace null checks with `Option` matching.
- Avoid exposing raw pointers unless required by exact callback signatures.
- Keep ownership of hook storage inside the Rust module or the existing owning crate structure.
- Preserve C return conventions for the exported migrated functions where those conventions affect callers.
- Do not introduce new error enums unless the original code already distinguishes error cases beyond integer/status returns.

## Implementation Phases

## Phase 1: Translate Module State and Signatures

- Create `src/gnu/fd_hook.rs`.
- Inspect `gnu/fd-hook.c` and name each anonymous C structure by operational role.
- Define Rust structs/type aliases for:
  - close hook entries;
  - ioctl hook entries;
  - any list/head/module-level state required by the two execution functions.
- Translate the two public-facing function signatures as closely as possible to the original C call contract.
- Establish internal visibility (`pub(crate)` vs private) strictly according to actual module use.

## Phase 2: Port Hook Execution Logic

- Implement `execute_all_close_hooks` by translating the original iteration and callback invocation order directly.
- Implement `execute_all_ioctl_hooks` with the same direct migration approach.
- Preserve:
  - current traversal order;
  - callback argument passing;
  - return-value propagation;
  - early-exit behavior, if present in C.
- Replace manual C null/pointer checks with Rust `Option` handling and safe iteration.
- Keep helper logic local to this file unless existing Rust project structure already defines the owning state elsewhere.

## Phase 3: Integrate With Existing Crate Layout

- Wire `src/gnu/fd_hook.rs` into the existing Rust module tree using standard `mod` declarations.
- Connect the migrated functions to the existing Rust equivalents of any shared state already present on branch `021-module_gnu_execute_all_15-rust-port`.
- Ensure there is no duplicate state introduced if adjacent ports already define shared hook storage.
- Remove or avoid any temporary compatibility code once the direct Rust path is in place.

## Phase 4: Validation and Tests

- Add focused unit tests for:
  - execution of all registered close hooks in order;
  - execution of all registered ioctl hooks in order;
  - empty hook-list behavior;
  - optional/null callback handling if present in the original C logic;
  - return propagation and early exit behavior, if applicable.
- Use `cargo test` to validate the migrated behavior.
- Confirm the implementation does not allocate during hook execution beyond any preexisting storage setup.
- Review the final code for:
  - no unnecessary unsafe blocks;
  - exact preservation of observable behavior from `gnu/fd-hook.c`;
  - minimal API surface limited to the migrated module.