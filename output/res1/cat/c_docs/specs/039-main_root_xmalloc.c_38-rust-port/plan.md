# Implementation Plan: main_root_xmalloc.c_38

## Summary

This module ports the allocation-helper logic from `xmalloc.c` into a focused Rust module that preserves the original call surface and intent: duplication and zero-initialized allocation helpers for byte buffers and strings.

The Rust implementation should favor safe standard-library abstractions over direct manual allocation. The C functions in this module primarily create newly owned memory regions from requested sizes or existing memory/string inputs. In Rust, these behaviors map naturally to `Vec<u8>`, boxed slices, and `String`/`&str` duplication. The implementation should keep the migration narrow: port only the existing helper functions and adapt callers to the Rust return types used in the project.

Because the C variants typically assume infallible allocation or process-terminating behavior on allocation failure, the Rust port should use the standard allocation behavior of owned containers, which aborts on OOM consistently with Rust’s default runtime behavior. For size computations and buffer construction, explicit checked arithmetic should be used where multiplication or length-plus-terminator logic is involved, so integer overflow does not silently produce undersized allocations.

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**:
  - Rust standard library only
- **Testing**:
  - `cargo test`
- **Performance Goals**:
  - Match C module behavior with no unnecessary extra copies beyond what ownership requires
  - Use contiguous standard-library buffers (`Vec<u8>`, `String`, boxed slices where appropriate)
  - Preserve linear-time duplication behavior
  - Avoid per-element initialization patterns beyond what zeroed allocation semantics require

## Module Mapping

### C to Rust File Mapping

- `xmalloc.c` → `src/main_root_xmalloc.rs`

If the crate already uses a different root module layout, place the implementation in the existing module tree and expose only the functions needed by migrated callers. Do not create additional helper submodules unless required by the current project structure.

### Function Mapping

- `xcalloc` → Rust function allocating a zero-filled owned byte buffer sized by `count * size`
- `xicalloc` → Rust function allocating a zero-filled owned byte buffer sized by integer count/size variant used by existing callers
- `xmemdup` → Rust function duplicating a byte region into owned memory
- `ximemdup` → Rust function duplicating a byte region using the integer-sized variant expected by callers
- `ximemdup0` → Rust function duplicating a byte region and appending a trailing zero byte
- `xstrdup` → Rust function cloning a string into owned Rust string storage

### Rust API Shape

Use Rust signatures that reflect actual ownership and safety:

- Raw memory duplication in C should become slice-based input where possible:
  - `&[u8] -> Vec<u8>` or `Box<[u8]>`
- Zero-initialized allocation should become:
  - size/count inputs -> `Vec<u8>` of computed length
- String duplication should become:
  - `&str -> String`

If existing migrated code still operates on byte-oriented buffers rather than UTF-8 text, keep `xstrdup` narrowly aligned to the actual call sites during integration, but prefer `String` when inputs are validated Rust strings.

## Data Model

This module has no standalone C structs to port.

### Data-Structure Mapping

- C raw allocated memory (`void *`, byte buffers) → `Vec<u8>` or `Box<[u8]>`
- C NUL-terminated string copies (`char *`) → `String` when UTF-8 is already guaranteed by surrounding Rust code
- C size values (`size_t`, integer length variants) → `usize`
- C duplicated memory with explicit trailing zero → `Vec<u8>` containing copied bytes plus one final `0`

### Memory Management Notes

- C heap ownership transfers become Rust owned values with automatic drop.
- No manual free-equivalent logic should be introduced in this module.
- All size arithmetic must use checked multiplication/addition before allocation.
- For `ximemdup0`, explicitly reserve and push a trailing zero byte after copying, or allocate exact final length and write into it safely.

## Implementation Phases

### Phase 1: Create the Rust module skeleton and define allocation semantics

- Add `src/main_root_xmalloc.rs` in the current crate layout.
- Define the Rust equivalents for:
  - `xcalloc`
  - `xicalloc`
  - `xmemdup`
  - `ximemdup`
  - `ximemdup0`
  - `xstrdup`
- Select the owned return types based on current caller expectations, preferring:
  - `Vec<u8>` for memory buffers
  - `String` for string duplication
- Implement checked size calculations for count/size multiplication and terminator extension.
- Keep the implementation entirely within safe Rust where possible.

### Phase 2: Port function bodies and align call sites

- Implement zero-filled allocation with `vec![0; len]` after validated length computation.
- Implement byte duplication with slice copying methods (`to_vec`, `extend_from_slice`).
- Implement zero-terminated duplication by copying the source bytes and appending a single `0`.
- Implement string duplication with `to_owned()`/`to_string()`.
- Update existing users of `xmalloc.c` helpers to consume Rust owned types instead of raw pointers.
- Keep the migration limited to the direct consumers required for this module port; do not introduce broader allocation abstractions.

### Phase 3: Error-handling review and boundary-condition tests

- Confirm all conversions to `usize` and all length arithmetic are explicit and checked.
- Add unit tests covering:
  - zero-length allocation
  - normal `count * size` allocation
  - multiplication overflow handling path
  - empty and non-empty byte duplication
  - `ximemdup0` producing an added trailing zero
  - string duplication of empty and non-empty strings
- Ensure tests validate exact lengths and content rather than internal allocation details.

### Phase 4: Final integration cleanup

- Remove or stop referencing the old C implementation in the migrated Rust build path.
- Verify module visibility matches only what current crate consumers need.
- Run `cargo test` and fix any ownership/signature mismatches exposed during integration.
- Keep the final module focused on the original helper set without adding new utility layers.