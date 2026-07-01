# Implementation Plan

## Summary

Port `src/wordsplit/wordsplit.c` into an idiomatic Rust module that preserves the existing node-based word-splitting workflow, segment accumulation, node coalescing, quote removal, and environment/assignment helper behavior without adding new features.

The Rust implementation should stay close to the current C execution model:

- represent the mutable split state and node chain explicitly,
- translate pointer-linked list operations into safe ownership-based structures,
- preserve operation ordering for insert/coalesce/finalization steps,
- model environment lookup and assignment helpers as narrow internal functions,
- replace manual allocation/free logic with `Vec`, `String`, and drop-driven cleanup.

The technical approach is a direct file/function migration, centered on one Rust source module for this C file, with small internal types for nodes, segments, and variable lookup state. Unsafe Rust should be avoided unless a specific linked-list mutation cannot be expressed cleanly with standard ownership patterns; the default plan is to redesign the list handling into index-based or `Vec`-backed storage to keep memory safety explicit.

## Technical Context

- **Language/Version**: Rust 1.78+ stable
- **Primary Dependencies**:
  - Rust standard library
  - No third-party crates required based on the available module evidence
- **Testing**:
  - `cargo test`
  - unit tests for node insertion/coalescing/quote removal/finalization
  - focused regression tests for environment lookup and assignment helpers
- **Performance Goals**:
  - maintain linear or near-linear behavior for ordinary word-splitting passes
  - avoid unnecessary string cloning during segment coalescing where possible
  - keep allocation count comparable to or lower than the C port by using `String` growth and `Vec` reuse
  - preserve predictable behavior for long token/segment chains

## Module Mapping

### Source File Mapping

- **C source**: `src/wordsplit/wordsplit.c`
- **Rust target**: `src/wordsplit/wordsplit.rs`

If the crate already exposes a `wordsplit` module, this file should be added under that existing module tree rather than introducing a new top-level design.

### Function Mapping

| C Function | Rust Target | Notes |
|---|---|---|
| `wsnode_insert` | `fn wsnode_insert(...)` | Internal mutable node insertion helper; prefer index-based insertion or `Vec` splice-like logic over raw links. |
| `wordsplit_add_segm` | `fn wordsplit_add_segm(...)` | Append/add segment data into current split state; use `String`/byte slices as appropriate. |
| `wordsplit_free_nodes` | removed as explicit function or retained as `fn clear_nodes(&mut self)` | Manual free should become drop-based; retain only if logic also resets internal state. |
| `wordsplit_dump_nodes` | `fn wordsplit_dump_nodes(...)` gated for debug/test use | Keep only if currently used by diagnostics/tests. |
| `coalesce_segment` | `fn coalesce_segment(...)` | Merge adjacent segment content in place. |
| `wsnode_quoteremoval` | `fn wsnode_quoteremoval(...)` | In-place quote removal over node content/segments. |
| `wsnode_coalesce` | `fn wsnode_coalesce(...)` | Merge compatible neighboring nodes. |
| `wsnode_tail_coalesce` | `fn wsnode_tail_coalesce(...)` | Tail-oriented coalescing helper; likely simplified with contiguous storage. |
| `wordsplit_finish` | `fn wordsplit_finish(...) -> Result<..., ...>` or plain finalizer | Final pass converting node state to output representation and cleanup/reset. |
| `node_split_prefix` | `fn node_split_prefix(...)` | Split one node into prefix/remainder while preserving ordering. |
| `wsplt_env_find` | `fn wsplt_env_find(...)` | Internal environment search helper over configured variable source. |
| `wsplt_env_lookup` | `fn wsplt_env_lookup(...)` | Wrapper for lookup semantics and status handling. |
| `wsplt_env_getvar` | `fn wsplt_env_getvar(...)` | Obtain variable value in owned/borrowed Rust form. |
| `wsplt_assign_var` | `fn wsplt_assign_var(...)` | Internal assignment helper; return structured error/status instead of sentinel integers where feasible. |
| `wsplt_assign_param` | `fn wsplt_assign_param(...)` | Parameter assignment helper; align with existing state mutation order. |

## Data Model

The C analysis only exposes anonymous structures, so the Rust plan should introduce narrowly scoped named types derived from usage in the listed functions rather than inventing broader abstractions.

### Planned Rust Types

| C Shape | Rust Type | Purpose |
|---|---|---|
| anonymous node struct | `struct WsNode` | Represents one split node/token with content, flags, and adjacency/index metadata if needed. |
| anonymous segment struct | `struct Segment` | Represents a segment accumulated before/within node coalescing. |
| anonymous wordsplit state struct | `struct WordSplit` | Main mutable state previously passed around C helpers. |
| anonymous environment entry/view | `struct EnvEntry<'a>` or tuple form | Temporary result for variable lookup helpers. |
| anonymous assignment state/parameter view | `struct AssignTarget` | Internal helper data for variable/parameter assignment flow. |
| anonymous flags fields | `bitflags`-like replacement using plain Rust newtype or `u32` constants | Use standard library constants/newtypes unless stronger typing is already present elsewhere in crate. |
| nullable string pointer fields | `Option<String>` / `Option<&str>` | Replace nullability explicitly. |
| linked list pointers | `Vec<WsNode>` plus indices, or `Linked` fields as `Option<usize>` | Chosen to preserve insertion/coalescing semantics safely. |
| char buffer + length | `String` or `Vec<u8>` | Use `String` if content is text-managed; use `Vec<u8>` only if byte-exact mutation is required. |
| C status/error codes | `Result<T, WordSplitError>` or internal enum | Preserve call-site branching while making failure explicit. |

### Ownership and Memory Decisions

- Replace all explicit allocation/free pairs with owned Rust containers.
- Prefer storing node content as `String`.
- If the C code mutates byte buffers in place during quote removal or splitting, use temporary `Vec<u8>` internally and convert to `String` only when validity is guaranteed.
- Any former borrowed pointers into mutable buffers must be reworked into indices/ranges to avoid invalidation during reallocation.
- Functions equivalent to `wordsplit_free_nodes` should become state-reset helpers only, not memory-management requirements.

### Error Handling Mapping

- C integer return codes should map to a compact internal error enum such as `WordSplitError`.
- Pure helpers that cannot fail after type conversion should return plain values instead of `Result`.
- Lookup helpers should distinguish:
  - variable not found,
  - variable found with value,
  - assignment/update failure,
  only if that distinction exists in current call flow.

## Implementation Phases

## Phase 1: Scaffold the Rust module and core state

- Create `src/wordsplit/wordsplit.rs`.
- Define the minimum Rust equivalents for the wordsplit state, node, segment, flags, and lookup/assignment helper data.
- Choose the internal node storage strategy:
  - prefer `Vec<WsNode>` with explicit indices for previous/next relationships if insertion order and adjacent merges matter,
  - avoid raw pointers.
- Establish function signatures for all listed C functions as internal Rust functions/methods.
- Add basic unit tests for construction, empty finalization, and state reset behavior.

### Deliverables

- compilable Rust module skeleton
- named Rust structs/enums replacing anonymous C data shapes
- initial error/status type
- empty-path tests

## Phase 2: Port node and segment mutation logic

- Implement `wordsplit_add_segm`, `wsnode_insert`, `coalesce_segment`, `node_split_prefix`, `wsnode_coalesce`, and `wsnode_tail_coalesce`.
- Keep migration order aligned with data flow:
  1. segment addition,
  2. node insertion,
  3. prefix splitting,
  4. local segment merge,
  5. node coalescing.
- Preserve original mutation order and boundary conditions exactly, especially around empty segments, prefix lengths, and adjacent-node compatibility checks.
- Replace any C pointer surgery with index updates or `Vec` reorganization.
- Add unit tests covering:
  - insertion at head/middle/tail,
  - splitting a node into prefix/remainder,
  - merging adjacent compatible nodes,
  - no-op coalescing cases.

### Deliverables

- working node/segment transformation path
- tests for structural mutations
- no explicit free logic beyond state reset

## Phase 3: Port quote removal and finalization flow

- Implement `wsnode_quoteremoval`, `wordsplit_finish`, and `wordsplit_dump_nodes` if still required for diagnostics.
- Ensure quote removal operates on the same content representation used by coalescing to avoid duplicate conversions.
- Make finalization responsible for:
  - consuming or flattening node state into the module’s expected output form,
  - applying final coalescing/cleanup steps in the same sequence as C,
  - resetting temporary state when needed.
- If `wordsplit_dump_nodes` is only observational, keep it behind `#[cfg(test)]` or debug-only usage unless public behavior already depends on it.
- Add tests for:
  - quote removal edge cases,
  - final output ordering,
  - finalization after multiple segment/node operations.

### Deliverables

- completed transformation-to-output path
- quote-removal coverage
- debug dump helper only if needed

## Phase 4: Port environment lookup and assignment helpers, then align behavior

- Implement `wsplt_env_find`, `wsplt_env_lookup`, `wsplt_env_getvar`, `wsplt_assign_var`, and `wsplt_assign_param`.
- Keep these helpers internal and narrowly coupled to the wordsplit state and call sites already present in this module.
- Map C null/not-found/update-status behavior into `Option`/`Result` without altering observable branch decisions.
- Reconcile any string ownership differences so environment values can be used without dangling references.
- Add focused tests for:
  - found vs not-found lookup,
  - empty value handling if present in C behavior,
  - successful assignment,
  - assignment rejection/error paths.
- Finish by comparing all migrated functions against the C file for control-flow parity and removal of dead scaffolding.

### Deliverables

- completed helper migration
- regression tests for lookup/assignment behavior
- final pass for semantic parity with `wordsplit.c`