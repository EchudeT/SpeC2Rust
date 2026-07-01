# Implementation Plan: main_root_xmalloc.c_38

## Summary

This module ports the allocation-helper routines from `xmalloc.c` into Rust, preserving the existing role of centralizing infallible or checked memory allocation and duplication operations used by the main program cluster.

The Rust implementation should stay narrow and map each existing C function to a small Rust helper with equivalent intent:

- zero-initialized allocation for element counts
- size-checked allocation variants
- byte-slice duplication
- byte-slice duplication with trailing NUL
- string duplication

The preferred technical approach is to express these operations with standard library owned types (`Vec<u8>`, `Box<[u8]>`, `String`) rather than manual heap management. C overflow-sensitive size computations should be converted into explicit checked arithmetic with `checked_mul`. Allocation failure behavior should follow the surrounding porting strategy for this branch: either panic/abort via standard allocation behavior or return a project-local `Result` only if the current Rust codebase already uses one for fatal utility helpers. No new recovery layer should be introduced in this module.

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Match C utility behavior with no unnecessary extra copying beyond what duplication semantics require.
  - Use contiguous standard allocations (`Vec`, `Box<[u8]>`, `String`).
  - Preserve linear-time duplication characteristics.
  - Ensure size multiplication is checked before allocation to avoid silent overflow.

## Module Mapping

| C File | Rust File | Notes |
|---|---|---|
| `xmalloc.c` | `src/xmalloc.rs` | Direct migration target for allocation and duplication helpers. |

| C Function | Rust Function | Return Shape | Migration Notes |
|---|---|---|---|
| `xcalloc` | `xcalloc<T>` or byte-oriented helper | `Vec<T>` / `Box<[T]>` / byte buffer | Implement zero-initialized allocation using safe standard constructs; check `count * size` where applicable. |
| `xicalloc` | `xicalloc` | byte buffer or generic zeroed container | Preserve checked-count semantics; keep separate only if call sites rely on distinct naming. |
| `xmemdup` | `xmemdup` | `Vec<u8>` or `Box<[u8]>` | Copy exact byte length from source slice. |
| `ximemdup` | `ximemdup` | `Vec<u8>` or `Box<[u8]>` | Checked-size duplication variant; retain separate function for call-site compatibility. |
| `ximemdup0` | `ximemdup0` | `Vec<u8>` | Duplicate bytes and append trailing `0u8`. |
| `xstrdup` | `xstrdup` | `String` | Use owned string cloning when source is valid UTF-8 Rust string data; if ported call sites operate on raw bytes/C strings, use a byte-backed equivalent local to those call sites instead of widening this module. |

## Data Model

This module has no named C structs to migrate.

### Data Mapping

| C Concept | Rust Representation | Notes |
|---|---|---|
| raw allocated zeroed region | `Vec<u8>` / `Box<[u8]>` | Prefer `Vec<u8>` during migration for easy sizing and ownership. |
| duplicated memory block | `Vec<u8>` or `Box<[u8]>` | Choose based on call-site expectations; `Vec<u8>` is simplest if mutability or resizing is needed later. |
| duplicated C string | `String` | Use only where the source is already represented as Rust string data. |
| duplicated bytes with NUL terminator | `Vec<u8>` | Natural fit for `ximemdup0`. |

## Implementation Phases

### Phase 1: Establish Rust module skeleton and allocation semantics

- Create `src/xmalloc.rs`.
- Add the exported function stubs corresponding to:
  - `xcalloc`
  - `xicalloc`
  - `xmemdup`
  - `ximemdup`
  - `ximemdup0`
  - `xstrdup`
- Decide the exact return types by examining current branch call sites and keep them minimal:
  - prefer `Vec<u8>` for raw-memory helpers
  - prefer `String` for string duplication only where existing ported callers already use Rust strings
- Define the module-local policy for checked size arithmetic using `checked_mul`.
- Keep failure behavior aligned with the existing Rust port style rather than introducing new error abstractions.

### Phase 2: Implement byte allocation and duplication helpers

- Implement `xcalloc` and `xicalloc` with:
  - explicit count/size validation
  - zero-initialized allocation via `vec![0; len]`
- Implement `xmemdup` and `ximemdup` as exact-length copies from source slices.
- Implement `ximemdup0` by allocating `len + 1`, copying the source bytes, and setting the trailing byte to `0`.
- Ensure no unsafe code is used unless a specific migrated signature makes it unavoidable; if unavoidable, isolate it to the narrowest possible block and document the invariant.

### Phase 3: Implement string duplication and integrate call sites

- Implement `xstrdup` as a direct owned duplication of the Rust string representation used by migrated callers.
- Update imports and call sites in the main cluster to use `src/xmalloc.rs`.
- Preserve original helper naming where useful to reduce churn during migration.
- Remove any temporary compatibility code created during earlier phases once all call sites compile cleanly.

### Phase 4: Validate edge cases with unit tests

- Add unit tests covering:
  - zero-length allocation
  - checked overflow handling for count/size multiplication
  - exact byte preservation in `xmemdup`/`ximemdup`
  - appended trailing NUL in `ximemdup0`
  - independent ownership after duplication
  - string duplication correctness for `xstrdup`
- Run `cargo test` and fix any mismatches in ownership or allocation sizing behavior.
- Keep tests focused on migrated helper semantics only; do not add unrelated infrastructure.