# Implementation Plan

## Summary

This plan ports `src/wordsplit/wordsplit.c` into a Rust module that preserves the existing module boundary and internal function responsibilities without adding new capabilities.

The C file appears to combine three tightly related concerns:

- initialization of wordsplit state,
- error/context recording,
- internal node allocation and linked-list style manipulation.

The Rust implementation should therefore stay as a single focused module cluster mirroring the original file layout, with direct migration of the listed functions into Rust equivalents. The implementation approach is:

- represent mutable module state with explicit Rust structs,
- replace manual allocation and pointer arithmetic with `Vec`, owned structs, and index-based access where possible,
- convert C error-setting helpers into internal methods on the wordsplit state,
- preserve internal helper layering so call flow remains recognizable during migration,
- keep API and behavior aligned with the original module rather than redesigning it.

Memory safety will come from owned allocations and borrow-checked mutation. Any C patterns that depend on nullable pointers, append/remove node chains, or expandable storage should be mapped to `Option`, `Vec`, and explicit indices or handles. Panic-based allocation failure behavior should only be mirrored where the original helper semantics require immediate abort-like handling; otherwise, fallible allocation should be surfaced through internal `Result` returns.

## Technical Context

### Language/Version

- Rust 1.78+
  This is a conservative stable baseline suitable for standard-library-only migration work.

### Primary Dependencies

- Rust standard library only

Recommended to avoid third-party crates unless later source review reveals a hard dependency requirement not visible in the current module analysis.

Standard library facilities expected:

- `String`
- `Vec`
- `Option`
- `Result`
- `Box` only if direct heap indirection is still needed after struct redesign

### Testing

- `cargo test`

Test scope should cover:

- initialization paths,
- error state updates,
- allocation growth behavior,
- node append/remove behavior,
- character classification behavior for `is_name_char`,
- propagation of sub-error/context state.

### Performance Goals

- Maintain behaviorally comparable performance to the C implementation for normal wordsplit state initialization and node operations.
- Avoid unnecessary heap allocations beyond those implied by the original storage growth logic.
- Preserve amortized efficient append behavior by using `Vec` growth for internal storage.
- Keep node lookup and mutation O(1) where the C implementation relied on pointer/direct-slot access.

## Module Mapping

### Source Mapping

| C Source File | Rust Target |
|---|---|
| `src/wordsplit/wordsplit.c` | `src/wordsplit/wordsplit.rs` |

### Function Mapping

| C Function | Rust Mapping |
|---|---|
| `is_name_char` | private function `is_name_char(...) -> bool` |
| `_wsplt_alloc_die` | private helper or method `alloc_die(...) -> !` or narrow internal error path |
| `_wsplt_seterr` | private method on wordsplit state `set_err(...)` |
| `_wsplt_nomem` | private method/helper `set_nomem(...)` |
| `_wsplt_store_errctx` | private method `store_errctx(...)` |
| `_wsplt_setctxerr` | private method `set_ctx_err(...)` |
| `_wsplt_subsplit` | private method/function `subsplit(...) -> Result<...>` |
| `_wsplt_seterr_sub` | private method `set_err_sub(...)` |
| `wordsplit_init0` | private/init helper `init0(...)` |
| `wordsplit_init` | public or crate-visible `init(...) -> Result<...>` matching existing usage |
| `alloc_space` | private method `alloc_space(...) -> Result<...>` |
| `wsnode_ptr` | private accessor returning index/reference into node storage |
| `wsnode_new` | private method `wsnode_new(...) -> NodeId / index` |
| `wsnode_append` | private method `wsnode_append(...)` |
| `wsnode_remove` | private method `wsnode_remove(...)` |

### Rust Module Placement

Use the existing project structure and keep this module under the wordsplit area only:

- `src/wordsplit/mod.rs` if already present, updated only as needed to expose `wordsplit`
- `src/wordsplit/wordsplit.rs` for the migrated implementation

Do not split this work into additional helper modules unless required by existing project layout.

## Data Model

The analysis lists only anonymous C data structures, so the Rust data model should be derived directly from the concrete structs in `wordsplit.c` during implementation. The migration should preserve shape and ownership, not reinterpret the design.

### Planned Mapping Rules

| C Pattern | Rust Mapping |
|---|---|
| anonymous internal struct carrying wordsplit state | named `struct WordSplit` |
| anonymous node/list record | named `struct WsNode` |
| pointer to optional subordinate split/context | `Option<SubsplitState>` or `Option<Box<...>>` only if self-referential layout requires indirection |
| C string pointer for owned error text/context | `String` |
| borrowed C string input | `&str` where lifetimes are local, otherwise owned `String` if state retains it |
| integer flags/fields | fixed-width integer or `usize`/`u32` as dictated by source semantics |
| raw dynamic array with capacity tracking | `Vec<T>` or explicit `{ items: Vec<T> }` |
| nullable node pointer | `Option<usize>` for index-based storage, or `Option<NodeId>` |
| intrusive linked list pointers | index links (`Option<usize>`) inside `WsNode` if original ordering/removal semantics depend on linkage |

### Expected Core Rust Structures

Because the C structs are anonymous in the analysis, exact field names should be finalized from source review, but the module will likely need only the following named Rust structures:

```rust
struct WordSplit {
    // migrated state fields from the primary C wordsplit structure
}

struct ErrorContext {
    // error code, message/context location, subordinate error details
}

struct WsNode {
    // migrated node payload plus next/prev or equivalent linkage
}
```

If the original file uses multiple anonymous helper structs for storage metadata, convert them into private named Rust structs only when each maps to a real C storage role. Do not invent abstraction layers beyond those needed to replace anonymous C layouts.

### Memory Management Decisions

- Replace manual allocation/free growth with `Vec` reservation and push semantics.
- Replace pointer-returning node access with:
  - stable indices into a node store, or
  - direct mutable references when no aliasing conflict exists.
- Use `Option` instead of null pointers.
- Avoid `unsafe` unless source-level requirements force stable self-references or exact layout dependencies; this module should likely be implementable in safe Rust.
- If the C module assumes node addresses remain stable after growth, prefer:
  - index-based storage over references, or
  - `Vec<Box<WsNode>>` only if source review proves address stability is semantically required.

### Error Handling Decisions

- Convert error setters into mutation of explicit error fields on `WordSplit`.
- Use `Result<T, WordSplitError>` internally for fallible helpers such as allocation and subsplit setup where that simplifies control flow.
- Preserve separate helpers for:
  - generic error assignment,
  - out-of-memory assignment,
  - contextual error storage,
  - subordinate error propagation.
- Keep any externally visible initialization function behavior aligned with the original contract, even if internal implementation uses `Result`.

## Implementation Phases

## Phase 1: Establish Rust Module Skeleton and State Types

### Goals

- Create the Rust file corresponding to `src/wordsplit/wordsplit.c`.
- Identify and name the anonymous C structs.
- Define the Rust state, error, and node types with fields matching the C layouts.
- Port the simplest stateless/helper logic first.

### Tasks

- Add `src/wordsplit/wordsplit.rs`.
- Read `wordsplit.c` and enumerate each anonymous struct into a concrete Rust type.
- Define `WordSplit`, `WsNode`, and any storage/error helper structs required by the original file.
- Port `is_name_char` as a direct private helper.
- Port `wordsplit_init0` as the zero/default-state initializer.
- Decide whether `Default` can be derived or manually implemented for the primary state based strictly on C initialization semantics.

### Exit Criteria

- Module compiles with state types present.
- Initialization baseline exists.
- No behavior added beyond direct structural migration.

## Phase 2: Port Error and Context Management Helpers

### Goals

- Migrate the internal error-setting path exactly enough to support later operational code.
- Centralize stateful error updates on the Rust wordsplit struct.

### Tasks

- Port `_wsplt_seterr`.
- Port `_wsplt_nomem`.
- Port `_wsplt_store_errctx`.
- Port `_wsplt_setctxerr`.
- Port `_wsplt_seterr_sub`.
- Implement any required internal error enum/struct to hold translated error state while preserving original distinctions.
- Review whether `_wsplt_alloc_die` truly represents unconditional termination or only a specialized allocation-failure path; mirror the narrowest correct semantics in Rust.

### Exit Criteria

- Error fields and context fields can be set and overwritten according to original helper flow.
- Internal fallible helpers can report allocation/context failures without placeholder logic.

## Phase 3: Port Storage Allocation and Node Management

### Goals

- Replace C allocation and node pointer mechanics with safe Rust storage.
- Preserve append/remove behavior and access semantics for internal node structures.

### Tasks

- Port `alloc_space`.
- Port `wsnode_ptr`.
- Port `wsnode_new`.
- Port `wsnode_append`.
- Port `wsnode_remove`.
- Choose final storage representation after reviewing C assumptions:
  - `Vec<WsNode>` with indices if relocation is acceptable, or
  - `Vec<Box<WsNode>>` / equivalent stable indirection if node identity must survive reallocation.
- Maintain original ordering and removal semantics exactly.

### Exit Criteria

- Node creation, lookup, append, and removal compile and work through Rust-managed storage.
- No raw-memory management from C remains in these paths.

## Phase 4: Port Initialization Flow and Subsplit Integration, Then Validate

### Goals

- Complete the higher-level initialization and subordinate split behavior.
- Verify parity of migrated helpers through focused tests.

### Tasks

- Port `_wsplt_subsplit`.
- Port `wordsplit_init`.
- Connect initialization with error handling and storage helpers.
- Add unit tests covering:
  - `is_name_char`,
  - empty/default initialization,
  - allocation growth,
  - node append/remove sequences,
  - subordinate error propagation,
  - context-setting behavior.
- Adjust visibility (`pub`, `pub(crate)`, private) to match actual project usage and keep internals encapsulated.

### Exit Criteria

- The module builds and passes `cargo test`.
- All listed C functions have Rust equivalents.
- The Rust code remains confined to the migrated module scope without speculative refactoring.

## Notes and Constraints

- Keep migration scoped to `src/wordsplit/wordsplit.c` only.
- Preserve recognizable function boundaries to ease review against the C source.
- Do not introduce new public APIs unless the original project structure requires them.
- Do not add concurrency features, serialization, FFI layers, or generalized utility modules.
- Prefer safe Rust and explicit ownership over low-level emulation of C pointer behavior.
- Any unavoidable semantic uncertainty from anonymous structs should be resolved by direct field-for-field reading of the source file during implementation, not by redesign.