# Implementation Plan: module_gnu_msvc-nothrow.c_37

## Summary

Port `gnu/msvc-nothrow.c` into a focused Rust module that preserves the existing behavior of `_gl_nothrow_get_osfhandle` without adding new surface area. The Rust implementation should provide the same core responsibility: obtain an OS file handle from a C-runtime file descriptor on Windows while avoiding exception-style behavior and expressing failure through ordinary return values.

The technical approach should stay minimal and close to the source module:

- create one Rust module corresponding to `gnu/msvc-nothrow.c`
- implement the single function as a thin Windows-specific wrapper
- use Rust’s platform-specific standard-library facilities where sufficient
- use a small amount of Windows-specific FFI only if the standard library does not expose the required conversion directly
- preserve sentinel/error semantics explicitly, rather than redesigning the API into a broader abstraction

Memory management remains simple because the source function does not allocate owned data structures. The main migration concern is accurate type mapping, Windows ABI compatibility if FFI is needed, and preserving C-like invalid-handle/error behavior in a Rust-safe implementation.

## Technical Context

- **Language/Version**: Rust 1.77 or newer
- **Primary Dependencies**:
  - Rust standard library
  - No third-party crates by default
  - If direct binding to the CRT entry point is required, use manual `extern "C"` declarations instead of introducing extra crates
- **Testing**: `cargo test`
- **Performance Goals**:
  - constant-time wrapper behavior
  - no heap allocation
  - no additional handle duplication or ownership transfer
  - negligible overhead compared with the original C function

## Module Mapping

| C Source File | Rust Target File | Notes |
|---|---|---|
| `gnu/msvc-nothrow.c` | `src/gnu/msvc_nothrow.rs` | Direct port of the single-function module |
| function `_gl_nothrow_get_osfhandle` | `pub(crate)` or internal Rust function of the same logical role | Keep naming close to source, adapting to Rust naming conventions only if required by project style |

Recommended crate-local structure:

| Rust Path | Purpose |
|---|---|
| `src/gnu/mod.rs` | Declares `msvc_nothrow` submodule if a `gnu` module tree already exists |
| `src/gnu/msvc_nothrow.rs` | Contains the full implementation of the ported function |
| related parent module re-export | Only if needed by existing call sites; avoid unnecessary public API expansion |

## Data Model

This module has no custom structs in the analyzed input. The required work is limited to C primitive and platform-handle type translation.

| C Type / Concept | Rust Mapping | Notes |
|---|---|---|
| C runtime file descriptor (`int`) | `i32` | Preserve exact width assumptions used by the CRT-facing API |
| OS file handle return value (`intptr_t`/handle-sized integer) | `isize` | Matches pointer-sized signed integer semantics |
| invalid handle sentinel | constant `-1isize` or equivalent named constant | Preserve original failure signaling |
| Windows `HANDLE` conceptual value | represented as `isize` when mirroring `_get_osfhandle` result | Avoid introducing ownership wrappers since the function only reports a borrowed/raw handle value |

Error handling model:

| C Behavior | Rust Behavior |
|---|---|
| returns invalid sentinel on failure | return the same sentinel-compatible `isize` value |
| no allocation cleanup required | no special memory management needed |
| no exception propagation intended | do not use panicking paths for normal failure cases |

## Implementation Phases

### Phase 1: Create the Rust module skeleton

- Add `src/gnu/msvc_nothrow.rs`.
- Wire it into the existing module tree with the smallest required `mod` declaration changes.
- Define the Rust signature for the ported function using C-compatible primitive types.
- Add Windows-only compilation guards as appropriate.
- Decide visibility based strictly on existing project call paths; prefer `pub(crate)` over `pub` unless external exposure is already required.

**Exit criteria**:
- module compiles within the crate structure
- function stub exists with final intended signature
- no extra modules or helper layers introduced

### Phase 2: Port `_gl_nothrow_get_osfhandle`

- Translate the C implementation directly into Rust logic.
- Prefer a minimal implementation path:
  - if Rust std facilities can express the descriptor-to-handle conversion exactly, use them
  - otherwise, declare the needed CRT symbol with `extern "C"` and call it inside a small `unsafe` block
- Keep the return convention aligned with the C module, especially invalid-handle signaling.
- Document the safety assumptions around:
  - valid ABI for the CRT function
  - integer width compatibility
  - non-owning/raw-handle semantics

**Exit criteria**:
- Rust function behavior matches the original module’s success/failure contract
- unsafe usage is narrowly scoped and justified
- no ownership transfer or resource-lifetime changes introduced

### Phase 3: Add focused tests

- Add unit tests or platform-gated tests in the same module or crate test layout.
- Cover only behavior directly implied by the migrated function:
  - invalid file descriptor returns the expected failure sentinel
  - a valid descriptor derived from an open file yields a non-invalid handle result on Windows
- Gate Windows-specific tests with `#[cfg(windows)]`.
- Avoid speculative cross-platform emulation; on non-Windows targets, either omit tests or compile out the module as dictated by the original usage context.

**Exit criteria**:
- `cargo test` passes
- tests validate both failure-path and basic success-path behavior where supported
- no broad test harness or unrelated fixtures added

### Phase 4: Integration cleanup

- Update any existing call sites to reference the Rust module path.
- Remove or disable the original C source from the Rust port build path as appropriate for the branch migration.
- Confirm naming, visibility, and cfg boundaries are consistent with the rest of the crate.
- Perform a final review for:
  - sentinel correctness
  - absence of unnecessary allocations
  - minimal unsafe surface
  - no API expansion beyond the original single-function module

**Exit criteria**:
- Rust port is the active implementation for this module in the branch
- build and tests succeed cleanly
- migration stays limited to the original file and function scope