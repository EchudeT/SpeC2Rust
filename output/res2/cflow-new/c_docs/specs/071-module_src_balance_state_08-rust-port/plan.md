# Implementation Plan: module_src_balance_state_08

## Summary

This module covers the parser-side balance-state stack logic currently implemented in `src/parser.c`, specifically the migration of:

- `push_balance_state`
- `pop_balance_state`
- `free_balance_stack`
- `find_closing_paren`

The Rust implementation should keep the same narrow responsibility: maintain parser balance state during nested scanning and provide the delimiter-matching helper used by the parser. The preferred approach is a direct port into a focused Rust module that preserves current control flow and ownership boundaries while replacing manual allocation/free patterns with standard Rust ownership and collection types.

The implementation should avoid introducing new parser abstractions beyond what is required to port the existing functions. Stack management should be represented with `Vec<T>` or a small dedicated wrapper around `Vec<T>`, and delimiter scanning should operate on borrowed string/byte slices to avoid unnecessary copying.

## Technical Context

### Language / Version
- Rust 1.78 or newer

### Primary Dependencies
- Rust standard library only
- No third-party crates are recommended based on the available module evidence

### Testing
- `cargo test`

### Performance Goals
- Preserve the current asymptotic behavior of the C implementation
- Keep push/pop operations amortized O(1)
- Keep closing-parenthesis search linear in the scanned input segment
- Avoid heap allocations beyond those already implied by stack growth
- Avoid unnecessary string cloning during scan operations

## Module Mapping

### C Source to Rust Module
- `src/parser.c` -> `src/parser.rs`

If `parser.rs` already exists as part of the broader port, this work should be added into that file or its existing internal submodule structure without creating unrelated new modules.

### Function Mapping
- `push_balance_state` -> `push_balance_state(...)`
- `pop_balance_state` -> `pop_balance_state(...)`
- `free_balance_stack` -> removed as an explicit public operation where Rust ownership makes it unnecessary; if call-site parity is needed, retain a small compatibility function that clears or takes ownership of the stack
- `find_closing_paren` -> `find_closing_paren(...)`

The Rust names may remain close to the C names to reduce migration risk and ease review.

## Data Model

The C analysis only identifies multiple anonymous data structures, so the Rust plan should derive concrete types from actual field usage in `src/parser.c` during implementation. The mapping should stay minimal and local to the parser module.

### Planned Mapping Strategy
- C anonymous balance-state node/record -> Rust `struct BalanceState`
- C linked stack representation, if present -> Rust `Vec<BalanceState>`
- C parser-owned pointer to stack head -> Rust parser field such as `balance_stack: Vec<BalanceState>`
- C integer/flag fields -> Rust integer primitives (`usize`, `isize`, `i32`) or `bool`, chosen by usage
- C delimiter character fields -> Rust `u8` or `char`, chosen to match source scanning logic
- C nullable node pointers used for stack navigation -> eliminated in favor of `Vec` indexing and `Option`

### Ownership and Memory Management Decisions
- Manual node allocation in `push_balance_state` should be replaced by pushing a value into a `Vec`
- Manual unlinking/free in `pop_balance_state` should be replaced by `Vec::pop`
- `free_balance_stack` should become implicit through drop semantics; if call sites require an explicit action, use `Vec::clear`
- Any temporary scan state should use borrowed references (`&str` or `&[u8]`) rather than owned buffers

### Error Handling Decisions
- C null-pointer and empty-stack conditions should map to explicit return values such as `Option<T>` or `Result<T, E>`
- For `find_closing_paren`, failure to find a matching delimiter should be represented explicitly rather than via sentinel pointer math where possible
- Internal invariants formerly assumed by C code should be expressed through type signatures and guarded indexing

## Implementation Phases

### Phase 1: Extract and Define Rust Data Structures
- Inspect `src/parser.c` usage around the four target functions
- Identify the exact anonymous C structures participating in balance-state handling
- Define the minimal Rust `struct` types required to represent:
  - one balance-state entry
  - the parser-owned stack state
  - any local scan metadata directly used by `find_closing_paren`
- Add these definitions to `src/parser.rs` in the existing parser area, keeping visibility as narrow as possible
- Replace pointer-based stack representation with `Vec<BalanceState>`

### Phase 2: Port Stack Operations
- Port `push_balance_state` using `Vec::push`
- Port `pop_balance_state` using `Vec::pop`
- Replace `free_balance_stack` with Rust drop-based cleanup, or a compatibility method that clears the vector if existing control flow still calls it
- Update affected parser call sites to use borrowed mutable access instead of raw pointer manipulation
- Preserve existing semantics for empty-stack handling and state restoration

### Phase 3: Port Delimiter Scan Logic
- Port `find_closing_paren` closely from the C implementation
- Choose `&[u8]` if the original logic is byte-oriented and assumes ASCII delimiter matching; otherwise use `char_indices()` only if the C behavior clearly depends on character semantics
- Preserve nesting behavior and any skip rules already encoded in the C function
- Return an index, slice position, or `Option`-wrapped result consistent with surrounding parser code rather than emulating raw C pointers directly

### Phase 4: Validation and Cleanup
- Add targeted unit tests covering:
  - push/pop ordering
  - pop on empty stack
  - explicit clear/free-equivalent behavior if retained
  - matching and non-matching closing-parenthesis scans
  - nested delimiter cases reflected by the C behavior
- Run `cargo test`
- Remove any remaining manual-lifetime patterns that are no longer needed after the port
- Confirm that the final Rust code does not introduce broader parser refactors beyond this module migration

## Notes and Constraints

- Keep the migration scoped to the identified functions and their directly required data structures
- Do not introduce concurrency primitives, trait-heavy abstractions, or generalized parsing frameworks
- Prefer direct transliteration first, then small Rust-native cleanup only where it reduces memory-unsafety or sentinel-style error handling
- Where the C code relies on anonymous structs, name Rust types according to their actual role in the four migrated functions rather than inventing broader domain terminology