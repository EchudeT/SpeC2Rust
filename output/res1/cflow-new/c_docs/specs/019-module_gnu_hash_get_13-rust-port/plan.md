# Implementation Plan

## Summary

This module ports the bucket and entry-count access logic from `gnu/hash.c` into Rust, preserving the existing behavior and call boundaries for:

- `hash_get_n_buckets`
- `hash_get_n_buckets_used`
- `hash_get_n_entries`

The Rust implementation should remain narrowly scoped to the existing file and functions, translating the current C data access patterns into explicit Rust types and checked indexing/math where applicable. The main technical approach is:

- model the GNU-hash-related in-memory state with Rust structs representing the C layout at the level needed by these three functions;
- implement direct accessor-style functions with minimal behavioral change;
- replace implicit C nullability and unchecked field access with `Option`, slices, and explicit validation where required by the original usage;
- keep ownership simple and local, preferring borrowed views over introducing new abstraction layers.

## Technical Context

- **Language/Version:** Rust 1.76 or newer
- **Primary Dependencies:** Rust standard library only
- **Testing:** `cargo test`
- **Performance Goals:**
  - Preserve constant-time access for bucket-count and entry-count retrieval.
  - Keep bucket-used counting linear in the number of buckets, matching the expected C traversal cost.
  - Avoid unnecessary allocation and copying; operate on borrowed data or existing owned structures.
  - Maintain predictable memory layout and low overhead without adding wrapper layers not required by the migrated functions.

## Module Mapping

### Source File Mapping

- **C:** `gnu/hash.c`
- **Rust:** `src/gnu/hash.rs`

### Function Mapping

- **C:** `hash_get_n_buckets`
  - **Rust:** `pub(crate) fn hash_get_n_buckets(...) -> ...`
  - Notes: direct field-derived accessor; use Rust integer types aligned to the stored count representation.

- **C:** `hash_get_n_buckets_used`
  - **Rust:** `pub(crate) fn hash_get_n_buckets_used(...) -> ...`
  - Notes: iterate over bucket storage using safe slice traversal; count non-empty/used buckets according to the original sentinel semantics.

- **C:** `hash_get_n_entries`
  - **Rust:** `pub(crate) fn hash_get_n_entries(...) -> ...`
  - Notes: direct accessor or derived count, depending on how the C state currently stores the value.

### Rust Module Placement

Use the standard project layout only:

- `src/gnu/mod.rs`
- `src/gnu/hash.rs`

If the project already has the surrounding module tree, only add the minimal declarations needed to expose this migrated file within the existing structure.

## Data Model

Because the analysis only identifies anonymous C data structures, the Rust plan should map them by role rather than inventing new capabilities. Only the fields needed by the three migrated functions should be represented initially.

### Data-Structure Mapping Strategy

| C structure | Rust representation | Notes |
|---|---|---|
| anonymous GNU-hash holder/context | `struct GnuHash<'a>` or `struct GnuHash` | Primary state container for bucket and entry metadata used by the migrated functions. |
| anonymous bucket array/view | `&'a [u32]`, `&'a [usize]`, or `Vec<u32>` | Choose borrowed slice if data is parsed elsewhere; choose owned vector only if this module already owns decoded storage. |
| anonymous count fields | `u32` or `usize` | Preserve semantics from C; convert to `usize` for indexing only at use sites. |
| anonymous optional pointer-bearing fields | `Option<T>` / `Option<&'a [T]>` | Replaces nullable C pointers. |
| anonymous sentinel-based entries | integer element type with explicit comparisons | Preserve the original “used bucket” test exactly. |

### Initial Rust Types

The implementation should introduce only the minimum Rust types required to support these functions. A likely shape is:

```rust
pub(crate) struct GnuHash<'a> {
    pub(crate) n_buckets: u32,
    pub(crate) n_entries: u32,
    pub(crate) buckets: &'a [u32],
}
```

If the original C code derives `n_entries` rather than storing it, omit that field and compute it the same way in Rust. If the data is owned in this module, replace borrowed slices with `Vec<u32>` and keep the same field set.

### Memory Management Notes

- Replace raw C pointer traversal with slices wherever the data is contiguous.
- Eliminate null checks by representing absent arrays or state as `Option`.
- Keep lifetimes explicit if the Rust module borrows parsed binary data from elsewhere.
- Avoid `unsafe` unless the surrounding project architecture already exposes raw binary views that cannot be expressed safely; if unavoidable, isolate `unsafe` to parsing/binding edges and keep these three functions safe.

### Error Handling Notes

These three functions are count/query operations, so they should not invent new fallible APIs unless the C behavior depends on invalid-state checks. Preferred handling:

- use infallible return types when the original logic assumes valid initialized state;
- use internal assertions or validation at construction boundaries rather than adding recovery logic to the accessors;
- if the existing project already models malformed state, return `Option<usize>` or `Result<usize, _>` consistently with adjacent Rust code, but do not broaden error variants beyond current needs.

## Implementation Phases

## Phase 1: Establish Rust file and core state mapping

- Create `src/gnu/hash.rs` and wire it into the existing Rust module tree.
- Identify the exact C state read by:
  - `hash_get_n_buckets`
  - `hash_get_n_buckets_used`
  - `hash_get_n_entries`
- Define the minimal Rust struct(s) needed to represent those fields.
- Resolve integer-type mapping:
  - C count fields -> `u32` if stored as 32-bit values
  - Rust indexing/count accumulation -> `usize` where needed
- Convert nullable pointers or array references into `Option` and slices as appropriate.

### Deliverables

- Compiling Rust module skeleton.
- Minimal data model covering only fields required by the three functions.

## Phase 2: Port the three functions with behavior-preserving logic

- Implement `hash_get_n_buckets` as a direct accessor over the mapped bucket-count field.
- Implement `hash_get_n_entries` as either:
  - a direct accessor, or
  - the same derivation used by the C source if no dedicated field exists.
- Implement `hash_get_n_buckets_used` by traversing the bucket slice and applying the original used/unused test exactly.
- Keep function signatures narrow and aligned with current project conventions (`pub(crate)` preferred unless wider visibility already exists).
- Ensure conversions between stored integer widths and Rust return/index types are explicit and checked where necessary.

### Deliverables

- Safe Rust implementations of all three functions.
- No added functionality beyond the migrated behavior.

## Phase 3: Validate edge conditions and memory-safety assumptions

- Add unit tests for:
  - zero buckets
  - all buckets unused
  - some buckets used
  - all buckets used
  - entry count retrieval from representative state
- Confirm behavior for empty slices and any sentinel values used by the original C code.
- Verify no out-of-bounds indexing is possible under the chosen data model.
- Review any constructor or parser boundary to ensure the `buckets` slice length matches `n_buckets` if both are stored.

### Deliverables

- `cargo test` coverage for normal and boundary cases.
- Documented assumptions on slice length and valid initialized state.

## Phase 4: Integrate and finalize migration

- Replace any temporary placeholders or compatibility shims with the final Rust calls.
- Align naming and visibility with the rest of the Rust branch.
- Remove any redundant C-style patterns that remain after the port, provided behavior does not change.
- Perform final compile/test pass for the branch `019-module_gnu_hash_get_13-rust-port`.

### Deliverables

- Final Rust module integrated into the branch.
- Passing `cargo test`.
- Completed migration for `gnu/hash.c` limited to the three listed functions.