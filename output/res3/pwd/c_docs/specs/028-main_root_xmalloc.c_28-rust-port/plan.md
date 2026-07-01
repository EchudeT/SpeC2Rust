# Implementation Plan

## Summary

Port `xmalloc.c` into a single Rust module that preserves the existing allocation-helper role of the C code: centralized heap allocation, resizing, zero-initialized allocation, element-count × element-size checked allocation, and growth-oriented reallocation helpers.

The Rust implementation should stay narrow in scope and mirror the existing C entry points as closely as practical on branch `028-main_root_xmalloc.c_28-rust-port`. The main technical approach is:

- migrate the functions from `xmalloc.c` into one Rust source module;
- implement checked size arithmetic explicitly to preserve overflow behavior;
- use standard library allocation containers and raw buffer primitives only as needed to match the C helpers;
- keep failure handling centralized and consistent, with no expansion into broader memory frameworks.

Because this module is fundamentally about allocation policy rather than domain data, the Rust port should focus on:
- translating size computations from `size_t`/`idx_t`-style C usage into `usize`-based checked arithmetic;
- preserving the distinction between uninitialized-capacity style allocation helpers and zeroed allocation helpers where the original interfaces require it;
- making reallocation and growth helpers deterministic and overflow-safe.

## Technical Context

### Language/Version
- Rust stable, edition 2021
- Minimum recommended compiler: `rustc 1.74+`

### Primary Dependencies
- Rust standard library only

Recommended standard components:
- `std::alloc` for low-level allocation behavior when direct buffer control is needed
- `std::process` only if the original module semantics require immediate termination on allocation failure
- `std::mem` and `std::ptr` for byte/count calculations and initialization details
- `std::num` patterns via `checked_mul`, `checked_add`, and related integer methods for overflow checks

No third-party crates are recommended because the input only shows a direct C allocation-helper migration and does not justify external dependencies.

### Testing
- `cargo test`

Testing focus:
- unit tests for each size-calculation helper;
- overflow-path tests for `n * s` and growth calculations;
- zero-initialization tests for the zeroed allocation functions;
- reallocation behavior tests covering grow/shrink transitions and boundary conditions.

### Performance Goals
- keep allocation helper overhead near-zero beyond required overflow checks;
- avoid redundant initialization except for explicitly zeroing APIs;
- preserve amortized growth behavior for the dynamic resizing helpers;
- ensure no additional abstraction layers are introduced beyond what is necessary to replace the C implementation.

## Module Mapping

### Source File Mapping
- `xmalloc.c` -> `src/main_root_xmalloc.rs`

If the crate already has a module tree for this cluster, expose it with the smallest possible change, for example:
- `src/lib.rs` or existing module root: `mod main_root_xmalloc;`

### Function Mapping
Map the existing C functions into Rust functions with closely corresponding names, adapting only where Rust keywords or type conventions require it.

- `_GL_ATTRIBUTE_PURE`
  - no direct Rust item
  - handled implicitly by using side-effect-free helper functions where applicable

- `xmalloc`
  - Rust: `pub(crate) fn xmalloc(size: usize) -> ...`
  - responsibility: allocate a buffer/object region of `size` bytes with centralized failure behavior

- `ximalloc`
  - Rust: `pub(crate) fn ximalloc(size: usize) -> ...`
  - responsibility: same allocation policy for the index-sized variant used by the C source; in Rust this likely collapses to `usize` unless surrounding code requires a distinct alias

- `xcharalloc`
  - Rust: `pub(crate) fn xcharalloc(size: usize) -> ...`
  - responsibility: allocate character/byte storage; likely maps to byte-buffer allocation in Rust

- `xrealloc`
  - Rust: `pub(crate) fn xrealloc(buf: ..., size: usize) -> ...`
  - responsibility: resize an allocation to an exact byte count

- `xirealloc`
  - Rust: `pub(crate) fn xirealloc(buf: ..., size: usize) -> ...`
  - responsibility: index-sized reallocation counterpart; same Rust size domain unless a project alias exists

- `xreallocarray`
  - Rust: `pub(crate) fn xreallocarray(buf: ..., n: usize, s: usize) -> ...`
  - responsibility: checked `n * s` then reallocate

- `xireallocarray`
  - Rust: `pub(crate) fn xireallocarray(buf: ..., n: usize, s: usize) -> ...`
  - responsibility: index-sized checked array reallocation

- `xnmalloc`
  - Rust: `pub(crate) fn xnmalloc(n: usize, s: usize) -> ...`
  - responsibility: checked array allocation

- `xinmalloc`
  - Rust: `pub(crate) fn xinmalloc(n: usize, s: usize) -> ...`
  - responsibility: checked array allocation for index-sized callers

- `x2realloc`
  - Rust: `pub(crate) fn x2realloc(buf: ..., size: &mut usize) -> ...`
  - responsibility: grow allocation using the original doubling-style policy

- `x2nrealloc`
  - Rust: `pub(crate) fn x2nrealloc(buf: ..., n: &mut usize, s: usize) -> ...`
  - responsibility: grow element-count capacity using checked arithmetic and update count in place

- `xpalloc`
  - Rust: `pub(crate) fn xpalloc(buf: ..., n: &mut usize, n_incr_min: usize, n_max: Option<usize>, s: usize) -> ...`
  - responsibility: generalized growth helper preserving the existing bounded-growth semantics

- `xzalloc`
  - Rust: `pub(crate) fn xzalloc(size: usize) -> ...`
  - responsibility: zero-initialized allocation

- `xizalloc`
  - Rust: `pub(crate) fn xizalloc(size: usize) -> ...`
  - responsibility: index-sized zero-initialized allocation counterpart

### Notes on Return Types
The exact Rust return types should be chosen based on how these helpers are consumed elsewhere in the port:
- if callers operate on raw memory regions, use raw allocation representations with explicit ownership handling;
- if callers only need byte storage, prefer `Vec<u8>`/boxed slices where that preserves semantics;
- do not invent generic abstractions beyond the immediate need to replace the C functions.

## Data Model

This module has no explicit C structs in the provided input.

### Type Mapping
Since the C file is an allocation-helper unit, the relevant migration is type-level rather than struct-level:

- C `size_t` -> Rust `usize`
- C index/count aliases used by `xi*` helpers -> Rust `usize` unless the surrounding project already defines a dedicated alias
- C raw pointer allocations -> Rust ownership-bearing buffer type or raw pointer wrapper, selected to match existing call sites without widening scope

### Error/Failure Model
C allocation helpers of this family typically centralize out-of-memory and overflow handling. The Rust port should preserve one consistent failure model across all functions:

- arithmetic overflow in size computation is detected before allocation;
- allocation failure follows the same project-level behavior across all helpers;
- avoid mixed strategies inside this module.

If the surrounding port already uses infallible “terminate on allocation failure” semantics, keep that behavior here. If not yet established, isolate the policy in one internal helper so later call-site migration remains local to this module.

## Implementation Phases

## Phase 1: Establish the Rust module skeleton and shared size-check helpers

### Goals
- create the Rust file for the module;
- define the minimal internal helpers needed for checked size arithmetic and centralized allocation failure handling;
- map C integer sizing behavior into Rust `usize`.

### Tasks
- add `src/main_root_xmalloc.rs`;
- expose the module from the current crate root with the smallest required change;
- implement internal helpers for:
  - checked multiplication for `n * s`;
  - checked growth arithmetic for doubling/increment paths;
  - centralized failure path for overflow/allocation failure;
- document in code which functions are intended to mirror which C entry points.

### Completion Criteria
- module compiles;
- checked arithmetic helpers have unit tests for normal and overflow cases;
- there is one clear internal failure path used by later functions.

## Phase 2: Port the fixed-size allocation and exact reallocation functions

### Goals
- migrate the direct allocation APIs before the growth APIs;
- preserve byte-count and zero-initialization semantics.

### Tasks
- implement:
  - `xmalloc`
  - `ximalloc`
  - `xcharalloc`
  - `xrealloc`
  - `xirealloc`
  - `xzalloc`
  - `xizalloc`
- keep `xi*` variants aligned with the same underlying Rust size type unless an existing project alias requires separation;
- ensure zeroed functions explicitly initialize memory to zero and non-zeroed functions do not add unnecessary initialization.

### Testing
- allocation of small and zero sizes, if the original semantics allow zero;
- resize up and down;
- zeroed buffer content verification.

### Completion Criteria
- all fixed-size and exact-resize helpers compile and pass unit tests;
- behavior is consistent across plain and `xi*` variants.

## Phase 3: Port checked array allocation and array reallocation helpers

### Goals
- migrate the `n × s` APIs with explicit overflow protection;
- keep implementation layered on the phase-2 primitives where appropriate.

### Tasks
- implement:
  - `xnmalloc`
  - `xinmalloc`
  - `xreallocarray`
  - `xireallocarray`
- route all `n * s` calculations through the shared checked arithmetic helpers;
- ensure overflow is detected before any allocation or resize attempt.

### Testing
- successful allocation/reallocation with small counts and element sizes;
- overflow tests at `usize::MAX` boundaries;
- equivalence checks between direct byte-count calls and checked `n × s` calls where sizes match.

### Completion Criteria
- all checked-array helpers are implemented;
- overflow-path tests pass reliably.

## Phase 4: Port growth-oriented reallocation helpers and finalize module integration

### Goals
- migrate the dynamic growth helpers last, after exact allocation behavior is stable;
- preserve the original update-in-place count semantics.

### Tasks
- implement:
  - `x2realloc`
  - `x2nrealloc`
  - `xpalloc`
- mirror the C growth rules as closely as possible rather than replacing them with Rust container heuristics;
- update mutable size/count outputs in the same situations as the C code;
- verify interactions between minimum growth, maximum bounds, and overflow checks;
- finalize any call-site adjustments needed to consume the Rust signatures.

### Testing
- growth from empty and non-empty states;
- repeated growth sequences;
- minimum increment behavior;
- bounded growth behavior for `xpalloc`;
- overflow and maximum-capacity edge cases.

### Completion Criteria
- all listed functions from `xmalloc.c` have Rust counterparts;
- module is integrated into the crate;
- `cargo test` passes for the module and affected call sites.