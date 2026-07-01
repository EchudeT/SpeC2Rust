# Implementation Plan: main_root_setlocale_null_05

## Summary

Port the C locale-query helpers in `setlocale_null.c` and `setlocale_null-unlocked.c` into a Rust module that preserves the existing function boundaries and behavior around retrieving locale strings when the locale argument is null-equivalent. The Rust implementation should stay narrowly aligned with the original files and exported functions, using standard-library string ownership and explicit result handling in place of C buffer and pointer management.

The implementation approach is:

- migrate the logic from the two C files into a single Rust module or closely paired Rust source files under the existing crate layout for the main cluster;
- preserve the distinction between unlocked and lock-using call paths at the API level only if the original module exposes both paths internally, while avoiding adding new synchronization facilities beyond what is required by the migrated logic;
- replace C string/pointer handling with `String`, `&str`, and `Option`/`Result` as appropriate;
- model failure paths explicitly rather than relying on null pointers or buffer-size side effects;
- keep the scope limited to the functions listed for this module, without introducing extra locale abstractions.

## Technical Context

- **Language/Version**: Rust 1.76+
- **Primary Dependencies**:
  - Rust standard library
  - No third-party crates recommended from the provided evidence
- **Testing**:
  - `cargo test`
  - unit tests focused on function-level behavior and error cases derived from the current C control flow
- **Performance Goals**:
  - preserve the current lightweight behavior of locale string retrieval;
  - avoid unnecessary heap allocations where a borrowed string or direct conversion is possible;
  - keep call-path overhead close to the C implementation, especially for the unlocked helper flow;
  - do not introduce broader synchronization or abstraction layers beyond the original module structure.

## Module Mapping

### C to Rust file mapping

- `setlocale_null.c`
  - migrate into Rust as the primary module file for public-facing wrappers and shared logic
  - recommended target: `src/main_root_setlocale_null_05.rs` or the equivalent crate-local module file following the current project naming pattern

- `setlocale_null-unlocked.c`
  - migrate into the same Rust module as private helper functions, or into a sibling internal module only if needed to mirror file separation without expanding functionality
  - keep unlocked helper logic close to the public wrappers to minimize divergence

### Function mapping

- `setlocale_null_unlocked`
  - Rust private helper or crate-visible function implementing the unlocked retrieval path

- `setlocale_null_r_unlocked`
  - Rust private helper handling the reentrant/buffer-oriented unlocked path
  - map C buffer mutation semantics into either:
    - `fn ... -> Result<String, ErrorLike>` if the rest of the Rust port uses owned strings, or
    - `fn ...(out: &mut String) -> Result<(), ErrorLike>` if preserving caller-provided destination style is more consistent with adjacent migrated modules

- `setlocale_null_r_with_lock`
  - Rust internal helper preserving the locked wrapper behavior from C
  - if duplicated in the analysis list, treat this as a single implementation target with one canonical Rust function

- `setlocale_null_r`
  - Rust wrapper corresponding to the public/internal reentrant entry point
  - delegates to the lock-aware or unlocked helper according to the original control flow

- `setlocale_null`
  - Rust wrapper corresponding to the simpler top-level entry point
  - delegates to the reentrant or unlocked helper as in the C implementation

### Visibility plan

- Expose only the Rust functions that correspond to externally used module entry points in the current crate.
- Keep helper functions private unless another already-migrated Rust module requires crate-level visibility.
- Do not create new facade APIs.

## Data Model

No explicit C structs were identified in the input for this module.

### C to Rust type mapping

Because this module is string- and pointer-oriented rather than struct-oriented, the main migration is at the scalar/type level:

- `char *` / `const char *`
  - map to `String` for owned returned locale names
  - map to `&str` for borrowed input where lifetime is naturally bounded
  - map nullability to `Option<&str>` or `Option<String>` depending on ownership direction

- output buffer parameters such as `char *buf, size_t bufsize`
  - prefer `&mut String` if the surrounding Rust port is using owned text output
  - if exact capacity-sensitive behavior must be preserved for compatibility with adjacent migrated code, use `&mut [u8]` plus UTF-8 validation only at the boundary where a string is required

- `size_t`
  - map to `usize`

- null return / error sentinel
  - map to `Option<T>` for simple absence
  - map to `Result<T, E>` when the C logic distinguishes failure causes that matter to callers

### Error representation

Use a small module-local error enum only if the C code has more than one observable failure mode. Otherwise use a minimal `Option`-based return style. Avoid introducing a shared project-wide error type unless one already exists in the current Rust port.

### Memory management notes

- Eliminate manual buffer lifetime concerns by using Rust-owned strings where possible.
- If a temporary C-style buffer behavior must be mirrored, keep mutation confined to the function scope and return validated Rust values.
- Do not use unsafe code unless interaction with already-ported low-level locale access requires it; if unavoidable, isolate it to the smallest helper boundary and document the invariants.

## Implementation Phases

### Phase 1: Translate unlocked core logic

- Create the Rust module file for `main_root_setlocale_null_05`.
- Port `setlocale_null_unlocked` and `setlocale_null_r_unlocked` first, since they contain the core retrieval behavior.
- Replace null checks and raw pointer flow with `Option` and `Result`.
- Decide the exact Rust return style based on neighboring migrated modules:
  - prefer owned `String` return values;
  - use mutable output parameters only if needed to preserve existing call sites.
- Add focused unit tests for:
  - null-equivalent locale query handling;
  - successful locale string extraction;
  - failure/empty-result paths.

### Phase 2: Port lock-aware wrappers and top-level entry points

- Implement `setlocale_null_r_with_lock` as the Rust equivalent of the C lock-aware helper.
- Port `setlocale_null_r` and `setlocale_null` as thin wrappers over the unlocked/core logic.
- Preserve the original call hierarchy and avoid collapsing distinct entry points if they are part of the current crate’s expected interface.
- Resolve the duplicate function listing by keeping one canonical implementation of `setlocale_null_r_with_lock` and one corresponding test set.

### Phase 3: Align behavior and integrate into the crate

- Wire the new module into the existing Rust crate structure for the main cluster.
- Update any internal call sites from the C-port area to use the migrated Rust functions.
- Verify edge behavior matches the original module expectations:
  - empty or missing locale category result handling;
  - output sizing or truncation semantics if a buffer-oriented API was retained;
  - consistent propagation of failure conditions.
- Run `cargo test` and fix any mismatches introduced by pointer-to-string conversion assumptions.

### Phase 4: Final cleanup and parity review

- Remove any migration-only scaffolding that is no longer needed.
- Ensure helper visibility is restricted to the minimum required scope.
- Review for unnecessary allocations and simplify where direct borrowing is sufficient.
- Confirm the final Rust module remains limited to the original file/function responsibilities and does not introduce extra locale-management capabilities.