# Implementation Plan: module_src_wordsplit_wordsplit_01

## Summary

This module migrates the core initialization, allocation, node-list handling, and error-context routines from `src/wordsplit/wordsplit.c` into Rust, preserving the existing execution model and control flow as closely as practical.

The Rust implementation should center on a single module corresponding to the original C file, with direct ports of the listed functions and only the data structures required by those functions. The technical approach is a conservative C-to-Rust translation:

- replace manual memory allocation with owned Rust containers and structs,
- convert sentinel/error-code mutation patterns into explicit result-based APIs where possible,
- retain internal helper boundaries that mirror the C function layout,
- preserve list/node semantics used by `wsnode_*`,
- keep sub-split and error-context propagation behavior localized to the same module.

No additional capabilities should be introduced beyond what is needed to migrate the existing file and function set.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**:
  - Rust standard library
  - No third-party crates are required based on the available input
- **Testing**:
  - `cargo test`
  - module-local unit tests for initialization, error setting, allocation growth, and node insertion/removal behavior
- **Performance Goals**:
  - preserve linear-time behavior of append/remove operations as in the original design
  - avoid unnecessary string/data copying during initialization and sub-split setup
  - use amortized-growth allocation strategies for internal buffers/lists instead of per-element allocation where the C code used expandable storage
  - maintain predictable memory ownership and zero unsafe code unless a specific layout constraint from later source inspection proves unavoidable

## Module Mapping

### Source File Mapping

- `src/wordsplit/wordsplit.c`
  - Rust target: `src/wordsplit.rs` if the crate already exposes a flat module
  - or `src/wordsplit/mod.rs` if `wordsplit` is already a directory module in the Rust project

Use the existing Rust project layout if present; do not split this migration into additional submodules unless the repository already requires that shape.

### Function Mapping

| C Function | Rust Target | Notes |
|---|---|---|
| `is_name_char` | `fn is_name_char(ch: char) -> bool` or byte-based helper | Keep semantics identical to C character classification used by this module |
| `_wsplt_alloc_die` | `fn wsplt_alloc_die(...) -> !` or `fn ... -> Result<_, WordsplitError>` | Prefer non-panicking propagation unless caller contract requires fatal behavior |
| `_wsplt_seterr` | `fn wsplt_seterr(state: &mut WordSplit, ...)` | Centralized mutable error-state update |
| `_wsplt_nomem` | `fn wsplt_nomem(state: &mut WordSplit) -> WordsplitError` or setter helper | Map out-of-memory path into explicit error variant |
| `_wsplt_store_errctx` | `fn wsplt_store_errctx(state: &mut WordSplit, ...)` | Preserve context capture fields |
| `_wsplt_setctxerr` | `fn wsplt_setctxerr(state: &mut WordSplit, ...)` | Combine error code and context mutation |
| `_wsplt_subsplit` | `fn wsplt_subsplit(state: &mut WordSplit, ...) -> Result<..., WordsplitError>` | Keep recursive/sub-operation state passing explicit |
| `_wsplt_seterr_sub` | `fn wsplt_seterr_sub(parent: &mut WordSplit, child: &WordSplit)` | Copy/translate child error state into parent |
| `wordsplit_init0` | `fn wordsplit_init0(...) -> WordSplit` or `fn ... -> Result<WordSplit, WordsplitError>` | Base-state initialization |
| `wordsplit_init` | `fn wordsplit_init(...) -> Result<WordSplit, WordsplitError>` | Public initializer wrapper |
| `alloc_space` | `fn alloc_space(state: &mut WordSplit, additional: usize) -> Result<(), WordsplitError>` | Replace realloc-style logic with `Vec`/capacity management |
| `wsnode_ptr` | `fn wsnode_ptr(...) -> Option<&...>` / index helper | Exact form depends on final node storage model |
| `wsnode_new` | `fn wsnode_new(...) -> WsNode` | Allocate/create node value |
| `wsnode_append` | `fn wsnode_append(state: &mut WordSplit, node: WsNode)` | Preserve insertion order |
| `wsnode_remove` | `fn wsnode_remove(state: &mut WordSplit, ...) -> Option<WsNode>` | Preserve unlink semantics |

## Data Model

The input lists anonymous C data structures without field definitions, so the Rust data model should be finalized only after direct inspection of `src/wordsplit/wordsplit.c`. The migration should still follow these mapping rules.

### Core Mapping Rules

| C Shape | Rust Shape | Migration Rule |
|---|---|---|
| opaque or stateful module struct | `struct WordSplit` | Consolidate mutable module state here |
| integer error/status fields | `enum WordsplitError` plus optional raw status field | Prefer typed errors; keep raw code only if needed for exact compatibility |
| C strings (`char *`) | `String`, `&str`, or `Vec<u8>` | Use `String` for validated text, `Vec<u8>` for byte-oriented parser state |
| allocation-managed arrays | `Vec<T>` | Use capacity reservation to mirror `alloc_space` intent |
| linked-list nodes | `struct WsNode` plus `Vec<WsNode>`/index-based links, or intrusive optional links if required | Prefer safe index-based storage over pointer-linked unsafe code |
| nullable pointers | `Option<T>` / `Option<usize>` / `Option<Box<T>>` | Match nullability explicitly |
| bitflags/options fields | integer fields or small enums | Keep representation simple unless exact flag behavior is needed |

### Expected Rust Types

These names are recommended placeholders until the C fields are inspected:

- `struct WordSplit`
  - owns parser/splitter state
  - stores error code/message/context
  - stores dynamic node collection and any token/output buffers
- `struct WsNode`
  - stores one node entry formerly managed by `wsnode_new`, `wsnode_append`, and `wsnode_remove`
- `struct ErrorContext`
  - stores source position / fragment / nested-subsplit context if the C code maintains this separately
- `enum WordsplitError`
  - variants for generic failure, invalid state, and out-of-memory-equivalent allocation failure

### Memory Management Decisions

- Replace `malloc`/`realloc`/`free` behavior with ownership through `Vec`, `String`, and stack-owned structs.
- Treat allocation failures as standard Rust allocation abort behavior unless the original module requires explicit recoverable error-state mutation; if so, preserve outward API behavior by returning `Result` and setting module error fields in the same places as C.
- Avoid raw pointers for node management unless the original algorithm depends on stable self-referential addresses that cannot be represented safely with indices.

### Error Handling Decisions

- Internal helpers that only mutate module state in C should become Rust methods/functions that update `WordSplit` and return `Result` where it reduces hidden control flow.
- If the surrounding project requires C-like stored error state, keep:
  - an internal numeric/status field,
  - optional message/context fields,
  - helper setters mirroring `_wsplt_seterr*`.
- Fatal allocation helper behavior from `_wsplt_alloc_die` should be validated against actual callers before choosing between:
  - `panic!`/diverging function, or
  - normalization into a returned `WordsplitError`.

## Implementation Phases

## Phase 1: Inspect and Define the Rust State Model

- Read `src/wordsplit/wordsplit.c` and identify the actual structs and field usage touched by:
  - initialization functions,
  - error/context helpers,
  - allocation helpers,
  - node helpers.
- Create the Rust module file matching the project’s existing layout.
- Define minimal Rust equivalents for:
  - the main wordsplit state,
  - node storage,
  - error/context storage.
- Decide per field whether it maps to:
  - `String`,
  - `Vec<u8>`,
  - `Vec<T>`,
  - `Option<_>`,
  - integer/enum values.
- Keep visibility narrow; expose only the Rust entry points needed to replace `wordsplit_init`.

## Phase 2: Port Initialization and Error Paths

- Implement:
  - `is_name_char`
  - `wordsplit_init0`
  - `wordsplit_init`
  - `_wsplt_seterr`
  - `_wsplt_nomem`
  - `_wsplt_store_errctx`
  - `_wsplt_setctxerr`
  - `_wsplt_seterr_sub`
  - `_wsplt_alloc_die`
- Preserve the original initialization order and default field values.
- Ensure all error helpers update the Rust state in the same sequence expected by later functions.
- Convert any C implicit-null initialization into explicit `Option`/empty-container initialization.
- Add unit tests for:
  - default initialization,
  - explicit error-state updates,
  - context overwrite/retention behavior,
  - sub-error propagation.

## Phase 3: Port Allocation and Node Management

- Implement:
  - `alloc_space`
  - `wsnode_new`
  - `wsnode_ptr`
  - `wsnode_append`
  - `wsnode_remove`
- Select the safest storage model that still preserves behavior:
  - first choice: `Vec<WsNode>` with indices,
  - fallback only if required by source semantics: boxed nodes with explicit links.
- Reproduce any capacity-growth logic from C using `Vec::reserve` or `try_reserve` if recoverable allocation behavior must be surfaced.
- Verify removal semantics carefully:
  - head removal,
  - middle removal,
  - last-node removal,
  - empty-list behavior.
- Add unit tests covering append order, pointer/index lookup behavior, and removal edge cases.

## Phase 4: Port Sub-split Wiring and Finalize Integration

- Implement `_wsplt_subsplit` using the completed state, error, and node infrastructure.
- Preserve parent/child state handoff and error-context translation exactly as in the C flow.
- Connect all helpers so the module can execute the same initialization-to-subsplit path without placeholder logic.
- Run `cargo test` and resolve borrow/ownership issues without changing external behavior.
- Perform a final pass to reduce unnecessary cloning and ensure helper signatures are no broader than needed for the migrated file.