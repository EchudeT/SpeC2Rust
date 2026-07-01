# Implementation Plan

## Summary

Port `localcharset.c` into a Rust module that preserves the current role of `locale_charset` as the module’s single externally relevant function. The Rust implementation should focus on reproducing the existing charset-detection behavior using standard-library facilities where possible, while keeping the result shape and fallback behavior aligned with the C source.

The implementation approach should remain narrow:

- migrate the logic from `localcharset.c` into one Rust source file,
- translate static tables and helper data into Rust constants and private helper types,
- return Rust string types without exposing raw pointers,
- keep platform-specific branching only where required by the existing C logic,
- preserve deterministic fallback handling for unknown or unavailable locale information.

Because this module appears to be centered on locale-derived charset selection, the Rust port should avoid introducing broader locale abstractions and instead implement only the logic needed to support `locale_charset`.

## Technical Context

- **Language/Version**: Rust 1.78 or newer
- **Primary Dependencies**:
  - Rust standard library
  - No third-party crates by default, since the input does not require external parsing or OS bindings beyond what can be represented with conditional compilation and standard process/environment access
- **Testing**:
  - `cargo test`
- **Performance Goals**:
  - Constant-time or near-constant-time lookup for static charset alias data
  - No unnecessary heap allocation beyond constructing the returned Rust string result
  - Comparable practical runtime to the C implementation for single-call locale charset detection
  - Minimal overhead for fallback paths and alias normalization

## Module Mapping

### C to Rust File Mapping

- `localcharset.c` → `src/main_root_localcharset.rs`

If the crate already exposes a central `main_cluster` module tree, this file should be registered there without creating extra layers beyond what is needed to match the existing project layout.

### Function Mapping

- `locale_charset` → `pub(crate) fn locale_charset(...) -> ...` or `pub fn locale_charset(...) -> ...`

The exact visibility should match current crate usage. If the function is only consumed internally, prefer `pub(crate)`.

### Internal Logic Mapping

Any C-internal helpers embedded in `localcharset.c` should become:

- private Rust helper functions,
- private constant tables,
- small private enums for platform/lookup distinctions where this improves direct translation of branches.

No additional public API should be introduced.

## Data Model

The analysis lists only anonymous C data structures, which strongly suggests internal static tables or local helper records rather than externally shared types. These should be translated into Rust private types only as needed.

### Data-Structure Mapping

- `anonymous` static table records → private Rust `struct` with named fields
- `anonymous` lookup groupings / paired string entries → private Rust `struct` or tuple constants
- `anonymous` platform-specific records → private Rust `enum` only if the C code uses tagged branching semantics
- repeated `anonymous` table nodes → `const` slices or arrays of private structs

### Recommended Rust Representations

Where the C code uses static alias mappings such as locale-name to charset-name pairs:

```rust
struct CharsetAlias {
    locale_key: &'static str,
    charset: &'static str,
}
```

Where the C code uses grouped platform rules:

```rust
enum LocaleSource {
    Environment,
    PlatformSpecific,
    Fallback,
}
```

Only introduce enums if they map directly to existing branch categories in the C file. Otherwise prefer helper functions and constant arrays.

### Memory Management

- Replace C string pointers and static storage assumptions with `&'static str` for compile-time constants.
- For computed results, prefer returning `String` if normalization or owned output is necessary.
- If the effective behavior is “return pointer to static string or discovered environment string,” then a Rust `Cow<'static, str>` can be considered internally, but only if it simplifies preserving borrowed constants without complicating the public API.
- Avoid unsafe code unless the original implementation depends on OS interfaces that cannot be expressed safely; if unsafe is unavoidable, isolate it in a tiny helper.

### Error Handling

The C function likely relies on fallback rather than explicit error propagation. Mirror that in Rust by:

- using `Option`/`Result` internally for parsing and lookup steps,
- collapsing failures into the same final fallback charset used by the C implementation,
- avoiding a new public error type unless the original function semantics already expose failure separately.

## Implementation Phases

## Phase 1: Establish Module Skeleton and Translate Static Data

- Create `src/main_root_localcharset.rs`.
- Add the Rust function signature for `locale_charset`.
- Extract and translate all static C tables into Rust `const` arrays/slices.
- Replace anonymous C records with minimal private Rust structs with explicit field names.
- Register the module in the existing crate module tree.
- Confirm the code compiles with placeholder logic wired through the translated constants.

### Deliverables

- New Rust source file corresponding to `localcharset.c`
- Private Rust data structures replacing anonymous C table definitions
- Compiling module skeleton with no expanded API surface

## Phase 2: Port Detection and Lookup Logic

- Migrate the core control flow of `locale_charset` from C into Rust.
- Translate locale/environment inspection logic directly, preserving branch order and fallback precedence.
- Port alias matching and normalization logic using standard-library string handling.
- Replace pointer-based string comparisons with Rust `&str` comparisons.
- Keep platform-specific code behind `cfg` gates only where the C source differentiates by target OS or libc behavior.
- Ensure all internal failure paths resolve to the same fallback outcome as the C implementation.

### Deliverables

- Functional Rust implementation of `locale_charset`
- Private helpers for normalization, alias lookup, and fallback selection
- Conditional compilation blocks only where required by the original file

## Phase 3: Align Return Semantics and Remove C-Style Assumptions

- Finalize the Rust return type based on actual call-site needs in the ported project.
- Eliminate any residual C assumptions about static mutable buffers or raw pointer lifetimes.
- Review whether returned data can be borrowed from constants or must be owned.
- Minimize allocation while keeping ownership and lifetime rules explicit and safe.
- Verify there is no reliance on null-terminated string behavior outside narrowly contained helper logic.

### Deliverables

- Final return type chosen and applied consistently
- Safe ownership/lifetime model for detected charset values
- Cleaned implementation with no unnecessary unsafe patterns

## Phase 4: Testing and Behavioral Verification

- Add unit tests for alias lookup and normalization using cases derived from the migrated C tables and branches.
- Add tests for environment-driven locale detection where behavior can be validated through controlled inputs.
- Add fallback tests for empty, malformed, or unsupported locale values.
- Verify platform-gated code compiles appropriately on supported targets, using conditional tests where necessary.
- Run `cargo test` and fix any behavioral mismatches revealed during port validation.

### Deliverables

- `cargo test` coverage for core lookup and fallback behavior
- Regression checks for representative locale-to-charset mappings
- Finalized Rust module ready for integration on branch `030-main_root_localcharset.c_29-rust-port`