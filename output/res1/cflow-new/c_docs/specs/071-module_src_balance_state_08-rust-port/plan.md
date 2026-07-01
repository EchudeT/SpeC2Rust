# Implementation Plan: module_src_balance_state_08

## Summary

Port the balance-state support logic from `src/parser.c` into Rust on branch `071-module_src_balance_state_08-rust-port`, keeping scope limited to the existing parser-facing behavior embodied by:

- `push_balance_state`
- `pop_balance_state`
- `free_balance_stack`
- `find_closing_paren`

The Rust implementation should preserve the current operational model: maintain parser-local balancing state with explicit stack semantics and support parenthesis-matching scans without introducing new abstractions beyond what is needed to represent the existing C logic safely.

Technical approach:

- Migrate the relevant balance-state logic into a Rust parser module that remains close to the current source layout.
- Replace manual heap allocation and linked/manual stack ownership with standard-library owned containers such as `Vec`.
- Represent nullable and sentinel-driven C state with explicit Rust types (`Option`, enums, and owned structs).
- Keep function responsibilities aligned with the C originals rather than redesigning parser architecture.
- Convert C memory cleanup responsibilities into Rust drop-based ownership, retaining an explicit clear/free operation only where the original module behavior requires it.

## Technical Context

### Language / Version

- Rust 1.78 or newer

### Primary Dependencies

- Rust standard library only

Recommended crates: none.
There is no evidence in the source summary that third-party parsing, memory, or utility crates are required for this migration.

### Testing

- `cargo test`

Testing focus:

- stack push/pop sequencing
- empty-stack behavior
- cleanup/reset behavior equivalent to `free_balance_stack`
- parenthesis scan behavior for matched and unmatched input
- preservation of parser state transitions around nested balancing cases

### Performance Goals

- Preserve the C module’s effective linear behavior for stack operations and delimiter scans.
- `push_balance_state`: amortized O(1)
- `pop_balance_state`: O(1)
- `free_balance_stack`/stack clear: O(n) due to element drop
- `find_closing_paren`: O(n) over the scanned slice/input region
- Avoid unnecessary string or token copying during scans; operate on borrowed input where possible.
- Keep allocations bounded to stack growth already implied by nesting depth.

## Module Mapping

### Source File Mapping

| C Source | Rust Target | Notes |
|---|---|---|
| `src/parser.c` | `src/parser.rs` | Keep the ported balance-state functions within the parser module unless the existing Rust project already splits parser internals. |

### Function Mapping

| C Function | Rust Function / Method | Migration Notes |
|---|---|---|
| `push_balance_state` | `Parser::push_balance_state` or private helper in `parser.rs` | Implement as mutation of parser-owned stack; replace manual node allocation with `Vec::push`. |
| `pop_balance_state` | `Parser::pop_balance_state` or private helper in `parser.rs` | Return `Option<BalanceState>` or mutate parser state directly, depending on current call pattern. |
| `free_balance_stack` | `Parser::free_balance_stack` or `clear_balance_stack` | Implement via `Vec::clear` / resetting `Option`; keep explicit method only if parser flow expects a named cleanup step. |
| `find_closing_paren` | `find_closing_paren` helper or `Parser::find_closing_paren` | Implement as bounded scan over parser input/token stream while preserving original nesting semantics. |

### Scope Boundaries

This plan covers only migration of the balance-state stack handling and closing-parenthesis search logic currently residing in `src/parser.c`. It does not introduce:

- parser redesign
- new module splits for convenience alone
- concurrency support
- FFI wrappers
- generalized delimiter frameworks beyond the existing function needs

## Data Model

Because the analysis exposes only anonymous C data structures, the Rust mapping should be driven by observed field usage in the targeted functions rather than by speculative type design.

### Data-Structure Mapping Strategy

| C Data Shape | Rust Representation | Notes |
|---|---|---|
| anonymous parser-owned balance-state record | `struct BalanceState` | Name by role, based on fields read/written by push/pop logic. |
| anonymous stack node or linked-stack element | eliminated in favor of `Vec<BalanceState>` | Prefer contiguous owned stack unless the C code proves pointer-linked semantics are externally observed. |
| anonymous parser context fields used by these functions | existing `Parser` struct fields, adding `balance_stack: Vec<BalanceState>` if absent | Keep new fields minimal and local to the migrated functions. |
| anonymous temporary scan state in `find_closing_paren` | local variables and small enums if needed | Do not persist temporary scan state unless the C behavior requires it. |
| nullable pointers indicating absent state | `Option<T>` / empty `Vec` | Replace null checks with explicit absence handling. |
| ownership-dependent cleanup structures | standard ownership/drop | No manual free logic outside explicit reset method. |

### Expected Rust Types

The exact field list should be finalized after reading the C implementations, but the following pattern should guide the migration:

```rust
struct BalanceState {
    // fields copied directly from the C state actually used by push/pop
}

struct Parser {
    // existing parser fields...
    balance_stack: Vec<BalanceState>,
}
```

If `pop_balance_state` in C restores parser fields from the popped state rather than returning the state to callers, mirror that in Rust by making it a mutating method with `Option<()>` or a small result type instead of exposing internal stack entries unnecessarily.

### Memory Management Decisions

- Replace `malloc`/`free` stack management with `Vec<BalanceState>`.
- Use ownership to ensure balance-state entries are dropped automatically.
- If the C code stores borrowed references into parser-owned buffers, model them with Rust borrows only when lifetimes remain straightforward; otherwise store indices/offsets into the parser input to avoid self-referential structures.
- Convert explicit free-on-error branches into normal Rust early returns, ensuring stack state remains valid.

### Error Handling Decisions

- For stack underflow previously represented by null returns or no-op behavior, use `Option` or a small internal `Result` matching the original call expectations.
- For `find_closing_paren`, represent “not found” with `Option<usize>` or the parser’s existing error signaling convention.
- Do not invent richer public error hierarchies unless required by current parser APIs.

## Implementation Phases

## Phase 1: Inspect and Define Minimal Rust State

Goals:

- Read the exact C implementations in `src/parser.c` for the four target functions.
- Identify all parser fields and anonymous structures touched by these functions.
- Define the minimal Rust `BalanceState` representation and any parser field additions required.

Tasks:

- Trace all reads/writes in `push_balance_state` and `pop_balance_state`.
- Determine whether the C stack is linked, array-based, or embedded in parser state.
- Record whether `find_closing_paren` operates on raw characters, token stream positions, or parser offsets.
- Add/adjust the parser struct in `src/parser.rs` to hold the migrated stack state.

Exit criteria:

- Every anonymous C structure used by these functions has a named Rust counterpart or has been intentionally collapsed into parser fields/local variables.
- The Rust parser module compiles with placeholder implementations for the four functions.

## Phase 2: Port Stack Operations

Goals:

- Implement stack push/pop/free behavior in Rust with ownership-safe semantics.
- Preserve parser state restoration order and empty-stack behavior from C.

Tasks:

- Port `push_balance_state` into a parser method/helper using `Vec::push`.
- Port `pop_balance_state` using `Vec::pop`, restoring parser fields exactly as in the C flow.
- Port `free_balance_stack` as explicit stack clearing/reset logic where required by parser lifecycle.
- Translate any pointer-null and manual cleanup branches into `Option`-based logic.

Validation:

- Add unit tests for:
  - pushing one and multiple states
  - popping restores the most recent state first
  - popping an empty stack matches original behavior
  - clearing the stack resets parser-owned balance tracking

Exit criteria:

- Stack-related functions are fully implemented and tested.
- No manual memory management remains in the migrated logic.

## Phase 3: Port `find_closing_paren`

Goals:

- Recreate the original closing-parenthesis search semantics in Rust without changing parser-visible behavior.

Tasks:

- Port the scan loop faithfully, including nested parenthesis tracking and any stop conditions present in C.
- Use borrowed input slices or parser indices instead of copying input.
- Preserve the original unmatched/not-found handling convention.
- Integrate the helper into existing parser call sites within `src/parser.rs`.

Validation:

- Add tests covering:
  - simple matched pair
  - nested parentheses
  - unmatched open parenthesis
  - extra closing parenthesis handling if applicable to the C logic
  - boundary cases at start/end of input

Exit criteria:

- `find_closing_paren` matches the C behavior on representative parser inputs.
- All targeted functions are wired into the Rust parser module.

## Phase 4: Final Integration and Cleanup

Goals:

- Remove temporary migration scaffolding and verify the module is stable within the Rust parser.

Tasks:

- Replace placeholders/todo branches introduced during earlier phases.
- Align function visibility with actual usage (`pub(crate)` or private as appropriate).
- Ensure naming reflects parser-internal roles while preserving a clear mapping to the C originals.
- Run `cargo test` and fix any borrow, lifetime, or state-reset issues discovered in integration.

Validation:

- Full test suite passes with the migrated balance-state logic enabled.
- The Rust implementation remains limited to the original module responsibilities.

## Notes on Migration Restraint

- Keep the implementation in `src/parser.rs` unless the current Rust codebase already has an established equivalent split.
- Do not create generic stack utilities or delimiter-matching frameworks.
- Name Rust structs after observed roles only where needed to replace anonymous C data.
- Prefer direct mechanical migration plus safety-oriented type changes over architectural refactoring.