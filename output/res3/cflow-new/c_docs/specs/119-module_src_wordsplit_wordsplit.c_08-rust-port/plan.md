# Implementation Plan

## Summary

Port `src/wordsplit/wordsplit.c` into an idiomatic Rust module that preserves the current parsing-oriented behavior of the selected function set without adding new capabilities. The scope of this plan is limited to migrating the listed functions and the directly supporting data they require.

The Rust implementation should keep the logic close to the C source, especially for:
- parser-style character scanning,
- quote and escape handling,
- simple numeric conversion,
- shell-like pattern checks,
- variable / command opening detection,
- append-style string accumulation,
- internal error reporting.

The technical approach is:
- migrate the C file into a single Rust source module with a narrow internal API,
- replace pointer arithmetic with slice/index-based scanning over `&[u8]` or `&str` as appropriate,
- represent mutable output buffers with `String` or `Vec<u8>` depending on whether the original behavior is character-oriented or byte-oriented,
- convert C error signaling into `Result<_, WordSplitError>` only where the original functions can fail; keep pure predicates as plain return values,
- preserve flag-driven behavior through Rust bitflags-style representations or plain newtype flag wrappers if external crates are unnecessary.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only by default
  - `std::string::String`
  - `std::vec::Vec`
  - `std::fmt`
  - `std::num`
  - `std::ops`
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve linear-time scanning behavior for all parser helpers.
  - Avoid unnecessary string reallocations during append and quote/unquote operations.
  - Keep conversions allocation-light by scanning input in place and returning borrowed positions or indices instead of copying substrings where feasible.
  - Match the C implementation’s practical runtime characteristics for character-by-character parsing.

## Module Mapping

### Source File Mapping

| C Source | Rust Target |
|---|---|
| `src/wordsplit/wordsplit.c` | `src/wordsplit.rs` |

If the existing Rust crate already places this logic under a wordsplit namespace, keep the migration inside that existing file path or the closest equivalent single module file. Do not split this work into additional helper modules unless the branch already requires that structure.

### Function Mapping

| C Function | Rust Function | Notes |
|---|---|---|
| `_wsplt_error` | `fn wsplt_error(...) -> WordSplitError` or `fn wsplt_error(ctx: &mut WordSplit, ...)` | Convert variadic / formatted internal error handling into structured Rust error creation and context update. |
| `wsnode_flagstr` | `fn wsnode_flagstr(flags: NodeFlags) -> String` | Keep as internal formatting helper for debug/status text. |
| `wordsplit_append` | `fn wordsplit_append(buf: &mut String, data: &str)` or byte equivalent | Use safe buffer growth; preserve append semantics. |
| `find_closing_paren` | `fn find_closing_paren(input: &[u8], start: usize) -> Option<usize>` | Index-based scanner replacing pointer traversal. |
| `begin_var_p` | `fn begin_var_p(input: &[u8], pos: usize) -> bool` | Predicate over indexed input. |
| `begin_cmd_p` | `fn begin_cmd_p(input: &[u8], pos: usize) -> bool` | Predicate over indexed input. |
| `isglob` | `fn isglob(input: &str) -> bool` | Preserve shell-pattern detection logic. |
| `skip_sed_expr` | `fn skip_sed_expr(input: &[u8], start: usize) -> Option<usize>` | Byte scanner returning next index after sed-style expression. |
| `xtonum` | `fn xtonum(input: &str, base: u32) -> Result<..., WordSplitError>` | Replace C manual numeric conversion and overflow handling with checked Rust parsing plus explicit validation where behavior differs. |
| `wsplt_unquote_char` | `fn wsplt_unquote_char(...) -> Result<char_or_u8, WordSplitError>` | Preserve escape decoding rules. |
| `wsplt_quote_char` | `fn wsplt_quote_char(...) -> ...` | Preserve quoting/escaping rules used by the module. |

## Data Model

The analysis only exposes anonymous C data structures, so the migration should avoid inventing broad new types and instead define only the minimal Rust representations required by the listed functions and their immediate call sites.

### Core Mapping Strategy

| C Form | Rust Form | Notes |
|---|---|---|
| anonymous internal struct used as parser/context state | `struct WordSplit { ... }` | Consolidate fields actually used by the migrated functions. |
| anonymous flags field / bit mask | `type NodeFlags = u32` or `struct NodeFlags(u32)` | Prefer plain integer wrapper unless stronger typing is already used elsewhere in the branch. |
| `char *` mutable output buffer | `String` or `Vec<u8>` | Choose `String` for validated text; use `Vec<u8>` if byte-level escaping must preserve non-UTF-8 semantics. |
| `const char *` input pointer | `&str` or `&[u8]` | Use `&[u8]` for direct C-like scanning and indexing. |
| pointer + length scan state | `(input: &[u8], pos: usize)` | Replaces raw pointer arithmetic safely. |
| integer error code | `enum WordSplitError` or `Result<T, WordSplitError>` | Centralized error type for migrated routines. |

### Recommended Rust Types

#### `WordSplit`
Define a narrow context struct only if `_wsplt_error` and append/error-related behavior require persistent state. Suggested fields should be restricted to migrated usage, for example:
- error status / last error,
- optional error message buffer,
- any active flags already required by `wsnode_flagstr` or quoting behavior.

Do not preemptively model the entire original C object graph if these functions only touch a subset.

#### `WordSplitError`
Use a dedicated enum for internal failure cases exposed by this function set, such as:
- invalid numeric input,
- numeric overflow,
- malformed escape sequence,
- unmatched delimiter / parenthesis,
- invalid parser state if needed.

Include message text only where the C path already formats an error string.

#### `NodeFlags`
If `wsnode_flagstr` formats a node flag mask, represent flags in one of these restrained forms:
- `type NodeFlags = u32` with constants, or
- `struct NodeFlags(u32)` with associated constants.

Use a third-party flags crate only if the existing Rust branch already depends on it; otherwise keep this in std-only form.

### String and Character Handling

Because the C module likely scans shell-style syntax byte-by-byte, the safest default is:
- parse using `&[u8]`,
- convert to `String` only for owned textual results,
- validate UTF-8 only at boundaries where the public Rust API requires it.

This avoids accidental semantic drift from Rust `char` iteration, which is Unicode scalar-based and does not match C `char` scanning.

## Implementation Phases

## Phase 1: Establish Rust module skeleton and core type replacements

### Goals
Create the Rust destination for `src/wordsplit/wordsplit.c` logic and define the minimum types needed for the migrated functions.

### Tasks
- Add or update `src/wordsplit.rs` as the target module for this C file’s port.
- Introduce the minimal `WordSplit` context struct only if required by `_wsplt_error` or append behavior.
- Define `WordSplitError` and the return conventions for fallible helpers.
- Define `NodeFlags` as a restrained integer-backed representation.
- Decide per function whether input is `&str` or `&[u8]`; default scanners to `&[u8]`.
- Add unit test scaffolding for the listed functions only.

### Notes
This phase should not re-architect the parser. It should only create safe Rust equivalents for memory ownership, mutable buffers, flags, and error transport.

## Phase 2: Port pure scanners and predicate helpers

### Goals
Migrate the parsing helpers that are primarily read-only scans over input.

### Tasks
- Port `begin_var_p`.
- Port `begin_cmd_p`.
- Port `find_closing_paren`.
- Port `isglob`.
- Port `skip_sed_expr`.

### Implementation guidance
- Replace all pointer increments with explicit `usize` index movement.
- Preserve delimiter and escape rules exactly, including early-exit cases.
- Return `Option<usize>` for “not found” style scans instead of sentinel pointers or `NULL`.
- Keep helper visibility private unless already needed elsewhere in the Rust crate.

### Testing focus
- opening sequence detection at valid and invalid positions,
- nested and escaped parenthesis handling,
- glob metacharacter detection with escaped characters,
- sed-expression skipping with alternate delimiters and escaped delimiters.

## Phase 3: Port conversion and quote-handling routines

### Goals
Migrate the helpers that transform characters or parse numeric values.

### Tasks
- Port `xtonum`.
- Port `wsplt_unquote_char`.
- Port `wsplt_quote_char`.

### Implementation guidance
- For `xtonum`, preserve accepted bases and input validation rules before relying on Rust parse helpers.
- Use checked arithmetic or standard parsing APIs to avoid overflow differences from C.
- For quote/unquote helpers, keep behavior byte-oriented if escapes may operate outside valid UTF-8 character boundaries.
- Where the C code returns a transformed byte and advances a pointer, model the Rust equivalent as a return of both value and next index when necessary.

### Testing focus
- valid and invalid numeric forms,
- base-specific digits,
- overflow and underflow handling if applicable,
- escaped quote sequences,
- octal/hex escape handling if present in the original logic,
- passthrough of unescaped characters.

## Phase 4: Port append/error helpers and align behavior with call sites

### Goals
Finish the mutable-state helpers and integrate all migrated functions into a coherent Rust module.

### Tasks
- Port `wordsplit_append`.
- Port `_wsplt_error`.
- Port `wsnode_flagstr`.
- Adjust any call sites within the Rust port of this file so signatures are consistent.
- Normalize buffer ownership and error propagation across the module.

### Implementation guidance
- `wordsplit_append` should use `String::push_str`, `String::push`, or `Vec<u8>::extend_from_slice` depending on the chosen buffer type; avoid manual capacity logic unless needed to mirror behavior.
- `_wsplt_error` should centralize construction of `WordSplitError` and any context mutation, rather than reproducing C-style global or side-channel state.
- `wsnode_flagstr` should remain a narrow formatter for existing flags, not a generalized inspection API.

### Testing focus
- append behavior for empty and non-empty buffers,
- repeated appends without corruption,
- error creation and stored message consistency,
- flag-string output for empty, single, and combined flags.

## Completion Criteria

The module migration is complete when:
- all listed functions from `src/wordsplit/wordsplit.c` have Rust equivalents in the target module,
- anonymous C state used by these functions has been replaced by minimal named Rust types,
- unsafe code is avoided unless a specific upstream interface makes it unavoidable,
- all fallible paths use explicit Rust error handling,
- `cargo test` covers the migrated behaviors for scanning, quoting, numeric conversion, append, and error reporting,
- no additional capabilities or new public subsystems have been introduced beyond the C module’s current responsibilities.