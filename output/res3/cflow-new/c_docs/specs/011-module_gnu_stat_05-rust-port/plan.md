# Implementation Plan: module_gnu_stat_05

## Summary

Port the `gnu/stat.c` functionality into a Rust module that preserves the existing module boundary and behavior while replacing C-level allocation and errno-style error propagation with standard Rust filesystem and I/O primitives.

The implementation should focus on migrating:
- `rpl_stat` from `gnu/stat.c`
- any allocation/error-support usage implied by `gnu/xmalloc.c`, only to the extent required by this module

Technical approach:
- Use `std::fs::metadata` / platform-appropriate standard library calls as the primary basis for `stat`-like behavior.
- Represent fallible operations with `std::io::Result` rather than implicit global error state.
- Avoid introducing broader abstractions beyond the module’s current scope.
- Treat `_GL_ATTRIBUTE_PURE` as a C annotation with no direct runtime equivalent in Rust.

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Keep filesystem status lookup to a single metadata fetch per call path where possible.
  - Avoid heap allocation unless required by existing call signatures or path conversion.
  - Preserve behavior close to the C implementation without adding caching or indirection.

## Module Mapping

### Source File Mapping

- `gnu/stat.c` → `src/module_cluster/module_gnu_stat_05.rs`
- `gnu/xmalloc.c` → no standalone Rust port unless this module directly depends on helper behavior; replace required allocation paths with standard owned Rust types (`PathBuf`, `OsString`, `Vec<u8>`, `String`) and normal allocation failure semantics

### Function Mapping

- `rpl_stat`
  - C role: replacement wrapper around `stat`
  - Rust mapping: `pub(crate)` function in `src/module_cluster/module_gnu_stat_05.rs`
  - Suggested Rust shape:
    - internal API should accept `&Path` when possible
    - return `std::io::Result<std::fs::Metadata>` or, if existing project interfaces require, a small Rust struct carrying the required stat fields
  - migration note: preserve edge-case handling from the C implementation rather than broadening semantics

- `_GL_ATTRIBUTE_PURE`
  - C role: compile-time function attribute macro
  - Rust mapping: no direct equivalent required
  - migration note: omit from Rust code; if useful, preserve only as a comment when attached to translated declarations

## Data Model

The analysis lists only anonymous C data structures and one attribute macro. No named persistent data model appears to be central to this module.

### Mapping Rules

- `anonymous` C data structures
  - If they are local helper structs in `stat.c`, convert only if they are required for control flow or field grouping in Rust.
  - Prefer:
    - local tuples for temporary grouped values
    - private Rust structs with named fields if the C code relies on multi-field intermediate state
  - Do not expose these as public API unless demanded by existing call sites.

- `struct stat` usage from C library
  - Rust mapping preference:
    - `std::fs::Metadata` for general file status access
    - if exact field-level compatibility is required by downstream code, define a private adapter struct in this module containing only the fields actually consumed elsewhere

### Memory Management Mapping

- C manual allocation from `xmalloc` paths → Rust ownership via stack values and standard containers
- No manual free logic is required
- Any path/string conversion should avoid unnecessary copies and should use `Path` / `OsStr` directly where possible

### Error Handling Mapping

- C return codes plus `errno` → `std::io::Result<T>`
- If the surrounding crate expects integer-style status returns, isolate that translation at the module boundary rather than throughout the implementation
- Preserve not-found, permission, and invalid-path distinctions to the extent exposed by standard library error kinds

## Implementation Phases

### Phase 1: Establish Rust module skeleton and map `rpl_stat`

- Create `src/module_cluster/module_gnu_stat_05.rs`
- Add the minimal public/internal function signatures needed to replace `rpl_stat`
- Identify the exact expected caller-facing shape:
  - direct metadata return, or
  - adapter struct if existing Rust-side interfaces need explicit stat fields
- Remove `_GL_ATTRIBUTE_PURE` from the translation plan as non-semantic

### Phase 2: Translate filesystem status logic

- Port the core logic from `gnu/stat.c` into idiomatic Rust using `std::fs`
- Use `Path`-based inputs internally
- Reproduce C edge-case behavior relevant to:
  - path handling
  - symlink/follow behavior, if present in `rpl_stat`
  - propagation of OS errors
- Replace any `xmalloc`-backed temporary storage with standard owned Rust values only where necessary

### Phase 3: Align integration boundaries and error semantics

- Adapt return values and call-site expectations so the Rust module matches the surrounding project’s interface conventions
- Ensure any required conversion from `Metadata` to project-local data is kept private and minimal
- Verify there is no leftover dependence on C-style allocation helpers from `gnu/xmalloc.c`

### Phase 4: Add focused tests and finalize migration

- Add unit tests covering:
  - successful stat on existing file
  - missing path error
  - directory path handling
  - permission-related failure where testable in a portable way
- Run `cargo test`
- Confirm the module is limited to the migrated functionality and does not introduce extra wrappers or utilities

## Notes and Constraints

- Keep the port confined to the existing file/function scope from `gnu/stat.c` and only the `xmalloc` behavior directly needed by that scope.
- Prefer private helpers over adding new reusable infrastructure.
- Avoid platform-specific crates unless the C behavior cannot be represented with the Rust standard library alone.
- If exact POSIX field parity is later required by callers, add only a narrow internal adapter rather than broad compatibility layers.