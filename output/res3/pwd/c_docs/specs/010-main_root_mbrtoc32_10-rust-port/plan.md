# Implementation Plan: main_root_mbrtoc32_10

## Summary

This module migrates the C file `mbrtoc32.c` into an idiomatic Rust implementation that preserves the existing conversion behavior and call surface as closely as possible within Rust’s type system.

The implementation should focus narrowly on porting the `mbrtoc32` logic from the C source into a Rust module in the main executable/library cluster, without introducing broader Unicode utilities or new abstractions beyond what is required for parity. The likely technical approach is:

- represent the C routine as a Rust function in the crate’s main module tree,
- map the C input/output pointer-oriented API into Rust parameters using slices, `Option`, and explicit state structures where needed,
- preserve stateful decoding behavior for partial multibyte sequences,
- use standard-library Unicode facilities only where they match the C semantics closely enough,
- handle incomplete input, invalid sequences, and reset/state transitions explicitly rather than through panics.

The plan should treat this as a direct migration of one existing source file and its function behavior, with tests centered on parity with the C implementation’s observable results.

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Match the C implementation’s asymptotic behavior for per-call decoding.
  - Avoid unnecessary allocation; perform decoding directly from input byte slices.
  - Keep state updates explicit and cheap.
  - Maintain predictable error paths with no panic-driven control flow in normal operation.

## Module Mapping

| C Source File | C Function | Rust Module | Rust Item |
|---|---|---|---|
| `mbrtoc32.c` | `mbrtoc32` | `src/main_root_mbrtoc32_10.rs` or integrated into the existing main-cluster module file | `pub(crate) fn mbrtoc32(...) -> ...` |
| `mbrtoc32.c` | internal/shared `mbrtoc32` entry handling if duplicated by analysis | same Rust module | single Rust implementation with one canonical function |

### Mapping Notes

- Because the input names list `mbrtoc32` twice, treat this as one exported behavior unless the source file clearly contains separate declarations/wrappers. If there is a wrapper and a core routine, keep both only if the file requires that structure for exact migration.
- Place the code in the existing main-cluster Rust layout. If the branch already has a root module for this cluster, integrate there rather than creating extra submodules.

## Data Model

No explicit C structs were identified in the analysis input. The migration should therefore keep the data model minimal and only introduce Rust types required to represent C state safely.

| C Concept | Rust Mapping | Notes |
|---|---|---|
| `char *` / `const char *` input buffer | `&[u8]` | Use borrowed byte slices instead of raw pointers where possible. |
| `char32_t` output value | `u32` or `char` | Prefer `u32` if exact C value/error boundary handling is needed; convert to `char` only after validation. |
| `mbstate_t *` conversion state | dedicated Rust state struct, or `Option<&mut State>` | Use an explicit state type if the C logic depends on incremental decoding across calls. |
| null pointer reset/query behavior | `Option` on inputs/state | Model nullable C arguments explicitly. |
| C error/status return (`size_t` with special values) | `Result<usize, DecodeError>` or exact `usize`-style wrapper | Choose exact-status representation if strict parity with C sentinel returns is required by surrounding code. |

### Error Handling Mapping

If the surrounding port expects libc-like return conventions, define a small internal enum to represent outcomes before converting to the final return type:

```rust
enum MbrToC32Status {
    Complete { bytes: usize, ch: u32 },
    Incomplete,
    Invalid,
    Reset,
}
```

This keeps the port readable while still allowing the public Rust function to expose the closest required signature.

### Memory Management Notes

- No heap allocation should be introduced for decoding.
- Input should be borrowed only for the duration of the call.
- Stateful decoding must store only the minimum carry-over data needed to match `mbstate_t` behavior.
- Avoid unsafe code unless exact pointer/null semantics from the original API cannot be represented otherwise. If unsafe is necessary, isolate it at the boundary and keep the decoding core safe.

## Implementation Phases

## Phase 1: Inspect and define the direct Rust surface

- Review `mbrtoc32.c` and identify:
  - exact function signature(s),
  - special return values,
  - how `mbstate_t` is read and updated,
  - whether the implementation depends on locale-specific or UTF-specific assumptions.
- Decide the Rust function signature based on actual call sites in the port branch.
- Create the destination Rust module/file in the existing main-cluster layout.
- Define minimal Rust state and status types required to express the C behavior without adding broader infrastructure.

### Deliverables

- Rust module file stub for the migrated implementation.
- Function signature finalized from the C source and existing Rust branch integration points.
- Internal state/error/status types sketched in code.

## Phase 2: Port the decoding logic

- Translate the body of `mbrtoc32` into Rust in a structure that mirrors the C control flow closely.
- Preserve:
  - state reset behavior,
  - partial-sequence handling,
  - invalid-sequence detection,
  - output assignment rules,
  - return-value semantics.
- Use standard-library primitives for byte inspection and Unicode validation only where they do not alter edge-case behavior.
- Keep all mutation of conversion state explicit and local to the function.

### Deliverables

- Working Rust implementation of `mbrtoc32`.
- Inline comments only where needed to document non-obvious C-to-Rust semantic preservation.
- No additional helper modules beyond small local helpers required by the port.

## Phase 3: Add parity-focused tests

- Write unit tests covering:
  - valid single-call conversions,
  - multibyte sequences,
  - incomplete input with preserved state,
  - invalid input handling,
  - reset behavior when nullable inputs/state are used, if applicable.
- Prefer test vectors derived directly from the C function behavior rather than generalized Unicode testing.
- Confirm `cargo test` passes in the target branch.

### Deliverables

- Unit tests in the module or adjacent test file.
- Verified behavior for normal, incomplete, and error cases.

## Phase 4: Integration cleanup

- Align naming, visibility, and placement with the rest of the Rust port branch.
- Remove any temporary scaffolding introduced during migration.
- Verify no unnecessary allocations, no panic-based error handling, and no unused abstractions remain.
- Ensure the final implementation maps one-for-one to the existing module responsibility only.

### Deliverables

- Final integrated Rust module for `main_root_mbrtoc32_10`.
- Clean compilation and test pass on `010-main_root_mbrtoc32_10-rust-port`.

## Notes and Constraints

- Keep this migration limited to `mbrtoc32.c` and its directly required state/return representations.
- Do not introduce new Unicode conversion layers or generalized encoding frameworks.
- Prefer exact behavioral parity over stylistic refactoring.
- If the original C code relies on sentinel return values that callers inspect directly, preserve those semantics at the Rust boundary rather than normalizing them away.
- If locale-sensitive behavior is present in the C source, document the exact assumption used by the Rust port and keep the implementation constrained to that observed behavior only.