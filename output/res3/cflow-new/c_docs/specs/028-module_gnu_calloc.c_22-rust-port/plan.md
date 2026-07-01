# Implementation Plan: module_gnu_calloc.c_22

## Summary

Port `gnu/calloc.c` into an idiomatic Rust module that preserves the original module boundary and behavior of `rpl_calloc`, with particular attention to allocation-size overflow handling and zero-initialized memory semantics.

The Rust implementation should stay minimal and module-scoped:
- map the single C source file to a single Rust module,
- implement `rpl_calloc` as a focused allocation helper,
- rely on the Rust standard library for allocation and checked size computation,
- express allocation failure and invalid size conditions explicitly rather than through unchecked raw allocation paths where possible.

The technical approach is to reproduce the C logic around `nmemb * size` validation using checked multiplication, then allocate a zeroed buffer using standard Rust facilities. Any boundary where the C implementation signals failure must be represented clearly in Rust, with memory ownership modeled safely and without adding broader allocation abstractions.

## Technical Context

- **Language/Version**: Rust 1.77+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - preserve constant-time size validation prior to allocation,
  - use zero-initialized allocation without redundant initialization passes,
  - avoid extra copies and avoid introducing wrapper layers beyond the migrated function,
  - match expected allocation behavior closely enough that the port does not become a measurable hotspot relative to the C module’s role.

## Module Mapping

### C to Rust File Mapping

- `gnu/calloc.c` → `src/gnu/calloc.rs`

### Function Mapping

- `rpl_calloc` → `pub(crate) fn rpl_calloc(...) -> ...`

### Module Placement

Keep the Rust port within a `gnu` module namespace so the source layout remains recognizable and migration-scoped:

- `src/gnu/mod.rs`
- `src/gnu/calloc.rs`

If the existing crate layout already has a `gnu` namespace, place the port directly there rather than introducing any new intermediate modules.

## Data Model

This module has no named C structs or custom data containers to port.

### Data Mapping

- C scalar size arguments (`size_t`) → `usize`
- C allocated memory block (`void *`) → Rust-owned zeroed byte storage or equivalent allocation result, chosen to match the surrounding crate API
- C null failure result → Rust failure representation appropriate to the crate boundary:
  - preferred internal form: `Result<_, AllocErrorLike>`
  - if the surrounding port requires preserving a nullable allocation interface, use `Option<_>` at the narrowest boundary only

### Memory Management Notes

- Use checked multiplication for `nmemb * size` before allocation.
- Preserve zero-initialization semantics exactly.
- Keep ownership explicit so deallocation is automatic through Rust drops when using owned buffers.
- Do not expose unchecked raw pointers unless required by pre-existing crate interfaces.

## Implementation Phases

## Phase 1: Establish the Rust Module Skeleton

- Create `src/gnu/calloc.rs`.
- Register the module in `src/gnu/mod.rs`.
- Add a direct Rust counterpart for `rpl_calloc` with a placeholder signature aligned to the crate’s existing allocation-facing conventions.
- Determine the narrowest viable return type based on current project usage:
  - owned byte buffer if this module is consumed internally as safe Rust,
  - otherwise a minimal compatibility form that does not force broader architectural changes.

### Deliverables

- Rust file and module registration in place
- Compiling function stub for `rpl_calloc`

## Phase 2: Port Core Allocation Logic

- Implement the size computation using `usize::checked_mul`.
- Map overflow to the function’s failure path instead of permitting wraparound or partial allocation.
- Allocate zeroed memory using standard library facilities appropriate to the chosen return type.
- Keep the implementation local and direct; do not introduce allocator helper layers unless already required by nearby migrated code.

### Key Technical Decisions

- Prefer standard allocation-safe constructs over manual raw allocation.
- Ensure zero-length cases follow the intended crate behavior consistently and do not rely on undefined assumptions from C.
- Keep error signaling explicit and limited to the function boundary.

### Deliverables

- Working Rust implementation of `rpl_calloc`
- Overflow-safe and zero-initializing behavior encoded in Rust

## Phase 3: Validate Behavioral Edge Cases

- Add unit tests covering:
  - normal allocation with small sizes,
  - zero-initialized output content,
  - multiplication overflow rejection,
  - zero-sized argument combinations,
  - large-but-valid size computation behavior where practical in tests.
- Confirm the function does not panic for invalid multiplication inputs and instead follows the selected failure contract.

### Deliverables

- `cargo test` coverage for the migrated function
- Verified handling of edge conditions relevant to C `calloc` semantics

## Phase 4: Integrate and Tighten API Surface

- Adjust visibility to the minimum needed (`pub(crate)` unless broader access is already required).
- Align naming and call sites with the existing Rust port structure without broad refactoring.
- Remove any temporary compatibility code introduced during migration if it is no longer needed after call-site adjustment.

### Deliverables

- Finalized module integrated into the crate
- Minimal, migration-focused API surface with no extra abstractions