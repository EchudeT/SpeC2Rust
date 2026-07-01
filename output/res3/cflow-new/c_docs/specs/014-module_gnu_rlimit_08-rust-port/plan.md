# Implementation Plan: module_gnu_rlimit_08

## Summary

Port the C source file `gnu/getdtablesize.c` into a single Rust module that preserves the existing module scope and behavior of `getdtablesize` without adding new facilities. The Rust implementation should focus on reproducing the current file-descriptor table size query logic using idiomatic Rust where possible, while allowing a small amount of platform interop only where the standard library does not expose the required operating-system limit directly.

The implementation approach is:

- map the single C source file to a single Rust source module;
- implement one Rust function corresponding to `getdtablesize`;
- use standard-library integer types and explicit error-aware control flow;
- keep ownership simple by avoiding heap-managed state, since the C module appears to be function-only;
- preserve the operational behavior of returning a descriptor-table size value rather than expanding into a broader resource-limit API.

## Technical Context

- **Language/Version**: Rust 1.77 or newer
- **Primary Dependencies**:
  - Rust standard library
  - `libc` crate, only if needed to access `getrlimit`, `sysconf`, or `RLIMIT_NOFILE`/equivalent constants not exposed by `std`
- **Testing**: `cargo test`
- **Performance Goals**:
  - constant-time lookup with no meaningful overhead beyond the underlying OS query;
  - no heap allocation;
  - behavior suitable as a direct replacement for the C helper in normal process startup/runtime paths.

## Module Mapping

| C File | Rust File | Notes |
|---|---|---|
| `gnu/getdtablesize.c` | `src/module_gnu_rlimit_08.rs` | Direct port of the existing file-level logic |
| `getdtablesize` | `pub(crate)` or module-scoped Rust function `getdtablesize` | Visibility should match actual crate usage; avoid exporting more broadly than needed |

If the project already uses a per-module directory layout, the equivalent constrained mapping is:

| C File | Rust File | Notes |
|---|---|---|
| `gnu/getdtablesize.c` | `src/gnu/getdtablesize.rs` | Prefer this only if the repository already mirrors source subpaths |

## Data Model

This module does not introduce persistent domain data structures. The C analysis only reports an anonymous structure, which is likely incidental to system-call interaction rather than a true module-owned model.

| C Construct | Rust Mapping | Notes |
|---|---|---|
| anonymous | No dedicated Rust struct unless required by syscall interop | Prefer direct use of `libc::rlimit` or scalar values rather than defining a wrapper |
| integer return value from `getdtablesize` | `i32` or `libc::c_int` internally, converted carefully as needed | Preserve C-compatible range expectations |
| resource-limit fields | native integer scalar types (`u64`, `libc::rlim_t`, or converted `i32`) | Conversion must be explicit and checked/truncated only in a way consistent with current C behavior |

## Implementation Phases

### Phase 1: Establish module skeleton and API mapping

- Create the Rust source file corresponding to `gnu/getdtablesize.c`.
- Add the Rust function for `getdtablesize` with a signature aligned to crate conventions and expected C-compatible semantics.
- Keep the implementation local to this module and avoid introducing shared abstractions unless already required by the surrounding codebase.
- Confirm expected return type based on current call sites:
  - prefer `libc::c_int` or `i32` if replacing a C-style function directly;
  - avoid wrapping the result in `Result` unless the surrounding Rust port already standardizes that conversion.

### Phase 2: Port the OS limit query logic

- Translate the existing C control flow into Rust with minimal reshaping.
- Use the standard library first; if it does not expose descriptor-limit queries, use `libc` narrowly for the same syscall or limit access used by the C file.
- Preserve fallback order from the C implementation if it queries multiple sources.
- Handle conversions explicitly:
  - read the OS-provided limit type;
  - cap or translate values to the function’s return type consistently;
  - avoid undefined behavior from unchecked casts.
- Ensure no manual memory management is introduced; any syscall structs should be stack-allocated and initialized safely.

### Phase 3: Error handling and edge-case alignment

- Mirror the C module’s failure-path behavior rather than inventing new error reporting.
- If the original function returns a fallback size when the limit query fails, preserve that exact style in Rust.
- If special values such as “infinite” limits are possible, map them deterministically into the Rust return type in a way consistent with the C implementation.
- Keep panic-free behavior for ordinary runtime failures; the function should return a usable integer result rather than abort.

### Phase 4: Tests and integration verification

- Add unit tests for the Rust function covering:
  - successful retrieval path;
  - result is positive and within `i32`/`c_int` bounds;
  - fallback behavior for mocked or factored error paths, if the implementation structure permits this without extra framework work.
- Add integration coverage only if this module is called through existing crate interfaces.
- Run `cargo test` and verify the module compiles cleanly on the intended Unix-like target(s).
- Confirm the final module layout replaces the original C file’s responsibility without adding unrelated helpers or APIs.