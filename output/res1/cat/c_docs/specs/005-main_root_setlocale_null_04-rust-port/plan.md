# Implementation Plan

## Summary
This module ports the C locale-query helpers in `setlocale_null.c` and `setlocale_null-unlocked.c` into a Rust module that preserves the existing behavior and call structure as closely as practical.

The Rust implementation should focus on:
- migrating the existing functions only:
  - `setlocale_null_unlocked`
  - `setlocale_null_r_unlocked`
  - `setlocale_null_r_with_lock`
  - `setlocale_null_r`
  - `setlocale_null`
- keeping the implementation centered on querying locale information with a null locale argument pattern, matching the C module’s current responsibilities
- preserving the distinction between unlocked and locking code paths at the module/function level where it exists in C, without introducing new abstraction layers beyond what is needed for safe Rust
- translating C string and buffer handling into Rust-owned buffers and `String`/byte-slice logic while making allocation failure and invalid locale data explicit in return values

The preferred technical approach is to implement a small Rust module in the main crate that mirrors the current file split and function boundaries. Internal helpers may be used only where necessary to avoid duplicated unsafe or platform-dependent logic, but the migration should remain structurally close to the source C files.

## Technical Context

### Language/Version
- Rust stable, edition 2021
- Minimum practical toolchain target: Rust 1.74 or newer

### Primary Dependencies
- Rust standard library
- No third-party crates are recommended from the provided evidence

### Testing
- `cargo test`

### Performance Goals
- Maintain behavior comparable to the C implementation for locale lookup paths
- Avoid unnecessary allocations beyond what is required to materialize returned locale strings
- Preserve the lightweight nature of unlocked paths where the C code differentiates them
- Keep copying of locale text bounded to the minimum needed for safe Rust ownership

## Module Mapping

### C to Rust File Mapping
- `setlocale_null.c` -> `src/main_cluster/main_root_setlocale_null_04.rs`
- `setlocale_null-unlocked.c` -> `src/main_cluster/main_root_setlocale_null_04.rs`

If the crate already separates modules under `src/main_cluster/`, this module should be added there directly rather than introducing additional submodules. The two C files should be merged into one Rust source file unless the current Rust project layout already requires one-file-per-source mapping.

### Function Mapping
- `setlocale_null_unlocked` -> `pub(crate)` or private Rust function `setlocale_null_unlocked`
- `setlocale_null_r_unlocked` -> `pub(crate)` or private Rust function `setlocale_null_r_unlocked`
- `setlocale_null_r_with_lock` -> private Rust function `setlocale_null_r_with_lock`
- `setlocale_null_r` -> Rust function `setlocale_null_r`
- `setlocale_null` -> Rust function `setlocale_null`

### Visibility Guidance
- Export only the function(s) required by the surrounding crate API
- Keep helper functions non-public unless the existing Rust crate structure requires broader visibility
- Do not create extra façade APIs beyond the migrated C entry points

## Data Model

No named C structs are listed for this module, so the migration should remain function-oriented.

### C to Rust Data Mapping
- `char *` locale result -> `String` when ownership is required by the Rust API, or `Vec<u8>`/`OsString` internally if platform handling needs byte-preserving conversion before validation
- output buffer plus size parameters -> `&mut String`, `&mut Vec<u8>`, or a dedicated return value such as `Result<String, Error>`
- `int`/status return codes -> `Result<_, ModuleError>` or `Option<_>` depending on whether the C function distinguishes specific failure causes
- null pointer input pattern used with `setlocale` -> represented explicitly in Rust control flow; no raw nullable public API unless unavoidable for a direct low-level port

### Error Model
Use a small module-local error type only if the C functions expose distinguishable failure modes such as:
- insufficient output capacity
- invalid locale data returned by the platform
- locale query failure

If the surrounding crate already uses a common error/status style, follow that existing style instead of introducing a new shared abstraction.

### Memory Management Notes
- Replace C-managed buffers with Rust-owned storage
- Keep any platform calls requiring raw pointers tightly scoped
- Convert returned C strings immediately into owned Rust data after null checks
- Ensure no borrowed data outlives temporary C pointers or lock scopes

## Implementation Phases

### Phase 1: Establish Rust module skeleton and function signatures
- Create `src/main_cluster/main_root_setlocale_null_04.rs`
- Add Rust equivalents for:
  - `setlocale_null_unlocked`
  - `setlocale_null_r_unlocked`
  - `setlocale_null_r_with_lock`
  - `setlocale_null_r`
  - `setlocale_null`
- Mirror the current call relationships from C, including the separation between unlocked and lock-aware functions
- Decide the exact Rust return types based on the surrounding crate’s existing conventions for C-port modules
- Add placeholder unit tests for the basic success/failure paths to anchor the migration

### Phase 2: Port unlocked locale query logic and buffer handling
- Migrate logic from `setlocale_null-unlocked.c` first
- Implement the raw locale query path corresponding to a null-locale request
- Translate C string extraction and copying into Rust-owned data
- Reproduce any C size checks or truncation-prevention behavior explicitly
- Keep unsafe code, if needed for platform interop, isolated to the smallest possible section
- Add tests covering:
  - successful locale string retrieval
  - empty or null-like failure cases
  - buffer/result sizing behavior reflected by the Rust API

### Phase 3: Port lock-aware wrappers and top-level entry points
- Migrate the locking/wrapper logic from `setlocale_null.c`
- Implement `setlocale_null_r_with_lock`, `setlocale_null_r`, and `setlocale_null` in the same order they delegate in C
- Preserve existing semantics between unlocked and wrapped entry points without adding new synchronization facilities not evidenced by the source module
- Align error propagation with the chosen Rust return model
- Add tests validating that the wrapper functions match the unlocked path’s observable results

### Phase 4: Integration cleanup and parity verification
- Review function naming, visibility, and module registration in the crate tree
- Remove duplicated conversion logic only where needed to keep the port maintainable and close to the C source
- Validate that all migrated functions compile cleanly with no lifetime or ownership leaks
- Run `cargo test`
- Confirm the final Rust module covers the full function list from the C sources and does not add extra capabilities

## Notes and Constraints
- Keep the migration narrowly scoped to the listed files and functions
- Do not introduce new locale-management features or generalized locale abstractions
- Do not add thread-safety wrappers beyond the semantics already present in the C code
- Prefer standard library types and idioms for ownership and error handling
- Keep function decomposition close to the original source so future parity checks against the C implementation remain straightforward