# Implementation Plan

## Summary

Port `gnu/fd-hook.c` into a focused Rust module that preserves the existing hook-execution behavior for:

- `execute_all_close_hooks`
- `execute_all_ioctl_hooks`

The Rust implementation should mirror the current C control flow and data access patterns as closely as practical, while replacing raw pointer and manual list handling with standard Rust ownership and borrowing. The module should remain narrow in scope: migrate the existing file and the functions it contains, without introducing new abstractions beyond what is needed to represent hook storage and iteration safely.

The implementation approach is:

- create a Rust module corresponding to `gnu/fd-hook.c`
- model the internal hook entries and hook collections with private Rust structs/enums
- preserve execution order and callback invocation semantics from the C implementation
- use explicit result handling where fallible operations exist, but avoid inventing new error surfaces if the original logic is effectively infallible
- keep memory lifetime managed by Rust containers instead of manual allocation/free logic

## Technical Context

### Language/Version

- Rust 1.78 or newer

### Primary Dependencies

Use the Rust standard library only:

- `std` for collections, ownership, and basic OS-integrated integer types where needed

No third-party crates are recommended based on the available module input.

### Testing

- `cargo test`

Testing should cover:

- hook traversal for all registered close hooks
- hook traversal for all registered ioctl hooks
- empty-hook behavior
- ordering behavior consistent with the C implementation
- callback invocation with expected arguments

### Performance Goals

- maintain linear traversal cost equivalent to the C implementation
- avoid unnecessary allocations during hook execution
- keep callback dispatch overhead minimal
- preserve low-level behavior suitable for repeated file-descriptor hook processing

## Module Mapping

### Source File Mapping

- C: `gnu/fd-hook.c`
- Rust: `src/gnu/fd_hook.rs`

If the project already exposes a GNU-oriented module tree, use:

- `src/gnu/mod.rs`
- `src/gnu/fd_hook.rs`

If the existing Rust crate layout is flatter, keep only the single migrated file and a minimal module declaration required to compile.

### Function Mapping

- `execute_all_close_hooks`
  -> `pub(crate)` or `pub` Rust function with the same responsibility and near-equivalent signature adapted to Rust callback storage

- `execute_all_ioctl_hooks`
  -> `pub(crate)` or `pub` Rust function with the same responsibility and near-equivalent signature adapted to Rust callback storage

Visibility should be set to the minimum required by the surrounding crate.

## Data Model

The analysis identifies several anonymous C data structures. Since only one C file is in scope, these should be converted into private Rust types named by role rather than by synthetic numbering.

### Data-Structure Mapping Strategy

Because the source names are anonymous, map them according to usage in `fd-hook.c`:

- anonymous hook node storing a close callback
  -> `struct CloseHookEntry`

- anonymous hook node storing an ioctl callback
  -> `struct IoctlHookEntry`

- anonymous list/container for close hooks
  -> `struct CloseHookList`

- anonymous list/container for ioctl hooks
  -> `struct IoctlHookList`

- any shared linkage/list element representation used internally
  -> folded into `Vec<T>` storage or a private entry struct field set, rather than reproduced as manual next pointers unless exact semantics require intrusive linkage

- any callback type carriers
  -> Rust type aliases such as:
  - `type CloseHookFn = fn(/* mapped args */);`
  - `type IoctlHookFn = fn(/* mapped args */);`

- any C integral flag/request carrier used for ioctl dispatch
  -> Rust primitive integer type matching C width expectations, typically `libc`-style integer equivalents via `std::os::raw`-compatible primitives if needed, but prefer plain Rust integer aliases when exact external ABI matching is not exposed

### Recommended Rust Shapes

Prefer:

```rust
type CloseHookFn = fn(/* args */);
type IoctlHookFn = fn(/* args */);

struct CloseHookEntry {
    callback: CloseHookFn,
    // migrated metadata fields only if present in C usage
}

struct IoctlHookEntry {
    callback: IoctlHookFn,
    // migrated metadata fields only if present in C usage
}

struct CloseHookList {
    hooks: Vec<CloseHookEntry>,
}

struct IoctlHookList {
    hooks: Vec<IoctlHookEntry>,
}
```

If the C implementation depends on linked-list mutation during traversal, use index-based iteration over a snapshot length or a borrowed slice, depending on exact semantics. Only reproduce a linked structure if the source file truly relies on node-level linkage behavior.

### Memory Management

- replace manual allocation/free and null checks with `Vec`, `Option`, and references
- avoid exposing borrowed callback storage with ambiguous lifetimes; prefer owned containers
- if the original callbacks are static function pointers, represent them as plain function pointers rather than boxed trait objects
- if optional callback slots exist, use `Option<fn(...) -> ...>`

### Error Handling

- if hook execution functions in C return no status, keep Rust return type as `()`
- if C propagates callback return values or aggregates status, represent that explicitly with the smallest faithful Rust type
- do not introduce `Result` unless an actual failure path exists in migrated logic

## Implementation Phases

### Phase 1: Read-through Mapping and Type Definition

- inspect `gnu/fd-hook.c` and identify the exact storage layout behind the anonymous data structures
- determine the exact callback signatures used by close and ioctl hooks
- define private Rust type aliases and structs that correspond directly to the C file’s internal representations
- create `src/gnu/fd_hook.rs` and minimal module declarations required by the crate

Deliverable:

- compiling Rust module skeleton with type definitions and placeholder functions for the two execution routines

### Phase 2: Port Hook Execution Logic

- implement `execute_all_close_hooks` by translating the C iteration logic into Rust
- implement `execute_all_ioctl_hooks` by translating the C iteration logic into Rust
- preserve the original ordering, argument passing, and any conditional execution behavior
- replace pointer traversal and null termination with safe iteration over stored entries
- keep signatures and visibility aligned with crate usage

Deliverable:

- behaviorally equivalent execution functions compiled in Rust

### Phase 3: Integrate State Access and Finalize Semantics

- connect the execution functions to the actual hook storage used by the crate
- ensure any mutable/immutable borrowing matches the C module’s read/update behavior during traversal
- resolve edge cases such as empty lists, optional callbacks, or callback removal assumptions if present in the source
- confirm that the implementation does not broaden scope beyond `fd-hook.c`

Deliverable:

- integrated Rust port of the C module with matching internal behavior

### Phase 4: Tests and Cleanup

- add unit tests for empty hook sets, single hook execution, multiple hook execution, and ordering
- add ioctl-specific tests for argument forwarding and callback coverage
- verify that no unnecessary unsafe code remains; if any unsafe is required, constrain it to the smallest local section and document why
- remove dead placeholders and align naming with the final Rust module structure

Deliverable:

- `cargo test` passing for the migrated module and finalized `plan` scope completion