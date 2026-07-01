# Implementation Plan

## Summary
Port the GNU `stat` wrapper logic from `gnu/stat.c` into a focused Rust module that preserves the existing behavior boundary of `rpl_stat` without adding broader filesystem abstractions. The Rust implementation should rely on `std::fs` and platform-specific metadata access from the standard library where needed, with error reporting expressed through `std::io::Result`.

`gnu/xmalloc.c` is present in the source set, but the listed function scope does not require reproducing a general allocator layer. Its relevance in this module should be limited to confirming that no manual allocation behavior from the C side must be preserved for `rpl_stat`. Memory ownership should therefore be handled entirely through Rust values and borrowing.

The implementation should migrate the existing file/function responsibilities directly into a small Rust module, keep the interface narrow, and validate parity with the C behavior through targeted tests around file status lookup and error propagation.

## Technical Context

### Language/Version
- Rust stable, edition 2021
- Minimum recommended compiler: Rust 1.74 or newer

### Primary Dependencies
- Rust standard library only:
  - `std::fs`
  - `std::io`
  - `std::path`
  - `std::os::unix::fs::MetadataExt` on Unix if low-level field parity is required

No third-party crates are recommended because the provided module scope does not justify external dependencies.

### Testing
- `cargo test`

Test coverage should focus on:
- successful stat-like metadata retrieval for existing paths
- error propagation for missing paths
- behavior on regular files and directories
- any path edge cases directly implied by the migrated C logic

### Performance Goals
- Match the practical cost profile of a single filesystem metadata lookup per call
- Avoid unnecessary heap allocation and string copying
- Preserve straightforward syscall-to-result flow with minimal wrapping overhead

## Module Mapping

### C to Rust File Mapping
- `gnu/stat.c` → `src/module_gnu_stat_05.rs`
- `gnu/xmalloc.c` → no direct standalone Rust port required for this module unless code inspection shows a helper used by `rpl_stat`; otherwise omitted from implementation

### C to Rust Function Mapping
- `rpl_stat` → `pub(crate) fn rpl_stat(path: &Path) -> io::Result<RustStat>`
- `_GL_ATTRIBUTE_PURE` → omitted as a direct construct; treated as a non-functional C attribute with no Rust equivalent required

### Rust Module Placement
Use a conventional flat module entry in the crate:
- `src/module_gnu_stat_05.rs`

If the crate already uses `mod` declarations from `src/lib.rs`, add only the single corresponding module declaration needed to expose this ported unit.

## Data Model

The input lists only anonymous data structures and does not identify a named C struct owned by this module. For planning purposes, the Rust side should map only the data actually needed to represent the result of `rpl_stat`.

### Data-structure Mapping
- C anonymous/stat-related output data → `RustStat` struct or direct `std::fs::Metadata`, depending on call-site needs
- C attribute-like metadata declaration → no Rust data type needed

### Recommended Rust Representation
Prefer the smallest migration that fits the current callers:

1. **If callers only need pass/fail and can consume standard metadata immediately**
   - Use `std::fs::Metadata` directly
   - No custom Rust struct required

2. **If callers expect a stable module-owned record analogous to C stat output**
   - Introduce:
     ```rust
     pub(crate) struct RustStat {
         pub(crate) is_file: bool,
         pub(crate) is_dir: bool,
         pub(crate) len: u64,
         // add platform fields only if existing callers require them
     }
     ```
   - On Unix, derive additional fields from `MetadataExt` only when required by the migrated logic

### Memory Management
- Replace all implicit C caller-managed output handling with owned Rust return values
- Eliminate manual allocation concerns from `xmalloc` for this module path
- Borrow input paths as `&Path` or `&OsStr` to avoid unnecessary allocation

### Error Handling
- Map syscall and filesystem failures to `std::io::Error`
- Do not emulate C errno mutation beyond what `io::Error` already captures
- Keep error flow explicit in function return types rather than sentinel return codes

## Implementation Phases

### Phase 1: Source Audit and Interface Fixing
- Inspect `gnu/stat.c` and confirm the exact semantics of `rpl_stat`
- Verify whether `gnu/xmalloc.c` contributes any helper actually used by this function
- Identify the exact output shape expected by existing callers
- Define the Rust function signature and decide between returning `Metadata` directly or a minimal `RustStat` struct
- Add the single Rust module file and crate module declaration

### Phase 2: Core Port of `rpl_stat`
- Implement the Rust equivalent of `rpl_stat` using `std::fs` metadata APIs
- Preserve existing path handling behavior as closely as the standard library allows
- Map result and failure paths into `io::Result`
- Add only the minimal platform-specific accessors needed for caller parity
- Exclude non-functional C attributes such as `_GL_ATTRIBUTE_PURE`

### Phase 3: Caller Alignment and Data Mapping Finalization
- Replace C-style output handling assumptions with returned Rust values
- Adjust internal call sites, if any, to use the new Rust return type
- Remove any unnecessary allocation-oriented patterns inherited from the C design
- Confirm that no extra support module from `xmalloc` is needed after integration

### Phase 4: Verification
- Add `cargo test` coverage for:
  - existing file metadata retrieval
  - directory metadata retrieval
  - missing path error handling
  - any module-specific edge case found during the source audit
- Validate that the implementation remains limited to the original module scope and file responsibilities