# Implementation Plan: module_gnu_obstack.c_38

## Summary

This module migration covers `gnu/obstack.c`, limited to the existing exported behavior represented here by `print_and_abort`. The Rust implementation should preserve the current low-level control flow and failure semantics without introducing broader allocator abstractions or new APIs.

The technical approach is to translate the C file into a single Rust module with a narrow surface area, keeping the implementation close to the original layout and intent. Because the identified function is an abort-style path, the Rust port should model it with explicit process termination behavior rather than recovery-oriented error propagation. Memory handling should remain explicit and minimal, using Rust types only where they directly replace C storage and pointer patterns required by the existing code path.

## Technical Context

- **Language/Version**: Rust 1.75+ edition 2021
- **Primary Dependencies**:
  - Rust standard library
  - No third-party crates recommended from the available evidence
- **Testing**:
  - `cargo test`
- **Performance Goals**:
  - Preserve constant-time behavior for the abort/reporting path
  - Avoid unnecessary heap allocation beyond what is strictly needed for message formatting
  - Maintain behavior close to the original C implementation rather than optimizing beyond existing needs

## Module Mapping

| C Source File | Rust Module/File | Notes |
|---|---|---|
| `gnu/obstack.c` | `src/module_gnu_obstack.rs` | Direct migration target for the current file scope and `print_and_abort` implementation |

If the project already mirrors source-tree naming more closely, an acceptable alternative is:

| C Source File | Rust Module/File | Notes |
|---|---|---|
| `gnu/obstack.c` | `src/gnu/obstack.rs` | Prefer only if the existing Rust crate already uses a `gnu` submodule layout |

Preferred rule: choose one file and keep all migrated logic for this C file in that file unless another Rust file already exists for this exact source mapping.

## Data Model

The analysis reports only anonymous C data structures and does not identify named public structs used by the function list provided. For this module, the migration plan should therefore be conservative:

| C Data Structure | Rust Mapping | Notes |
|---|---|---|
| anonymous struct/union instances in `gnu/obstack.c` | Private `struct`, `enum`, or tuple types only if required by translated code | Introduce only when a concrete field layout is necessary for compilation |
| C pointer-based internal state | `*mut T`, `*const T`, or references where lifetime/aliasing rules are clear | Preserve raw pointers only for direct low-level compatibility |
| C integer flags or mode values | `i32`, `u32`, `usize`, or small private enums | Prefer primitive types first unless the original control flow clearly benefits from an enum |
| C string/message data | `&'static str`, `String`, or `&CStr` depending on source usage | Use the least powerful type that matches actual call sites |

### Data-structure migration rules

- Do not invent Rust domain models beyond what the C file already needs.
- Keep anonymous C layout private to the module.
- Replace nullable pointers with `Option<NonNull<T>>` only where this improves correctness without changing call patterns materially.
- For abort/reporting code paths, prefer borrowed string slices or static strings to avoid avoidable ownership complexity.
- If any translated structure depends on exact field ordering or pointer arithmetic, use `#[repr(C)]` and keep the layout minimal.

## Implementation Phases

### Phase 1: File scaffolding and signature migration

- Create the Rust target file for `gnu/obstack.c`.
- Add the module declaration in the existing crate tree without introducing unrelated modules.
- Port the `print_and_abort` function signature and identify all direct inputs, outputs, and side effects.
- Resolve whether the Rust function should return `!` (never type) or terminate via a final abort/exit path while retaining compile-time clarity.
- Add placeholder private types only if required to represent anonymous C state referenced by the function.

### Phase 2: Behavior-preserving implementation

- Translate the body of `print_and_abort` as directly as possible.
- Preserve message emission order and termination semantics.
- Map C I/O and process-abort behavior to standard library facilities such as `eprintln!`, `std::process::abort`, or `std::process::exit` based on the original code path.
- Keep memory usage explicit and limited; avoid introducing reusable formatting layers or error wrappers not present in the source.
- Where the original function depends on global or file-local state, keep that state module-private and migrate only the required pieces.

### Phase 3: Safety review and low-level cleanup

- Audit all pointer, string, and formatting conversions needed by the translated code.
- Minimize `unsafe` usage and isolate it to the smallest possible blocks.
- Verify that any raw-pointer or layout-sensitive code is documented inline with the source constraint that required it.
- Remove speculative abstractions introduced during translation so the final module reflects only the migrated C behavior.

### Phase 4: Tests and integration validation

- Add focused tests for observable behavior that can be exercised under `cargo test`.
- For non-returning abort logic, prefer validation of formatting/helpers separated from the final termination call, or use test-only seams limited to this module.
- Confirm the module builds cleanly on the target branch and that integration points match the existing crate API expectations.
- Ensure the final implementation does not expose extra public API beyond what is needed for the migrated function.

## Notes on Memory Management and Error Handling

- Memory ownership should remain local and simple; avoid heap-backed state unless formatting or existing interfaces require it.
- Since `print_and_abort` represents a terminal path, use explicit termination rather than converting the behavior into recoverable `Result` flows.
- Any temporary strings should have the narrowest lifetime possible.
- If the C implementation writes directly to standard error and aborts, the Rust implementation should do the same with no additional buffering or retry logic.

## Deliverables

- One Rust module replacing the implementation role of `gnu/obstack.c`
- Migrated `print_and_abort` behavior
- Minimal private type definitions required for compilation
- Unit tests limited to the migrated behavior and any extracted formatting helpers