# Implementation Plan: module_gnu_fd_hook_06

## Summary

This module ports `gnu/fd-hook.c` into a focused Rust implementation that preserves the existing hook-registration and hook-execution behavior around file-descriptor operations. The Rust version should mirror the current C module structure and function boundaries rather than introducing new abstractions or subsystem reshaping.

The implementation approach is to migrate the module into a single Rust module that:
- stores registered hooks in explicit Rust-owned collections,
- preserves registration and unregistration semantics,
- executes close and ioctl hook lists in the existing call order,
- uses Rust lifetime and ownership rules to remove manual memory-management concerns present in C,
- converts C-style status handling into narrow Rust result/boolean conventions only where needed by the migrated API.

The plan keeps the scope limited to the existing file and its functions, with technical emphasis on safe ownership of hook records, predictable mutation of the registration list, and compatibility with the module’s current internal behavior.

## Technical Context

- **Language/Version**: Rust 1.76 or newer
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Maintain effectively constant-time append behavior for hook registration where the C implementation used linear structures with tail insertion or equivalent simple mutation.
  - Keep hook execution overhead limited to linear traversal of the registered hooks for each operation.
  - Avoid unnecessary allocation during hook execution; allocation should occur only during registration or structural mutation.
  - Preserve predictable drop behavior and avoid leaks or dangling references through owned Rust containers.

## Module Mapping

### Source Mapping

| C File | Rust File | Notes |
|---|---|---|
| `gnu/fd-hook.c` | `src/gnu/fd_hook.rs` | Direct module port of the existing hook management logic |

### Function Mapping

| C Function | Rust Function | Migration Notes |
|---|---|---|
| `execute_close_hooks` | `execute_close_hooks` | Preserve iteration order and invocation behavior for close hooks |
| `execute_ioctl_hooks` | `execute_ioctl_hooks` | Preserve ioctl hook traversal and argument passing semantics |
| `register_fd_hook` | `register_fd_hook` | Port registration logic using owned Rust storage for hook entries |
| `unregister_fd_hook` | `unregister_fd_hook` | Port removal logic carefully to preserve identity-based unregister behavior |

### Rust Module Placement

The migrated code should remain narrowly scoped under the project’s normal Rust source tree:

```text
src/
  gnu/
    mod.rs
    fd_hook.rs
```

If the branch already has an equivalent namespace layout, place `fd_hook.rs` into that existing structure instead of adding alternate organization.

## Data Model

The C analysis reports several anonymous data structures. Since the file-level functionality is limited to hook registration and dispatch, the Rust model should reconstruct only the structures actually needed by those four functions.

### Data-Structure Mapping

| C Construct | Rust Construct | Notes |
|---|---|---|
| anonymous hook record for close hook storage | `struct CloseHookEntry` | Stores the close-hook callback and any associated registration identity fields needed for unregister |
| anonymous hook record for ioctl hook storage | `struct IoctlHookEntry` | Stores the ioctl-hook callback and any associated registration identity fields needed for unregister |
| anonymous aggregate registration node/list element | `struct FdHookEntry` or split typed entries | Choose the layout that most directly matches the original C file logic |
| anonymous list head / global registry state | `struct FdHookRegistry` | Owns the active hook collections |
| anonymous callback type fields | Rust function pointer type aliases | Use explicit type aliases for readability and signature preservation |
| anonymous temporary traversal state | local iterator/index variables | Do not materialize as standalone Rust structs unless required by the C control flow |
| anonymous removal/linkage metadata | `usize` index-based removal or explicit linked representation | Prefer `Vec`-backed storage unless the original logic requires stable linkage semantics |

### Recommended Rust Types

The exact callback signatures should be derived from the C function prototypes during implementation. The plan should keep them close to C semantics by using function pointers rather than trait objects unless captured environments are explicitly required by the original code.

Example shape:

```rust
type CloseHook = fn(fd: i32);
type IoctlHook = fn(fd: i32, request: libc_compatible_request_type, arg: libc_compatible_arg_type);
```

If the original C code uses nullable function pointers or optional hook kinds, map them to `Option<fn(...) -> ...>`.

For storage:

```rust
struct CloseHookEntry {
    hook: CloseHook,
    // additional identity fields only if needed by unregister logic
}

struct IoctlHookEntry {
    hook: IoctlHook,
    // additional identity fields only if needed by unregister logic
}

struct FdHookRegistry {
    close_hooks: Vec<CloseHookEntry>,
    ioctl_hooks: Vec<IoctlHookEntry>,
}
```

### Ownership and Memory Management

- Replace C manual allocation and release with Rust-owned containers.
- Avoid raw pointers for internal storage unless required to preserve identity semantics from the C API.
- If unregister depends on pointer identity or exact callback identity, compare function pointers and any companion registration fields directly.
- Ensure removal does not invalidate active iteration in hook execution paths; if the C code assumes no mutation during execution, preserve that assumption and document it in code comments rather than adding synchronization facilities.

### Error Handling

- Convert C integer status returns into `bool`, `Option`, or `Result<(), ErrorKind>` only if the surrounding Rust codebase already uses such forms.
- If external compatibility requires C-like integer return values, keep the Rust function return types minimal and explicit.
- Registration failure cases are likely limited to allocation failure or duplicate/absent entry conditions depending on original semantics; preserve only the conditions evidenced by the C logic.

## Implementation Phases

## Phase 1: Establish module skeleton and callback/data mappings

- Create `src/gnu/fd_hook.rs`.
- Translate the C file’s top-level state into a single Rust registry representation.
- Reconstruct the callback type aliases from the original function signatures.
- Define minimal Rust structs for hook entries and registry ownership.
- Add the module export in `src/gnu/mod.rs` only as needed for the existing branch layout.

### Deliverables
- Compiling Rust module skeleton
- Type definitions for hook callbacks
- Initial registry and entry structs
- No expanded APIs beyond the four migrated functions

## Phase 2: Port registration and unregistration paths

- Implement `register_fd_hook` by translating the C insertion logic into Rust container mutation.
- Implement `unregister_fd_hook` by translating the C removal logic with careful preservation of match criteria.
- Keep mutation logic local to this module; do not introduce generalized registry helpers unless directly reused by these functions.
- Validate that ownership and removal behavior do not leave stale entries or invalid references.

### Deliverables
- Working registration path
- Working unregistration path
- Unit tests covering:
  - registering hooks,
  - unregistering existing hooks,
  - attempting to unregister non-matching hooks if supported by current semantics,
  - preservation of stored hook order after successive registrations

## Phase 3: Port close/ioctl hook execution logic

- Implement `execute_close_hooks` as a direct traversal over registered close hooks.
- Implement `execute_ioctl_hooks` as a direct traversal over registered ioctl hooks.
- Preserve original invocation order and argument forwarding.
- Ensure execution code does not allocate or clone registry contents unless strictly required to satisfy borrowing rules derived from the chosen storage design.

### Deliverables
- Working close hook execution
- Working ioctl hook execution
- Unit tests covering:
  - execution order,
  - execution with no registered hooks,
  - correct forwarding of file descriptor and ioctl arguments,
  - behavior after unregister removes previously active hooks

## Phase 4: Final correctness pass and module-level cleanup

- Review the Rust implementation against the original C file to confirm that all control-flow paths and return conventions have been carried over.
- Remove any migration scaffolding or placeholder types that are not required by the final implementation.
- Add concise module comments documenting any deliberate Rust-side representation choices made to preserve C semantics.
- Run `cargo test` and fix any borrow-check or mutation edge cases exposed by the final tests.

### Deliverables
- Finalized Rust port of `gnu/fd-hook.c`
- Passing `cargo test`
- Minimal documentation comments for maintenance and C-to-Rust traceability