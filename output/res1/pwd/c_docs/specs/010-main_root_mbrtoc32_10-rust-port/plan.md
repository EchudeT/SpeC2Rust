# Implementation Plan

## Summary
Port the C source file `mbrtoc32.c` into a Rust module that preserves the existing `mbrtoc32` behavior and call surface as closely as practical within the Rust project. The implementation should focus on reproducing the current multibyte-to-`char32_t` conversion logic, including stateful decoding behavior and edge-case handling, without introducing broader Unicode facilities or alternate APIs.

The Rust approach should:
- translate the existing decoding logic directly from `mbrtoc32.c`,
- represent C conversion state explicitly in Rust,
- use standard-library integer and byte types,
- keep error and partial-input behavior aligned with the C implementation,
- place tests around the migrated function behavior using representative byte sequences and state transitions.

## Technical Context
- **Language/Version**: Rust 1.75+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Maintain behaviorally equivalent decoding with no unnecessary allocation.
  - Operate directly on input byte slices and mutable conversion state.
  - Keep per-call overhead close to the C implementation by using straightforward branching and scalar operations.
  - Avoid introducing buffering or abstraction layers beyond what is needed to model C state and return semantics.

## Module Mapping
| C File | Rust Module/File | Notes |
|---|---|---|
| `mbrtoc32.c` | `src/main_root_mbrtoc32_10.rs` | Direct migration target for the `mbrtoc32` implementation. |

| C Function | Rust Item | Mapping Notes |
|---|---|---|
| `mbrtoc32` | `pub(crate) fn mbrtoc32(...)` | Main port of the conversion routine; preserve stateful decoding semantics. |
| `mbrtoc32` | single Rust implementation | Duplicate listing in analysis is treated as the same source function, not separate work items. |

## Data Model
No explicit C structs were identified in the analysis input. The Rust port should therefore introduce only the minimum state representation required by the migrated function.

| C Concept | Rust Representation | Notes |
|---|---|---|
| `char32_t` output value | `u32` | Direct scalar code point carrier. |
| input multibyte buffer | `&[u8]` | Borrowed slice instead of raw pointer/length pairs where compatible with surrounding code. |
| conversion state (`mbstate_t`-like behavior if present in logic) | private Rust `struct` | Minimal state fields only, matching the C routine’s decoding progress requirements. |
| C return status / size result | `usize` plus internal error/status encoding as needed | Keep semantics aligned with the original function’s result behavior; use a narrow Rust wrapper only if required to express incomplete/invalid input distinctly. |

If the surrounding project already has a shared multibyte-state type, reuse it instead of creating a parallel abstraction.

## Implementation Phases

### Phase 1: Establish module skeleton and function surface
- Create `src/main_root_mbrtoc32_10.rs`.
- Define the Rust function corresponding to `mbrtoc32`.
- Identify the exact input/output/state signature needed by the Rust project and map the C parameters to Rust references, slices, and mutable outputs.
- Add minimal internal state type only if required by the original C logic.
- Document any places where C pointer-null semantics must be represented explicitly in Rust.

### Phase 2: Port core decoding logic
- Translate the byte-processing logic from `mbrtoc32.c` into Rust with a close structural correspondence to the original control flow.
- Preserve handling for:
  - complete character decoding,
  - incomplete multibyte sequences,
  - invalid sequences,
  - state resets and state continuation across calls.
- Replace raw memory access with checked slice indexing or carefully structured positional access.
- Keep the implementation allocation-free.
- Ensure integer conversions are explicit to avoid accidental sign or width changes relative to C.

### Phase 3: Align return semantics and error behavior
- Reproduce the C routine’s return conventions exactly as far as the Rust codebase allows.
- Encode special outcomes such as incomplete input and invalid input in a form consistent with the existing project style while staying faithful to the original behavior.
- Verify that output code point mutation occurs only in the same cases as in the C implementation.
- Confirm that state mutation on success/failure matches the C routine’s expectations.

### Phase 4: Add focused tests and finalize migration
- Add `cargo test` coverage for:
  - ASCII single-byte decoding,
  - valid multibyte decoding,
  - incomplete sequences across multiple calls,
  - invalid byte sequences,
  - reset/initial-state behavior.
- Use tests to confirm output value, return value, and state transitions together.
- Remove any temporary scaffolding used during translation.
- Keep the final module limited to the migrated functionality from `mbrtoc32.c` only.