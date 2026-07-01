# Implementation Plan

## Summary

Port `hard-locale.c` into a Rust module that preserves the original responsibility of `hard_locale`: determining whether the active locale for a category is a real non-trivial locale rather than the default `"C"` or `"POSIX"` setting.

The Rust implementation should stay minimal and close to the existing C behavior. Because locale category queries are not fully covered by the Rust standard library, the implementation should use direct libc bindings for `setlocale` in read-only query mode and convert the returned C string into a Rust string view for comparison. The port should keep the logic isolated to a single Rust source file and expose one function matching the existing module role.

## Technical Context

- **Language/Version**: Rust 1.78+ edition 2021
- **Primary Dependencies**:
  - Standard library
  - `libc` crate for locale category constants and `setlocale`
- **Testing**: `cargo test`
- **Performance Goals**:
  - Constant-time locale-name checks after the libc query
  - No heap allocation in the normal path beyond unavoidable string conversion checks
  - Behavior equivalent to the C implementation, with negligible overhead relative to the libc call

## Module Mapping

| C File | Rust File | Notes |
|---|---|---|
| `hard-locale.c` | `src/hard_locale.rs` | Direct port of the locale-check helper |
| `hard-locale.c` | `src/lib.rs` or existing module declaration point | Re-export or wire the module into the crate without adding extra layers |

## Data Model

This module has no owned C struct data to migrate.

| C Construct | Rust Mapping | Notes |
|---|---|---|
| locale category integer parameter | `libc::c_int` | Keeps direct compatibility with libc category constants such as `LC_CTYPE` |
| `char *` returned by `setlocale` | `*mut libc::c_char` / `*const libc::c_char` handled in a narrow unsafe block | Borrow only for immediate inspection; do not free or store |
| boolean return from `hard_locale` | `bool` | Natural Rust mapping |

## Implementation Phases

### Phase 1: Create module skeleton and API mapping

- Add `src/hard_locale.rs`.
- Define the Rust function corresponding to `hard_locale`.
- Use a narrow public signature based on the original C usage:
  - preferred: `pub fn hard_locale(category: libc::c_int) -> bool`
- Add the module declaration in the crate entry point already used by the project.
- Keep naming close to the original file/function to simplify review against the C source.

### Phase 2: Port locale query logic

- Implement the locale lookup with `libc::setlocale(category, std::ptr::null())` to query the active locale without modifying it.
- Handle the raw pointer carefully:
  - if the returned pointer is null, return `false`
  - convert via `std::ffi::CStr`
  - inspect as bytes/string without taking ownership
- Reproduce the C decision logic exactly:
  - return `false` for `"C"`
  - return `false` for `"POSIX"`
  - return `true` for other locale names
- Keep all unsafe operations inside the smallest possible block and document the safety assumptions.

### Phase 3: Integrate error-handling expectations and memory rules

- Do not introduce Rust error types if the original function only returns truthiness.
- Ensure the function never panics on invalid locale bytes:
  - prefer byte-wise comparison against C/POSIX literals rather than requiring UTF-8
- Avoid storing the locale pointer or references beyond the immediate check because the libc-owned buffer may be overwritten by later locale calls.
- Confirm that the function performs no allocation-dependent ownership transfers and no manual memory management.

### Phase 4: Add focused tests and finish wiring

- Add unit tests covering the pure comparison behavior through an internal helper if needed.
- If direct process-locale mutation is too environment-sensitive for unit tests, isolate the string-classification portion into a private helper and test cases such as:
  - `"C"` -> `false`
  - `"POSIX"` -> `false`
  - `"en_US.UTF-8"` -> `true`
  - `"C.UTF-8"` -> `true` if that matches the original C logic
- Keep tests deterministic and avoid broad environment assumptions where possible.
- Run `cargo test` and verify the module is reachable from the crate in the same role as the original C helper.