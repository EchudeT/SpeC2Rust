# Implementation Plan: module_src_wordsplit_wordsplit_03

## Summary

This module covers the `src/wordsplit/wordsplit.c` portion responsible for variable expansion, command expansion, node-level expansion traversal, null-elimination after expansion, and whitespace trimming in the word-splitting pipeline.

The Rust implementation should be a direct port of the existing behavior into the current crate structure, keeping the expansion logic localized to the wordsplit module rather than introducing new subsystem boundaries. The technical approach is:

- migrate the listed functions into a Rust `wordsplit` module segment dedicated to expansion;
- preserve the original processing order: parameter/variable expansion, command expansion, node expansion, null elimination, and trimming;
- convert pointer-based node manipulation into ownership-checked mutation over Rust structs and enums;
- replace ad hoc C error propagation and cleanup with `Result`-based returns plus scoped ownership;
- keep allocation patterns simple and local, using `String`, `Vec`, and `Option` instead of manual buffer management.

The implementation should prioritize behavioral equivalence with the C module over refactoring.

## Technical Context

- **Language/Version:** Rust 1.78+
- **Primary Dependencies:** Rust standard library only
- **Testing:** `cargo test`
- **Performance Goals:**
  - Maintain linear or near-linear behavior for per-token expansion paths where the C implementation is linear.
  - Avoid unnecessary intermediate allocations during string assembly and node rewriting.
  - Preserve in-place style transformations at the container level where practical with `Vec` mutation and `String` reuse.
  - Do not introduce global state, synchronization overhead, or extra abstraction layers beyond what is needed for the port.

## Module Mapping

### Source Mapping

- **C source:** `src/wordsplit/wordsplit.c`
- **Rust target:** existing Rust wordsplit implementation file for this port branch, preferably:
  - `src/wordsplit/mod.rs`, if the crate keeps the wordsplit logic in one file, or
  - `src/wordsplit.rs`, if that is the crate’s current flat layout

If the Rust port already split the wordsplit code across files, this module should map only to the existing expansion-related file under the same module, without creating extra architectural layers.

### Function Mapping

Each C function should be migrated as a Rust function with the same processing role and as close a name as is idiomatic while preserving traceability:

- `expvar_recover` -> `fn expvar_recover(...) -> ...`
- `expand_paramv` -> `fn expand_paramv(...) -> Result<..., WordSplitError>`
- `expvar` -> `fn expvar(...) -> Result<..., WordSplitError>`
- `node_expand` -> `fn node_expand(...) -> Result<..., WordSplitError>`
- `wsnode_nullelim` -> `fn wsnode_nullelim(...) -> ...`
- `wordsplit_varexp` -> `fn wordsplit_varexp(...) -> Result<..., WordSplitError>`
- `expcmd` -> `fn expcmd(...) -> Result<..., WordSplitError>`
- `wordsplit_cmdexp` -> `fn wordsplit_cmdexp(...) -> Result<..., WordSplitError>`
- `wordsplit_trimws` -> `fn wordsplit_trimws(...) -> ...`

Notes for mapping:

- Functions that mutate parser/split state should take `&mut WordSplit` or equivalent existing context type.
- Functions that operate on node lists should take `&mut Node`, `&mut Vec<Node>`, or `&mut LinkedNode` depending on the existing Rust port’s chosen representation.
- Recovery-style helpers should remain private to the module unless already needed externally.

## Data Model

The input analysis only reports anonymous C data structures, so the plan should map them through usage rather than by exact original names. The Rust port should avoid inventing new public types unless required by already-existing crate structure.

### Expected C-to-Rust Structure Mapping

| C shape / usage pattern | Rust mapping |
|---|---|
| wordsplit state struct | `struct WordSplit { ... }` or existing equivalent |
| expansion/input cursor fields | integer indices as `usize` |
| mutable string buffers | `String` |
| dynamic arrays of words/nodes | `Vec<T>` |
| nullable owned char pointers | `Option<String>` or `Option<Vec<u8>>` if byte semantics are required |
| linked expansion nodes | `Vec<Node>` preferred; use existing linked representation only if already established in port |
| node kind tags / flags | `enum NodeKind` and/or bitflags stored in integer fields already present in the Rust port |
| temporary rollback snapshots | small owned snapshot structs or cloned local values |
| command expansion result buffers | `String` or `Vec<String>` depending on current Rust pipeline |
| booleans stored as ints | `bool` |
| status / error return codes | `Result<T, WordSplitError>` |
| optional substructures | `Option<T>` |
| borrowed string slices during scanning | `&str` where UTF-8 assumptions are valid; otherwise index-based access over bytes |

### Anonymous Structures

Because the C analysis only identifies anonymous structures, the migration should proceed by locating the concrete state holders used by the listed functions in `wordsplit.c` and mapping them into the already-defined Rust equivalents in the port branch. The plan should not introduce placeholder structs solely to mirror anonymity in C.

### Memory Management Decisions

- Replace manual allocation/free pairs with owned Rust values.
- Convert rollback/recovery code to scoped ownership first; only preserve explicit restoration state where behavior requires undoing partial mutation.
- For node-list edits, prefer `Vec` filtering/rewrite over pointer relinking unless the port already uses boxed linked nodes.
- Avoid exposing references that outlive mutation steps; perform expansion in clearly bounded mutable passes.

### Error Handling Decisions

- C integer status returns should become `Result`.
- Functions that cannot fail semantically, such as simple trimming or null elimination, may return plain values or mutate in place without `Result`.
- Preserve distinguishable error categories already implied by the C logic, such as malformed expansion syntax, failed command expansion, or invalid state transitions, through an existing or local `WordSplitError` enum.
- Recovery helpers should restore internal state before returning the error when the C logic relies on that guarantee.

## Implementation Phases

## Phase 1: Establish Rust placements and port variable expansion core

Scope:

- Identify the existing Rust wordsplit state, node, and error types already present on branch `114-module_src_wordsplit_wordsplit_03-rust-port`.
- Add or complete the direct Rust counterparts for:
  - `expvar_recover`
  - `expand_paramv`
  - `expvar`
  - `wordsplit_varexp`
- Wire these functions into the existing wordsplit execution path without changing higher-level flow.

Technical decisions:

- Keep function visibility minimal; helper routines remain private.
- Use `String` accumulation for expanded text.
- Represent parser position and substring boundaries with `usize`.
- Convert C cleanup paths into ownership-based locals and `Result` returns.

Validation:

- Add focused unit tests covering variable/parameter expansion behavior through the public wordsplit entry point if one exists; otherwise test the smallest reachable internal layer in-module.
- Confirm state restoration behavior for malformed or incomplete variable expansions.

## Phase 2: Port node-level expansion and post-expansion cleanup

Scope:

- Port:
  - `node_expand`
  - `wsnode_nullelim`
  - `wordsplit_trimws`
- Adapt the Rust node container manipulation to preserve original ordering and removal semantics.

Technical decisions:

- If the current Rust port uses `Vec` for node storage, implement null elimination as retain/filter logic.
- If trimming in C mutates buffer endpoints, implement the Rust equivalent with slice-boundary computation and controlled `String` rewriting.
- Keep node expansion as an in-place mutable pass over the existing node structure.

Validation:

- Add tests for:
  - expansion over multi-node inputs,
  - null-result elimination after expansion,
  - whitespace trimming edge cases,
  - preservation of node ordering and token boundaries.

## Phase 3: Port command expansion path

Scope:

- Port:
  - `expcmd`
  - `wordsplit_cmdexp`
- Integrate command expansion into the same execution sequence used by the C module.

Technical decisions:

- Match the current project’s existing mechanism for executing or simulating command expansion; do not introduce a new abstraction layer.
- Ensure command expansion results are normalized into the same node/string representation used by variable expansion.
- Preserve failure propagation and partial-state cleanup requirements using `Result`.

Validation:

- Add tests for successful command expansion paths as supported by the existing project behavior.
- Add failure-path tests ensuring no invalid residual node state remains after errors.

## Phase 4: Behavioral alignment and consolidation

Scope:

- Review the combined expansion pipeline across variable expansion, command expansion, node mutation, null elimination, and trimming.
- Remove temporary porting scaffolding and align naming/signatures with the rest of the Rust wordsplit module while preserving traceability to the C functions.

Technical decisions:

- Keep the final code in the same Rust module area as the rest of wordsplit logic.
- Avoid introducing new modules, traits, or generic layers unless already required by existing port code.
- Ensure all listed functions are either directly present or clearly subsumed by private helpers plus one direct wrapper per C function role.

Validation:

- Run `cargo test`.
- Add regression tests from C-observed edge cases encountered during the port.
- Verify that all migrated functions are connected and no dead partial implementation remains.