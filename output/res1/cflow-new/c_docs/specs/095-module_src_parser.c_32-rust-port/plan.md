# Implementation Plan: module_src_parser.c_32

## Summary

This plan ports the logic currently contained in `src/parser.c` for the functions `reference` and `reset_static_caller` into an idiomatic but behavior-preserving Rust module within the `cflow-new` project on branch `095-module_src_parser.c_32-rust-port`.

The Rust implementation should keep scope tightly aligned with the existing C file and function boundaries. The migration approach is:

- translate file-local state and function logic from `src/parser.c` into a single Rust source module;
- replace raw C memory patterns with explicit Rust ownership and borrowing where possible;
- preserve observable behavior, especially around parser-side state updates and any static caller reset semantics;
- represent anonymous C data shapes with locally scoped Rust structs/enums named by role discovered during migration, without introducing new abstraction layers beyond what is needed to compile and preserve behavior.

Special attention should be given to:

- converting static mutable C state into controlled Rust module state;
- handling nullable references and pointer-linked structures via `Option`, references, or owned containers as appropriate;
- preserving call ordering and mutation semantics in `reference` and `reset_static_caller`;
- minimizing architectural expansion beyond the direct port.

## Technical Context

### Language/Version

- Rust stable, edition 2021
- Minimum recommended toolchain: `rustc 1.76+`

### Primary Dependencies

Use the Rust standard library by default.

Recommended crates: none unless the existing Rust workspace already provides a parser-support crate that this module must match. Based on the available input, no third-party dependency is required for this port.

### Testing

- `cargo test`

Testing should cover:

- direct behavior of `reference`;
- reset behavior of `reset_static_caller`;
- state transitions before and after reset;
- null/empty/input-edge cases equivalent to C pointer checks.

### Performance Goals

- Preserve the current asymptotic behavior of the C implementation.
- Avoid unnecessary heap allocation beyond what is required to model existing C-owned structures safely.
- Keep parser-state mutation and lookup operations at constant or equivalent cost relative to the C version where feasible.
- Do not introduce synchronization or indirection that would materially slow the hot path.

## Module Mapping

### C to Rust File Mapping

- `src/parser.c` -> `src/parser.rs`

If the current Rust crate already uses a different file layout for parser code, place the migrated implementation in the existing parser module file rather than splitting it further.

### Function Mapping

- `reference` -> `pub(crate)` or private Rust function `reference`
- `reset_static_caller` -> `pub(crate)` or private Rust function `reset_static_caller`

Visibility should be restricted to the smallest scope compatible with current call sites. Do not widen API exposure unless existing integration requires it.

### State Mapping

Any C file-static state used by these functions should move into one of the following Rust forms, chosen strictly by actual usage:

- plain module-local struct state owned by a parser context object, if the C static is effectively per-parser state;
- `static` immutable data, if read-only;
- internal mutable state encapsulated behind a single module-owned object, if mutation must remain shared.

Avoid global `static mut`. Prefer passing mutable context explicitly if the surrounding Rust port already follows that pattern.

## Data Model

Because the analysis only reports multiple anonymous C data structures, the Rust data model should be finalized during implementation by naming each structure according to its role in `reference` and `reset_static_caller`.

### Mapping Rules

For each anonymous C struct/union used by these functions:

- anonymous C record carrying parser state -> named Rust `struct` such as `ParserState`
- anonymous C record carrying caller/reference tracking -> named Rust `struct` such as `CallerState` or `ReferenceRecord`
- anonymous linked node -> named Rust `struct` with `Option<Box<Node>>` or index-based storage, depending on actual ownership
- anonymous tagged state -> Rust `enum` if C logic branches on a discriminator
- nullable pointer field -> `Option<T>`, `Option<Box<T>>`, or `Option<NonNull<T>>` only if aliasing/lifetime constraints require it
- borrowed non-owning relationship -> reference-based access where lifetimes remain local and simple
- C strings (`char *`) -> `String` if owned, `&str` if borrowed input, `OsString`/`PathBuf` only if the field is clearly path-oriented
- C integer flags -> integer type or small Rust enum/bit-pattern type matching actual use
- resettable static slot -> `Option<...>` field in a containing state struct

### Expected Rust Type Conventions

Use exact-width integer types only when C interoperability or overflow-sensitive behavior requires them; otherwise:

- counters/lengths -> `usize`
- parser status codes -> `Result` where the original function already indicates failure paths, otherwise preserve side-effect-only signature
- booleans/flags -> `bool` when the C code uses logical conditions rather than bitsets

### Memory Management Notes

- Replace manual lifetime management with ownership in container fields.
- Where the C implementation keeps non-owning backreferences or shared mutable links, prefer stable indices or scoped mutable borrows over reference-counted pointers.
- Use `Box` only for recursive or heap-resident structures already implicit in the C design.
- Do not add arenas, interning, or custom allocators unless they are already required elsewhere in the Rust project.

### Error Handling Notes

For these functions:

- preserve no-fail signatures if the C logic is side-effect-only and assumes valid state;
- introduce `Result` only where the translated code must represent existing failure behavior explicitly;
- avoid `unwrap` on data derived from translated C pointer checks;
- map null-pointer and missing-state conditions to `Option` handling with early return or explicit no-op where that matches current semantics.

## Implementation Phases

## Phase 1: Establish Rust parser module skeleton

Scope:

- create or update `src/parser.rs`;
- identify the exact C-local state and anonymous structures touched by `reference` and `reset_static_caller`;
- define minimal Rust structs/enums needed to represent those shapes;
- map any file-static C variables involved in these functions into Rust module or parser-context state.

Deliverables:

- compilable Rust module skeleton;
- placeholder or translated type definitions for all data touched by the two target functions;
- comments or internal notes tying each Rust type back to the corresponding anonymous C role.

Exit criteria:

- all data dependencies of `reference` and `reset_static_caller` are represented in Rust;
- no use of `static mut`;
- ownership model for resettable/shared state is decided.

## Phase 2: Port `reset_static_caller`

Scope:

- translate `reset_static_caller` first because it defines the baseline mutation/reset semantics for parser caller state;
- implement exact reset behavior on the Rust state container;
- preserve no-op behavior for absent/uninitialized state if that exists in C.

Technical focus:

- convert pointer nulling/reset logic into `Option::None`, default-value restoration, or field reassignment;
- ensure no stale borrows or aliasing remain after reset;
- keep visibility restricted to current module usage.

Testing focus:

- reset from initialized state;
- repeated reset calls;
- reset from empty/uninitialized state.

Exit criteria:

- `reset_static_caller` behavior is covered by unit tests;
- state after reset matches the C semantics.

## Phase 3: Port `reference`

Scope:

- translate the `reference` function using the Rust data model established earlier;
- preserve sequencing of reads/writes to parser state and caller/reference tracking structures;
- resolve any pointer traversal or conditional branches directly into idiomatic Rust control flow without changing behavior.

Technical focus:

- replace null checks with `Option` matching;
- model any linked or nested structure updates safely;
- maintain equivalent mutation visibility to surrounding parser logic.

Testing focus:

- normal reference creation/update path;
- edge cases involving missing caller/current context;
- interactions with `reset_static_caller`;
- regression tests for state consistency after multiple calls.

Exit criteria:

- `reference` compiles and passes behavior-oriented tests;
- all unsafe code is avoided unless a specific C aliasing pattern makes it unavoidable.

## Phase 4: Integration cleanup and parity review

Scope:

- align signatures and module visibility with existing Rust call sites;
- remove temporary migration scaffolding;
- verify that the final Rust file remains limited to the original C module responsibilities.

Technical focus:

- rename temporary anonymous-type placeholders to stable role-based names;
- simplify any overly broad ownership choices discovered during translation;
- confirm no extra helper modules or generalized parser frameworks were introduced.

Validation:

- run `cargo test`;
- review translated logic against the original C functions for state-reset and reference-update parity.

Exit criteria:

- Rust implementation is integrated into the existing crate structure;
- tests pass;
- port remains narrowly scoped to `src/parser.c`, `reference`, and `reset_static_caller`.