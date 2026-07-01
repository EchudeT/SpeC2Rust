# Implementation Plan: main_root_mbrtoc32_10

## Summary

Port the C source file `mbrtoc32.c` into an idiomatic Rust module that preserves the existing conversion behavior and stateful decoding semantics of `mbrtoc32`. The Rust implementation should focus on a direct migration of the current logic rather than introducing broader Unicode conversion facilities.

The technical approach is to implement the function in a single Rust module with explicit handling for:
- input byte sequence consumption,
- conversion into a 32-bit character value,
- restartable conversion state equivalent to C multibyte state handling,
- return-value behavior that mirrors the C implementation as closely as practical within Rust.

Where the original C code relies on pointer-based input/output and `mbstate_t` mutation, the Rust port should use borrowed slices, mutable references, and a small internal state type if required by the source logic. Error and incomplete-sequence conditions should be represented explicitly without adding unrelated abstractions.

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Match the C implementation’s asymptotic behavior for single-pass multibyte decoding.
  - Avoid heap allocation in the conversion path.
  - Keep state handling lightweight and copy-free where possible.
  - Preserve efficient processing for ASCII or single-unit fast paths if present in the original logic.

## Module Mapping

| C File | Rust Module/File | Notes |
|---|---|---|
| `mbrtoc32.c` | `src/main_root_mbrtoc32_10.rs` or `src/main_root_mbrtoc32_10/mod.rs` | Direct migration target for the `mbrtoc32` logic. |
| `mbrtoc32` | `pub fn mbrtoc32(...)` | Rust function mirroring the C routine’s conversion and state update behavior. |

If the crate already organizes main-cluster ports under a shared layout, place this file in the existing location without introducing new top-level subsystems.

## Data Model

No standalone C structs were identified in the analysis input. The only expected data-model concern is the conversion state used by `mbrtoc32`.

| C Concept | Rust Mapping | Notes |
|---|---|---|
| `char32_t` output | `u32` or `core::primitive::char`-adjacent storage | Prefer `u32` to preserve C-level value behavior, especially for direct migration. |
| `mbstate_t` | Local Rust state struct or existing crate-internal state type | Use only the fields needed by the original logic. Avoid adding extra state machinery. |
| input `const char *` + length | `&[u8]` | Supports bounded decoding without pointer arithmetic. |
| output pointer | `&mut u32` or optional mutable output reference | Keep mutation explicit and close to original semantics. |
| C status/return codes | `usize` plus explicit error/result enum if needed internally | Public behavior should remain aligned with C semantics; internal enums are acceptable if they simplify branching. |

### Memory Management

The Rust port should rely entirely on stack-based state and borrowed input. No heap allocation is necessary for this module. Any persistent conversion state should be passed in by mutable reference, replacing C’s raw mutable state object.

### Error Handling

Because `mbrtoc32` has C-specific return conventions for success, incomplete input, and invalid sequences, the Rust implementation should preserve those externally visible outcomes rather than replacing them with a broad `Result` API. Internally, a small enum for:
- complete character,
- incomplete sequence,
- invalid sequence

may be used to keep the implementation readable.

## Implementation Phases

## Phase 1: Establish Rust Module Skeleton

- Create the Rust destination file for the ported module.
- Define the public `mbrtoc32` function signature consistent with the surrounding Rust porting conventions used by this project.
- Identify the exact state elements needed from the C implementation and represent them in the minimal Rust form.
- Set up unit-test scaffolding in the same module or the crate’s existing test layout.

**Deliverable**: Compiling module skeleton with placeholders for conversion-state handling and return semantics.

## Phase 2: Port Core Decoding Logic

- Translate the byte-consumption and character-construction logic from `mbrtoc32.c` into Rust.
- Replace pointer arithmetic with slice indexing and explicit bounds checks.
- Port restartable state transitions faithfully, including partial multibyte sequence handling.
- Preserve C-compatible return behavior for:
  - completed conversion,
  - null-character handling if applicable in the source,
  - incomplete input,
  - invalid multibyte sequence.

**Deliverable**: Functional Rust implementation of `mbrtoc32` with direct behavioral parity target.

## Phase 3: Validate Edge Cases and State Semantics

- Add tests covering:
  - empty input,
  - ASCII/simple single-byte input,
  - valid multibyte input,
  - incomplete sequences across calls,
  - invalid byte sequences,
  - reset/default-state behavior.
- Verify that output mutation occurs only on successful conversion paths as required by the original logic.
- Confirm that no unnecessary allocations or copies occur in normal execution.

**Deliverable**: Passing correctness tests for normal, partial, and error paths.

## Phase 4: Final Conformance Cleanup

- Compare the Rust control flow and return values against the C source to remove any behavioral drift introduced during translation.
- Simplify any temporary helper abstractions that exceed the needs of the original module.
- Ensure naming, visibility, and file placement match the project’s existing Rust port conventions.

**Deliverable**: Finalized minimal Rust port ready on branch `010-main_root_mbrtoc32_10-rust-port`.