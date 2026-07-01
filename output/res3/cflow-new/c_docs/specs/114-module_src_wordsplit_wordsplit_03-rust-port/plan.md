# Implementation Plan: module_src_wordsplit_wordsplit_03

## Summary

This module covers the word-splitting expansion path in `src/wordsplit/wordsplit.c`, specifically variable expansion, command expansion, whitespace trimming, null-node elimination, and restoration/recovery steps around expansion state. The Rust port should migrate the existing logic as closely as possible into a single Rust module aligned with the original file scope, preserving call order and mutation patterns rather than redesigning the subsystem.

The implementation approach is to:
- port the listed functions into a Rust module under the existing wordsplit area,
- map C stateful mutation to `&mut`-based APIs on the Rust wordsplit state,
- replace pointer-linked node manipulation with owned structs plus indexed or boxed links as needed by the current project layout,
- convert integer status/error returns into `Result` where the surrounding Rust port already supports it, otherwise maintain narrow internal status codes and adapt at module boundaries,
- keep expansion behavior and sequencing consistent with the C implementation, especially for in-place edits, null elimination, and recovery of partially expanded state.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Maintain linear or near-linear traversal behavior for word/node processing.
  - Avoid unnecessary string cloning during expansion and trimming.
  - Preserve in-place mutation where practical to match the C implementation’s cost profile.
  - Keep allocation growth bounded to expansion results that are required by the original behavior.

## Module Mapping

### C to Rust File Mapping

- `src/wordsplit/wordsplit.c`
  - migrate relevant expansion logic into:
    - `src/wordsplit.rs`, if the current Rust port keeps wordsplit in a single file, or
    - `src/wordsplit/mod.rs`, if the Rust port already uses the standard module directory form

### Function Mapping

The following C functions should be ported into the Rust wordsplit module with closely corresponding names, adjusted only if required by existing Rust naming conventions:

- `expvar_recover` -> `expvar_recover`
- `expand_paramv` -> `expand_paramv`
- `expvar` -> `expvar`
- `node_expand` -> `node_expand`
- `wsnode_nullelim` -> `wsnode_nullelim`
- `wordsplit_varexp` -> `wordsplit_varexp`
- `expcmd` -> `expcmd`
- `wordsplit_cmdexp` -> `wordsplit_cmdexp`
- `wordsplit_trimws` -> `wordsplit_trimws`

### Internal Scope Guidance

- Keep these functions in the same Rust module as the wordsplit state they mutate.
- Do not extract additional helper modules unless an existing Rust project structure already requires it.
- Prefer private helper functions for C-static equivalents and expose only the same effective module boundary already present in the Rust port.

## Data Model

Because the input only identifies anonymous C data structures, the data-model plan should be driven by the structures actually touched by these functions in `wordsplit.c`. The mapping below is intentionally restrained to the expansion-related state that must be migrated.

### Core Mapping Strategy

- **C anonymous structs used as local/state carriers**
  - map to named Rust `struct`s only when they persist across function boundaries
  - otherwise convert to local tuples or small private structs inside the Rust module

- **C pointer-owned mutable wordsplit state**
  - map to a central Rust `struct` representing the wordsplit context, passed as `&mut Wordsplit`

- **C linked word/node elements**
  - map to Rust owned node structs
  - use `Option<Box<Node>>` for direct linked-list preservation if the original Rust port uses list semantics
  - use `Vec<Node>` only if adjacent migrated code already uses indexed storage and can preserve ordering/mutation semantics without behavioral drift

- **C char buffers / slices**
  - map to `String` for owned mutable text
  - map to `&str` for borrowed read-only input
  - use byte-wise handling only where the C logic depends on exact delimiter or whitespace scanning

- **C flags / mode fields**
  - map to `u32`/`usize` bitflags or a small Rust enum set, depending on the shape already established in the port
  - prefer plain integer flags if this matches neighboring migrated code and avoids widening the redesign scope

- **C integer error codes**
  - map to `Result<T, WordsplitError>` at public/internal boundaries where already supported
  - otherwise preserve narrow internal status returns and translate once at the outer entry points

### Expected Rust Structures

These functions are likely to operate on a small set of state categories that should become explicit Rust types if not already present:

- **Wordsplit context**
  - holds options/flags, current node chain or token collection, expansion intermediates, and error state

- **Word/node entry**
  - holds the text fragment being expanded and any metadata needed to distinguish empty/null/expandable segments

- **Expansion recovery snapshot**
  - temporary state used by `expvar_recover` to restore prior text/node content after failed or partial expansion

- **Parameter expansion input/result carrier**
  - temporary representation for variable lookup result and generated fragments used by `expand_paramv` and `expvar`

- **Command expansion output carrier**
  - temporary representation for command substitution result before trimming and reinsertion

### Ownership and Memory Management

- Replace manual allocation/free patterns with owned Rust values and scoped temporaries.
- Use `std::mem::take`, `Option::take`, and temporary buffers to model C-style detach/replace operations safely.
- Where C mutates linked nodes while iterating, structure Rust code to avoid aliasing:
  - detach current content,
  - compute replacement,
  - write back,
  - then advance.
- Preserve empty-vs-null distinctions explicitly where they affect `wsnode_nullelim` or whitespace trimming behavior.

### Error Handling

- Expansion failure, invalid parameter state, or command expansion errors should not leave partially mutated visible state unless the C logic permits it.
- `expvar_recover` should be implemented as the explicit rollback path for operations that stage changes before commit.
- Avoid `panic!`; return structured errors or module-local status values.

## Implementation Phases

## Phase 1: Port Expansion State and Node Manipulation Primitives

- Identify the exact C structs and fields referenced by:
  - `node_expand`
  - `wsnode_nullelim`
  - `wordsplit_trimws`
  - `expvar_recover`
- Recreate only the required Rust state fields in the existing wordsplit context and node types.
- Port low-level node/text mutation logic first:
  - null elimination,
  - whitespace trimming,
  - recovery/restore of node state after failed expansion.
- Add focused unit tests for:
  - empty node removal behavior,
  - trimming behavior on leading/trailing whitespace,
  - restoration of pre-expansion content after rollback.

## Phase 2: Port Variable Expansion Path

- Port:
  - `expand_paramv`
  - `expvar`
  - `wordsplit_varexp`
- Keep the C expansion order and mutation style intact, especially where one function prepares intermediates consumed by the next.
- Implement variable lookup and replacement against the existing Rust wordsplit/environment representation already present in the project.
- Ensure empty expansion results are handled exactly as needed for later null elimination.
- Add tests covering:
  - successful variable substitution,
  - empty/unset variable outcomes as represented by the C path,
  - multiple expansions in one node or word sequence,
  - failure cases that trigger recovery.

## Phase 3: Port Command Expansion Path

- Port:
  - `expcmd`
  - `wordsplit_cmdexp`
- Map command expansion output into the same node/text pipeline used by variable expansion.
- Preserve post-processing order, especially interactions with whitespace trimming and node cleanup.
- Keep implementation limited to the existing project’s command-execution abstraction; do not introduce new execution layers.
- Add tests covering:
  - insertion of command output,
  - trimming of command output when required by the original logic,
  - empty command output and subsequent node elimination behavior,
  - error propagation without leaked partial state.

## Phase 4: Integrate and Verify Full Expansion Flow

- Wire the variable and command expansion paths into the surrounding Rust wordsplit flow in the same order as the C module.
- Reconcile return types/status handling so callers observe the same success/failure boundaries.
- Review all string and node mutations for avoidable cloning and borrow conflicts.
- Add integration-style tests around combined cases:
  - variable then command expansion in one pass,
  - command output feeding into trimming and null elimination,
  - rollback correctness when later expansion stages fail after earlier mutations.
- Finalize naming, visibility, and file placement to match the project’s Rust conventions without expanding module scope.