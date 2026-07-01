# Implementation Plan

## Summary

This module cluster covers two tightly related concerns from the C sources:

1. intrusive-style linked-list operations used as shared utility logic
2. output/tree-printing routines that traverse list-backed relationships and emit formatted text

The Rust port should preserve the existing behavior and migration boundaries by translating the current file-level responsibilities into a small set of Rust modules with direct function correspondence. The implementation should prefer safe ownership with `Vec`, `LinkedList`, or slice-based traversal only where it matches current usage, but for a close migration of append/prepend/unlink/iterate semantics, a dedicated list container with explicit node ownership should be used only if the existing call patterns require stable insertion/removal behavior. If the C list is only used as a generic sequence for traversal and membership checks, a `Vec`-backed container is the preferred standard-library choice.

The technical approach is:

- migrate linked-list helpers first because they underpin traversal and membership logic
- represent formerly anonymous C structs with named Rust structs scoped to the destination modules
- convert raw-pointer ownership and manual destruction into RAII-driven drop behavior
- replace null checks and sentinel-style return values with `Option` / `Result` as appropriate
- keep output functions side-effect-oriented and close to the current formatting flow, using `std::io::Write` for emitters where feasible
- avoid adding capabilities beyond the listed functions and current source-file boundaries

## Technical Context

### Language / Version

- Rust 1.78 or newer
- Edition: Rust 2021

### Primary Dependencies

- Standard library only (`std`)
- No third-party crates are recommended from the available evidence

### Testing

- `cargo test`

Testing scope should include:

- append/prepend ordering
- unlink behavior for head, middle, tail, and missing entries
- destroy/drop behavior through ownership-based tests
- iterate and size consistency
- membership checks for `data_in_list`
- output behavior for symbol printing and tree/ref rendering using string-backed writers

### Performance Goals

- Preserve current asymptotic behavior of list operations as closely as practical
- Avoid unnecessary cloning during traversal and output generation
- Keep tree/output routines single-pass where the C code is single-pass
- Maintain predictable memory cleanup without leaks or double frees
- Prefer standard-library containers and borrowing to minimize allocation churn during formatting

## Module Mapping

### Source File Mapping

| C File | Rust File | Notes |
|---|---|---|
| `src/linked-list.c` | `src/linked_list.rs` | Direct migration target for list container logic and helper functions |
| `src/dot.c` | `src/dot.rs` | Direct migration target for dot/symbol output routines |
| `src/output.c` | `src/output.rs` | Direct migration target for refs/tree/printability helpers |
| `src/main.c` | `src/main.rs` or `src/lib.rs` + `src/main.rs` | Keep only the current entry-point wiring and module declarations needed by migrated functions |

### Function Mapping

| C Function | Rust Target | Migration Notes |
|---|---|---|
| `linked_list_append` | `linked_list::append` | Mutating method on list struct |
| `linked_list_prepend` | `linked_list::prepend` | Mutating method on list struct |
| `linked_list_destroy` | drop semantics / `linked_list::clear` if needed | Prefer ownership-based cleanup; add explicit `clear` only if call sites require it |
| `linked_list_unlink` | `linked_list::unlink` | Return removed element as `Option<T>` or removal flag depending on call usage |
| `linked_list_iterate` | `linked_list::iter` / callback adapter | Prefer iterator exposure; add callback-style wrapper only where required by migrated callers |
| `data_in_list` | `linked_list::contains_data` or free helper | Exact equality semantics must match C usage |
| `linked_list_size` | `linked_list::len` | Simple accessor |
| `dot_print_symbol` | `dot::print_symbol` | Output helper using writer abstraction |
| `optfile_register` | `output` or `main` scoped function | Place according to current call ownership; do not broaden responsibility |
| `print_refs` | `output::print_refs` | Preserve traversal order and formatting |
| `is_printable` | `output::is_printable` | Pure helper |
| `is_last` | `output::is_last` | Pure helper tied to traversal context |
| `direct_tree` | `output::direct_tree` | Tree rendering routine |
| `inverted_tree` | `output::inverted_tree` | Tree rendering routine |
| `tree_output` | `output::tree_output` | Shared dispatch/output entry |

## Data Model

Because the analysis only exposes anonymous C structures, the Rust port should introduce named types only where needed to support the listed functions. Naming should follow the source file that owns the data.

### Data-Structure Mapping Strategy

| C Representation | Rust Representation | Notes |
|---|---|---|
| anonymous linked-list node | `struct ListNode<T>` | Only introduce if node-level unlink semantics require explicit nodes |
| anonymous linked-list container | `struct LinkedList<T>` | Primary owner of list state |
| anonymous traversal cursor / iterator state | `struct` or standard iterator type | Prefer standard iterator types when possible |
| anonymous output context | file-local `struct` in `output.rs` if required | Only if multiple output functions share state |
| anonymous dot/output symbol record | file-local `struct` or borrowed existing project type | Use borrowed references where possible |
| anonymous tree/ref temporary state | local structs/enums in `output.rs` | Keep private and minimal |

### Recommended Rust Shapes

#### Linked list layer

Use one of the following, chosen after inspecting actual C call patterns during port:

1. **Preferred:** `Vec<T>`-backed wrapper
   - best if operations are mostly append, prepend, iterate, size, contains, and occasional unlink by equality
   - simplest ownership model
   - least migration risk in safe Rust if stable node identity is not required

2. **Fallback:** custom owned node chain
   - use `Option<Box<ListNode<T>>>`
   - appropriate only if unlink behavior depends on node-style structure or insertion/removal semantics that are awkward with `Vec`
   - requires careful mutable traversal but remains fully safe

Suggested initial public shape:

```rust
pub struct LinkedList<T> {
    items: Vec<T>,
}
```

or, if needed:

```rust
pub struct LinkedList<T> {
    head: Option<Box<ListNode<T>>>,
    len: usize,
}

struct ListNode<T> {
    data: T,
    next: Option<Box<ListNode<T>>>,
}
```

### Ownership and Memory Management

- C manual destruction maps to Rust ownership and automatic drop
- explicit destroy functions should become no-op wrappers around clearing owned containers only if required by migrated call sites
- no raw pointer ownership should be preserved unless unavoidable for compatibility inside this branch
- equality-based membership/unlink should be implemented through `PartialEq` bounds or explicit comparator logic extracted from current call sites

### Error Handling Mapping

- null returns -> `Option`
- status/int error codes -> `Result` only when the C function actually signals errors that callers inspect
- pure formatting predicates remain boolean
- output routines should return `std::io::Result<()>` if they write to a generic writer; otherwise preserve infallible string-building where applicable

## Implementation Phases

## Phase 1: Port the list core from `src/linked-list.c`

### Goals

- establish the Rust data structure used by the rest of the module cluster
- migrate all list operations before touching output traversal code

### Tasks

- create `src/linked_list.rs`
- implement the Rust list container and any private node type needed
- port:
  - `linked_list_append`
  - `linked_list_prepend`
  - `linked_list_unlink`
  - `linked_list_iterate`
  - `data_in_list`
  - `linked_list_size`
- replace `linked_list_destroy` with ownership-driven cleanup and add an explicit `clear`/compatibility method only if the translated callers still invoke destroy explicitly
- add unit tests for insertion order, removal cases, iteration order, membership, and length tracking

### Technical Decisions

- keep interfaces narrow and shaped around existing call patterns
- do not generalize the container beyond the functions already present
- prefer borrowed iteration (`iter()`) and adapt callback-based C traversal only at call sites if possible

## Phase 2: Port output helpers from `src/dot.c` and `src/output.c`

### Goals

- migrate formatting and tree/ref traversal onto the Rust list/data model
- keep emitted text and traversal order aligned with the C behavior

### Tasks

- create `src/dot.rs` for `dot_print_symbol`
- create `src/output.rs`
- port:
  - `print_refs`
  - `is_printable`
  - `is_last`
  - `direct_tree`
  - `inverted_tree`
  - `tree_output`
  - `optfile_register` in the module where its current responsibility best fits
- define minimal private structs/enums for any formerly anonymous formatting state encountered during migration
- route all writing through `String` or `std::io::Write` according to the original destination of the C output functions
- add tests using expected output strings for representative traversal cases

### Technical Decisions

- preserve helper decomposition from the C files rather than merging functions
- use borrowing for symbol/tree/reference inputs to avoid copying
- make formatting helpers private unless they are called across Rust modules exactly as the C sources were split

## Phase 3: Integrate with `src/main.c` responsibilities and align call sites

### Goals

- wire migrated modules into the Rust crate structure
- replace remaining C-style lifecycle assumptions with Rust ownership and return types

### Tasks

- create module declarations and entry-point wiring in `src/main.rs` and/or `src/lib.rs`
- port only the portions of `src/main.c` needed to connect the migrated functions in this module cluster
- update call sites to use:
  - `Option` instead of null checks
  - return values from unlink/contains/size helpers
  - borrowed iteration instead of callback traversal where safe and local
- ensure any explicit destroy calls become harmless `clear` calls or are removed where ownership already handles cleanup
- add integration-oriented tests covering list-backed traversal reaching output routines

### Technical Decisions

- keep `main`-level changes limited to module registration and invocation already present in the C source
- do not redesign program flow or command handling beyond what is necessary to compile and preserve behavior

## Phase 4: Verification and parity cleanup

### Goals

- confirm behavior parity for migrated functions
- remove residual C-oriented patterns that are no longer needed after safe ownership conversion

### Tasks

- review edge cases in unlink and traversal for off-by-one or last-element logic
- verify output parity for:
  - printable filtering
  - last-element detection
  - direct vs inverted tree modes
  - reference printing order
- audit for unnecessary clones, interior mutability, or boxed state introduced during translation
- normalize visibility and module boundaries to match the original file ownership
- run `cargo test` until the ported module cluster is stable

### Exit Criteria

- all listed functions are represented in the Rust branch
- linked-list memory management relies on Rust ownership rather than manual free logic
- output routines compile and produce stable expected strings in tests
- no extra modules or unevidenced support systems were introduced