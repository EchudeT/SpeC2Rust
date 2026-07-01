# Implementation Plan: module_src_yy_scan_19

## Summary

This module migration covers the scanner buffer entry points currently implemented in `src/c.c`: `yy_scan_string` and `yy_scan_bytes`. The Rust port should preserve the existing scanner-facing behavior by translating the C buffer construction logic into Rust with explicit ownership and lifetime control, while keeping the implementation narrowly scoped to these two functions and the scanner state they directly require.

The technical approach is to move the relevant scanner buffer management code from the monolithic C source into a Rust module that:
- accepts string and byte input,
- materializes scanner-owned input storage where the original C code relied on copied buffers and sentinel bytes,
- constructs or updates the scanner buffer state needed by the lexer,
- returns scanner buffer handles in a form usable by the rest of the Rust-ported scanner.

The implementation should favor standard-library containers such as `Vec<u8>` and `String`/`&str` where appropriate, with exact handling of trailing sentinel bytes and buffer length bookkeeping to match the C scanner contract. Error handling should remain minimal and behavior-preserving: where the original C code assumes successful allocation, Rust code should make allocation and bounds assumptions explicit but should not introduce unrelated recovery features.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve linear-time buffer copy/setup behavior equivalent to the C implementation.
  - Avoid unnecessary intermediate allocations beyond the scanner-owned byte buffer required by `yy_scan_string` and `yy_scan_bytes`.
  - Keep per-call setup overhead limited to input copy, sentinel insertion, and scanner buffer state construction.
  - Maintain predictable ownership so dropped buffers release memory promptly without manual cleanup paths.

## Module Mapping

### C Source to Rust Source

- **C file**: `src/c.c`
- **Rust target**: `src/module_src_yy_scan_19.rs`

### Function Mapping

- `yy_scan_string`
  - Migrate to a Rust function in `src/module_src_yy_scan_19.rs`
  - Implement as a thin wrapper over the byte-oriented scan entry point where practical
  - Input mapping: C string pointer -> Rust `&str` or `&CStr` only if upstream scanner state requires C-compatible input; otherwise prefer `&str`
  - Responsibility remains limited to creating a scanner-owned scan buffer from string input

- `yy_scan_bytes`
  - Input mapping: C byte pointer plus length -> Rust `&[u8]`
  - Responsibility remains limited to allocating/copied storage, appending scanner sentinel bytes, and producing the scanner buffer state/handle

### Integration Boundary

Because these functions typically depend on scanner buffer types and helper routines generated or embedded in the same C scanner source, the Rust implementation should reuse already-ported scanner state types if they exist on this branch. If not yet present, only the minimal directly referenced buffer/state definitions should be moved alongside these functions into the same Rust module, without introducing additional abstraction layers.

## Data Model

The analysis lists only anonymous C data structures, which is typical for generated scanner internals. The Rust plan should therefore map by role rather than by original tag name.

### Data-structure Mapping

- **C anonymous scanner buffer record**
  - **Rust**: `struct YyBufferState`
  - Fields should include only the members needed by `yy_scan_string` and `yy_scan_bytes`, typically:
    - owned byte storage
    - current buffer size / logical input length
    - current position indices or pointers represented as offsets
    - flags equivalent to ownership/fill/interactivity markers if those fields are directly touched by these functions

- **C anonymous raw byte buffer with trailing sentinels**
  - **Rust**: `Vec<u8>`
  - Store copied input bytes followed by the required two end-of-buffer sentinel bytes
  - Replace pointer arithmetic with index-based access

- **C anonymous scanner/global state carrier**
  - **Rust**: `struct ScannerState` or reuse existing scanner context type
  - Include current buffer selection only if these functions directly install the new buffer into scanner state

- **C anonymous buffer handle pointer**
  - **Rust**: either
    - `usize`/index into owned scanner buffer storage, or
    - `Rc<RefCell<YyBufferState>>` only if existing scanner architecture already uses shared mutable handles, or
    - `Box<YyBufferState>` if direct owned return best matches the migrated API
  - Prefer the simplest ownership model already established by neighboring scanner ports; do not introduce a new handle system solely for elegance

- **C pointer/length pairs**
  - **Rust**: `&[u8]`, `&str`, and `usize`

- **C integer flags and counters**
  - **Rust**: `usize`, `bool`, or small integer types as required by adjacent scanner code
  - Use `usize` for buffer lengths and indices to avoid repeated casts

### Memory Management Decisions

- Replace manual allocation/free with `Vec<u8>` and owned Rust structs.
- Preserve copied-buffer semantics: the created scan buffer must own its contents independently of the caller input.
- Model the two terminating sentinel bytes explicitly in the owned vector to match scanner expectations.
- Avoid raw pointers internally unless required for compatibility with already-ported scanner code; if unavoidable, confine unsafe code to narrow conversion boundaries and document invariants.

### Error Handling Decisions

- Public migration functions should remain behavior-preserving and simple.
- If the surrounding Rust scanner API already returns `Result`, propagate that type.
- Otherwise, keep these functions infallible except for allocation failure panic behavior inherited from `Vec`.
- Validate only the invariants needed to safely mirror C semantics, such as length calculations for sentinel extension.

## Implementation Phases

## Phase 1: Extract and map scanner buffer dependencies

- Review `src/c.c` to identify the exact buffer/state fields touched by `yy_scan_string` and `yy_scan_bytes`.
- Isolate the minimal set of scanner types and helper functions these entry points rely on.
- Create `src/module_src_yy_scan_19.rs` and define the Rust equivalents for:
  - scanner buffer state,
  - owned byte storage,
  - any current-buffer linkage required by these functions.
- Replace C pointer-based field access with Rust-owned fields and index-based bookkeeping.
- Keep all definitions limited to what these two functions require; defer unrelated scanner internals.

## Phase 2: Port `yy_scan_bytes` as the core implementation

- Implement `yy_scan_bytes` first, since it is the lower-level constructor.
- Accept input as `&[u8]` or the project’s established scanner-compatible signature.
- Allocate a `Vec<u8>` with capacity for input length plus two sentinel bytes.
- Copy input bytes exactly once into owned storage.
- Append the required trailing sentinel bytes in the same positions used by the C scanner.
- Construct the Rust buffer state with:
  - logical byte count,
  - owned storage,
  - initial scan position markers,
  - ownership flags equivalent to the C behavior.
- If the C function also switches the active scanner buffer, apply the same state mutation in the Rust scanner context.

## Phase 3: Port `yy_scan_string` as a thin wrapper

- Implement `yy_scan_string` in terms of `yy_scan_bytes` where the original behavior is byte-for-byte equivalent.
- Accept `&str` if the surrounding Rust API is UTF-8-oriented; otherwise accept byte-compatible string input matching the existing ported scanner interface.
- Pass the string bytes and logical length through to the byte scan constructor without introducing extra normalization or encoding logic.
- Ensure null bytes inside input are handled only to the extent supported by the original function contract; do not add new validation behavior.

## Phase 4: Verification and cleanup

- Add focused unit tests under the module’s existing test layout or inline `#[cfg(test)]` tests for:
  - empty input,
  - short ASCII input,
  - arbitrary byte input through `yy_scan_bytes`,
  - correct owned-buffer length including sentinel bytes,
  - independence from caller-owned input after scan buffer creation.
- Compare resulting scanner buffer state against C behavior for the touched fields.
- Remove any temporary compatibility code not required by these two functions.
- Confirm the final file/module mapping stays limited to the migrated functions and their direct data dependencies.