# Implementation Plan: module_gnu_fd_hook_06

## Summary

Port `gnu/fd-hook.c` to a Rust module that preserves the existing file-descriptor hook registration and dispatch behavior without adding new capabilities. The Rust implementation should keep the same operational shape: maintain an internal hook registry, provide registration and unregistration entry points, and execute registered callbacks for close and ioctl-related events.

The technical approach is to translate the C module into a single Rust source module with private registry state and a minimal set of public functions matching the current function responsibilities:

- `register_fd_hook`
- `unregister_fd_hook`
- `execute_close_hooks`
- `execute_ioctl_hooks`

The implementation should prefer standard-library collections and ownership rules to replace C-managed linked storage and manual lifetime handling. Callback storage should be represented explicitly, with registration records carrying function pointers and any required user data in a way that mirrors the C layout as closely as practical. Error handling should be explicit through `Result` or boolean/status returns, depending on what best matches the calling pattern already present in the Rust port.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve constant-time or near-constant-time registration/unregistration expectations appropriate to the original module size.
  - Keep hook execution overhead low and linear in the number of registered hooks.
  - Avoid unnecessary heap churn during dispatch beyond storage required for the registry itself.
  - Preserve predictable behavior for repeated registration and removal operations.

## Module Mapping

### C to Rust File Mapping

- `gnu/fd-hook.c` → `src/gnu/fd_hook.rs`

If the existing Rust project already has a `gnu` module tree, this file should be added there and exposed only as needed by current callers. No extra module layers should be introduced.

### Function Mapping

- `register_fd_hook` → `pub(crate)` or `pub` Rust function `register_fd_hook`
- `unregister_fd_hook` → `pub(crate)` or `pub` Rust function `unregister_fd_hook`
- `execute_close_hooks` → internal or crate-visible Rust function `execute_close_hooks`
- `execute_ioctl_hooks` → internal or crate-visible Rust function `execute_ioctl_hooks`

Visibility should be kept as narrow as possible based on actual cross-module use.

## Data Model

The analysis only identifies anonymous C data structures, so the Rust mapping should derive directly from actual field usage in `gnu/fd-hook.c` during implementation. The goal is a structural translation, not redesign.

### Expected Mapping Strategy

| C construct | Rust mapping |
|---|---|
| Anonymous hook record struct | Named Rust `struct` representing one registered hook entry |
| Anonymous list node / registry storage | `Vec<HookEntry>` or `LinkedList<HookEntry>` only if the original algorithm requires node-stable removal semantics |
| Anonymous callback group distinctions | Rust `enum` or separate optional function-pointer fields on the hook entry |
| Function pointer fields | Rust function pointer types, preserving signatures as closely as possible |
| Opaque user data pointer | Raw pointer such as `*mut c_void` / `*const c_void` if required by the original API |
| C integer status fields | `i32`, `u32`, or `bool` depending on exact usage |
| Global registry state | Module-private static state with explicit initialization pattern only as required by the current codebase |

### Rust Structure Guidance

A likely minimal shape is:

- `HookEntry`
  - registration identity fields needed for unregistration
  - close-hook callback pointer if present
  - ioctl-hook callback pointer if present
  - user data pointer if present

- `FdHookKind` or equivalent only if the C code differentiates hook categories in a way that benefits from an enum; otherwise keep the original flat record layout.

### Memory Management Notes

- Replace manual C allocation/free logic with owned Rust storage for registry entries.
- If callback APIs require raw user-data pointers, keep them opaque and avoid taking ownership unless the original C module does so.
- Unregistration should remove entries cleanly without leaving dangling references.
- During hook execution, iteration must not create invalid references if removal is possible during dispatch; if the C behavior depends on mutation during traversal, the Rust port should explicitly model that with index-based iteration or temporary snapshots rather than borrowing through mutable iterators.

### Error Handling Notes

- Registration failures should be represented explicitly, ideally with `Result<(), RegisterError>` if allocation or invalid input can fail.
- If the surrounding port expects C-like success/failure codes, keep a narrow integer/bool return and document the mapping internally.
- Hook execution functions should not panic for routine callback outcomes; they should preserve the original status propagation behavior.

## Implementation Phases

## Phase 1: Read-through and Type Extraction

- Inspect `gnu/fd-hook.c` and identify:
  - exact callback signatures
  - registry storage layout
  - unregistration identity rules
  - return-value conventions
  - any mutation-during-dispatch behavior
- Define the Rust module file `src/gnu/fd_hook.rs`.
- Introduce named Rust types for each anonymous C structure actually used by the file.
- Map C scalar and pointer types to Rust equivalents using `std::os::raw` / `core::ffi::c_void` as needed.

**Deliverable**: compilable Rust type skeletons and function signatures matching the C module responsibilities.

## Phase 2: Registry Port

- Implement the internal hook registry using the smallest standard-library container that supports the original operations.
- Port `register_fd_hook` with:
  - entry construction
  - storage insertion
  - duplicate-handling behavior matching the C code
  - explicit failure path for allocation or invalid arguments if present
- Port `unregister_fd_hook` with matching identity comparison and removal semantics.
- Keep visibility and state scoped only to the translated module and its existing callers.

**Deliverable**: registration and unregistration behavior working under unit tests for add/remove cases.

## Phase 3: Dispatch Port

- Port `execute_close_hooks`.
- Port `execute_ioctl_hooks`.
- Preserve callback invocation order from the C implementation.
- Preserve any early-exit, accumulated-status, or ignore-error behavior exactly as observed in the source.
- Ensure iteration remains valid if callbacks can affect registry contents; use the least invasive safe Rust strategy that matches current behavior.

**Deliverable**: hook dispatch behavior implemented and validated against ordering and return-path tests.

## Phase 4: Validation and Cleanup

- Add focused unit tests covering:
  - empty registry dispatch
  - single registration and dispatch
  - multiple registrations and execution order
  - successful unregistration
  - repeated unregistration or missing-entry handling, if defined
  - ioctl and close dispatch separately
- Review unsafe usage and reduce it to the minimum needed for raw callback/user-data interoperability.
- Confirm that final naming, module placement, and public surface match the existing Rust project conventions and do not introduce extra APIs.

**Deliverable**: `cargo test` passing for the translated module with final file integration.