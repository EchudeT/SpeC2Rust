# Implementation Plan: module_gnu_stat_05

## Summary
This module port centers on migrating the GNU `stat` compatibility wrapper from C into idiomatic Rust while preserving its existing role and call shape as closely as practical within the Rust codebase. The main implementation target is `rpl_stat`, with `gnu/xmalloc.c` considered only insofar as any allocation-related behavior is required by the migrated code path. Since the listed functionality is narrow and no explicit custom data model is exposed, the Rust version should prefer the standard library’s filesystem metadata APIs and `Result`-based error propagation rather than reproducing C-level allocation and errno patterns.

The technical approach is to:
- map `gnu/stat.c` into a small Rust module focused on file-status lookup;
- implement `rpl_stat` as a thin wrapper over `std::fs::metadata` or a similarly direct standard-library call, depending on the exact path semantics required by the existing code;
- avoid introducing extra abstraction layers beyond what is needed to preserve behavior;
- translate C memory/error conventions into Rust ownership and `std::io::Error`.

## Technical Context

### Language / Version
- Rust stable
- Recommended minimum version: **Rust 1.74+**

### Primary Dependencies
- **Rust standard library**
  - `std::fs`
  - `std::path`
  - `std::io`
  - platform-specific `std::os::*` metadata extensions only if required to preserve observed behavior

No third-party crates are recommended based on the available module scope.

### Testing
- `cargo test`

Testing should cover:
- successful metadata lookup for an existing file;
- failure behavior for a missing path;
- directory vs regular-file handling if the original call sites depend on that distinction;
- any path-type edge cases required by the migrated signature.

### Performance Goals
- Maintain near-system-call-bound behavior with no meaningful overhead beyond Rust’s normal error and path handling.
- Avoid unnecessary heap allocation in the `rpl_stat` path.
- Keep the wrapper thin enough that performance remains effectively equivalent to direct metadata queries.

## Module Mapping

### C to Rust File Mapping
- `gnu/stat.c` → `src/module_gnu_stat_05.rs`
- `gnu/xmalloc.c` → **not ported as a standalone module unless directly required by `rpl_stat` migration**

### Function Mapping
- `rpl_stat` → `pub(crate)` Rust function in `src/module_gnu_stat_05.rs`
- `_GL_ATTRIBUTE_PURE` → omitted as a direct construct; represented implicitly by keeping any helper function side-effect free where applicable

### Rust Module Placement
Use a standard Rust layout with a single focused module for this port:
- `src/module_gnu_stat_05.rs`

If the crate already uses `mod` declarations from `src/lib.rs`, add only the necessary module declaration there and do not introduce extra submodules.

## Data Model

The provided analysis lists only anonymous data structures and no named exported struct tied to the module’s API. The implementation should therefore minimize custom data modeling.

### Data-Structure Mapping
- C anonymous/internal structures → no direct Rust struct unless discovered during line-by-line migration
- C `struct stat` usage pattern → `std::fs::Metadata` in Rust where possible

### Type Mapping Notes
- C path input (`char *` / `const char *`) → `&Path` or `&std::ffi::OsStr` at the internal Rust boundary
- C integer/error return convention → `std::io::Result<T>`
- C output-through-pointer for stat data → direct return of `Metadata`, or a small internal adapter only if existing crate interfaces require a different shape

### Memory Management
- Replace any C allocation assumptions with Rust ownership and borrowing.
- Do not recreate `xmalloc` behavior unless a directly migrated code path still needs explicit dynamic allocation.
- Prefer stack values and borrowed paths; only allocate when converting or storing owned path data is unavoidable.

### Error Handling
- Replace `errno`-style signaling with `std::io::Error`.
- Preserve failure cases semantically by propagating I/O errors without inventing recovery logic.
- If callers require C-like success/failure mapping, confine that translation to the module boundary.

## Implementation Phases

### Phase 1: Inspect and Define the Minimal Rust Surface
- Read `gnu/stat.c` and identify the exact behavior of `rpl_stat`, including:
  - whether it follows symlinks or not;
  - expected path parameter form;
  - exact return/output conventions.
- Confirm whether `gnu/xmalloc.c` is actually referenced by this module path.
- Define the Rust function signature that best matches existing crate usage while still using idiomatic `Result`.

**Deliverable:**
- `src/module_gnu_stat_05.rs` created with module skeleton and selected function signature.

### Phase 2: Port `rpl_stat` Using Standard Library Filesystem APIs
- Implement `rpl_stat` with the closest standard-library equivalent:
  - `std::fs::metadata` if symlink-following behavior matches;
  - `std::fs::symlink_metadata` only if required by the original logic.
- Map C path handling to Rust `Path`/`OsStr`.
- Keep helper logic local and minimal; do not create generic filesystem utility layers.

**Deliverable:**
- Working Rust implementation of the stat wrapper with direct error propagation.

### Phase 3: Handle Any Required Compatibility Translation
- If existing crate code expects a C-like contract, add the smallest necessary adapter around the Rust implementation.
- If the original module relied on details from `struct stat`, use `std::fs::Metadata` and platform-specific extension traits only for fields that are actually consumed elsewhere.
- Resolve any remaining allocation-related fragments without porting `xmalloc.c` wholesale.

**Deliverable:**
- Module integrated with the surrounding crate interfaces, with no unnecessary support code.

### Phase 4: Add Focused Tests and Finalize Integration
- Add `cargo test` coverage for success and failure paths.
- Verify behavior for ordinary files and directories, and symlink behavior if applicable.
- Confirm that no extra modules or compatibility scaffolding were introduced beyond this migration’s needs.

**Deliverable:**
- Passing tests and completed module registration in the Rust crate.