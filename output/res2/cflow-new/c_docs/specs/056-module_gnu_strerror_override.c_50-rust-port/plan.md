# Implementation Plan: module_gnu_strerror-override.c_50

## Summary

This module migrates `gnu/strerror-override.c` into a small Rust module that preserves the existing override lookup behavior of `strerror_override` without adding new facilities. The Rust implementation should provide a direct, table-driven or match-based mapping from selected error numbers to override messages and return the absence of an override when no mapping applies.

The implementation approach should stay close to the C source shape:

- port the single function `strerror_override` into a single Rust module;
- represent returned messages as borrowed static string data;
- avoid heap allocation and avoid introducing broader error abstraction layers;
- keep the logic deterministic and lightweight, matching the original module’s intent.

Because this module is a narrow compatibility layer, the Rust version should emphasize faithful behavior, explicit integer-to-message mapping, and simple ownership semantics through `Option<&'static str>` or an equivalent borrowed return type.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - constant-time or near-constant-time override lookup;
  - zero heap allocations on the lookup path;
  - static string storage only;
  - behavior equivalent to the C implementation for supported error codes.

## Module Mapping

| C Source File | Rust Target | Notes |
|---|---|---|
| `gnu/strerror-override.c` | `src/gnu/strerror_override.rs` | Direct migration of the single-function module. |
| `strerror_override` | `pub(crate) fn strerror_override(errnum: i32) -> Option<&'static str>` | Rust return type should express “override present or not” explicitly. |

If the project already exposes a `gnu` module namespace, this file should be added there with only the minimum `mod` declaration needed to compile. No extra module layering should be introduced.

## Data Model

This C module does not define custom structs, so the data mapping is minimal.

| C Element | Rust Mapping | Notes |
|---|---|---|
| error number argument (`int`) | `i32` | Preserve signed integer input unless surrounding project conventions require a narrower libc-compatible alias. |
| returned message pointer (`const char *` or nullable equivalent) | `Option<&'static str>` | `Some(message)` for an override, `None` for no override. |
| static message literals | `&'static str` | Stored as compile-time string constants. |

### Memory Management

- C string literals and returned pointers become Rust string slices with `'static` lifetime.
- No owned `String` values should be created.
- No unsafe memory handling should be required for the core port if the function remains purely internal and Rust-native.

### Error Handling

- The function should not raise Rust errors for unknown error numbers.
- Missing overrides should be represented as `None`, matching the original nullable/no-match behavior.
- Any integration layer that needs a different surface API should adapt outside this module, not inside it.

## Implementation Phases

### Phase 1: Inspect and Fix the Rust Surface

- Review `gnu/strerror-override.c` and enumerate all explicit error-code-to-message mappings.
- Identify the exact C return convention:
  - whether it returns `NULL` when no override applies;
  - whether messages are always static literals.
- Create `src/gnu/strerror_override.rs`.
- Define the Rust function signature as close as practical to the C behavior, preferably:
  - `pub(crate) fn strerror_override(errnum: i32) -> Option<&'static str>`

**Deliverable**: Compiling Rust module skeleton with the final function signature and placeholder mapping structure.

### Phase 2: Port the Override Logic

- Translate the C branching or switch logic directly into Rust.
- Use a restrained representation:
  - `match errnum { ... }` for a fixed set of known values;
  - return `Some("...")` for overridden messages;
  - return `None` for all other cases.
- Keep message text byte-for-byte aligned with the C literals where applicable.
- Avoid normalization, localization changes, or message rewriting beyond what is required by Rust string literal syntax.

**Deliverable**: Functional Rust implementation of `strerror_override` with all original mappings preserved.

### Phase 3: Integrate Module Placement

- Add the minimum module declarations needed for the new file under the existing crate structure.
- Update any internal call sites that previously relied on C-style nullable string returns to use the Rust `Option` form.
- Keep the adaptation narrow:
  - map `Some(msg)` to the existing caller behavior;
  - leave non-module logic unchanged.

**Deliverable**: Integrated build with the Rust module wired into the project’s existing layout.

### Phase 4: Validate Behavior with Tests

- Add focused unit tests covering:
  - each known overridden error number;
  - at least one unknown error number returning `None`;
  - boundary-style cases if the C implementation includes unusual integer values.
- Confirm returned strings exactly match expected override text.
- Run `cargo test` and resolve any signature or module visibility issues.

**Deliverable**: Passing tests that demonstrate parity for the migrated function.

## Notes and Constraints

- Keep the migration limited to `gnu/strerror-override.c` and `strerror_override`.
- Do not introduce extra helper subsystems, generalized error registries, or portability wrappers unless required by existing code structure.
- Prefer a direct translation over abstraction so the Rust module remains easy to verify against the C original.
- If platform-specific error constants are involved in the C source, keep the Rust handling as simple as possible and tied only to the constants actually used by this module.