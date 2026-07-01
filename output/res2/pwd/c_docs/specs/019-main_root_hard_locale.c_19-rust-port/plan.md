# Implementation Plan

## Summary

Port `hard-locale.c` into a focused Rust module that preserves the original locale-detection behavior of `hard_locale` without expanding scope. The Rust implementation should expose a small internal function that determines whether the active locale for a requested category is a real locale rather than the default `"C"`/`"POSIX"` locale.

The implementation should prefer Rust standard library facilities where possible, but because locale category queries are not provided by `std`, the plan should use direct libc bindings through the `libc` crate for `setlocale` access. The migration should keep the logic close to the C source: query the current locale for a category, inspect the returned locale name, and return a boolean result. Memory ownership must remain borrowed-only for locale strings returned by libc, with immediate conversion to Rust views and no retention beyond the call.

## Technical Context

- **Language/Version**: Rust 1.77+ edition 2021
- **Primary Dependencies**:
  - `libc` for locale category constants and `setlocale` access
- **Testing**: `cargo test`
- **Performance Goals**:
  - Constant-time locale classification apart from libc string length scan
  - No heap allocation required in the hot path unless string conversion demands it
  - Match C behavior closely with negligible overhead relative to the original implementation

## Module Mapping

| C File | C Function | Rust Module | Rust Item | Notes |
|---|---|---|---|---|
| `hard-locale.c` | `hard_locale` | `src/hard_locale.rs` | `pub(crate) fn hard_locale(category: libc::c_int) -> bool` | Direct migration of the existing function |
| `hard-locale.c` | internal locale string checks | `src/hard_locale.rs` | private helper logic only if needed | Keep helper use minimal and local |

## Data Model

This module does not define persistent C structs and does not require new Rust data structures.

| C Construct | Rust Mapping | Notes |
|---|---|---|
| locale category integer parameter | `libc::c_int` | Preserves ABI-compatible category values used by libc |
| `char *` returned by `setlocale` | borrowed C string via `*mut libc::c_char` / `*const libc::c_char` and `std::ffi::CStr` | Borrow only during the function call; do not free or store |
| boolean-style return | `bool` | Natural Rust mapping |

## Implementation Phases

### Phase 1: Create the Rust module skeleton

- Add `src/hard_locale.rs`.
- Define the Rust entry point corresponding to `hard_locale`.
- Wire the module into the crate using the existing project layout, without introducing extra abstraction layers.
- Keep the public visibility minimal, ideally `pub(crate)` unless broader use already exists in the Rust port.

### Phase 2: Port the locale query logic

- Implement the call to `libc::setlocale(category, null)` to read the active locale for the requested category.
- Handle the null-pointer case conservatively as a non-hard locale result unless the original C logic requires a different fallback.
- Convert the returned C string to a borrowed `CStr`.
- Compare against `"C"` and `"POSIX"` using byte/string comparison matching the C behavior.
- Keep all unsafe code tightly scoped around the libc call and C string handling.
- Do not cache the returned pointer or convert the logic into broader locale-management facilities.

### Phase 3: Validate edge behavior and error handling

- Add unit tests for the pure classification logic where feasible.
- Add tests that verify the `"C"` and `"POSIX"` cases are classified as non-hard locales.
- If environment-dependent testing of process locale is unstable, isolate such behavior and prefer testing the string-classification portion directly.
- Confirm that no owned memory is created from libc pointers beyond temporary borrowed inspection.
- Review unsafe blocks for pointer null checks and valid `CStr` construction assumptions.

### Phase 4: Integrate and align with crate usage

- Replace or connect any existing call sites in the Rust branch that depend on this module’s behavior.
- Ensure the function signature and module path match the surrounding port structure.
- Run `cargo test` and fix any portability issues related to locale category constants or libc imports.
- Keep the final implementation as a narrow one-function port with only minimal local helpers if they reduce unsafe surface area.