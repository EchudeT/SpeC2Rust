# Implementation Plan

## Summary

This module ports the helper and utility portions of `src/wordsplit/wordsplit.c` that support word-splitting, quoting, numeric decoding, delimiter scanning, and append/error routines. The Rust implementation should stay tightly aligned with the existing C file’s behavior and migration boundaries, preserving parsing order and observable return semantics rather than redesigning the subsystem.

Technical approach:

- Create a Rust module corresponding directly to `src/wordsplit/wordsplit.c`.
- Migrate the listed functions first as internal helpers, keeping signatures close to the C logic while using Rust slices, `String`, `Vec`, and enums for safety.
- Represent parser state and flags with explicit Rust structs/enums/bitflags-style newtypes only where the C code already implies stateful data.
- Convert pointer-walking logic into index-based scanning over `&[u8]` or `&str` as appropriate, to preserve byte-precise behavior for quoting and delimiter detection.
- Replace C error reporting and append-side memory growth with `Result`, optional error fields in the parser state, and `Vec`/`String` growth managed by Rust.

The implementation should not add new parsing features or split this work into broader refactors beyond the migrated file and its directly required local data definitions.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**:
  - Rust standard library
  - No third-party crates required by the provided evidence
- **Testing**:
  - `cargo test`
  - Unit tests colocated with the Rust module or in the crate test layout
- **Performance Goals**:
  - Preserve the C module’s linear scan behavior for character-by-character parsing helpers
  - Avoid unnecessary UTF-8 reallocation when byte-oriented scanning is sufficient
  - Keep append operations amortized linear by using `Vec`/`String`
  - Maintain comparable behavior for small utility functions without introducing regex or parser-generator overhead

## Module Mapping

### Source Mapping

| C Source File | Rust Target |
|---|---|
| `src/wordsplit/wordsplit.c` | `src/wordsplit/wordsplit.rs` |

### Function Mapping

| C Function | Rust Target | Notes |
|---|---|---|
| `_wsplt_error` | `fn wsplt_error(...)` | Internal helper; convert C-style reporting into state update and/or `Result` construction |
| `wsnode_flagstr` | `fn wsnode_flagstr(...) -> String` or `&'static str`-based assembly | Keep output format identical to C behavior |
| `wordsplit_append` | `fn wordsplit_append(...) -> Result<(), WordSplitError>` | Use `Vec`/`String` growth instead of manual allocation |
| `find_closing_paren` | `fn find_closing_paren(...) -> Option<usize>` | Index-based scan replacing pointer arithmetic |
| `begin_var_p` | `fn begin_var_p(...) -> bool` | Preserve exact detection rules |
| `begin_cmd_p` | `fn begin_cmd_p(...) -> bool` | Preserve exact detection rules |
| `isglob` | `fn isglob(...) -> bool` | Byte scan for glob metacharacters |
| `skip_sed_expr` | `fn skip_sed_expr(...) -> usize` or `Option<usize>` | Return next index consistently with C scan outcome |
| `xtonum` | `fn xtonum(...) -> Result<_, WordSplitError>` | Use checked numeric conversion matching C constraints |
| `wsplt_unquote_char` | `fn wsplt_unquote_char(...) -> Option<char>` or byte result | Match escape decoding semantics exactly |
| `wsplt_quote_char` | `fn wsplt_quote_char(...) -> ...` | Preserve quote/escape emission rules |

### Visibility Mapping

- Keep these functions crate-private or private unless another already-migrated Rust module needs them.
- Do not widen visibility beyond what is required to replace current C-local coupling.

## Data Model

The analysis only exposes anonymous C data structures, so the Rust data model should be derived minimally from direct usage within the migrated functions.

### C-to-Rust Mapping Strategy

| C Pattern | Rust Mapping | Notes |
|---|---|---|
| Anonymous parser/config struct | Named `struct` in `wordsplit.rs` | Name according to role inferred from use sites, not invented subsystem layering |
| Anonymous node/flag holder | Named `struct` plus flag newtype or constants | Keep field set minimal and migration-driven |
| `char *` mutable buffer | `String` or `Vec<u8>` | Use `Vec<u8>` for byte-exact parsing/emission; convert to `String` only when valid text is required |
| `const char *` input cursor | `&str` plus byte index, or `&[u8]` plus index | Prefer `&[u8]` for exact C-like scanning |
| Linked/owned dynamic arrays | `Vec<T>` | Replace manual allocation and resizing |
| Integer flags/bitmasks | `u32`/`u64` newtype or associated constants | Avoid extra crates unless already required elsewhere |
| Error code + side-channel message | `enum WordSplitError` and/or stored error state | Preserve call-site behavior expected by the rest of the module |

### Expected Rust Types

The following named Rust types should be introduced only if required by the migrated code paths:

- `struct WordSplit`
  Rust counterpart for the main state object used by append and error helpers.

- `struct WsNode`
  Rust counterpart if `wsnode_flagstr` formats flags from a node-like value.

- `enum WordSplitError`
  Encodes error cases currently represented by C status codes/messages.

- `type WsFlags = u32` or a small newtype wrapper
  Used for node/parser flags if the C code uses bitmasks.

### Memory Management Decisions

- Replace all manual buffer growth with `Vec` or `String`.
- Eliminate raw ownership concerns by storing owned data directly in Rust structs.
- Use borrowed slices for scanner helpers to avoid copying.
- Keep return values explicit instead of writing through output pointers where practical, unless preserving a mutable-state API makes migration of neighboring code simpler.

### Error Handling Decisions

- Internal scanning predicates (`begin_var_p`, `begin_cmd_p`, `isglob`) remain infallible.
- Scanners with failure cases (`find_closing_paren`, `skip_sed_expr`) should use `Option<usize>` or `Result<usize, WordSplitError>` depending on whether the C caller distinguishes malformed input from “not found”.
- `_wsplt_error` should centralize message creation and parser-state updates; neighboring migrated code can then convert that into returned `Result` values without duplicating formatting logic.
- Numeric conversion in `xtonum` must use checked arithmetic and explicit radix handling to avoid silent overflow differences from C.

## Implementation Phases

## Phase 1: Establish module skeleton and state/type mappings

Goals:

- Create `src/wordsplit/wordsplit.rs` as the direct Rust home for this C file’s migrated logic.
- Identify the exact state objects touched by the listed functions and define the minimum Rust structs/enums/constants needed for compilation.
- Set up flag representations and basic error type(s) without broad redesign.

Tasks:

- Add Rust equivalents for the main wordsplit state and any node/flag-bearing structures referenced by these functions.
- Define flag constants/newtypes matching the C bitmask layout used by `wsnode_flagstr` and related helpers.
- Define `WordSplitError` and any internal error payload fields required by `_wsplt_error`.
- Add placeholder unit tests for module wiring and type construction.

Exit criteria:

- The Rust module compiles with type definitions and function stubs in place.
- No extra modules or abstraction layers are introduced beyond the direct file replacement need.

## Phase 2: Port pure scanning and quoting helpers

Goals:

- Migrate the side-effect-light helper functions first to lock down byte-level behavior.

Tasks:

- Implement `begin_var_p`, `begin_cmd_p`, and `isglob` as direct byte scanners.
- Implement `find_closing_paren` using indexed traversal with nesting/escape handling matching the C logic.
- Implement `skip_sed_expr` with the same delimiter and escape skipping behavior as the original.
- Implement `wsplt_unquote_char` and `wsplt_quote_char`, preserving exact escape mappings and return conventions.
- Implement `xtonum` with explicit radix parsing and checked overflow behavior aligned with the C function.

Testing focus:

- Predicate coverage for variable/command/glob starts.
- Balanced and unbalanced delimiter cases for parenthesis scanning.
- Escaped delimiter handling in sed-expression skipping.
- Quote/unquote round-trip cases for supported escapes.
- Numeric parsing edge cases, including invalid digits and overflow boundaries.

Exit criteria:

- All helper functions have deterministic unit tests based on current C behavior.
- Byte-level behavior is captured without introducing UTF-8 assumptions not present in the original code.

## Phase 3: Port append and error-path behavior

Goals:

- Migrate the functions that mutate parser state or produce formatted diagnostics.

Tasks:

- Implement `_wsplt_error` as the centralized error-state update/helper used by this module.
- Implement `wordsplit_append` using `Vec`/`String` growth and preserving append ordering and failure semantics expected by callers.
- Implement `wsnode_flagstr` using the migrated flag constants and formatting order from the C code.
- Adjust function signatures and local call sites in the Rust module so state mutation and error propagation remain close to the original control flow.

Testing focus:

- Error message/state updates for representative failure paths.
- Append behavior for empty, single, and repeated additions.
- Flag-string formatting order and inclusion/exclusion rules.

Exit criteria:

- Stateful helpers are functionally ported.
- Memory growth and error paths are handled solely through safe Rust containers and explicit results/state.

## Phase 4: Integration validation against surrounding wordsplit logic

Goals:

- Confirm that the migrated functions fit the existing Rust port of the surrounding file without expanding scope.

Tasks:

- Replace any temporary stubs with final implementations wired into neighboring migrated code from `wordsplit.c`.
- Normalize signatures only where required for compatibility with the rest of the Rust port.
- Add focused regression tests covering interactions among scanning, quoting, append, and error helpers as used by actual wordsplit flows.
- Remove dead transitional code introduced during porting.

Exit criteria:

- The module builds cleanly on branch `119-module_src_wordsplit_wordsplit.c_08-rust-port`.
- `cargo test` passes for unit and integration coverage relevant to this migrated file segment.
- The migrated Rust module remains a direct replacement path for the C functionality in scope, without added capabilities.