# Implementation Plan

## Summary

Port `gnu/stat.c` into a focused Rust module that preserves the existing behavior of `is_unc_root` without adding new surface area. The Rust implementation should model the original path-inspection logic directly, using standard-library string and path-byte handling where possible, and keep the scope limited to the migrated function and any minimal internal helpers required to express the same rules clearly.

The implementation approach is:
- create a Rust module corresponding to `gnu/stat.c`,
- translate `is_unc_root` into a small, deterministic Rust function,
- preserve edge-case handling around path prefixes and root detection,
- avoid heap allocation unless required by the incoming Rust API shape,
- represent absence/failure through idiomatic Rust return types only if the original function semantics require it; otherwise keep a simple boolean result.

## Technical Context

### Language/Version
- Rust 1.78+ stable

### Primary Dependencies
- Rust standard library only
- No third-party crates are recommended based on the available module evidence

### Testing
- `cargo test`

### Performance Goals
- Match the constant-time/linear-scan characteristics of the original C logic for a single path check
- Avoid unnecessary allocations and path normalization
- Keep branch structure straightforward so the migrated function remains a thin behavioral port

## Module Mapping

### C to Rust File Mapping
- `gnu/stat.c` -> `src/module_gnu_stat.rs`

### Function Mapping
- `is_unc_root` -> `pub(crate) fn is_unc_root(...) -> bool`

### Rust Module Placement
- Register `src/module_gnu_stat.rs` from the crate root (`src/lib.rs` or existing module entry point) using standard Rust module declarations
- Do not split this migration into additional submodules unless required by existing project layout

## Data Model

The analysis identifies only anonymous C data structures and a single exported function target. Since no named struct usage is evidenced for this function-level migration, the Rust port should avoid inventing persistent data models.

### Data-structure Mapping
- anonymous -> no direct Rust struct unless required by code discovered during implementation

### Representation Notes
- Prefer borrowed inputs (`&str`, `&Path`, or `&[u8]`) based on the exact semantics needed by `is_unc_root`
- If the C function relies on raw byte inspection rather than Unicode path semantics, prefer `&[u8]` internally to preserve exact prefix checks
- If a public-facing Rust wrapper is needed for ergonomics within the crate, keep it thin and delegate to the byte-oriented implementation
- No manual memory-management layer is needed; Rust ownership and borrowing should replace C pointer handling directly

## Implementation Phases

### Phase 1: Module Skeleton and Signature Selection
- Create `src/module_gnu_stat.rs`
- Add the Rust module declaration in the crate root
- Inspect the original `is_unc_root` signature and choose the narrowest equivalent Rust signature that preserves behavior
- Decide whether the implementation should operate on `&str`, `&Path`, or `&[u8]` based strictly on the C function’s character-level behavior
- Keep visibility minimal (`pub(crate)` unless broader visibility is already required by callers)

### Phase 2: Direct Logic Port
- Translate `is_unc_root` into Rust with the same decision order as the C code
- Preserve path-prefix and separator checks exactly, especially for UNC root edge cases
- Replace C pointer arithmetic with indexed or iterator-based byte inspection
- Keep the function allocation-free
- Use simple boolean returns rather than introducing custom error types if the original function is a predicate

### Phase 3: Test Coverage for Behavioral Parity
- Add unit tests in the same module or the standard test module layout
- Cover:
  - valid UNC root inputs
  - similar-but-not-root paths
  - empty and short inputs
  - separator and prefix boundary cases
- Verify that tests reflect original C behavior rather than normalized filesystem behavior

### Phase 4: Integration Review and Cleanup
- Confirm call sites compile against the new Rust function signature
- Remove any temporary translation scaffolding not needed after the direct port
- Ensure there are no unnecessary clones, allocations, or widened APIs
- Run `cargo test` and resolve any platform-specific path-parsing mismatches by tightening the implementation to the original byte-level rules