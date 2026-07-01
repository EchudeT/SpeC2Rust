# Implementation Plan: module_src_parser.c_32

## Summary

Port the `src/parser.c` module subset covering `reference` and `reset_static_caller` into Rust on branch `095-module_src_parser.c_32-rust-port`. The Rust implementation should preserve the existing control flow and state transitions of the C code while replacing implicit C memory and static-state handling with explicit Rust ownership and borrowing.

The technical approach is to migrate only the functionality required by these functions into a Rust parser module, keeping the implementation close to the current file-level organization. Any C file-scope static state used by `reset_static_caller` should be represented as explicit module-local or parser-owned state in Rust, preferring instance state over global mutable state where the original call patterns allow it. Pointer-driven C data access should be converted into `Option`, references, and owned containers from the standard library.

## Technical Context

- **Language/Version**: Rust 1.78+ stable
- **Primary Dependencies**:
  - Rust standard library only
- **Testing**:
  - `cargo test`
- **Performance Goals**:
  - Preserve the current asymptotic behavior of parser-related operations.
  - Avoid unnecessary heap allocations beyond those already implied by the C logic.
  - Keep state reset operations constant-time where they are constant-time in C.
  - Maintain predictable memory usage by using explicit ownership for parser state and referenced entities.

## Module Mapping

### C to Rust File Mapping

- `src/parser.c` -> `src/parser.rs`

### Function Mapping

- `reference` -> `parser::reference`
- `reset_static_caller` -> `parser::reset_static_caller`

### Rust Module Shape

Keep the migration constrained to a single Rust module corresponding to the original C file:

- `src/parser.rs`
  - migrated parser-local data structures required by these functions
  - `reference`
  - `reset_static_caller`
  - minimal helper functions only if needed to separate unsafe-free state updates from parsing logic

If the translated functions depend on existing parser context or symbol structures from adjacent migrated modules, expose them through crate-local imports rather than introducing new abstraction layers.

## Data Model

Because the source analysis lists only anonymous C data structures, the Rust data model should be derived from actual field usage inside `reference` and `reset_static_caller`, not from speculative full-file redesign.

### Mapping Rules

- **Anonymous C structs used only within `parser.c`**
  - Map to private Rust `struct` types in `src/parser.rs`.
  - Rename according to role inferred from field usage, for example:
    - parser-local state -> `ParserState`
    - reference/caller tracking node -> `ReferenceState` or `CallerState`
    - temporary parse record -> `ParseEntry`
- **Anonymous C unions or tagged variants**
  - Map to Rust `enum` when the C logic distinguishes variants explicitly.
  - Otherwise map to a `struct` with optional fields if the original code treats fields independently.
- **C pointers**
  - Non-owning nullable pointer -> `Option<&T>` / `Option<&mut T>` where borrow lifetimes are local.
  - Long-lived links or graph edges -> indices, owned values, or `Option<Box<T>>` only if ownership is truly hierarchical.
  - Shared graph-like references -> likely `usize` identifiers or crate-local handles if the original logic links existing records without ownership transfer.
- **C strings**
  - Read-only textual data -> `&str` when borrowed from caller-owned input.
  - Stored parser names/symbol text -> `String`.
- **C integer flags**
  - Map to `bool` for binary state.
  - Map to small integer types or enums when values carry distinct modes.

### Static State Handling

Any file-scope static variables touched by `reset_static_caller` should be mapped as one of:

1. Fields on an existing parser/session state struct, preferred when reset occurs in the context of parser execution.
2. Private module-local state using `Option<T>` only if the original behavior is truly global to the module and cannot be cleanly attached to parser state without changing semantics.

Prefer eliminating mutable global state in favor of explicit state passed into `reference` and `reset_static_caller`, but do not broaden the API beyond what is necessary to preserve existing call structure.

### Memory Management Decisions

- Replace manual lifetime management and null checks with:
  - owned `String`, `Vec`, and `Box` where C allocates storage
  - `Option` for nullable state
  - borrowing for transient parser access
- Avoid `Rc`, `Arc`, `RefCell`, or synchronization primitives unless the original code structure makes ownership otherwise impossible; this migration does not require adding concurrency-oriented indirection.
- Any previous static pointer reset in C should become a simple field reassignment such as `state.current_caller = None`.

### Error Handling

- If the C code signals failure through null returns or integer status codes, translate to:
  - `Option<T>` for simple presence/absence
  - `Result<T, ParserError>` only when the function already has meaningful failure branches that must be surfaced
- Do not introduce broad custom error frameworks; keep error types local and minimal.

## Implementation Phases

### Phase 1: Analyze and Scaffold Direct Rust Mapping

- Inspect `src/parser.c` field usage for `reference` and `reset_static_caller`.
- Identify all anonymous structs and static variables touched directly or through helper calls.
- Create `src/parser.rs` with:
  - private struct definitions for only the data required by these functions
  - placeholder signatures for `reference` and `reset_static_caller`
  - crate-local imports for any adjacent migrated parser or symbol types already present in the Rust port
- Decide whether static caller state belongs in parser-owned state or strict module-local state based on the original call graph.

**Exit criteria**:
- Rust file exists with compile-ready type skeletons.
- Every C data dependency used by the two functions has a planned Rust representation.

### Phase 2: Port State Reset and Static Caller Semantics

- Implement `reset_static_caller` first.
- Translate any C static/global reset behavior into explicit Rust state clearing.
- Verify that null-pointer resets become `Option::None` and that no unsafe code is needed.
- Keep function semantics aligned with original ordering and side effects.

**Exit criteria**:
- `reset_static_caller` compiles and its effects are testable through parser/module state inspection.
- No mutable global raw-pointer pattern remains.

### Phase 3: Port `reference` with Minimal Structural Adaptation

- Translate the `reference` function closely from C to Rust.
- Replace pointer traversal and null checks with:
  - `Option` pattern matching
  - mutable references for in-place updates
  - standard collections only where the C code already implies owned lists or buffers
- Preserve existing mutation order, especially if `reference` updates caller/reference tracking state that interacts with `reset_static_caller`.
- Add only the helper routines required to keep borrow scopes manageable.

**Exit criteria**:
- `reference` compiles and matches C-side behavior for state mutation and return values.
- Ownership and borrowing are explicit, with no hidden aliasing assumptions.

### Phase 4: Validate Behavior with Focused Tests and Cleanup

- Add unit tests in the parser module or adjacent test module covering:
  - caller/reference state initialization
  - repeated `reference` calls across reset boundaries
  - `reset_static_caller` idempotence
  - null/empty-state scenarios corresponding to C defensive branches
- Run `cargo test`.
- Remove unused scaffolding and confirm naming consistency with the rest of the Rust port.

**Exit criteria**:
- Tests pass under `cargo test`.
- Migrated code is limited to the original module scope and required supporting types only.

## Notes and Constraints

- Keep the migration file-scoped: do not split the parser subset into additional modules unless the existing Rust project layout already requires it.
- Do not add new parser features, thread-safe wrappers, serialization, FFI layers, or benchmarking support.
- Preserve C behavior first; only make structural Rust changes necessary for safety, ownership clarity, and compilation.