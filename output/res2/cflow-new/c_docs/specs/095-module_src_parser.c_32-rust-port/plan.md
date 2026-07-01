# Implementation Plan

## Summary

Port `src/parser.c` into an idiomatic Rust module with a narrow migration scope centered on the existing parser-local functionality represented by `reference` and `reset_static_caller`. The Rust implementation should preserve current control flow and state behavior while replacing C-style global/static mutation and nullable pointer handling with explicit Rust ownership and mutable module state.

The implementation approach is:

- create a Rust parser module that contains only the migrated equivalents needed for this C module segment;
- translate the parser state and anonymous C data carriers into named Rust structs with minimal field reshaping;
- convert static mutable state used by `reset_static_caller` into explicit internal state managed through `Option`, slices, and owned values as appropriate;
- keep behavior aligned with the C implementation rather than redesigning parser architecture;
- use standard-library collections and borrowing rules to replace raw pointer traversal and manual lifetime management.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - preserve the existing asymptotic behavior of the C implementation;
  - avoid unnecessary heap allocation beyond what is required to replace C dynamic/manual storage;
  - keep parsing-state updates and reference resolution on the same hot-path complexity as the original code;
  - prefer borrowed string/slice views where practical, but use owned data when required to match C lifetime behavior safely.

## Module Mapping

| C Source | Rust Target | Notes |
|---|---|---|
| `src/parser.c` | `src/parser.rs` | Primary migration target for this module cluster. |
| `reference` | `parser::reference` | Keep function scope and behavior aligned with C logic; adjust signature to Rust references and result/option types as needed. |
| `reset_static_caller` | `parser::reset_static_caller` | Replace static mutable C state manipulation with explicit mutable parser-local/module-local state reset. |

If the current Rust crate already exposes parser functionality through `src/lib.rs` or `src/main.rs`, only add the minimal `mod parser;` / `pub(crate)` wiring required to reach the migrated functions.

## Data Model

The analysis identifies only anonymous C data structures. Since names are unavailable, the Rust plan should introduce local, purpose-based names derived from actual field usage during porting, without inventing extra abstractions.

| C Data Structure | Rust Mapping | Migration Notes |
|---|---|---|
| anonymous struct #1 | `struct ParserState` | Candidate for parser-wide mutable state if fields are shared by both migrated functions. |
| anonymous struct #2 | `struct ReferenceContext` | Use when `reference` operates on a grouped set of inputs/state. |
| anonymous struct #3 | `struct CallerState` | Use for state reset by `reset_static_caller`. |
| anonymous struct #4 | `struct TokenView` | Use if the C code groups token pointers/offsets. |
| anonymous struct #5 | `struct SourceSpan` | Use if the C code stores positional ranges or offsets. |
| anonymous struct #6 | `struct NameRef` | Use if the C code holds identifier/reference data. |
| anonymous struct #7 | `enum ReferenceKind` | Use only if C tag/flag fields encode mutually exclusive variants. |
| anonymous struct #8 | `struct ParseCursor` | Use if pointer-walking state must be preserved. |
| anonymous struct #9 | `struct LookupEntry` | Use if `reference` traverses table/list entries. |
| anonymous struct #10 | `struct StaticCallerSlot` | Use if a dedicated static record is being reset. |
| anonymous struct #11 | `struct ParserScratch` | Use only for temporary grouped values that exist as a C helper struct. |

### C-to-Rust Mapping Rules

- **Raw pointers** -> `&T`, `&mut T`, `Option<&T>`, `Option<&mut T>`, or owned structs depending on lifetime and mutation needs.
- **Nullable object pointers** -> `Option<T>` / `Option<&T>`.
- **C strings** -> `String` for owned storage, `&str` for borrowed inputs after validation.
- **Pointer-linked lists / arrays** -> `Vec<T>` or slices if contiguous ownership is clear from the C code.
- **Integral flags** -> `u32`/`usize` initially; refine to enums/bitflags-like constants only if directly supported by observed C usage.
- **Static mutable storage** -> fields on a dedicated state struct, reset through `&mut self` or `&mut ParserState`.
- **Manual memory cleanup** -> automatic drop; if C relies on borrowed external buffers, model them with lifetimes instead of copying when feasible.

## Implementation Phases

### Phase 1: Establish Rust module skeleton and state inventory

- Create or update `src/parser.rs` as the migration target for this module.
- Inspect `src/parser.c` and isolate the exact code and local helpers used by:
  - `reference`
  - `reset_static_caller`
- Identify all anonymous structs, static variables, macros, and typedef-like patterns touched by these functions.
- Assign minimal Rust type names based strictly on usage.
- Define Rust struct/enum skeletons and function signatures with placeholder bodies.
- Determine whether the C static state belongs in:
  - a parser state struct passed explicitly, or
  - a private module-local state object with controlled mutable access.
- Keep exported visibility minimal (`pub(crate)` unless wider access is already required by the crate).

### Phase 2: Port data structures and state reset behavior

- Translate the C storage used by `reset_static_caller` into Rust fields.
- Replace any zeroing/reset patterns from C with explicit Rust reassignment:
  - `Option::None`
  - empty `Vec`
  - zero/false defaults
  - `Default` implementations where they directly match C initialization semantics
- Implement `reset_static_caller` first, since it defines the baseline state invariants required by `reference`.
- Add focused unit tests for reset behavior:
  - state returns to expected empty/default values;
  - repeated reset calls are harmless;
  - no stale references remain reachable after reset.

### Phase 3: Port `reference` logic with explicit ownership and error paths

- Translate control flow from C into Rust in near-structural form to reduce migration risk.
- Replace pointer/null checks with `Option` matching.
- Replace out-parameters and sentinel returns with the narrowest Rust equivalent consistent with caller usage:
  - plain return value if the C function always succeeds,
  - `Option<T>` for not-found/absent outcomes,
  - `Result<T, E>` only if the original function has a real error branch that callers distinguish.
- Preserve lookup/update order exactly where parser behavior may depend on it.
- Avoid speculative refactoring of nested conditions until behavior is validated.
- Add unit tests covering:
  - successful reference handling;
  - empty/uninitialized caller state interaction;
  - boundary cases previously represented by null pointers or zero-length input.

### Phase 4: Integrate, validate, and remove C-centric assumptions

- Wire the migrated functions into the existing Rust crate call sites, if present.
- Remove any remaining C idioms that survived initial translation but are unnecessary in Rust, provided behavior does not change.
- Confirm all anonymous-structure mappings are consolidated into the smallest necessary set of Rust types.
- Run `cargo test` and fix borrow/lifetime issues by tightening scopes rather than introducing indirection.
- Review for:
  - accidental copies of parser state,
  - hidden allocation in tight paths,
  - misuse of `String` where `&str` or slices suffice,
  - any unsafe code; eliminate it unless a direct C memory layout dependency is proven unavoidable.