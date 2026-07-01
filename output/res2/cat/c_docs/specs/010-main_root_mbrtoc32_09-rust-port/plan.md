# Implementation Plan

## Summary

Port `mbrtoc32.c` into a Rust module that preserves the existing conversion behavior and call surface as closely as possible within idiomatic Rust constraints. The implementation should focus narrowly on the `mbrtoc32` routine, including its stateful multibyte-to-`char32_t` conversion semantics and edge-case handling around incomplete input, invalid sequences, and restartable decoding state.

The Rust approach should prefer the standard library and model the original C behavior explicitly rather than introducing broader text-processing abstractions. Because the C function is stateful and may depend on `mbstate_t`-like behavior, the Rust implementation should define a small internal state representation that tracks partial decoding progress and maps C return/error conventions into a Rust result shape used internally by the crate. Memory ownership remains simple because the source module is function-based and does not define heap-managed data structures.

## Technical Context

- **Language/Version**: Rust 1.74+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Match the C implementation’s asymptotic behavior for single-pass decoding.
  - Avoid unnecessary allocation; operate directly on input byte slices.
  - Keep per-call state updates minimal and predictable.
  - Preserve restartable decoding without introducing buffering beyond what the original state requires.

## Module Mapping

| C File | C Function | Rust Target | Notes |
|---|---|---|---|
| `mbrtoc32.c` | `mbrtoc32` | `src/main_root_mbrtoc32_09.rs` -> `pub(crate) fn mbrtoc32(...)` | Primary port of the conversion routine |
| `mbrtoc32.c` | internal/static logic within `mbrtoc32` | private helpers in `src/main_root_mbrtoc32_09.rs` | Only extract helpers required to mirror existing branches and state transitions |

### Rust module placement

- Create a single Rust source file for this migration unit:
  - `src/main_root_mbrtoc32_09.rs`
- Expose only the minimal crate-internal API needed by the existing project structure.
- Wire the module into the current crate module tree without creating extra layers or utility modules.

## Data Model

No explicit C structs were identified in the input, but the C routine implies stateful decoding data similar to `mbstate_t`. The Rust port should introduce only the minimal internal representations necessary to preserve behavior.

| C Concept | Rust Mapping | Notes |
|---|---|---|
| `char32_t` output | `u32` or `char` plus internal `u32` handling | Use `u32` where exact scalar value transport is needed; convert to `char` only if validation is guaranteed and compatible with original behavior |
| `const char *` input buffer | `&[u8]` | Avoid raw pointers in the core logic; pointer-oriented adaptation, if needed by surrounding code, should stay thin |
| `size_t` | `usize` | Direct mapping |
| `mbstate_t *` | private Rust state struct, e.g. `MbrToC32State` | Store only partial multibyte decode progress required by the C algorithm |
| C error/return sentinel values | `Result<..., DecodeError>` or small internal enum plus adapter | Preserve exact branch behavior, including incomplete-sequence and invalid-sequence outcomes |

### Proposed internal state shape

A minimal private state type should cover:

- partial bytes accumulated for an in-progress multibyte character
- count of bytes currently stored
- any mode/shift information required by the original algorithm

Example shape:

```rust
struct MbrToC32State {
    partial: [u8; 4],
    len: u8,
    // add only fields proven necessary by the C logic
}
```

If the original C implementation relies on external locale-driven conversion machinery rather than explicit UTF-8 assembly, keep the Rust state equally minimal and centered on observable behavior rather than inferred generality.

## Implementation Phases

## Phase 1: Inspect and map the C routine

- Read `mbrtoc32.c` and identify:
  - exact function signature and all call-site expectations
  - return-value meanings for success, null character, incomplete input, and invalid input
  - how conversion state is initialized, updated, and reset
  - whether the implementation delegates to another conversion primitive or contains direct decode logic
- Define the Rust function signature and internal result types to mirror those semantics.
- Create `src/main_root_mbrtoc32_09.rs` and add the module to the crate.

### Deliverables

- Module skeleton compiled into the crate
- Documented mapping of C parameters and return paths to Rust equivalents in code comments
- Private state type stub, if required

## Phase 2: Port the core decoding logic

- Translate the body of `mbrtoc32` branch-by-branch into Rust.
- Replace pointer arithmetic with indexed slice access while preserving byte consumption semantics.
- Implement state transitions exactly as in the C code:
  - fresh decode
  - continued decode from partial state
  - completion and state reset
  - invalid-sequence handling
  - incomplete-sequence handling
- Keep memory handling stack-based; do not allocate.
- Where C uses sentinel return codes, centralize the mapping in a small internal enum or adapter to avoid duplicated control flow mistakes.

### Deliverables

- Functional Rust implementation of `mbrtoc32`
- Private helper functions only where they directly reduce translation risk
- No added functionality beyond the original routine

## Phase 3: Validate semantic parity with tests

- Add unit tests covering:
  - ASCII / single-byte success path
  - multibyte success path
  - null character behavior
  - incomplete sequence with preserved state
  - invalid sequence error path
  - state reset after completion or failure, as required by the C behavior
- Add regression-style tests derived from observed C edge cases in `mbrtoc32.c`.
- Verify compilation and test execution with `cargo test`.

### Deliverables

- Rust unit tests in the module or adjacent test module
- Confirmed semantic parity for the identified edge cases

## Phase 4: Integrate and tighten interfaces

- Adjust visibility to the minimum needed by the rest of the crate.
- Confirm naming and module references match the branch’s migration conventions.
- Remove any temporary translation scaffolding that is no longer necessary after tests pass.
- Ensure error handling remains local and explicit, with no added abstraction layers beyond what the port requires.

### Deliverables

- Finalized module integrated into the crate
- Cleaned implementation with restrained internal API surface
- Passing `cargo test`

## Notes on Memory Management and Error Handling

- Use borrowed byte slices for input to avoid ownership transfer and allocation.
- Keep decode state in a small mutable struct passed by mutable reference where restartable semantics are required.
- Avoid unsafe code unless the surrounding crate API forces pointer-compatible boundaries; if unavoidable, isolate unsafe usage to thin adapters and keep the core conversion logic safe.
- Represent invalid and incomplete input distinctly so the Rust port does not collapse separate C outcomes into one generic error.
- Reset or preserve decoder state exactly according to the original C routine’s behavior, since this is the main correctness risk in the migration.