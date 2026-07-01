# Implementation Plan

## Summary
Port `hard-locale.c` into a focused Rust module that preserves the existing locale-detection behavior needed by `cat` without adding new abstractions or features. The Rust implementation should translate the single exported function, `hard_locale`, into a small module-level function that determines whether the active locale for a category is effectively non-trivial.

The preferred approach is to keep the implementation narrow and close to the C control flow:
- read the current locale value for the requested category,
- interpret null/empty/C/POSIX cases conservatively,
- return a boolean result matching the original intent.

Because the source module is small and function-oriented, the Rust port should remain similarly small: one source file, one public function, and targeted unit tests around locale-string interpretation. Memory management should rely on Rust ownership and borrowed string handling, with explicit handling for any OS interaction needed to inspect locale state.

## Technical Context

### Language/Version
- Rust stable, edition 2021
- Minimum recommended toolchain: Rust 1.75+

### Primary Dependencies
- Rust standard library
- No third-party crates by default

If direct locale-category inspection cannot be expressed with the standard library alone, use minimal libc bindings already present in the project if available; otherwise add:
- `libc` for `setlocale`/locale category constants

This module should not introduce any other dependency.

### Testing
- `cargo test`

Testing should cover:
- classification logic for locale names equivalent to `"C"` and `"POSIX"`,
- handling of empty or unavailable locale strings,
- expected `true` result for representative non-default locale names,
- integration expectations at the Rust module boundary.

### Performance Goals
- Constant-time decision logic after locale retrieval
- No unnecessary allocation in the common path beyond what is required to read the locale string safely
- Behavior should be operationally equivalent to the C version; this module is not performance-critical and should not introduce repeated parsing or caching not present in the original

## Module Mapping

### C to Rust File Mapping
- `hard-locale.c` -> `src/hard_locale.rs`

### Function Mapping
- `hard_locale` -> `pub(crate) fn hard_locale(category: i32) -> bool`
  or, if the surrounding port uses libc types explicitly:
  `pub(crate) fn hard_locale(category: libc::c_int) -> bool`

The exact signature should follow the call sites in the Rust port of the main cluster so category constants can be passed through without adaptation layers.

### Integration Mapping
- Expose the Rust function within the main executable crate only as needed by migrated callers
- Keep call-site migration local to the existing main-cluster port; do not introduce a new utility hierarchy beyond the direct module file

## Data Model

This module has no dedicated C structs to migrate.

### Scalar/Type Mapping
- C locale category parameter (`int`) -> Rust `i32` or `libc::c_int`
- C string result from locale lookup (`char *`) -> Rust borrowed view derived from `CStr`
- C boolean-style return -> Rust `bool`

### Memory and Error Handling Model
- Any locale string obtained from C APIs must be treated as borrowed, not owned
- Convert C strings with `std::ffi::CStr` and avoid retaining pointers after subsequent locale calls
- If locale lookup returns null, invalid UTF-8, or an unusable empty string, handle conservatively according to the original function’s behavior rather than panicking
- Unsafe code, if required for `libc::setlocale`, should be isolated inside this module and kept minimal

## Implementation Phases

### Phase 1: Establish Module Skeleton and Signature
- Create `src/hard_locale.rs`
- Define the Rust equivalent of `hard_locale` with a signature compatible with migrated callers
- Add the module declaration in the crate root or relevant `main_cluster` module tree
- Decide the exact category type (`i32` vs `libc::c_int`) based on existing migrated interfaces, preferring the narrowest change to callers

### Phase 2: Port Locale Detection Logic
- Translate the C function logic directly into Rust
- Implement locale retrieval using the standard library plus minimal libc interaction only if necessary
- Convert returned locale data using `CStr`
- Preserve the original classification rules for default/simple locales versus hard locales
- Keep unsafe operations tightly scoped and documented at the expression level

### Phase 3: Migrate Call Sites and Validate Behavior
- Update any users of `hard_locale` in the main cluster to call the Rust function
- Remove reliance on the original C module for this functionality within the Rust branch
- Confirm there is no lifetime leakage of locale pointers or accidental ownership assumptions
- Ensure return-value semantics match the expectations of the existing callers

### Phase 4: Add Focused Tests and Final Cleanup
- Add unit tests for the string-classification portion of the logic
- Where practical, separate pure classification from OS-bound locale retrieval so tests remain deterministic
- Run `cargo test`
- Perform a final pass to keep the module minimal, with no extra helpers beyond what is needed for the port