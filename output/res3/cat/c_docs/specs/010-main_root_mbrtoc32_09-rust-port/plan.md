# Implementation Plan: main_root_mbrtoc32_09

## Summary

Port the C source file `mbrtoc32.c` into an equivalently scoped Rust implementation for the `cat` project branch `010-main_root_mbrtoc32_09-rust-port`.

This module is centered on the `mbrtoc32` routine, which converts a multibyte character sequence into a 32-bit character value while maintaining conversion state across calls. The Rust implementation should preserve the original function-level behavior and state-driven decoding flow rather than redesigning the interface around broader Unicode abstractions.

Technical approach:

- Create a Rust module dedicated to the migrated `mbrtoc32` logic.
- Represent the C conversion state explicitly in Rust using a small state carrier compatible with the original call patterns.
- Use standard library types and byte-slice processing as the default implementation technique.
- Preserve observable C semantics as closely as possible for:
  - partial input handling,
  - restartable conversion state,
  - error reporting,
  - null and empty input edge cases.
- Keep the implementation narrowly scoped to the migrated file and function behavior only.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**:
  - Rust standard library
  - No third-party crates recommended based on the available module evidence
- **Testing**: `cargo test`
- **Performance Goals**:
  - Match the C implementation’s practical complexity for per-call decoding.
  - Avoid unnecessary heap allocation; operate on caller-provided buffers/slices and explicit state.
  - Keep per-invocation overhead low and state transitions lightweight.
  - Preserve incremental decoding behavior without introducing extra buffering beyond what is needed for the conversion state.

## Module Mapping

| C File | C Function | Rust Module/File | Rust Item |
|---|---|---|---|
| `mbrtoc32.c` | `mbrtoc32` | `src/main_root_mbrtoc32_09.rs` or project-appropriate existing main-cluster module file | `pub(crate) fn mbrtoc32(...)` |
| `mbrtoc32.c` | internal helper logic, if present in file scope | same Rust module | private helper functions |

Notes:

- Since the source analysis lists only `mbrtoc32.c`, keep the Rust migration contained to a single module file.
- If the project already has a central module declaration for main-cluster ports, register this file there without introducing extra layers.
- The duplicate function listing is treated as the same exported routine rather than a need for two Rust functions.

## Data Model

No explicit C structs were identified in the analysis input. The main data mapping concern is conversion state and result signaling.

| C Concept | Rust Mapping | Notes |
|---|---|---|
| `char *` / `const char *` input | `&[u8]` or optional byte-slice input form | Prefer slices for memory safety; preserve null-like call distinctions via `Option<&[u8]>` only if required by original semantics |
| output `char32_t *` | mutable output parameter such as `&mut u32` or `&mut char` depending on exact behavior | Use `u32` if direct scalar preservation is needed before validating Unicode scalar constraints |
| `mbstate_t *` | explicit Rust state struct, e.g. `MbState` | Must retain partial-sequence state across calls |
| `size_t` return value | `usize` or a dedicated result enum wrapping `usize` and status | Choose the narrowest form that can still represent C-style special return cases cleanly |
| C error/special sentinel returns | Rust enum for conversion outcome | Prefer explicit outcome modeling over sentinel-only encoding internally |

Recommended internal Rust state model:

```rust
pub(crate) struct MbState {
    pending: [u8; 4],
    pending_len: u8,
    // add only fields required to mirror the C state machine
}
```

Recommended outcome model:

```rust
pub(crate) enum MbrToC32Result {
    Complete { bytes_read: usize, ch: u32 },
    Incomplete,
    InvalidSequence,
    StateReset,
}
```

Notes:

- Exact field layout should be driven by the original C logic, not generalized beyond what `mbrtoc32.c` requires.
- If the surrounding port already defines a shared multibyte state type, reuse it instead of introducing a parallel abstraction.
- Avoid unsafe code unless the surrounding project interface forces pointer-level compatibility inside Rust internals.

## Implementation Phases

## Phase 1: Inspect and Scaffold Direct File Port

Goals:

- Read `mbrtoc32.c` and identify the exact control flow, state transitions, and special return conditions.
- Create the target Rust module file and wire it into the existing crate module tree.
- Translate file-local constants and helper logic directly, keeping naming close enough to ease review.

Tasks:

- Map the C function signature to the Rust project’s existing conventions.
- Identify whether the implementation depends on locale-sensitive or UTF-specific assumptions and reflect only what is present in the C source.
- Introduce the minimal Rust state carrier required for `mbstate_t` behavior.
- Add comments only where needed to clarify non-obvious C-to-Rust semantic preservation.

Exit criteria:

- Rust module exists with compilable function skeletons and state representation.
- All C file-scope logic has a direct Rust home.

## Phase 2: Implement Conversion Logic and Error Semantics

Goals:

- Complete the Rust translation of the decoding logic.
- Preserve restartable behavior and C-equivalent special cases.

Tasks:

- Port byte consumption logic exactly, including:
  - initial-state handling,
  - partial multibyte sequence accumulation,
  - completed character emission,
  - invalid sequence detection,
  - reset behavior.
- Model C special return cases in Rust internally, then expose the project-appropriate return form.
- Ensure no out-of-bounds access occurs when reading input slices.
- Keep memory management stack-based and state-owned; do not allocate dynamically.

Error-handling decisions:

- Invalid multibyte sequences should be represented explicitly in the Rust control flow.
- Incomplete input must remain distinguishable from invalid input.
- Any C null-pointer-style branch should be mapped to an explicit Rust option or prevalidated caller contract, depending on existing crate patterns.

Exit criteria:

- The Rust function behaves equivalently to the C routine for normal, partial, reset, and invalid-input paths.
- The implementation compiles cleanly with no unnecessary unsafe usage.

## Phase 3: Add Focused Tests for Behavioral Parity

Goals:

- Validate the port against expected `mbrtoc32` semantics from the original file behavior.

Tasks:

- Add unit tests covering:
  - single-byte input,
  - multibyte complete input,
  - split input across multiple calls using persistent state,
  - incomplete sequence return behavior,
  - invalid sequence handling,
  - reset or null-equivalent input handling if present in C.
- Add tests that verify state mutation across calls rather than only final output.
- If the project already contains neighboring port tests, follow that structure exactly.

Exit criteria:

- `cargo test` passes.
- Core edge cases from the C logic are covered.
- Tests demonstrate parity for stateful decoding paths.

## Phase 4: Integration Review and Cleanup

Goals:

- Finalize module integration without expanding scope.

Tasks:

- Confirm the Rust module name, visibility, and placement match the project’s main-cluster conventions.
- Remove any temporary translation scaffolding left from the port.
- Verify that public/internal API exposure is no broader than required by the migrated C usage.
- Run formatting and ensure the code remains idiomatic while still traceable to the original C implementation.

Exit criteria:

- The migrated module is integrated into the branch cleanly.
- The implementation remains limited to the original file/function responsibilities.
- Formatting and tests pass.