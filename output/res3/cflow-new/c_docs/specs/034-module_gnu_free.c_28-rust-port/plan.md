# Implementation Plan

## Summary
Port `gnu/free.c` into a focused Rust module that preserves the existing module boundary and behavior around `rpl_free` without introducing broader allocation utilities or extra abstraction layers. The Rust implementation should reflect the original module’s narrow role: provide the migrated equivalent of the replacement free routine while relying on Rust ownership and standard allocation behavior wherever possible.

Because `free`-style logic in C operates on raw pointers and explicit deallocation, the Rust port should treat this module as a compatibility boundary. The implementation approach is to:
- map the C source file to a single Rust source file/module,
- migrate `rpl_free` as a minimal function with behavior aligned to the original call pattern,
- keep unsafe code tightly scoped to the exact pointer handling needed,
- prefer standard library allocation primitives and null-safe pointer checks,
- avoid inventing new APIs beyond what is needed to represent the existing module.

## Technical Context

### Language/Version
- Rust stable, edition 2021
- Minimum recommended toolchain: Rust 1.75 or newer

### Primary Dependencies
- Rust standard library only
- No third-party crates are recommended, since the input provides no evidence of external dependency needs

### Testing
- `cargo test`

### Performance Goals
- Preserve constant-time deallocation behavior at the call site
- Avoid additional heap allocations or wrapper objects beyond what is required for the port
- Keep unsafe pointer handling minimal and direct
- Maintain behavior close to the C implementation without adding runtime indirection

## Module Mapping

### Source File Mapping
- `gnu/free.c` -> `src/gnu/free.rs`

### Function Mapping
- `rpl_free` -> `pub(crate)` or private Rust function in `src/gnu/free.rs`, depending on actual crate-internal call usage

### Module Placement
- Add or update `src/gnu/mod.rs` to expose `free`
- Do not create additional helper modules unless required by existing Rust crate structure

## Data Model

This module does not define standalone data structures in the provided input.

### Data-Structure Mapping
- C raw memory pointer arguments used by `rpl_free` -> Rust raw pointers such as `*mut core::ffi::c_void` or a more specific pointer type if dictated by existing call sites

### Memory Model Notes
- C `free(NULL)` semantics should be preserved explicitly if the original module depends on null-tolerant deallocation
- Ownership must remain clear at the call boundary: any pointer passed to the migrated `rpl_free` must represent memory eligible for deallocation by the same allocation strategy assumed in the Rust port
- Unsafe blocks should be restricted to the final deallocation step and documented with the allocation assumptions they require

## Implementation Phases

### Phase 1: Inspect and Define the Rust Module Boundary
- Review `gnu/free.c` and all in-project call sites of `rpl_free`
- Determine the exact signature required in Rust based on how the function is invoked
- Create `src/gnu/free.rs` and wire it into the existing module tree with the smallest necessary visibility
- Document the allocation/deallocation assumptions that must hold for the migrated function

### Phase 2: Port `rpl_free` with Scoped Unsafe Memory Handling
- Implement the Rust equivalent of `rpl_free`
- Preserve null-pointer tolerance if present in the C behavior
- Use standard-library-compatible raw pointer handling only
- Keep the implementation minimal, without introducing generalized allocator wrappers or broader memory APIs
- Ensure the function’s safety contract is explicit in comments and in its signature choice

### Phase 3: Integrate Call Sites and Validate Behavior
- Update existing internal callers to use the Rust module path and signature
- Adjust any pointer type conversions required at the call boundary
- Confirm there are no duplicate deallocation paths introduced by the ownership model
- Add targeted tests covering:
  - null-pointer handling, if applicable
  - valid deallocation path assumptions
  - module compilation and integration with existing crate structure

### Phase 4: Final Cleanup and Conformance Check
- Remove or isolate any leftover C-specific assumptions that are no longer needed after migration
- Verify the module remains narrowly scoped to the original file/function responsibility
- Run `cargo test`
- Confirm the final layout matches the intended one-to-one migration from `gnu/free.c` to `src/gnu/free.rs`