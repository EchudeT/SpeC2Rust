# Implementation Plan

## Summary

This module ports the locale-query helpers from `setlocale_null.c` and `setlocale_null-unlocked.c` into Rust for the `cat` project’s main cluster. The Rust implementation should preserve the existing function boundaries and behavior focus: obtaining locale strings, handling null-like/absent results safely, and keeping the distinction between locked and unlocked internal paths only as far as needed to mirror the current C organization.

The implementation approach should be conservative:

- translate the existing C entry points into a small Rust module with matching internal helper structure;
- represent locale results using owned Rust strings where possible to avoid borrowed-pointer lifetime hazards from C-style APIs;
- keep error and absence handling explicit with `Option`/`Result` rather than raw null pointers;
- avoid introducing new abstraction layers or broader locale facilities beyond what these files already cover.

Because this module belongs to the main executable cluster, the Rust port should integrate into the existing binary crate layout using standard Rust modules and tests, without adding extra subsystems.

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**:
  - Rust standard library
  - No third-party crates required by the provided analysis
- **Testing**:
  - `cargo test`
  - unit tests for helper behavior and edge cases around empty/missing locale values
- **Performance Goals**:
  - maintain negligible overhead relative to the C helpers for normal startup/runtime use;
  - avoid unnecessary repeated allocations within a single call path;
  - keep locale string retrieval O(n) in the returned string length, with no extra copying beyond what is required for safe ownership.

## Module Mapping

### Source File Mapping

- `setlocale_null.c`
  → `src/main_root_setlocale_null_04.rs` or equivalent module file under the existing main-cluster source tree

- `setlocale_null-unlocked.c`
  → merged into the same Rust module as private helper functions unless the current Rust project layout already splits files similarly

### Function Mapping

The Rust port should preserve the existing migration surface by mapping each C function to a Rust function with the same conceptual role.

- `setlocale_null_unlocked`
  - Rust: `fn setlocale_null_unlocked(...) -> Option<String>` or `Result<Option<String>, _>` depending on current project error style
  - Scope: private unless used externally

- `setlocale_null_r_unlocked`
  - Rust: `fn setlocale_null_r_unlocked(...) -> Option<String>` / `Result<String, _>`
  - Scope: private helper

- `setlocale_null_r_with_lock`
  - Rust: `fn setlocale_null_r_with_lock(...) -> Option<String>` / `Result<String, _>`
  - Note: duplicate appearances in the input should be treated as one implementation target

- `setlocale_null_r`
  - Rust: `fn setlocale_null_r(...) -> Option<String>` / `Result<String, _>`
  - Scope: module-visible or public according to existing call sites in the Rust branch

- `setlocale_null`
  - Rust: `fn setlocale_null(...) -> Option<String>`
  - Scope: public within the crate if this is the module entry point used by `cat`

### Behavioral Mapping Notes

- C null pointer return cases should become `None` or an error variant, depending on whether the original behavior indicates “no locale available” versus an operational failure.
- C buffer-writing helper variants with `_r` suffix should become Rust functions that return owned data instead of mutating caller-managed raw buffers unless an existing Rust caller interface already requires output buffers.
- Any distinction between “with lock” and “unlocked” should remain internal and minimal; do not introduce new synchronization constructs unless directly required by the migrated logic.

## Data Model

No explicit C structs are listed for this module. The primary migration concern is converting C string and pointer conventions into safe Rust types.

### Type Mapping

- `char *` / `const char *`
  - Rust: `String` for owned returned locale values
  - Rust: `&str` only for internal read-only processing after conversion

- nullable C string pointer result
  - Rust: `Option<String>`

- integer category parameters typically used by `setlocale`
  - Rust: `libc`-compatible integer type only if already present in the project interface
  - otherwise Rust: `i32` matching the original C call surface

- caller-provided output buffer patterns from `_r` functions
  - Rust: prefer direct return of `String`
  - if exact buffer semantics are already required by neighboring migrated code, use `&mut String` or `&mut Vec<u8>` only where necessary

### Ownership and Memory Notes

- Never expose borrowed references tied to temporary locale query storage.
- Convert any external or C-derived locale data into owned Rust strings before returning.
- Validate string conversion carefully if underlying bytes may not be valid UTF-8; if exact C byte preservation is required by surrounding code, use `OsString` or `Vec<u8>` internally and convert at the API edge only if current callers expect UTF-8 strings. If no such evidence exists, start with `String` plus fallible conversion handling.

## Implementation Phases

## Phase 1: Establish module skeleton and migrate core helper flow

- Create the Rust module file corresponding to `setlocale_null.c` and `setlocale_null-unlocked.c`.
- Add Rust function stubs for:
  - `setlocale_null`
  - `setlocale_null_r`
  - `setlocale_null_unlocked`
  - `setlocale_null_r_unlocked`
  - `setlocale_null_r_with_lock`
- Resolve duplicate function names in the analysis as a single implementation target per unique function.
- Choose the return model (`Option<String>` or `Result<..., ...>`) based on existing Rust branch conventions, keeping null/absence distinct from hard failure if the C logic does so.
- Keep functions narrowly scoped to match current C call layering rather than redesigning the API.

## Phase 2: Port string retrieval logic and null-handling semantics

- Translate the actual locale retrieval logic from the C implementation into Rust.
- Replace raw pointer checks with explicit `Option` handling.
- Replace C buffer manipulation with owned Rust string construction.
- Preserve the separation between unlocked and locked paths only when it reflects real logic in the source; if the Rust environment makes one path a trivial wrapper around another, keep that simplification local and unobtrusive.
- Ensure returned values do not depend on temporary storage or mutable global state lifetimes.

## Phase 3: Integrate with callers and normalize error behavior

- Update the main-cluster call sites to use the Rust module in place of the C implementation.
- Align parameter and return types with adjacent migrated code without widening scope beyond this module.
- Verify that all former null-return cases, empty-string cases, and category handling paths compile cleanly and remain behaviorally consistent.
- Remove or gate any now-unused transitional code introduced during migration.

## Phase 4: Add focused tests and finalize migration

- Add unit tests for:
  - successful locale string retrieval path;
  - null/absent locale result mapping;
  - empty or invalid string edge handling if present in the C logic;
  - wrapper consistency between `_r`, locked, and unlocked variants.
- Run `cargo test` and fix mismatches in ownership, string conversion, and return semantics.
- Confirm the final Rust module fully replaces the functionality of:
  - `setlocale_null.c`
  - `setlocale_null-unlocked.c`