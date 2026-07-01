# Implementation Plan

## Summary
This module ports the C file `hard-locale.c` into a small Rust module that preserves the existing locale-detection behavior centered on the `hard_locale` function. The Rust implementation should remain narrowly scoped: translate the current logic for determining whether the active locale is a non-trivial locale, keep behavior aligned with the original C code, and integrate it into the existing main-cluster crate layout without introducing new capabilities.

The technical approach should prefer the Rust standard library for string handling, environment access, and control flow. If the C implementation depends on locale state that is not directly exposed by the standard library, the Rust port should isolate that access behind a minimal internal function and keep unsafe or platform-specific code constrained to the smallest possible surface. The result should be a direct migration of the existing file and function, with explicit handling for null-like cases, invalid strings, and locale-name comparisons.

## Technical Context

- **Language/Version**: Rust 1.76+
- **Primary Dependencies**:
  - Rust standard library
  - No third-party crates by default
- **Testing**:
  - `cargo test`
- **Performance Goals**:
  - Match the C module’s constant-time/constant-space style for locale classification
  - Avoid unnecessary allocations where simple string comparison is sufficient
  - Keep startup-path overhead negligible since this is a small utility function likely used in command execution flow

## Module Mapping

| C Source File | C Function | Rust Module/File | Rust Item |
|---|---|---|---|
| `hard-locale.c` | `hard_locale` | `src/hard_locale.rs` | `pub(crate) fn hard_locale(...) -> bool` |

### Notes
- Keep the Rust module focused on the single migrated function from `hard-locale.c`.
- If the current crate uses a `main.rs` plus internal modules layout, register this as `mod hard_locale;` from the existing entry-point or nearest equivalent existing module root.
- Do not split this into extra helper modules unless a tiny private helper is needed to isolate platform-specific locale retrieval.

## Data Model

This module appears function-oriented and does not define dedicated C structs.

| C Data | Rust Mapping |
|---|---|
| Locale category argument (likely integer macro/category selector) | Function parameter using the closest available Rust representation for the call site, preferably a narrow integer or dedicated internal enum if categories are known at migration time |
| `char *` / locale string returned from locale APIs | `Option<String>` or borrowed string view where possible; use `Option` to represent null-like results |
| Boolean-style return (`true`/`false` semantics) | `bool` |

### Memory Management
- Replace raw C string handling with safe Rust string handling wherever possible.
- If platform locale access requires C interop internally, convert returned pointers immediately into checked Rust string data and avoid exposing raw pointers outside a tiny private boundary.
- No heap ownership model beyond ordinary Rust values should be introduced.

### Error Handling
- Preserve the original function contract by returning a plain `bool`.
- Internal retrieval/parsing failures should resolve to the conservative behavior that matches the C implementation’s intent.
- Avoid introducing `Result` in the public migrated function unless the surrounding Rust call sites already require it.

## Implementation Phases

### Phase 1: Create the Rust module skeleton and map the public interface
- Add `src/hard_locale.rs`.
- Define the Rust version of `hard_locale` with a signature matching the existing Rust call expectations as closely as possible.
- Wire the module into the current crate structure without adding unrelated modules.
- Add a minimal doc comment describing that the function is a direct port of locale hardness detection.

### Phase 2: Port the locale classification logic directly from C
- Translate the C control flow and string comparisons into Rust.
- Preserve the original checks for default/POSIX/C locale names and any category-based branching present in the source.
- Keep locale retrieval localized to this module.
- If direct standard-library support is insufficient, use a minimal internal platform-specific path only for obtaining the active locale string, while keeping the comparison/classification logic in safe Rust.

### Phase 3: Validate edge cases and behavior parity
- Add unit tests covering the known classification outcomes for:
  - `"C"`
  - `"POSIX"`
  - representative non-default locale names
  - missing or invalid locale values if reachable in the Rust port path
- Confirm that null-equivalent or undecodable locale cases do not panic.
- Verify behavior against the original C logic rather than redesigning semantics.

### Phase 4: Integrate and clean up migration details
- Replace or connect any existing call sites to use the Rust `hard_locale` function.
- Remove any temporary migration scaffolding used during translation.
- Ensure the final module uses idiomatic ownership and borrowing while remaining a direct behavioral port.
- Run `cargo test` and fix any parity issues discovered during integration.