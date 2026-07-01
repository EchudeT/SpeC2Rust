# Implementation Plan

## Summary

Port the logic currently contained in `gnu/vasnprintf.c` into a Rust module that preserves the existing formatting-control behavior used by this file, with special attention to the conditional branches identified in the source analysis. The Rust implementation should stay narrowly aligned to the existing C file rather than redesigning formatting infrastructure.

The technical approach is to translate the file’s internal control flow and buffer-management behavior into safe Rust where possible, using standard-library string and byte-buffer types as the primary replacement for manually managed C memory. Any operations that depend on byte-level formatting assembly should be implemented with `Vec<u8>` and converted to `String` only when UTF-8 assumptions are valid; otherwise, keep internal handling byte-oriented until the final boundary required by the surrounding project API. Error paths that in C rely on null pointers, allocation failure handling, or size checks should become explicit `Result`-based returns in Rust.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve the original module’s asymptotic behavior for dynamic formatting and buffer growth.
  - Avoid unnecessary intermediate allocations during format assembly.
  - Keep buffer expansion amortized and linear relative to produced output size.
  - Maintain behavior suitable for the existing project usage without introducing broader abstractions.

## Module Mapping

| C Source File | Rust Target | Notes |
|---|---|---|
| `gnu/vasnprintf.c` | `src/module_gnu_if_10.rs` | Single-file migration mirroring the original file’s responsibilities and control flow. |
| internal conditional logic (`if` occurrences identified by analysis) | local Rust `if` branches within the translated functions | Preserve branch order and guard semantics from the C implementation. |

If the project already groups module-cluster ports under a dedicated namespace, the file may instead be placed at:

- `src/module_cluster/module_gnu_if_10.rs`

In that case, only mirror the existing repository layout; do not introduce new layering beyond what the current Rust project already uses.

## Data Model

The source analysis reports only an **anonymous** data structure. Since `gnu/vasnprintf.c` commonly relies on local helper state rather than a stable exported type, the Rust port should keep data modeling minimal and function-local unless the C file clearly uses a reusable struct-shaped state block.

| C Representation | Rust Representation | Migration Decision |
|---|---|---|
| anonymous local struct/state | private `struct` with named fields, only if required for readability or ownership control | Introduce only when the C file groups mutable formatting/buffer state that must be passed across helper functions. |
| raw character buffer (`char *`, length, capacity patterns) | `Vec<u8>` or `String` | Use `Vec<u8>` for low-level assembly and size management; convert to `String` only at a defined textual boundary. |
| pointer + size pairs | slices (`&[u8]`, `&str`) or `(Vec<u8>, usize)` as needed | Prefer slices for borrowed inputs and standard container length for owned outputs. |
| sentinel/error return values | `Result<T, E>` | Replace null/error-code paths with explicit typed errors. |

### Memory Management Mapping

- Replace manual allocation/reallocation/free logic with `Vec<u8>` growth and ownership-based cleanup.
- Convert capacity checks from C into explicit `reserve`/`try_reserve`-style logic where allocation failure handling needs to remain visible.
- Avoid exposing raw pointers unless unavoidable for compatibility with already existing internal Rust APIs.

### Error Handling Mapping

- Allocation and size-overflow checks should map to a small module-private error type.
- Invalid formatting state previously represented by impossible branches or error codes in C should become `Err(...)`.
- If the surrounding crate already has a common error type, adapt to it without widening this module’s scope.

## Implementation Phases

## Phase 1: Source Audit and Rust Skeleton

- Inspect `gnu/vasnprintf.c` and enumerate the actual translated functions, local helpers, and anonymous state blocks.
- Create the Rust destination file matching the project’s established module layout.
- Define narrow function signatures for the translated entry points based on current project call sites.
- Establish a private error type and select the internal output representation (`Vec<u8>` first, `String` only where justified).
- Record every C conditional branch that affects allocation, truncation, width/precision handling, or output termination so they can be migrated in order.

**Deliverable**: compilable Rust module skeleton with function stubs, private types, and tests placeholders.

## Phase 2: Core Control-Flow and Buffer Translation

- Translate the main formatting/buffer-building path from `gnu/vasnprintf.c` into Rust.
- Migrate branch logic in the same order as the C source, especially the analyzed `if` sites and any neighboring guards tied to capacity and formatting state.
- Replace raw buffer writes with `Vec<u8>` push/extend operations.
- Implement size and capacity checks explicitly to preserve the original failure behavior where relevant.
- Keep helper routines private and colocated in the same Rust file unless the existing Rust project structure already requires otherwise.

**Deliverable**: working Rust implementation of the file’s primary logic with preserved branch behavior and explicit error returns.

## Phase 3: Data-State Refinement and Edge-Case Handling

- Introduce a small private struct only if the translated code benefits from grouping mutable state such as output buffer, current length, and formatting flags.
- Resolve all C-specific edge cases:
  - zero-length output handling
  - capacity growth boundaries
  - termination/finalization behavior
  - invalid or unsupported formatting states encountered in the original file
- Ensure ownership and borrowing eliminate all manual cleanup paths from the C implementation.

**Deliverable**: idiomatic but behavior-faithful Rust translation with stable internal state management.

## Phase 4: Verification and Cleanup

- Add `cargo test` coverage focused on:
  - normal formatting flow
  - branch-sensitive edge cases derived from the original `if` logic
  - empty and large-output cases
  - error-path behavior for size/allocation-related checks
- Compare Rust behavior against the observable behavior of the C implementation for representative inputs from this module’s usage.
- Remove any leftover placeholder code and keep the module surface minimal.

**Deliverable**: completed Rust port of `module_gnu_if_10` passing tests and integrated on branch `016-module_gnu_if_10-rust-port`.