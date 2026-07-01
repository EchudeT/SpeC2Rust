# Implementation Plan: module_src_parseopt_parseopt_02

## Summary

This module ports the command-line option parsing logic from `src/parseopt/parseopt.c` into Rust, preserving existing parsing behavior, option lookup rules, argument permutation, help/usage preparation, and parser state transitions.

The Rust implementation should remain a direct migration of the existing C module rather than a redesign. The technical approach is to:

- translate the parser state and option-definition data into explicit Rust structs and enums,
- replace pointer-based traversal with slice indexing and iterator-like state fields,
- model parser errors with a small internal error enum while preserving current outward behavior,
- keep mutation localized to the parser state object for routines such as lookahead, skip, permutation, and next-option advancement,
- implement the functions in migration order around the existing parsing flow: option definition preparation, lookup helpers, parser stepping, then help/usage setters.

The implementation should prefer standard library facilities (`Vec`, `String`, `Option`, `Result`, slice operations) and avoid introducing new abstraction layers beyond what is needed to represent the existing C logic safely.

## Technical Context

- **Language/Version**: Rust 1.78+ stable
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve near-linear parsing behavior over `argv` and option-definition arrays.
  - Avoid unnecessary string allocations during token scanning and option matching.
  - Keep argument permutation in-place within owned vectors/slices where possible.
  - Match C behavior closely enough that parser throughput remains dominated by command-line size, not conversion overhead.

## Technical Context Details

### Project Placement

Use standard Rust project conventions and place the migrated module in the existing crate under a path aligned with the source module, for example:

- `src/parseopt.rs` if the crate uses flat module files, or
- `src/parseopt/mod.rs` if `parseopt` is already a directory-based module.

Do not split this migration into multiple new helper modules unless the current Rust crate layout already requires it.

### Recommended Rust Design Constraints

- Represent parser state as a mutable struct instead of passing raw pointers and counters.
- Accept command-line data as borrowed string slices where possible; use owned `String` only when the original C logic stores generated help/usage text.
- Use `usize` for indexes and lengths.
- Use `Option<usize>` or equivalent for absent index slots instead of sentinel integers.
- Use `Result` only for internal error propagation where it simplifies translation; if the original API reports errors through parser state or return codes, preserve that outward shape.

## Module Mapping

### Source Mapping

- **C source file**: `src/parseopt/parseopt.c`
- **Rust destination**: `src/parseopt.rs` or `src/parseopt/mod.rs`

### Function Mapping

Each C function should map to a same-purpose Rust function or associated method, keeping names close to the original where practical to ease review:

| C Function | Rust Mapping | Notes |
|---|---|---|
| `option_find_short` | `fn option_find_short(...)` or parser-associated helper | Scan option definitions for short-name match. |
| `negmatch` | `fn negmatch(...)` | Private helper for long-option negation/prefix matching logic. |
| `option_find_long` | `fn option_find_long(...)` | Scan option definitions for long-name match. |
| `permute` | `fn permute(...)` | Internal mutation of argument ordering/state; keep private. |
| `parseopt_lookahead` | `fn parseopt_lookahead(...)` or `Parser::lookahead` | Read-only/limited-mutation parser state check. |
| `parseopt_skip` | `fn parseopt_skip(...)` or `Parser::skip` | Advance parser position consistently with C logic. |
| `parseopt_next_internal` | `fn parseopt_next_internal(...)` or `Parser::next_internal` | Core parser step routine. |
| `parseopt_next` | `pub fn parseopt_next(...)` or `Parser::next` | Public-facing next-option entry point. |
| `parseopt_argv` | `pub fn parseopt_argv(...)` | Initialize/parse from argv collection. |
| `parseopt_error` | `fn parseopt_error(...)` | Centralize error message/state update. |
| `optidx_slot` | `fn optidx_slot(...)` | Index lookup helper for option definitions. |
| `collect_optdef` | `fn collect_optdef(...)` | Build internal option-definition table. |
| `prepare_optdef` | `fn prepare_optdef(...)` | Normalize option definitions before parsing. |
| `set_help` | `fn set_help(...)` | Assign prepared help text/state. |
| `set_usage` | `fn set_usage(...)` | Assign prepared usage text/state. |

### API Shape Guidance

Prefer grouping stateful functions as methods on a parser struct if the C code operates on a shared parser object. Keep stateless helpers as private free functions if that more closely mirrors the original data flow. The goal is traceable migration, not API redesign.

## Data Model

The input lists anonymous C data structures rather than named structs, so the Rust mapping should be derived from actual field usage in `parseopt.c`. During migration, define only the minimum set of structs/enums needed to represent the existing parser state and option-definition storage.

### Expected Rust Data Structures

| C Data Shape | Rust Mapping | Migration Notes |
|---|---|---|
| Parser state struct | `struct ParseOptState` | Holds argv storage, current index, current short-option position, parsing mode flags, error/help/usage state, and prepared option definitions. |
| Option definition record | `struct OptDef` | Holds short name, long name, argument expectation, identifier/index, and any flags used by lookup logic. |
| Option index slot / lookup entry | `struct OptIdxSlot` or `Option<usize>` in vectors | Use explicit option index mapping instead of pointer arithmetic or sentinel values. |
| Return/status codes | `enum ParseStep` / integer-compatible enum | Represent next-token results, end-of-input, positional argument, and error conditions. |
| Error state/message storage | `enum ParseOptError` plus optional message field | Preserve original emitted error text behavior without unsafe buffers. |
| Help/usage text storage | `Option<String>` | Use owned strings only if text is synthesized or retained beyond call scope. |
| Temporary matching state | local enums/tuples | Keep matching-specific state local unless the C code persists it. |

### C-to-Rust Structure Translation Rules

Because the C analysis only reports anonymous structures, the implementation phase should first identify concrete field groups in `parseopt.c` and map them as follows:

- **Raw C strings (`char *`, `const char *`)**
  - Use `&str` for borrowed immutable inputs when lifetimes are straightforward.
  - Use `String` for stored/generated help and usage text.
  - Use `Option<&str>` or `Option<String>` for nullable text fields.

- **Arrays and pointer spans**
  - Use `Vec<T>` for owned collections.
  - Use slices (`&[T]`, `&mut [T]`) for borrowed views.
  - Replace pointer walking with index-based traversal.

- **Integer flags / booleans**
  - Use `bool` for binary flags.
  - Use small enums for multi-state parse modes instead of ad hoc integer constants where the mapping is direct and does not alter behavior.

- **Nullable references to definitions or argv entries**
  - Use `Option<usize>` for indexes into collections.
  - Use `Option<T>` for optional stored values.

### Memory Management Notes

- Eliminate manual ownership tracking from C by storing parser-owned mutable state inside Rust structs.
- Avoid cloning argv strings unless permutation or retained ownership requires it.
- If the parser must reorder argv entries like the C implementation, store argv in a mutable owned `Vec<String>` or `Vec<OsString>` only if required by the existing Rust crate conventions; otherwise prefer borrowed slices plus an index-permutation representation.
- Keep all temporary matching logic stack-local to avoid hidden allocations.

### Error Handling Notes

- Centralize parse errors in a dedicated helper matching `parseopt_error`.
- Preserve existing error timing and message formatting as closely as possible.
- Do not convert the module into a broad exception-style API; keep return values aligned with the existing parser contract.

## Implementation Phases

## Phase 1: Extract and Model Existing Parser State

### Goal
Create the Rust data structures and signatures needed to host a direct translation of `parseopt.c`.

### Tasks
- Inspect `src/parseopt/parseopt.c` and identify each anonymous structure’s actual field usage.
- Define Rust structs/enums for:
  - parser state,
  - option definitions,
  - lookup/index slots,
  - parse result/status,
  - internal error state.
- Create the Rust module file at the mapped destination.
- Add function stubs for all migrated functions with names close to the C originals.
- Decide, based on the C call graph, which functions should become associated methods on the parser state and which remain private helpers.

### Acceptance Criteria
- The Rust module compiles with placeholder bodies.
- All C functions in scope have a clear Rust destination.
- Anonymous C data layouts are replaced by named Rust types with documented field-to-field mapping comments.

## Phase 2: Port Option Definition Preparation and Lookup Helpers

### Goal
Migrate the definition-building and lookup layer that the parser core depends on.

### Tasks
- Implement `optidx_slot`.
- Implement `collect_optdef`.
- Implement `prepare_optdef`.
- Implement `option_find_short`.
- Implement `negmatch`.
- Implement `option_find_long`.
- Implement `set_help`.
- Implement `set_usage`.

### Technical Notes
- Keep lookup behavior faithful to C, including prefix, negation, or ambiguity handling used by long-option matching.
- Replace raw pointer comparisons with string/slice comparisons.
- If the C logic constructs intermediate tables, preserve their shape in Rust rather than redesigning into maps unless the original code clearly uses indexable arrays and Rust vectors map directly.

### Acceptance Criteria
- Prepared option definitions can be built from the same source data expected by the C parser.
- Short and long option lookup behavior is covered by unit tests derived from the C logic.
- Help and usage state are stored safely without lifetime issues.

## Phase 3: Port Core Parser Advancement Logic

### Goal
Translate the stateful option-scanning behavior used during command-line traversal.

### Tasks
- Implement `permute`.
- Implement `parseopt_lookahead`.
- Implement `parseopt_skip`.
- Implement `parseopt_error`.
- Implement `parseopt_next_internal`.
- Implement `parseopt_next`.

### Technical Notes
- Preserve the original sequencing of:
  - current argument inspection,
  - short/long option dispatch,
  - attached vs separate option arguments,
  - non-option handling,
  - permutation behavior,
  - end-of-options detection.
- Translate pointer arithmetic into explicit index updates.
- Keep state mutations narrow and reviewable; this phase is the most behavior-sensitive part of the port.
- Where C uses sentinel return codes, mirror them with Rust enums internally and convert only at the public boundary if needed.

### Acceptance Criteria
- The parser can iterate through argv inputs with behavior aligned to the C implementation.
- Error paths and skip/lookahead transitions are exercised by tests.
- Argument permutation behavior matches the original ordering rules.

## Phase 4: Integrate Entry Point and Validate Behavior

### Goal
Complete the public entry path and verify parity of the migrated module.

### Tasks
- Implement `parseopt_argv`.
- Wire the module into the Rust crate on branch `099-module_src_parseopt_parseopt_02-rust-port`.
- Add focused unit tests for:
  - short option parsing,
  - long option parsing,
  - negated/prefix long-option cases handled by `negmatch`,
  - missing argument errors,
  - permutation and skip behavior,
  - help/usage preparation state.
- Run `cargo test` and fix parity issues revealed during integration.

### Acceptance Criteria
- The Rust module is fully wired into the crate.
- All migrated functions are implemented.
- Tests cover the main parser control-flow cases from the C module.
- `cargo test` passes.

## Validation Strategy

- Use the C source as the behavior reference for unit-test expectations.
- Prefer case-based tests around individual migrated helpers before end-to-end parser tests.
- Validate edge cases involving:
  - clustered short options,
  - `--` end-of-options handling,
  - long-option ambiguity or negation matching,
  - positional argument preservation after permutation,
  - repeated calls to next/skip/lookahead on the same parser state.

## Out of Scope

The following should not be added in this migration plan:

- new command-line parsing features,
- API redesign beyond what is required for safe Rust representation,
- thread-safety wrappers,
- FFI layers,
- serialization support,
- benchmark harnesses,
- release or deployment work,
- recovery mechanisms beyond the existing parser behavior.