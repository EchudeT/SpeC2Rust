# Implementation Plan: module_src_wordsplit_wordsplit_03

## Summary

This plan covers the Rust port of the variable expansion, command expansion, node expansion, null-elimination, and whitespace-trimming logic currently implemented in `src/wordsplit/wordsplit.c` for the `module_src_wordsplit_wordsplit_03` module cluster.

The Rust implementation should migrate the existing behavior into a focused Rust module that preserves current processing order and state transitions rather than redesigning the subsystem. The implementation should center on:

- translating the expansion pipeline represented by:
  - `wordsplit_varexp`
  - `expvar`
  - `expand_paramv`
  - `expvar_recover`
  - `node_expand`
  - `expcmd`
  - `wordsplit_cmdexp`
  - `wsnode_nullelim`
  - `wordsplit_trimws`
- replacing pointer-based list and buffer manipulation with ownership-based Rust structures
- expressing recoverable failures and parse/expansion status using `Result`, explicit enums, and mutable context state
- keeping the module narrowly scoped to the existing file/function set, with no extra capabilities beyond the migrated logic

The technical approach is to port the C file into one Rust source module, introduce Rust equivalents for the C node and working-state structures used by these functions, and migrate behavior in dependency order so higher-level entry points are implemented only after the lower-level expansion helpers are stable.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Maintain linear or near-linear traversal behavior across word/node sequences where the C implementation is already linear
  - Avoid unnecessary string cloning during expansion by using `String`, `Vec<u8>` only if byte-level fidelity is required by the existing code, and in-place mutation where practical
  - Preserve existing expansion ordering without introducing extra passes beyond those already implied by the C logic
  - Keep allocation growth bounded to current expansion needs, primarily through `Vec` and `String` reuse inside the active expansion context

## Module Mapping

### C to Rust File Mapping

| C File | Rust File | Notes |
|---|---|---|
| `src/wordsplit/wordsplit.c` | `src/wordsplit.rs` or `src/wordsplit/mod.rs` | Keep the migrated logic together in a single Rust module unless the existing Rust crate layout already requires `mod.rs`. Do not split further without a direct structural need from the current codebase. |

### Function Mapping

| C Function | Rust Function | Migration Notes |
|---|---|---|
| `expvar_recover` | `fn expvar_recover(...) -> Result<..., WordSplitError>` or `fn expvar_recover(...)` | Preserve its role in restoring expansion state after a failed or partial variable expansion; final signature should follow the surrounding state model. |
| `expand_paramv` | `fn expand_paramv(...) -> Result<..., WordSplitError>` | Translate parameter-vector expansion logic using slices/vectors instead of pointer arithmetic. |
| `expvar` | `fn expvar(...) -> Result<..., WordSplitError>` | Core variable expansion routine; migrate before top-level variable-expansion entry points. |
| `node_expand` | `fn node_expand(...) -> Result<..., WordSplitError>` | Operates on one parsed node/unit at a time; should use mutable node references. |
| `wsnode_nullelim` | `fn wsnode_nullelim(...)` | Port list/node cleanup behavior using Rust collection filtering or explicit linked-node replacement, depending on the selected node representation. |
| `wordsplit_varexp` | `pub(crate) fn wordsplit_varexp(...) -> Result<..., WordSplitError>` | Top-level variable expansion pass across the active word/node set. |
| `expcmd` | `fn expcmd(...) -> Result<..., WordSplitError>` | Command expansion helper; preserve current subprocess/output integration assumptions from the existing project. |
| `wordsplit_cmdexp` | `pub(crate) fn wordsplit_cmdexp(...) -> Result<..., WordSplitError>` | Top-level command expansion pass. |
| `wordsplit_trimws` | `pub(crate) fn wordsplit_trimws(...)` or `-> Result` | Port whitespace trimming with exact behavior matching current node/string semantics. |

## Data Model

Because the provided analysis only exposes anonymous C structures, the Rust data model should be derived directly from the structures actually touched by the listed functions in `src/wordsplit/wordsplit.c`. The migration should not invent new abstraction layers; it should create minimal Rust equivalents for the concrete state carriers already in use.

### Data-Structure Mapping Strategy

| C Shape | Rust Shape | Migration Rule |
|---|---|---|
| Anonymous struct used as global/module state carrier for word splitting | `struct WordSplitState` | Collect the fields read/written by the target functions into one explicit Rust struct. Preserve field semantics and mutability boundaries. |
| Anonymous struct used for parsed word/node elements | `struct WsNode` plus `enum WsNodeKind` if node type tags exist | Replace pointer-linked nodes with either `Vec<WsNode>` indexed traversal or boxed next-links only if the original algorithm depends on splice-heavy linked-list behavior. |
| Anonymous struct used for temporary expansion buffers | `struct ExpansionBuffer` or direct `String`/`Vec<String>` fields within `WordSplitState` | Prefer standard types directly unless several fields move together repeatedly. |
| Anonymous struct used for parameter expansion bookkeeping | `struct ParamExpansionState` | Store current variable name, offsets, produced fragments, and flags previously encoded in mutable C locals/struct fields. |
| Anonymous struct used for command expansion bookkeeping | `struct CommandExpansionState` | Keep only the fields needed for command text, output accumulation, and status propagation. |
| C string pointers (`char *`, `const char *`) | `String`, `&str`, `Option<String>` | Use `&str` for borrowed input and `String` for owned mutable output. Use `Option` for nullable pointers. |
| Pointer/length string regions | `&str`, range indices, or `Vec<u8>` if exact byte mutation is required | Default to UTF-8 `String`/`&str`; switch to bytes only if the C logic demonstrably relies on raw byte operations incompatible with UTF-8 indexing. |
| C arrays of pointers | `Vec<String>` / `Vec<WsNode>` / slices | Preserve order exactly. |
| Bitflag/integer mode fields | `u32`/`usize` plus associated constants, or small enums | Use enums where the values are mutually exclusive; use integer masks only if multiple flags are combined in the original code. |
| Return-code integers | `Result<T, WordSplitError>` and small status enums | Convert success/error codes into typed errors while preserving non-error control outcomes through enums where needed. |

### Planned Core Rust Types

These names are intentionally minimal and should be adjusted to match the surrounding crate naming once the existing Rust codebase is inspected:

```rust
struct WordSplitState {
    // fields migrated from the C wordsplit state used by the target functions
}

struct WsNode {
    kind: WsNodeKind,
    text: String,
    // additional migrated fields as needed
}

enum WsNodeKind {
    // variants derived from C node type tags
}

enum WordSplitError {
    // variants derived from existing C error returns observed in the target functions
}
```

### Memory Management Decisions

- Replace manual allocation/free paths with owned Rust values.
- Remove explicit recovery/free logic whose only purpose in C was memory safety; keep only semantic recovery behavior that changes expansion state or output.
- Use `Option` instead of nullable pointers for optional intermediate nodes, buffers, and expansion products.
- Where C mutates linked structures during traversal, prefer:
  - `Vec<WsNode>` with retain/rewrite if ordering is stable and random insertion is limited
  - explicit index-based iteration to avoid borrow conflicts
- Avoid self-referential layouts and raw pointers.

### Error Handling Decisions

- Use `Result` for real failures from parsing, invalid expansion state, command expansion execution, or environment lookup rules already represented in C.
- Use explicit status enums for non-failure outcomes such as “no expansion performed”, “empty result”, or “recovered state”.
- Keep function signatures close to the role of the C originals; do not force all helpers into one shared generic abstraction.

## Implementation Phases

## Phase 1: Extract and Model Existing Expansion State

### Goal
Create the minimal Rust state and node types needed to support the listed functions, based directly on the fields and invariants used in `src/wordsplit/wordsplit.c`.

### Tasks
- Inspect the exact structs and typedefs referenced by:
  - `expvar_recover`
  - `expand_paramv`
  - `expvar`
  - `node_expand`
  - `wsnode_nullelim`
  - `wordsplit_varexp`
  - `expcmd`
  - `wordsplit_cmdexp`
  - `wordsplit_trimws`
- Define Rust equivalents for:
  - module/global wordsplit state
  - node representation
  - variable expansion temporary state
  - command expansion temporary state
- Map C constants, flags, and node-type tags into Rust constants/enums.
- Add skeletal function signatures in dependency order, returning `todo!()` initially.
- Establish shared error type(s) corresponding to current C error returns used by this function group.

### Deliverables
- Rust module file created and wired into the crate
- Core structs/enums/constants compiled
- Function signatures present for all listed functions

### Exit Criteria
- `cargo test` builds successfully with placeholder implementations or gated tests
- All required fields for the targeted functions are identified and represented in Rust types

## Phase 2: Port Variable and Node Expansion Logic

### Goal
Migrate the variable expansion path and node-local expansion behavior before introducing command expansion.

### Tasks
- Port `expand_paramv` first, translating parameter list/vector handling from pointer logic to slices/`Vec`.
- Port `expvar`, including:
  - variable-name scanning
  - value retrieval and substitution
  - propagation of empty or multi-part expansion results
  - any mode/flag checks currently used by the C logic
- Port `expvar_recover`, keeping only semantic rollback/state restoration behavior.
- Port `node_expand` on top of the variable expansion helpers.
- Port `wordsplit_varexp` as the traversal entry point over the active node/word sequence.
- Port `wsnode_nullelim` to remove or collapse empty nodes exactly where the C path does so after expansion.

### Testing Focus
- variable expands to non-empty value
- variable expands to empty value
- unset/missing variable behavior matches current C semantics
- multi-fragment parameter expansion behavior
- node removal/null elimination after empty expansion
- recovery path leaves state consistent after failed/aborted expansion

### Exit Criteria
- Variable expansion path is fully implemented
- Node list/state remains valid after expansion and null elimination
- `cargo test` passes for the variable-expansion scenarios

## Phase 3: Port Command Expansion and Whitespace Trimming

### Goal
Migrate the remaining expansion stage and final cleanup behavior using the state model established earlier.

### Tasks
- Port `expcmd`, keeping subprocess/output behavior aligned with the existing project’s current mechanism rather than redesigning it.
- Port `wordsplit_cmdexp` as the traversal/pass function that applies command expansion to the relevant nodes.
- Port `wordsplit_trimws`, preserving exact trimming semantics and ordering relative to prior expansion passes.
- Validate interaction order between:
  - variable expansion
  - command expansion
  - null elimination
  - whitespace trimming

### Testing Focus
- command expansion output insertion into node/text state
- empty command output behavior
- whitespace trimming before/after expansion results matches the C flow
- combined variable + command expansion path on the same input sequence

### Exit Criteria
- All listed functions are implemented in Rust
- Pass ordering matches the current C implementation
- `cargo test` passes for combined expansion/trimming cases

## Phase 4: Behavior Parity Cleanup

### Goal
Remove remaining C-shaped artifacts from the implementation while preserving behavior and keeping the port narrowly scoped.

### Tasks
- Review for unnecessary clones, temporary allocations, and placeholder status handling.
- Tighten signatures where helper return types can be made more explicit without changing behavior.
- Replace any residual sentinel-style state with `Option`/enum representations.
- Add regression tests for edge cases discovered during line-by-line comparison with `src/wordsplit/wordsplit.c`.
- Confirm no extra module splits or helper subsystems were introduced beyond what this port requires.

### Exit Criteria
- Rust implementation is idiomatic but still structurally traceable to the C original
- Tests cover the main branches of all migrated functions
- Module is ready for use on branch `114-module_src_wordsplit_wordsplit_03-rust-port`

## Notes and Constraints

- Keep the migration constrained to the behavior already present in `src/wordsplit/wordsplit.c`.
- Do not introduce additional parsing layers, generic expansion frameworks, or unrelated utility modules.
- Preserve the original processing sequence and mutation semantics where they affect output.
- Prefer standard-library collections and strings; add no third-party crates unless later source inspection shows an unavoidable existing dependency.
- Base final type names on the actual identifiers present in the codebase where available, but keep the one-file migration boundary intact.