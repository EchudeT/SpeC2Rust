# Implementation Plan: module_gnu_hash_get_13

## Summary

This module ports the bucket- and entry-count access logic from `gnu/hash.c` into a Rust module that preserves the existing behavior and call structure as closely as possible. The Rust implementation should focus on the three existing functions only:

- `hash_get_n_buckets`
- `hash_get_n_buckets_used`
- `hash_get_n_entries`

The technical approach is to translate the existing C read-only hash metadata access into safe Rust APIs wherever possible, while keeping the implementation narrowly scoped to the current file and functions. The port should model the source hash data with explicit Rust structs representing the layout actually needed by these functions, avoid introducing new capabilities, and keep memory ownership simple: borrowed views for existing data and plain integer return values for computed counts.

Because the input indicates numerous anonymous C data structures, the Rust side should avoid inventing a broad new type system. Instead, it should define only the minimal internal structs and helper representations required to support the three functions and the field access they depend on. Any uncertain or layout-sensitive areas should be isolated behind small parsing or accessor helpers so the migrated functions remain direct and testable.

## Technical Context

- **Language/Version**: Rust 1.76+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve the constant-time or linear-time characteristics of the original functions.
  - Avoid unnecessary allocation during metadata reads and count computation.
  - Keep iteration over buckets/entries direct and cache-friendly.
  - Match C behavior closely enough that the Rust port does not introduce measurable overhead for these simple accessors in normal use.

## Module Mapping

### Source-to-target mapping

- **C source file**: `gnu/hash.c`
- **Rust target module**: `src/gnu/hash.rs`

If the crate already exposes a GNU-related namespace, the migrated code should be placed under that existing hierarchy rather than creating additional layers. The Rust module should contain only the migrated equivalents of the current functions and the minimum supporting types/helpers they require.

### Function mapping

- `hash_get_n_buckets`
  -> `pub(crate)` or `pub` Rust function in `src/gnu/hash.rs`, depending on current crate visibility needs

- `hash_get_n_buckets_used`
  -> `pub(crate)` or `pub` Rust function in `src/gnu/hash.rs`

- `hash_get_n_entries`
  -> `pub(crate)` or `pub` Rust function in `src/gnu/hash.rs`

### Implementation style mapping

- C direct field access
  -> Rust struct field access on borrowed references

- C integer count return values
  -> Rust unsigned integer types chosen to reflect the original domain, preferably `usize` for in-memory counts unless exact-width compatibility is required by surrounding code

- C null-sensitive access patterns, if present
  -> `Option<&T>` or explicit validation at function entry, depending on how the wider port represents absent data

## Data Model

The analysis reports only anonymous C structures, so the Rust plan should derive a minimal named model from actual field usage in the three target functions.

### Mapping strategy

Because anonymous structs in C often arise from nested declarations or file-local compound types, the Rust port should not attempt a one-to-one naming expansion for all 20 reported anonymous structures. Instead:

- identify the concrete storage object passed into the three functions,
- extract only the fields read by those functions,
- define narrowly scoped named Rust structs for those fields,
- leave unrelated C anonymous structures unmapped until required by later ports.

### Proposed Rust representations

The exact names should follow the crate’s existing conventions, but the structure should remain minimal.

- **Primary hash container in C (anonymous / file-local aggregate)**
  -> `struct GnuHashData<'a>` or similarly scoped Rust struct
  Purpose: borrowed view over the GNU hash-related data required by the three functions

- **Bucket storage in C (anonymous array or pointer-based field)**
  -> `&'a [u32]`, `&'a [usize]`, or a dedicated newtype wrapper if index semantics need to be made explicit
  Purpose: represent bucket table without allocation

- **Chain / entry storage in C (anonymous array or pointer-based field)**
  -> `&'a [u32]` or `&'a [EntryType]` depending on what the original functions inspect
  Purpose: support entry counting without recreating unrelated parsing logic

- **Optional count-bearing header fragment in C (anonymous nested struct)**
  -> `struct GnuHashHeader` with only required fields, such as bucket count or symbol base, if those values are read directly by the functions

### C-to-Rust type guidance

- C `size_t`, counts, lengths
  -> prefer `usize`
- C fixed-width unsigned hash table values
  -> prefer `u32` when representing GNU hash bucket/chain elements
- C signed sentinel-style integers, if any
  -> use `i32` only if negative values are semantically required; otherwise normalize to unsigned with checked conversion

### Memory management

- Prefer borrowed slices and references over owned buffers for this module.
- Avoid heap allocation in these three functions.
- If raw binary data must be interpreted before counts can be read, perform conversion once in a small constructor/helper and expose validated borrowed views to the accessor functions.
- Do not use `unsafe` unless the surrounding project already stores these structures in a raw-memory form that cannot be expressed safely otherwise. If `unsafe` is unavoidable, confine it to one parsing/access helper and document the invariants precisely.

### Error handling

These C functions likely return counts directly and may assume valid input. In Rust:

- keep count-returning functions simple if the surrounding API already guarantees validated hash data;
- otherwise, use a small error type such as `Result<usize, HashError>` at the boundary where raw or partially validated data is converted into the internal Rust representation;
- avoid embedding error-handling complexity into every accessor if one validated data model can centralize checks.

## Implementation Phases

### Phase 1: Inspect and model the required hash state

- Read `gnu/hash.c` and isolate the exact fields and data dependencies used by:
  - `hash_get_n_buckets`
  - `hash_get_n_buckets_used`
  - `hash_get_n_entries`
- Determine whether these functions operate on:
  - a single hash object,
  - a parsed GNU hash header,
  - bucket and chain arrays,
  - or a larger enclosing object.
- Define the minimal Rust structs and slice-based views needed to represent that state.
- Choose count types (`usize` vs `u32`) based on how the source code uses the values and how they interact with indexing.

**Deliverable**: `src/gnu/hash.rs` skeleton with minimal type definitions and function signatures.

### Phase 2: Port the three functions with behavior-preserving logic

- Implement `hash_get_n_buckets` as a direct accessor over the mapped Rust data.
- Implement `hash_get_n_buckets_used` by translating the original bucket traversal/count logic exactly, preserving any treatment of zero/empty buckets.
- Implement `hash_get_n_entries` by porting the original entry counting logic without adding inferred semantics beyond the C behavior.
- Keep helper functions private and limited to repeated field extraction or iteration patterns already present in the source.

**Deliverable**: working Rust implementations for all three functions in `src/gnu/hash.rs`.

### Phase 3: Validate edge conditions and numeric behavior

- Verify empty-table behavior matches the C implementation.
- Check conversions between stored integer widths and Rust indexing/count types.
- Confirm no accidental panics are introduced by indexing; use slice iteration instead of unchecked indexing where practical.
- If raw data parsing is involved, ensure malformed or truncated input is rejected at construction time rather than causing accessor failure later.

**Deliverable**: finalized internal data model and stable function behavior for valid and invalid inputs as applicable.

### Phase 4: Add focused tests and complete integration

- Add unit tests covering:
  - bucket count retrieval,
  - used-bucket counting with empty and non-empty buckets,
  - entry counting across representative small fixtures,
  - edge cases such as zero buckets or minimal valid tables.
- Where possible, derive test fixtures from the original C expectations or hand-constructed minimal tables matching GNU hash conventions used by the source file.
- Hook the module into the crate’s existing namespace/export pattern without introducing new public API surface beyond what the migrated module requires.

**Deliverable**: `cargo test` coverage for the migrated functions and completed module wiring.