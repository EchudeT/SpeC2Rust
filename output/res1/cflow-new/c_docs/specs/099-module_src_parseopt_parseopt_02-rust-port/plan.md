# Implementation Plan

## Summary

Port `src/parseopt/parseopt.c` into an idiomatic Rust module that preserves the existing option-parsing behavior, state transitions, and argument permutation logic without adding new parsing features. The Rust implementation should migrate the current file as a focused parser component, keeping the original control flow recognizable where useful so behavior can be validated function-by-function during the port.

The technical approach is to:
- move the parser state and option-definition handling into explicit Rust structs and enums,
- replace pointer/index arithmetic with slice- and index-based traversal over argument vectors,
- encode parse outcomes and parser errors using Rust enums and `Result` where internal clarity helps, while preserving external behavior expected by the surrounding project,
- port helper routines first, then the main iteration path, then usage/help preparation functions.

This module should remain a single Rust module corresponding closely to the original C source, with only minimal internal type decomposition needed to express the existing logic safely.

## Technical Context

- **Language/Version**: Rust 1.76+
- **Primary Dependencies**:
  - Rust standard library only
  - No third-party crates are recommended from the available evidence
- **Testing**:
  - `cargo test`
  - module-focused unit tests covering option lookup, long/short matching, permutation, lookahead/skip, and sequential parse behavior
- **Performance Goals**:
  - preserve linear scan behavior and similar asymptotic cost to the C implementation
  - avoid unnecessary allocation during iterative parsing
  - keep argument permutation in-place where feasible via `Vec` operations or index-driven mutation
  - maintain low-overhead option lookup consistent with the original implementation rather than introducing heavier abstractions

## Module Mapping

### Source File Mapping

| C Source | Rust Target |
|---|---|
| `src/parseopt/parseopt.c` | `src/parseopt.rs` or `src/parseopt/mod.rs` |

Use whichever location matches the existing Rust crate layout, but keep the implementation concentrated in one module corresponding to the original file.

### Function Mapping

| C Function | Rust Target | Notes |
|---|---|---|
| `option_find_short` | `fn option_find_short(...) -> ...` | Internal helper for short option definition lookup |
| `negmatch` | `fn negmatch(...) -> bool` | Internal helper preserving original string-match semantics |
| `option_find_long` | `fn option_find_long(...) -> ...` | Internal helper for long option lookup |
| `permute` | `fn permute(...)` | Internal state/argv reordering helper |
| `parseopt_lookahead` | `pub(crate) fn parseopt_lookahead(...) -> ...` | Keep visibility limited unless broader crate use requires `pub` |
| `parseopt_skip` | `pub(crate) fn parseopt_skip(...)` | Parser state advancement helper |
| `parseopt_next_internal` | `fn parseopt_next_internal(...) -> ...` | Core parser engine |
| `parseopt_next` | `pub fn parseopt_next(...) -> ...` | Public iteration entry point |
| `parseopt_argv` | `pub fn parseopt_argv(...) -> ...` | Public argv-oriented setup/accessor function |
| `parseopt_error` | `pub fn parseopt_error(...) -> ...` | Error text/reporting adapter |
| `optidx_slot` | `fn optidx_slot(...) -> ...` | Internal option-index bookkeeping helper |
| `collect_optdef` | `fn collect_optdef(...) -> ...` | Internal option-definition aggregation |
| `prepare_optdef` | `fn prepare_optdef(...) -> ...` | Internal option-definition preparation |
| `set_help` | `fn set_help(...)` | Internal mutation of prepared definitions/messages |
| `set_usage` | `fn set_usage(...)` | Internal mutation of prepared definitions/messages |

## Data Model

Because the input only identifies anonymous C data structures, the Rust plan should derive named internal types directly from usage sites in `parseopt.c` rather than preserving anonymous layout mechanically. The goal is to create a small set of explicit Rust types that cover parser state, option definitions, parse events, and error state.

### Data Structure Mapping Strategy

| C Shape | Rust Type | Mapping Decision |
|---|---|---|
| anonymous parser state struct | `struct ParseOptState` | Holds argv data, current index, short-option cursor, permutation bounds, and parse status |
| anonymous option definition entry | `struct OptDef` | Represents one option's short/long names, flags, argument mode, and destination metadata |
| anonymous prepared option table | `struct PreparedOptDef` or `Vec<OptDef>` | Use owned/vector-backed storage for prepared definitions |
| anonymous lookup/index slot | `struct OptIdxSlot` or `Option<usize>` | Prefer `Option<usize>` unless the C logic requires extra metadata |
| anonymous parse result record | `enum ParseEvent` / `struct ParseItem` | Encodes what `parseopt_next*` yields to callers |
| anonymous error carrier | `enum ParseOptError` | Replaces integer/error-string combinations internally |
| anonymous help/usage storage | `struct HelpUsageText` or fields on parser/options struct | Keep colocated with prepared definitions if only used there |
| anonymous temporary matching state | local variables / small private structs | Do not preserve standalone types unless reused |

### Recommended Rust Types

#### Parser State
Use a dedicated mutable state struct rather than free-form mutable parameters:

```rust
struct ParseOptState {
    argv: Vec<String>,
    idx: usize,
    short_pos: usize,
    nonopt_start: usize,
    nonopt_end: usize,
    parsing_done: bool,
    // prepared option definitions and message fields
}
```

If ownership of `argv` must remain external, use borrowed forms only if lifetimes stay simple. Prefer owned `Vec<String>` if the C code mutates argument order and the surrounding Rust code can pass ownership cleanly.

#### Option Definitions
Represent option definitions with explicit optional names and argument expectations:

```rust
struct OptDef {
    short: Option<char>,
    long: Option<String>,
    takes_argument: ArgMode,
    // any flags or identifiers needed by existing behavior
}
```

```rust
enum ArgMode {
    None,
    Required,
    Optional,
}
```

If the original module uses integer IDs or output slots, keep them as fields rather than redesigning the API.

#### Parse Outcomes
For the iterative parser entry points, use a compact result model:

```rust
enum ParseEvent {
    Option { /* matched option data */ },
    Argument(String),
    End,
}
```

If the existing caller contract expects integer codes plus mutable output fields, keep that external signature and use `ParseEvent` only internally as a migration aid.

#### Error Handling
Map parse failures to an internal enum and convert to caller-visible messages at the module boundary:

```rust
enum ParseOptError {
    UnknownShort(char),
    UnknownLong(String),
    MissingArgument,
    AmbiguousLong(String),
    InvalidState,
}
```

`parseopt_error` should format or expose these errors in the same shape the rest of the project expects. Avoid panics for parse failures; reserve panics for impossible internal invariant violations only.

### Memory Management Notes

- Replace C string pointers with `String`/`&str`.
- Replace raw arrays and manual counts with `Vec<T>`.
- Replace nullable pointers with `Option<T>` or `Option<usize>`.
- Avoid self-referential borrowed data structures; prefer owned option-definition strings during preparation.
- Keep mutation localized in `ParseOptState` to make permutation and scan progress explicit and safe.

## Implementation Phases

## Phase 1: Establish Rust Module Skeleton and Core Types

Create the Rust module corresponding to `parseopt.c` and define the minimal data structures required to express current behavior:
- `ParseOptState`
- `OptDef`
- `ArgMode`
- internal parse/error enums as needed

Port the option-definition preparation path first:
- `optidx_slot`
- `collect_optdef`
- `prepare_optdef`
- `set_help`
- `set_usage`

Implementation notes:
- keep field names close to the original C concepts for easier comparison
- use `Vec<OptDef>` and `Option` in place of manual slot/pointer management
- preserve any original ordering guarantees from option collection/preparation
- add focused unit tests for definition preparation and message field assignment

## Phase 2: Port Option Lookup and Argument-Reordering Helpers

Port the internal helper logic that the main parser depends on:
- `option_find_short`
- `negmatch`
- `option_find_long`
- `permute`
- `parseopt_lookahead`
- `parseopt_skip`

Implementation notes:
- translate string matching carefully, especially any prefix/negation rules in long-option handling
- express permutation with safe indexed swaps/rotations over `Vec<String>`
- verify boundary conditions for empty argv segments, `--`, clustered short options, and long-option prefixes
- add unit tests directly mirroring helper behavior from the C logic

## Phase 3: Port Core Iteration and Public Entry Points

Port the main parsing flow:
- `parseopt_next_internal`
- `parseopt_next`
- `parseopt_argv`
- `parseopt_error`

Implementation notes:
- preserve the original state-machine order of checks to avoid subtle behavioral drift
- keep internal helpers private unless required by current crate usage
- convert internal error/state results into the public return shape expected by callers
- ensure parser progress is monotonic except for intended permutation effects
- add sequential tests for mixed short/long options, attached and separate option arguments, non-option arguments, end-of-options marker handling, and error cases

## Phase 4: Behavioral Validation and Cleanup

Validate the full Rust module against the original C behavior and simplify only where equivalence is clear.

Tasks:
- compare function-by-function outcomes against the C implementation for representative argv inputs
- remove temporary migration scaffolding not needed by final callers
- tighten signatures and visibility to crate-local/public only where actually used
- confirm no unnecessary allocations were introduced in hot parsing paths
- finalize documentation comments describing state ownership, mutation points, and parse error semantics

Acceptance for this phase:
- all module tests pass under `cargo test`
- the Rust module fully replaces the C file’s responsibilities for this parser component
- no extra parser features or supporting subsystems are introduced