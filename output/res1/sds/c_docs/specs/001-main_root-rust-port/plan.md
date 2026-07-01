# Implementation Plan

## Summary

Port the `main_root` C module in `sds.c` to a single Rust module that preserves the existing SDS string behavior and function boundaries as closely as practical. The Rust implementation should focus on the current responsibilities only: string creation, duplication, deallocation, length tracking, capacity growth/shrink, zero-growth, append/copy operations, and allocation metadata access.

The technical approach is to replace the C header-prefix memory layout with an internal Rust-owned byte buffer plus explicit length/capacity bookkeeping in a dedicated SDS type. The migration should keep the original function-oriented API semantics through Rust methods and small compatibility-style free functions where needed, while using Rust ownership to eliminate manual frees and invalid raw-pointer arithmetic in normal paths. Any behavior that depends on C allocation classes (`sdsHdrSize`, `sdsReqType`) should be mapped to internal capacity policy logic rather than reproducing packed header layouts byte-for-byte unless a direct test requires that detail.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve amortized append/grow behavior comparable to the C implementation.
  - Avoid unnecessary reallocations by maintaining explicit spare capacity.
  - Keep length updates O(1).
  - Ensure copy/append operations are linear in input size.
  - Minimize zero-fill work to only the newly exposed region in grow-zero operations.

## Module Mapping

- **C source file**: `sds.c`
- **Rust target file**: `src/main_root.rs`

Suggested crate integration:

- `src/main.rs` or `src/lib.rs`
  - `mod main_root;`

Function mapping from C to Rust should remain restrained and centered on existing behavior:

- `sdsHdrSize` -> internal helper in `src/main_root.rs`
- `sdsReqType` -> internal helper in `src/main_root.rs`
- `sdsnewlen` -> constructor/free function in `src/main_root.rs`
- `sdsempty` -> constructor/free function in `src/main_root.rs`
- `sdsnew` -> constructor/free function in `src/main_root.rs`
- `sdsdup` -> clone-style function in `src/main_root.rs`
- `sdsfree` -> ownership-based drop/no-op compatibility function in `src/main_root.rs`
- `sdsupdatelen` -> method/helper in `src/main_root.rs`
- `sdsclear` -> method in `src/main_root.rs`
- `sdsMakeRoomFor` -> reserve/grow method in `src/main_root.rs`
- `sdsRemoveFreeSpace` -> shrink method in `src/main_root.rs`
- `sdsAllocSize` -> method/helper in `src/main_root.rs`
- `sdsAllocPtr` -> internal allocation-access helper only if still needed by tests
- `sdsIncrLen` -> method in `src/main_root.rs`
- `sdsgrowzero` -> method in `src/main_root.rs`
- `sdscatlen` -> append-bytes method in `src/main_root.rs`
- `sdscat` -> append-str/bytes method in `src/main_root.rs`
- `sdscatsds` -> append-from-sds method in `src/main_root.rs`
- `sdscpylen` -> overwrite-copy-bytes method in `src/main_root.rs`
- `sdscpy` -> overwrite-copy-str/bytes method in `src/main_root.rs`

## Data Model

The C file uses multiple anonymous header layouts to encode different length/allocation widths. In Rust, these should be represented by a single owned type with explicit metadata, plus an enum for capacity class when internal logic still needs to mirror C growth decisions.

### C to Rust structure mapping

- **C anonymous SDS header variants** -> `Sds`
  - Rust should use one concrete struct rather than multiple packed header structs.
  - Proposed shape:
    - `buf: Vec<u8>` for owned allocation
    - `len: usize` for logical used length when not relying directly on `Vec::len`
    - optionally `alloc: usize` only if separate logical allocation tracking is required
  - Preferred approach:
    - Store the full allocated buffer in `Vec<u8>`
    - Maintain logical content length separately when spare capacity must remain initialized/managed independently of `Vec::len`

- **C flags/header type encoding** -> `SdsType`
  - Internal enum only:
    - `Type5`
    - `Type8`
    - `Type16`
    - `Type32`
    - `Type64`
  - Used only for helper logic mirroring request/type selection where relevant to tests or allocation policy.

### Rust type direction

Recommended primary type:

```rust
pub struct Sds {
    buf: Vec<u8>,
    len: usize,
}
```

Notes:

- `buf.len()` should represent allocated initialized storage if using zeroed extension to manage spare area, while `len` represents logical string length.
- The active string bytes are `&buf[..len]`.
- A trailing NUL byte is not naturally required by Rust, so include one only if preserving C-style semantics is necessary for compatibility checks inside this module.
- If trailing NUL preservation is required, treat allocation as `len + 1` minimum and keep `buf[len] == 0` after mutating operations.

### Memory management decisions

- Replace `sdsfree` manual deallocation with Rust `Drop` through ownership.
- If a compatibility-style `sdsfree` function is kept, it should consume the `Sds` value and do nothing else.
- Avoid exposing raw pointers except for narrowly scoped internal helpers.
- Any operation analogous to C reallocation should be implemented with `Vec` reserve/shrink operations and explicit post-conditions on logical length and sentinel byte if used.

### Error handling decisions

- C functions typically return `NULL` on allocation failure; Rust should use:
  - `Result<Sds, std::collections::TryReserveError>` for fallible constructors/grow operations if fallible allocation is preserved explicitly, or
  - panic-on-OOM default behavior if the project expects conventional Rust allocation semantics.
- For this migration, prefer restrained explicit fallible signatures only on functions that perform growth/allocation if preserving allocation-failure behavior is important to existing tests.
- Length underflow/overflow cases in `sdsIncrLen`, `sdsgrowzero`, and copy/append operations should be validated and surfaced as `Result` errors rather than unchecked arithmetic.
- Use `checked_add` and bounds assertions for all size computations.

## Implementation Phases

### Phase 1: Establish the Rust SDS core type and constructors

Scope:

- Create `src/main_root.rs`
- Define `Sds`
- Add internal helper enum for requested header/allocation class if needed
- Implement:
  - `sdsHdrSize`
  - `sdsReqType`
  - `sdsnewlen`
  - `sdsempty`
  - `sdsnew`
  - `sdsdup`
  - `sdsfree`

Technical notes:

- Decide early whether the Rust port will preserve an always-present trailing NUL byte.
- Implement constructors so they initialize internal invariants consistently:
  - logical length tracked
  - backing storage allocated correctly
  - sentinel byte maintained if chosen
- Keep helper functions private unless external call sites require direct exposure.
- Add unit tests for empty, from-bytes, from-string, duplicate, and free-by-drop behavior.

Exit criteria:

- A valid `Sds` can be created, cloned, and dropped safely.
- Initial allocation/type-selection logic is covered by tests.
- No raw pointer arithmetic remains in public behavior.

### Phase 2: Implement length/capacity mutation operations

Scope:

- Implement:
  - `sdsupdatelen`
  - `sdsclear`
  - `sdsMakeRoomFor`
  - `sdsRemoveFreeSpace`
  - `sdsAllocSize`
  - `sdsAllocPtr`
  - `sdsIncrLen`
  - `sdsgrowzero`

Technical notes:

- `sdsupdatelen` in C derives length from NUL termination; in Rust, map this to a controlled recomputation rule only if NUL-backed storage is retained. Otherwise, limit it to internal invariant repair from current content conventions.
- `sdsMakeRoomFor` should reserve enough additional capacity without changing logical length.
- `sdsRemoveFreeSpace` should shrink allocation down to the logical minimum needed.
- `sdsIncrLen` must validate negative or positive deltas against current length and reserved capacity semantics.
- `sdsgrowzero` must extend logical length and zero-fill the newly exposed region only.
- `sdsAllocPtr` should remain internal and only exist if migration requires a direct analogue for tests; otherwise omit public exposure.

Exit criteria:

- Length and capacity operations preserve invariants across grow/shrink paths.
- Tests cover empty/non-empty growth, shrink-after-growth, zero extension, clear, and length increment/decrement edge cases.

### Phase 3: Implement append and overwrite-copy operations

Scope:

- Implement:
  - `sdscatlen`
  - `sdscat`
  - `sdscatsds`
  - `sdscpylen`
  - `sdscpy`

Technical notes:

- Reuse `sdsMakeRoomFor` internally rather than duplicating allocation logic.
- Append operations should extend from raw bytes, Rust `&str`, and another `Sds` without changing unrelated spare capacity policy.
- Copy operations should overwrite current contents, growing as needed and updating logical length exactly.
- Maintain trailing NUL invariant after every mutation if chosen in Phase 1.
- Prefer byte-slice based internal implementations, with string helpers delegating to them.

Exit criteria:

- All append/copy functions behave consistently for empty input, self-like data sources where relevant, larger replacements, and repeated growth cycles.
- Tests confirm resulting logical length, stored bytes, spare capacity behavior, and zero/sentinel invariants.

### Phase 4: Stabilization and parity cleanup

Scope:

- Review naming and visibility to match actual module use.
- Remove any unnecessary abstractions introduced during porting.
- Tighten invariants and error handling around arithmetic and capacity transitions.
- Complete parity tests for the full migrated function set in `main_root`.

Technical notes:

- Keep the module single-purpose; do not split into additional utility modules.
- If any helper added during migration is not required by the original file's behavior, inline or remove it.
- Ensure all tests run with `cargo test` only.

Exit criteria:

- `src/main_root.rs` contains the full migrated functionality for the functions listed from `sds.c`.
- Public API surface is limited to what the port actually needs.
- The module is ready on branch `001-main_root-rust-port` with behavior-focused unit coverage.