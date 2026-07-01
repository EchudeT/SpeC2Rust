# Implementation Plan

## Summary
Port the C module `hard-locale.c` into a focused Rust module that preserves the existing locale-check behavior exposed through `hard_locale`. The Rust implementation should keep the scope narrow: migrate the single function, represent its result using idiomatic Rust return types, and rely primarily on the standard library for environment and string handling.

The technical approach is to translate the locale inspection logic into a small Rust module under the main executable crate, with behavior aligned to the original C code’s locale-category check. Any interaction with process locale state that cannot be expressed through the Rust standard library alone should be handled with minimal, well-contained platform calls, avoiding broader abstraction layers or new subsystem design. Memory management will be fully owned by Rust types, and error handling should remain simple and deterministic, matching the original module’s practical semantics rather than introducing new recovery flows.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**:
  - Rust standard library
  - No third-party crates by default
  - If locale category access requires libc constants/functions on the target platform, use the minimal `libc` crate only for those bindings
- **Testing**: `cargo test`
- **Performance Goals**:
  - Constant-time or near-constant-time locale checks, equivalent to the C implementation’s overhead
  - No unnecessary heap allocation beyond what is required to read locale/environment values
  - Preserve startup-path suitability for a command-line utility

## Module Mapping

| C File | C Function | Rust Module | Rust Item |
|---|---|---|---|
| `hard-locale.c` | `hard_locale` | `src/hard_locale.rs` | `pub(crate) fn hard_locale(...) -> bool` |

### Integration Notes
- Expose the Rust function only at the visibility needed by the main crate.
- Register the module from the executable crate root using standard Rust module declarations.
- Keep the file layout minimal and aligned with the original module boundary; do not split this into extra helper modules unless required by compilation constraints.

## Data Model

This module does not define persistent C structs and does not require a complex Rust data model.

| C Representation | Rust Representation | Notes |
|---|---|---|
| locale category parameter (integer constant) | `core::ffi::c_int` or `i32` | Prefer `c_int` if interoperating with libc APIs/constants |
| locale name string (`char *` / C string) | `&CStr`, `CString`, or `String`/`OsString` as appropriate | Use borrowed `CStr` only where calling C APIs; convert into Rust-owned/string views only when needed |
| boolean result | `bool` | Direct mapping from C truth-value semantics |

### Memory Management
- Avoid manual allocation/free patterns from C.
- If C locale APIs are used, treat returned pointers as borrowed and never free them unless the API explicitly requires ownership transfer.
- Keep conversions from C strings to Rust string types localized and validated.

### Error Handling
- The public/internal Rust function should continue to produce a plain `bool`, preserving the narrow behavior of the original module.
- Internal failures such as invalid locale string decoding or null returns from locale APIs should be handled conservatively with behavior matching the original C intent rather than surfacing new error types.

## Implementation Phases

## Phase 1: Create the Rust module skeleton
- Add `src/hard_locale.rs`.
- Declare the migrated function with a signature matching the original usage pattern in the Rust crate.
- Add the module declaration in the crate root or the existing main module file.
- Identify the locale category type needed by callers and choose `c_int` if direct parity with C constants is required.

### Deliverables
- Compiling Rust module stub
- Crate integration with no behavioral expansion

## Phase 2: Port `hard_locale` logic
- Translate the original C control flow directly into Rust.
- Implement locale-category inspection using the standard library where sufficient.
- If the original logic depends on `setlocale`/locale-category constants unavailable in `std`, introduce only the minimal `libc` usage required to query the active locale.
- Preserve the original decision rules for distinguishing the default/simple locale from other locales.
- Keep all unsafe code, if needed, confined to the smallest possible block with comments describing pointer and lifetime assumptions.

### Deliverables
- Functional Rust equivalent of `hard_locale`
- Minimal and contained platform interop, only if strictly necessary

## Phase 3: Wire callers and validate behavior
- Replace existing call sites in the Rust port branch to use the new Rust module/function.
- Ensure returned boolean semantics match the original expectations in the main execution path.
- Remove or stop depending on the C implementation for this module once the Rust path is active.
- Confirm no extra abstractions or wrappers were introduced during integration.

### Deliverables
- Main-cluster integration completed
- C module usage eliminated for this function in the Rust port branch

## Phase 4: Add focused tests
- Add unit tests for the locale classification logic that can be exercised deterministically.
- Prefer tests around isolated helper logic if direct process-locale mutation is brittle in `cargo test`.
- If environment or locale-dependent tests are used, keep them narrowly scoped and avoid assuming unsupported locales are present on all systems.
- Cover at least:
  - default/C/POSIX-style locale result
  - non-default locale naming path
  - null/invalid/unexpected locale string handling path, as applicable to the implementation approach

### Deliverables
- `cargo test` coverage for migrated behavior
- Verified parity for key decision paths