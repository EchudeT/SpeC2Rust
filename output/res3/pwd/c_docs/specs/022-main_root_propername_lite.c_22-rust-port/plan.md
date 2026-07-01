# Implementation Plan

## Summary
This module ports the C source `propername-lite.c` into a focused Rust implementation that preserves the existing behavior of `proper_name_lite` without introducing new capabilities. The Rust version should keep the logic localized to a single module and use standard-library string handling to replace C pointer- and buffer-oriented operations.

The technical approach is:
- map the C function `proper_name_lite` to a Rust function in a single corresponding module;
- translate C string processing into borrowed `&str` and owned `String` operations as needed;
- make memory ownership explicit so there is no manual allocation/free lifecycle;
- represent any failure conditions through a Rust result type only if the C logic has meaningful failure paths; otherwise keep the API direct and minimal;
- validate parity with the original behavior using targeted unit tests derived from the C function’s observed input/output behavior.

## Technical Context

### Language/Version
- Rust 1.78+
  A current stable Rust toolchain is sufficient for this migration.

### Primary Dependencies
- Rust standard library only

No third-party crates are recommended from the available module evidence. The module appears limited to string/name handling and should be implementable with `std`.

### Testing
- `cargo test`

Testing should emphasize:
- direct behavior coverage for `proper_name_lite`;
- string edge cases relevant to the C implementation, such as empty input, ASCII/non-ASCII handling if applicable, and formatting/presentation boundaries implied by the original function.

### Performance Goals
- Preserve the practical performance characteristics of the C routine for typical command-line utility usage.
- Avoid unnecessary allocations where a borrowed-string path is sufficient.
- Keep per-call overhead low and linear in input length.
- Prefer straightforward standard-library operations over layered abstractions.

## Module Mapping

### C to Rust File Mapping
- `propername-lite.c` → `src/propername_lite.rs`

### Function Mapping
- `proper_name_lite` → `proper_name_lite`

If the function is only used by the crate entry path, it should be exposed with the narrowest visibility that still supports existing call sites, ideally `pub(crate)` rather than broader export.

## Data Model

No explicit C structs or custom data containers were identified for this module.

### C to Rust Type Mapping
- C string inputs (`char *` / `const char *`) → `&str` where UTF-8 text is expected in the Rust port
- C-owned returned string or constructed textual output → `String`
- C nullability conventions → `Option<&str>` or `Option<String>` only if null is a real part of the original function contract
- C status/error signaling → `Result<T, E>` only if the original behavior requires explicit failure propagation

### Memory Management Notes
- Replace manual allocation and lifetime management from C with Rust ownership.
- Avoid exposing raw buffers.
- Keep conversions minimal: borrow input text where possible, allocate only for produced output.
- If the original C logic tolerated null pointers, make that explicit in the Rust signature rather than emulating unsafe behavior internally.

## Implementation Phases

### Phase 1: Module Skeleton and API Translation
- Create `src/propername_lite.rs`.
- Define the Rust signature for `proper_name_lite` based on the C call pattern and actual usage needs.
- Decide visibility and return type conservatively, matching the existing module role.
- Add the module declaration from the crate root or current caller location with no extra structural expansion.

### Phase 2: Core Logic Port
- Translate the body of `proper_name_lite` directly from C into idiomatic but restrained Rust.
- Replace pointer checks and buffer manipulation with explicit string/option handling.
- Preserve existing formatting and decision order from the C implementation.
- Keep helper logic inline unless a tiny private helper is necessary to mirror a repeated local operation from the source.

### Phase 3: Error and Ownership Review
- Confirm that all C memory-management assumptions are removed cleanly in favor of owned/borrowed Rust values.
- Review any edge conditions previously represented by null pointers, empty strings, or sentinel return values.
- Minimize allocations and temporary strings while keeping the port readable and faithful.

### Phase 4: Tests and Integration Verification
- Add unit tests for `proper_name_lite` covering normal and edge inputs inferred from the C behavior.
- Verify the module builds and integrates with the rest of the Rust port branch.
- Run `cargo test` and adjust only for behavioral parity or compile-time integration issues.