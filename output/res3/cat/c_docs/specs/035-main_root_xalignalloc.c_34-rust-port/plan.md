# Implementation Plan: main_root_xalignalloc.c_34

## Summary

This module ports the C file `xalignalloc.c`, which contains the `xalignalloc` allocation helper, into Rust with behavior focused on aligned memory allocation and failure propagation appropriate to the existing `cat` codebase.

The Rust implementation should stay narrowly scoped to the existing C function and migrate it as a small utility within the main module area rather than introducing broader allocation abstractions. The technical approach is:

- map the single C function to a single Rust function with equivalent allocation semantics;
- use Rust’s low-level allocation APIs from `std::alloc` because the source behavior is explicitly about alignment-sensitive allocation;
- represent allocation failure with explicit error handling or process-terminating behavior only if that is already required by surrounding migrated code, avoiding additional facilities;
- keep ownership and deallocation rules explicit so the allocated region can be safely consumed by the rest of the Rust port.

Because aligned raw allocation in Rust is `unsafe`, the implementation should isolate that unsafety in one small function and document its invariants clearly.

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**: Rust standard library only (`std::alloc`, `std::ptr`, `std::num` as needed)
- **Testing**: `cargo test`
- **Performance Goals**:
  - preserve the constant-time allocation setup expected from the C implementation;
  - avoid unnecessary initialization or copying beyond what the C function performs;
  - maintain requested alignment exactly, subject to Rust allocation layout rules;
  - keep the wrapper thin enough that call overhead is negligible relative to allocation cost.

## Module Mapping

| C Source File | C Function | Rust Target |
|---|---|---|
| `xalignalloc.c` | `xalignalloc` | `src/main_root_xalignalloc.rs` -> `pub(crate) unsafe fn xalignalloc(...)` or `pub(crate) fn xalignalloc(...) -> Result<...>` |

### Mapping Notes

- Since the input contains one source file and one function, the Rust port should remain a single focused module file.
- The final function signature should be chosen based on surrounding migrated call sites:
  - if callers expect raw pointers and manage lifetime manually, keep a raw-pointer-oriented API;
  - if callers can be updated locally without broadening scope, return a small owned allocation wrapper or `NonNull<u8>` paired with size/layout metadata.
- Do not split this into multiple helper modules unless required by compilation or direct call-site migration.

## Data Model

No named C structs are listed for this module.

### Data-structure Mapping

| C Type / Concept | Rust Mapping |
|---|---|
| aligned heap allocation returned as pointer | `*mut u8` or `NonNull<u8>` |
| allocation size | `usize` |
| alignment argument | `usize` |
| allocation layout constraints | `std::alloc::Layout` |
| allocation failure | `Result<_, AllocErrorLike>` or immediate termination consistent with migrated project behavior |

### Memory Management Notes

- The C function likely returns heap memory with caller-managed lifetime. In Rust, this should remain explicit.
- If a raw pointer is returned, the module must also preserve enough layout knowledge at call sites to deallocate correctly.
- `std::alloc::Layout::from_size_align` should be used to validate size/alignment combinations and reject invalid inputs rather than constructing unchecked layouts.
- Alignment must be a non-zero power of two; any C-side assumptions should be converted into checked Rust preconditions.
- Zero-sized allocations should be handled deliberately:
  - either mirror C behavior exactly if known from the source;
  - or normalize through Rust allocation rules in a documented way without inventing extra semantics.

## Implementation Phases

### Phase 1: Inspect and map the existing C allocation contract

- Review `xalignalloc.c` and identify:
  - exact parameter types and return type;
  - whether the function aborts on failure or reports failure to caller;
  - handling of invalid alignment, zero size, and overflow conditions;
  - expected deallocation path used by callers.
- Determine the narrowest Rust signature that preserves existing call patterns.
- Create the Rust module file at `src/main_root_xalignalloc.rs` and wire it into the crate module tree with no extra public surface beyond what migration requires.

### Phase 2: Implement aligned allocation with Rust low-level allocation primitives

- Translate `xalignalloc` using `std::alloc::{alloc, Layout}`.
- Build `Layout` via checked constructors to cover:
  - valid alignment requirements;
  - representable size/alignment combinations.
- Keep unsafe code limited to:
  - calling the allocator;
  - converting allocator results into the chosen return type.
- Preserve failure behavior from the C implementation:
  - if the original is infallible and terminates on allocation failure, keep that behavior in the Rust port at this boundary;
  - otherwise return an error directly without adding recovery layers.
- Add concise safety comments describing:
  - required input invariants;
  - ownership expectations for the returned allocation;
  - deallocation requirements.

### Phase 3: Integrate with migrated callers and deallocation expectations

- Update only the direct call sites that use `xalignalloc`, matching the chosen Rust signature.
- Ensure each caller retains or reconstructs the correct `Layout` information needed for deallocation if raw allocation is exposed.
- Remove C-specific assumptions such as implicit cast behavior or unchecked null handling.
- Confirm that ownership transfer is unambiguous at each caller boundary.

### Phase 4: Validate behavior with focused tests

- Add unit tests covering the Rust module’s observable contract:
  - successful allocation with common alignments;
  - pointer alignment correctness;
  - invalid alignment rejection or failure behavior;
  - zero-size handling as defined by the migrated contract.
- Where practical, verify that allocated memory can be written through safely within the requested size.
- Run `cargo test` and resolve any mismatches between the C assumptions and Rust allocation rules without expanding module scope.