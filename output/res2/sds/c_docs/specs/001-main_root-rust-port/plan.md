# Implementation Plan: main_root Rust Port

## Summary

Port the `sds.c` main module into a single Rust module that preserves the existing dynamic-string behavior and function-level responsibilities from the C implementation without widening scope.

The Rust implementation should replace the C header-variant memory layout with an internal Rust-owned byte buffer plus explicit length/capacity tracking where needed by the migrated API. The technical approach is to map the current SDS operations onto a compact Rust string-bytes type focused on:

- storing arbitrary bytes, not only UTF-8 text,
- preserving efficient append/grow/truncate behavior,
- mirroring the existing function boundaries and migration order from `sds.c`,
- using safe Rust where possible, with any low-level memory-sensitive logic isolated and minimized.

The port should stay centered on the functions listed for `sds.c`, implemented in one Rust source module under standard Cargo layout. The initial shape should prioritize faithful behavior for creation, duplication, free/clear, length updates, capacity growth/shrink, and concatenation/copy operations.

## Technical Context

### Language/Version
- Rust 1.78 or newer

### Primary Dependencies
- Rust standard library only
- No third-party crates are required by the analyzed module inputs

### Testing
- `cargo test`

### Performance Goals
- Preserve amortized append/growth behavior comparable to the C module
- Avoid unnecessary UTF-8 validation by operating on raw bytes
- Keep reallocation frequency low by using capacity-aware growth
- Maintain constant-time length queries
- Keep shrink/remove-free-space behavior explicit rather than automatic, matching the C module’s manual memory behavior as closely as practical in Rust

## Module Mapping

### C to Rust File Mapping
- `sds.c` -> `src/main_root.rs`

### Rust Module Scope
The Rust module should contain:
- the migrated SDS storage type,
- internal helpers corresponding to header-size/type decisions from C,
- public or crate-visible methods/functions matching the listed operations.

No extra submodules are needed for this port.

### Function Mapping
Map each C function into either an inherent method on the Rust SDS type or a small free helper when it represents internal allocation-class logic:

| C Function | Rust Mapping |
|---|---|
| `sdsHdrSize` | private helper for header-class/accounting decisions, or removed if no longer needed by representation |
| `sdsReqType` | private helper for capacity class selection, if retained |
| `sdsnewlen` | constructor from byte slice and explicit length |
| `sdsempty` | empty constructor |
| `sdsnew` | constructor from NUL-terminated-compatible byte/string input without storing terminator |
| `sdsdup` | clone/duplicate method |
| `sdsfree` | implicit via `Drop`; keep compatibility wrapper only if required by calling style |
| `sdsupdatelen` | method to recompute logical length from internal bytes if external mutation remains possible; otherwise adapt to invariant-preserving internal API |
| `sdsclear` | clear method resetting length while preserving allocation |
| `sdsMakeRoomFor` | reserve/grow-capacity method |
| `sdsRemoveFreeSpace` | shrink-to-fit style method |
| `sdsAllocSize` | allocation-size query method |
| `sdsAllocPtr` | internal raw buffer pointer accessor only if strictly needed internally |
| `sdsIncrLen` | method adjusting logical length after manual writes, with bounds checks |
| `sdsgrowzero` | grow-and-zero-fill method |
| `sdscatlen` | append bytes with explicit length |
| `sdscat` | append C-string-like input |
| `sdscatsds` | append from another SDS instance |
| `sdscpylen` | replace contents from bytes with explicit length |
| `sdscpy` | replace contents from string/C-string-like input |

## Data Model

The C module uses several anonymous header layouts to encode string length/allocation metadata. In Rust, these should be represented explicitly rather than reproducing packed header tricks.

### Data Structure Mapping

| C Data Structure | Rust Mapping |
|---|---|
| anonymous SDS header variant 1 | folded into a single `Sds` struct |
| anonymous SDS header variant 2 | folded into a single `Sds` struct |
| anonymous SDS header variant 3 | folded into a single `Sds` struct |
| anonymous SDS header variant 4 | folded into a single `Sds` struct |
| anonymous SDS header variant 5 | folded into a single `Sds` struct |

### Proposed Rust Storage Type

```rust
pub struct Sds {
    buf: Vec<u8>,
}
```

This representation is preferred because:
- `Vec<u8>` already owns allocation and capacity,
- arbitrary byte content is supported,
- append, reserve, truncate, clear, and shrink operations map directly,
- deallocation is automatic and safe.

### Logical Model Decisions
- The visible SDS content is the full `buf[..len]`, with `len == buf.len()`.
- Spare allocation is represented by `Vec<u8>::capacity() - Vec<u8>::len()`.
- No trailing NUL is stored unless a compatibility edge requires temporary conversion for tests or callers.
- Header-type selection functions from C become implementation details for growth/accounting only, and may collapse to simpler helpers if `Vec<u8>` makes them unnecessary.

### Error Handling and Memory Management
- Replace C null-return allocation failure handling with Rust allocation behavior; use `Result` only where an operation can fail for semantic reasons introduced by bounds validation.
- Operations analogous to `sdsIncrLen` must validate resulting length against capacity and current invariants.
- Any method exposing raw pointers should be private and narrowly scoped.
- Avoid `unsafe` unless required for exact low-level mutation steps; if used, isolate it in one helper with clear invariants.

## Implementation Phases

### Phase 1: Establish Core SDS Type and Constructors
Implement the Rust module and migrate creation/destruction-oriented behavior first.

Scope:
- Create `src/main_root.rs`
- Define `Sds { buf: Vec<u8> }`
- Implement:
  - `sdsnewlen`
  - `sdsempty`
  - `sdsnew`
  - `sdsdup`
  - `sdsfree` compatibility behavior if needed
  - `sdsAllocSize`
- Add unit tests for:
  - empty creation
  - creation from bytes
  - creation from text input
  - duplication independence
  - allocation-size reporting basics

Technical decisions:
- Keep interfaces byte-oriented first; add string convenience only where directly replacing existing call patterns.
- Model `sdsfree` as ownership drop; if a direct migrated function is required, make it consume `Sds`.

### Phase 2: Port Length and Capacity Management
Migrate the functions that manage logical length and spare capacity.

Scope:
- Implement:
  - `sdsclear`
  - `sdsupdatelen`
  - `sdsMakeRoomFor`
  - `sdsRemoveFreeSpace`
  - `sdsIncrLen`
  - `sdsgrowzero`
  - `sdsAllocPtr` only if required internally
  - private helpers replacing `sdsHdrSize` and `sdsReqType`
- Add unit tests for:
  - clear preserves capacity
  - reserve increases available room
  - shrink removes spare capacity
  - grow-zero extends with zero bytes
  - incr-len rejects invalid adjustments or preserves invariants
  - update-len behavior after internal byte mutations, if such mutation remains supported

Technical decisions:
- Prefer `Vec::reserve`, `Vec::shrink_to_fit`, `Vec::resize`, and `Vec::set_len` only if a justified internal unsafe path is necessary.
- If `sdsupdatelen` is no longer naturally needed under safe ownership, keep a narrowed version only for compatibility with migrated call sites.

### Phase 3: Port Append and Copy Operations
Migrate all content mutation APIs that build on the capacity primitives.

Scope:
- Implement:
  - `sdscatlen`
  - `sdscat`
  - `sdscatsds`
  - `sdscpylen`
  - `sdscpy`
- Add unit tests for:
  - append bytes
  - append empty input
  - append another SDS
  - overwrite with shorter and longer content
  - capacity reuse across repeated append/copy operations

Technical decisions:
- Use `extend_from_slice` for append behavior.
- Use `clear` + `extend_from_slice` or `resize` patterns for copy behavior.
- Preserve byte-oriented semantics rather than introducing UTF-8 constraints.

### Phase 4: Finalize Compatibility and Cleanup
Tighten the module around the migrated `sds.c` surface and remove C-specific leftovers that are no longer needed by the Rust representation.

Scope:
- Review all function names and visibility against migrated call sites
- Ensure internal helpers are private
- Remove unnecessary header-class emulation if not referenced
- Complete module-level documentation describing representation invariants
- Add targeted regression tests covering mixed operation sequences:
  - create -> reserve -> append -> clear -> append
  - grow-zero -> copy -> shrink
  - duplicate -> mutate original -> verify copy unchanged

Exit criteria:
- All listed `sds.c` functions are either migrated directly or intentionally folded into private Rust helpers with documented justification
- `cargo test` passes
- The port remains confined to the `main_root` module and existing function set only