# Implementation Plan: module_src_linked_list_entry_01

## Summary

This module covers the migration of the C linked-list and output-related entry points found in `src/linked-list.c`, `src/dot.c`, `src/output.c`, and the related integration points in `src/main.c`. The Rust implementation should preserve the current behavior and call structure while replacing manual memory management and pointer-based list traversal with standard-library ownership and borrowing.

The technical approach is to port the linked-list utilities first, then migrate the output and tree-printing functions onto Rust data structures that expose equivalent iteration and membership behavior. The implementation should remain narrow: reproduce the existing file/function responsibilities, keep output generation logic close to the original source split, and avoid introducing extra abstraction layers beyond what is required for safe ownership, mutation, and iteration.

## Technical Context

### Language/Version

- Rust 1.78 or newer
- Edition: 2021

### Primary Dependencies

Use the Rust standard library by default.

Recommended crates:
- None required for this module based on the provided input

### Testing

- `cargo test`

Testing focus:
- linked-list insertion order for append/prepend
- unlink behavior for head, middle, tail, and missing entries
- destroy behavior via ownership drop semantics
- iteration order and size accounting
- membership checks corresponding to `data_in_list`
- output helpers such as printable filtering and last-element detection
- tree and reference output stability for representative fixtures

### Performance Goals

- Preserve linear-time traversal characteristics where the C implementation traverses lists
- Avoid unnecessary cloning of node payloads during append, prepend, unlink, and iteration
- Keep output construction efficient by writing to `String` or generic writers rather than repeated temporary allocations where possible
- Maintain predictable memory behavior through ownership-based cleanup instead of manual free chains

## Module Mapping

### Source File Mapping

| C File | Rust File | Notes |
|---|---|---|
| `src/linked-list.c` | `src/linked_list.rs` | Port list operations and list-query helpers directly into one Rust module |
| `src/dot.c` | `src/dot.rs` | Port symbol/output formatting logic related to dot output |
| `src/output.c` | `src/output.rs` | Port printable checks, tree/ref printing, and output traversal helpers |
| `src/main.c` | `src/main.rs` or existing integration module | Reconnect call sites to Rust module APIs without expanding responsibilities |

### Function Mapping

| C Function | Rust Target | Notes |
|---|---|---|
| `linked_list_append` | `linked_list::append` | Preserve insertion-at-tail semantics |
| `linked_list_prepend` | `linked_list::prepend` | Preserve insertion-at-head semantics |
| `linked_list_destroy` | ownership drop / `linked_list::clear` if needed | Prefer implicit drop; only add explicit clear if existing call sites require it |
| `linked_list_unlink` | `linked_list::unlink` | Return removed element or status instead of raw pointer mutation |
| `linked_list_iterate` | `linked_list::iter` / iterator method | Replace callback-style traversal with iterator-based traversal where call sites permit |
| `data_in_list` | `linked_list::contains_data` | Use equality/identity rules matching original payload behavior |
| `linked_list_size` | `linked_list::len` | Constant-time if length is stored, otherwise linear only if required by shape |
| `dot_print_symbol` | `dot::print_symbol` | Port escaping/formatting behavior exactly |
| `optfile_register` | `output` or `main` integration helper | Keep near current registration/integration logic; do not generalize |
| `print_refs` | `output::print_refs` | Preserve output ordering and filtering |
| `is_printable` | `output::is_printable` | Keep as small helper with equivalent predicate behavior |
| `is_last` | `output::is_last` | Adapt to iterator/index-based Rust traversal |
| `direct_tree` | `output::direct_tree` | Preserve traversal/output structure |
| `inverted_tree` | `output::inverted_tree` | Preserve traversal/output structure |
| `tree_output` | `output::tree_output` | Main tree rendering entry point |

## Data Model

The source analysis only reports anonymous C data structures, so the Rust plan should derive data models from actual field usage during porting rather than inventing broader replacements.

### Data-Structure Mapping Strategy

| C Structure | Rust Representation | Notes |
|---|---|---|
| anonymous linked-list node | `struct LinkedListNode<T>` only if a node-level type is truly required | Use private node type if preserving node-chain structure is necessary |
| anonymous linked-list container | `struct LinkedList<T>` | Owns nodes or wraps `Vec<T>` depending on observed unlink/iteration semantics |
| anonymous output/tree traversal state | Small module-private `struct` values only if current C state needs bundling | Avoid creating new exported types unless required by migrated function signatures |
| anonymous symbol/reference payloads | Named Rust structs/enums inferred from field usage at migration time | Keep names aligned to their originating C usage, not generalized domain models |

### Preferred Rust Container Choice

The first implementation decision should be driven by how `linked_list_unlink`, `linked_list_iterate`, and payload identity are used by the dependent functions:

- Use `Vec<T>` internally if:
  - list elements are accessed sequentially,
  - unlink is value-based or index-locatable,
  - no external stable node pointers are required.

- Use an owned node chain such as:
  - `Option<Box<LinkedListNode<T>>>`

  if:
  - removal depends on preserving list-node behavior,
  - the C implementation mutates next pointers directly,
  - call sites rely on head/tail list semantics that would become awkward with `Vec<T>`.

Given the function set, a small custom `LinkedList<T>` backed by owned nodes is the safer default for behavioral parity, but this should remain private to the module and not expand beyond the current API surface.

### Memory Management Mapping

- C manual allocation/free -> Rust ownership and `Drop`
- C nullable pointers -> `Option<T>` / `Option<Box<Node>>`
- C out-parameters and mutation through pointers -> `&mut` references and return values
- C raw traversal pointers -> iterator methods or scoped mutable traversal
- C sentinel/error integer returns -> `bool`, `Option<T>`, or `Result<T, E>` depending on whether failure is expected or exceptional

### Error Handling

- Use `Option` for not-found cases such as unlinking a missing element
- Use `Result` only for operations that can genuinely fail, primarily output/write paths if they are adapted to `fmt::Write` or `io::Write`
- Keep helper predicates total and side-effect free
- Do not add custom error frameworks unless actual write-path needs require a small local error type

## Implementation Phases

### Phase 1: Port Core Linked List Operations

Scope:
- Migrate `linked_list_append`
- Migrate `linked_list_prepend`
- Migrate `linked_list_unlink`
- Migrate `linked_list_destroy`
- Migrate `linked_list_iterate`
- Migrate `data_in_list`
- Migrate `linked_list_size`

Technical work:
- Create `src/linked_list.rs`
- Define the minimum `LinkedList<T>` representation needed by current call sites
- Implement append/prepend/unlink with ownership-safe mutation
- Replace explicit destroy logic with drop semantics, adding `clear` only if call sites need explicit reset
- Expose iteration in a form that lets the output code be migrated without callback emulation unless necessary
- Add focused unit tests for ordering, removal, empty-list handling, and size tracking

Exit criteria:
- All list behaviors used by dependent modules are available in Rust
- No manual memory cleanup remains for the migrated list functionality
- Tests cover edge cases around head/tail mutations

### Phase 2: Port Output and Dot Helpers

Scope:
- Migrate `dot_print_symbol`
- Migrate `is_printable`
- Migrate `is_last`
- Migrate `print_refs`

Technical work:
- Create `src/dot.rs` and `src/output.rs`
- Port formatting helpers first, keeping function boundaries close to the C originals
- Adapt list traversal consumers to use the Rust list API from Phase 1
- Where output currently accumulates text, prefer `String` or generic writer parameters matching actual call-site needs
- Preserve filtering, ordering, and delimiter decisions exactly

Exit criteria:
- Dot/symbol and reference-printing helpers compile against the new list structures
- Output helper behavior is covered by unit tests using stable textual expectations

### Phase 3: Port Tree Output Functions

Scope:
- Migrate `direct_tree`
- Migrate `inverted_tree`
- Migrate `tree_output`

Technical work:
- Reconstruct the minimal traversal state required by the C implementation
- Translate recursive or iterative tree rendering logic directly into Rust
- Use borrowed views into list/payload data to avoid unnecessary duplication
- Keep direct and inverted traversal implementations separate unless they are already structurally identical in the C source

Exit criteria:
- Tree output functions produce equivalent structure and ordering
- Tests cover representative direct and inverted output cases
- No unresolved ownership issues remain in traversal code

### Phase 4: Integrate Main-Module Entry Points

Scope:
- Migrate `optfile_register`
- Rewire relevant usage from `src/main.c` into Rust entry points

Technical work:
- Connect the new Rust modules into `src/main.rs` or the current crate entry structure
- Keep registration and dispatch logic in the same layer as the original code
- Adjust signatures to use Rust references and owned values while preserving execution order
- Add integration-level tests where practical for registration plus output invocation paths

Exit criteria:
- The migrated module functions are callable from the Rust application entry path
- Build passes cleanly with `cargo test`
- Remaining behavior in the covered files is represented without adding new architectural layers