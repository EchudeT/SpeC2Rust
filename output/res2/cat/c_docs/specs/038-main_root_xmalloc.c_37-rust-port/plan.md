# Implementation Plan: main_root_xmalloc.c_37

## Summary

This module ports the allocation helpers from `xmalloc.c` into Rust while preserving their role as centralized checked allocation and reallocation routines. The Rust implementation should keep the existing behavior boundary narrow: provide equivalent helpers for size-checked allocation growth, zero-initialized allocation, and overflow-aware capacity computation, without introducing broader allocator abstractions.

The implementation approach should rely on Rust’s standard allocation primitives and containers where possible, with explicit checked arithmetic for element-count and byte-size calculations. Since the C functions are centered on allocation failure handling and integer overflow prevention, the Rust port should map these concerns to a small internal API that:

- computes allocation sizes with `checked_mul` / `checked_add`,
- performs allocation and resizing through `Vec`, `Box<[u8]>`, or standard allocation APIs as appropriate,
- surfaces unrecoverable allocation conditions through a single module-local failure path consistent with the surrounding port,
- preserves the migration shape of the original function set instead of redesigning the module.

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**: Rust standard library only (`std`, `core`, `alloc` via std)
- **Testing**: `cargo test`
- **Performance Goals**:
  - Maintain constant-time checked size computation overhead relative to allocation calls.
  - Avoid extra copying beyond what reallocation semantics require.
  - Preserve amortized growth behavior for helper routines corresponding to dynamic resizing.
  - Keep zero-fill behavior limited to functions that require it.

## Module Mapping

- **C source**: `xmalloc.c`
- **Rust target**: `src/main_root_xmalloc.rs`

### Function Mapping

Because several listed C names are type-width or macro variants of the same underlying behavior, the Rust module should consolidate implementation internally while preserving recognizable function-level entry points where they are still referenced by migrated code.

| C Function | Rust Mapping | Notes |
|---|---|---|
| `_GL_ATTRIBUTE_PURE` | omitted | C attribute macro has no direct runtime equivalent; purity is expressed by function design only. |
| `xmalloc` | `pub(crate) fn xmalloc(size: usize) -> Box<[u8]>` or equivalent raw byte buffer helper | Checked byte allocation. |
| `ximalloc` | `pub(crate) fn ximalloc(size: usize) -> Box<[u8]>` | Alias-level wrapper if call sites distinguish it; otherwise delegate directly to `xmalloc`. |
| `xcharalloc` | `pub(crate) fn xcharalloc(n: usize) -> Vec<u8>` or `Box<[u8]>` | Character/byte allocation maps to byte buffer allocation. |
| `xrealloc` | `pub(crate) fn xrealloc(buf: Vec<u8>, size: usize) -> Vec<u8>` | Preserve resize semantics for byte buffers. |
| `xirealloc` | `pub(crate) fn xirealloc(buf: Vec<u8>, size: usize) -> Vec<u8>` | Thin wrapper over `xrealloc` if still needed by call sites. |
| `xreallocarray` | `pub(crate) fn xreallocarray<T>(buf: Vec<T>, n: usize) -> Vec<T>` pattern or byte-specialized helper | Must perform overflow check on `n * size_of::<T>()`. |
| `xireallocarray` | same internal implementation as `xreallocarray` | Width variant only; no separate arithmetic model needed in Rust. |
| `xnmalloc` | `pub(crate) fn xnmalloc<T>(n: usize) -> Vec<T>`-or size helper | Element-count allocation with checked multiplication. |
| `xinmalloc` | wrapper to `xnmalloc` | Preserve naming only if referenced. |
| `x2realloc` | `pub(crate) fn x2realloc(buf: Vec<u8>) -> Vec<u8>` | Growth helper built on doubling policy. |
| `x2nrealloc` | `pub(crate) fn x2nrealloc<T>(buf: Vec<T>, n: &mut usize) -> Vec<T>` or equivalent internal growth routine | Should preserve caller-visible updated capacity/count behavior. |
| `xpalloc` | `pub(crate) fn xpalloc<T>(...)` | Implement only to the degree required by existing call patterns; keep signature aligned with migrated callers. |
| `xzalloc` | `pub(crate) fn xzalloc(size: usize) -> Box<[u8]>` or `Vec<u8>` | Zero-initialized byte allocation. |
| `xizalloc` | wrapper to `xzalloc` | Preserve if referenced. |

## Data Model

This module does not define persistent C structs in the provided input. The migration should therefore avoid inventing new data structures and instead map the original allocation helpers onto Rust standard ownership types.

### Data-structure Mapping

| C Data Shape | Rust Mapping | Notes |
|---|---|---|
| raw allocated memory block | `Vec<u8>` or `Box<[u8]>` | Choose based on whether resize is required by the helper. |
| typed element array allocated via count/size helpers | `Vec<T>` | Natural fit for count-based allocation and growth. |
| mutable size/capacity out-parameter patterns | `&mut usize` | Direct replacement for caller-updated count/capacity values. |
| null pointer + failure path | `panic!` / process-terminating helper consistent with port | Rust should not model successful allocation as nullable. |

### Error and Memory Handling Decisions

- Integer overflow checks should be explicit and precede allocation/resizing.
- Allocation failure behavior should be centralized in one helper so the module preserves the original fail-fast style.
- Functions should avoid exposing raw pointers unless required by unmigrated surrounding code.
- Where a C function’s behavior depends on uninitialized memory, the Rust port should prefer safe initialized storage unless a concrete caller requires otherwise; do not expand this into a general unsafe allocation layer.

## Implementation Phases

## Phase 1: Create the Rust module skeleton and size-check helpers

- Add `src/main_root_xmalloc.rs`.
- Define the module-local failure routine for overflow/allocation-related abort behavior, aligned with the project’s existing error style.
- Implement pure arithmetic helpers for:
  - checked byte-count multiplication,
  - checked growth computation,
  - minimum nonzero growth selection where needed for `x2nrealloc`/`xpalloc`.
- Add unit tests for overflow detection and edge-size calculations.
- Keep `_GL_ATTRIBUTE_PURE` as a non-port item.

## Phase 2: Port the fixed-size allocation and zero-allocation entry points

- Implement `xmalloc`, `ximalloc`, `xcharalloc`, `xzalloc`, and `xizalloc`.
- Use standard library allocation-backed containers only.
- Keep wrapper functions where names are still used by migrated code; otherwise delegate internally to a shared byte-allocation helper.
- Add tests covering:
  - zero-size requests,
  - exact-size allocation lengths,
  - zero-filled allocation behavior for `xzalloc` / `xizalloc`.

## Phase 3: Port realloc and array-count allocation functions

- Implement `xrealloc`, `xirealloc`, `xreallocarray`, `xireallocarray`, `xnmalloc`, and `xinmalloc`.
- Preserve C ordering of checks: validate multiplication before resize/allocation.
- Represent resizing with `Vec`-based operations or equivalent standard mechanisms.
- Ensure semantics remain narrow and migration-focused: no generalized allocator traits or alternative backends.
- Add tests for:
  - growth and shrink cases,
  - checked `count * element_size` overflow,
  - preservation of existing prefix contents after reallocation where applicable.

## Phase 4: Port growth-policy helpers and complete call-site alignment

- Implement `x2realloc`, `x2nrealloc`, and `xpalloc`.
- Match the original growth-policy intent closely enough for existing callers, especially:
  - doubling or monotonic expansion behavior,
  - update of caller-visible size/capacity values,
  - upper-bound overflow rejection.
- Adjust signatures only as required by Rust ownership and borrowing rules.
- Add targeted tests for repeated growth, boundary transitions, and overflow-triggered failure paths.
- Finish integration by updating current call sites on branch `038-main_root_xmalloc.c_37-rust-port` to use the Rust module directly, removing dependency on `xmalloc.c` for this module’s responsibilities only.