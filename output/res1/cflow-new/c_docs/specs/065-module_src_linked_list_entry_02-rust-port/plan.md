# Implementation Plan

## Summary

Port the linked-list and caller-target processing logic currently implemented in `src/symbol.c` into an idiomatic Rust module while preserving existing behavior and traversal order. The Rust implementation should focus on a direct migration of the listed functions:

- `collect_functions`
- `move_parms`
- `first_starter`
- `next_starter`
- `mark_callers`
- `eliminate_non_targets`

The technical approach is a conservative translation of the current in-memory list processing and state mutation patterns into Rust-owned data structures. Pointer-linked traversal from C should be replaced with explicit ownership and indexed/link-style references where needed, avoiding unsafe code unless the exact C algorithm cannot be expressed safely without excessive behavioral drift. The preferred implementation is a single Rust source module corresponding closely to the existing C file scope, with helper types only as required to represent current list nodes, flags, and traversal state.

Key migration goals:

- Preserve function ordering semantics and list mutation behavior.
- Replace raw pointer manipulation with `Option`, `Vec`, and explicit node identifiers or boxed nodes.
- Encode nullable relationships and membership markers explicitly.
- Keep error handling minimal and local, using `Result` only where the surrounding Rust project API requires fallible behavior.
- Avoid adding capabilities beyond the current C module responsibilities.

## Technical Context

### Language / Version

- Rust stable, edition 2021
- Minimum recommended compiler: `rustc 1.76+`

### Primary Dependencies

Use the Rust standard library by default.

Recommended crates:

- None required for this module based on the provided input.

### Testing

- `cargo test`

Testing scope should include:

- direct unit tests for each migrated function
- fixture-style tests covering linked-list traversal and mutation order
- regression tests for starter iteration and caller marking/elimination behavior

### Performance Goals

- Preserve asymptotic behavior of the C implementation as closely as practical.
- Avoid unnecessary cloning of symbol/function records during list rewrites.
- Use contiguous storage (`Vec`) where it can replace repeated pointer chasing without changing observable behavior.
- Keep per-pass traversal linear in the number of nodes/functions for the main marking and elimination routines.
- Do not introduce heap allocations inside tight traversal loops beyond what is required to represent the existing structure safely.

## Module Mapping

### Source File Mapping

| C File | Rust File | Notes |
|---|---|---|
| `src/symbol.c` | `src/symbol.rs` | Direct port target for the listed linked-list and traversal functions |

### Function Mapping

| C Function | Rust Function | Migration Notes |
|---|---|---|
| `collect_functions` | `collect_functions` | Port as the primary function collection pass; preserve ordering and filtering criteria |
| `move_parms` | `move_parms` | Port list/node relocation logic carefully to avoid aliasing issues |
| `first_starter` | `first_starter` | Represent first-match traversal using explicit iterator state or index lookup |
| `next_starter` | `next_starter` | Continue starter traversal using stored state rather than raw pointer advancement |
| `mark_callers` | `mark_callers` | Port graph/list marking logic with explicit mutable access to node state |
| `eliminate_non_targets` | `eliminate_non_targets` | Preserve in-place filtering/removal behavior using retained indices or list relinking |

### Rust Module Boundary

Keep implementation within a single Rust module mirroring the current C scope:

- `src/symbol.rs`

If the existing Rust crate already exposes a module tree, register this file with the minimal required `mod` declaration only. Do not split the functionality into extra submodules unless the current project layout already demands it.

## Data Model

The input only identifies multiple anonymous C data structures, so the Rust plan should treat them as internal migration targets derived from actual `src/symbol.c` usage rather than inventing new domain abstractions.

### Data-Structure Mapping Strategy

| C Pattern | Rust Representation | Notes |
|---|---|---|
| anonymous struct used as list node | named `struct` with explicit fields | Assign stable names based on role in the C file, not generic placeholders |
| nullable pointer to next node | `Option<NodeId>` or `Option<Box<Node>>` | Prefer `NodeId` + storage arena/`Vec` when mutation across traversal is frequent |
| nullable pointer to related symbol/function/caller | `Option<NodeId>` / `Option<usize>` | Avoid self-referential borrowing |
| integer flags/markers | `bool`, small `enum`, or integer field | Preserve bit-level meaning only if the C code depends on combined flags |
| pointer-owned record chains | `Vec<Record>` plus explicit linkage, or boxed linked nodes | Choose one representation consistently per list family |
| C strings (`char *`) | `String` or `Option<String>` | Use owned strings unless borrowed input already exists elsewhere in the crate |
| raw counters / lengths | `usize` | Convert signed C counts only if negative sentinel values are not semantically used |

### Naming Guidance for Anonymous Structures

Because the C analysis reports only anonymous data structures, create Rust names from actual usage in `src/symbol.c`, such as:

- function entry record
- parameter entry
- starter iterator state
- caller relation entry
- target-mark state

Do not introduce umbrella abstraction layers. Each Rust type should correspond to one concrete C record role.

### Ownership and Memory Management

- Replace manual lifetime management and node unlinking with Rust ownership.
- For structures that are repeatedly traversed and relinked, prefer:
  - `Vec<T>` storage
  - stable indices (`usize`) for links
  - `Option<usize>` for nullable links
- Use `Box<T>` linked nodes only if the C code’s insertion/removal behavior is strictly sequential and does not require random access to multiple aliases.
- Elimination passes should avoid invalidating active traversal state; if using `Vec`, perform two-phase marking then compaction, or preserve next indices before mutation.
- Avoid `Rc<RefCell<_>>` unless the C graph shape makes unique mutable access impossible in a straightforward indexed design.

### Error Handling

- C routines that assume valid internal state should map to infallible internal Rust functions when the surrounding call graph guarantees preconditions.
- Use `debug_assert!` for invariants discovered from C pointer assumptions.
- Use `Result` only for externally visible parsing/allocation/state-construction boundaries if those are already present in the Rust crate.
- Do not silently ignore structurally invalid states introduced during porting; fail fast in tests and internal checks.

## Implementation Phases

## Phase 1: Extract and Model Existing C State

### Goal

Create Rust data structures that exactly cover the records and links required by the six target functions, based on field-by-field analysis of `src/symbol.c`.

### Tasks

- Inspect `src/symbol.c` and enumerate the actual anonymous struct roles used by:
  - `collect_functions`
  - `move_parms`
  - `first_starter`
  - `next_starter`
  - `mark_callers`
  - `eliminate_non_targets`
- Define corresponding named Rust structs/enums in `src/symbol.rs`.
- Choose one link strategy per record family:
  - index-linked `Vec` storage preferred
  - boxed linked nodes only if simpler and behaviorally equivalent
- Map nullable C pointers to `Option`.
- Map flag fields to `bool` or enums where semantics are clear; otherwise keep integer fields initially to avoid accidental behavior changes.
- Add minimal constructors/helpers only when needed to keep translated functions readable.

### Deliverables

- Compiling Rust type definitions in `src/symbol.rs`
- Documented field mapping comments tied to original C roles
- No behavioral expansion beyond the C data model

## Phase 2: Port Collection and Starter Traversal

### Goal

Migrate the list-building and iteration logic first, establishing stable traversal behavior before graph marking and elimination.

### Tasks

- Port `collect_functions` with preserved insertion/order semantics.
- Port `move_parms`, taking special care with ownership transfer and link rewiring.
- Port `first_starter` and `next_starter` as a matched traversal pair sharing the same state model.
- Replace raw pointer stepping with:
  - index advancement through stored links, or
  - iterator-like state held explicitly by the caller/module
- Add unit tests for:
  - empty input
  - single-node lists
  - multi-node traversal order
  - parameter movement without loss or duplication

### Deliverables

- Working Rust implementations of four functions
- Tests proving starter enumeration and parameter relocation semantics

## Phase 3: Port Caller Marking and Target Elimination

### Goal

Translate the marking/filtering passes after the foundational list structures are verified.

### Tasks

- Port `mark_callers`, preserving exact marking propagation rules and mutation points.
- Port `eliminate_non_targets`, maintaining the same effective removal/filter behavior as the C implementation.
- Ensure traversal remains valid during removal:
  - precompute next link before mutation, or
  - use a mark-then-sweep pass with equivalent output ordering
- Add tests for:
  - reachable vs. non-reachable caller chains
  - repeated marking/idempotent behavior
  - elimination that removes head, middle, and tail elements
  - preservation of surviving node order

### Deliverables

- Working Rust implementations of `mark_callers` and `eliminate_non_targets`
- Regression coverage for marking and filtering passes

## Phase 4: Integrate, Validate, and Simplify Unsafe Assumptions

### Goal

Complete module integration and verify that the Rust port matches the original module behavior without retaining unnecessary C-style implementation artifacts.

### Tasks

- Integrate `src/symbol.rs` into the existing crate module tree.
- Align function signatures with current crate conventions while keeping logic unchanged.
- Review for remaining C-isms:
  - sentinel null handling
  - implicit initialization
  - alias-dependent mutation
- Replace any provisional integer/pointer-surrogate fields with stronger Rust types only where this does not alter behavior.
- Confirm all targeted functions are covered by `cargo test`.
- Add `debug_assert!` invariants for assumptions previously represented by C pointer discipline.

### Deliverables

- Fully integrated Rust module on branch `065-module_src_linked_list_entry_02-rust-port`
- Passing unit/regression tests via `cargo test`
- Final code focused strictly on migrated behavior from `src/symbol.c`