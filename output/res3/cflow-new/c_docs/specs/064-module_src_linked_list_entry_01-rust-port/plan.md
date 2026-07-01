# Implementation Plan: module_src_linked_list_entry_01

## Summary

This module centers on an intrusive-style linked-list utility used by output and traversal routines in the existing C sources, plus the small set of functions that consume that list while producing symbol, reference, and tree-oriented output. The Rust port should preserve current behavior and call relationships by translating the existing list operations and dependent output functions with minimal structural expansion.

The Rust implementation approach is:

- migrate the linked-list behavior first, keeping ownership explicit and destruction automatic through Rust drop semantics;
- port the output-facing functions in the same module cluster using borrowed access to list contents instead of manual pointer traversal;
- keep function boundaries close to the C originals so migration can proceed file-by-file and behavior can be compared during tests;
- replace C null/error conventions with `Option`, `bool`, and narrow `Result` returns only where I/O or formatting can fail.

The implementation should prefer standard-library collections if they preserve the observable behavior of the current module. Since the identified C code provides append, prepend, unlink, iterate, membership, and size operations, a Rust list wrapper backed by `Vec<T>` is acceptable if callers do not depend on node identity; if unlink semantics require stable element targeting, use a small internal node-based representation limited to this module. The decision should be made by inspecting how `linked_list_unlink` is called in the listed files and keeping the simplest faithful mapping.

## Technical Context

- **Language/Version**: Rust 1.78+ stable
- **Primary Dependencies**:
  - Rust standard library
  - No third-party crates by default
- **Testing**: `cargo test`
- **Performance Goals**:
  - Maintain practical parity with the C implementation for current input sizes
  - Avoid unnecessary cloning during list traversal and output generation
  - Keep append/prepend/unlink costs aligned with the chosen internal representation
  - Preserve linear traversal behavior for list iteration, membership checks, and size reporting unless current call sites naturally allow cached length

## Module Mapping

### Source File Migration

| C File | Rust Target | Notes |
|---|---|---|
| `src/linked-list.c` | `src/linked_list.rs` | Port list container, iteration helpers, membership, size, and unlink behavior first. |
| `src/dot.c` | `src/dot.rs` | Port symbol-printing logic that depends on current output conventions and possibly list-fed traversal results. |
| `src/output.c` | `src/output.rs` | Port printable checks, reference printing, tree printing, and tree-walk related output helpers. |
| `src/main.c` | `src/main.rs` or existing crate entry integration | Only migrate the functions listed from this file if they are defined there; keep the crate entry minimal and aligned with current project layout. |

### Function Mapping

| C Function | Rust Function/Method Target | Migration Note |
|---|---|---|
| `linked_list_append` | `LinkedList::append` | Preserve insertion order and ownership transfer semantics. |
| `linked_list_prepend` | `LinkedList::prepend` | Preserve front insertion semantics. |
| `linked_list_destroy` | `LinkedList` drop / `clear` | Replace explicit free logic with owned storage cleanup. |
| `linked_list_unlink` | `LinkedList::unlink` | Preserve caller-visible removal semantics; define whether removal is by value, handle, or predicate based on C usage. |
| `linked_list_iterate` | `LinkedList::iter` / callback-style helper | Prefer iterator-based traversal; keep callback wrapper only if needed by direct call-site parity. |
| `data_in_list` | `LinkedList::contains_data` or free helper | Keep comparison semantics equivalent to current C equality checks. |
| `linked_list_size` | `LinkedList::len` | Return `usize`; convert at boundaries only if required. |
| `dot_print_symbol` | `dot::print_symbol` | Return `io::Result<()>` if writing to a stream/file. |
| `optfile_register` | `output` or `main` scoped function | Preserve current registration side effects without introducing new config layers. |
| `print_refs` | `output::print_refs` | Keep traversal and formatting order. |
| `is_printable` | `output::is_printable` | Translate to boolean predicate with explicit input borrowing. |
| `is_last` | `output::is_last` | Preserve list/tree position logic exactly. |
| `direct_tree` | `output::direct_tree` | Port recursive/iterative traversal without changing output shape. |
| `inverted_tree` | `output::inverted_tree` | Same as above, keeping traversal order. |
| `tree_output` | `output::tree_output` | Centralize writer-facing tree output while preserving current formatting. |

## Data Model

Because the provided analysis exposes only anonymous C structures, the Rust data model should be established by reading the listed source files and naming types after their role in those files, not by inventing broader abstractions.

### Expected Structure Mapping Strategy

| C Shape | Rust Mapping | Decision Rule |
|---|---|---|
| anonymous linked-list container struct | `struct LinkedList<T>` | Holds owned elements and list state; exact fields depend on unlink requirements. |
| anonymous linked-list node struct | `struct ListNode<T>` only if needed | Introduce only if call sites rely on node-level unlink or stable links. Omit if `Vec<T>` is sufficient. |
| anonymous traversal/output record structs | Named Rust structs matching file-local role | Derive names from actual usage in `dot.c`/`output.c`; keep fields private unless cross-module access is required. |
| C string fields (`char *`, string literals) | `String` or `&str` | Use borrowed `&str` for read-only inputs; owned `String` where C code stores copied text. |
| nullable pointers | `Option<T>` / `Option<&T>` / `Option<NonNull<T>>` | Choose the narrowest safe representation based on ownership. |
| function-pointer iteration callbacks | iterator or closure parameter | Preserve callback order and early-exit behavior if present. |
| integer flags | `bool` or small integer type | Convert to `bool` where only truthiness is used. |

### Linked List Representation Decision

Use one of these two mappings after inspecting `linked_list_unlink` and all list call sites:

1. **Preferred: `Vec<T>`-backed wrapper**
   - Suitable if removal is by data equality or index-like discovery during traversal.
   - Simplifies ownership, drop, and iteration.
   - `append`, `prepend`, `contains`, `len`, and iteration are straightforward.
   - `prepend` is `O(n)`, which is acceptable only if current usage volume is modest and behavior is unaffected.

2. **Fallback: private node-based list**
   - Use if unlink depends on direct node references or pointer identity.
   - Implement with `Option<Box<ListNode<T>>>` and explicit traversal.
   - Keep the node type private to avoid exposing complexity beyond the migrated module.

### Memory Management and Error Handling

- Replace manual allocation/free with owned Rust values.
- Eliminate explicit destroy/free entry points where possible; retain a `clear`-style method only if the original call pattern requires an explicit reset.
- Convert output functions that write to files/streams to `std::io::Result<()>`.
- Keep pure predicates and traversal helpers infallible unless the underlying C logic visibly propagates I/O failures.
- Avoid `unsafe` unless the C behavior cannot be represented safely without materially changing semantics; if used, isolate it inside the list internals and document invariants.

## Implementation Phases

### Phase 1: Establish the list core and type boundaries

- Inspect `src/linked-list.c` and all usages in `src/dot.c`, `src/output.c`, and `src/main.c` to determine whether list removal is value-based or node-based.
- Create `src/linked_list.rs`.
- Port:
  - `linked_list_append`
  - `linked_list_prepend`
  - `linked_list_destroy`
  - `linked_list_unlink`
  - `linked_list_iterate`
  - `data_in_list`
  - `linked_list_size`
- Define the minimal Rust structs required to represent the list and any list element ownership assumptions.
- Add focused unit tests for append/prepend ordering, unlink behavior, membership, iteration order, and size tracking.

### Phase 2: Port symbol and reference output functions

- Create `src/dot.rs` and `src/output.rs`.
- Port:
  - `dot_print_symbol`
  - `print_refs`
  - `is_printable`
  - `is_last`
  - `optfile_register`
- Replace C file/output handling with standard Rust writer-based code where possible (`&mut dyn Write` or concrete writer types already implied by the current code).
- Keep formatting byte-for-byte as close as practical to current output.
- Add tests for formatting-critical functions using fixed input fixtures derived from current behavior.

### Phase 3: Port tree traversal/output logic

- Complete migration of:
  - `direct_tree`
  - `inverted_tree`
  - `tree_output`
- Preserve current traversal order, last-element detection, and indentation/edge formatting.
- Reuse the list and output helpers from earlier phases without introducing new abstraction layers.
- Add tests that validate the emitted tree text for representative small structures and edge cases such as empty/single-element inputs.

### Phase 4: Integrate with crate entry and finalize parity checks

- Wire the migrated modules into `src/main.rs` or the existing crate entry layout with minimal reshaping.
- Remove or isolate any remaining C-style control-flow artifacts that are unnecessary in Rust but keep external behavior unchanged.
- Run `cargo test` and resolve mismatches against the original module behavior.
- Perform a final review for:
  - ownership correctness
  - absence of unnecessary cloning
  - exact unlink/iteration semantics
  - narrow and explicit error propagation only at I/O boundaries