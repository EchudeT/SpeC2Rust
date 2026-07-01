# Implementation Plan

## Summary

This module ports the behavior of `gnu/getdtablesize.c` into Rust, focusing on the single exported function `_setmaxstdio_nothrow`. The implementation should preserve the original module boundary and keep the behavior minimal: accept the same conceptual input, enforce the same result constraints as the C code, and expose a Rust function with clear error-free return semantics matching the original “nothrow” intent.

The Rust implementation should prefer the standard library and only use direct OS interaction if the original logic depends on process file-descriptor or stdio limits that are not available through `std`. Because the source module is small and function-focused, the port should remain equally compact: one Rust source file for the module logic, plus unit tests covering boundary behavior and platform-specific branches.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**:
  - Rust standard library
  - `libc` crate only if required to query or clamp platform descriptor/stdio limits in a way not available in `std`
- **Testing**:
  - `cargo test`
- **Performance Goals**:
  - Constant-time execution for `_setmaxstdio_nothrow`
  - No heap allocation
  - No persistent state unless strictly required by the original C behavior
  - Minimal syscall usage; compute limits directly or query once per call only if unavoidable

## Module Mapping

### C to Rust File Mapping

- `gnu/getdtablesize.c`
  → `src/module_gnu_getdtablesize.rs`

### Function Mapping

- `_setmaxstdio_nothrow`
  → `pub(crate)` or `pub` Rust function with an integer-based signature reflecting the C contract

The exact visibility should match how the wider Rust port references this module. Do not introduce additional public APIs beyond the migrated function.

## Data Model

This module does not define named C structs. The only listed data structure is anonymous, so the Rust port should avoid inventing replacement structs unless they are strictly needed for internal clarity.

### Data-Structure Mapping

- anonymous C data
  → local Rust variables / constants / private helper values

### Type Mapping Notes

- C integer parameters and return values
  → Rust fixed-width or platform integer types chosen to preserve range and comparison behavior
- Any sentinel return convention from C
  → Rust integer return value preserved directly if the wider port expects C-like semantics

Because the function is explicitly “nothrow”, prefer:
- total functions returning plain integers when the original C code uses sentinel values
- internal checked conversions where narrowing may occur
- no `panic!` paths in normal operation

## Implementation Phases

### Phase 1: Inspect and Map C Semantics

- Read `gnu/getdtablesize.c` and isolate the exact behavior of `_setmaxstdio_nothrow`.
- Identify:
  - parameter type and valid range handling
  - return value rules
  - clamp/min/max behavior
  - any dependency on compile-time constants or platform APIs
- Determine whether the function is:
  - a stub-style compatibility function
  - a limit-query helper
  - a limit-setting helper with constrained success/failure behavior
- Record any platform assumptions that must remain in the Rust port.

**Exit criteria**:
- Rust signature selected
- required constants and any OS calls identified
- no extra APIs proposed

### Phase 2: Implement the Rust Module

- Create `src/module_gnu_getdtablesize.rs`.
- Port `_setmaxstdio_nothrow` directly, preserving:
  - integer behavior
  - boundary checks
  - no-throw semantics
  - original return conventions
- Use `std` first for numeric and environment-independent logic.
- If the C code depends on OS-level descriptor/stdio limits unavailable in `std`, add `libc` narrowly for the exact call or constant needed.
- Keep unsafe code isolated and documented if `libc` access is required.
- Avoid introducing shared global state, caching, or abstraction layers not present in the source.

**Exit criteria**:
- function compiles
- implementation behavior mirrors C logic
- any unsafe block is minimal and justified

### Phase 3: Add Focused Tests

- Add unit tests in the same module or the standard Rust test location.
- Cover:
  - normal accepted input
  - lower-bound and upper-bound inputs
  - out-of-range handling
  - any sentinel or unchanged-value return behavior
- If behavior differs by platform, gate assertions appropriately and test invariant properties instead of over-specifying OS-specific values.

**Exit criteria**:
- `cargo test` passes
- boundary behavior is covered
- tests validate compatibility-oriented semantics rather than Rust-idiomatic redesign

### Phase 4: Integration Cleanup

- Wire the module into the crate with the minimal required `mod` declaration and visibility.
- Remove unused imports and confirm no accidental API expansion occurred.
- Verify formatting and lint cleanliness at the module level.
- Recheck that memory management is trivial and stack-based, with no leaks or ownership issues.
- Confirm error handling remains non-panicking and consistent with the original C nothrow contract.

**Exit criteria**:
- module is reachable from the intended crate path
- no unnecessary dependencies or helper modules added
- final code remains a direct migration of the original unit