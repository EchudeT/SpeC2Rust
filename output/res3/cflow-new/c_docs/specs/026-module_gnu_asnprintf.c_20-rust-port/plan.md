# Implementation Plan

## Summary

Port `gnu/asnprintf.c` into a focused Rust module that preserves the existing module boundary and behavior scope around `asnprintf`. The Rust implementation should center on safe, owned string/buffer construction using `Vec<u8>` and/or `String`, with explicit handling for output size, truncation semantics if required by the existing call pattern, and allocation failure represented through Rust error returns rather than C-style null/error signaling.

The implementation should migrate only the functionality present in `asnprintf`, not introduce broader formatting infrastructure. The preferred approach is to express the formatting and buffer-growth logic with Rust standard library facilities, keeping allocation ownership explicit and avoiding manual memory management patterns from C.

## Technical Context

- **Language/Version**: Rust 1.76+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Match the C module’s practical allocation behavior closely enough for equivalent workloads.
  - Avoid unnecessary intermediate allocations where the final buffer can be built directly.
  - Preserve predictable resizing behavior for dynamically formatted output.
  - Keep copying bounded to the final produced output plus any unavoidable formatting work.

## Module Mapping

| C Source File | Rust Target |
|---|---|
| `gnu/asnprintf.c` | `src/module_gnu_asnprintf.rs` |

| C Function | Rust Function |
|---|---|
| `asnprintf` | `pub fn asnprintf(...) -> Result<..., ...>` |

### Notes
- Keep the Rust implementation contained in a single module file corresponding to the source being migrated.
- Do not split helpers into additional modules unless required by compilation constraints inside the same file.
- If the surrounding crate already has an established module layout, expose this module through the existing `mod` declarations without expanding public API surface beyond what is needed for `asnprintf`.

## Data Model

No explicit C structs are listed for this module.

### C-to-Rust Representation

| C Concept | Rust Representation |
|---|---|
| Dynamically allocated output buffer | `Vec<u8>` or `String` depending on byte vs UTF-8 requirements |
| Output length / buffer size (`size_t`) | `usize` |
| Null/error return | `Result<T, AsnprintfError>` |
| Possibly optional incoming buffer ownership | `Option<Vec<u8>>` or direct owned buffer parameter, depending on actual C signature usage |

### Data Structure Decisions

- Prefer `Vec<u8>` if the original function operates on raw bytes or does not guarantee UTF-8.
- Prefer `String` only if the formatting contract is text-oriented and valid UTF-8 can be guaranteed by the Rust-side inputs.
- Introduce a small module-local error type only if needed to distinguish formatting failure, capacity overflow, or invalid input assumptions.
- Do not invent replacement structs where a direct standard-library container is sufficient.

## Implementation Phases

### Phase 1: Inspect Signature and Establish Rust API Surface

- Confirm the exact `asnprintf` signature and responsibilities in `gnu/asnprintf.c`.
- Determine whether the function:
  - allocates a new buffer unconditionally,
  - resizes/reuses an incoming buffer,
  - returns byte length, pointer, or both through out-parameters.
- Define the narrowest Rust function signature that preserves the same operational contract within Rust idioms.
- Decide early whether the implementation must be byte-oriented (`Vec<u8>`) or string-oriented (`String`).
- Map C error signaling to `Result` and identify any length/output values that must be returned alongside the buffer.

**Deliverable**:
- Compiling Rust module skeleton in `src/module_gnu_asnprintf.rs` with the target function signature and placeholder tests.

### Phase 2: Port Core Buffer Construction Logic

- Translate the allocation and growth logic from `asnprintf` into Rust using owned buffers.
- Replace manual C memory handling with:
  - `Vec::with_capacity` / `reserve` for byte buffers, or
  - `String::with_capacity` for text buffers.
- Reproduce size accounting carefully:
  - requested capacity,
  - produced length,
  - terminating-null expectations if the original contract depends on C-string compatibility.
- If the C function’s semantics include returning usable length separate from storage capacity, preserve that explicitly in the Rust return type.
- Avoid unsafe code unless the exact C behavior requires low-level buffer manipulation that cannot be expressed directly with safe APIs.

**Deliverable**:
- Functional Rust implementation of `asnprintf` with explicit ownership and error propagation.

### Phase 3: Align Edge Cases and Error Semantics

- Port boundary behavior from the C implementation:
  - empty format/output case,
  - zero-length or minimal-capacity handling,
  - resize failure equivalents,
  - length overflow checks where C used `size_t`.
- Add explicit guards for:
  - `usize` overflow during capacity calculations,
  - mismatch between logical output length and allocated storage,
  - invalid assumptions about UTF-8 if text storage was chosen.
- Ensure the Rust function does not expose partially initialized output on failure.
- Verify whether a trailing NUL must be stored internally for compatibility with the original semantics; if so, keep that as an internal byte-level concern rather than exposing C pointers.

**Deliverable**:
- Completed edge-case handling and stable error model.

### Phase 4: Test Migration and Final Integration

- Write unit tests directly against `asnprintf` behavior derived from the C module:
  - basic formatted output,
  - larger output requiring growth,
  - empty output,
  - exact-size boundary conditions,
  - failure-path assertions where representable in Rust.
- Confirm the module is wired into the crate with minimal public exposure.
- Run `cargo test` and resolve any behavioral mismatches with the original C logic.
- Keep the final implementation limited to the migrated file and function scope.

**Deliverable**:
- Passing tests and integrated Rust module matching the original module boundary.