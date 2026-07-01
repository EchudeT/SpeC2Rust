# Implementation Plan

## Summary

Port `xmalloc.c` into a single Rust module that preserves the current utility scope: allocation and duplication helpers corresponding to `xcalloc`, `xicalloc`, `xmemdup`, `ximemdup`, `ximemdup0`, and `xstrdup`.

The Rust implementation should favor standard library allocation and ownership types instead of reproducing C-style raw allocation patterns. The main technical approach is:

- Replace heap allocation helpers with safe constructors over `Vec<u8>`, boxed slices, and `String`/`CString`-adjacent representations only where needed by call sites.
- Preserve the semantics of “allocate-or-fail” helpers by making failure behavior explicit in Rust:
  - use checked size arithmetic before allocation,
  - use infallible ownership conversions where practical,
  - panic only where the C helper would have terminated rather than returned an error.
- Keep the port narrowly scoped to this file and its exported helper functions, without introducing broader memory utility layers.

This module should remain a small internal utility module in the `main_cluster` migration and should be implemented in a way that supports direct replacement of current call sites as they are ported.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**:
  - Rust standard library
  - No third-party crates recommended at this stage
- **Testing**: `cargo test`
- **Performance Goals**:
  - Match the effective asymptotic cost of the C helpers for zeroed allocation and memory duplication
  - Avoid unnecessary intermediate buffers or copies
  - Use contiguous owned buffers (`Vec<u8>`, `Box<[u8]>`, `String`) for predictable allocation behavior
  - Ensure overflow checks occur before allocation

## Module Mapping

| C File | Rust Module/File | Notes |
|---|---|---|
| `xmalloc.c` | `src/main_cluster/main_root_xmalloc.rs` | Direct port target for allocation/duplication helpers |

If the project already uses a different existing module layout for `main_cluster`, place the Rust file into that existing structure rather than adding new architectural layers. The port should stay as one Rust module corresponding to this C file.

## Data Model

This C module does not define custom structs. The migration is therefore focused on pointer-and-buffer semantics.

| C Concept | Rust Mapping | Notes |
|---|---|---|
| `void *` allocated buffer | `Vec<u8>` or `Box<[u8]>` | Prefer `Vec<u8>` during construction; convert to boxed slice if fixed-size ownership is a better call-site fit |
| `char *` duplicated string | `String` | Use when source data is valid text and current call sites are string-oriented |
| NUL-terminated duplicated byte string | `Vec<u8>` | Use for `ximemdup0`-style behavior when an explicit trailing `0` byte must be preserved |
| size/count arguments | `usize` | Convert from C integer types with explicit checked conversion where needed |
| signed integer size/count inputs | `isize` or dedicated checked input handling | For `xicalloc` / `ximemdup` parity, reject negative values before conversion |

## Implementation Phases

### Phase 1: Create the Rust module skeleton and define function signatures

- Add the Rust file for the module in the existing `main_cluster` source tree.
- Establish narrow helper APIs mirroring the C function set:
  - `xcalloc`
  - `xicalloc`
  - `xmemdup`
  - `ximemdup`
  - `ximemdup0`
  - `xstrdup`
- Decide per function on the Rust return type based on actual source semantics:
  - raw memory duplication helpers should return owned byte storage
  - string duplication should return `String` when the source is textual
- Centralize checked multiplication/addition needed for:
  - element count × element size
  - duplicate length + optional trailing NUL byte
- Keep failure handling consistent across the module so that allocation overflow and invalid signed-size inputs do not silently wrap.

### Phase 2: Port allocation and duplication behavior

- Implement `xcalloc` with:
  - checked `count * size`
  - zero-initialized allocation via `vec![0; total]`
- Implement `xicalloc` as the signed-input variant:
  - validate non-negative inputs
  - convert to `usize`
  - delegate to the same checked allocation path
- Implement `xmemdup`:
  - allocate exact-size owned storage
  - copy source bytes without extra capacity requirements
- Implement `ximemdup`:
  - validate signed length input
  - convert and delegate to the unsigned duplication path
- Implement `ximemdup0`:
  - allocate `len + 1` with checked addition
  - copy payload bytes
  - append a single trailing zero byte
- Implement `xstrdup`:
  - duplicate textual input into owned Rust string storage
  - if the source representation in the port remains byte-oriented, keep this helper narrowly adapted to the calling convention already used by the migrated code

### Phase 3: Align error behavior and ownership with C expectations

- Review each function for places where C would have aborted on allocation failure or invalid size calculations.
- In Rust, use:
  - explicit overflow checks before allocation
  - clear panic paths only for invariants that are treated as fatal by the original helper layer
- Ensure no returned buffer aliases the source memory.
- Ensure zero-length cases behave predictably:
  - zeroed allocation returns an empty owned buffer
  - zero-length duplication returns empty storage
  - `ximemdup0` on zero length still returns a one-byte buffer containing only `0`
- Keep ownership fully safe and automatic; do not introduce manual deallocation logic.

### Phase 4: Add focused tests and integrate with migrated call sites

- Add unit tests covering:
  - zero-sized allocation
  - normal zeroed allocation contents
  - overflow detection in multiplication/addition
  - negative-size rejection for signed-input variants
  - exact byte preservation for `xmemdup` / `ximemdup`
  - trailing NUL behavior for `ximemdup0`
  - string duplication independence for `xstrdup`
- Use `cargo test` only; do not add benchmark or property-test infrastructure.
- Adjust immediate migrated call sites, if any, to consume the Rust-owned return types directly instead of emulating C pointer ownership.
- Confirm the module remains limited to the original file’s responsibilities and does not grow into a general allocator abstraction.