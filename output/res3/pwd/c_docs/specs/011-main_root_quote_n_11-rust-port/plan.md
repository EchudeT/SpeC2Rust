# Implementation Plan

## Summary

Port the `quotearg.c` functionality for `quote_n_mem` and `quote_n` into a Rust module that preserves the existing call patterns and quoting behavior expected by the main program path. The Rust implementation should stay narrowly scoped to these two exported functions and the internal state they require, without introducing new abstractions beyond what is needed to replace the C file.

The technical approach is to migrate the relevant logic from `quotearg.c` into a single Rust source module, using owned `String`/`Vec<u8>` storage where the C code relied on reusable buffers and pointer-managed memory. Because `quote_n_mem` operates on byte sequences and `quote_n` is a string-oriented wrapper, the Rust implementation should keep the byte-oriented core path and layer the string convenience function on top. Any C static slot management used to support numbered quote buffers should be mapped to a Rust-local storage strategy that preserves function semantics while avoiding manual allocation and frees.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve linear-time processing relative to input length.
  - Avoid unnecessary intermediate allocations beyond what is needed to produce the quoted result.
  - Reuse per-slot storage where the C implementation reused buffers, as long as this remains simple and contained within the module.
  - Maintain behavior suitable for command-line utility execution without adding heavier infrastructure.

## Module Mapping

- **C source**
  - `quotearg.c`

- **Rust destination**
  - `src/main_root_quote_n_11.rs` or the project’s existing main-cluster module file layout for this branch
  - If the project already has a consolidated quoting module, place the migrated functions there instead of creating extra helper modules

- **Function mapping**
  - `quote_n_mem` -> `pub(crate) fn quote_n_mem(...) -> ...`
  - `quote_n` -> `pub(crate) fn quote_n(...) -> ...`

- **Internal migration scope**
  - Migrate only the helper routines and local state strictly required by `quote_n_mem` and `quote_n`
  - Do not broaden the port to unrelated quoting entry points from `quotearg.c` unless directly required for these two functions to compile and behave correctly

## Data Model

The analysis only exposes anonymous C data structures, so the Rust plan should treat them as implementation-local types and port only those proven necessary for the two target functions.

| C representation | Rust mapping | Notes |
|---|---|---|
| anonymous struct used for quoting options/state | private `struct` | Name according to its actual role once identified during port, e.g. quoting options or slot state |
| anonymous enum-like integer flags | private `enum` or `bitflags`-style constants using plain integers | Prefer plain constants or small enums from `std`; avoid external crates |
| anonymous static buffer array / slot table | private `Vec<Slot>` or fixed internal collection | Use Rust-owned storage instead of raw pointers |
| anonymous buffer record | private `struct Slot { buf: Vec<u8> / String, ... }` | Exact fields depend on C usage discovered during migration |
| anonymous pointer-based string view | `&[u8]` or `&str` | Use `&[u8]` for byte-preserving quoting core |
| C null-terminated string input | `&str` for `quote_n`; `&[u8]` plus length for `quote_n_mem` | Keep API split aligned with original function roles |

### Memory Management Notes

- Replace manual allocation and free behavior with ownership through `Vec<u8>` or `String`.
- If the C implementation returns pointers into reusable static slot buffers, emulate this with module-local owned storage whose lifetime is managed safely within Rust.
- Avoid leaking memory to mimic static C return pointers unless the surrounding project API absolutely forces borrowed `&str` results with static duration; prefer returning owned values when compatible with the existing Rust port architecture.

### Error Handling Notes

- The original C functions are not typically error-rich APIs, so Rust error handling should remain minimal.
- For string conversion, avoid lossy UTF-8 assumptions in the core path; keep quoting logic byte-based.
- If wrapper functions require `&str`, perform conversions only at the wrapper boundary and keep internal processing on bytes.

## Implementation Phases

### Phase 1: Isolate C scope and define Rust module surface

- Inspect `quotearg.c` and identify the exact internal helpers, constants, and static state referenced by `quote_n_mem` and `quote_n`.
- Determine whether the C implementation depends on shared quoting-option defaults or slot arrays defined in the same file.
- Create the target Rust module file in the branch layout and declare only the two exported functions plus the minimum private support types needed.
- Define Rust signatures that match the surrounding project’s conventions while preserving the distinction between byte-length input and string wrapper behavior.

### Phase 2: Port quoting state and core byte-processing path

- Port the internal option/state structures required by `quote_n_mem` into private Rust structs/enums/constants.
- Replace raw buffer management with owned Rust buffers.
- Implement the slot-indexed storage behavior required by `quote_n_mem`, preserving numbered-buffer semantics from the C code.
- Port the quoting loop and escaping rules exactly as needed for this function’s current behavior, staying byte-oriented to avoid semantic drift.
- Ensure boundary cases from the C implementation are preserved, including empty input, embedded non-UTF-8 bytes, and slot reuse.

### Phase 3: Implement `quote_n` wrapper and integrate with callers

- Implement `quote_n` as the string-oriented wrapper over the byte-based `quote_n_mem` path.
- Align return types and borrowing/ownership behavior with the rest of the Rust port so callers can migrate without widening the interface surface.
- Remove any remaining direct dependence on C-style null termination in the migrated path.
- Verify that module visibility and imports fit the existing main-cluster structure without adding new architectural layers.

### Phase 4: Validation and cleanup

- Add targeted unit tests for:
  - basic quoting output
  - empty strings
  - repeated calls using different slot numbers
  - repeated calls reusing the same slot
  - byte-preserving behavior for non-UTF-8 input in `quote_n_mem`
  - wrapper parity between `quote_n` and `quote_n_mem` for valid string inputs
- Run `cargo test` and fix behavioral mismatches against the original C logic.
- Remove dead helper code not required by these two functions.
- Keep the final module limited to the migrated file/function scope for this branch.