# Implementation Plan

## Summary
Port `gnu/reallocarray.c` into a focused Rust module that preserves the original module boundary and behavior of `reallocarray`: allocate or resize an array while detecting `nmemb * size` overflow before attempting allocation. The Rust implementation should mirror the C routine’s responsibility rather than broaden it into a general allocation subsystem.

The technical approach is to provide a small Rust API centered on checked size multiplication and reallocation logic using Rust’s allocation facilities. The port should make overflow detection explicit, map allocation failure into a clear result type, and keep ownership and deallocation rules narrow and auditable. Since the source module contains a single function and no custom data structures, the Rust work should remain similarly compact.

## Technical Context
- **Language/Version**: Rust 1.78+ stable
- **Primary Dependencies**: Rust standard library only (`std::alloc`, `std::ptr`, `std::mem`, `std::num` as needed)
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve constant-time overflow checking before allocation.
  - Keep allocation/reallocation overhead equivalent to direct allocator use.
  - Avoid extra copying beyond what reallocation semantics require.
  - Maintain predictable behavior for zero-sized and large allocation requests.

## Module Mapping
| C Source File | Rust Target | Notes |
|---|---|---|
| `gnu/reallocarray.c` | `src/module_gnu_reallocarray.rs` | Direct port of the single-function module. |
| `reallocarray` | `module_gnu_reallocarray::reallocarray` | Rust function implementing checked array-size multiplication and allocation/reallocation behavior. |

## Data Model
This module does not define standalone C structs or enums.

| C Concept | Rust Mapping | Notes |
|---|---|---|
| `void *` input/output pointer | Raw pointer (`*mut u8`) or a narrow internal raw allocation representation | Keep raw-pointer semantics only where needed to preserve low-level reallocation behavior. |
| `size_t` | `usize` | Natural mapping for object and allocation sizes. |
| allocation failure / overflow signaling | `Result<_, ReallocArrayError>` or equivalent internal error enum | Separate overflow from allocator failure in implementation, even if public behavior is kept minimal. |

## Implementation Phases

### Phase 1: Establish module skeleton and API surface
- Create `src/module_gnu_reallocarray.rs`.
- Add the Rust function corresponding to `reallocarray` with a signature aligned to the expected project usage.
- Keep the implementation scoped to this module only; do not introduce broader allocation abstractions.
- Define a minimal internal/public error representation for:
  - size multiplication overflow
  - allocation or reallocation failure
- Document ownership expectations of the input pointer and returned pointer so later phases can preserve C semantics safely.

### Phase 2: Implement checked size computation and allocation path
- Port the core `nmemb * size` logic using `usize::checked_mul`.
- Handle zero-size cases explicitly and consistently.
- Implement allocation/reallocation through the Rust standard allocator interfaces.
- Ensure pointer handling remains valid for:
  - null input pointer as allocation request
  - non-null input pointer as resize request
- Keep unsafe blocks tightly bounded around allocator calls and raw pointer operations.
- Make failure paths explicit and avoid hidden panics.

### Phase 3: Validate memory behavior and edge cases
- Add unit tests covering:
  - successful allocation with small sizes
  - successful reallocation growth
  - successful reallocation shrink
  - overflow detection on `nmemb * size`
  - null-pointer allocation path
  - zero-sized request behavior
  - allocation failure handling where testable without platform-specific assumptions
- Verify no double-free or ownership ambiguity is introduced by the Rust translation.
- Confirm the module builds cleanly with `cargo test`.

### Phase 4: Final integration review
- Review the Rust file against the original C module to confirm only existing functionality was migrated.
- Check that naming, file placement, and exported surface are consistent with the branch scope.
- Tighten comments to focus on safety invariants, allocation contracts, and overflow rationale.
- Remove any incidental helpers not required by this one-module port.