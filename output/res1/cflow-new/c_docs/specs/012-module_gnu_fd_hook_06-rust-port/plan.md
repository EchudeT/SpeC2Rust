# Implementation Plan: module_gnu_fd_hook_06

## Summary

This module ports `gnu/fd-hook.c` into Rust while preserving the existing hook-registration and hook-execution behavior around file-descriptor operations. The Rust implementation should stay narrowly aligned with the current C module scope: maintain an internal registry of hooks, support registration and unregistration, and execute the registered callbacks for close- and ioctl-related events.

The technical approach is to translate the C module into a single Rust module with equivalent internal state and function boundaries. The hook registry should be represented with standard-library collections and explicit ownership rules so callback storage and removal are deterministic and memory-safe. Error handling should use Rust return types where needed, while preserving the C behavior shape as closely as possible instead of redesigning the API surface. Implementation should prioritize semantic equivalence, especially around callback ordering, removal semantics, and safe mutation of the registry during hook dispatch.

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve low-overhead registration and unregistration suitable for a small in-process hook list.
  - Keep hook execution overhead proportional to the number of registered hooks.
  - Avoid unnecessary allocation during dispatch beyond what is required to safely mirror C mutation semantics.
  - Maintain predictable memory usage through owned callback entries and bounded temporary state.

## Module Mapping

### Source Mapping

| C File | Rust File | Notes |
|---|---|---|
| `gnu/fd-hook.c` | `src/gnu/fd_hook.rs` | Direct module migration with the same operational scope |

### Function Mapping

| C Function | Rust Function | Migration Notes |
|---|---|---|
| `execute_close_hooks` | `execute_close_hooks` | Preserve iteration order and callback invocation behavior |
| `execute_ioctl_hooks` | `execute_ioctl_hooks` | Preserve argument forwarding and dispatch semantics |
| `register_fd_hook` | `register_fd_hook` | Translate registration bookkeeping into owned Rust state |
| `unregister_fd_hook` | `unregister_fd_hook` | Preserve matching and removal semantics without leaking entries |

### Module Placement

The Rust code should remain concentrated in one module corresponding to the original C file. If the crate already exposes GNU-related modules, place this file under the existing `gnu` namespace and re-export only what is currently needed by the surrounding codebase.

## Data Model

The C analysis reports several anonymous data structures. Since the source module is hook-oriented, these should be normalized into explicit Rust types that represent registry entries and internal list state, without adding broader abstractions.

### Data-Structure Mapping

| C Data Structure | Rust Representation | Notes |
|---|---|---|
| anonymous hook entry struct | `struct FdHookEntry` | Stores the registered callback identity and any associated user context needed by the original API |
| anonymous close-hook callback form | `type CloseHook = ...` | Rust callback type alias matching the original call signature as closely as possible |
| anonymous ioctl-hook callback form | `type IoctlHook = ...` | Rust callback type alias matching the original call signature as closely as possible |
| anonymous registry/list node | `Vec<FdHookEntry>` or `Vec<Option<FdHookEntry>>` | Final choice depends on whether removal during iteration must preserve stable positions |
| anonymous temporary iteration state | local snapshot/index state | Use explicit local variables instead of heap-managed traversal nodes |
| anonymous registration token/identity form | function pointer plus context identity | Preserve unregister matching rules from C |
| anonymous module-global state | `struct FdHookRegistry` | Encapsulates the migrated internal mutable state |

### Rust Model Notes

- Prefer explicit structs over anonymous tuple-heavy representations so unregister and dispatch logic remain readable and equivalent to the C flow.
- Use owned storage for registry entries to eliminate manual lifetime and deallocation concerns present in C.
- If callbacks in the C code are plain function pointers with opaque context, represent them directly with function pointer types plus a raw context pointer or equivalent narrow identity field, only if required by the original signatures.
- If unregister works by exact pointer comparison, preserve that comparison behavior in Rust by storing the same comparable identity components.
- If dispatch in C tolerates mutation of the hook list while iterating, use a representation that makes this safe without changing visible behavior, such as deferred removal, tombstoned entries, or iteration over a temporary snapshot of callable identities.

## Implementation Phases

## Phase 1: Establish Module Skeleton and Type Mappings

- Create `src/gnu/fd_hook.rs` and map the C file contents into Rust at function-for-function granularity.
- Define explicit Rust types for:
  - hook entry records
  - callback signatures
  - registry state
- Identify the exact registration key used by the C code for unregistration and encode it into the Rust entry type.
- Decide the minimal internal collection type based on C behavior:
  - `Vec<FdHookEntry>` if simple append/remove is sufficient
  - `Vec<Option<FdHookEntry>>` if stable iteration positions are needed during mutation
- Keep visibility narrow; expose only the functions required to replace the C module.

## Phase 2: Port Registration and Unregistration Logic

- Implement `register_fd_hook` using owned insertion into the registry.
- Implement `unregister_fd_hook` with matching rules equivalent to the C implementation.
- Ensure removal does not leave invalid references or require manual cleanup.
- Convert any C sentinel or integer status results into Rust return values while preserving caller-visible success/failure behavior.
- Add unit tests covering:
  - single registration
  - multiple registrations
  - unregister of an existing hook
  - unregister of a non-existent hook
  - repeated unregister behavior if applicable in C

## Phase 3: Port Hook Dispatch Functions

- Implement `execute_close_hooks` and `execute_ioctl_hooks`.
- Preserve callback invocation order from the C implementation.
- Preserve argument passing exactly, including descriptor values and ioctl-related parameters.
- Handle registry mutation during dispatch according to the original C semantics:
  - either by safe indexed traversal over mutable storage
  - or by snapshotting callable identities before invocation if that better matches the original behavior
- Ensure dispatch does not borrow the registry in a way that prevents legal updates required by the ported logic.
- Add unit tests covering:
  - dispatch with no hooks
  - dispatch with one and multiple hooks
  - unregister effects on subsequent dispatch
  - registration/removal interactions during dispatch, if supported by the C code

## Phase 4: Finalize Semantics and Integrate

- Review all function signatures against the surrounding Rust crate so the module drops into the existing project structure with minimal adaptation.
- Align error handling and return types with existing crate conventions only where required for compatibility.
- Remove any temporary translation scaffolding introduced during porting.
- Add focused regression tests for ordering and identity-based removal semantics.
- Confirm the module builds cleanly and passes `cargo test` on branch `012-module_gnu_fd_hook_06-rust-port`.

## Migration Notes

- Keep the port narrowly scoped to the existing file and listed functions; do not introduce extra facilities beyond what is needed to replace `gnu/fd-hook.c`.
- Prefer standard-library memory management and collections over custom allocators or third-party crates.
- Preserve the original operational assumptions of the C module, especially callback identity, execution ordering, and registry mutation behavior.
- Replace manual C memory handling with Rust ownership, avoiding raw-pointer manipulation except where the original callback API requires identity-compatible opaque context values.