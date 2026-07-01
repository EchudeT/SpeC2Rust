# Implementation Plan

## Summary

This module ports the word-splitting node and segment processing portion of `src/wordsplit/wordsplit.c` into Rust, preserving the existing execution model and function boundaries as closely as practical. The target scope includes node insertion, segment accumulation, node list cleanup/debug dumping, segment coalescing, quote removal, finish-time normalization, prefix splitting, and environment/parameter lookup and assignment helpers.

The Rust implementation should translate the C linked-structure and mutable in-place processing into ownership-based Rust data structures with explicit mutation through `&mut` references. The plan is to migrate the existing file-local logic into one Rust module dedicated to the same responsibilities, avoiding expansion into new subsystems. Memory management will be handled by `Vec`, `String`, and enums instead of manual allocation/free paths, while preserving list order, segment combination rules, and lookup behavior. Error handling should replace implicit C failure cases with explicit `Result` returns only where allocation or invalid state must be surfaced; otherwise helper functions may remain total and internal.

## Technical Context

- **Language/Version**: Rust 1.76+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve linear or near-linear processing over node/segment collections.
  - Avoid unnecessary string copying during coalescing and quote-removal passes.
  - Replace recursive or pointer-chasing cleanup with deterministic container drops.
  - Keep environment lookup behavior efficient enough to match the C implementation’s expected usage patterns, using borrowed access where possible.

## Module Mapping

- **C source**
  - `src/wordsplit/wordsplit.c`

- **Rust target**
  - `src/wordsplit/wordsplit.rs`

- **Function migration mapping**
  - `wsnode_insert` -> `wsnode_insert`
  - `wordsplit_add_segm` -> `wordsplit_add_segm`
  - `wordsplit_free_nodes` -> removed as explicit public cleanup; retained as internal `clear_nodes`-style logic only if needed for state reset
  - `wordsplit_dump_nodes` -> `wordsplit_dump_nodes`
  - `coalesce_segment` -> `coalesce_segment`
  - `wsnode_quoteremoval` -> `wsnode_quoteremoval`
  - `wsnode_coalesce` -> `wsnode_coalesce`
  - `wsnode_tail_coalesce` -> `wsnode_tail_coalesce`
  - `wordsplit_finish` -> `wordsplit_finish`
  - `node_split_prefix` -> `node_split_prefix`
  - `wsplt_env_find` -> `wsplt_env_find`
  - `wsplt_env_lookup` -> `wsplt_env_lookup`
  - `wsplt_env_getvar` -> `wsplt_env_getvar`
  - `wsplt_assign_var` -> `wsplt_assign_var`
  - `wsplt_assign_param` -> `wsplt_assign_param`

- **Scope constraint**
  - Keep all migrated logic within the `wordsplit` area of the Rust crate.
  - Do not introduce additional abstraction layers beyond what is needed to represent the original state and processing safely.

## Data Model

Because the input only identifies anonymous C structures, the Rust data model should be derived from actual field usage in `wordsplit.c` and mapped conservatively.

### Core mapping approach

- **C linked lists of nodes/segments** -> `Vec<T>` where stable order is sufficient
- **C mutable text buffers / `char *`** -> `String` or `Vec<u8>` depending on whether byte-exact mutation is required
- **C flags / integer kinds** -> Rust `enum` or bitfield-like integer type, depending on how many named constants are used in the file
- **C nullable pointers** -> `Option<T>` / `Option<usize>` for indexes / `Option<&str>` for borrowed lookup results
- **Manual free ownership** -> automatic drop; explicit clear/reset only when the original logic reuses parser state

### Planned Rust structures

Since the C structs are unnamed in the analysis, define only the minimum equivalents needed by the listed functions:

- **Word-split state struct**
  - Represents the mutable parsing/splitting context currently carried through `wordsplit.c`
  - Expected fields:
    - node collection
    - environment source/reference
    - assignment/parameter state needed by `wsplt_assign_var` and `wsplt_assign_param`
    - option/flag fields used by coalescing and finish-time logic

- **Word node struct**
  - Represents one node in the intermediate split representation
    - segment collection or accumulated text
    - node kind/flags
    - quote/removal/coalescing markers if present in C logic
    - prefix/suffix split metadata if used by `node_split_prefix`

- **Segment struct**
  - Represents one text fragment before final node coalescing
    - fragment text
    - quoting/classification flags
    - whether the segment can be merged with neighbors

- **Environment source representation**
  - If the C code uses callback-style lookup, model this as:
    - borrowed function pointer/closure only if already required by surrounding Rust port
    - otherwise a borrowed map-like structure or slice of key/value pairs
  - Keep the representation local to existing project constraints; do not generalize further

- **Parameter / assignment representation**
  - Use a small enum to distinguish variable assignment targets if the C code encodes multiple assignment modes with flags
  - Otherwise retain plain strings and optional values

### C-to-Rust type conversion guidelines

- `char *` text under mutation:
  - Use `String` when semantic content is text and quote removal/coalescing can be expressed safely
  - Use `Vec<u8>` only if byte-level edits are required before UTF-8 validity can be guaranteed
- linked `next`/`prev` fields:
  - Remove when iteration order alone is needed
  - Use indices only if functions need insertion at arbitrary positions while preserving references
- numeric status/error returns:
  - Convert to `Result<(), WordSplitError>` or `Option<T>` based on original behavior
- debug dump output:
  - Accept a generic formatter or return a string for tests if direct stream dumping is not mandated by surrounding interfaces

## Implementation Phases

### Phase 1: State and container migration

- Inspect `src/wordsplit/wordsplit.c` and identify the exact anonymous struct layouts touched by:
  - `wsnode_insert`
  - `wordsplit_add_segm`
  - `wordsplit_free_nodes`
  - `wordsplit_dump_nodes`
- Define the minimum Rust structs/enums in `src/wordsplit/wordsplit.rs`.
- Replace C node/segment linked-list ownership with Rust-owned containers.
- Implement:
  - internal node reset/clear logic corresponding to `wordsplit_free_nodes`
- Add unit tests covering:
  - insertion order
  - empty/non-empty segment addition
  - repeated clear/reset behavior
  - debug dump stability for representative node layouts

### Phase 2: Coalescing and quote-removal passes

- Port the text normalization helpers in their existing dependency order:
  - `coalesce_segment`
  - `wsnode_quoteremoval`
  - `wsnode_coalesce`
  - `wsnode_tail_coalesce`
- Preserve original pass ordering and in-place mutation semantics as closely as Rust containers allow.
- Make ownership decisions explicit to minimize cloning when joining adjacent segments.
- Validate edge cases from the C logic:
  - empty segments
  - fully removed quoted content
  - tail-only coalescing
  - nodes containing multiple mergeable and non-mergeable segment boundaries

### Phase 3: Finish-stage node transformation

- Port:
  - `node_split_prefix`
  - `wordsplit_finish`
- Integrate these functions against the Rust node/segment model from Phases 1-2 rather than introducing a second representation.
- Preserve existing mutation order, especially where prefix splitting changes later coalescing or final node emission behavior.
- Add tests for:
  - prefix extraction/splitting cases present in the C implementation
  - finish-time behavior on mixed node lists
  - idempotence expectations if `wordsplit_finish` can be called after prior normalization

### Phase 4: Environment lookup and assignment helpers

- Port:
  - `wsplt_env_find`
  - `wsplt_env_lookup`
  - `wsplt_env_getvar`
  - `wsplt_assign_var`
  - `wsplt_assign_param`
- Keep the environment access shape narrowly aligned with how `wordsplit.c` uses it.
- Prefer borrowed return types during lookup where lifetimes are tractable; otherwise clone only at module boundaries.
- Convert C null/missing-variable behavior into `Option`/`Result` without altering lookup semantics.
- Add tests for:
  - found vs missing variables
  - lookup precedence if multiple sources exist in the C logic
  - assignment overwrite/update rules
  - parameter assignment edge cases already encoded by the original functions

## Notes on Memory Management and Error Handling

- No Rust equivalent of `wordsplit_free_nodes` should be exposed unless the wider port requires explicit state reuse; normal drop semantics are the default.
- Functions that only rearrange internal vectors/strings should avoid fallible APIs in their signatures unless the original control flow depends on allocation failure reporting.
- Where the C code relies on nullable pointers or sentinel states, introduce explicit enums/options rather than unchecked placeholders.
- Any debug-only dumping should avoid side effects on parser state and be testable through deterministic output.

## Completion Criteria

- All listed functions from `src/wordsplit/wordsplit.c` are present in Rust with matching processing scope.
- Anonymous C state used by those functions is mapped to explicit Rust types.
- Manual memory cleanup paths are removed or reduced to internal state-reset helpers.
- `cargo test` covers representative node lifecycle, coalescing, finish-stage, and environment/assignment behavior for this module.