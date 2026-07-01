# Implementation Plan: module_src_yy_scan_19

## Summary

This module migration covers the C scanner buffer entry points `yy_scan_string` and `yy_scan_bytes` currently located in `src/c.c`. The Rust implementation should preserve the existing scanner-facing behavior by porting only the buffer construction and scan initialization logic required by these two functions.

The implementation approach is to translate the C buffer-handling flow into Rust with explicit ownership over temporary scan input storage. Since these functions accept external byte or string input and prepare it for scanner consumption, the Rust port should model the original buffer state explicitly, including any end-of-buffer sentinel handling and lifetime ownership that was implicit in C allocation patterns.

The plan should keep the port tightly scoped:
- migrate the logic of `yy_scan_string`
- migrate the logic of `yy_scan_bytes`
- migrate only the data structures and helper state directly required by those functions
- keep behavior aligned with the existing scanner contract rather than redesigning scanner internals

## Technical Context

- **Language/Version:** Rust 1.82 stable
- **Primary Dependencies:** Rust standard library only
- **Testing:** `cargo test`
- **Performance Goals:**
  - Maintain linear-time copying/setup cost relative to input length, matching the C implementation profile for byte-buffer preparation.
  - Avoid unnecessary reallocations beyond the single owned buffer required to represent scanner input safely in Rust.
  - Preserve scanner initialization behavior without introducing extra abstraction layers in the hot path.

## Module Mapping

### C to Rust File Mapping

- `src/c.c`
  - migrate the scanner buffer functions related to this module into a Rust source file under standard project layout
  - preferred target: `src/module_src_yy_scan_19.rs`
  - if scanner code is already being consolidated elsewhere in the Rust port, place these functions in the existing scanner module instead of creating additional layers

### Function Mapping

- `yy_scan_string`
  - Rust function with equivalent behavior, likely taking `&str` and delegating to the byte-oriented implementation
  - should append or otherwise ensure the scanner-required terminal sentinel layout in owned storage

- `yy_scan_bytes`
  - Rust function taking byte input (`&[u8]`)
  - should allocate owned scan storage, copy source bytes, append required end markers, and initialize the scanner buffer state

### Scope Boundary

Only migrate the code paths and state needed for:
- constructing a scan buffer from a Rust string/byte slice
- registering that buffer with the scanner state expected by the ported lexer

Do not expand into unrelated scanner API surface unless a direct dependency is discovered during migration.

## Data Model

The source analysis lists only anonymous C data structures, which is typical for generated scanner internals and local structural typedefs. Because names are unavailable, the Rust port should introduce only the minimum explicit types needed to support `yy_scan_string` and `yy_scan_bytes`.

### Data Structure Mapping Strategy

- **anonymous scanner buffer structure**
  - map to a named Rust struct, e.g. `ScannerBuffer`
  - responsibility:
    - owned input storage (`Vec<u8>`)
    - current position/index fields if required by the migrated scanner logic
    - buffer size / character count fields if present in the C path
    - flags corresponding to ownership or buffer status when those flags affect behavior

- **anonymous scanner state structure**
  - map to a named Rust struct, e.g. `ScannerState`
  - include only fields directly read or updated by the migrated functions
  - if the wider port already defines scanner state, extend that existing struct rather than duplicating it

- **anonymous temporary/local record types**
  - replace with:
    - local variables where structure is not needed across calls
    - small private Rust structs only if field grouping is required for clarity or borrow management

### Suggested Rust Representations

- C character buffer used for scan input
  - `Vec<u8>` in Rust
  - ensure room for the same trailing sentinel bytes expected by the scanner logic

- C pointer/length pairs
  - `&[u8]` or `&str` at the API boundary
  - converted into owned `Vec<u8>` for scanner-managed lifetime

- C integer length/count fields
  - `usize` for lengths and indices
  - only use fixed-width integers if required by interaction with already-ported scanner code

- C ownership flags / boolean markers
  - `bool` or a small private enum if multiple states exist in the original logic

### Memory Management Decisions

- Replace C heap allocation with `Vec<u8>` ownership.
- The scan buffer object should own the copied bytes for as long as the scanner may read them.
- Avoid borrowing caller memory into scanner state unless the existing Rust scanner design already safely supports it.
- Preserve any C behavior that depends on extra end-of-buffer bytes by explicitly appending them during buffer creation.

### Error Handling Decisions

If the original functions assume infallible allocation and return raw pointers, the Rust port should adapt this to the surrounding project conventions:
- prefer returning a concrete buffer/state handle directly if the surrounding scanner API is infallible
- otherwise return `Result<_, ScanError>` only if the existing Rust port already models construction failure
- do not invent new recovery behavior; allocation failure may remain implicit/panic-driven if that matches the project-wide approach

## Implementation Phases

## Phase 1: Isolate scanner buffer dependencies

- Inspect `src/c.c` and identify the exact fields, helper routines, and buffer/state mutations used by `yy_scan_string` and `yy_scan_bytes`.
- Determine whether these functions depend on:
  - an existing buffer-state type
  - a current-buffer switch routine
  - end-of-buffer marker conventions
  - scanner-global versus passed-in state
- Create or reuse the minimal Rust structs required to represent that state.
- Define the Rust module/file placement according to the existing port layout, without adding extra abstraction modules.

## Phase 2: Port `yy_scan_bytes` as the base implementation

- Implement the byte-oriented function first, since it is the lower-level constructor.
- Translate the C logic into Rust in the same operational order:
  - validate or normalize the byte length assumptions used by the original code
  - allocate owned buffer storage
  - copy source bytes
  - append scanner-required sentinel bytes
  - construct/update the scanner buffer state
  - hand the buffer to the scanner in the same state the C code establishes
- Preserve observable behavior around empty input and boundary sizes.
- Keep helper extraction minimal; only factor out code that is directly shared with `yy_scan_string`.

## Phase 3: Port `yy_scan_string` on top of the byte path

- Implement the string-oriented wrapper using Rust `&str`.
- Delegate to the byte-based implementation using `as_bytes()`.
- Preserve the same effective handling as the C function for null-terminated source semantics, without introducing C-string APIs unless the surrounding port already requires them.
- Confirm that the resulting scanner buffer layout is identical to the byte-path expectations.

## Phase 4: Validate behavior and complete cleanup

- Add focused unit tests covering:
  - scanning from a normal string
  - scanning from arbitrary bytes
  - empty input
  - inputs containing interior `\0` bytes for the byte-oriented function, if the C behavior allows it
  - correct terminal sentinel placement and retained ownership lifetime
- Remove or avoid unused placeholders introduced during migration.
- Ensure the final Rust code compiles cleanly and that `cargo test` passes for the migrated module and any dependent scanner tests.