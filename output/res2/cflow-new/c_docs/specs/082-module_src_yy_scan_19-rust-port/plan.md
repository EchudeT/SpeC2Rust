# Implementation Plan: module_src_yy_scan_19

## Summary

This module migration covers the scanner buffer entry points currently implemented in `src/c.c`: `yy_scan_string` and `yy_scan_bytes`. The Rust implementation should preserve the existing scanner-oriented behavior by porting only the buffer construction and handoff logic required for these two functions, without widening the API surface.

The technical approach is to translate the C scanner buffer setup into Rust using owned byte storage and explicit lifetime control. The port should model the original temporary scan buffer semantics with a Rust-owned buffer that includes any required scanner sentinels, then connect that buffer to the existing or migrated scanner state in the smallest possible way. Where the C code relies on raw pointers and manually managed memory, Rust should use standard-library containers such as `Vec<u8>` and narrow internal mutability only where required by the surrounding scanner design.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve linear-time copying behavior for byte/string scan setup.
  - Avoid unnecessary intermediate allocations beyond the scanner-owned buffer required by the original C logic.
  - Maintain low-overhead scanner state transitions comparable to the C implementation.
  - Keep memory ownership explicit so buffer teardown remains deterministic.

## Technical Context Details

### Rust Edition and Compatibility
- Use the project’s standard Rust edition if already established; otherwise target Rust 2021.
- Keep implementation compatible with stable Rust 1.78+.
- Avoid nightly features and unnecessary external crates.

### Dependency Choices
- Prefer:
  - `Vec<u8>` for scanner buffer storage
  - `String`/`&str` handling only at the API boundary for `yy_scan_string`
  - `Option`, `Result`, and enums for internal state representation where needed
- Do not introduce third-party crates unless another migrated scanner component already requires them.

### Testing Approach
- Unit tests should validate:
  - `yy_scan_string` copies input bytes correctly into scanner-owned storage.
  - `yy_scan_bytes` accepts arbitrary byte content, including embedded NUL bytes.
  - Sentinel/end-marker bytes expected by the scanner are appended exactly as required.
  - Empty input is handled identically to the C behavior.
  - Buffer ownership remains valid for the duration required by scanner state.
- If scanner state APIs already exist in Rust, add focused integration tests that confirm the new buffer can be activated and read by the scanner.

### Performance and Memory
- Allocate exactly enough capacity for input bytes plus required trailing scanner markers.
- Copy once from the source slice/string into owned scanner storage.
- Do not add caching, pooling, or alternate buffer strategies not present in the C module.
- Ensure teardown occurs via normal Rust drop semantics rather than manual free logic.

## Module Mapping

### Source Mapping
- **C source file**: `src/c.c`
- **Rust target module**: `src/c.rs`

### Function Mapping
- `yy_scan_string`
  - Port into `src/c.rs` as a direct Rust function with the same conceptual role.
  - Accept a Rust string reference at the boundary, convert to bytes, then delegate to shared byte-scan setup logic where possible.
- `yy_scan_bytes`
  - Port into `src/c.rs` as the primary byte-buffer construction function.
  - Implement the core allocation, copy, trailing marker insertion, and scanner buffer activation steps here or in a private helper used only by these two functions.

### Internal Helper Mapping
- If the original C implementation depends on shared local scanner-buffer setup code in the same file, keep that logic private within `src/c.rs`.
- Do not split this migration into additional Rust modules unless the surrounding project already requires a scanner state type in another existing file.

## Data Model

The analysis reports only anonymous C data structures. For this plan, the migration should treat them conservatively and map only the structures directly touched by `yy_scan_string` and `yy_scan_bytes`.

### Data-Structure Mapping Strategy
- **Anonymous C scanner buffer struct(s)**
  → named Rust `struct` representing scanner buffer state, only if these functions directly create or return such a value.
- **Anonymous C scanner state struct(s)**
  → named Rust `struct` or mutable context type already used by the scanner port, referenced rather than redesigned.
- **Anonymous flag/integer fields**
  → `bool`, `usize`, or fixed-width integer types (`i32`, `u8`) based on actual C usage in the source.
- **Anonymous pointer-to-buffer fields**
  → `Vec<u8>` for owned storage, and index/offset fields instead of raw pointer arithmetic where feasible.
- **Anonymous optional pointer fields**
  → `Option<T>` / `Option<usize>` / `Option<NonNull<T>>` only if raw linkage to existing scanner internals is unavoidable.

### Expected Rust Structures
Because the C analysis exposes only anonymous structures, finalize names from actual usage during implementation. The likely minimal mappings are:

- `struct YyBufferState`
  - Holds the owned scan buffer.
  - Tracks current usable length and any scanner-required metadata.
  - Replaces C heap allocation and raw byte pointer ownership.

- `struct ScannerState` or existing scanner context type
  - Receives the new active buffer.
  - Stores or references `YyBufferState` according to the scanner architecture already being migrated.

### Memory Management Decisions
- Replace C manual allocation/free for scan buffers with Rust ownership.
- If C returns a buffer handle that must outlive the call, return an owned Rust buffer state object or insert it into scanner-owned state explicitly.
- If the surrounding scanner design still requires internal pointer-like references, derive them from owned storage at use time and keep unsafe code tightly scoped and documented.

### Error Handling Decisions
- Mirror the C behavior closely:
  - If the original functions assume allocation success and otherwise fail fatally, the Rust implementation may keep infallible APIs if that matches project conventions.
  - If there is an existing Rust scanner error type, use it only where allocation or state activation can already report failure.
- Do not invent new recoverable error paths unless required by the existing Rust module interfaces.

## Implementation Phases

## Phase 1: Inspect and Define Minimal Rust Buffer Types
- Review `src/c.c` for the exact buffer layout and state touched by `yy_scan_string` and `yy_scan_bytes`.
- Identify the anonymous C structures actually used by these functions.
- Create or reuse the minimal Rust scanner buffer/state structs in `src/c.rs`.
- Define field mappings for:
  - owned byte storage
  - logical length
  - scanner end markers
  - any ownership or fill-status flags required by the original logic
- Decide whether the Rust function returns a buffer handle, mutates scanner state, or both, based strictly on the C function signatures and call flow.

## Phase 2: Port `yy_scan_bytes` Core Logic
- Implement the byte-oriented scan buffer creation first, since `yy_scan_string` is typically a thin wrapper over it.
- Translate:
  - input length handling
  - allocation sizing
  - copy into owned buffer
  - insertion of trailing scanner sentinel bytes
  - initialization of buffer metadata
  - activation or registration with scanner state
- Replace pointer arithmetic with slice indexing where possible.
- If unsafe code is unavoidable due to existing scanner internals, confine it to the smallest helper and document invariants around buffer ownership and valid indices.

## Phase 3: Port `yy_scan_string` as a Thin Wrapper
- Implement `yy_scan_string` in terms of `yy_scan_bytes` using `&str::as_bytes()`.
- Preserve any C-specific length semantics exactly, including exclusion of Rust’s implicit string terminator concept.
- Ensure the wrapper does not add extra allocation beyond the byte-scan buffer already required by the ported logic.

## Phase 4: Validate Behavior and Remove C-Specific Memory Assumptions
- Add unit tests for empty, simple ASCII, multibyte UTF-8 strings, and raw byte slices containing embedded NULs.
- Verify scanner-visible buffer content includes the required trailing markers and no extra data.
- Confirm ownership/drop behavior replaces any explicit C cleanup assumptions for these allocated scan buffers.
- Check that function signatures and return behavior align with the surrounding Rust scanner port and adjust only for compile-time correctness, not API expansion.