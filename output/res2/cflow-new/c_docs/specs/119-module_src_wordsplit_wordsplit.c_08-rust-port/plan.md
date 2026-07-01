# Implementation Plan

## Summary

Port `src/wordsplit/wordsplit.c` functionality for the selected function set into a Rust module that preserves the existing parsing and string-processing behavior without adding new features. The implementation should focus on direct migration of the current responsibilities in `wordsplit.c`: internal error reporting helper behavior, node flag formatting, append logic for split output, delimiter scanning, variable/command substitution detection, glob detection, sed-expression skipping, numeric conversion, and quote/unquote character handling.

The Rust approach should keep the logic concentrated in a single corresponding module under the existing project layout, using standard-library string and byte-slice processing rather than introducing a redesigned parser architecture. Where the C code relies on pointer arithmetic and mutable buffers, the Rust port should use index-based traversal over `&[u8]` / `&str`, `String` for owned text accumulation, and explicit result types for conversions and error paths. The goal is behavioral parity with the C implementation while replacing manual memory handling with Rust ownership and bounds-checked access.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve linear-time behavior for scanning helpers such as parenthesis matching, sed-expression skipping, and glob detection.
  - Avoid unnecessary intermediate allocations during append and character-quoting operations.
  - Keep traversal logic close to the original control flow so performance remains comparable to the C implementation for typical shell-style input sizes.

## Module Mapping

- **C source file**
  - `src/wordsplit/wordsplit.c`

- **Rust target module**
  - `src/wordsplit/wordsplit.rs`

- **Function migration mapping**
  - `_wsplt_error` -> `fn wsplt_error(...)` as an internal helper returning/recording Rust error state
  - `wsnode_flagstr` -> `fn wsnode_flagstr(...) -> String` or `&'static str`-driven formatter, depending on original output ownership
  - `wordsplit_append` -> `fn wordsplit_append(...) -> Result<(), WordSplitError>`
  - `find_closing_paren` -> `fn find_closing_paren(...) -> Option<usize>`
  - `begin_var_p` -> `fn begin_var_p(...) -> bool`
  - `begin_cmd_p` -> `fn begin_cmd_p(...) -> bool`
  - `isglob` -> `fn isglob(...) -> bool`
  - `skip_sed_expr` -> `fn skip_sed_expr(...) -> Option<usize>`
  - `xtonum` -> `fn xtonum(...) -> Result<_, WordSplitError>`
  - `wsplt_unquote_char` -> `fn wsplt_unquote_char(...) -> Option<char>` or byte-oriented equivalent
  - `wsplt_quote_char` -> `fn wsplt_quote_char(...) -> String` or append-into-buffer helper

The implementation should remain in one Rust source module unless the existing Rust tree already defines a narrower placement for `wordsplit` internals. No extra abstraction layers should be introduced beyond what is needed to map existing C-local helpers into Rust-private functions.

## Data Model

Because the analysis only exposes anonymous C data structures, the Rust plan should derive mappings from actual usage inside `src/wordsplit/wordsplit.c` and avoid speculative redesign. The data model work for this module should be limited to the structures directly touched by the listed functions.

### Expected mapping approach

- **Anonymous C structs used as wordsplit state**
  - Map to named Rust `struct`s with the narrowest field set required by migrated functions.
  - Replace raw owned string buffers with `String` or `Vec<u8>`.
  - Replace pointer/length buffer pairs with:
    - `String` for text buffers
    - `Vec<T>` for resizable collections
    - `&str` / `&[u8]` for borrowed scan inputs

- **Anonymous C flag containers / bitfields**
  - Map to integer-backed flag types:
    - `u32`/`u64` plus associated constants by default
  - Use a Rust `enum` only if the original semantics are mutually exclusive rather than bit-combined.

- **Anonymous node-like records referenced by `wsnode_flagstr` and append logic**
  - Map to a named `struct WsNode` or equivalent existing project type.
  - Preserve field-level meaning and flag layout from C rather than normalizing the representation.

- **Error state structures used by `_wsplt_error`**
  - Map to either:
    - a dedicated `WordSplitError` enum for returned errors, and/or
    - mutable fields on the wordsplit context if the original API stores last-error information internally.
  - Preserve whether the original behavior accumulates errors, formats messages, or sets status codes.

### C-to-Rust representation rules

- `char *` input inspected but not owned -> `&str` when valid UTF-8 is guaranteed by surrounding Rust API; otherwise `&[u8]`
- Mutable C output buffers -> `String` or `Vec<u8>`
- Pointer walking over source text -> indexed access with `usize`
- Sentinel returns such as `NULL` or negative offsets -> `Option<usize>` / `Result<T, WordSplitError>`
- Numeric conversions from textual escape/number parsing -> fixed-width integer types matching the consumed C width after code inspection

## Implementation Phases

## Phase 1: Establish module skeleton and migrate low-risk helpers

- Create or update `src/wordsplit/wordsplit.rs` for the direct port of this C file region.
- Identify the Rust equivalents for the wordsplit context and any node/flag structures touched by the listed functions.
- Implement internal helper signatures first, using private functions that mirror current call patterns.
- Port:
  - `begin_var_p`
  - `begin_cmd_p`
  - `isglob`
  - `wsplt_unquote_char`
  - `wsplt_quote_char`
- Keep these helpers close to byte-level C behavior; prefer byte-slice scanning if escaping rules are not purely Unicode-oriented.
- Add focused unit tests for each helper using examples extracted from current C behavior and edge cases around empty input, escapes, and special characters.

## Phase 2: Port scanning and conversion logic

- Port traversal-heavy helpers with explicit index management:
  - `find_closing_paren`
  - `skip_sed_expr`
  - `xtonum`
- Preserve original delimiter, nesting, and escape-handling rules exactly, especially where C pointer arithmetic previously allowed tight loops.
- For `xtonum`, model invalid input and overflow using `Result` instead of unchecked casts or partial parsing.
- Validate that all returned positions are in Rust index units consistent with the surrounding caller logic; if byte indexing is required, keep all related helpers byte-oriented.
- Add tests covering:
  - balanced and unbalanced delimiters
  - escaped delimiters
  - sed separators and embedded escapes
  - accepted numeric bases/ranges
  - malformed numeric input

## Phase 3: Port append and flag/error integration

- Port `wsnode_flagstr` using the actual flag constants from the C implementation.
- Port `_wsplt_error` with attention to whether the original API stores formatted state in a context object or returns immediate failure information.
- Port `wordsplit_append`, preserving ordering, concatenation rules, and ownership transitions from the C implementation.
- Replace manual allocation/reallocation patterns with `String::push_str`, `String::push`, `Vec::push`, or `Vec::extend_from_slice` as appropriate.
- Ensure append operations do not invalidate references by structuring ownership clearly around the wordsplit context.
- Add tests for:
  - empty and repeated append operations
  - flag-string formatting stability
  - error propagation/state updates
  - append behavior under quoting/escaping-sensitive inputs

## Phase 4: Module integration and parity validation

- Integrate the migrated functions with the existing Rust wordsplit module entry points on branch `119-module_src_wordsplit_wordsplit.c_08-rust-port`.
- Remove any temporary compatibility code that is no longer needed after direct migration.
- Run `cargo test` and fix mismatches caused by byte/character indexing assumptions.
- Perform a final review to confirm:
  - no extra functionality was introduced
  - all memory ownership previously handled manually in C is represented safely in Rust
  - internal helpers remain private unless existing project APIs require exposure
  - function behavior remains aligned with the original `wordsplit.c` implementation